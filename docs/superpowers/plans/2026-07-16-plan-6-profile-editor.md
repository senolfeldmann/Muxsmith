# Plan 6: profile editor, apply-suggestion, schema keyword domains

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** implement ADRs D41-D48 (`docs/superpowers/specs/2026-07-15-plan-6-design.md`): a hand-built profile editor over a canonical-save core writer, one-click apply-suggestion, ts-rs-generated wire types with a CI drift check, and the schema keyword-domain fix that makes `muxsmith schema` a supported user artifact.

**Architecture:** four waves. Wave 1 runs three independent streams in parallel worktrees: stream A is a four-task serial chain because D41/D46/D48/D44 all converge on `crates/muxsmith-core/src/profile/model.rs`; stream B is the planner-side applier; stream C is docs. Wave 2 is the Tauri shell's IPC surface, which needs A's writer and B's applier. Wave 3 is the frontend, which needs A's generated bindings and wave 2's commands. Wave 4 is the batch view's apply button. Merge sequentially, nine-part gate after every merge.

**Tech Stack:** Rust workspace (schemars 1.2.1, serde, yaml_serde 0.10.4, ts-rs 12.0.1 behind a `ts` feature), Tauri 2 shell, Vue 3 + TypeScript 6.0.3, Fluent (en+de), Playwright.

## Global Constraints

- **The design document is the contract**; the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative above it on conflict. Its decisions are settled and reviewed - do not reopen them, and do not re-derive their rationales.
- **Every fork in this plan is closed.** No task brief, verdict or fix-round dispatch may add a design-latitude clause, in either form: an explicit permission ("if a simpler alternative exists, implement it") or an omission (an unenumerated set in a normative position). The test is "must the implementer invent something it is not allowed to invent?" (`proc-latitude-clause-boundary`). A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** - options, costs against the named invariants, a recommendation - and is routed by the controller before it is resolved. It is never decided at the keyboard, and "push back if you disagree" is not a licence to decide-then-report.
- **Two owner rulings post-date the design document and bind over its text** (Şenol, 2026-07-16). Task 1 folds both into the design; every later task reads the amended document:
  1. **The save writer's error currency is `SaveError`, not `Diagnostic`** (design `:132-133` is superseded). `profile::save` returns `Result<_, SaveError>` with `SaveError::{Io(String), Serialize(String)}`; the shell maps it to `IpcError` codes in `gui-common.ftl`. No new `DiagCode`, no `diagnostics.ftl` change. Rationale is Tier-2 `core-124-error-currency-split`: a `Diagnostic` describes a profile/plan problem, a write failure does not; `ParseError` is never reused for a non-parse failure because its prose asserts a parse.
  2. **The save-surface note is ONE Fluent message** (design `:278` "Two new Fluent keys" is superseded). `gui-editor.ftl` carries **43** keys: 42 registry labels + 1 note, exactly as design `:1736`/`:1749` already state.
- **Tier-2 files are ground truth alongside the spec**: `docs/product-boundaries.yaml` (product scope), `docs/conventions.yaml` (house code style), `docs/process-conventions.yaml` (method). Conform to them; surface, never silently resolve, any new pattern you establish or deliberate deviation.
- **Nine-part gate green before any push**, per BUILDING.md, run foreground, no subsets: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo deny check`, `pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`.
- **Every new or changed Fluent message lands bilingual (en + de) in the same commit.** The de register follows the de catalog header rules (config keywords literal, straight quotes, du-imperative).
- **Versions are pinned and registry-verified, never typed from memory** (`ci-10-pin-everything`, `proc-07-verify-against-source`): `ts-rs = { version = "12.0.1", optional = true }`, caret semantics matching `schemars = "1.2.1"`. `=`-pins are reserved for dev-dependencies in this tree.
- **Commits unsigned** (`git -c commit.gpgsign=false commit ...`), trailer `Co-Authored-By: <your model name> <noreply@anthropic.com>`, stage files explicitly - **never `git add -A`**.
- **Typography**: ASCII hyphens, straight quotes, no Unicode ellipsis, in all docs, code comments and messages.
- **`#![deny(missing_docs)]` is on**: every new public item needs rustdoc, and `cargo doc` gates that the intra-doc links resolve.

## How this plan cites the design

The design document is 2006 lines of settled, measured decisions, and it is what every reviewer grades against. Where it already states the implementation exactly - the 17-field table, the 43-field widget table, the predicates, the emitter, the `extend` shape - **this plan cites it by section and line range rather than copying it**. That is deliberate: a second copy of a normative table is a drift surface, and refusing to create one is this design's own most-repeated argument (D45's keyword arrays, D48's `extend` derivation, `capability::CODEC_KIND_NAMES`). Every task below names the exact lines its implementer must read. Everything the design does *not* already carry - test code, commands, file paths, task boundaries - is written out in full here.

---

## Wave 1

Three streams, parallel worktrees, no shared files between them.

- **Stream A** (`.worktrees/plan6-a`): Tasks 2, 3, 4, 5 - **serial**. All four converge on `crates/muxsmith-core/src/profile/model.rs`, so they are a chain, not a fan-out. Task 4 additionally needs Task 2's writer for its round-trip guard, and Task 5 needs Task 3's constants for its emitter.
- **Stream B** (`.worktrees/plan6-b`): Task 6 - `planner.rs` only.
- **Stream C** (`.worktrees/plan6-c`): Task 7 - docs only.

Task 1 lands on master before any stream branches, because all three read the design document.

---

### Task 1: Fold the two owner rulings into the design document

**Files:**
- Modify: `docs/superpowers/specs/2026-07-15-plan-6-design.md`

**Interfaces:**
- Consumes: nothing.
- Produces: the amended design document every later task reads as ground truth.

This is a **documentation-only** task. It changes no code. Its purpose is that no implementer reads a superseded signature: the design as approved fixes the save writer at `Result<_, Diagnostic>` and names no `DiagCode`, which is the fork the owner ruled on. Do not re-argue either ruling; record them.

- [ ] **Step 1: Amend D41's writer signature and error currency**

At `:130-138` the document reads (in part) "`to_string(&Profile, Format) -> Result<String, Diagnostic>` and `to_file(&Profile, &Path) -> Result<(), Diagnostic>`". Replace the error type in both signatures with `SaveError` and add, immediately after that paragraph, a subsection recording the ruling:

```markdown
**Error currency: `SaveError`, mapped to `IpcError` at the shell** (owner
ruling 2026-07-16, superseding this ADR's original `Result<_, Diagnostic>`).

```rust
/// A failure of the profile writer. Not a `Diagnostic`: a `Diagnostic`
/// describes a problem with the profile or the plan, and a write failure
/// leaves a valid model and a full disk (`core-124-error-currency-split`).
pub enum SaveError {
    /// The file could not be written (permissions, full disk, bad path).
    Io(String),
    /// The model could not be serialized to the target format.
    Serialize(String),
}
```

The shell maps it in `src-tauri/src/error.rs`, mirroring `SettingsError`:
`SaveError::Io` -> `profile-save-io-failed`, `SaveError::Serialize` ->
`profile-save-failed`, both carrying a `detail` param (the spec 8.4
third-party-message exception). No new `DiagCode`; `diagnostics.ftl` is
untouched.

**Why not a `Diagnostic`.** The original signature was chosen for symmetry
with `profile::load`, which does return `Result<Profile, Diagnostic>`. The
symmetry does not carry: the loader's `Diagnostic` is right because a parse
failure IS a profile problem - the file's content is wrong - whereas a write
failure is not. `src-tauri/src/error.rs:8-15` already drew that line ("an
`IpcError` describes an IPC-protocol-level failure ... an unreadable path")
and this ADR contradicted it unnoticed through four review rounds, because
the boundary was written only in rustdoc and nowhere a reviewer checks. It
is now Tier-2 `core-124-error-currency-split`. Reusing `ParseError` was
rejected outright: its catalog prose is `parse-error = The profile could not
be parsed: { $detail }`, which is a false statement for a full disk. Adding
a new `DiagCode` was rejected because `catalog_completeness.rs` matches
`DiagCode` exhaustively, so it would force new user-facing bilingual prose
for a condition that is not a profile diagnostic at all.
```

