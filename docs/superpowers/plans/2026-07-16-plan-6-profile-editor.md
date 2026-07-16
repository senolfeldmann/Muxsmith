# Plan 6: profile editor, apply-suggestion, schema keyword domains

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** implement ADRs D41-D48 (`docs/superpowers/specs/2026-07-15-plan-6-design.md`) and the apply-seam amendment D49 (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`): a hand-built profile editor over a canonical-save core writer, one-click apply-suggestion, ts-rs-generated wire types with a CI drift check, and the schema keyword-domain fix that makes `muxsmith schema` a supported user artifact.

**Architecture:** four waves. Wave 1 runs three independent streams in parallel worktrees and then one join task. Stream A (`.worktrees/plan6-a`) is the core model + writer stream (Tasks 2, 3, 4). Stream B (`.worktrees/plan6-b`) is the planner-side applier and the `StructuredEdit` reshape (Task 6, D43/D49). Stream C (`.worktrees/plan6-c`) is docs (Task 7). Task 5 (D44, ts-rs bindings) is the wave-1 **join**: it needs stream A's model settled *and* stream B's final `StructuredEdit` shape, so it lands after both merge. Wave 2 is the Tauri shell's IPC surface (Task 8), which needs A's writer and B's applier. Wave 3 is the frontend (Tasks 9-13), which needs A's generated bindings (via Task 5) and wave 2's commands. Wave 4 is the batch view's apply button (Task 14), sequenced after Task 13. Merge sequentially, nine-part gate after every merge.

**Tech Stack:** Rust workspace (schemars 1.2.1, serde, yaml_serde 0.10.4, ts-rs 12.0.1 behind a `ts` feature), Tauri 2 shell, Vue 3 + TypeScript 6.0.3, Fluent (en+de), Playwright.

## Global Constraints

- **The design document is the contract**; the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative above it on conflict. Its decisions are settled and reviewed - do not reopen them, and do not re-derive their rationales.
- **D49 (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`) is a settled scoped amendment to D43** and is ground truth for Tasks 6 and 8. It resolves the apply seam the design review flagged Critical. Tasks 6 and 8 read it in full and take its decisions, signatures, and test bodies as given.
- **Every fork in this plan is closed.** No task brief, verdict or fix-round dispatch may add a design-latitude clause, in either form: an explicit permission ("if a simpler alternative exists, implement it") or an omission (an unenumerated set in a normative position). The test is "must the implementer invent something it is not allowed to invent?" (`proc-latitude-clause-boundary`). A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** - options, costs against the named invariants, a recommendation - and is routed by the controller before it is resolved. It is never decided at the keyboard, and "push back if you disagree" is not a licence to decide-then-report.
- **Four owner rulings/decisions post-date the design document and bind over its text** (Şenol, 2026-07-16). Task 1 folds all four into the design; every later task reads the amended document:
  1. **The save writer's error currency is `SaveError`, not `Diagnostic`** (design `:132-133` is superseded). `profile::save` returns `Result<_, SaveError>` with `SaveError::{Io(String), Serialize(String)}`; the shell maps it to `IpcError` codes in `gui-common.ftl`. No new `DiagCode`, no `diagnostics.ftl` change. Rationale is Tier-2 `core-124-error-currency-split`: a `Diagnostic` describes a profile/plan problem, a write failure does not; `ParseError` is never reused for a non-parse failure because its prose asserts a parse.
  2. **The save-surface note is ONE Fluent message** (design `:278` "Two new Fluent keys" is superseded). `gui-editor.ftl` carries **43** keys: 42 registry labels + 1 note, exactly as design `:1736`/`:1749` already state.
  3. **`load_profile` returns no bespoke `ProfileDocument` struct** (design `:311`/`:341-349`/`:404` are superseded). It returns the existing `report::json::config_only_document(&diags, None, renderer)` envelope PLUS a `"profile"` key (the parsed model, or `null` on `ParseError`), so its diagnostics live under `config_diagnostics` with the injected `rendered` field, byte-identical to `validate_profile`'s output, and no second document shape is invented. Rationale is Tier-2 `core-85-report-json-dry`: neither surface owns document logic.
  4. **The editor's spec-8.3 tooltip baseline rides Plan 7, not Plan 6** (design `:1878-1880` is corrected). The editor ships in Plan 6 *without* tooltips; `gui-editor.ftl` stays exactly 43 keys with no tooltip budget. The ROADMAP Plan 7 anchor (`docs/ROADMAP.md:74-84`) already carries the editor's 42 tooltip keys.
- **Tier-2 files are ground truth alongside the spec**: `docs/product-boundaries.yaml` (product scope), `docs/conventions.yaml` (house code style), `docs/process-conventions.yaml` (method). Conform to them; surface, never silently resolve, any new pattern you establish or deliberate deviation.
- **Nine-part gate green before any push**, per BUILDING.md, run foreground, no subsets: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo deny check`, `pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`.
- **Every new or changed Fluent message lands bilingual (en + de) in the same commit.** The de register follows the de catalog header rules (config keywords literal, straight quotes, du-imperative).
- **Versions are pinned and registry-verified, never typed from memory** (`ci-10-pin-everything`, `proc-07-verify-against-source`): `ts-rs = { version = "12.0.1", optional = true }`, caret semantics matching `schemars = "1.2.1"`. `=`-pins are reserved for dev-dependencies in this tree.
- **Commits unsigned** (`git -c commit.gpgsign=false commit ...`), trailer `Co-Authored-By: <your model name> <noreply@anthropic.com>`, stage files explicitly - **never `git add -A`**.
- **Typography**: ASCII hyphens, straight quotes, no Unicode ellipsis, in all docs, code comments and messages.
- **`#![deny(missing_docs)]` is on in `muxsmith-core` and `muxsmith-cli`** (`crates/muxsmith-core/src/lib.rs:1`); `src-tauri` deliberately opts out (`src-tauri/src/lib.rs:14-17`: bin-shaped crate whose `[lib]` exists only for Tauri's mobile entry point) but still documents its public items. Every new public item in core/cli needs rustdoc, and `cargo doc` gates that the intra-doc links resolve; the shell task writes the same rustdoc without the `-D warnings` enforcement.

## How this plan cites the design

The design document is 2005 lines of settled, measured decisions, and it is what every reviewer grades against; D49 is its settled apply-seam amendment. Where either already states the implementation exactly - the 17-field table, the 43-field widget table, the predicates, the emitter, the `extend` shape, D49's applier and its seven guard tests - **this plan cites it by section and line range rather than copying it**. That is deliberate: a second copy of a normative block is a drift surface, and refusing to create one is this design's own most-repeated argument (D45's keyword arrays, D48's `extend` derivation, `capability::CODEC_KIND_NAMES`). Every task below names the exact lines its implementer must read. Everything the design and D49 do *not* already carry - task-authored test code, commands, file paths, task boundaries - is written out in full here.

## Amendment 2026-07-16: test-mount harness for wave 3

Tasks 10-12 mandate failing-first e2e assertions of the shape "opening the editor renders ...", but no editor mount point exists in the running app before Task 13: `src/main.ts` mounts only `App.vue` (view union `"batch" | "jobs"`), `EditorView.vue` is nav-wired only in Task 13, and Playwright's `webServer` serves the single-entry `dist/`, so those step-1 assertions are unexecutable as written (traced in `.superpowers/sdd/plan-6/task-10-report.md`). The controller ruled **Option A** (2026-07-16): Tasks 10-12 get real DOM red/green loops through a minimal test-mount harness that extends the established `e2e/vite.harness.config.ts` pattern - test-only, gitignored output under `e2e/.generated/`, never in `dist/`, never shipped - and Option B (compile-only proofs until Task 13) is rejected. The mechanism: a second Vite lib build (`e2e/vite.mount.config.ts`, Vue plugin) bundles `e2e/mount-entry.ts` - which builds a component registry via `import.meta.glob` over `src/editor/widgets/*.vue` and `src/views/EditorView.vue` and wires Fluent through the app's own `buildBundles` (real `gui-editor.ftl`, not a stub) - into `e2e/.generated/mount-harness.js`, injected via `page.setContent` + `page.addScriptTag` and driven by `window.__muxsmithMount__` from the `e2e/mount.ts` helper. Tasks 10-12 mount the widget/dispatcher/view under test with its model as a prop and assert on the rendered DOM, while Task 13's steps continue to run against the real nav-reachable app. The harness is set up once in Task 10 (its Step 1) and reused verbatim by Tasks 11-12.

---

## Wave 1

Three streams in parallel worktrees plus one join task. **Task 1 lands on master first**, because all streams read the amended design document.

**Dependency graph (stated explicitly - closed decision, D49 ripple):**

- **Stream A** (`.worktrees/plan6-a`): Tasks 2, 3, 4. Tasks 3, 4 and 5 converge on `crates/muxsmith-core/src/profile/model.rs`, so they serialize on that file; Task 2 does **not** touch `model.rs` (it creates `save.rs`), but it precedes Task 4, which consumes its writer (`save::to_string`) and extends its test file. So within stream A: Task 2 and Task 3 are mutually independent (disjoint file sets) and may run in parallel, and Task 4 follows both (`2 -> 4` content, `3 -> 4` shared `model.rs`).
- **Stream B** (`.worktrees/plan6-b`): Task 6 - `planner.rs` and its test file only. Independent of streams A and C. Sets `StructuredEdit`'s final D49 shape and writes the applier.
- **Stream C** (`.worktrees/plan6-c`): Task 7 - docs only (README + the v1 spec). Independent.
- **Join: Task 5** (D44, ts-rs bindings). It depends on **Task 4** (the `model.rs` chain must be settled: Task 5 adds TS derives to the same 20 structs/enums Task 3 and Task 4 modify, and carries Task 3's `KEYWORDS` into `keywords.ts`) **and on Task 6** (it adds the `#[cfg_attr(feature = "ts", ...)]` TS derive to `StructuredEdit` in `planner.rs`, which only exists in its final shape after Task 6). Task 5 is therefore the join of the keyword chain and the `StructuredEdit` chain, and lands **after streams A and B merge to master**. No cycle exists: Task 6 defers *all* ts concerns to Task 5 (closed decision 7 / D49 "D49 cannot land before D44"), so the edge is `6 -> 5`, never `5 -> 6`.

Full edge set: `1 -> {2,3,6,7}`, `2 -> 4`, `3 -> 4`, `4 -> 5`, `6 -> 5`, `7` independent. Task 5 has two predecessors (Task 4 and Task 6) and is acyclic.

---

### Task 1: Fold the four owner rulings into the design document

**Files:**
- Modify: `docs/superpowers/specs/2026-07-15-plan-6-design.md`

**Interfaces:**
- Consumes: nothing.
- Produces: the amended design document every later task reads as ground truth.

This is a **documentation-only** task. It changes no code. Its purpose is that no implementer reads a superseded signature: the design as approved fixes the save writer at `Result<_, Diagnostic>`, describes a bespoke `ProfileDocument` struct, and records spec 8.3's tooltip baseline as applying to the editor - the three forks the owner ruled on, plus the save-note count. Do not re-argue the rulings; record them.

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

- [ ] **Step 4: Point section 2's `ApplyError` row at D49's three codes**

Section 2's D43 row (`:1739`) still names the single `suggestion-rule-not-found` code that D49 superseded with three, so the approved design's own catalog table now contradicts D49 - the dangling-reference class step 2 also fixes. Replace the row:

```markdown
| D43 | `ApplyError` codes (`apply-unparsable-config-path`, `apply-rule-index-out-of-range`, `apply-edit-changed-nothing`) | `gui-common.ftl` | 3 |
```

D49 (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`, section "The catalog entries") is the authority on these three; this row records them, it does not re-specify them.

- [ ] **Step 5: Amend D42's `ProfileDocument` bullet to the owner's resolution**

The design describes `ProfileDocument` in three places, all now superseded by owner decision 2026-07-16 (`core-85-report-json-dry`). Amend all three so no implementer reads the bespoke-struct shape:

At `:341-349`, replace the `ProfileDocument` paragraph (from "`ProfileDocument` is `{ profile: Option<Profile>, diagnostics: Vec<Diagnostic> }`," through "a second call would let them disagree.") with:

```markdown
`load_profile` returns **no bespoke struct**. It returns the existing
`report::json::config_only_document(&diags, None, &ShellRenderer)` envelope
(the same document machinery `validate_profile` uses) with one added key,
`"profile"`: the parsed model, or `null` on a `ParseError`. Its diagnostics
therefore live under `config_diagnostics`, carry the injected `rendered`
field, and are **byte-identical in shape** to what `validate_profile` already
returns (`core-85-report-json-dry`: neither surface owns document logic, and
no second document shape is invented). On a `ParseError` the `"profile"` value
is `null` and the single diagnostic explains why, mirroring
`config_diagnostics_from_file`'s own short-circuit
(`profile/validate.rs:203-208`). One round trip, because the editor needs
both and a second call would let them disagree. (Owner decision 2026-07-16,
superseding the original bespoke `ProfileDocument` struct.)
```

At `:311`, replace the `load_profile` command-table row's return type and note:

```markdown
| `load_profile` | `async fn load_profile(path: String) -> Result<serde_json::Value, IpcError>` | New. Returns the `config_only_document` envelope plus a `"profile"` key (the model, or `null` on `ParseError`); no bespoke struct (owner 2026-07-16, `core-85`). |
```

At `:404`, replace "new `ProfileDocument` wire shape;" in the interface-changes sentence with "the `load_profile` document shape (the `config_only_document` envelope plus a `"profile"` key);".

- [ ] **Step 6: Correct section 6's editor tooltip statement**

At `:1878-1880`, section 6 currently reads "The spec 8.3 **tooltip/inline-explanation baseline still applies to the editor's views** (D22's 'NOT deferred' clause); only the sidebar machinery waits." Owner ruling 2026-07-16: the editor's 8.3 tooltip baseline defers to Plan 7 with the sidebar. Replace those lines with:

```markdown
The editor's own spec 8.3 tooltip/inline-explanation baseline **defers to
Plan 7** with the sidebar (owner ruling 2026-07-16): the editor ships in
Plan 6 WITHOUT tooltips, and its 42 controls get their tooltip keys in the
Plan 7 pass, in the same pass as their help-ids, rather than as a retrofit -
the re-cut's own stated reason for sequencing Plan 7 after Plan 6. So
`gui-editor.ftl` carries 43 keys in Plan 6 (42 labels + 1 save-surface note)
and grows by the tooltip set in Plan 7 (`docs/ROADMAP.md:74-84`).
```

- [ ] **Step 7: Record all four rulings in section 8**

Section 8 ("What the implementer must not decide") is the list a dispatched implementer reads to know what is pre-decided. Append four bullets:

```markdown
- The writer returns `SaveError`, **not** a `Diagnostic`, and the shell maps
  it to `profile-save-io-failed` / `profile-save-failed` in `gui-common.ftl`.
  No new `DiagCode` and no `diagnostics.ftl` change (owner ruling 2026-07-16,
  `core-124-error-currency-split`).
- The save-surface note is **one** Fluent message, so `gui-editor.ftl` carries
  43 keys: 42 registry labels + 1 note (owner ruling 2026-07-16).
- `load_profile` returns the `config_only_document` envelope plus a `"profile"`
  key, **not** a bespoke `ProfileDocument` struct; its diagnostics are
  byte-identical in shape to `validate_profile`'s (owner decision 2026-07-16,
  `core-85-report-json-dry`).
- The editor ships in Plan 6 **without tooltips**; spec 8.3's editor tooltip
  baseline defers to Plan 7. `gui-editor.ftl` gets no tooltip budget here
  (owner ruling 2026-07-16, `docs/ROADMAP.md:74-84`).
```

- [ ] **Step 8: Verify no superseded text survives**

Run each and confirm the stated expectation. Each grep is scoped so it produces output for the defect it guards and none once fixed - the two split greps below replace one that spanned a line break and could never match its second alternative.

```bash
D=docs/superpowers/specs/2026-07-15-plan-6-design.md

grep -n "Result<String, Diagnostic>\|Result<(), Diagnostic>" "$D"
# Expected: no output. Both signatures now name SaveError (step 1).

grep -n "Two new Fluent keys" "$D"
# Expected: no output. The count was corrected in step 2.

grep -n "D47's" "$D"
# Expected: no output. The dangling cross-reference was the only "D47's" in the
# file (confirmed: it occurs once, at :278) and step 2 removed it.

grep -n "suggestion-rule-not-found" "$D"
# Expected: no output. Section 2's D43 row named it (once, at :1739) and step 4
# repointed it to D49's three codes; it survives nowhere else in the design.

grep -cn "SaveError" "$D"
# Expected: 6 or more (D41's enum, its two variants in prose, section 8, the
# mapping note). It was 0 before this task.

grep -n "carries \*\*43\*\*" "$D"
# Expected: one hit at :1749 - unchanged, and now consistent with D41.

grep -n "profile: Option<Profile>, diagnostics: Vec<Diagnostic>" "$D"
# Expected: no output. The bespoke ProfileDocument struct is gone (step 5).

grep -c "config_only_document" "$D"
# Expected: 1 or more. Step 5 introduced the envelope reference; it was 0 before.

grep -n "still applies to the editor" "$D"
# Expected: no output. The tooltip statement was corrected in step 6.
```

- [ ] **Step 9: Commit**

```bash
git add docs/superpowers/specs/2026-07-15-plan-6-design.md
git -c commit.gpgsign=false commit -m "plan 6 design: fold the four owner rulings (SaveError currency, one-key save note, load_profile envelope, editor tooltips to Plan 7) and repoint section 2's ApplyError row to D49"
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

Create `crates/muxsmith-core/src/profile/save.rs`. `SaveError` derives `Debug` (the test's `{other:?}` needs it) and `PartialEq`; it is a plain enum, not a `std::error::Error`, matching `SettingsError`'s shape in the shell. Give every public item rustdoc - `#![deny(missing_docs)]` is on in core. The two `to_string` arms are `yaml_serde::to_string(profile)` and `serde_json::to_string_pretty(profile)`, each mapping its error into `SaveError::Serialize(e.to_string())`; `to_file` selects the format from the extension exactly as `load::from_file:57-62` does, then `fs::write`, mapping into `SaveError::Io(e.to_string())`.

- [ ] **Step 4: Wire the module in**

In `crates/muxsmith-core/src/profile/mod.rs`, add `pub mod save;` in alphabetical position (after `pub mod model;`), and extend the module doc's sentence so `save` is named beside `load` - it currently reads "[`load`] parses YAML/JSON into the serde model".

- [ ] **Step 5: Add the dev-dependency if absent**

The test uses `tempfile`. Check first, and only add it if it is missing:

```bash
grep -n "tempfile" crates/muxsmith-core/Cargo.toml
```
If absent, add it under `[dev-dependencies]` with the exact version already used elsewhere in the workspace (`grep -rn "tempfile" crates/*/Cargo.toml`) - do not invent a version. (At `0cc15d7`, `tempfile = "3.27.0"` is already present, so this is a no-op; the check is here so a re-cut does not assume it.)

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

- [ ] **Step 1: Extract the schema helper and write the failing schema test**

`cli_schema.rs` today has **no** helper: both its tests inline `Command::cargo_bin("muxsmith").unwrap().arg("schema").assert().success()...` and parse the stdout. Extract that block into a `schema_json()` helper and re-point the existing `schema_prints_json_schema_and_exits_zero` test at it - both tests then share the one invocation (this is the only legal move: the file forbids a second way to run the command, and duplicating the block would be that). Read the file first to copy its exact invocation.

```rust
fn schema_json() -> serde_json::Value {
    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&out).unwrap()
}
```

Then add the new test, whose target shape D46 measured empirically (`:1112-1120`):

```rust
#[test]
fn keyword_domains_project_as_closed_enums_not_bare_strings() {
    let schema = schema_json();
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

The spec 8.2 amendment is **not** in this task - it lives in Task 7 (stream C, the single v1-spec owner), so stream A touches no shared file (F3).

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

- [ ] **Step 2: Baseline - observe guard 1 green on the unmodified serializer**

```bash
cargo test -p muxsmith-core --test profile_save all_non_default
```
Expected: **PASS.** This is *not* a TDD red step and cannot be one: before D48's `skip_serializing_if` attributes exist nothing is omitted, so a save/load round-trip must pass (the design records this at `:90-92`, "Canonical round-trip is exact ... it holds today"). Step 1 already created the fixture, so there is no "fixture missing" arm. The real red proof is **step 4**, where a deliberately-naive predicate makes guard 1 go red. Record the baseline PASS in the log.

- [ ] **Step 3: Add the predicates and the 17 field attributes**

Work the table at `:1517-1535` row by row. Four predicates beyond the generic one, per `:1447-1457`. Every row gets `#[serde(default...)]`, `skip_serializing_if`, and `#[schemars(extend("default" = ...))]`, and **all three name the same function**.

- [ ] **Step 4: Prove guard 1 catches the naive predicate (the real red proof)**

Before moving on, deliberately break one row and confirm the guard fires - this is the evidence that guard 1 works, and it costs thirty seconds:

```bash
# temporarily change TracksCfg.unmatched's skip_serializing_if to the generic "is_default"
cargo test -p muxsmith-core --test profile_save
# Expected: FAIL - the_core83_passthrough_profile_survives_a_save goes red.
# Then revert to is_drop_policy and confirm green again.
```
Record the observed failure text in your report. If it does **not** go red, guard 1 is not testing what D48 says it tests - stop and return NEEDS_CONTEXT.

- [ ] **Step 5: Write guard 2 - schema-default honesty**

A table test asserting, for each of the 17 fields, that the schema's `default` equals `serde_json::to_value` of that field's serde default. Follow the house's existing table-test shape - `capability/mod.rs`'s `settable_maps_to_mkvmerge_options` asserts a `const EXPECTED` table against the real thing, length first, then row by row - rather than inventing a pattern. The three struct-valued fields (`Profile.output`, `Profile.attachments`, `Profile.tags`) expect `{}`, per `:1612-1637`.

- [ ] **Step 6: Run everything**

```bash
cargo test -p muxsmith-core --test profile_save
cargo test --workspace
git diff --exit-code crates/muxsmith-cli/tests/snapshots/
```
Expected: all green; snapshots unmoved.

- [ ] **Step 7: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/tests/profile_save.rs crates/muxsmith-core/tests/fixtures/all-non-default.yaml
git -c commit.gpgsign=false commit -m "core: a canonical save omits default-valued fields, with both guards (D48)"
```

---

### Task 6: D43 + D49 - apply_suggestion in core, and the typed `StructuredEdit` seam

**Stream B** (`.worktrees/plan6-b`), parallel with streams A and C - touches `planner.rs` and its test file only.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (`StructuredEdit` reshape + `Deserialize`; `delta_for` loses its scalar param; four engine call sites; new `apply_suggestion`, `ApplyError`, `edit_key`; the `:10` import)
- Test: `crates/muxsmith-core/tests/suggestions.rs`

**Interfaces:**
- Produces, for Task 5: `StructuredEdit` in its **final D49 shape** - `value: Scalar` on the two `Exact` variants, plus `Deserialize` - but **without** the `ts` derive, which Task 5 adds.
- Produces, for Task 8: `muxsmith_core::planner::apply_suggestion(profile: &Profile, config_path: &str, edit: &StructuredEdit) -> Result<Profile, ApplyError>` and `muxsmith_core::planner::ApplyError` (three variants).
- Consumes: nothing from other tasks.

**Read first:** design D43 (`:410-495`) and **D49 in full** (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`). D49 supersedes D43 on the seam; where the two differ, D49 wins.

Binding points (all from D49; each is a place the design review found the original plan handed the implementer an uncloseable fork):
- **The seam D43 called "reuse" is resolved by D49, not by reconstruction.** `StructuredEdit`'s two `Exact` variants carry `value: Scalar` (not `String`), so the engine's own typed value rides on the edit and apply reconstructs nothing. The two substring variants keep `value: String` (their target map `MatchExpr.substring` holds `String`). This asymmetry is settled, not the implementer's to reconsider (D49 §"The wire shape").
- **`delta_for` loses its `scalar` parameter and stays private** (D49 §"delta_for"): it now reads the edit's own typed `value`. `apply_suggestion` reaches it, `with_rule_match` and `rule_index_of` by ordinary in-module visibility - no visibility change is needed anywhere (`with_rule_match`/`rule_index_of` are already `#[doc(hidden)] pub`; `delta_for` stays private).
- **`apply_suggestion` has exactly three exits, so `ApplyError` has exactly three variants** (D49 §"ApplyError"): `rule_index_of` returns `None` -> `UnparsableConfigPath`; the parsed index is past `tracks.rules.len()` -> `RuleIndexOutOfRange { index, rules }`; the spliced model equals the input (the `core-44` `or_insert` merge dropped the delta) -> `EditChangedNothing { index, property }`. The last is detected by one whole-model comparison (`applied == *profile`), which is a single `PartialEq`, not a re-plan and not a validation. `ApplyError` is `pub`, derives `Debug, Clone, PartialEq`, and does **not** derive `Deserialize`.
- **`StructuredEdit` gains `Deserialize`; `Suggestion` and `DiagCode` do not** (`core-37-prose-free-core`, the most-reinforced house entry: the shell must not be able to synthesize diagnostics).
- **`apply_suggestion` does not validate and does not re-plan.** It returns the mutated model; the caller validates through the normal `validate_profile_model` path (D43). `core-33` (narrow-only), `core-44` (no-clobber via `with_rule_match`) and `core-72` (typed equality) all hold unchanged (D49 §"Not changed by this ADR").
- **The `ts` derive on `StructuredEdit` is Task 5's, not this task's** (closed decision 7 / D49 "D49 cannot land before D44"). Task 6 lands `StructuredEdit` with `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]` and `#[serde(tag = "kind", rename_all = "snake_case")]` **only**; it does **not** add the `#[cfg_attr(feature = "ts", derive(TS), ...)]` line and does **not** add the `#[cfg(feature = "ts")] use ts_rs::TS;` import. Those need the `ts` feature and `Scalar: TS`, which Task 5 provides. Task 6 does change `planner.rs:10` to `use serde::{Deserialize, Serialize};` (the `Deserialize` derive needs it).

- [ ] **Step 1: Write the failing tests, fixture and helpers (D49 gives all bodies)**

Add to `crates/muxsmith-core/tests/suggestions.rs`, verbatim from D49's guard section (a second copy of settled test code would be a drift surface - copy from D49, do not paraphrase):

- The **harness split** (D49 §"Harness change the guard needs", `:815-857`): introduce `plan_model(profile: &Profile, files) -> (Batch, TempDir)` as today's `plan_multi` body with the `from_str` lifted to the caller; re-express `plan_multi` and `plan` on top of it (this removes an existing hand-rolled duplicate, it does not add one). Add the two imports D49 names: `use muxsmith_core::profile::model::Profile;` and `use muxsmith_core::planner::{ApplyError, apply_suggestion};`.
- The **`spliced_scalar` accessor** (D49 `:864-891`) and the **`yaml_scalar` helper** (D49 `:1264-1276`).
- The fixture **`P_ALREADY_CONSTRAINED`** (D49 `:1073-1079`).
- The **seven tests**, each copied verbatim from its D49 section:
  1. `apply_splices_the_simulated_scalar_for_a_bool_property` - G1, the Bool case (D49 `:895-933`), with its `checked > 0` anti-vacuity assertion.
  2. `apply_splices_the_simulated_scalar_for_an_int_property` - G2, the Int case (D49 `:943-980`), with its `checked > 0` assertion.
  3. `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level` - G3, the `core-03` re-plan at the model level with no YAML laundering (D49 `:989-1019`).
  4. `apply_rejects_an_unparsable_config_path` (D49 `:1034-1045`).
  5. `apply_rejects_a_rule_index_past_the_end` (D49 `:1047-1058`).
  6. `apply_rejects_an_edit_the_no_clobber_merge_drops` - G4, the `EditChangedNothing` case (D49 `:1081-1095`).
  7. `apply_returns_ok_when_the_edit_reaches_the_model` - the control proving the no-op detection does not fire when the edit lands (D49 `:1100-1112`).

- [ ] **Step 2: Run to confirm it fails**

```bash
cargo test -p muxsmith-core --test suggestions apply
```
Expected: FAIL to compile - `apply_suggestion` and `ApplyError` are undefined, and the tests construct `StructuredEdit` with `Scalar`-typed values the current `String` field rejects. Both are the red. (The existing suggestion tests still bind `value` as `&str`; they stop compiling too, and step 4 updates them.)

- [ ] **Step 3: Implement the reshape and the applier (D49 gives the code)**

In `planner.rs`, exactly per D49:
- Change `:10` to `use serde::{Deserialize, Serialize};`.
- Reshape `StructuredEdit` to D49 §"The wire shape" (`:345-383`) **minus** the `#[cfg_attr(feature = "ts", ...)]` line (Task 5 adds it): `value: Scalar` on `AddExact`/`AddNotExact`, `value: String` on the two substring variants, deriving `Debug, Clone, PartialEq, Serialize, Deserialize`, tagged `kind`/`snake_case`.
- Rewrite `delta_for` to the single-argument, private form (D49 §"delta_for", `:415-448`).
- Apply the four engine call-site edits (D49 §"The engine call sites", `:454-459`): `:1746` and `:1753` become `value: scalar.clone()`; `:1762` and `:1791` become `delta_for(&edit)`, dropping the synthetic `Scalar::Str` at `:1791`. `prop_value_as` keeps its `(String, Scalar)` return - `scalar` now goes into the edit, `display` still keys the `seen` dedup and the `rank` tuple.
- Add `apply_suggestion` (`pub`), `ApplyError` (`pub`, three variants), and the private `edit_key` helper, all verbatim from D49 §"apply_suggestion" (`:465-519`) and §"ApplyError" (`:522-560`).

- [ ] **Step 4: Update the seven existing `value`-binding sites (D49 gives each)**

D49 §"Interface changes" > "Tests" (`:1252-1311`) enumerates all seven and gives the fix for each; apply them exactly:
- **Four template sites** (`:97`, `:100`, `:203`, `:206`) interpolate `yaml_scalar(value)` where they interpolated `value`. `yaml_scalar` (added in step 1) renders byte-identically to the old `display` string.
- **Three comparison sites**, each getting the typed literal from D49's table (`:1291-1295`), never derived from the old string: `:325` `track_name` -> `&Scalar::Str("Chapter 1: Intro".to_string())`; `:722` `forced_track` -> `&Scalar::Bool(true)` (this one does **not** follow `:325` - `forced_track` is Boolean); `:890` `language` -> `&Scalar::Str("eng".to_string())` (this is a **negative** assertion, so a wrong literal passes vacuously - it must be `Str` because `language` is String-typed).

- [ ] **Step 5: Run the tests**

```bash
cargo test -p muxsmith-core --test suggestions
```
Expected: PASS - the seven new tests plus the updated existing ones.

- [ ] **Step 6: Prove the no-clobber reuse is structural, not asserted**

`core-44` is the one thing a plausible reimplementation gets wrong silently, and G4 proves the behaviour. Confirm structurally that `apply_suggestion` splices through the engine's own helpers and merges nothing itself, scoping the grep to the function body so it actually fires (the earlier plan's grep opened its `sed` range on a line that never contains both patterns and so could never match):