- [ ] **Step 2: Correct the save-surface note's key count**

At `:278-279` the text reads "Two new Fluent keys, en+de (D47's catalog table)". Two defects: the count contradicts section 2, and the cross-reference is dangling - D47 is the schema ADR and has no catalog table. Replace with:

```markdown
One new Fluent key, en+de (section 2's catalog table). Owner ruling
2026-07-16: the note is a single message; `gui-editor.ftl` carries 43 keys,
as section 2 already states.
```

- [ ] **Step 3: Make section 2's `gui-common.ftl` row concrete**

Section 2's table (`:1737`) currently reads `| D41 | save-failure IpcError codes | gui-common.ftl | codes |`. Replace the row's code column with the two now-named codes:

```markdown
| D41 | save-failure `IpcError` codes (`profile-save-io-failed`, `profile-save-failed`) | `gui-common.ftl` | 2 |
```

- [ ] **Step 4: Record both rulings in section 8**

Section 8 ("What the implementer must not decide") is the list a dispatched implementer reads to know what is pre-decided. Append two bullets:

```markdown
- The writer returns `SaveError`, **not** a `Diagnostic`, and the shell maps
  it to `profile-save-io-failed` / `profile-save-failed` in `gui-common.ftl`.
  No new `DiagCode` and no `diagnostics.ftl` change (owner ruling 2026-07-16,
  `core-124-error-currency-split`).
- The save-surface note is **one** Fluent message, so `gui-editor.ftl` carries
  43 keys: 42 registry labels + 1 note (owner ruling 2026-07-16).
```

- [ ] **Step 5: Verify no superseded text survives**

Run each and confirm the stated expectation:

```bash
grep -n "Result<String, Diagnostic>\|Result<(), Diagnostic>" docs/superpowers/specs/2026-07-15-plan-6-design.md
# Expected: no output. Both signatures now name SaveError.

grep -n "Two new Fluent keys\|D47's catalog table" docs/superpowers/specs/2026-07-15-plan-6-design.md
# Expected: no output. Both defects corrected in step 2.

grep -c "SaveError" docs/superpowers/specs/2026-07-15-plan-6-design.md
# Expected: 6 or more (D41's enum, its two variants in prose, section 8, the mapping note).

grep -n "carries \*\*43\*\*" docs/superpowers/specs/2026-07-15-plan-6-design.md
# Expected: one hit at section 2 - unchanged, and now consistent with D41.
```

- [ ] **Step 6: Commit**

```bash
git add docs/superpowers/specs/2026-07-15-plan-6-design.md
git -c commit.gpgsign=false commit -m "plan 6 design: fold the two owner rulings (SaveError currency, one-key save note)"
```

---

### Task 2: D41 - the core profile writer

**Files:**
- Create: `crates/muxsmith-core/src/profile/save.rs`
- Modify: `crates/muxsmith-core/src/profile/mod.rs` (module doc sentence + `pub mod save;`)
- Test: `crates/muxsmith-core/tests/profile_save.rs` (new)

**Interfaces:**
- Consumes: `profile::load::Format` and `profile::model::Profile` (both exist).
- Produces, for Tasks 4 and 8:
  - `muxsmith_core::profile::save::to_string(profile: &Profile, format: Format) -> Result<String, SaveError>`
  - `muxsmith_core::profile::save::to_file(profile: &Profile, path: &Path) -> Result<(), SaveError>`
  - `muxsmith_core::profile::save::SaveError` with variants `Io(String)` and `Serialize(String)`

**Read first:** design D41 (`:122-300`), for the decision and its rejected alternatives, **as amended by Task 1**.

Binding points, because each is a place an implementer would otherwise improvise:
- The writer lives in **core**, not in `src-tauri` and not in the CLI (`core-85-report-json-dry`: neither surface owns document logic).
- `to_file` picks `Format` from the path extension **exactly as `load::from_file` does** (`profile/load.rs:57-62`): `Some("json")` -> `Format::Json`, everything else -> `Format::Yaml`. A `.json` profile saves as JSON and never silently changes format.
- `yaml_serde::to_string` and `serde_json::to_string_pretty` are the writers; no new dependency.
- `SaveError`, not `Diagnostic` (Global Constraints ruling 1).

- [ ] **Step 1: Write the failing round-trip test**

Create `crates/muxsmith-core/tests/profile_save.rs`:

```rust
//! Spec 8.2 / D41: saving writes canonical YAML from the model. These tests
//! pin the correctness floor (a saved profile reloads equal) and the
//! format-from-extension contract; the default-omission behaviour is D48's
//! and is tested in its own guards.

use std::path::Path;

use muxsmith_core::profile::load::{Format, from_file, from_str};
use muxsmith_core::profile::save::{SaveError, to_file, to_string};

const REFERENCE: &str = include_str!("fixtures/reference.yaml");

#[test]
fn canonical_yaml_round_trips_to_an_equal_model() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Yaml).expect("model serializes");
    let p2 = from_str(&text, Format::Yaml).expect("canonical output re-parses");
    assert_eq!(p, p2, "a saved profile must reload to an equal model (D41)");
}

#[test]
fn canonical_json_round_trips_to_an_equal_model() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Json).expect("model serializes");
    let p2 = from_str(&text, Format::Json).expect("canonical JSON re-parses");
    assert_eq!(p, p2);
}

#[test]
fn to_file_picks_json_from_the_extension_and_never_changes_format() {
    let dir = tempfile::tempdir().unwrap();
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");

    let json_path = dir.path().join("profile.json");
    to_file(&p, &json_path).expect("writes");
    let text = std::fs::read_to_string(&json_path).unwrap();
    assert!(
        text.trim_start().starts_with('{'),
        "a .json path must save as JSON, not YAML: {text}"
    );
    assert_eq!(from_file(&json_path).unwrap(), p);

    let yaml_path = dir.path().join("profile.yaml");
    to_file(&p, &yaml_path).expect("writes");
    let text = std::fs::read_to_string(&yaml_path).unwrap();
    assert!(
        !text.trim_start().starts_with('{'),
        "a .yaml path must save as YAML: {text}"
    );
    assert_eq!(from_file(&yaml_path).unwrap(), p);
}

#[test]
fn an_unwritable_path_is_an_io_error_not_a_panic() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let dir = tempfile::tempdir().unwrap();
    let nope = dir.path().join("no-such-dir").join("profile.yaml");
    match to_file(&p, &nope) {
        Err(SaveError::Io(detail)) => assert!(!detail.is_empty(), "detail carries the io message"),
        other => panic!("expected SaveError::Io, got {other:?}"),
    }
}
```

- [ ] **Step 2: Run it to confirm it fails**

```bash
cargo test -p muxsmith-core --test profile_save
```
Expected: FAIL - `unresolved import muxsmith_core::profile::save`.

- [ ] **Step 3: Write the module**