```bash
awk '/pub fn apply_suggestion/,/^}/' crates/muxsmith-core/src/planner.rs \
  | grep -nE "with_rule_match|delta_for|extend\("
# Expected, read from the OUTPUT (not from its absence): at least one
# `with_rule_match` line and one `delta_for` line, and NO `extend(` line.
# A BTreeMap::extend inside the applier is the Bug C shape and fails review;
# the only legitimate `extend` is inside with_rule_match's `not` merge, which
# is a different function and outside this range.
```

- [ ] **Step 7: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/planner.rs crates/muxsmith-core/tests/suggestions.rs
git -c commit.gpgsign=false commit -m "core: StructuredEdit carries the typed Scalar; apply_suggestion splices through the engine's own helper (D43, D49)"
```

---

### Task 7: D47 - the schema as a supported user artifact, and the v1-spec amendments

**Stream C** (`.worktrees/plan6-c`), parallel with streams A and B - docs only. **This task is the single owner of the v1 spec** (F3): it carries all three spec amendments (8.1, 8.2, 8.4) and runs the self-contradiction sweep once against the merged result.

**Files:**
- Modify: `README.md` (new section)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (spec 8.1 + spec 8.2 + spec 8.4 exception)

**Interfaces:**
- Consumes: nothing. Produces: nothing code-facing.

**Read first:** design D47 (`:1227-1332`) and design section 3 (`:1772-1813`, the three spec amendments and the already-run sweep).

Binding points:
- The README documents the **editor-settings binding**, not the in-file modeline, and **the reason is D41**: the modeline `# yaml-language-server: $schema=...` is a YAML comment, and a canonical save does not preserve comments - so a user who wires up autocompletion with a modeline and then saves once from the GUI loses their schema binding silently, with no message. The README states that consequence explicitly rather than leaving the user to find it.
- The schema's English `description` fields (Rust doc comments) become user-facing under D47. This is an **accepted, deliberate boundary**, not an oversight: the schema documents a *file format*, the same category as the README and the spec, both English-only by design. Spec 8.4 gains an explicit exception entry so a future reviewer does not read it as a standing violation.
- Do **not** add SchemaStore publication or a GUI startup write; both are rejected in D47 and parked behind triggers.

- [ ] **Step 1: Write the README section**

Document `muxsmith schema > muxsmith-profile.schema.json`, the VS Code `yaml.schemas` mapping over a glob such as `*.muxsmith.yaml`, and the equivalent `lspconfig` settings block for Neovim/Helix. State the modeline consequence. Keep the README's established sell-tone register (the case-scoped exception recorded in the ROADMAP's README entry), not the writeup register.