Create `crates/muxsmith-core/src/profile/save.rs`. `SaveError` derives `Debug` (the test's `{other:?}` needs it) and `PartialEq`; it is a plain enum, not a `std::error::Error`, matching `SettingsError`'s shape in the shell. Give every public item rustdoc - `#![deny(missing_docs)]` is on. The two `to_string` arms are `yaml_serde::to_string(profile)` and `serde_json::to_string_pretty(profile)`, each mapping its error into `SaveError::Serialize(e.to_string())`; `to_file` selects the format from the extension exactly as `load::from_file:57-62` does, then `fs::write`, mapping into `SaveError::Io(e.to_string())`.

- [ ] **Step 4: Wire the module in**

In `crates/muxsmith-core/src/profile/mod.rs`, add `pub mod save;` in alphabetical position (after `pub mod model;`), and extend the module doc's sentence so `save` is named beside `load` - it currently reads "[`load`] parses YAML/JSON into the serde model".

- [ ] **Step 5: Add the dev-dependency if absent**

The test uses `tempfile`. Check first, and only add it if it is missing:

```bash
grep -n "tempfile" crates/muxsmith-core/Cargo.toml
```
If absent, add it under `[dev-dependencies]` with the exact version already used elsewhere in the workspace (`grep -rn "tempfile" crates/*/Cargo.toml`) - do not invent a version.

- [ ] **Step 6: Run the tests**

```bash
cargo test -p muxsmith-core --test profile_save
```
Expected: PASS, 4 tests.

- [ ] **Step 7: Full gate, then commit**

Run the nine-part gate. Then:

```bash
git add crates/muxsmith-core/src/profile/save.rs crates/muxsmith-core/src/profile/mod.rs crates/muxsmith-core/tests/profile_save.rs crates/muxsmith-core/Cargo.toml
git -c commit.gpgsign=false commit -m "core: profile::save writes canonical YAML/JSON from the model (D41)"
```

---

### Task 3: D46 - keyword domains from one constant set

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs` (4 `KEYWORDS` consts, 4 `schema_with` projections)
- Modify: `crates/muxsmith-core/src/profile/validate.rs` (guards at `:105`, `:129`, `:149`, `:166`; the 4 `allowed` params)
- Test: `crates/muxsmith-cli/tests/cli_schema.rs`, `crates/muxsmith-core/tests/validate_semantics.rs`

**Interfaces:**
- Produces, for Task 5's emitter: `FilenameCfg::KEYWORDS`, `SourceCfg::KEYWORDS`, `ChaptersCfg::KEYWORDS`, `TitleCfg::KEYWORDS`, each `pub const &'static [&'static str]`.
- Consumes: nothing from other tasks.

**Read first:** design D46 (`:1103-1224`) in full. It carries the exact constant values, the placement rationale (`profile::model`, **not** `capability`), the `domain_hint` refactor, and the empirically verified schemars output.

Binding points:
- The `Keyword(String)` arm **keeps its `String`**. Do not "fix" it by typing the arm - that destroys the `InvalidKeyword` diagnostic and replaces it with serde's untagged error. D46 records the full steelman for the typed arm; it lost, and the reason is recorded so it is not re-litigated.
- The schema override emits **`enum`, not `oneOf`+`const`** (D46's rejected alternative).
- The `allowed` param goes through the existing `domain_hint` (`profile/validate.rs:430-437`). The four hand-typed strings today are `"primary"`, `"keep"`, `"keep, drop"`, `"keep, clear"`, and the const-derived values are byte-identical, so **the CLI snapshots must not move**. Any diff in `crates/muxsmith-cli/tests/snapshots/` means the refactor is wrong.

- [ ] **Step 1: Write the failing schema test**

The design measured the target shape empirically (`:1112-1120`). Add to `crates/muxsmith-cli/tests/cli_schema.rs`:

```rust
#[test]
fn keyword_domains_project_as_closed_enums_not_bare_strings() {
    let schema = schema_json(); // the file's existing helper that runs `muxsmith schema`
    let cases = [
        ("FilenameCfg", vec!["keep"]),
        ("SourceCfg", vec!["primary"]),
        ("ChaptersCfg", vec!["keep", "drop"]),
        ("TitleCfg", vec!["keep", "clear"]),
    ];
    for (ty, expected) in cases {
        let branches = schema["$defs"][ty]["anyOf"]
            .as_array()
            .unwrap_or_else(|| panic!("{ty} must still project anyOf (D46 narrows the string branch only)"));
        let string_branch = branches
            .iter()
            .find(|b| b["type"] == "string")
            .unwrap_or_else(|| panic!("{ty} must keep a string branch"));
        let got: Vec<&str> = string_branch["enum"]
            .as_array()
            .unwrap_or_else(|| panic!("{ty}'s string branch must carry an enum, not a bare string type"))
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(got, expected, "{ty} keyword domain");
    }
}
```

Adapt the helper call to whatever `cli_schema.rs` already uses to obtain the schema JSON - read the file first; do not add a second way to run the command.

- [ ] **Step 2: Run it to confirm it fails**

```bash
cargo test -p muxsmith-cli --test cli_schema keyword_domains
```
Expected: FAIL - the string branch has no `enum` key today (design `:70-77` measured it as a bare `{type: "string"}`).

- [ ] **Step 3: Add the four constant sets**

In `profile/model.rs`, beside each enum it belongs to, exactly as design `:1156-1161` states. The shape deliberately copies the house pattern at `capability/mod.rs:55` (`pub static &[&str]` closed domain read by a lookup fn) and `:125-129` (`CODEC_KIND_NAMES` derived from `CODEC_KINDS` "so the two can never drift").

- [ ] **Step 4: Point the validate guards at the constants**

Replace the four bare-literal match guards (`validate.rs:105`, `:129`, `:149`, `:166`) with `k if FilenameCfg::KEYWORDS.contains(&k.as_str())` and its three siblings, and replace the four hand-typed `allowed` strings with `domain_hint` calls over the same constants.

- [ ] **Step 5: Add the schema projections**

Add one `schema_with` function per enum and attach it to the `Keyword` arm, per D46's measured output (`:1112-1116`). The variant's doc comment must survive as the branch `description`, merged rather than replaced - the design measured that it does.

- [ ] **Step 6: Verify the diagnostic still reaches the keyword**

`InvalidKeyword` must stay reachable with its `found` and `allowed` params - that is the whole reason the arm keeps its `String`. Confirm with the design's own probe (`:1118-1120`): `'kepp'` deserializes to `Keyword("kepp")` and validate rejects it. Add a test to `crates/muxsmith-core/tests/validate_semantics.rs` asserting a misspelled keyword yields `DiagCode::InvalidKeyword` with `allowed` equal to the const-derived hint (e.g. `"keep, drop"` for `chapters`).

- [ ] **Step 7: Run the tests and prove the snapshots did not move**

```bash
cargo test -p muxsmith-cli --test cli_schema
cargo test -p muxsmith-core --test validate_semantics
cargo test --workspace
git diff --exit-code crates/muxsmith-cli/tests/snapshots/
```
Expected: all pass, and the `git diff` exits **0** with no output. A moved snapshot means the `domain_hint` refactor changed user-visible output, which D46 says it must not.

- [ ] **Step 8: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/src/profile/validate.rs crates/muxsmith-cli/tests/cli_schema.rs crates/muxsmith-core/tests/validate_semantics.rs
git -c commit.gpgsign=false commit -m "core: keyword domains project into the schema from one constant set (D46)"
```

---

### Task 4: D48 - a canonical save omits default-valued fields

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs` (17 fields x 2 attributes, 4 predicates)
- Create: `crates/muxsmith-core/tests/fixtures/all-non-default.yaml`
- Test: `crates/muxsmith-core/tests/profile_save.rs` (extend Task 2's file)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (spec 8.2 amendment)

**Interfaces:**
- Consumes: Task 2's `save::to_string`.
- Produces: `Profile`'s serialized form omits default-valued fields. No API change.

**Read first:** design D48 (`:1336-1722`) in full, especially:
- the **17-row table** at `:1517-1535` - location, field, serde default, predicate. This is the authoritative enumeration; work from it row by row.
- the mechanism at `:1442-1508`: **every predicate calls the very function the field's own `default` attribute names**.
- the schemars interaction at `:1557-1637`: each of the 17 also carries `#[schemars(extend("default" = <derived>))]`, derived from that same function.

Binding points, each one a place where getting it wrong loses user data silently:
- **A generic `is_default` is correct for 13 of the 17 and silently destroys data on the other 4.** Two of those four fail to compile (`FilenameCfg` and `SourceCfg` have no `Default` impl, so `is_default<T: Default>` cannot instantiate - `E0277`). **Two compile and are the hazard**: `TracksCfg.unmatched` (default `drop_policy()`, so a naive predicate omits `unmatched: keep` and it reloads as `drop` - destroying the owner-ruled-legal `core-83` passthrough profile) and `Input.recursive` (default `default_true()`, so a naive predicate omits `recursive: false` and it reloads inverted).
- Omission is implemented with `skip_serializing_if` **on the derives**, not by post-processing the tree in `save::to_string` (D48's rejected alternative: a walker cannot tell `tracks.unmatched` from `attachments.unmatched`, whose defaults are opposite).
- The three struct-valued fields derive to `"default": {}` and **that is accepted, not patched with a literal**.
- **Both guards ship with the serializer, not after it.** Guard 2 is not optional and is **not** to be argued out at the keyboard on the grounds that the derivation makes it vacuous - that argument is already recorded and answered in D48 `:1657-1701`, and `proc-proposed-safeguard-stays` holds the guard in until it exists and can be measured. If you believe it cannot fail, that belief is the trigger for design trigger 2, not for deleting the test.

- [ ] **Step 1: Write guard 1 - round-trip fidelity on an all-non-default fixture**

Create `crates/muxsmith-core/tests/fixtures/all-non-default.yaml`: a profile setting **every one of the 17 fields** to a value that is **not** its default, per the table at `:1517-1535`. This is what catches a predicate that skips a non-default value, and it catches it for all 17 at once. Then extend `crates/muxsmith-core/tests/profile_save.rs`:

```rust
const ALL_NON_DEFAULT: &str = include_str!("fixtures/all-non-default.yaml");

/// D48 guard 1: every one of the 17 defaulted fields set to a NON-default
/// value must survive a save/load round trip. A predicate that skips a value
/// which is not the default silently destroys it - the core-83 passthrough
/// class of bug (`unmatched: keep` reloading as `drop`).
#[test]
fn all_non_default_fields_survive_the_round_trip() {
    let p = from_str(ALL_NON_DEFAULT, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Yaml).expect("serializes");
    let p2 = from_str(&text, Format::Yaml).expect("re-parses");
    assert_eq!(p, p2, "a non-default value must never be omitted (D48 guard 1)");
}

/// The sharpest instance, called out because it is an owner-ruled-legal
/// profile (`core-83`) that a naive `is_default` turns into a NoTrackRules
/// error: zero rules plus `unmatched: keep` is a pure-passthrough remux.
#[test]
fn the_core83_passthrough_profile_survives_a_save() {
    let y = "profile_version: 1\ninput: { pattern: 'E(\\d+)', extensions: [mkv] }\ntracks:\n  unmatched: keep\n  rules: []\n";
    let p = from_str(y, Format::Yaml).expect("parses");
    let text = to_string(&p, Format::Yaml).expect("serializes");
    assert!(
        text.contains("unmatched: keep"),
        "tracks.unmatched defaults to DROP, so `keep` is not a default and must be written: {text}"
    );
    assert_eq!(from_str(&text, Format::Yaml).unwrap(), p);
}
```

- [ ] **Step 2: Run guard 1 to confirm it fails for the right reason**

```bash
cargo test -p muxsmith-core --test profile_save all_non_default
```
Expected: FAIL - the fixture does not exist yet, or (once it does) it passes trivially because nothing is skipped yet. Both are expected at this point; guard 1's real value is that it goes **red** if step 3 uses a naive predicate. Note that in the log.

- [ ] **Step 3: Add the predicates and the 17 field attributes**

Work the table at `:1517-1535` row by row. Four predicates beyond the generic one, per `:1447-1457`. Every row gets `#[serde(default...)]`, `skip_serializing_if`, and `#[schemars(extend("default" = ...))]`, and **all three name the same function**.

- [ ] **Step 4: Prove guard 1 catches the naive predicate**

Before moving on, deliberately break one row and confirm the guard fires - this is the evidence that guard 1 works, and it costs thirty seconds:

```bash
# temporarily change TracksCfg.unmatched's skip_serializing_if to the generic "is_default"
cargo test -p muxsmith-core --test profile_save
# Expected: FAIL - the_core83_passthrough_profile_survives_a_save goes red.
# Then revert to is_drop_policy and confirm green again.
```
Record the observed failure in your report. If it does **not** go red, guard 1 is not testing what D48 says it tests - stop and return NEEDS_CONTEXT.

- [ ] **Step 5: Write guard 2 - schema-default honesty**

A table test asserting, for each of the 17 fields, that the schema's `default` equals `serde_json::to_value` of that field's serde default. Follow the house's existing table-test shape - `capability/mod.rs`'s `settable_maps_to_mkvmerge_options` asserts a `const EXPECTED` table against the real thing, length first, then row by row - rather than inventing a pattern. The three struct-valued fields (`Profile.output`, `Profile.attachments`, `Profile.tags`) expect `{}`, per `:1612-1637`.

- [ ] **Step 6: Amend spec 8.2**

In `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, the profile-editor bullet currently says only "open/save YAML". State that saving writes canonical YAML from the model and does not preserve comments, key order or formatting (D41), and that fields left at their default are not written back (D48). Then run the self-contradiction sweep `proc-04-spec-wins` mandates. Design section 3 (`:1797-1813`) already ran it and found spec 4.1's reference example stays correct under this amendment because it, too, omits `source` and `optional`; confirm that still holds and do not re-derive it.

- [ ] **Step 7: Run everything**

```bash
cargo test -p muxsmith-core --test profile_save
cargo test --workspace
git diff --exit-code crates/muxsmith-cli/tests/snapshots/
```
Expected: all green; snapshots unmoved.

- [ ] **Step 8: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/tests/profile_save.rs crates/muxsmith-core/tests/fixtures/all-non-default.yaml docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "core: a canonical save omits default-valued fields, with both guards (D48)"
```

---

### Task 5: D44 - ts-rs bindings, committed, with a CI drift check

**Files:**
- Modify: `crates/muxsmith-core/Cargo.toml` (`ts-rs` optional dep + `ts` feature)
- Create: `.cargo/config.toml` (`[env]` block)
- Modify: `crates/muxsmith-core/src/profile/model.rs`, `crates/muxsmith-core/src/profile/match_expr.rs` (cfg_attr TS derives on the 20 types)
- Create: `crates/muxsmith-core/tests/ts_export.rs` (the export test + the keywords emitter)
- Create (committed, generated): `src/bindings/profile.ts`, `src/bindings/keywords.ts`
- Modify: `.github/workflows/ci.yml` (new Linux-leg drift step)

**Interfaces:**
- Consumes: Task 3's four `KEYWORDS` constants.
- Produces, for Tasks 9-11: `src/bindings/profile.ts` (the 20 model types) and `src/bindings/keywords.ts` (`FILENAME_KEYWORDS`, `SOURCE_KEYWORDS`, `CHAPTERS_KEYWORDS`, `TITLE_KEYWORDS` as `as const` arrays).

**Read first:** design D44 (`:498-682`) in full - it carries the `[env]` block, the emitter, the drift step, and the measured evidence for each.

Binding points:
- Generation is `cargo test -p muxsmith-core --features ts`, **not** an xtask, and **the reason is feature unification, not taste**: xtask would need `muxsmith-core = { features = ["ts"] }`, and Cargo unifies features across workspace members within one invocation, so `cargo build --workspace` would enable `ts` for every consumer of core and put `ts-rs` into the shipped cli and src-tauri builds. `-p muxsmith-core --features ts` cannot leak that way.
- Bindings are **committed, not built** (`core-06-schema-build-time-extraction` already mandates a committed generated artifact).
- `TS_RS_LARGE_INT = "number"` is **mandatory**: without it `Scalar::Int(i64)` maps to `bigint`, which does not survive `JSON.stringify` and would break the IPC wire at the model's most-used point.
- `export_to = "profile.ts"` without a trailing slash names a **file**, so all 20 types land in one file with no cross-imports.

- [ ] **Step 1: Add the dependency and feature**

In `crates/muxsmith-core/Cargo.toml`: `ts-rs = { version = "12.0.1", optional = true }` and a `ts = ["dep:ts-rs"]` feature. Before writing the version, re-verify it against the registry rather than trusting this plan (`proc-07-verify-against-source`):

```bash
curl -s https://crates.io/api/v1/crates/ts-rs | python3 -c "import json,sys; print(json.load(sys.stdin)['crate']['max_stable_version'])"
```
Expected: `12.0.1`. If it differs, that is a NEEDS_CONTEXT, not a silent bump - `ci-10-pin-everything` binds.

- [ ] **Step 2: Prove the default build stays clean**

The whole isolation claim rests on this. Measure it now, before the derives exist, and again in step 7:

```bash
cargo tree -p muxsmith-core | grep -c ts-rs
```
Expected: `0`.

- [ ] **Step 3: Create `.cargo/config.toml`**

```toml
[env]
TS_RS_EXPORT_DIR = { value = "src/bindings", relative = true }
TS_RS_LARGE_INT = "number"
```

- [ ] **Step 4: Add the derives**

`#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]` on each of the 20 model types - the 13 structs in D45's table (`:768-782`) plus the 7 enums (`FilenameCfg`, `SourceCfg`, `ChaptersCfg`, `TitleCfg`, `CollisionPolicy`, `KeepDrop`, `Scalar`). That set is the whole model reachable from `Profile`, with no residue.

- [ ] **Step 5: Write the export test and the keywords emitter**

Create `crates/muxsmith-core/tests/ts_export.rs`. ts-rs's `#[ts(export)]` writes `profile.ts` as a test side effect; the emitter for `keywords.ts` is ~12 lines and the design gives it verbatim at `:556-570`. It reads `TS_RS_EXPORT_DIR` so both artifacts share one destination. `ts-rs` exports **types, not values**, which is why the four keyword arrays need an emitter at all.

- [ ] **Step 6: Generate and commit the bindings**

```bash
cargo test -p muxsmith-core --features ts
ls -1 src/bindings/
```
Expected: `keywords.ts` and `profile.ts`. Inspect `profile.ts` and confirm `Scalar` emits `boolean | number | number | string` (the duplicate `number` is cosmetic and expected) - a `bigint` anywhere means `TS_RS_LARGE_INT` is not reaching the build.

- [ ] **Step 7: Re-prove the isolation**

```bash
cargo tree -p muxsmith-core | grep -c ts-rs
```
Expected: still `0` - the derives are behind the feature.

- [ ] **Step 8: Add the CI drift step**

In `.github/workflows/ci.yml`, on the **Linux leg only** (matching the existing `check:i18n` and Playwright gating):

```yaml
      - name: TS bindings are not stale
        if: runner.os == 'Linux'
        run: |
          cargo test -p muxsmith-core --features ts
          git diff --exit-code src/bindings/
```

Add a comment recording the step's one known hole, so it is not rediscovered as a bug: `git diff --exit-code` does not see a **new untracked** file (measured, D44 `:614-631`), so the gate catches a *stale* committed artifact from the first commit onward but cannot catch a never-committed first-generation one. That hole is closed elsewhere - a missing `keywords.ts` fails the TypeScript build on the registry's import of it, which `pnpm build` runs on every leg. `git status --porcelain` would close it directly and is deliberately **not** adopted: it would also fire on unrelated untracked files and turn every CI leg into a working-tree cleanliness assertion.

- [ ] **Step 9: Prove the drift check actually catches drift**

```bash
printf '\n// drift\n' >> src/bindings/keywords.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"
# Expected: exit=1  (the gate fires)
git checkout src/bindings/keywords.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"
# Expected: exit=0
```

- [ ] **Step 10: Full gate, then commit**

```bash
git add crates/muxsmith-core/Cargo.toml Cargo.lock .cargo/config.toml crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/src/profile/match_expr.rs crates/muxsmith-core/tests/ts_export.rs src/bindings/profile.ts src/bindings/keywords.ts .github/workflows/ci.yml
git -c commit.gpgsign=false commit -m "core: ts-rs generates the wire types behind a ts feature, committed + CI drift-checked (D44)"
```

---

### Task 6: D43 - apply_suggestion in core

**Stream B** (`.worktrees/plan6-b`), parallel with stream A - touches `planner.rs` only.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (`StructuredEdit` gains `Deserialize` at `:201`; new `apply_suggestion` + `ApplyError`)
- Test: `crates/muxsmith-core/tests/suggestions.rs`

**Interfaces:**
- Produces, for Task 8: `muxsmith_core::planner::apply_suggestion(profile: &Profile, config_path: &str, edit: &StructuredEdit) -> Result<Profile, ApplyError>`, and `StructuredEdit` as a **bidirectional** wire type.
- Consumes: nothing from other tasks.

**Read first:** design D43 (`:410-495`) in full.

Binding points:
- `StructuredEdit` gains `Deserialize`. **`Suggestion` and `DiagCode` do not.** Making `DiagCode` constructible from the frontend would let the shell synthesize diagnostics, which is exactly what `core-37-prose-free-core` (count 11, the most-reinforced entry in the house files) exists to prevent.
- **Do not write a new applier.** The brief's original "hoist `apply_edit_to_first_rule` from `tests/suggestions.rs:95`" is refuted - correction #1 (`:41`) establishes it is a fixture generator that takes no `Profile` and mutates nothing. But the real reuse targets are binding: `rule_index_of` (`planner.rs:2032`) parses `config_path` - do not re-parse it; and the engine's own `with_rule_match` narrowing helper must be reused, not re-implemented, because `core-44-suggestion-no-clobber` records that it must use `or_insert` semantics and never overwrite an existing `exact`/`substring` key. An independently written applier reproduces that bug (Bug C), and apply must narrow through the **same** code the engine simulated with, or `core-03-suggestion-verified-edit`'s guarantee ("an applied suggestion survives the next dry run") is void.
- `core-33-suggestion-narrow-only` bounds what apply may do: **narrow the conflicted rule's match only.** Never reorder, never touch other rules, never relax.
- **Apply does not validate.** It returns the mutated model; the editor's existing round-trip validates. No compound apply-and-validate command.
- `config_path` is the narrow token it is - only ever `tracks[<N>].match` (`:1437`, `:1516`) - and is **not** `Diagnostic.config_path`, which is a different, general field.

- [ ] **Step 1: Write the failing tests**

In `crates/muxsmith-core/tests/suggestions.rs`:

```rust
/// D43: apply narrows the named rule's match and returns a new model.
/// `core-03`: the applied result is what the engine simulated, so it must
/// come through the engine's own narrowing helper.
#[test]
fn apply_narrows_the_named_rule_only() {
    // Build a profile with two rules; apply an edit to tracks[1].match.
    // Assert: rule 1's match gained the edit's key; rule 0 is byte-identical
    // to before; rule order is unchanged (core-33: narrow only, never reorder).
}

/// `core-44`: never overwrite an existing exact/substring key (or_insert
/// semantics). A clobbering edit is a no-op, not a widened match - the Bug C
/// regression, which an independently written applier reproduces.
#[test]
fn apply_never_clobbers_an_existing_match_key() {
}

/// A config_path naming no rule is a frontend bug: surfaced as an error,
/// never a silent no-op.
#[test]
fn an_unparseable_or_out_of_range_config_path_is_an_error() {
}

/// The grammar stays closed at the boundary: an unknown `kind` tag fails
/// deserialization rather than reaching the applier.
#[test]
fn an_unknown_structured_edit_kind_fails_to_deserialize() {
}
```

Fill each body against the fixtures already in the file - read it first and reuse its existing profile builders rather than adding new ones (`testing-support-helpers`: duplicating a helper within a crate is a defect).

- [ ] **Step 2: Run to confirm they fail**

```bash
cargo test -p muxsmith-core --test suggestions apply
```
Expected: FAIL - `apply_suggestion` does not exist.

- [ ] **Step 3: Implement**

Add `Deserialize` to `StructuredEdit` (`:201`), and `ApplyError` carrying the one failure this can have: `rule_index_of` returning `None`, or an index past the end of `tracks.rules`. Implement `apply_suggestion` reusing `rule_index_of` and `with_rule_match`.

- [ ] **Step 4: Run the tests**

```bash
cargo test -p muxsmith-core --test suggestions
```
Expected: PASS.

- [ ] **Step 5: Prove the no-clobber reuse is real, not asserted**

`core-44` is the one thing here that a plausible-looking reimplementation gets wrong silently. Confirm your `apply_suggestion` calls `with_rule_match` rather than merging maps itself:

```bash
grep -n "with_rule_match\|extend(" crates/muxsmith-core/src/planner.rs | sed -n '/apply_suggestion/,$p'
```
A `BTreeMap::extend` anywhere in the applier is the Bug C shape and fails review.

- [ ] **Step 6: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/planner.rs crates/muxsmith-core/tests/suggestions.rs
git -c commit.gpgsign=false commit -m "core: apply_suggestion narrows through the engine's own helper (D43)"
```

---

### Task 7: D47 - the schema as a supported user artifact

**Stream C** (`.worktrees/plan6-c`), parallel with streams A and B - docs only.

**Files:**
- Modify: `README.md` (new section)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (spec 8.1 + spec 8.4 exception)

**Interfaces:**
- Consumes: nothing. Produces: nothing code-facing.

**Read first:** design D47 (`:1227-1332`).

Binding points:
- The README documents the **editor-settings binding**, not the in-file modeline, and **the reason is D41**: the modeline `# yaml-language-server: $schema=...` is a YAML comment, and a canonical save does not preserve comments - so a user who wires up autocompletion with a modeline and then saves once from the GUI loses their schema binding silently, with no message. The README states that consequence explicitly rather than leaving the user to find it.
- The schema's English `description` fields (Rust doc comments) become user-facing under D47. This is an **accepted, deliberate boundary**, not an oversight: the schema documents a *file format*, the same category as the README and the spec, both English-only by design. Spec 8.4 gains an explicit exception entry so a future reviewer does not read it as a standing violation - which is exactly what would otherwise happen.
- Do **not** add SchemaStore publication or a GUI startup write; both are rejected in D47 and parked behind triggers.

- [ ] **Step 1: Write the README section**

Document `muxsmith schema > muxsmith-profile.schema.json`, the VS Code `yaml.schemas` mapping over a glob such as `*.muxsmith.yaml`, and the equivalent `lspconfig` settings block for Neovim/Helix. State the modeline consequence. Keep the README's established sell-tone register (the case-scoped exception recorded in the ROADMAP's README entry), not the writeup register.