- [ ] **Step 2: Amend spec 8.1**

`muxsmith schema` is a supported user feature, not only a debug aid; cross-reference the README section (design section 3, amendment 2).

- [ ] **Step 3: Amend spec 8.2** (moved here from Task 4)

The profile-editor bullet currently says only "open/save YAML". State that saving writes canonical YAML from the model and does not preserve comments, key order or formatting (D41), and that fields left at their default are not written back (D48). Design section 3, amendment 1, gives the exact scope; do not imply the editor reproduces spec 4.1's flow-style example verbatim (`:1809-1813`).

- [ ] **Step 4: Amend spec 8.4**

Add the JSON Schema's `description` fields to the accepted-v1-exceptions list, with the file-format-documentation rationale (design section 3, amendment 3).

- [ ] **Step 5: Run the self-contradiction sweep once, against all three amendments**

`proc-04-spec-wins` mandates the sweep after any amendment; run it **once** over the file now carrying all of 8.1, 8.2 and 8.4 (this is why the v1 spec has a single owner - a per-stream sweep would each see only its own amendment). Design section 3 (`:1797-1813`) records the sweep as already run and complete for exactly these three amendments, including the finding that spec 4.1's reference example stays correct because it, too, omits `source`/`optional`. Confirm that still holds against current spec text; do not re-derive it.

- [ ] **Step 6: Gate the docs**

```bash
pnpm lint
grep -rn "—\|–\|“\|”\|…" README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```
Expected: the grep returns **no output** (typography constraint: ASCII only).

- [ ] **Step 7: Commit**

```bash
git add README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "docs: the JSON schema is a supported hand-authoring artifact; fold spec 8.1/8.2/8.4 amendments (D47, D41, D48)"
```

---

### Task 5: D44 - ts-rs bindings, committed, with a CI drift check (the wave-1 join)