- [ ] **Step 2: Amend spec 8.1**

`muxsmith schema` is a supported user feature, not only a debug aid; cross-reference the README section.

- [ ] **Step 3: Amend spec 8.4**

Add the JSON Schema's `description` fields to the accepted-v1-exceptions list, with the file-format-documentation rationale.

- [ ] **Step 4: Run the self-contradiction sweep**

`proc-04-spec-wins` mandates it after any amendment. Design section 3 (`:1797-1813`) records the sweep as already run and complete for these amendments; confirm it still holds against current spec text and do not re-derive it.

- [ ] **Step 5: Gate the docs**

```bash
pnpm lint
grep -rn "—\|–\|“\|”\|…" README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```
Expected: the grep returns **no output** (typography constraint: ASCII only).

- [ ] **Step 6: Commit**

```bash
git add README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "docs: the JSON schema is a supported hand-authoring artifact (D47)"
```

---

## Wave 2

Streams A, B and C merge to master first, gate green after each merge. Then:

### Task 8: D42 - the editor's IPC surface

**Files:**
- Modify: `src-tauri/src/lib.rs` (3 new commands + `apply_suggestion` + `ProfileDocument`; `invoke_handler` at `:440-452`)
- Modify: `src-tauri/src/error.rs` (`From<SaveError>`, `From<ApplyError>`)
- Modify: `locales/en/gui-common.ftl`, `locales/de/gui-common.ftl`
- Test: `src-tauri/src/lib.rs` unit tests, `src-tauri/src/error.rs` unit tests

**Interfaces:**
- Consumes: Task 2's `profile::save::{to_file, SaveError}`; Task 6's `planner::{apply_suggestion, ApplyError}`; the existing `profile::validate::config_diagnostics` (`profile/validate.rs:193`).
- Produces, for Tasks 9-12: four commands and the `ProfileDocument` wire shape.

| command | signature |
|---|---|
| `load_profile` | `async fn load_profile(path: String) -> Result<ProfileDocument, IpcError>` |
| `save_profile` | `async fn save_profile(path: String, profile: Profile) -> Result<(), IpcError>` |
| `validate_profile_model` | `async fn validate_profile_model(profile: Profile) -> Result<serde_json::Value, IpcError>` |
| `apply_suggestion` | `async fn apply_suggestion(profile: Profile, config_path: String, edit: StructuredEdit) -> Result<Profile, IpcError>` |

**Read first:** design D42 (`:303-407`) and D43's command paragraph (`:417-421`).

Binding points:
- **`validate_profile(path)` is kept, not changed, not retargeted, not renamed, not removed.** It has a live consumer at `src/views/BatchView.vue:118`, and the batch view has no model to send. The two commands are not redundant: one validates a file the user picked by path, the other validates a model the user is editing. Both funnel into the same `config_diagnostics`, so no logic is duplicated (spec 7).
- All four are `async` on `on_blocking` (`:73-79`), but for **two different reasons**, and the distinction is the thing an implementer gets wrong by pattern-matching. `load_profile`/`save_profile` touch the disk. **`validate_profile_model` does not touch the disk at all** - `config_diagnostics` is pure (`validate.rs:20-21`). It is on `on_blocking` because it is CPU-bound work on every keystroke: it compiles every regex and parses every template, and Tauri 2 runs a non-`async` command on the main thread, so a plain `fn` would stall the webview on each edit. "Touches the disk" is not the criterion; "could stall the webview" is - which is why `get_settings`/`set_settings` are deliberately non-async despite real file I/O.
- `ProfileDocument` is `{ profile: Option<Profile>, diagnostics: Vec<Diagnostic> }`, serialized through the existing `report::json` document machinery so its `diagnostics` array is **byte-identical in shape** to what `validate_profile` already returns (`core-85-report-json-dry`). On a `ParseError` the profile is absent and the single diagnostic explains why. One round trip, because the editor needs both and a second call would let them disagree.
- The `Err` case stays what it is everywhere else in this file: the blocking task itself panicking. **Expected failures are diagnostics in the document, not `Err`.**

- [ ] **Step 1: Write the failing error-mapping tests**

In `src-tauri/src/error.rs`'s test module, mirroring the existing `settings_errors_map_to_distinct_codes` shape:

```rust
#[test]
fn save_errors_map_to_distinct_codes() {
    let io: IpcError = SaveError::Io("permission denied".into()).into();
    let ser: IpcError = SaveError::Serialize("bad float".into()).into();
    assert_eq!(io.code, "profile-save-io-failed");
    assert_eq!(io.params["detail"], "permission denied");
    assert_eq!(ser.code, "profile-save-failed");
    assert_ne!(io.code, ser.code);
}

#[test]
fn apply_error_maps_to_the_rule_not_found_code() {
    let e: IpcError = ApplyError::RuleNotFound("tracks[9].match".into()).into();
    assert_eq!(e.code, "suggestion-rule-not-found");
}
```

Adapt `ApplyError`'s variant name/shape to what Task 6 actually produced - read `planner.rs` first rather than assuming this plan's spelling.

- [ ] **Step 2: Run to confirm they fail**

```bash
cargo test -p muxsmith --lib error
```
Expected: FAIL - the `From` impls do not exist. (Confirm the crate name from `src-tauri/Cargo.toml` first.)

- [ ] **Step 3: Implement the mappings and the commands**

Add both `From` impls to `error.rs`, then `ProfileDocument` and the four commands to `lib.rs`, registering each in the `invoke_handler` (`:440-452`).

- [ ] **Step 4: Add the catalog entries, bilingual**

Three new `IpcError` codes across `locales/{en,de}/gui-common.ftl`: `profile-save-io-failed`, `profile-save-failed`, `suggestion-rule-not-found`. Every `IpcError` code in this tree lives in `gui-common.ftl` today (`mkvmerge-spawn-failed`, `settings-io-failed`, `internal-task-failed`), so this follows the existing split rather than inventing one. Each carries `detail` where the mapping sets it. Follow the de catalog header's register rules.

- [ ] **Step 5: Run the tests**

```bash
cargo test -p muxsmith
pnpm check:i18n
```
Expected: PASS; `check:i18n` green (check 3 enforces en/de parity on the new keys).