Runs **after streams A and B merge to master**. It is the join of the model.rs chain (Task 4, carrying Task 3's `KEYWORDS`) and the `StructuredEdit` chain (Task 6), because it owns **all** ts concerns: the `ts` feature, the derives on the 20 model types, and the derive on `StructuredEdit` in `planner.rs`.

**Files:**
- Modify: `crates/muxsmith-core/Cargo.toml` (`ts-rs` optional dep + `ts` feature)
- Create: `.cargo/config.toml` (`[env]` block)
- Modify: `crates/muxsmith-core/src/profile/model.rs`, `crates/muxsmith-core/src/profile/match_expr.rs` (cfg_attr TS derives on the 20 model types)
- Modify: `crates/muxsmith-core/src/planner.rs` (the `ts` import + the cfg_attr TS derive on `StructuredEdit` - the one ts concern Task 6 deferred)
- Create: `crates/muxsmith-core/tests/ts_export.rs` (the export test + the keywords emitter)
- Create (committed, generated): `src/bindings/profile.ts`, `src/bindings/keywords.ts`
- Modify: `.github/workflows/ci.yml` (new Linux-leg drift step)

**Interfaces:**
- Consumes: Task 3's four `KEYWORDS` constants; Task 6's final `StructuredEdit` shape.
- Produces, for Tasks 9-13: `src/bindings/profile.ts` (the 20 model types **plus** `StructuredEdit`, 21 types total) and `src/bindings/keywords.ts` (`FILENAME_KEYWORDS`, `SOURCE_KEYWORDS`, `CHAPTERS_KEYWORDS`, `TITLE_KEYWORDS` as `as const` arrays).

**Read first:** design D44 (`:498-682`) in full - it carries the `[env]` block, the emitter, the drift step, and the measured evidence for each - and D49 §"Interface changes" > "What a ts-rs binding emits" (`:1201-1218`), which pins `StructuredEdit`'s generated shape.

Binding points:
- Generation is `cargo test -p muxsmith-core --features ts`, **not** an xtask, and **the reason is feature unification, not taste**: xtask would need `muxsmith-core = { features = ["ts"] }`, and Cargo unifies features across workspace members within one invocation, so `cargo build --workspace` would enable `ts` for every consumer of core and put `ts-rs` into the shipped cli and src-tauri builds. `-p muxsmith-core --features ts` cannot leak that way.
- Bindings are **committed, not built** (`core-06-schema-build-time-extraction` already mandates a committed generated artifact).
- `TS_RS_LARGE_INT = "number"` is **mandatory**: without it `Scalar::Int(i64)` maps to `bigint`, which does not survive `JSON.stringify` and would break the IPC wire at the model's most-used point.
- `export_to = "profile.ts"` without a trailing slash names a **file**, so all 21 types land in one file with no cross-imports.
- **`StructuredEdit`'s ts derive is added here, in `planner.rs`** (Task 6 deferred it): add `#[cfg(feature = "ts")] use ts_rs::TS;` and the `#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]` on `StructuredEdit`, per D49 §"The wire shape". The `cfg` on the import is not optional: a bare `use ts_rs::TS;` is an unused import on a default build and fails `cargo clippy -- -D warnings`.

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

`#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]` on each of the 20 model types - the 13 structs in D45's table (`:768-782`) plus the 7 enums (`FilenameCfg`, `SourceCfg`, `ChaptersCfg`, `TitleCfg`, `CollisionPolicy`, `KeepDrop`, `Scalar`) - **and** on `StructuredEdit` in `planner.rs` (the 21st type, D49), together with the `#[cfg(feature = "ts")] use ts_rs::TS;` import there. That set is the whole model reachable from `Profile` plus the one wire type the shell accepts back, with no residue.

- [ ] **Step 5: Write the export test and the keywords emitter**

Create `crates/muxsmith-core/tests/ts_export.rs`. ts-rs's `#[ts(export)]` writes `profile.ts` as a test side effect; the emitter for `keywords.ts` is ~12 lines and the design gives it verbatim at `:556-570`. It reads `TS_RS_EXPORT_DIR` so both artifacts share one destination. `ts-rs` exports **types, not values**, which is why the four keyword arrays need an emitter at all.

- [ ] **Step 6: Generate and commit the bindings**

```bash
cargo test -p muxsmith-core --features ts
ls -1 src/bindings/
```
Expected: `keywords.ts` and `profile.ts`. Inspect `profile.ts` and confirm two things: `Scalar` emits `boolean | number | number | string` (the duplicate `number` is cosmetic and expected - a `bigint` anywhere means `TS_RS_LARGE_INT` is not reaching the build), and `StructuredEdit` emits with `value: Scalar` on its two `add_exact`/`add_not_exact` members exactly as D49 measured (`:1214-1218`). Because the drift-check hole (a never-committed first-generation file) is not closed by the CI step until wave 3 (see step 8), this `ls` plus inspection **is** the first-generation check in-task.

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

Add a comment recording the step's one known hole, so it is not rediscovered as a bug: `git diff --exit-code` does not see a **new untracked** file (measured, D44 `:614-631`), so the gate catches a *stale* committed artifact from the first commit onward but cannot catch a never-committed first-generation one. **That hole is closed only in wave 3**, when Task 9's `src/editor/registries.ts` imports `keywords.ts` and a missing file fails the TypeScript build (`pnpm build` = `vue-tsc --noEmit && vite build`); between this merge and Task 9 nothing imports the bindings, so no gate closes it for two waves. Until then, step 6's `ls` + inspection is the check. `git status --porcelain` would close it directly and is deliberately **not** adopted: it would also fire on unrelated untracked files and turn every CI leg into a working-tree cleanliness assertion.

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
git add crates/muxsmith-core/Cargo.toml Cargo.lock .cargo/config.toml crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/src/profile/match_expr.rs crates/muxsmith-core/src/planner.rs crates/muxsmith-core/tests/ts_export.rs src/bindings/profile.ts src/bindings/keywords.ts .github/workflows/ci.yml
git -c commit.gpgsign=false commit -m "core: ts-rs generates the wire types (model + StructuredEdit) behind a ts feature, committed + CI drift-checked (D44, D49)"
```

---

## Wave 2

Streams A, B and C merge to master and the join (Task 5) lands, gate green after each merge. Then:

### Task 8: D42 - the editor's IPC surface

**Files:**
- Modify: `src-tauri/src/lib.rs` (3 new commands + `apply_suggestion` command + a `load_profile_body`; `invoke_handler` at `:440-452`)
- Modify: `src-tauri/src/error.rs` (`From<SaveError>`, `From<ApplyError>`)
- Modify: `locales/en/gui-common.ftl`, `locales/de/gui-common.ftl`
- Test: `src-tauri/src/lib.rs` unit tests, `src-tauri/src/error.rs` unit tests

**Interfaces:**
- Consumes: Task 2's `profile::save::{to_file, SaveError}`; Task 6's `planner::{apply_suggestion, ApplyError, StructuredEdit}`; the existing `profile::validate::{config_diagnostics, config_diagnostics_from_file}` (`profile/validate.rs:193`) and `report::json::config_only_document`.
- Produces, for Tasks 9-14: four commands and the `load_profile` document shape.

| command | signature |
|---|---|
| `load_profile` | `async fn load_profile(path: String) -> Result<serde_json::Value, IpcError>` |
| `save_profile` | `async fn save_profile(path: String, profile: Profile) -> Result<(), IpcError>` |
| `validate_profile_model` | `async fn validate_profile_model(profile: Profile) -> Result<serde_json::Value, IpcError>` |
| `apply_suggestion` | `async fn apply_suggestion(profile: Profile, config_path: String, edit: StructuredEdit) -> Result<Profile, IpcError>` |

**Read first:** design D42 (`:303-407`, **as amended by Task 1** - `load_profile` returns the `config_only_document` envelope plus a `"profile"` key, no bespoke struct), D43's command paragraph (`:417-421`), and D49 §"The shell mapping" (`:592-620`) and §"The catalog entries" (`:622-659`).

Binding points:
- **`load_profile` returns no bespoke struct.** Add a `load_profile_body(path: &Path) -> serde_json::Value`, mirroring the existing `validate_profile_body` (`src-tauri/src/lib.rs:174-177`): on `load::from_file(path)` -> `Ok(profile)`, build `report::json::config_only_document(&validate::config_diagnostics(&profile), None, &ShellRenderer)` and inject `doc["profile"] = serde_json::to_value(&profile)`; on `Err(d)` (a `ParseError`), build `report::json::config_only_document(&[d], None, &ShellRenderer)` and set `doc["profile"] = serde_json::Value::Null`. Its `config_diagnostics` array is then byte-identical in shape to `validate_profile`'s (both go through `config_only_document`/`rendered_diags`), with the added `"profile"` key the only difference (owner decision, `core-85`).
- **`validate_profile(path)` is kept, not changed, not retargeted, not renamed, not removed.** It has a live consumer at `src/views/BatchView.vue:118`, and the batch view has no model to send. The two commands are not redundant: one validates a file the user picked by path, the other validates a model the user is editing. Both funnel into the same `config_diagnostics`, so no logic is duplicated (spec 7).
- **`validate_profile_model` returns `serde_json::Value`, not a typed struct**, and is byte-identical in envelope to `validate_profile`/`load_profile` (all three go through `config_only_document`). It wraps `validate::config_diagnostics(&profile)` directly (no disk).
- All four are `async` on `on_blocking` (`:73-79`), but for **two different reasons**, and the distinction is the thing an implementer gets wrong by pattern-matching. `load_profile`/`save_profile` touch the disk. **`validate_profile_model` does not touch the disk at all** - `config_diagnostics` is pure (`validate.rs:20-21`). It is on `on_blocking` because it is CPU-bound work on every keystroke: it compiles every regex and parses every template, and Tauri 2 runs a non-`async` command on the main thread, so a plain `fn` would stall the webview on each edit. "Touches the disk" is not the criterion; "could stall the webview" is - which is why `get_settings`/`set_settings` are deliberately non-async despite real file I/O.
- The `Err` case stays what it is everywhere else in this file: the blocking task itself panicking. **Expected failures are diagnostics in the document, not `Err`.** The `Err(IpcError)` path exists only for `save_profile` and `apply_suggestion`, whose `SaveError`/`ApplyError` are genuine operational failures.

- [ ] **Step 1: Write the failing error-mapping and load-profile-shape tests**

First, in `src-tauri/src/error.rs`'s test module, mirroring the existing `settings_errors_map_to_distinct_codes` shape. The `ApplyError` variants and `IpcError` codes are D49's (`:530-559`, `:597-616`); Task 1 already repointed the design's section-2 catalog row to the same three:

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
fn apply_errors_map_to_distinct_codes() {
    let unparsable: IpcError = ApplyError::UnparsableConfigPath("not-a-rule-path".into()).into();
    let oob: IpcError = ApplyError::RuleIndexOutOfRange { index: 7, rules: 1 }.into();
    let noop: IpcError =
        ApplyError::EditChangedNothing { index: 0, property: "forced_track".into() }.into();

    assert_eq!(unparsable.code, "apply-unparsable-config-path");
    assert_eq!(unparsable.params["path"], "not-a-rule-path");
    assert_eq!(oob.code, "apply-rule-index-out-of-range");
    assert_eq!(oob.params["index"], "7");
    assert_eq!(oob.params["rules"], "1");
    assert_eq!(noop.code, "apply-edit-changed-nothing");
    assert_eq!(noop.params["index"], "0");
    assert_eq!(noop.params["property"], "forced_track");

    assert_ne!(unparsable.code, oob.code);
    assert_ne!(oob.code, noop.code);
    assert_ne!(unparsable.code, noop.code);
}
```

Then, in `src-tauri/src/lib.rs`'s test module, beside the existing `validate_profile_body_*` tests (`:554-589`), pin the `load_profile` document shape against `validate_profile`'s output (F5 / `core-85`). Both fixtures already exist in that module and must be **reused, not duplicated** (`testing-support-helpers`):

- The loadable-invalid profile is currently an **inline literal** inside `validate_profile_body_reports_validate_diagnostics_for_a_loadable_invalid_profile` (`lib.rs:569-572`: the zero-rules profile that yields `no-track-rules`). Extract it into a module-level `const LOADABLE_INVALID_PROFILE: &str = ...` and repoint **both** that existing test and the new shape test at it - so the two share one fixture instead of forking it.
- The parse/load-error case is the existing **missing-file** fixture: `validate_profile_body_reports_load_failure_with_no_mkvmerge_key` (`lib.rs:557-563`) passes a path to a file that was never written (`dir.path().join("missing.yaml")`). Reuse that same missing-file shape for the error half rather than inlining a fresh malformed-YAML string.

```rust
// Extracted from lib.rs:569-572 (was inline); the loadable-invalid test above
// is repointed here too.
const LOADABLE_INVALID_PROFILE: &str =
    "profile_version: 1\ninput: { pattern: '.*', extensions: [mkv] }\ntracks:\n  rules: []\n";

#[test]
fn load_profile_body_matches_validate_profile_diagnostics_and_adds_the_model() {
    let dir = tempfile::tempdir().unwrap();

    // A loadable but invalid profile: load_profile's diagnostics envelope is
    // byte-identical to validate_profile's, plus the parsed model under "profile".
    let invalid = dir.path().join("p.yaml");
    std::fs::write(&invalid, LOADABLE_INVALID_PROFILE).unwrap();
    let loaded = load_profile_body(&invalid);
    let validated = validate_profile_body(&invalid);
    assert_eq!(
        loaded["config_diagnostics"], validated["config_diagnostics"],
        "load_profile's config_diagnostics must be shape-identical to validate_profile's (core-85)"
    );
    assert_eq!(loaded["files"], validated["files"], "same envelope, not a second shape");
    assert!(!loaded["profile"].is_null(), "a loadable profile is present under \"profile\"");
    assert!(validated.get("profile").is_none(), "validate_profile carries no model");

    // A load failure (missing file, the same fixture shape as lib.rs:557-563):
    // "profile" is null and the single diagnostic still matches validate's.
    let missing = dir.path().join("missing.yaml"); // never written
    let loaded_missing = load_profile_body(&missing);
    let validated_missing = validate_profile_body(&missing);
    assert_eq!(loaded_missing["config_diagnostics"], validated_missing["config_diagnostics"]);
    assert!(loaded_missing["profile"].is_null(), "a load failure yields profile: null");
}
```

- [ ] **Step 2: Run to confirm they fail**

```bash
cargo test -p muxsmith-gui --lib
```
Expected: FAIL - the `From` impls and `load_profile_body` do not exist. (The package is `muxsmith-gui`, `src-tauri/Cargo.toml:2`; the binary named `muxsmith` is the CLI, `crates/muxsmith-cli/Cargo.toml:9` - do not confuse them.)

- [ ] **Step 3: Implement the mappings and the commands**

Add `From<SaveError>` (mapping `Io` -> `profile-save-io-failed`, `Serialize` -> `profile-save-failed`, each with a `detail` param) and `From<ApplyError>` (verbatim from D49 §"The shell mapping", `:597-616`) to `error.rs`. Then add `load_profile_body`, the four commands, and register each in the `invoke_handler` (`:440-452`). `apply_suggestion` calls core's `planner::apply_suggestion(&profile, &config_path, &edit).map_err(Into::into)`.

- [ ] **Step 4: Add the catalog entries, bilingual**

Five new `IpcError` codes across `locales/{en,de}/gui-common.ftl` (every `IpcError` code in this tree lives there today - `mkvmerge-spawn-failed`, `settings-io-failed`, `internal-task-failed` - so this follows the existing split). The three apply codes are verbatim from D49 §"The catalog entries" (`:627-641`); the two save codes match the register of the existing `{ $detail }` codes:

`locales/en/gui-common.ftl`:
```
profile-save-io-failed = The profile could not be written: { $detail }
profile-save-failed = The profile could not be serialized for saving: { $detail }
apply-unparsable-config-path = The suggestion could not be applied: "{ $path }" does not name a rule.
apply-rule-index-out-of-range = The suggestion could not be applied: no rule at index { $index } (rule count: { $rules }).
apply-edit-changed-nothing = The suggestion changed nothing: rule { $index } already constrains "{ $property }".
```

`locales/de/gui-common.ftl`:
```
profile-save-io-failed = Das Profil konnte nicht geschrieben werden: { $detail }
profile-save-failed = Das Profil konnte für das Speichern nicht serialisiert werden: { $detail }
apply-unparsable-config-path = Der Vorschlag konnte nicht angewendet werden: "{ $path }" benennt keine Regel.
apply-rule-index-out-of-range = Der Vorschlag konnte nicht angewendet werden: keine Regel an Index { $index } (Regelanzahl: { $rules }).
apply-edit-changed-nothing = Der Vorschlag hat nichts geändert: Regel { $index } schränkt "{ $property }" bereits ein.
```

`apply-rule-index-out-of-range` states the count as a labelled value `(rule count: { $rules })` / `(Regelanzahl: { $rules })`, never as a counted noun - D49 `:643-659` records why a Fluent plural selector cannot be used here and must not be added.

- [ ] **Step 5: Run the tests**

```bash
cargo test -p muxsmith-gui
pnpm check:i18n
```
Expected: PASS; `check:i18n` green (check 3 enforces en/de parity on the five new keys).

- [ ] **Step 6: Full gate, then commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/error.rs locales/en/gui-common.ftl locales/de/gui-common.ftl
git -c commit.gpgsign=false commit -m "shell: load/save/validate-model/apply commands and their error codes (D42, D43, D49)"
```

---

## Wave 3

Task 8 merges to master, gate green. Then the frontend, serial within stream E (`.worktrees/plan6-e`): Task 9 -> Task 10 -> Task 11 -> Task 12 -> Task 13. Task 14 is **wave 4**, sequenced after Task 13 (F4: not parallel with 9-13).

### Task 9: D45 - the registry data layer, catalogs, and the i18n gate

**Files:**
- Create: `src/editor/fieldSpec.ts` (the `FieldSpec`/`FieldWidget`/`RegistryName` types)
- Create: `src/editor/registries.ts` (the 13 registries + the option arrays + the completeness guards)
- Create: `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl`
- Modify: `scripts/check-i18n.mjs` (the `LABEL_KEY_RE` scan)

`e2e/catalogs.spec.ts` is **not** a deliverable of this task and needs no work: its single test delegates to `assertAllCatalogsParseCleanly`, which globs `locales/<tag>/*.ftl` (`e2e/i18n-en.ts`) and so picks up the new `gui-editor.ftl` automatically with zero change to either file (design `:1728-1731`). It is listed here only to record that the guard already covers the new catalog for free; do not stage it, do not edit it.

**Interfaces:**
- Consumes: Task 5's `src/bindings/profile.ts` and `src/bindings/keywords.ts`.
- Produces, for Tasks 10-13: the 13 exported registries, `FieldSpec`, `FieldWidget`, `RegistryName`, `COLLISION_POLICIES`, `KEEP_DROP`.

**Read first:** design D45 (`:685-1099`) in full. It carries the type definitions (`:694-706`, `:806-819`), the 13-struct table (`:768-782`), the **complete 43-field widget table** (`:848-894`), the option-array guard shape (`:905-909`), and the `check-i18n` fix (`:1082-1091`).

Binding points:
- **All 13 structs get a registry and all 7 enums get a `never` arm.** No subset: the registry's entire value is total coverage, and registering only a subset reintroduces the exact silent-absence failure the mechanism exists to close.
- **42 of 43 fields are `EditableField`; the one `FixedField` is `Profile.profile_version`**, which spec 4 pins at 1. Do **not** reach for `Omit<Profile, "profile_version">` - that silently disables the forcing function for that key forever.
- `FieldWidget` has **10 variants**, closed. `fixed` is **not** one of them: it is the other half of the `FieldSpec` union.
- **The four keyword arrays are imported from `src/bindings/keywords.ts`, never hand-written.** `COLLISION_POLICIES` and `KEEP_DROP` are declared here with the `satisfies` completeness guard, because TS can see their unions; the keyword domains are not in the TS type at all (the untagged enums project to `Block | string`), which is why they are generated instead.
- `gui-editor.ftl` carries exactly **43** keys: 42 labels + 1 save-surface note (Global Constraints ruling 2). **No tooltip budget** (ruling 4: the editor ships without tooltips; spec 8.3's editor baseline is Plan 7).
- Widget facets add **no** keys: `select` and `keywordOrBlock` render their options from the domain arrays, and those are profile-format tokens (`keep`, `drop`, `error`, `primary`), not prose - the same call D39 made for the `allowed` param.
- Three widget choices are settled by evidence, not by the Rust type, and are the ones most likely to be got wrong: `optionalFlag` is a checkbox whose off-state is **absence** (not a tri-state - `validate.rs:466-472` rejects `Some(false)`); `TextSyntax` has **four** values because `Locator.match_pattern` is a template in *regex* mode, a genuinely third thing from `Input.pattern`'s regex and `TemplateBlock.template`'s literal-mode template; `propertyMap.properties` is `matchable | settable` because `exact` and `changes` offer **different** domains (`codec_kind` is matchable-only).

- [ ] **Step 1: Write the types and the registries**

`src/editor/fieldSpec.ts` gets `EditableField`, `FixedField`, `FieldSpec`, `TextSyntax`, `FieldWidget`, `RegistryName` per `:694-706` and `:806-819`. `src/editor/registries.ts` gets the 13 registries, filled from the **43-row table at `:848-894`** - work it row by row; it is complete and it is the contract.

`reorderable` is semantic, not taste: `tracks.rules` is output track order and `attachments.rules` resolves first-match-wins in list order, so both reorder; `any` (logical OR) and `not` (logical NOR) carry no order, so neither does.

- [ ] **Step 2: Prove the registry-completeness proof fires (deliberate break)**

The registry's whole value is that it fails the **build**, not a test. Now that step 1 has created `registries.ts`, prove it:

```bash
# delete one entry from outputFields and run:
pnpm build
# Expected: error TS2741: Property 'on_collision' is missing in type '{...}'
#           but required in type 'Record<keyof OutputCfg, FieldSpec>'
# Restore it and confirm green. Record the observed error in your report.
```
Note: two or more missing keys report **TS2739** listing them all. Same check, two messages; the design does not depend on which fires.

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
git add src/editor/fieldSpec.ts src/editor/registries.ts locales/en/gui-editor.ftl locales/de/gui-editor.ftl scripts/check-i18n.mjs
git -c commit.gpgsign=false commit -m "gui: the field registry, its catalogs, and the label-key i18n gate (D45)"
```

---

### Task 10: D45 - the widget components

**Files:**
- Create: `src/editor/widgets/` - one component per `FieldWidget` variant (10)
- Create (test-mount harness, one-time - Step 1, see the amendment subsection): `e2e/mount-entry.ts`, `e2e/vite.mount.config.ts`, `e2e/mount.ts`
- Modify (test-mount harness, one-time - Step 1): `e2e/global.d.ts`, `e2e/tsconfig.json`, `package.json`
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 9's `FieldWidget`, `RegistryName`, option arrays.
- Produces, for Task 12: a widget dispatcher that renders any `FieldSpec`.

Binding points:
- **The frontend performs zero semantic validation** (spec 7). It holds the model as data, sends it, renders the returned diagnostics. Its only sanctioned local logic is the UX affordance spec 7 names: disabling Save while errors exist.
- **Sum types get an explicit `never` arm**: `const _exhaustive: never = x`. Both shapes fire, but only TS2322 **names** the unhandled variant, which is the same property that justifies the registry over the type. This is a deliberate, minimal improvement on the existing house shape (`src/jobRowState.ts:44-55`); `jobRowState.ts` is **not** required to change.
- **Cross-field constraints stay in core** (spec 7). Two exist in this surface and neither gets a widget: `AttachmentRule` requires exactly one of `select`/`drop`/`add`, and `Locator.match_to_source` is mutually exclusive with `match_pattern`. Both are already validated core-side and surface as diagnostics. A component **may** present the one-of as a mode selector - that is a UX affordance, not frontend semantic validation.
- The registry forces a label and widget to **exist** per field; it does **not** check the widget suits the field's type. That is accepted and recorded: a mismatched widget is a visible rendering bug caught the first time the panel opens, whereas a missing entry is silent absence. Do not add the mapped type `{ [K in keyof T]: FieldSpecFor<T[K]> }` - the brief settles the mechanism.
- **The wave-3 e2e RED/GREEN runs through the test-mount harness, not the served app** (amendment 2026-07-16, mount-harness routing). No editor mount point exists in the running app before Task 13 (`src/main.ts` mounts only `App.vue`, whose `View` union is `"batch" | "jobs"`; `EditorView.vue` is nav-wired only in Task 13; Playwright's `webServer` serves the single-entry `dist/`), so the step-1 assertions of Tasks 10-12 cannot render editor UI through `page.goto("/")`. Step 1 below builds a harness (reused verbatim by Tasks 11-12) that mounts the component under test in isolation. It passes the component's `props` verbatim and round-trips the standard Vue `modelValue`/`update:modelValue` v-model, exposing the live model via `window.__muxsmithModel__()`; it installs **no** Tauri IPC mock, because the widgets here and `EditorView` through Task 12 are fed their model as a prop - IPC wiring is Task 13.

- [ ] **Step 1: Set up the test-mount harness (one-time) (amended 2026-07-16, mount-harness routing)**

This harness is created here because Task 10 is the first task that needs a DOM-level render of editor UI, and it is reused verbatim by Tasks 11 and 12. It extends the established `e2e/vite.harness.config.ts` precedent (a pre-test Vite build into gitignored `e2e/.generated/`, injected into a plain page; it never touches `dist/` and never ships). Create three files and modify three; every file and its contents are named here, with no latitude to substitute another mechanism.

1. `e2e/mount-entry.ts` (create) - the page-side bundle source, pure `.ts` (runtime `h()`, no template), no static `.vue` import. Build a component registry with

   ```ts
   const modules = import.meta.glob<{ default: Component }>(
     ["../src/editor/widgets/*.vue", "../src/views/EditorView.vue"],
     { eager: true },
   );
   ```

   The `import.meta.glob` result is **path-keyed** (its keys are the full glob-relative paths, e.g. `"../src/editor/widgets/TextWidget.vue"`); do not build a separate basename-to-module map. The caller passes a bare basename (`TextWidget`, `FieldWidgetDispatcher`, `EditorView`, and the rest) and the mount driver reconstructs the path to index `modules` (see the `resolves` step below). `eager: true` is mandatory: an IIFE forbids code-splitting, which a lazy glob would introduce. Assign `window.__muxsmithMount__`, `window.__muxsmithModel__` and `window.__muxsmithEmitted__` as side effects (the window-global shape `e2e/tauri-mock-entry.ts` already uses). `__muxsmithMount__({ component, props, locale })`:
   - unmounts any previous app and resets `window.__muxsmithEmitted__ = []`;
   - resolves `modules["../src/editor/widgets/" + component + ".vue"]` (or the `EditorView` path), throwing `unknown mount component "<name>"` when absent - that throw is the Task-10/11/12 RED before the component exists;
   - creates a wrapper root that holds `const model = ref(props?.modelValue)`, renders `h(Comp, { ...props, modelValue: model.value, "onUpdate:modelValue": (v) => { model.value = v; window.__muxsmithEmitted__.push({ event: "update:modelValue", payload: v }); } })`, and sets `window.__muxsmithModel__ = () => model.value`;
   - `.use(createFluentVue({ bundles: buildBundles(locale ?? "en") }))`, importing `buildBundles` from `../src/i18n`. This is the load-bearing reuse: the real `locales/*/gui-editor.ftl` catalogs reach the page through the app's **own** `import.meta.glob` catalog loader (`src/i18n/index.ts`), so `$t` renders real messages, not stubs;
   - `.mount("#mount")`.
2. `e2e/vite.mount.config.ts` (create) - a second Vite build, parallel to `vite.harness.config.ts` but with the Vue plugin so `.vue` compiles and both `import.meta.glob` calls inline:

   ```ts
   import { resolve } from "node:path";
   import { defineConfig } from "vite";
   import vue from "@vitejs/plugin-vue";

   const here = import.meta.dirname;

   export default defineConfig({
     plugins: [vue()],
     build: {
       outDir: resolve(here, ".generated"),
       emptyOutDir: false, // must NOT wipe tauri-mock-harness.js, built by the step before
       minify: false,
       lib: {
         entry: resolve(here, "mount-entry.ts"),
         name: "MuxsmithMountHarness",
         formats: ["iife"],
         fileName: () => "mount-harness.js",
       },
     },
   });
   ```

   `emptyOutDir: false` is ordering-load-bearing: the `test:e2e` chain runs the tauri-mock build (which cleans `.generated/`) first, then this one, which must land `mount-harness.js` beside `tauri-mock-harness.js`, not replace it. The IIFE bundles Vue, fluent-vue and `@fluent/bundle` into one self-contained file exactly as `vite.harness.config.ts` bundles `@tauri-apps/api` today (that config's own doc calls its output a "dependency-free IIFE").
3. `e2e/mount.ts` (create) - the Playwright-side helper, parallel to `e2e/mocks.ts`:

   ```ts
   import { resolve } from "node:path";
   import type { Page } from "@playwright/test";

   const MOUNT_HARNESS_PATH = resolve(import.meta.dirname, ".generated/mount-harness.js");

   export interface MountSpec {
     component: string;
     props?: Record<string, unknown>;
     locale?: string;
   }

   export async function mountComponent(page: Page, spec: MountSpec): Promise<void> {
     await page.setContent('<!doctype html><div id="mount"></div>');
     await page.addScriptTag({ path: MOUNT_HARNESS_PATH });
     await page.evaluate((s) => window.__muxsmithMount__(s), spec);
   }

   export function readModel(page: Page): Promise<unknown> {
     return page.evaluate(() => window.__muxsmithModel__());
   }

   export function readEmitted(page: Page): Promise<Array<{ event: string; payload: unknown }>> {
     return page.evaluate(() => window.__muxsmithEmitted__);
   }
   ```
4. `e2e/global.d.ts` (modify) - add the three mount globals to the ambient `Window` interface, beside the existing `__muxsmithE2E__` block:

   ```ts
   __muxsmithMount__: (spec: { component: string; props?: Record<string, unknown>; locale?: string }) => void;
   __muxsmithModel__: () => unknown;
   __muxsmithEmitted__: Array<{ event: string; payload: unknown }>;
   ```
5. `e2e/tsconfig.json` (modify) - add `"vite/client"` to `compilerOptions.types`, making it `["node", "@playwright/test", "vite/client"]`, so the `tsc --noEmit -p e2e/tsconfig.json` gate types `import.meta.glob` in `mount-entry.ts` and in the transitively imported `src/i18n/index.ts`. **No `*.vue` module shim is added or needed**: components are reached only through `import.meta.glob` (whose result `vite/client` types), never through a static `import ... from "*.vue"` (which plain `tsc` cannot resolve in this tree - there is no `declare module "*.vue"` shim, and the app build relies on `vue-tsc` instead).
6. `package.json` (modify) - insert the mount build into the `test:e2e` chain, after the existing tauri-mock build and before `playwright test`:

   ```
   "test:e2e": "tsc --noEmit -p e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && vite build --config e2e/vite.mount.config.ts && playwright test"
   ```

- [ ] **Step 2: Write the failing mount-harness assertions (amended 2026-07-16, mount-harness routing)**

Extend `e2e/smoke.spec.ts` with per-widget rendering assertions that mount each widget through `e2e/mount.ts` (`mountComponent(page, { component: "TextWidget", props: { spec: <FieldSpec>, modelValue: <value> } })`), not the served app - `page.goto("/")` reaches no widget (there is no editor mount point until Task 13). Assert each widget renders its expected control with `getByRole` (e.g. `text` -> a textbox, `bool`/`optionalFlag` -> a checkbox, `select`/`keywordOrBlock` -> a combobox of its domain tokens), and that editing updates the held model via `readModel(page)`. Assert user-facing text against `e2e/i18n-en.ts` (the real en catalog), never a hand-duplicated literal, exactly as the existing smoke tests do.

- [ ] **Step 3: Run to confirm they fail (amended 2026-07-16, mount-harness routing)**

```bash
pnpm test:e2e
```
Expected: FAIL - `__muxsmithMount__` throws `unknown mount component "TextWidget"` because `src/editor/widgets/` is still empty, so the glob registry holds no widget. That throw is the genuine RED (the component does not exist yet).

- [ ] **Step 4: Implement the 10 widgets (amended 2026-07-16, mount-harness routing)**

One component per variant from `:806-819`. Follow the house component conventions - read two existing components first (`src/components/SuggestionCard.vue`, `src/components/JobRow.vue`) and match their prop/emit/`$t` style. Each widget exposes its editable value through the standard Vue `modelValue`/`update:modelValue` v-model, which is both the idiomatic shape and what the harness round-trips. Note the recorded `withDefaults` + `T | null` vue-tsc quirk in BUILDING.md's tooling section before fighting a type error.

- [ ] **Step 5: Run the e2e suite (amended 2026-07-16, mount-harness routing)**

```bash
pnpm build && pnpm test:e2e
```
Expected: PASS - `test:e2e` rebuilds `mount-harness.js` (the widgets are now in the glob) and the per-widget assertions render green. `pnpm build` still runs so Playwright's `vite preview` webServer has a `dist/` to boot, even though the mount assertions use `page.setContent`, not the served app.

- [ ] **Step 6: Full gate, then commit (amended 2026-07-16, mount-harness routing)**

```bash
git add src/editor/widgets e2e/mount-entry.ts e2e/vite.mount.config.ts e2e/mount.ts e2e/global.d.ts e2e/tsconfig.json package.json e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the ten field widgets, exhaustive by never-arm, plus the wave-3 test-mount harness (D45)"
```

---

### Task 11: D45 - the editor view, part a: the rule grid and drag-reorder

**Files:**
- Create: `src/views/EditorView.vue` (the rule grid + drag-reorder; sections and open/save follow in Tasks 12-13)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 5's `profile.ts` types (`Profile`, `TrackRule` (`model.rs:198`), ...).
- Produces, for Tasks 12-13: the `EditorView.vue` scaffold holding the rule list.

Binding points:
- The view holds the model as data; drag-reorder emits a **reordered model**, not a mutation of the DOM. `tracks.rules` is output track order (`reorderable` in the registry), so reordering is a semantic edit the user makes deliberately.
- No validation, no save, no widgets yet - those are Tasks 12 and 13. This task is the grid and its reordering only, so it stays reviewable as one unit.

- [ ] **Step 1: Write the failing e2e test (amended 2026-07-16, mount-harness routing)**

Extend `e2e/smoke.spec.ts`: mount `EditorView.vue` through the Task-10 harness (`e2e/mount.ts`), not the served app (which has no editor mount point until Task 13): `mountComponent(page, { component: "EditorView", props: { modelValue: <two-rule profile> } })`. Assert the rule grid renders both rows in order; perform a drag-reorder and assert both that the rendered rows swap and that `readModel(page)`'s `tracks.rules` reflects the new order (the harness round-trips `update:modelValue`). EditorView therefore takes the profile as its `modelValue` prop and emits `update:modelValue` on reorder - the natural pre-IPC shape; open/save IPC is Task 13.

- [ ] **Step 2: Run to confirm it fails (amended 2026-07-16, mount-harness routing)**

```bash
pnpm test:e2e
```
Expected: FAIL - `__muxsmithMount__` throws `unknown mount component "EditorView"` because `src/views/EditorView.vue` does not exist yet (the glob registry has no `EditorView`). That throw is the RED.

- [ ] **Step 3: Implement the rule grid and drag-reorder**

Create `src/views/EditorView.vue` with the rule list and its reordering. Match the house component conventions (read `src/views/BatchView.vue` first for the view-level prop/emit/`$t` shape). No nav wiring yet - `App.vue` is untouched until Task 13.

- [ ] **Step 4: Run the suite (amended 2026-07-16, mount-harness routing)**

```bash
pnpm build && pnpm lint && pnpm test:e2e
```
Expected: PASS - the harness rebuild picks up the new `EditorView.vue` and the mount assertions render green. `pnpm lint` includes the D27 `no-raw-text` rule - every string in the template comes from `$t`.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/views/EditorView.vue e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the profile editor's rule grid with drag-reorder (D45)"
```

---

### Task 12: D45 - the editor view, part b: section composition and widget dispatch

**Files:**
- Modify: `src/views/EditorView.vue` (section composition + widget dispatch over the 13 registries)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 9's 13 registries, Task 10's widget dispatcher, Task 11's `EditorView.vue` scaffold.
- Produces, for Task 13: an `EditorView.vue` that renders every field of the loaded profile through the right widget.

Binding points:
- **The section composition is driven by the 13 registries, not by hand-listed fields.** Each registry maps `keyof <Cfg>` to a `FieldSpec`; the view iterates the registry and dispatches Task 10's widget for each field's `FieldWidget`. Adding a field to the model + registry surfaces it here with no view edit - that is the registry's whole point.
- **The frontend performs zero semantic validation** (spec 7). It renders the field, holds the value, and (in Task 13) sends the model. No per-field validity logic here.
- The `FixedField` (`Profile.profile_version`) renders read-only; it has no `labelKey` and no widget (`FieldWidget` has no `fixed` variant).

- [ ] **Step 1: Write the failing e2e test (amended 2026-07-16, mount-harness routing)**

Extend `e2e/smoke.spec.ts`: mount `EditorView.vue` through the Task-10 harness (`e2e/mount.ts`), not the served app: `mountComponent(page, { component: "EditorView", props: { modelValue: <full profile> } })`. Assert each section (input, tracks, output, attachments, tags, and the rest) renders with its fields dispatched to the expected widget types (e.g. `optionalFlag` -> a checkbox, `select` -> a combobox of its domain tokens).

- [ ] **Step 2: Run to confirm it fails (amended 2026-07-16, mount-harness routing)**

```bash
pnpm test:e2e
```
Expected: FAIL - `EditorView` is in the glob registry (Task 11) and mounts, but without composed sections, so the per-section/per-widget assertions find nothing. That is this task's RED (the composition work is missing, not the component).

- [ ] **Step 3: Implement section composition and widget dispatch**

Drive the sections from the 13 registries and dispatch each field through Task 10's widget dispatcher. Do not hand-list fields.

- [ ] **Step 4: Run the suite (amended 2026-07-16, mount-harness routing)**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS - the composed sections render green through the mount harness.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/views/EditorView.vue e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: editor section composition and widget dispatch over the 13 registries (D45)"
```

---

### Task 13: D45 - the editor view, part c: open/save, the save note, nav, and ipc signatures

**Files:**
- Modify: `src/views/EditorView.vue` (open/save wiring + the save-surface note + validate-on-edit)
- Modify: `src/App.vue` (the `View` union at `:10`, the nav at `:71-96`, the mount block at `:98-112`)
- Modify: `src/ipc.ts` (hand-written command signatures for the four new commands)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 8's four commands, Task 12's composed `EditorView.vue`.
- Produces: the complete editor surface, reachable from the nav.

Binding points:
- **`ts-rs` types the model only; command signatures stay hand-written in `src/ipc.ts`** as they are today (D44's explicit out-of-scope). `load_profile`/`validate_profile_model` return the untyped `serde_json::Value` document; the frontend reads `config_diagnostics` and (for `load_profile`) `profile` off it.
- The save-surface note is a **standing note, not a modal**, stated once at the save surface, with **no detection** of whether comments are present (that would need the parser to see them). Its text is `editor-save-note` from Task 9.
- Validation runs through `validate_profile_model` on every edit (spec 7's "every profile edit"), and Save is disabled while errors exist - the one sanctioned frontend affordance.
- Apply-suggestion lives in the **batch view**, not here (Task 14). D41 records why the plan-scope pairing of editor+apply is not a UI-location one: they share the in-memory model's ownership.
- **The editor ships WITHOUT tooltips in Plan 6.** Spec 8.3's editor tooltip/inline-explanation baseline defers to Plan 7 (owner ruling 2026-07-16, folded into the design by Task 1, carried in `docs/ROADMAP.md:74-84`). Do **not** add tooltip keys; `gui-editor.ftl` stays 43 keys.
- Follow `App.vue:98-104`'s recorded reason for `v-show` over `v-if` when adding the third view (both views stay mounted so JobsView's live run listeners survive tab switches) - do **not** switch the block to `v-if`.
- **The Tasks 10-12 mount-harness specs (`e2e/mount.ts`) keep running and stay green alongside this task's real-app tests** (amendment 2026-07-16, mount-harness routing): they are neither deleted nor ported, so `EditorView` must stay mountable from an injected `modelValue`, and Task 13's `load_profile` wiring feeds that same model through the app's open flow rather than an unconditional on-mount fetch.

- [ ] **Step 1: Write the failing e2e test**

Extend `e2e/smoke.spec.ts`: the nav gains an editor tab; saving calls `save_profile`; the save note is visible at the save surface; Save is disabled while a diagnostic of severity error is present and enabled when clean; the editor tab stays mounted across a switch to Jobs and back.

- [ ] **Step 2: Run to confirm it fails**

```bash
pnpm test:e2e
```
Expected: FAIL - no nav entry, no save wiring.

- [ ] **Step 3: Implement open/save, the note, the nav entry, and the ipc signatures**

Wire `load_profile`/`save_profile`/`validate_profile_model` into `EditorView.vue`; add the four command signatures to `src/ipc.ts`; add the editor to `App.vue`'s `View` union, nav and `v-show` mount block; render the `editor-save-note`; disable Save while any error-severity diagnostic is present.

- [ ] **Step 4: Run the suite**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS. `pnpm lint` includes the D27 `no-raw-text` rule - every string in the template comes from `$t`.

- [ ] **Step 5: Full gate, then commit**

- **Review-check (mount-harness coverage survives, amendment 2026-07-16, mount-harness routing):** confirm `git diff <task-12-commit> -- e2e/smoke.spec.ts` shows no mount-harness spec deleted, ported to the served app, or guarded/skipped, and that they pass in this task's `pnpm test:e2e`. Confirm `EditorView` mounts from `modelValue` alone (no unconditional `load_profile` in `onMounted`; `load_profile` feeds the model through the app's open flow). A mount spec made green by an on-mount fetch or an injected IPC mock is a wave-3 coverage regression, not a passing gate.

```bash
git add src/views/EditorView.vue src/App.vue src/ipc.ts e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the editor's open/save with its standing note, nav entry and ipc signatures (D45, D41)"
```

---

## Wave 4

Task 13 merges to master, gate green. Then:

### Task 14: D43 - one-click apply in the batch view

Sequenced **after Task 13** (F4). Its only file it shares with waves 3 is `e2e/smoke.spec.ts`, and running after Task 13 means no concurrent writer - so no new spec file is needed.

**Files:**
- Modify: `src/components/SuggestionCard.vue` (add the apply control; update the stale D22 comment at `:6-13`)
- Modify: `src/components/DiagnosticsPanel.vue` (wire the emit through)
- Modify: `locales/en/gui-batch.ftl`, `locales/de/gui-batch.ftl` (2 keys)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 8's `apply_suggestion` command.
- Produces: the apply control.

Binding points:
- The frontend **forwards two opaque fields it never interprets** - `config_path` and `edit`. Core does all the interpreting. Do not parse `config_path` in TypeScript, and do not read into `edit` (it stays `unknown` in `src/ipc.ts:132`; D49 confirms the echo is type-preserving).
- The two new keys go in the existing **`gui-batch.ftl`**, beside `SuggestionCard.vue`'s current copy-button keys (`batch-suggestion-copy` / `batch-suggestion-copy-tooltip`, `gui-batch.ftl:54-55`), because the apply control lives in the batch view. Only the editor's own surface justified the new `gui-editor.ftl`.
- **The no-fix case has no apply button and that is not a gap to close.** `core-109-two-required-no-fix` records that two required rules colliding on one track yield no suggestion at all, only the partition report; the diagnostics panel renders it as it does today.
- Apply does not validate; the editor's existing round-trip does.

- [ ] **Step 1: Write the failing e2e test**

Extend `e2e/smoke.spec.ts`: a suggestion card renders an apply button; clicking it invokes `apply_suggestion` with the card's `config_path` and `edit` unmodified; a partition/no-fix diagnostic renders **no** apply button.

- [ ] **Step 2: Run to confirm it fails**

```bash
pnpm test:e2e
```
Expected: FAIL - no apply button.

- [ ] **Step 3: Implement, update the stale comment, and add the two bilingual keys**

Wire the apply button through `SuggestionCard.vue` -> `DiagnosticsPanel.vue` -> the `apply_suggestion` command. Add the two bilingual keys to `gui-batch.ftl`. **Update the D22 comment at `SuggestionCard.vue:6-13`**: it currently states `edit` "is deliberately never read" and that suggestions are "never applied", which Task 14 falsifies. Record that apply now reads `edit`, and that D41 supersedes D22's stated reason (the comment-preservation premise D22 rested on is dead - a canonical save does not preserve comments, so the editor+apply pairing survives on shared model ownership, not on the old machinery).

- [ ] **Step 4: Run the suite**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/components/SuggestionCard.vue src/components/DiagnosticsPanel.vue locales/en/gui-batch.ftl locales/de/gui-batch.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: one-click apply-suggestion in the batch view (D43, D49)"
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

**D49 adds one removal trigger** (D49 §"Removal trigger", `:1122-1137`): after D49 lands, change `delta_for`'s `AddExact` arm to `map.insert(property.clone(), Scalar::Str(scalar_display(value)))` and run the suite. If G1, G2 and G3 all fail, they are load-bearing and stay; if only G3 fails, G1/G2 are candidates for removal as localizers. They stay until that run happens (`proc-proposed-safeguard-stays`).

## Open, carried into the plan close, not assigned to a task

- **`gui-22` vs `exec-44-runlog-14day-autoprune` is a recorded-statement collision** in `product-boundaries.yaml`: `gui-22` (`:243-252`) says v1 keeps all run logs with pruning deferred to v1.x, while `exec-44` (`:15-23`) records D35 reversing exactly that to an automatic 14-day prune, which shipped. `gui-22` still carries `status: settled` with no supersession marker. Unrelated to Plan 6, found while reading Tier 2 for the design and independently confirmed by the design reviewer (design `:1947-1956`). Needs a controller action item; a recorded-statement collision is one of the observable contested criteria, so it routes to the owner as a spec question.
- **The D23 `runActive` reset re-check** stays listed under the ROADMAP's Plan-6 anchor although the design review established it is run-path only (`src/views/JobsView.vue:150-200`) and touches nothing this plan builds. It stays until an owner call re-points it, rather than being moved silently.
- **The `IpcError`-code i18n gap** (D49 `:654-659`): nothing gates `IpcError` codes against `gui-common.ftl`, and `IpcError` params are not number-promoted through Fluent. D49 sidesteps both for its three codes (labelled-value wording, no plural selector). The structural gap is a ROADMAP item with its own trigger, out of Plan 6's scope.