- [ ] **Step 6: Full gate, then commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/error.rs locales/en/gui-common.ftl locales/de/gui-common.ftl
git -c commit.gpgsign=false commit -m "shell: load/save/validate-model/apply commands and their error codes (D42, D43)"
```

---

## Wave 3

Task 8 merges to master, gate green. Then the frontend, serial within stream E (`.worktrees/plan6-e`): Task 9 -> Task 10 -> Task 11. Task 12 needs only Task 8 and may run as its own stream in parallel with 9-11.

### Task 9: D45 - the registry data layer, catalogs, and the i18n gate

**Files:**
- Create: `src/editor/fieldSpec.ts` (the `FieldSpec`/`FieldWidget`/`RegistryName` types)
- Create: `src/editor/registries.ts` (the 13 registries + the option arrays + the completeness guards)
- Create: `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl`
- Modify: `scripts/check-i18n.mjs` (the `LABEL_KEY_RE` scan)
- Test: `e2e/catalogs.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 5's `src/bindings/profile.ts` and `src/bindings/keywords.ts`.
- Produces, for Tasks 10-11: the 13 exported registries, `FieldSpec`, `FieldWidget`, `RegistryName`, `COLLISION_POLICIES`, `KEEP_DROP`.

**Read first:** design D45 (`:685-1099`) in full. It carries the type definitions (`:694-706`, `:806-819`), the 13-struct table (`:768-782`), the **complete 43-field widget table** (`:848-894`), the option-array guard shape (`:905-909`), and the `check-i18n` fix (`:1082-1091`).

Binding points:
- **All 13 structs get a registry and all 7 enums get a `never` arm.** No subset: the registry's entire value is total coverage, and registering only a subset reintroduces the exact silent-absence failure the mechanism exists to close.
- **42 of 43 fields are `EditableField`; the one `FixedField` is `Profile.profile_version`**, which spec 4 pins at 1. Do **not** reach for `Omit<Profile, "profile_version">` - that silently disables the forcing function for that key forever.
- `FieldWidget` has **10 variants**, closed. `fixed` is **not** one of them: it is the other half of the `FieldSpec` union.
- **The four keyword arrays are imported from `src/bindings/keywords.ts`, never hand-written.** `COLLISION_POLICIES` and `KEEP_DROP` are declared here with the `satisfies` completeness guard, because TS can see their unions; the keyword domains are not in the TS type at all (the untagged enums project to `Block | string`), which is why they are generated instead.
- `gui-editor.ftl` carries exactly **43** keys: 42 labels + 1 save-surface note (Global Constraints ruling 2).
- Widget facets add **no** keys: `select` and `keywordOrBlock` render their options from the domain arrays, and those are profile-format tokens (`keep`, `drop`, `error`, `primary`), not prose - the same call D39 made for the `allowed` param.
- Three widget choices are settled by evidence, not by the Rust type, and are the ones most likely to be got wrong: `optionalFlag` is a checkbox whose off-state is **absence** (not a tri-state - `validate.rs:466-472` rejects `Some(false)`); `TextSyntax` has **four** values because `Locator.match_pattern` is a template in *regex* mode, a genuinely third thing from `Input.pattern`'s regex and `TemplateBlock.template`'s literal-mode template; `propertyMap.properties` is `matchable | settable` because `exact` and `changes` offer **different** domains (`codec_kind` is matchable-only).

- [ ] **Step 1: Write the failing registry-completeness proof**

The registry's whole value is that it fails the **build**, not a test. Prove that first, with a deliberate temporary break:

```bash
# after step 2 exists, delete one entry from outputFields and run:
pnpm build
# Expected: error TS2741: Property 'on_collision' is missing in type '{...}'
#           but required in type 'Record<keyof OutputCfg, FieldSpec>'
# Restore it and confirm green. Record the observed error in your report.
```
Note: two or more missing keys report **TS2739** listing them all. Same check, two messages; the design does not depend on which fires.

- [ ] **Step 2: Write the types and the registries**

`src/editor/fieldSpec.ts` gets `EditableField`, `FixedField`, `FieldSpec`, `TextSyntax`, `FieldWidget`, `RegistryName` per `:694-706` and `:806-819`. `src/editor/registries.ts` gets the 13 registries, filled from the **43-row table at `:848-894`** - work it row by row; it is complete and it is the contract.

`reorderable` is semantic, not taste: `tracks.rules` is output track order and `attachments.rules` resolves first-match-wins in list order, so both reorder; `any` (logical OR) and `not` (logical NOR) carry no order, so neither does.

- [ ] **Step 3: Add the option arrays with their completeness guards**

Per `:905-909`, for `COLLISION_POLICIES` and `KEEP_DROP`. This is D45's own `never`-arm principle applied to a value list, so it is the house rule of this ADR rather than a new idea.

- [ ] **Step 4: Write the catalogs**

`locales/en/gui-editor.ftl` and `locales/de/gui-editor.ftl`, 43 keys each: one `labelKey` per `EditableField` plus `editor-save-note`. The note's content is fixed by D41 and must name the **whole** behaviour, not just comments - comments are the smaller half:

```
editor-save-note = Saving rewrites the file from the model: comments, key order and formatting are not preserved, and fields left at their default are not written back.
```

A note naming only comments would understate what the user is about to see and would be read as a defect report the first time someone diffs their profile. Write the de counterpart per the de header's register rules.

- [ ] **Step 5: Close check 1 of the i18n gate**

Per design `:1076-1091`, add a second scanning regex alongside `CALL_RE` in `scripts/check-i18n.mjs`:

```js
const LABEL_KEY_RE = /labelKey:\s*(['"])([^'"]*)\1/g;
```

applied to the same `src/**/*.{vue,ts}` sweep, with every match added to `literalCallIds` and pushed to `missing` when it is not a known catalog id. Match the script's own deliberate line-based approach - it is **not** a Fluent parser (`:102-120`), and this is not the place to make it one.

**Check 2 needs no change** and check 3 is untouched: check 2 already counts a key as used when it appears anywhere in `src/` as a quoted literal, single- or double-quoted (`:191-198`, the test at `:193`), which is precisely the registry's `labelKey: "editor-..."` shape - the same mechanism that already passes `src/jobRowState.ts:44-55`'s identical map-to-Fluent-key pattern. Net effect: registry label keys become **hard-gated**, which is a net gain over today, not a trade.

- [ ] **Step 6: Prove the new scan fires**

```bash
pnpm check:i18n
# Expected: green.
# Now break it deliberately:
#   change one labelKey to "editor-does-not-exist"
pnpm check:i18n
# Expected: FAILS naming editor-does-not-exist. Revert and confirm green.
```

- [ ] **Step 7: Full gate, then commit**

```bash
git add src/editor/fieldSpec.ts src/editor/registries.ts locales/en/gui-editor.ftl locales/de/gui-editor.ftl scripts/check-i18n.mjs e2e/catalogs.spec.ts
git -c commit.gpgsign=false commit -m "gui: the field registry, its catalogs, and the label-key i18n gate (D45)"
```

---

### Task 10: D45 - the widget components

**Files:**
- Create: `src/editor/widgets/` - one component per `FieldWidget` variant (10)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 9's `FieldWidget`, `RegistryName`, option arrays.
- Produces, for Task 11: a widget dispatcher that renders any `FieldSpec`.

Binding points:
- **The frontend performs zero semantic validation** (spec 7). It holds the model as data, sends it, renders the returned diagnostics. Its only sanctioned local logic is the UX affordance spec 7 names: disabling Save while errors exist.
- **Sum types get an explicit `never` arm**: `const _exhaustive: never = x`. Both shapes fire, but only TS2322 **names** the unhandled variant, which is the same property that justifies the registry over the type. This is a deliberate, minimal improvement on the existing house shape (`src/jobRowState.ts:44-55`); `jobRowState.ts` is **not** required to change.
- **Cross-field constraints stay in core** (spec 7). Two exist in this surface and neither gets a widget: `AttachmentRule` requires exactly one of `select`/`drop`/`add`, and `Locator.match_to_source` is mutually exclusive with `match_pattern`. Both are already validated core-side and surface as diagnostics. A component **may** present the one-of as a mode selector - that is a UX affordance, not frontend semantic validation.
- The registry forces a label and widget to **exist** per field; it does **not** check the widget suits the field's type. That is accepted and recorded: a mismatched widget is a visible rendering bug caught the first time the panel opens, whereas a missing entry is silent absence. Do not add the mapped type `{ [K in keyof T]: FieldSpecFor<T[K]> }` - the brief settles the mechanism.

- [ ] **Step 1: Write the failing e2e assertions**

Extend `e2e/smoke.spec.ts` with per-widget rendering assertions using the harness's existing mock (`e2e/mocks.ts`) - read it first and extend its `load_profile` mock rather than adding a parallel mocking mechanism.

- [ ] **Step 2: Run to confirm they fail**

```bash
pnpm test:e2e
```
Expected: FAIL - no widgets exist.

- [ ] **Step 3: Implement the 10 widgets**

One component per variant from `:806-819`. Follow the house component conventions - read two existing components first (`src/components/SuggestionCard.vue`, `src/components/JobRow.vue`) and match their prop/emit/`$t` style. Note the recorded `withDefaults` + `T | null` vue-tsc quirk in BUILDING.md's tooling section before fighting a type error.

- [ ] **Step 4: Run the e2e suite**

```bash
pnpm build && pnpm test:e2e
```
Expected: PASS.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/editor/widgets e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the ten field widgets, exhaustive by never-arm (D45)"
```

---

### Task 11: D45 - the editor view

**Files:**
- Create: `src/views/EditorView.vue` (rule grid, drag-reorder, section composition, open/save, the save-surface note)
- Modify: `src/App.vue` (the `View` union at `:10`, the nav at `:71-96`, the mount block at `:98-112`)
- Modify: `src/ipc.ts` (hand-written command signatures for the four new commands)
- Test: `e2e/smoke.spec.ts`

**Interfaces:**
- Consumes: Task 9's registries, Task 10's widgets, Task 8's four commands, Task 5's `profile.ts` types.
- Produces: the editor surface.

Binding points:
- **`ts-rs` types the model only; command signatures stay hand-written in `src/ipc.ts`** as they are today (D44's explicit out-of-scope).
- The save-surface note is a **standing note, not a modal**, stated once at the save surface, with **no detection** of whether comments are present (that would need the parser to see them).
- Validation runs through `validate_profile_model` on every edit (spec 7's "every profile edit"), and Save is disabled while errors exist - the one sanctioned frontend affordance.
- Apply-suggestion lives in the **batch view**, not here (Task 12). D41 records why the plan-scope pairing of editor+apply is not a UI-location one: they share the in-memory model's ownership, which is the reason the pairing survives at all now that D22's comment-machinery premise is dead.
- **Help mode is Plan 7** and out of scope, but spec 8.3's tooltip/inline-explanation baseline **still applies** to the editor's views (D22's "NOT deferred" clause). Only the sidebar machinery waits.

- [ ] **Step 1: Write the failing e2e test**

Assert: the nav gains an editor tab; opening a profile renders the rule grid; a drag-reorder emits the reordered model; saving calls `save_profile`; the save note is visible at the save surface; Save is disabled while a diagnostic of severity error is present.

Follow `App.vue:98-104`'s recorded reason for `v-show` over `v-if` (both views stay mounted so JobsView's live run listeners survive tab switches) when adding the third view - do not switch the block to `v-if`.

- [ ] **Step 2: Run to confirm it fails**

```bash
pnpm test:e2e
```

- [ ] **Step 3: Implement the view, the nav entry, and the ipc signatures**

- [ ] **Step 4: Run the suite**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS. `pnpm lint` includes the D27 `no-raw-text` rule - every string in the template comes from `$t`.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/views/EditorView.vue src/App.vue src/ipc.ts e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the profile editor view, canonical save with its standing note (D45, D41)"
```

---

## Wave 4

### Task 12: D43 - one-click apply in the batch view

**Files:**
- Modify: `src/components/SuggestionCard.vue` (`:6-9` already receives `config_path` and `yaml_fragment`)
- Modify: `src/components/DiagnosticsPanel.vue` (wire the emit through)
- Modify: `locales/en/gui-batch.ftl`, `locales/de/gui-batch.ftl` (2 keys)
- Test: `e2e/smoke.spec.ts`

**Interfaces:**
- Consumes: Task 8's `apply_suggestion` command.
- Produces: the apply control.

Binding points:
- The frontend **forwards two opaque fields it never interprets** - `config_path` and `edit`. Core does all the interpreting. Do not parse `config_path` in TypeScript.
- The two new keys go in the existing **`gui-batch.ftl`**, beside `SuggestionCard.vue`'s current copy-button keys, because the apply control lives in the batch view. Only the editor's own surface justified the new `gui-editor.ftl`.
- **The no-fix case has no apply button and that is not a gap to close.** `core-109-two-required-no-fix` records that two required rules colliding on one track yield no suggestion at all, only the partition report; the diagnostics panel renders it as it does today.
- Apply does not validate; the editor's existing round-trip does.

- [ ] **Step 1: Write the failing e2e test**

Assert: a suggestion card renders an apply button; clicking it invokes `apply_suggestion` with the card's `config_path` and `edit` unmodified; a partition/no-fix diagnostic renders **no** apply button.

- [ ] **Step 2: Run to confirm it fails**

```bash
pnpm test:e2e
```

- [ ] **Step 3: Implement, with the two bilingual keys**

- [ ] **Step 4: Run the suite**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/components/SuggestionCard.vue src/components/DiagnosticsPanel.vue locales/en/gui-batch.ftl locales/de/gui-batch.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: one-click apply-suggestion in the batch view (D43)"
```

---

## Triggers this plan creates (controller mirrors into ROADMAP at plan close)

Design section 7 (`:1895-1930`) names seven; they are the controller's to write, not a task's. Restated here only so the plan close has them in one place:

1. A profile-model field gains a `#[serde(default)]` -> it joins D48's 17-row table with **all three** attributes naming the same function. This is the one place in the plan where getting it wrong loses user data silently.
2. **D48's derivation exists in the tree -> re-examine guard 2.** Mutate one field's `extend` expression away from its `default` function and see whether guard 2 goes red. If it cannot be made to fail, it is measured redundant and removed **then**, with the measurement recorded. If it can, the design phase was wrong to think it a tautology and the guard stays for good. Either way the question gets settled by running it, which is exactly what the design phase could not do.
3. `tauri-specta` publishes a stable non-RC Tauri-2 release -> re-evaluate D44's rejection.
4. A second Muxsmith artifact needs TypeScript types -> extend the `ts` feature's export set rather than hand-mirroring again.
5. 1.0 is tagged, or a user asks for zero-config schema autocompletion -> re-evaluate SchemaStore publication.
6. A profile-model field is added or removed -> the D44 drift check and the D45 registry both fail by construction, naming the site. No tracker entry needed; the mechanism **is** the tracker.
7. A second generated artifact gains a CI drift check -> the committed-generated-plus-drift-check pattern reaches count 2 toward Tier-2 promotion.

## Open, carried into the plan close, not assigned to a task

- **`gui-22` vs `exec-44-runlog-14day-autoprune` is a recorded-statement collision** in `product-boundaries.yaml`: `gui-22` (`:243-252`) says v1 keeps all run logs with pruning deferred to v1.x, while `exec-44` (`:15-23`) records D35 reversing exactly that to an automatic 14-day prune, which shipped. `gui-22` still carries `status: settled` with no supersession marker. Unrelated to Plan 6, found while reading Tier 2 for the design and independently confirmed by the design reviewer (design `:1947-1956`). Needs a controller action item; a recorded-statement collision is one of the observable contested criteria, so it routes to the owner as a spec question.
- **The D23 `runActive` reset re-check** stays listed under the ROADMAP's Plan-6 anchor although the design review established it is run-path only (`src/views/JobsView.vue:150-200`) and touches nothing this plan builds. It stays until an owner call re-points it, rather than being moved silently.
