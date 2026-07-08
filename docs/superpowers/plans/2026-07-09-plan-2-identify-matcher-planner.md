# Muxsmith Plan 2: Identify, Matcher, Planner, and dry-run CLI

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the validated profile model from Plan 1 into a working planning pipeline: identify source files via `mkvmerge -J`, evaluate match expressions against real tracks, resolve each primary file to a plan under strict uniqueness, generate the batch report with a batch-wide suggestion engine, and expose it through `muxsmith dry-run` and `muxsmith identify`.

**Architecture:** All logic stays in `muxsmith-core`, prose-free (code + params diagnostics only). New core modules: `capability::runtime` (shell out to the local mkvmerge for version/types/languages), `identify` (`-J` wrapper + in-memory cache + track model), `matcher` (pure match-expression evaluation), `discovery` (primary-file scan + external locator resolution), `planner` (per-file resolution + batch report + suggestion engine). The CLI gains `dry-run` and `identify` subcommands rendering the same diagnostic/report structures through Fluent. This is stage 2 of 4; execution (`run`) and the GUI follow in Plans 3-4.

**Tech Stack:** Rust stable (edition 2024). New runtime dependency: none required beyond the standard library for process spawning (`std::process::Command`); JSON parsing reuses `serde_json`. Dev/CI: `cargo-deny` (already installed), tiny MKV fixtures muxed by mkvmerge from committed wav/srt seeds.

## Global Constraints

- Spec is authoritative: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`. On conflict, the spec wins; flag the conflict instead of improvising. The design decisions and the suggestion-engine algorithm live in `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md` (D1-D6); the spec's normative sections were amended to match, so the spec remains the single source of truth.
- `muxsmith-core` emits NO user-facing prose. Diagnostics carry `code` + `params` only (spec 5.2). All human text lives in `locales/*/*.ftl`. Every new `DiagCode` needs a message in `locales/en/diagnostics.ftl` or CI's catalog-completeness guard fails.
- `#![deny(missing_docs)]` on both lib crates: every new public item needs real rustdoc (semantics, not a restatement of the name).
- Strict independent uniqueness (spec 2): each track rule resolves to exactly one track regardless of rule order; zero is `MissingTrack` (unless `optional`), two or more is `AmbiguousRule`; a track claimed by two rules is `OverlappingRules`. No ordered consumption, ever.
- mkvmerge is an EXTERNAL executable (spec, decision log): core never links it, only invokes it. Identification uses `mkvmerge -J <file>`. Runtime queries use `--version`, `--list-types`, `--list-languages`. The pinned identification schema is v20 (matches `generated.rs`); `-J` output carries `identification_format_version`.
- Source files are never modified. `dry-run` performs NO mux invocations, only `-J` identification (spec 5.5).
- License MIT. ASCII only in identifiers and code comments. No em-dashes or curly quotes in ANY file (prose included); umlauts and Ş stay intact.
- Git: commits and pushes are AUTHORIZED for this repo. Trailer on every commit: `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. GPG blocks agent commits; use `git -c commit.gpgsign=false commit ...`. Every GitHub interaction (push included) gets an entry in `gh-log.md`.
- Controller verifies test counts independently by re-running suites; never trust an implementer report's arithmetic (Plan 1: haiku implementers mis-totaled 5/13).
- Model split for SDD: haiku for transcription-shaped tasks (fixtures, mechanical wiring), sonnet for judgment tasks (matcher, planner, suggestion engine) and reviewers, strongest model for the final whole-branch review.
- All commands run from the repo root `~/Git/Muxsmith` unless stated otherwise.

---

### Task 1: cargo-deny supply-chain gate

**Files:**
- Create: `deny.toml`
- Modify: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: nothing.
- Produces: `cargo deny check` passing locally and a Linux-only CI job running it. No code change to the crates.

- [ ] **Step 1: Write `deny.toml`**

`deny.toml` (repo root):

```toml
# cargo-deny configuration (https://embarkstudios.github.io/cargo-deny/).
# Linux-only CI job; see .github/workflows/ci.yml. Advisories, license
# policy, and multiple-version detection for the whole workspace.

[advisories]
version = 2
yanked = "deny"
ignore = []

[licenses]
version = 2
# MIT project (spec 12); allow the permissive licenses our dependency tree
# actually uses. Extend deliberately, never with a blanket allow.
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-3.0",
    "Zlib",
]
confidence-threshold = 0.9

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
```

- [ ] **Step 2: Run cargo-deny to verify it passes**

Run: `cargo deny check`
Expected: `advisories ok`, `licenses ok`, `bans` ok-or-warn, `sources ok`. If a real dependency license is missing from `allow`, add exactly that license (do not widen the policy); if an advisory fires, record it and stop for a decision rather than blanket-ignoring.

- [ ] **Step 3: Add the CI job**

In `.github/workflows/ci.yml`, add a `deny` job that runs on Linux only (the matrix trim keeps branch pushes Linux-only regardless). Match the existing job style; install cargo-deny via the released action pin already used in the ecosystem, or `cargo install cargo-deny --locked` if the file installs tools that way. Add, adjacent to the existing `test` job:

```yaml
  deny:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install cargo-deny
        run: cargo install cargo-deny --locked --version 0.19.9
      - name: cargo deny check
        run: cargo deny check
```

(Read the current `ci.yml` first and mirror its checkout/toolchain action versions and caching; do not downgrade an action the file already pins higher.)

- [ ] **Step 4: Commit**

```bash
git add deny.toml .github/workflows/ci.yml
git -c commit.gpgsign=false commit -m "ci: add cargo-deny supply-chain gate (Linux-only)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

Log the push in `gh-log.md` when the branch is pushed (batch with a later push if convenient).

---

### Task 2: Unify `Locator.path` on `PathBuf` (D3)

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs` (the `Locator.path` field)
- Modify: `crates/muxsmith-core/tests/profile_load.rs` (the assertion comparing `external.path`)
- Test: `crates/muxsmith-core/tests/profile_load.rs`

**Interfaces:**
- Consumes: `Locator` (Plan 1).
- Produces: `Locator.path: PathBuf` (was `String`). serde/schemars treat `PathBuf` as a string, so profile format and JSON Schema are unchanged; only Rust-side joins in the planner become natural (`primary_dir.join(&locator.path)`).

- [ ] **Step 1: Adjust the failing assertion**

In `crates/muxsmith-core/tests/profile_load.rs`, the reference-profile test compares the external locator path. Change the string comparison to a path comparison:

```rust
assert_eq!(external.path, std::path::Path::new("."));
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p muxsmith-core --test profile_load reference_profile_parses`
Expected: FAIL (type mismatch: `PathBuf` vs `&str`, or the field is still `String`).

- [ ] **Step 3: Change the field type**

In `crates/muxsmith-core/src/profile/model.rs`, the `Locator` struct:

```rust
    /// Directory to search, relative to the primary file's directory, or
    /// absolute. A filesystem path (`PathBuf`); serialized as a plain string,
    /// so the profile format is unchanged. Use forward slashes in profiles
    /// for portability (Windows accepts them).
    pub path: PathBuf,
```

`PathBuf` is already imported in `model.rs`. No other field changes.

- [ ] **Step 4: Run the whole core suite**

Run: `cargo test -p muxsmith-core`
Expected: PASS. Fix any other call site that constructed `Locator.path` from a `String` literal in tests (there should be none outside `profile_load.rs`).

- [ ] **Step 5: Verify the JSON Schema is unchanged for `path`**

Run: `cargo run -p muxsmith-cli -- schema | python3 -c "import json,sys; s=json.load(sys.stdin); print('schema emitted ok')"`
Expected: prints `schema emitted ok` (the `path` property remains a string in the schema; `PathBuf` serializes as string).

- [ ] **Step 6: Commit**

```bash
git add crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/tests/profile_load.rs
git -c commit.gpgsign=false commit -m "refactor(core): Locator.path String -> PathBuf (D3)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 3: New diagnostic codes and Fluent messages (D1, D2, D4)

**Files:**
- Modify: `crates/muxsmith-core/src/report.rs` (add four `DiagCode` variants + a catalog-completeness test)
- Modify: `locales/en/diagnostics.ftl` (four message templates)
- Test: inline `#[cfg(test)]` in `report.rs`; the catalog guard test if it lives in a test file

**Interfaces:**
- Consumes: `DiagCode` (Plan 1).
- Produces four new codes, defined now so later tasks emit them and the catalog stays complete:
  - `DiagCode::CodecKindExactOnly` -> `"codec-kind-exact-only"`
  - `DiagCode::InvalidPropertyValue` -> `"invalid-property-value"`
  - `DiagCode::PathSeparatorInRenderedName` -> `"path-separator-in-rendered-name"`
  - `DiagCode::EmptyRenderedName` -> `"empty-rendered-name"`

- [ ] **Step 1: Write the failing test**

Append to the `#[cfg(test)] mod tests` in `crates/muxsmith-core/src/report.rs`:

```rust
    #[test]
    fn plan2_codes_are_registered_with_keys() {
        assert_eq!(DiagCode::CodecKindExactOnly.key(), "codec-kind-exact-only");
        assert_eq!(
            DiagCode::InvalidPropertyValue.key(),
            "invalid-property-value"
        );
        assert_eq!(
            DiagCode::PathSeparatorInRenderedName.key(),
            "path-separator-in-rendered-name"
        );
        assert_eq!(DiagCode::EmptyRenderedName.key(), "empty-rendered-name");
        // All four are in the ALL catalog (backs the CI completeness guard).
        for c in [
            DiagCode::CodecKindExactOnly,
            DiagCode::InvalidPropertyValue,
            DiagCode::PathSeparatorInRenderedName,
            DiagCode::EmptyRenderedName,
        ] {
            assert!(DiagCode::ALL.contains(&c), "{c:?} missing from ALL");
        }
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p muxsmith-core report::tests::plan2_codes_are_registered_with_keys`
Expected: FAIL (variants not defined).

- [ ] **Step 3: Add the variants**

In the `diag_codes! { ... }` invocation in `crates/muxsmith-core/src/report.rs`, add the config-time codes next to `UnknownProperty` and the planning-time codes next to `OutputCollision`. Insert after the `UnknownProperty` line (before `UnknownPropertySkew`):

```rust
    /// `codec_kind` was used under a `substring`/`regex` condition; it is a curated alias and matchable only under `exact` (spec 4.4). Pattern-match `codec_id` instead.
    CodecKindExactOnly => "codec-kind-exact-only",
    /// An `exact` condition value lies outside a closed value domain: `type`/`codec_kind` are checked at config time, `language` at plan time (spec 4.4). `property`/`value` params carry the offender; `allowed` a hint sample.
    InvalidPropertyValue => "invalid-property-value",
```

Insert after the `OutputCollision` line (before `SourceOverwrite`):

```rust
    /// The rendered output filename contains a path separator (`/` or `\`); v1 never creates subdirectories, checked on the rendered name on all platforms (spec 4.8).
    PathSeparatorInRenderedName => "path-separator-in-rendered-name",
    /// The rendered output filename has an empty stem or is `.`/`..` (spec 4.8); the ".mkv appended if missing" rule would otherwise produce a hidden or invalid file.
    EmptyRenderedName => "empty-rendered-name",
```

- [ ] **Step 4: Add the Fluent messages**

Append to `locales/en/diagnostics.ftl` (keep the existing alphabetical-ish grouping loose; placement is not enforced):

```
codec-kind-exact-only = Property "codec_kind" can only be used with exact, not { $condition }. Match codec_id with { $condition } instead.
invalid-property-value = Value "{ $value }" is not valid for property "{ $property }". Allowed values include: { $allowed }.
path-separator-in-rendered-name = The rendered output filename "{ $name }" contains a path separator; Muxsmith never creates subdirectories.
empty-rendered-name = The rendered output filename is empty or invalid ("{ $name }").
```

- [ ] **Step 5: Run the test and the catalog guard**

Run: `cargo test -p muxsmith-core`
Expected: PASS, including the catalog-completeness guard (the CI guard from Plan 1 that checks every `DiagCode::ALL` entry has a Fluent template). If the guard lives in the CLI crate, run `cargo test --workspace`.

- [ ] **Step 6: Commit**

```bash
git add crates/muxsmith-core/src/report.rs locales/en/diagnostics.ftl
git -c commit.gpgsign=false commit -m "feat(core): add Plan 2 diagnostic codes (D1/D2/D4) with messages

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 4: Curated value domains for D2 validation

**Design note (grounded in the upstream schema at `~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json`):** the track `type` is schema-typed as a plain string with NO enum, and the only enum-bearing track field upstream is `aac_is_sbr` (not a D2 target). So there is nothing worth extracting from the schema for D2, and building an xtask codegen path for one irrelevant field would be an abstraction the scale has not earned. The two closed domains D2 needs (`type`, `codec_kind`) are stable and are curated directly in `capability`. If a future schema version enums more properties we care about, add generation then.

**Files:**
- Modify: `crates/muxsmith-core/src/capability/mod.rs` (add `matchable_domain`, `TYPE_VALUES`, `CODEC_KIND_NAMES`)
- Test: inline tests in `capability/mod.rs`

**Interfaces:**
- Consumes: the capability module (Plan 1).
- Produces: `capability::matchable_domain(name: &str) -> Option<&'static [&'static str]>`: the closed value domain of a matchable property, or `None` if open-valued. `type` returns `TYPE_VALUES`, `codec_kind` returns `CODEC_KIND_NAMES` (the curated alias keys); every other property (including `language`, whose domain is runtime-only and checked at plan time) returns `None`.

- [ ] **Step 1: Write the failing tests**

Add to the `#[cfg(test)] mod tests` in `crates/muxsmith-core/src/capability/mod.rs`:

```rust
    #[test]
    fn value_domains_are_closed_for_type_and_codec_kind() {
        let type_domain = matchable_domain("type").expect("type has a closed domain");
        assert!(type_domain.contains(&"video"));
        assert!(type_domain.contains(&"audio"));
        assert!(type_domain.contains(&"subtitles"));
        let ck = matchable_domain("codec_kind").expect("codec_kind has a closed domain");
        assert!(ck.contains(&"srt"));
        assert!(ck.contains(&"h264"));
        // Open-valued or runtime-domain properties: no closed domain here.
        assert_eq!(matchable_domain("track_name"), None);
        assert_eq!(matchable_domain("language"), None); // validated at plan time
    }

    #[test]
    fn codec_kind_domain_matches_kinds() {
        let from_kinds: Vec<&str> = CODEC_KINDS.iter().map(|(k, _)| *k).collect();
        assert_eq!(from_kinds, CODEC_KIND_NAMES);
    }
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test -p muxsmith-core capability`
Expected: FAIL (`matchable_domain`/`TYPE_VALUES`/`CODEC_KIND_NAMES` not defined).

- [ ] **Step 3: Implement the curated domains**

In `crates/muxsmith-core/src/capability/mod.rs`, add (near `matchable_type`):

```rust
/// The closed set of `type` values mkvmerge reports for a track. Curated
/// rather than generated: the upstream identification schema (v20) types
/// `type` as a plain string with no enum, and mkvmerge's track types are
/// long-stable. Verified against `mkvmerge -J` output, where `track.type` is
/// exactly one of these.
pub static TYPE_VALUES: &[&str] = &["audio", "buttons", "subtitles", "video"];

/// The curated `codec_kind` alias names ([`CODEC_KINDS`] keys), the closed
/// domain of the `codec_kind` virtual property. Kept in sync with
/// `CODEC_KINDS` by the `codec_kind_domain_matches_kinds` test.
pub static CODEC_KIND_NAMES: &[&str] = &[
    "srt", "ass", "pgs", "vobsub", "webvtt", "aac", "ac3", "eac3", "dts",
    "truehd", "flac", "opus", "mp3", "h264", "h265", "av1", "vp9",
];

/// The closed set of legal values for a matchable property, or `None` if the
/// property is open-valued (free text, numbers) or has a runtime-only domain.
/// Backs the config-time `InvalidPropertyValue` check (spec 4.4, D2): `type`
/// and `codec_kind` are curated closed sets; `language`'s domain needs
/// `mkvmerge --list-languages` and is validated at plan time, so it is `None`
/// here.
pub fn matchable_domain(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "type" => Some(TYPE_VALUES),
        "codec_kind" => Some(CODEC_KIND_NAMES),
        _ => None,
    }
}
```

- [ ] **Step 4: Run the core suite**

Run: `cargo test -p muxsmith-core capability`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/muxsmith-core/src/capability/mod.rs
git -c commit.gpgsign=false commit -m "feat(capability): curated value domains (type, codec_kind) for D2

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 5: Config-time hardening (D1, D2 config-time, D5)

**Files:**
- Modify: `crates/muxsmith-core/src/profile/validate.rs` (codec_kind exact-only; value-domain checks for `type`/`codec_kind`)
- Modify: `crates/muxsmith-core/src/template.rs` (add the brace-semantics locking test only; no code change intended)
- Test: inline tests in `validate.rs` and `template.rs`; a fixture-based test in `crates/muxsmith-core/tests/validate_hardening.rs`

**Interfaces:**
- Consumes: `capability::matchable_domain` (Task 4), the new codes (Task 3), the existing `validate_expr` machinery (Plan 1).
- Produces: `validate::validate` additionally emits `CodecKindExactOnly` (codec_kind under substring/regex) and `InvalidPropertyValue` (exact `type`/`codec_kind` value outside its domain). No signature change; still returns `Vec<Diagnostic>`.

- [ ] **Step 1: Write the failing tests**

Create `crates/muxsmith-core/tests/validate_hardening.rs`:

```rust
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::DiagCode;

fn codes(yaml: &str) -> Vec<DiagCode> {
    let p = from_str(yaml, Format::Yaml).expect("parses");
    validate(&p).into_iter().map(|d| d.code).collect()
}

const HEAD: &str = "profile_version: 1\ninput: { pattern: 'E(\\d+)', extensions: [mkv] }\ntracks:\n";

#[test]
fn codec_kind_under_substring_is_exact_only() {
    let y = format!("{HEAD}  - match: {{ substring: {{ codec_kind: srt }} }}\n");
    assert!(codes(&y).contains(&DiagCode::CodecKindExactOnly));
}

#[test]
fn codec_kind_under_regex_is_exact_only() {
    let y = format!("{HEAD}  - match: {{ regex: {{ codec_kind: 'sr.' }} }}\n");
    assert!(codes(&y).contains(&DiagCode::CodecKindExactOnly));
}

#[test]
fn codec_kind_under_exact_is_allowed() {
    let y = format!("{HEAD}  - match: {{ exact: {{ codec_kind: srt }} }}\n");
    let c = codes(&y);
    assert!(!c.contains(&DiagCode::CodecKindExactOnly));
    assert!(!c.contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn bad_type_value_is_invalid_property_value() {
    let y = format!("{HEAD}  - match: {{ exact: {{ type: vdieo }} }}\n");
    assert!(codes(&y).contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn bad_codec_kind_value_is_invalid_property_value() {
    let y = format!("{HEAD}  - match: {{ exact: {{ codec_kind: nope }} }}\n");
    assert!(codes(&y).contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn good_type_value_passes() {
    let y = format!("{HEAD}  - match: {{ exact: {{ type: subtitles }} }}\n");
    assert!(!codes(&y).contains(&DiagCode::InvalidPropertyValue));
}
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test -p muxsmith-core --test validate_hardening`
Expected: FAIL (checks not yet implemented).

- [ ] **Step 3: Implement the codec_kind exact-only check**

In `crates/muxsmith-core/src/profile/validate.rs`, inside `validate_expr`, in the loop over `substring`/`regex` maps, add a `codec_kind` guard BEFORE the string-type check (so codec_kind reports `CodecKindExactOnly`, not `NotStringProperty`, since it is String-typed). Replace the inner match on `prop_type(prop)` in that loop with:

```rust
                if prop == "codec_kind" {
                    diags.push(
                        Diagnostic::error(DiagCode::CodecKindExactOnly, p.clone())
                            .with("condition", kind.to_string()),
                    );
                    // Skip the type/regex checks below: the property is
                    // misused regardless of value, one diagnostic is enough.
                    if kind == "regex"
                        && let Err(e) = regex::Regex::new(value)
                    {
                        // Still surface a broken regex so a second edit is not
                        // needed once codec_kind is moved to exact.
                        diags.push(
                            Diagnostic::error(DiagCode::InvalidRegex, p.clone())
                                .with("detail", flatten_regex_error(&e)),
                        );
                    }
                    continue;
                }
                match prop_type(prop) {
                    None => diags.push(unknown_property(&p, prop)),
                    Some(PropType::String) => {}
                    Some(t) => diags.push(
                        Diagnostic::error(DiagCode::NotStringProperty, p.clone())
                            .with("property", prop.clone())
                            .with("actual_type", type_label(t))
                            .with("condition", kind.to_string()),
                    ),
                }
                if kind == "regex"
                    && let Err(e) = regex::Regex::new(value)
                {
                    diags.push(
                        Diagnostic::error(DiagCode::InvalidRegex, p)
                            .with("detail", flatten_regex_error(&e)),
                    );
                }
```

(This preserves the existing behavior for every non-`codec_kind` property and only diverts `codec_kind`.)

- [ ] **Step 4: Implement the config-time value-domain check**

Still in `validate_expr`, in the `exact` branch, after the existing type-fit check, add a value-domain check for closed-domain properties. Replace the `Some(t) => { ... }` arm of the `exact` loop's `match prop_type(prop)` with:

```rust
                Some(t) => {
                    if !scalar_fits(value, t) {
                        diags.push(
                            Diagnostic::error(DiagCode::ValueTypeMismatch, p.clone())
                                .with("property", prop.clone())
                                .with("expected", type_label(t))
                                .with("found", value.type_name()),
                        );
                    } else if let (Scalar::Str(s), Some(domain)) =
                        (value, capability::matchable_domain(prop))
                    {
                        // Closed-domain string property (type, codec_kind):
                        // the value must be one of the domain members (D2).
                        // language is deliberately absent from the domain map
                        // (validated at plan time), so this never fires for it.
                        if !domain.contains(&s.as_str()) {
                            diags.push(
                                Diagnostic::error(DiagCode::InvalidPropertyValue, p.clone())
                                    .with("property", prop.clone())
                                    .with("value", s.clone())
                                    .with("allowed", domain_hint(domain)),
                            );
                        }
                    }
                }
```

Add a small helper near `type_label`:

```rust
/// A short, deterministic sample of a closed value domain for the
/// `InvalidPropertyValue` hint: the full list if small, else the first few
/// plus an ellipsis marker. Keeps the message bounded for large domains.
fn domain_hint(domain: &[&str]) -> String {
    const MAX: usize = 8;
    if domain.len() <= MAX {
        domain.join(", ")
    } else {
        format!("{}, ...", domain[..MAX].join(", "))
    }
}
```

`capability` and `Scalar` are already imported in `validate.rs`.

- [ ] **Step 5: Run the hardening tests**

Run: `cargo test -p muxsmith-core --test validate_hardening`
Expected: PASS (6 tests).

- [ ] **Step 6: Add the D5 brace-semantics locking test**

In `crates/muxsmith-core/src/template.rs` tests, add a test locking the treatment of a lone `}` and reaffirming the unclosed-`{` error (no code change expected; if it fails, the fix is to make a lone `}` a literal, never to relax the unclosed-`{` error):

```rust
    #[test]
    fn lone_closing_brace_is_literal_and_lone_open_is_error() {
        // A lone `}` cannot start a field, so it is a literal.
        let t = Template::parse("a}b").expect("lone } is literal");
        assert_eq!(t.render_literal(&Ctx::new()), "a}b");
        // A lone unclosed `{` is a hard error (ambiguous, user mistake).
        assert!(matches!(
            Template::parse("a{b"),
            Err(TemplateError::UnclosedBrace { .. })
        ));
    }
```

- [ ] **Step 7: Run the template tests**

Run: `cargo test -p muxsmith-core template`
Expected: PASS. If `lone_closing_brace_is_literal_and_lone_open_is_error` fails on the `a}b` case, adjust `Template::parse` so a `}` with no preceding open brace is pushed as a literal character (it already falls through to the catch-all `c => literal.push(c)` arm, so this should pass unchanged; if not, that is the only permitted fix).

- [ ] **Step 8: Commit**

```bash
git add crates/muxsmith-core/src/profile/validate.rs crates/muxsmith-core/src/template.rs crates/muxsmith-core/tests/validate_hardening.rs
git -c commit.gpgsign=false commit -m "feat(core): codec_kind exact-only and value-domain validation (D1/D2/D5)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 6: Runtime mkvmerge capability queries

**Files:**
- Create: `crates/muxsmith-core/src/capability/runtime.rs`
- Modify: `crates/muxsmith-core/src/capability/mod.rs` (declare and re-export `runtime`)
- Test: inline unit tests (pure parsers) + `crates/muxsmith-core/tests/mkvmerge_runtime.rs` (gated on a real mkvmerge)

**Interfaces:**
- Consumes: nothing from earlier Plan 2 tasks.
- Produces:
  - `capability::runtime::Mkvmerge`: a resolved handle to the external binary. `Mkvmerge::locate() -> Result<Mkvmerge, RuntimeError>` (PATH, then a configured override via `Mkvmerge::at(PathBuf)`). Holds the resolved path.
  - `Mkvmerge::version(&self) -> Result<String, RuntimeError>` (the raw version line).
  - `Mkvmerge::list_types(&self) -> Result<Vec<String>, RuntimeError>`: lowercase extensions the local mkvmerge accepts, deduped and sorted.
  - `Mkvmerge::list_languages(&self) -> Result<LanguageIndex, RuntimeError>`.
  - `capability::runtime::LanguageIndex`: normalizes a language token to a canonical key so that `en` and `eng` compare equal. `LanguageIndex::normalize(&self, token: &str) -> Option<String>` returns the ISO 639-2/T (three-letter) canonical code for any recognized 639-1/639-2/639-3 token (case-insensitive), or `None` if unrecognized. `LanguageIndex::from_rows(rows: &[[&str; 4]]) -> LanguageIndex` builds one directly (tests use it; the parser uses it internally).
  - Pure parsers, testable without a subprocess: `runtime::parse_list_types(output: &str) -> Vec<String>` and `runtime::parse_list_languages(output: &str) -> LanguageIndex`.
  - `runtime::RuntimeError`: `{ NotFound, Spawn(String), NonZero { code: Option<i32>, stderr: String }, Parse(String) }` (all `Debug`; core maps these to diagnostics at call sites in later tasks, so no prose here).

- [ ] **Step 1: Write the failing unit tests (pure parsers)**

`crates/muxsmith-core/src/capability/runtime.rs` tests at the bottom. These pin the real mkvmerge output formats captured from v99:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_list_types_extensions() {
        let sample = "Supported file types:\n  \
            Dolby Digital/Dolby Digital Plus (AC-3, E-AC-3) [ac3 eac3 eb3 ec3]\n  \
            AAC (Advanced Audio Coding) [aac m4a mp4]\n  \
            Matroska [mkv mka mks mk3d webm]\n";
        let exts = parse_list_types(sample);
        assert!(exts.contains(&"mkv".to_string()));
        assert!(exts.contains(&"ac3".to_string()));
        assert!(exts.contains(&"mp4".to_string()));
        // Deduped and sorted.
        let mut sorted = exts.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(exts, sorted);
    }

    #[test]
    fn parses_list_languages_into_normalizer() {
        // Columns: English name | 639-3 | 639-2 | 639-1 (pipe-separated,
        // trailing columns may be blank). Captured from mkvmerge v99.
        let sample = "\
English language name | ISO 639-3 code | ISO 639-2 code | ISO 639-1 code\n\
----------------------+----------------+----------------+---------------\n\
English               | eng            | eng            | en\n\
German                | ger            | ger            | de\n\
Klingon               | tlh            |                |   \n";
        let idx = parse_list_languages(sample);
        // 639-1, 639-2, 639-3 all normalize to the same canonical key.
        let en = idx.normalize("en");
        assert!(en.is_some());
        assert_eq!(idx.normalize("eng"), en);
        assert_eq!(idx.normalize("EN"), en); // case-insensitive
        let de = idx.normalize("de");
        assert_eq!(idx.normalize("ger"), de);
        assert_ne!(en, de);
        // A language with only a 639-3 code still normalizes.
        assert!(idx.normalize("tlh").is_some());
        // Unknown token: None.
        assert_eq!(idx.normalize("zz-not-a-lang"), None);
    }

    #[test]
    fn language_index_from_rows_builds_directly() {
        let idx = LanguageIndex::from_rows(&[["English", "eng", "eng", "en"]]);
        assert_eq!(idx.normalize("en"), idx.normalize("eng"));
    }
}
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test -p muxsmith-core runtime`
Expected: FAIL (module not defined).

- [ ] **Step 3: Implement `runtime.rs`**

`crates/muxsmith-core/src/capability/runtime.rs`:

```rust
//! Runtime queries against the external mkvmerge (spec 4.4, 9): version,
//! supported file types, and the language table used to normalize match
//! values. Core shells out via `std::process::Command`; it never links
//! mkvmerge. The text-parsing halves are pure and unit-tested; the spawning
//! halves are covered by an integration test gated on a real mkvmerge.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A resolved handle to the local mkvmerge executable.
#[derive(Debug, Clone)]
pub struct Mkvmerge {
    path: PathBuf,
}

/// Failure of a runtime mkvmerge query. Data only; call sites in later tasks
/// map these to diagnostics (core stays prose-free).
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// No mkvmerge executable could be located (PATH and override both failed).
    NotFound,
    /// The process could not be spawned; the string is the OS error.
    Spawn(String),
    /// The process ran but exited non-zero.
    NonZero {
        /// Exit code, or `None` if the process was terminated by a signal.
        code: Option<i32>,
        /// Captured stderr, trimmed.
        stderr: String,
    },
    /// The output could not be parsed into the expected shape.
    Parse(String),
}

impl Mkvmerge {
    /// Uses the executable at `path` without searching PATH (an app-settings
    /// or `--mkvmerge` override; spec 8.2). The path is not probed here;
    /// the first query surfaces a `Spawn`/`NotFound` error if it is wrong.
    pub fn at(path: impl Into<PathBuf>) -> Mkvmerge {
        Mkvmerge { path: path.into() }
    }

    /// Locates mkvmerge on PATH by spawning `mkvmerge --version`. Returns
    /// `NotFound` if the spawn fails with a not-found OS error. Platform-
    /// standard install-location probing (spec 8.2) is a GUI/first-run concern
    /// deferred to Plan 4; the CLI relies on PATH plus the explicit override.
    pub fn locate() -> Result<Mkvmerge, RuntimeError> {
        let m = Mkvmerge { path: PathBuf::from("mkvmerge") };
        match m.version() {
            Ok(_) => Ok(m),
            Err(RuntimeError::Spawn(_)) => Err(RuntimeError::NotFound),
            Err(e) => Err(e),
        }
    }

    /// The resolved executable path (PATH-relative `mkvmerge` or an override).
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn run(&self, args: &[&str]) -> Result<String, RuntimeError> {
        let out = Command::new(&self.path)
            .args(args)
            .output()
            .map_err(|e| RuntimeError::Spawn(e.to_string()))?;
        if !out.status.success() {
            return Err(RuntimeError::NonZero {
                code: out.status.code(),
                stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
            });
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    /// The raw first line of `mkvmerge --version`.
    pub fn version(&self) -> Result<String, RuntimeError> {
        let out = self.run(&["--version"])?;
        Ok(out.lines().next().unwrap_or("").trim().to_string())
    }

    /// Lowercase source-file extensions the local mkvmerge accepts, from
    /// `--list-types`, deduped and sorted (spec 4.2 validation input).
    pub fn list_types(&self) -> Result<Vec<String>, RuntimeError> {
        Ok(parse_list_types(&self.run(&["--list-types"])?))
    }

    /// The language normalization index from `--list-languages` (spec 4.4).
    pub fn list_languages(&self) -> Result<LanguageIndex, RuntimeError> {
        Ok(parse_list_languages(&self.run(&["--list-languages"])?))
    }
}

/// Extracts the bracketed extension lists from `mkvmerge --list-types` output.
/// Each supported-type line ends with `[ext1 ext2 ...]`; we collect every
/// token, lowercase, dedupe, and sort.
pub fn parse_list_types(output: &str) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();
    for line in output.lines() {
        let (Some(open), Some(close)) = (line.rfind('['), line.rfind(']')) else {
            continue;
        };
        if close <= open + 1 {
            continue;
        }
        for tok in line[open + 1..close].split_whitespace() {
            exts.push(tok.to_ascii_lowercase());
        }
    }
    exts.sort();
    exts.dedup();
    exts
}

/// A language-token normalizer built from `mkvmerge --list-languages`.
/// Maps any recognized ISO 639-1/639-2/639-3 token (case-insensitive) to a
/// single canonical key (the 639-3 code, always present upstream), so that
/// profile values like `de` and file values like `ger` compare equal.
#[derive(Debug, Clone, Default)]
pub struct LanguageIndex {
    /// lowercased token -> canonical key.
    to_canonical: BTreeMap<String, String>,
}

impl LanguageIndex {
    /// Builds an index from `[english_name, iso639_3, iso639_2, iso639_1]`
    /// rows (blank cells allowed). The canonical key is the 639-3 code, or the
    /// first non-empty code if 639-3 is blank. Every non-empty code in the row
    /// maps to that key.
    pub fn from_rows(rows: &[[&str; 4]]) -> LanguageIndex {
        let mut to_canonical = BTreeMap::new();
        for row in rows {
            let codes = [row[1], row[2], row[3]]; // 639-3, 639-2, 639-1
            let canonical = codes
                .iter()
                .map(|c| c.trim())
                .find(|c| !c.is_empty());
            let Some(canonical) = canonical else { continue };
            let canonical = canonical.to_ascii_lowercase();
            for code in codes {
                let code = code.trim();
                if !code.is_empty() {
                    to_canonical.insert(code.to_ascii_lowercase(), canonical.clone());
                }
            }
        }
        LanguageIndex { to_canonical }
    }

    /// The canonical key for `token`, or `None` if unrecognized. Matching is
    /// case-insensitive. Two tokens are the same language iff their canonical
    /// keys are equal.
    pub fn normalize(&self, token: &str) -> Option<String> {
        self.to_canonical.get(&token.trim().to_ascii_lowercase()).cloned()
    }
}

/// Parses `mkvmerge --list-languages` table output into a [`LanguageIndex`].
/// The table is `name | 639-3 | 639-2 | 639-1`, pipe-separated, with a header
/// row and a `---+---` separator row that are skipped.
pub fn parse_list_languages(output: &str) -> LanguageIndex {
    let mut owned: Vec<[String; 4]> = Vec::new();
    for line in output.lines() {
        if !line.contains('|') {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() < 4 {
            continue;
        }
        // Skip the header (contains "639") and the separator (dashes/plus).
        if cols[1].contains("639") || cols[0].chars().all(|c| c == '-' || c == '+' || c == ' ') {
            continue;
        }
        owned.push([
            cols[0].to_string(),
            cols[1].to_string(),
            cols[2].to_string(),
            cols[3].to_string(),
        ]);
    }
    let rows: Vec<[&str; 4]> = owned
        .iter()
        .map(|r| [r[0].as_str(), r[1].as_str(), r[2].as_str(), r[3].as_str()])
        .collect();
    LanguageIndex::from_rows(&rows)
}
```

Add to `crates/muxsmith-core/src/capability/mod.rs`, after `mod generated;`:

```rust
pub mod runtime;
```

- [ ] **Step 4: Run the pure-parser tests**

Run: `cargo test -p muxsmith-core runtime`
Expected: PASS (3 tests).

- [ ] **Step 5: Write the mkvmerge-gated integration test**

`crates/muxsmith-core/tests/mkvmerge_runtime.rs`:

```rust
//! Integration tests that spawn the real mkvmerge. Skipped (pass trivially)
//! when mkvmerge is not on PATH, so the suite stays green on machines without
//! it; CI installs mkvtoolnix so these run there.

use muxsmith_core::capability::runtime::Mkvmerge;

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
}

#[test]
fn version_reports_mkvmerge() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };
    let v = m.version().expect("version query");
    assert!(v.to_lowercase().contains("mkvmerge"), "got: {v}");
}

#[test]
fn list_types_includes_matroska() {
    let Some(m) = mkvmerge() else { return };
    let types = m.list_types().expect("list-types");
    assert!(types.contains(&"mkv".to_string()));
}

#[test]
fn list_languages_normalizes_english_and_german() {
    let Some(m) = mkvmerge() else { return };
    let idx = m.list_languages().expect("list-languages");
    assert_eq!(idx.normalize("en"), idx.normalize("eng"));
    assert_eq!(idx.normalize("de"), idx.normalize("ger"));
    assert_ne!(idx.normalize("en"), idx.normalize("de"));
}
```

- [ ] **Step 6: Run the integration test**

Run: `cargo test -p muxsmith-core --test mkvmerge_runtime`
Expected: PASS (3 tests; mkvmerge v99 is installed per the environment). If mkvmerge were absent the tests self-skip.

- [ ] **Step 7: Commit**

```bash
git add crates/muxsmith-core/src/capability/
git -c commit.gpgsign=false commit -m "feat(capability): runtime mkvmerge queries (version, types, languages)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 7: `identify` module (mkvmerge -J wrapper, track model, cache)

**Scope note:** Plan 2 identifies and plans TRACKS and output naming. Attachments, chapters, tags, and title resolution plus command generation are Plan 3; this module therefore parses tracks and container status only, not attachments/chapters (added in Plan 3 where they are consumed), to avoid dead public API under `#![deny(missing_docs)]`.

**Files:**
- Create: `crates/muxsmith-core/src/identify.rs`
- Create: `crates/muxsmith-core/tests/fixtures/identify/series-s01e01.json`
- Modify: `crates/muxsmith-core/src/lib.rs` (add `pub mod identify;`)
- Modify: `crates/muxsmith-core/src/capability/mod.rs` (add the pinned-version const)
- Test: inline unit tests (JSON parsing) + `crates/muxsmith-core/tests/identify_live.rs` (gated on real mkvmerge)

**Interfaces:**
- Consumes: `capability::runtime::{Mkvmerge, RuntimeError}` (Task 6).
- Produces:
  - `capability::PINNED_IDENTIFICATION_FORMAT_VERSION: u64 = 20` (matches `generated.rs`).
  - `identify::PropValue { Bool(bool), Int(i64), Float(f64), Str(String) }` with `PropValue::from_json(&serde_json::Value) -> Option<PropValue>` (arrays/objects/null -> `None`).
  - `identify::Track { id: u64, kind: String, codec: String, properties: BTreeMap<String, PropValue> }` (`kind` is the `-J` `type`), with `fn get(&self, name: &str) -> Option<PropValue>` unifying the top-level `type`/`codec`/`id` pseudo-properties with the nested `properties` map.
  - `identify::Identification { file_name: String, format_version: u64, container_recognized: bool, container_supported: bool, tracks: Vec<Track> }` with `Identification::from_json(&str) -> Result<Identification, IdentifyError>` and `fn is_identifiable(&self) -> bool` (`container_recognized && container_supported && !tracks.is_empty()`).
  - `identify::IdentifyError { Runtime(RuntimeError), Json(String), Stat(String) }`.
  - `identify::IdentifyCache` (in-memory, per session): `IdentifyCache::new()`, `fn get_or_identify(&mut self, mkv: &Mkvmerge, path: &Path) -> Result<&Identification, IdentifyError>`, keyed on path + mtime + size (spec 5.5). A changed mtime or size re-identifies.

- [ ] **Step 1: Create the JSON fixture**

`crates/muxsmith-core/tests/fixtures/identify/series-s01e01.json` (trimmed real `mkvmerge -J` output; video `und`, English audio, one forced English SRT, one SDH English SRT):

```json
{
  "attachments": [],
  "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [],
  "file_name": "Show.S01E01.mkv",
  "global_tags": [],
  "identification_format_version": 20,
  "track_tags": [],
  "tracks": [
    {
      "codec": "AVC/H.264/MPEG-4p10",
      "id": 0,
      "type": "video",
      "properties": {
        "codec_id": "V_MPEG4/ISO/AVC",
        "default_track": true,
        "forced_track": false,
        "language": "und",
        "language_ietf": "und",
        "number": 1,
        "pixel_dimensions": "1920x1080"
      }
    },
    {
      "codec": "AAC",
      "id": 1,
      "type": "audio",
      "properties": {
        "audio_channels": 2,
        "codec_id": "A_AAC",
        "default_track": true,
        "forced_track": false,
        "language": "eng",
        "language_ietf": "en",
        "number": 2,
        "track_name": "English"
      }
    },
    {
      "codec": "SubRip/SRT",
      "id": 2,
      "type": "subtitles",
      "properties": {
        "codec_id": "S_TEXT/UTF8",
        "default_track": false,
        "forced_track": true,
        "language": "eng",
        "language_ietf": "en",
        "number": 3,
        "track_name": "English forced"
      }
    },
    {
      "codec": "SubRip/SRT",
      "id": 3,
      "type": "subtitles",
      "properties": {
        "codec_id": "S_TEXT/UTF8",
        "default_track": false,
        "flag_hearing_impaired": true,
        "forced_track": false,
        "language": "eng",
        "language_ietf": "en",
        "number": 4,
        "track_name": "English SDH"
      }
    }
  ]
}
```

- [ ] **Step 2: Write the failing unit tests**

Tests at the bottom of `crates/muxsmith-core/src/identify.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../tests/fixtures/identify/series-s01e01.json");

    #[test]
    fn parses_tracks_and_container() {
        let id = Identification::from_json(SAMPLE).unwrap();
        assert_eq!(id.file_name, "Show.S01E01.mkv");
        assert_eq!(id.format_version, 20);
        assert!(id.container_recognized && id.container_supported);
        assert!(id.is_identifiable());
        assert_eq!(id.tracks.len(), 4);
        let audio = &id.tracks[1];
        assert_eq!(audio.id, 1);
        assert_eq!(audio.kind, "audio");
        assert_eq!(audio.codec, "AAC");
    }

    #[test]
    fn get_unifies_toplevel_and_nested_properties() {
        let id = Identification::from_json(SAMPLE).unwrap();
        let sub = &id.tracks[2];
        assert_eq!(sub.get("type"), Some(PropValue::Str("subtitles".into())));
        assert_eq!(sub.get("id"), Some(PropValue::Int(2)));
        assert_eq!(sub.get("codec_id"), Some(PropValue::Str("S_TEXT/UTF8".into())));
        assert_eq!(sub.get("forced_track"), Some(PropValue::Bool(true)));
        assert_eq!(sub.get("language"), Some(PropValue::Str("eng".into())));
        assert_eq!(sub.get("no_such_prop"), None);
    }

    #[test]
    fn unrecognized_container_is_not_identifiable() {
        let json = r#"{ "container": { "recognized": false, "supported": false },
                        "file_name": "notes.txt", "identification_format_version": 20 }"#;
        let id = Identification::from_json(json).unwrap();
        assert!(!id.is_identifiable());
        assert!(id.tracks.is_empty());
    }

    #[test]
    fn prop_value_from_json_scalars_only() {
        use serde_json::json;
        assert_eq!(PropValue::from_json(&json!(true)), Some(PropValue::Bool(true)));
        assert_eq!(PropValue::from_json(&json!(7)), Some(PropValue::Int(7)));
        assert_eq!(PropValue::from_json(&json!(1.5)), Some(PropValue::Float(1.5)));
        assert_eq!(PropValue::from_json(&json!("x")), Some(PropValue::Str("x".into())));
        assert_eq!(PropValue::from_json(&json!([1, 2])), None);
        assert_eq!(PropValue::from_json(&json!(null)), None);
    }
}
```

- [ ] **Step 3: Run to verify they fail**

Run: `cargo test -p muxsmith-core --lib identify`
Expected: FAIL (module not defined).

- [ ] **Step 4: Implement `identify.rs`**

`crates/muxsmith-core/src/identify.rs`:

```rust
//! Source-file identification via `mkvmerge -J` (spec 5.5, 9). Wraps the
//! external process, parses its JSON into a track model, and caches results
//! in memory keyed on path + mtime + size so dry-run and run never re-identify
//! an unchanged file (spec 5.5). Attachments/chapters parsing arrives in
//! Plan 3 where command generation consumes them.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::capability::runtime::{Mkvmerge, RuntimeError};

/// A scalar track-property value from `-J` output. Non-scalar JSON (arrays,
/// objects, null) is not matchable and is dropped during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    /// A boolean flag (`default_track`, `forced_track`, ...).
    Bool(bool),
    /// A whole-number value (`audio_channels`, `number`, ...).
    Int(i64),
    /// A floating-point value (`min_luminance`, ...).
    Float(f64),
    /// A string value (`codec_id`, `language`, `track_name`, ...).
    Str(String),
}

impl PropValue {
    /// Converts a JSON scalar into a [`PropValue`]; returns `None` for arrays,
    /// objects, and null. An integral JSON number becomes `Int`, a
    /// non-integral one `Float` (mirrors the profile `Scalar` rule).
    pub fn from_json(v: &Value) -> Option<PropValue> {
        match v {
            Value::Bool(b) => Some(PropValue::Bool(*b)),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Some(PropValue::Int(i))
                } else {
                    n.as_f64().map(PropValue::Float)
                }
            }
            Value::String(s) => Some(PropValue::Str(s.clone())),
            _ => None,
        }
    }
}

/// One track from `-J` output. `kind` is the `-J` `type` (`video`/`audio`/
/// `subtitles`/`buttons`), a top-level field alongside `codec` and `id`;
/// everything else lives in `properties` (`codec_id`, `language`, flags, ...).
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    /// The `-J` track `id` (mkvmerge's per-file track index).
    pub id: u64,
    /// The `-J` track `type`: `video`, `audio`, `subtitles`, or `buttons`.
    pub kind: String,
    /// The `-J` `codec` human-readable name (distinct from the nested
    /// `codec_id`, e.g. `AAC` vs `A_AAC`).
    pub codec: String,
    /// The nested `properties` object, scalar values only.
    pub properties: BTreeMap<String, PropValue>,
}

impl Track {
    /// Looks up a matchable property by name, unifying the top-level
    /// `type`/`codec`/`id` fields with the nested `properties` map so the
    /// matcher sees one flat namespace (spec 4.4). Returns `None` if the
    /// track carries no such property.
    pub fn get(&self, name: &str) -> Option<PropValue> {
        match name {
            "type" => Some(PropValue::Str(self.kind.clone())),
            "codec" => Some(PropValue::Str(self.codec.clone())),
            "id" => Some(PropValue::Int(self.id as i64)),
            other => self.properties.get(other).cloned(),
        }
    }
}

/// The result of identifying one source file (spec 5.5). Carries the track
/// model plus enough container status to tell an identifiable media file from
/// a non-media file (mkvmerge exits 0 either way).
#[derive(Debug, Clone, PartialEq)]
pub struct Identification {
    /// The `file_name` mkvmerge echoed back.
    pub file_name: String,
    /// `identification_format_version`; compared against
    /// [`crate::capability::PINNED_IDENTIFICATION_FORMAT_VERSION`] to detect
    /// schema skew (spec 9.2).
    pub format_version: u64,
    /// `container.recognized`.
    pub container_recognized: bool,
    /// `container.supported`.
    pub container_supported: bool,
    /// The parsed tracks (empty for a non-media file).
    pub tracks: Vec<Track>,
}

impl Identification {
    /// Whether the file is a usable media container: recognized, supported,
    /// and carrying at least one track. A file matching an extension list but
    /// not identifiable is handled by discovery, not treated as a source.
    pub fn is_identifiable(&self) -> bool {
        self.container_recognized && self.container_supported && !self.tracks.is_empty()
    }

    /// Parses `mkvmerge -J` JSON into an [`Identification`]. Missing optional
    /// sections (a non-media file has no `tracks`) default to empty/false.
    pub fn from_json(text: &str) -> Result<Identification, IdentifyError> {
        let v: Value = serde_json::from_str(text).map_err(|e| IdentifyError::Json(e.to_string()))?;
        let container = v.get("container");
        let container_recognized = container
            .and_then(|c| c.get("recognized"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let container_supported = container
            .and_then(|c| c.get("supported"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let tracks = v
            .get("tracks")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(parse_track).collect())
            .unwrap_or_default();
        Ok(Identification {
            file_name: v
                .get("file_name")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            format_version: v
                .get("identification_format_version")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            container_recognized,
            container_supported,
            tracks,
        })
    }
}

fn parse_track(v: &Value) -> Option<Track> {
    let id = v.get("id").and_then(Value::as_u64)?;
    let kind = v.get("type").and_then(Value::as_str)?.to_string();
    let codec = v
        .get("codec")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let mut properties = BTreeMap::new();
    if let Some(props) = v.get("properties").and_then(Value::as_object) {
        for (k, val) in props {
            if let Some(pv) = PropValue::from_json(val) {
                properties.insert(k.clone(), pv);
            }
        }
    }
    Some(Track {
        id,
        kind,
        codec,
        properties,
    })
}

/// Failure identifying a file. Data only; call sites render it to diagnostics.
#[derive(Debug, Clone)]
pub enum IdentifyError {
    /// The mkvmerge process failed (spawn, non-zero, not found).
    Runtime(RuntimeError),
    /// The `-J` output was not valid JSON in the expected shape.
    Json(String),
    /// The file could not be stat'd for the cache key.
    Stat(String),
}

impl From<RuntimeError> for IdentifyError {
    fn from(e: RuntimeError) -> Self {
        IdentifyError::Runtime(e)
    }
}

/// In-memory identification cache for one session (spec 5.5). Keyed on path
/// plus (mtime, size); a changed file re-identifies, so a dry run is never
/// stale. On-disk caching is a future candidate (spec non-goals).
#[derive(Debug, Default)]
pub struct IdentifyCache {
    entries: HashMap<PathBuf, (CacheKey, Identification)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CacheKey {
    mtime_ns: i128,
    size: u64,
}

impl IdentifyCache {
    /// A fresh, empty cache.
    pub fn new() -> IdentifyCache {
        IdentifyCache::default()
    }

    /// Returns the identification for `path`, running `mkvmerge -J` only if the
    /// file is absent from the cache or its (mtime, size) changed. Borrows the
    /// cached value for the caller's lifetime.
    pub fn get_or_identify(
        &mut self,
        mkv: &Mkvmerge,
        path: &Path,
    ) -> Result<&Identification, IdentifyError> {
        let key = cache_key(path)?;
        let fresh = matches!(self.entries.get(path), Some((k, _)) if *k == key);
        if !fresh {
            let out = mkv.identify_json(path)?;
            let id = Identification::from_json(&out)?;
            self.entries.insert(path.to_path_buf(), (key, id));
        }
        Ok(&self.entries.get(path).expect("just inserted").1)
    }
}

fn cache_key(path: &Path) -> Result<CacheKey, IdentifyError> {
    let meta = std::fs::metadata(path).map_err(|e| IdentifyError::Stat(e.to_string()))?;
    let mtime_ns = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_nanos() as i128)
        .unwrap_or(0);
    Ok(CacheKey {
        mtime_ns,
        size: meta.len(),
    })
}
```

Add the `-J` runner to `Mkvmerge` in `crates/muxsmith-core/src/capability/runtime.rs`:

```rust
impl Mkvmerge {
    /// Runs `mkvmerge -J <file>` and returns the raw JSON stdout. mkvmerge
    /// exits 0 for a non-media file too (with `container.recognized: false`),
    /// so a non-zero exit here is a genuine invocation failure.
    pub fn identify_json(&self, file: &std::path::Path) -> Result<String, RuntimeError> {
        let file = file.to_str().ok_or_else(|| {
            RuntimeError::Parse("non-UTF-8 path cannot be passed to mkvmerge".into())
        })?;
        self.run(&["-J", file])
    }
}
```

Add to `crates/muxsmith-core/src/capability/mod.rs`:

```rust
/// The mkvmerge identification-output schema version this build was generated
/// against (spec 9). A file whose `-J` `identification_format_version` exceeds
/// this triggers the `UnknownPropertySkew` warning (untyped forward matching).
pub const PINNED_IDENTIFICATION_FORMAT_VERSION: u64 = 20;
```

Add `pub mod identify;` to `crates/muxsmith-core/src/lib.rs`.

- [ ] **Step 5: Run the unit tests**

Run: `cargo test -p muxsmith-core --lib identify`
Expected: PASS (4 tests).

- [ ] **Step 6: Write and run the live (mkvmerge-gated) cache test**

`crates/muxsmith-core/tests/identify_live.rs`:

```rust
//! Live identification against real mkvmerge, muxing a fixture MKV from a
//! committed wav + srt seed (no ffmpeg dependency). Self-skips when mkvmerge
//! is unavailable.

use std::path::Path;
use std::process::Command;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::IdentifyCache;

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
}

// Mux the committed seeds into a temp .mkv via mkvmerge itself (spec 10).
fn make_sample(dir: &Path) -> std::path::PathBuf {
    let out = dir.join("sample.mkv");
    let wav = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/seeds/tone.wav");
    let srt = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/seeds/sub.srt");
    let status = Command::new("mkvmerge")
        .args(["-q", "-o"])
        .arg(&out)
        .args(["--language", "0:eng", "--track-name", "0:English"])
        .arg(wav)
        .args(["--language", "0:ger"])
        .arg(srt)
        .status()
        .expect("spawn mkvmerge to build fixture");
    assert!(status.success(), "mkvmerge failed to build the sample");
    out
}

#[test]
fn identifies_and_caches_a_real_file() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };
    let dir = tempfile::tempdir().unwrap();
    let sample = make_sample(dir.path());

    let mut cache = IdentifyCache::new();
    let id = cache.get_or_identify(&m, &sample).unwrap().clone();
    assert!(id.is_identifiable());
    assert!(id.tracks.iter().any(|t| t.kind == "audio"));
    assert!(id.tracks.iter().any(|t| t.kind == "subtitles"));

    // Second call is served from cache (same mtime/size); result is identical.
    let again = cache.get_or_identify(&m, &sample).unwrap();
    assert_eq!(&id, again);
}
```

Create the seeds. A tiny WAV can be produced deterministically without ffmpeg; commit both:
- `crates/muxsmith-core/tests/fixtures/seeds/sub.srt`:

```
1
00:00:00,000 --> 00:00:00,500
seed
```

- `crates/muxsmith-core/tests/fixtures/seeds/tone.wav`: generate a ~0.1s silent mono 8kHz PCM WAV once and commit it. Produce it with this one-off (run at implementation time, then commit the file):

```bash
python3 - <<'PY'
import wave, struct
w = wave.open("crates/muxsmith-core/tests/fixtures/seeds/tone.wav", "w")
w.setnchannels(1); w.setsampwidth(2); w.setframerate(8000)
w.writeframes(b"".join(struct.pack("<h", 0) for _ in range(800)))
w.close()
PY
```

Add `tempfile` to the core crate dev-dependencies:

```bash
cargo add -p muxsmith-core --dev tempfile
```

Run: `cargo test -p muxsmith-core --test identify_live`
Expected: PASS (1 test; mkvmerge muxes the seed and identifies it).

- [ ] **Step 7: Commit (the wav seed is committed; it is a synthetic silent tone, not third-party media)**

```bash
git add crates/muxsmith-core/src/identify.rs crates/muxsmith-core/src/lib.rs \
  crates/muxsmith-core/src/capability/ crates/muxsmith-core/tests/fixtures/identify/ \
  crates/muxsmith-core/tests/fixtures/seeds/ crates/muxsmith-core/tests/identify_live.rs \
  crates/muxsmith-core/Cargo.toml Cargo.lock
git -c commit.gpgsign=false commit -m "feat(core): identify module (mkvmerge -J wrapper, track model, cache)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 8: `matcher` (pure match-expression evaluation)

**Files:**
- Create: `crates/muxsmith-core/src/matcher.rs`
- Modify: `crates/muxsmith-core/src/lib.rs` (add `pub mod matcher;`)
- Test: inline unit tests

**Interfaces:**
- Consumes: `profile::match_expr::{MatchExpr, Scalar}`, `identify::{Track, PropValue}`, `capability::{codec_kind_prefixes, runtime::LanguageIndex}`.
- Produces: `matcher::matches(expr: &MatchExpr, track: &Track, lang: &LanguageIndex) -> bool`. Pure and total: no I/O, no diagnostics (plan-time diagnostics are the planner's job). Evaluates the spec 4.3 algebra with the spec 4.4 special cases (language normalization against both `language` and `language_ietf`; `codec_kind` as a `codec_id` prefix match).

- [ ] **Step 1: Write the failing tests**

Tests at the bottom of `crates/muxsmith-core/src/matcher.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::runtime::LanguageIndex;
    use crate::identify::{PropValue, Track};
    use std::collections::BTreeMap;

    fn lang() -> LanguageIndex {
        LanguageIndex::from_rows(&[
            ["English", "eng", "eng", "en"],
            ["German", "ger", "ger", "de"],
        ])
    }

    fn track(kind: &str, props: &[(&str, PropValue)]) -> Track {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert((*k).to_string(), v.clone());
        }
        Track {
            id: 0,
            kind: kind.to_string(),
            codec: String::new(),
            properties,
        }
    }

    fn expr(yaml: &str) -> MatchExpr {
        yaml_serde::from_str(yaml).unwrap()
    }

    #[test]
    fn exact_matches_type_and_flags() {
        let t = track("subtitles", &[("forced_track", PropValue::Bool(true))]);
        assert!(matches(
            &expr("exact: { type: subtitles, forced_track: true }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("exact: { type: subtitles, forced_track: false }"),
            &t,
            &lang()
        ));
        // Absent property never matches.
        assert!(!matches(&expr("exact: { default_track: true }"), &t, &lang()));
    }

    #[test]
    fn language_normalizes_iso_and_bcp47_against_both_fields() {
        let t = track(
            "audio",
            &[
                ("language", PropValue::Str("ger".into())),
                ("language_ietf", PropValue::Str("de".into())),
            ],
        );
        // Profile value `de` (BCP-47) matches the `ger` language field.
        assert!(matches(&expr("exact: { language: de }"), &t, &lang()));
        // Profile value `ger` (ISO 639-2) also matches.
        assert!(matches(&expr("exact: { language: ger }"), &t, &lang()));
        // A different language does not.
        assert!(!matches(&expr("exact: { language: en }"), &t, &lang()));
    }

    #[test]
    fn language_falls_back_to_raw_compare_when_unknown() {
        let t = track("audio", &[("language", PropValue::Str("zxx".into()))]);
        // zxx (no linguistic content) is not in our tiny index; equal raw
        // strings still match, unequal do not.
        assert!(matches(&expr("exact: { language: zxx }"), &t, &lang()));
        assert!(!matches(&expr("exact: { language: qqq }"), &t, &lang()));
    }

    #[test]
    fn codec_kind_is_codec_id_prefix_match() {
        let srt = track("subtitles", &[("codec_id", PropValue::Str("S_TEXT/UTF8".into()))]);
        let ass = track("subtitles", &[("codec_id", PropValue::Str("S_TEXT/ASS".into()))]);
        assert!(matches(&expr("exact: { codec_kind: srt }"), &srt, &lang()));
        assert!(!matches(&expr("exact: { codec_kind: srt }"), &ass, &lang()));
        assert!(matches(&expr("exact: { codec_kind: ass }"), &ass, &lang()));
    }

    #[test]
    fn substring_is_case_insensitive_and_regex_is_literal() {
        let t = track("subtitles", &[("track_name", PropValue::Str("English SDH".into()))]);
        assert!(matches(&expr("substring: { track_name: sdh }"), &t, &lang()));
        assert!(matches(&expr("regex: { track_name: '(?i)^english' }"), &t, &lang()));
        assert!(!matches(&expr("regex: { track_name: '^SDH' }"), &t, &lang()));
    }

    #[test]
    fn any_and_not_recurse() {
        let t = track("subtitles", &[("track_name", PropValue::Str("English SDH".into()))]);
        assert!(matches(
            &expr("any:\n  - substring: { track_name: SDH }\n  - substring: { track_name: forced }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("not:\n  - substring: { track_name: SDH }"),
            &t,
            &lang()
        ));
    }

    #[test]
    fn empty_expression_matches_everything() {
        let t = track("video", &[]);
        assert!(matches(&expr("{}"), &t, &lang()));
    }

    #[test]
    fn numeric_exact_compares_across_int_and_float() {
        let t = track("audio", &[("audio_channels", PropValue::Int(6))]);
        assert!(matches(&expr("exact: { audio_channels: 6 }"), &t, &lang()));
        assert!(!matches(&expr("exact: { audio_channels: 2 }"), &t, &lang()));
    }
}
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test -p muxsmith-core --lib matcher`
Expected: FAIL (module not defined).

- [ ] **Step 3: Implement `matcher.rs`**

`crates/muxsmith-core/src/matcher.rs`:

```rust
//! Match-expression evaluation (spec 4.3, 4.4). Pure and total: given an
//! expression, a track, and a language index, decides membership with no I/O
//! and no diagnostics. The correctness core; covered by unit tests here and by
//! the planner's fixture tests. Config validity (unknown property, wrong type,
//! bad enum value) is checked earlier (validate) or reported by the planner;
//! this function assumes a validated expression and answers only "does this
//! track match?".

use crate::capability::codec_kind_prefixes;
use crate::capability::runtime::LanguageIndex;
use crate::identify::{PropValue, Track};
use crate::profile::match_expr::{MatchExpr, Scalar};

/// Whether `track` satisfies `expr` (spec 4.3): the conjunction of all present
/// parts. `lang` normalizes language tokens so ISO 639-2 and BCP-47 values
/// compare equal (spec 4.4).
pub fn matches(expr: &MatchExpr, track: &Track, lang: &LanguageIndex) -> bool {
    if let Some(exact) = &expr.exact {
        for (prop, want) in exact {
            if !exact_matches(prop, want, track, lang) {
                return false;
            }
        }
    }
    if let Some(sub) = &expr.substring {
        for (prop, needle) in sub {
            match track_str(prop, track) {
                Some(hay) if hay.to_lowercase().contains(&needle.to_lowercase()) => {}
                _ => return false,
            }
        }
    }
    if let Some(rx) = &expr.regex {
        for (prop, pattern) in rx {
            let hay = match track_str(prop, track) {
                Some(h) => h,
                None => return false,
            };
            // A validated expression compiles; an invalid regex was already an
            // InvalidRegex config error, so a failure here means no match.
            match regex::Regex::new(pattern) {
                Ok(re) if re.is_match(&hay) => {}
                _ => return false,
            }
        }
    }
    if let Some(any) = &expr.any {
        if !any.is_empty() && !any.iter().any(|e| matches(e, track, lang)) {
            return false;
        }
    }
    if let Some(not) = &expr.not {
        if not.iter().any(|e| matches(e, track, lang)) {
            return false;
        }
    }
    true
}

fn exact_matches(prop: &str, want: &Scalar, track: &Track, lang: &LanguageIndex) -> bool {
    match prop {
        // language matches against both `language` and `language_ietf`,
        // normalized so `de` and `ger` are equal (spec 4.4).
        "language" => {
            let Scalar::Str(want) = want else { return false };
            ["language", "language_ietf"]
                .iter()
                .filter_map(|f| track_str(f, track))
                .any(|have| lang_eq(want, &have, lang))
        }
        // codec_kind is a codec_id prefix match over a curated alias set.
        "codec_kind" => {
            let Scalar::Str(kind) = want else { return false };
            let Some(prefixes) = codec_kind_prefixes(kind) else {
                return false;
            };
            match track_str("codec_id", track) {
                Some(id) => prefixes.iter().any(|p| id.starts_with(p)),
                None => false,
            }
        }
        _ => match track.get(prop) {
            Some(have) => scalar_eq(want, &have),
            None => false,
        },
    }
}

/// True when two language tokens denote the same language. Both are normalized
/// through the index; if either is unrecognized, fall back to a raw
/// case-insensitive compare so unusual-but-equal tags still match.
fn lang_eq(a: &str, b: &str, lang: &LanguageIndex) -> bool {
    match (lang.normalize(a), lang.normalize(b)) {
        (Some(na), Some(nb)) => na == nb,
        _ => a.eq_ignore_ascii_case(b),
    }
}

/// The string form of a track property, for substring/regex/language. Only
/// `PropValue::Str` yields a value; numeric/boolean properties are not strings.
fn track_str(prop: &str, track: &Track) -> Option<String> {
    match track.get(prop) {
        Some(PropValue::Str(s)) => Some(s),
        _ => None,
    }
}

/// Value equality between a profile `Scalar` and a track `PropValue`, with
/// int/float cross-comparison (spec 4.3, `exact`). Strings compare
/// case-sensitively (language is special-cased before reaching here).
fn scalar_eq(want: &Scalar, have: &PropValue) -> bool {
    match (want, have) {
        (Scalar::Str(a), PropValue::Str(b)) => a == b,
        (Scalar::Bool(a), PropValue::Bool(b)) => a == b,
        (Scalar::Int(a), PropValue::Int(b)) => a == b,
        (Scalar::Int(a), PropValue::Float(b)) => (*a as f64) == *b,
        (Scalar::Float(a), PropValue::Float(b)) => a == b,
        (Scalar::Float(a), PropValue::Int(b)) => *a == (*b as f64),
        _ => false,
    }
}
```

Add `pub mod matcher;` to `crates/muxsmith-core/src/lib.rs`.

- [ ] **Step 4: Run the matcher tests**

Run: `cargo test -p muxsmith-core --lib matcher`
Expected: PASS (8 tests).

- [ ] **Step 5: Commit**

```bash
git add crates/muxsmith-core/src/matcher.rs crates/muxsmith-core/src/lib.rs
git -c commit.gpgsign=false commit -m "feat(core): pure match-expression evaluator with language and codec_kind semantics

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 9: `discovery` (primary-file scan and external locator resolution)

**Files:**
- Create: `crates/muxsmith-core/src/discovery.rs`
- Modify: `crates/muxsmith-core/src/lib.rs` (add `pub mod discovery;`)
- Test: inline unit tests using `tempfile`

**Interfaces:**
- Consumes: `profile::model::{Input, Locator}`, `template::{Template, Ctx}`, `report::{Diagnostic, DiagCode}`.
- Produces:
  - `discovery::Identifier { whole: String, groups: BTreeMap<String, String> }` with `fn to_ctx(&self) -> template::Ctx` (binds `{match}` to `whole` plus every named/numbered group).
  - `discovery::PrimaryFile { path: PathBuf, identifier: Identifier }`.
  - `discovery::scan_primaries(source: &Path, input: &Input) -> (Vec<PrimaryFile>, Vec<Diagnostic>)`: walks `source` (recursive per `input.recursive`), keeps files whose extension is in `input.extensions` (case-insensitive) and whose basename matches `input.pattern`; emits `IgnoredFile` (extension matches, pattern does not), `MultipleIdentifierMatches` (pattern matches more than once), `DuplicateIdentifier` (two primaries share an identifier). Deterministic order (sorted paths).
  - `discovery::resolve_locator(locator: &Locator, primary_dir: &Path, identifier: &Identifier) -> Vec<PathBuf>`: candidate donor files, sorted. Unreadable directories yield an empty list (the planner turns zero candidates into `MissingExternal`). No diagnostics: `MissingExternal`/`AmbiguousExternal`/`DonorIsPrimary` are the planner's call (they need rule `optional` and the primaries set).

- [ ] **Step 1: Write the failing tests**

Tests at the bottom of `crates/muxsmith-core/src/discovery.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::model::{Input, Locator};
    use std::fs;

    fn input(pattern: &str, exts: &[&str], recursive: bool) -> Input {
        Input {
            pattern: pattern.to_string(),
            extensions: exts.iter().map(|s| s.to_string()).collect(),
            recursive,
        }
    }

    #[test]
    fn scans_primaries_and_extracts_named_groups() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Show.S01E02.mkv"), b"x").unwrap();
        fs::write(dir.path().join("Show.S01E03.MP4"), b"x").unwrap(); // case-insensitive ext
        fs::write(dir.path().join("readme.txt"), b"x").unwrap(); // wrong ext, ignored silently
        fs::write(dir.path().join("notes.mkv"), b"x").unwrap(); // right ext, no pattern -> IgnoredFile

        let (primaries, diags) = scan_primaries(
            dir.path(),
            &input(r"S(?<season>\d{2})E(?<episode>\d{2})", &["mkv", "mp4"], true),
        );
        assert_eq!(primaries.len(), 2);
        let e02 = primaries
            .iter()
            .find(|p| p.identifier.whole == "S01E02")
            .expect("S01E02 primary");
        assert_eq!(e02.identifier.groups["season"], "01");
        assert_eq!(e02.identifier.groups["episode"], "02");
        assert_eq!(e02.identifier.groups["g1"], "01"); // numbered group alias
        assert!(diags.iter().any(|d| d.code == DiagCode::IgnoredFile));
    }

    #[test]
    fn duplicate_identifier_is_warned() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Show.S01E01.720p.mkv"), b"x").unwrap();
        fs::write(dir.path().join("Show.S01E01.1080p.mkv"), b"x").unwrap();
        let (primaries, diags) = scan_primaries(
            dir.path(),
            &input(r"S(?<season>\d{2})E(?<episode>\d{2})", &["mkv"], true),
        );
        assert_eq!(primaries.len(), 2);
        assert!(diags.iter().any(|d| d.code == DiagCode::DuplicateIdentifier));
    }

    #[test]
    fn multiple_identifier_matches_uses_first() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("E01.E02.mkv"), b"x").unwrap();
        let (primaries, diags) = scan_primaries(dir.path(), &input(r"E(\d{2})", &["mkv"], true));
        assert_eq!(primaries.len(), 1);
        assert_eq!(primaries[0].identifier.whole, "E01");
        assert!(diags.iter().any(|d| d.code == DiagCode::MultipleIdentifierMatches));
    }

    #[test]
    fn non_recursive_scan_skips_subdirs() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("sub").join("E01.mkv"), b"x").unwrap();
        fs::write(dir.path().join("E02.mkv"), b"x").unwrap();
        let (primaries, _) = scan_primaries(dir.path(), &input(r"E(\d{2})", &["mkv"], false));
        assert_eq!(primaries.len(), 1);
        assert_eq!(primaries[0].identifier.whole, "E02");
    }

    #[test]
    fn resolve_locator_matches_by_identifier() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Show.S01E01.srt"), b"x").unwrap();
        fs::write(dir.path().join("Show.S01E02.srt"), b"x").unwrap();
        let ident = Identifier {
            whole: "S01E01".to_string(),
            groups: Default::default(),
        };
        let locator = Locator {
            path: ".".into(),
            recursive: false,
            extensions: vec!["srt".into()],
            match_to_source: Some(true),
            match_pattern: None,
            case_sensitive: false,
        };
        let hits = resolve_locator(&locator, dir.path(), &ident);
        assert_eq!(hits.len(), 1);
        assert!(hits[0].ends_with("Show.S01E01.srt"));
    }
}
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test -p muxsmith-core --lib discovery`
Expected: FAIL (module not defined).

- [ ] **Step 3: Implement `discovery.rs`**

`crates/muxsmith-core/src/discovery.rs`:

```rust
//! Source-tree discovery (spec 3, 4.2, 4.6): find primary files by extension
//! and `input.pattern`, extract their identifiers, and resolve external donor
//! candidates for a locator. Filesystem-facing but diagnostic-light: only the
//! file-independent facts (ignored files, duplicate identifiers, repeated
//! pattern matches) are emitted here; per-rule external diagnostics belong to
//! the planner, which knows `optional` and the primaries set.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::profile::model::{Input, Locator};
use crate::report::{DiagCode, Diagnostic};
use crate::template::{Ctx, Template};

/// A primary file's identifier (spec 3): the substring matched by
/// `input.pattern` plus its capture groups, feeding template fields and
/// external-file matching.
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    /// The whole matched substring (`{match}`), capture group 0.
    pub whole: String,
    /// Named and numbered capture groups (`season`, `g1`, ...); numbered
    /// groups are keyed `g1`, `g2`, ... and named groups by their name.
    pub groups: BTreeMap<String, String>,
}

impl Identifier {
    /// A template render context binding `{match}` to `whole` and every
    /// capture group to its value (spec 4.7).
    pub fn to_ctx(&self) -> Ctx {
        let mut ctx = Ctx::new();
        ctx.set("match", self.whole.clone());
        for (k, v) in &self.groups {
            ctx.set(k.clone(), v.clone());
        }
        ctx
    }
}

/// A discovered primary file and its identifier (spec 3).
#[derive(Debug, Clone, PartialEq)]
pub struct PrimaryFile {
    /// Absolute or source-relative path to the file.
    pub path: PathBuf,
    /// The identifier extracted from its basename.
    pub identifier: Identifier,
}

/// Walks `source` and returns the primary files plus file-independent
/// diagnostics (spec 4.2). A file whose extension is in `input.extensions`
/// (case-insensitive) but whose basename does not match `input.pattern` is an
/// `IgnoredFile` (info); more than one pattern match in a basename is
/// `MultipleIdentifierMatches` (info, first match used); two primaries sharing
/// an identifier is `DuplicateIdentifier` (warning).
pub fn scan_primaries(source: &Path, input: &Input) -> (Vec<PrimaryFile>, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let re = match regex::Regex::new(&input.pattern) {
        Ok(re) => re,
        Err(_) => return (Vec::new(), diags), // validate already reported InvalidRegex
    };
    let exts: Vec<String> = input.extensions.iter().map(|e| e.to_ascii_lowercase()).collect();

    let mut primaries = Vec::new();
    for path in walk_files(source, input.recursive) {
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if !extension_matches(&path, &exts) {
            continue; // not a candidate at all; silent
        }
        let mut matches = re.find_iter(name);
        let Some(first) = matches.next() else {
            diags.push(Diagnostic::info(DiagCode::IgnoredFile, "input.pattern").for_file(&path));
            continue;
        };
        if matches.next().is_some() {
            diags.push(
                Diagnostic::info(DiagCode::MultipleIdentifierMatches, "input.pattern")
                    .for_file(&path)
                    .with("name", name),
            );
        }
        let caps = re.captures(name).expect("first match implies captures");
        let mut groups = BTreeMap::new();
        for (i, opt_name) in re.capture_names().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some(m) = caps.get(i) {
                groups.insert(format!("g{i}"), m.as_str().to_string());
                if let Some(n) = opt_name {
                    groups.insert(n.to_string(), m.as_str().to_string());
                }
            }
        }
        primaries.push(PrimaryFile {
            path: path.clone(),
            identifier: Identifier {
                whole: first.as_str().to_string(),
                groups,
            },
        });
    }

    // DuplicateIdentifier across the batch (spec 5.2).
    let mut by_id: BTreeMap<&str, Vec<&PrimaryFile>> = BTreeMap::new();
    for p in &primaries {
        by_id.entry(p.identifier.whole.as_str()).or_default().push(p);
    }
    for (id, group) in &by_id {
        if group.len() >= 2 {
            diags.push(
                Diagnostic::warning(DiagCode::DuplicateIdentifier, "input.pattern")
                    .with("identifier", *id)
                    .with("file_a", group[0].path.display().to_string())
                    .with("file_b", group[1].path.display().to_string()),
            );
        }
    }

    (primaries, diags)
}

/// Candidate donor files for a locator (spec 4.6): files under the locator's
/// directory (relative to `primary_dir`, or absolute) whose extension is in
/// `locator.extensions` and whose basename matches the rendered
/// `match_to_source`/`match_pattern`. Sorted; unreadable directories yield an
/// empty list.
pub fn resolve_locator(locator: &Locator, primary_dir: &Path, identifier: &Identifier) -> Vec<PathBuf> {
    let base = if locator.path.is_absolute() {
        locator.path.clone()
    } else {
        primary_dir.join(&locator.path)
    };
    let exts: Vec<String> = locator.extensions.iter().map(|e| e.to_ascii_lowercase()).collect();

    // The basename-matching regex: match_to_source is sugar for the template
    // "{match}" (spec 4.6). A validated template parses; on the off chance it
    // does not, match nothing rather than panic.
    let pattern_src = if matches!(locator.match_to_source, Some(true)) {
        "{match}".to_string()
    } else {
        locator.match_pattern.clone().unwrap_or_default()
    };
    let ctx = identifier.to_ctx();
    let re = Template::parse(&pattern_src)
        .ok()
        .map(|t| t.render_regex_pattern(&ctx, locator.case_sensitive))
        .and_then(|p| regex::Regex::new(&p).ok());

    let mut hits = Vec::new();
    for path in walk_files(&base, locator.recursive) {
        if !extension_matches(&path, &exts) {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let matched = match &re {
            Some(re) => re.is_match(name),
            None => false,
        };
        if matched {
            hits.push(path);
        }
    }
    hits
}

fn extension_matches(path: &Path, exts_lower: &[String]) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(e) => exts_lower.iter().any(|x| x == &e.to_ascii_lowercase()),
        None => false,
    }
}

/// Regular files under `dir`, recursing into subdirectories only if
/// `recursive`. Symlinks are not followed (avoids cycles). Sorted for
/// deterministic output; unreadable directories are skipped silently.
fn walk_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        let mut dir_entries: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        dir_entries.sort();
        for path in dir_entries {
            let Ok(meta) = std::fs::symlink_metadata(&path) else {
                continue;
            };
            if meta.is_dir() {
                if recursive {
                    stack.push(path);
                }
            } else if meta.is_file() {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}
```

Add `pub mod discovery;` to `crates/muxsmith-core/src/lib.rs`.

- [ ] **Step 4: Run the discovery tests**

Run: `cargo test -p muxsmith-core --lib discovery`
Expected: PASS (5 tests).

- [ ] **Step 5: Commit**

```bash
git add crates/muxsmith-core/src/discovery.rs crates/muxsmith-core/src/lib.rs
git -c commit.gpgsign=false commit -m "feat(core): source-tree discovery and external locator resolution

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 10: `planner` (per-file resolution, output paths, batch report)

**Files:**
- Create: `crates/muxsmith-core/src/planner.rs`
- Modify: `crates/muxsmith-core/src/identify.rs` (add the `Identify` trait + `LiveIdentifier`)
- Modify: `crates/muxsmith-core/src/lib.rs` (add `pub mod planner;`)
- Test: `crates/muxsmith-core/tests/planner_resolution.rs` (fixture-driven, no mkvmerge)

**Interfaces:**
- Consumes: `discovery`, `matcher`, `template`, `capability` (incl. `matchable_domain`, `PINNED_IDENTIFICATION_FORMAT_VERSION`, `runtime::LanguageIndex`), `identify::{Identification, Track, IdentifyError}`, `profile::model::*`, `report::{Diagnostic, DiagCode, Severity}`.
- Produces:
  - `identify::Identify` trait: `fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError>` (returns owned). `identify::LiveIdentifier<'a> { cache: IdentifyCache, mkv: &'a Mkvmerge }` implements it against real mkvmerge; tests provide a fake.
  - `planner::RunInputs { source: PathBuf, output: Option<PathBuf>, on_collision: Option<CollisionPolicy> }`.
  - `planner::Assignment { rule_index: usize, source: PathBuf, track_id: Option<u64> }` (`track_id: None` for a satisfied `optional` rule that matched nothing).
  - `planner::Plan { source: PathBuf, output: PathBuf, assignments: Vec<Assignment> }`.
  - `planner::FileReport { source: PathBuf, identifier: String, plan: Option<Plan>, diagnostics: Vec<Diagnostic> }` (`plan: None` iff the file has an error-severity diagnostic, spec 5.1).
  - `planner::Batch { files: Vec<FileReport>, batch_diagnostics: Vec<Diagnostic>, suggestions: Vec<crate::planner::Suggestion> }` (`suggestions` filled by Task 11; declare the field now as `Vec<Suggestion>` with `Suggestion` a placeholder-free struct defined in Task 11, so declare it here as an empty-for-now `Vec` of a type introduced in Task 11 -> to avoid a forward reference, define `Suggestion` in THIS task as an empty-capable struct and flesh out its construction in Task 11). All result types derive `Debug, Clone, PartialEq, Serialize`.
  - `planner::plan_batch(profile: &Profile, run: &RunInputs, id: &mut dyn Identify, lang: &LanguageIndex) -> Batch`.

  To avoid a forward reference, define the suggestion types in this task (empty engine), and Task 11 fills `plan_batch` in with the real generation:
  - `planner::StructuredEdit` and `planner::Suggestion` (see Task 11 for the full shape); in this task, define them and leave `Batch.suggestions` an empty `Vec` (the engine call is added in Task 11).

- [ ] **Step 1: Add the `Identify` trait to `identify.rs`**

Append to `crates/muxsmith-core/src/identify.rs`:

```rust
/// Abstracts identification so the planner can be unit-tested against fixture
/// data without spawning mkvmerge. Returns an owned [`Identification`] (a
/// clone from the cache in the live impl) to keep the trait object simple.
pub trait Identify {
    /// Identifies `path`, or returns why it could not be identified.
    fn identify(&mut self, path: &std::path::Path) -> Result<Identification, IdentifyError>;
}

/// The production [`Identify`]: an [`IdentifyCache`] plus the resolved
/// mkvmerge, wired together so `plan_batch` can drive real identification.
pub struct LiveIdentifier<'a> {
    /// The per-session identification cache (spec 5.5).
    pub cache: IdentifyCache,
    /// The resolved external mkvmerge.
    pub mkv: &'a Mkvmerge,
}

impl Identify for LiveIdentifier<'_> {
    fn identify(&mut self, path: &std::path::Path) -> Result<Identification, IdentifyError> {
        self.cache.get_or_identify(self.mkv, path).cloned()
    }
}
```

- [ ] **Step 2: Write the failing tests**

`crates/muxsmith-core/tests/planner_resolution.rs`:

```rust
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{plan_batch, RunInputs};
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::report::{DiagCode, Severity};

// A fake identifier backed by fixture JSON keyed on file name.
struct FakeIdent {
    by_name: HashMap<String, Identification>,
}
impl Identify for FakeIdent {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        let name = path.file_name().unwrap().to_str().unwrap();
        self.by_name
            .get(name)
            .cloned()
            .ok_or_else(|| IdentifyError::Json(format!("no fixture for {name}")))
    }
}

fn lang() -> LanguageIndex {
    LanguageIndex::from_rows(&[["English", "eng", "eng", "en"], ["German", "ger", "ger", "de"]])
}

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");

// Build a batch with a single primary file present on disk in a tempdir, whose
// identification is supplied by the fake. Returns (Batch, tempdir).
fn plan_one(profile_yaml: &str, file_name: &str, ident_json: &str) -> muxsmith_core::planner::Batch {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(file_name), b"x").unwrap();
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(file_name.to_string(), Identification::from_json(ident_json).unwrap());
    let mut ident = FakeIdent { by_name };
    // Leak the tempdir for the test's lifetime by keeping it alive:
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(dir); // keep files around; tempdir cleanup is not needed in a test process
    batch
}

const P_VIDEO_AUDIO: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { type: audio, language: en } }
"#;

#[test]
fn resolves_each_rule_to_one_track() {
    let batch = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments.len(), 2);
    assert_eq!(plan.assignments[0].track_id, Some(0)); // video
    assert_eq!(plan.assignments[1].track_id, Some(1)); // english audio
}

#[test]
fn ambiguous_rule_when_two_tracks_match() {
    // Both subtitle tracks are English SRT; a rule matching just that is ambiguous.
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles, codec_kind: srt, language: en } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(fr.diagnostics.iter().any(|d| d.code == DiagCode::AmbiguousRule));
}

#[test]
fn missing_track_when_no_match_and_not_optional() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: de } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(fr.diagnostics.iter().any(|d| d.code == DiagCode::MissingTrack));
}

#[test]
fn optional_absent_track_is_no_error() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { type: audio, language: de } }
    optional: true
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments[1].track_id, None); // optional, unmatched
}

#[test]
fn overlapping_rules_when_two_rules_claim_one_track() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { codec_id: 'V_MPEG4/ISO/AVC' } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(fr.diagnostics.iter().any(|d| d.code == DiagCode::OverlappingRules));
}

#[test]
fn keep_filename_renders_mkv_output() {
    let batch = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
    let plan = batch.files[0].plan.as_ref().unwrap();
    assert!(plan.output.file_name().unwrap().to_str().unwrap().ends_with(".mkv"));
    assert_eq!(plan.output.file_name().unwrap(), "Show.S01E01.mkv");
}

#[test]
fn bad_language_value_is_batch_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: zz } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    assert!(
        batch.batch_diagnostics.iter().any(|d| d.code == DiagCode::InvalidPropertyValue),
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
}
```

- [ ] **Step 3: Run to verify they fail**

Run: `cargo test -p muxsmith-core --test planner_resolution`
Expected: FAIL (module not defined).

- [ ] **Step 4: Implement `planner.rs`**

`crates/muxsmith-core/src/planner.rs`:

```rust
//! Batch planning (spec 5): resolve every track rule against each primary
//! file's tracks (and located donors) under strict independent uniqueness,
//! render output paths, and collect diagnostics into a batch report. No
//! filesystem mutation and no mux invocations (dry-run, spec 5.5); the only
//! external work is identification, driven through the injected [`Identify`].

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;

use crate::capability::{self, PINNED_IDENTIFICATION_FORMAT_VERSION};
use crate::capability::runtime::LanguageIndex;
use crate::discovery::{self, Identifier, PrimaryFile};
use crate::identify::{Identification, Identify};
use crate::matcher;
use crate::profile::match_expr::{MatchExpr, Scalar};
use crate::profile::model::{CollisionPolicy, FilenameCfg, Profile, SourceCfg};
use crate::report::{DiagCode, Diagnostic, Severity};
use crate::template::Template;

/// Run inputs, separable from the profile (spec 3): overrides for source and
/// output directories and the collision policy. `None` falls back to the
/// profile's stored value (and, for output, ultimately to the source dir).
#[derive(Debug, Clone, PartialEq)]
pub struct RunInputs {
    /// Directory scanned for primary files.
    pub source: PathBuf,
    /// Output directory; falls back to `profile.output.directory`, then to
    /// `source` (output beside the source, which surfaces `SourceOverwrite`
    /// for a keep-name `.mkv` source).
    pub output: Option<PathBuf>,
    /// Collision policy override; falls back to `profile.output.on_collision`.
    pub on_collision: Option<CollisionPolicy>,
}

/// One resolved rule-to-track assignment. `track_id` is `None` for a satisfied
/// `optional` rule that matched nothing (spec 5.1).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Assignment {
    /// Index into `profile.tracks`.
    pub rule_index: usize,
    /// The source file the track comes from (the primary, or a donor).
    pub source: PathBuf,
    /// The resolved `-J` track id, or `None` for an unmatched optional rule.
    pub track_id: Option<u64>,
}

/// The fully resolved plan for one primary (spec 3). Present only when the file
/// has no error-severity diagnostic.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Plan {
    /// The primary source file.
    pub source: PathBuf,
    /// The rendered absolute output path.
    pub output: PathBuf,
    /// One entry per track rule, in profile order (also the output track
    /// order, spec 4.5).
    pub assignments: Vec<Assignment>,
}

/// Per-file result: the plan (if any) and every diagnostic about the file.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FileReport {
    /// The primary source file.
    pub source: PathBuf,
    /// The identifier matched from its basename.
    pub identifier: String,
    /// The plan, or `None` if any diagnostic is error-severity (spec 5.1).
    pub plan: Option<Plan>,
    /// All diagnostics about this file (per-file and rule-level).
    pub diagnostics: Vec<Diagnostic>,
}

/// A structured, batch-validated suggested edit (spec 5.3, D6). Fully
/// specified and populated in Task 11; declared here so `Batch` is complete.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Suggestion {
    /// The diagnostic code this refinement resolves.
    pub resolves: DiagCode,
    /// Config path of the rule being edited.
    pub config_path: String,
    /// The structured edit (Task 11 defines the variants).
    pub edit: StructuredEdit,
    /// The exact YAML fragment the CLI prints / the GUI applies.
    pub yaml_fragment: String,
}

/// The closed grammar of suggestion edits (spec 5.3, D6); only ever narrows a
/// rule. Fully used by the engine in Task 11.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StructuredEdit {
    /// Add `property: value` to the rule's `exact` map.
    AddExact { property: String, value: String },
    /// Add `{ exact: { property: value } }` to the rule's `not` list.
    AddNotExact { property: String, value: String },
    /// Add `track_name: value` to the rule's `substring` map.
    AddSubstring { value: String },
    /// Add `{ substring: { track_name: value } }` to the rule's `not` list.
    AddNotSubstring { value: String },
}

/// The whole batch report (spec 3, 5): per-file results, batch-level
/// diagnostics (config/runtime/cross-file), and suggestions (Task 11).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Batch {
    /// One entry per discovered primary file.
    pub files: Vec<FileReport>,
    /// Diagnostics not tied to a single primary: runtime config checks
    /// (bad `language` value) and cross-file facts (`DuplicateIdentifier`).
    pub batch_diagnostics: Vec<Diagnostic>,
    /// Batch-validated suggested refinements (Task 11).
    pub suggestions: Vec<Suggestion>,
}

/// Plans the whole batch (spec 5.1, 5.5): validate runtime-only config
/// (language values), discover primaries, then resolve every file. Never
/// fails fast; a file with an error diagnostic contributes a plan-less
/// `FileReport` and planning continues.
pub fn plan_batch(
    profile: &Profile,
    run: &RunInputs,
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> Batch {
    let mut batch_diagnostics = Vec::new();

    // Runtime config validation: exact `language` values must be recognized
    // (plan-time InvalidPropertyValue, D2). File-independent, so batch-level.
    validate_language_values(profile, lang, &mut batch_diagnostics);

    // Discover primaries.
    let (primaries, discovery_diags) = discovery::scan_primaries(&run.source, &profile.input);
    batch_diagnostics.extend(discovery_diags);

    let primary_paths: Vec<PathBuf> = primaries.iter().map(|p| p.path.clone()).collect();
    let output_dir = run
        .output
        .clone()
        .or_else(|| profile.output.directory.clone())
        .unwrap_or_else(|| run.source.clone());
    let policy = run.on_collision.unwrap_or(profile.output.on_collision);

    // Resolve each file. Collect rendered outputs to detect cross-file
    // collisions in a second pass.
    let mut files: Vec<FileReport> = Vec::new();
    for primary in &primaries {
        files.push(resolve_file(profile, primary, &primary_paths, &output_dir, id, lang));
    }

    detect_output_collisions(&mut files, policy);
    finalize_plans(&mut files);

    Batch {
        files,
        batch_diagnostics,
        suggestions: Vec::new(), // filled by the suggestion engine (Task 11)
    }
}

fn validate_language_values(profile: &Profile, lang: &LanguageIndex, diags: &mut Vec<Diagnostic>) {
    for (i, rule) in profile.tracks.iter().enumerate() {
        walk_exact_languages(&rule.match_expr, &format!("tracks[{i}].match"), lang, diags);
    }
}

// Recurses match expressions collecting exact `language` values, checking each
// against the runtime index.
fn walk_exact_languages(expr: &MatchExpr, path: &str, lang: &LanguageIndex, diags: &mut Vec<Diagnostic>) {
    if let Some(exact) = &expr.exact {
        if let Some(Scalar::Str(v)) = exact.get("language") {
            if lang.normalize(v).is_none() {
                diags.push(
                    Diagnostic::error(DiagCode::InvalidPropertyValue, format!("{path}.exact.language"))
                        .with("property", "language")
                        .with("value", v.clone())
                        .with("allowed", "a valid ISO 639/BCP-47 language code"),
                );
            }
        }
    }
    if let Some(any) = &expr.any {
        for (i, sub) in any.iter().enumerate() {
            walk_exact_languages(sub, &format!("{path}.any[{i}]"), lang, diags);
        }
    }
    if let Some(not) = &expr.not {
        for (i, sub) in not.iter().enumerate() {
            walk_exact_languages(sub, &format!("{path}.not[{i}]"), lang, diags);
        }
    }
}

fn resolve_file(
    profile: &Profile,
    primary: &PrimaryFile,
    primary_paths: &[PathBuf],
    output_dir: &std::path::Path,
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> FileReport {
    let mut diagnostics = Vec::new();
    let primary_dir = primary.path.parent().unwrap_or(std::path::Path::new("."));

    // Identify the primary.
    let ident = match id.identify(&primary.path) {
        Ok(i) => i,
        Err(_) => {
            diagnostics.push(
                Diagnostic::error(DiagCode::MissingTrack, "input")
                    .for_file(&primary.path)
                    .with("detail", "file could not be identified"),
            );
            return FileReport {
                source: primary.path.clone(),
                identifier: primary.identifier.whole.clone(),
                plan: None,
                diagnostics,
            };
        }
    };
    if ident.format_version > PINNED_IDENTIFICATION_FORMAT_VERSION {
        diagnostics.push(
            Diagnostic::warning(DiagCode::UnknownPropertySkew, "input")
                .for_file(&primary.path)
                .with("version", ident.format_version.to_string()),
        );
    }

    // Resolve each rule independently (strict uniqueness, no consumption).
    let mut assignments = Vec::new();
    // (source_path, track_id) -> list of rule indices, to detect overlaps.
    let mut claims: BTreeMap<(PathBuf, u64), Vec<usize>> = BTreeMap::new();

    for (ri, rule) in profile.tracks.iter().enumerate() {
        let base = format!("tracks[{ri}]");
        let (source_path, source_ident): (PathBuf, Identification) = match &rule.source {
            SourceCfg::Keyword(_) => (primary.path.clone(), ident.clone()),
            SourceCfg::External(block) => {
                let hits = discovery::resolve_locator(&block.external, primary_dir, &primary.identifier);
                match hits.len() {
                    0 => {
                        if !rule.optional {
                            diagnostics.push(
                                Diagnostic::error(DiagCode::MissingExternal, format!("{base}.source.external"))
                                    .for_file(&primary.path),
                            );
                        }
                        assignments.push(Assignment { rule_index: ri, source: primary.path.clone(), track_id: None });
                        continue;
                    }
                    1 => {
                        let donor = hits.into_iter().next().unwrap();
                        if primary_paths.contains(&donor) {
                            diagnostics.push(
                                Diagnostic::warning(DiagCode::DonorIsPrimary, format!("{base}.source.external"))
                                    .for_file(&primary.path)
                                    .with("donor", donor.display().to_string()),
                            );
                        }
                        match id.identify(&donor) {
                            Ok(di) => (donor, di),
                            Err(_) => {
                                diagnostics.push(
                                    Diagnostic::error(DiagCode::MissingExternal, format!("{base}.source.external"))
                                        .for_file(&primary.path)
                                        .with("detail", "donor could not be identified"),
                                );
                                assignments.push(Assignment { rule_index: ri, source: primary.path.clone(), track_id: None });
                                continue;
                            }
                        }
                    }
                    n => {
                        diagnostics.push(
                            Diagnostic::error(DiagCode::AmbiguousExternal, format!("{base}.source.external"))
                                .for_file(&primary.path)
                                .with("count", n.to_string()),
                        );
                        assignments.push(Assignment { rule_index: ri, source: primary.path.clone(), track_id: None });
                        continue;
                    }
                }
            }
        };

        let matched: Vec<u64> = source_ident
            .tracks
            .iter()
            .filter(|t| matcher::matches(&rule.match_expr, t, lang))
            .map(|t| t.id)
            .collect();

        match matched.len() {
            0 => {
                if !rule.optional {
                    diagnostics.push(
                        Diagnostic::error(DiagCode::MissingTrack, format!("{base}.match")).for_file(&primary.path),
                    );
                }
                assignments.push(Assignment { rule_index: ri, source: source_path, track_id: None });
            }
            1 => {
                let tid = matched[0];
                claims.entry((source_path.clone(), tid)).or_default().push(ri);
                assignments.push(Assignment { rule_index: ri, source: source_path, track_id: Some(tid) });
            }
            n => {
                diagnostics.push(
                    Diagnostic::error(DiagCode::AmbiguousRule, format!("{base}.match"))
                        .for_file(&primary.path)
                        .with("count", n.to_string()),
                );
                assignments.push(Assignment { rule_index: ri, source: source_path, track_id: None });
            }
        }
    }

    // OverlappingRules: one track claimed by two or more rules (spec 5.2).
    for ((_src, tid), rules) in &claims {
        if rules.len() >= 2 {
            diagnostics.push(
                Diagnostic::error(DiagCode::OverlappingRules, format!("tracks[{}]", rules[0]))
                    .for_file(&primary.path)
                    .with("rule_a", format!("tracks[{}]", rules[0]))
                    .with("rule_b", format!("tracks[{}]", rules[1]))
                    .with("track", tid.to_string()),
            );
        }
    }

    // Output path (spec 4.8) + rendered-name invariants (D4).
    let output = render_output(profile, primary, output_dir, &mut diagnostics);

    // SourceOverwrite: output equals any input path (hard error, spec 5.2).
    if let Some(out) = &output {
        if primary_paths.contains(out) || out == &primary.path {
            diagnostics.push(
                Diagnostic::error(DiagCode::SourceOverwrite, "output")
                    .for_file(&primary.path)
                    .with("path", out.display().to_string()),
            );
        }
    }

    let plan = output.map(|output| Plan {
        source: primary.path.clone(),
        output,
        assignments,
    });

    FileReport {
        source: primary.path.clone(),
        identifier: primary.identifier.whole.clone(),
        plan,
        diagnostics,
    }
}

// Renders the output path, enforcing the D4 rendered-name invariants. Returns
// None (and pushes a diagnostic) if the rendered name is invalid.
fn render_output(
    profile: &Profile,
    primary: &PrimaryFile,
    output_dir: &std::path::Path,
    diags: &mut Vec<Diagnostic>,
) -> Option<PathBuf> {
    let name = match &profile.output.filename {
        FilenameCfg::Keyword(_) => {
            // keep: source basename, .mkv extension enforced.
            let stem = primary.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            format!("{stem}.mkv")
        }
        FilenameCfg::Template(block) => {
            let mut ctx = primary.identifier.to_ctx();
            if let Some(stem) = primary.path.file_stem().and_then(|s| s.to_str()) {
                ctx.set("source_stem", stem);
            }
            // A validated template parses; fall back to empty on the off chance.
            let mut rendered = Template::parse(&block.template)
                .map(|t| t.render_literal(&ctx))
                .unwrap_or_default();
            if !rendered.to_lowercase().ends_with(".mkv") {
                rendered.push_str(".mkv");
            }
            rendered
        }
    };

    // D4: separators and empty/./.. are errors, checked on the rendered name.
    if name.contains('/') || name.contains('\\') {
        diags.push(
            Diagnostic::error(DiagCode::PathSeparatorInRenderedName, "output.filename")
                .for_file(&primary.path)
                .with("name", name.clone()),
        );
        return None;
    }
    let stem_only = name.strip_suffix(".mkv").or_else(|| name.strip_suffix(".MKV")).unwrap_or(&name);
    if stem_only.is_empty() || name == ".mkv" || name == "." || name == ".." {
        diags.push(
            Diagnostic::error(DiagCode::EmptyRenderedName, "output.filename")
                .for_file(&primary.path)
                .with("name", name.clone()),
        );
        return None;
    }

    Some(output_dir.join(name))
}

// Second pass: two plans rendering to the same output path collide (spec 4.8).
fn detect_output_collisions(files: &mut [FileReport], policy: CollisionPolicy) {
    // Count planned outputs.
    let mut counts: BTreeMap<PathBuf, usize> = BTreeMap::new();
    for f in files.iter() {
        if let Some(p) = &f.plan {
            *counts.entry(p.output.clone()).or_default() += 1;
        }
    }
    for f in files.iter_mut() {
        let Some(plan) = &f.plan else { continue };
        let out = plan.output.clone();
        let planned_twice = counts.get(&out).copied().unwrap_or(0) >= 2;
        let exists_on_disk = out.exists();
        if !planned_twice && !exists_on_disk {
            continue;
        }
        // Two planned outputs to one path: always an error unless Skip. An
        // existing on-disk file: severity follows the policy.
        let severity = if planned_twice {
            match policy {
                CollisionPolicy::Skip => Severity::Warning,
                _ => Severity::Error,
            }
        } else {
            match policy {
                CollisionPolicy::Error => Severity::Error,
                CollisionPolicy::Skip => Severity::Warning,
                CollisionPolicy::Overwrite => Severity::Info,
            }
        };
        let mut d = Diagnostic::info(DiagCode::OutputCollision, "output")
            .for_file(&plan.source)
            .with("path", out.display().to_string());
        d.severity = severity;
        f.diagnostics.push(d);
    }
}

// Drop the plan for any file that has an error-severity diagnostic (spec 5.1).
fn finalize_plans(files: &mut [FileReport]) {
    for f in files.iter_mut() {
        let has_error = f.diagnostics.iter().any(|d| d.severity == Severity::Error);
        if has_error {
            f.plan = None;
        }
    }
}
```

Add `pub mod planner;` to `crates/muxsmith-core/src/lib.rs`.

Note for the implementer: `Diagnostic.severity` is a public field (Plan 1), so `d.severity = severity;` in `detect_output_collisions` compiles. If Plan 1 made it private, add a `Diagnostic::with_severity(self, Severity) -> Self` builder instead and use it. `capability` is imported but only `matchable_domain` may be unused here; if clippy flags an unused import, drop the `capability::{self, ...}` down to only what is used (`PINNED_IDENTIFICATION_FORMAT_VERSION`).

- [ ] **Step 5: Run the planner tests**

Run: `cargo test -p muxsmith-core --test planner_resolution`
Expected: PASS (7 tests). Re-run the whole core suite (`cargo test -p muxsmith-core`) and confirm the totals yourself; do not trust a reported count.

- [ ] **Step 6: Commit**

```bash
git add crates/muxsmith-core/src/planner.rs crates/muxsmith-core/src/identify.rs \
  crates/muxsmith-core/src/lib.rs crates/muxsmith-core/tests/planner_resolution.rs
git -c commit.gpgsign=false commit -m "feat(core): batch planner with per-file resolution and output paths

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 11: Suggestion engine (D6)

**Scope (flagged deferrals, not silent):** this task implements batch-validated suggestions for `AmbiguousRule` fully (the canonical single-rule case): generate discriminators, simulate against the whole batch via a re-plan, accept only refinements that resolve the conflict everywhere and introduce no new diagnostic, rank deterministically, cap at 3. Two D6 refinements are DEFERRED and must be carried in the HANDOFF: (a) auto-suggestions for `OverlappingRules` (the diagnostic still fires; the simulate/accept machinery generalizes to it later), and (b) the explicit no-single-fix partition report (when no refinement resolves every instance, spec 5.3's "list the files requiring different resolutions"). Deferring these keeps a useful, correct engine bounded; they are additive.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (extract `plan_core`, add the engine, rewire `plan_batch`)
- Test: `crates/muxsmith-core/tests/suggestions.rs`

**Interfaces:**
- Consumes: everything in Task 10.
- Produces:
  - `planner::plan_core(profile, run, primaries: &[PrimaryFile], id: &mut dyn Identify, lang: &LanguageIndex) -> Batch` (resolution only, `suggestions` empty). `plan_batch` now discovers primaries once, calls `plan_core`, then the engine.
  - The engine fills `Batch.suggestions` with `Suggestion { resolves, config_path, edit, yaml_fragment }` (types from Task 10), at most 3 per conflicted rule, deterministically ordered.

- [ ] **Step 1: Write the failing tests**

`crates/muxsmith-core/tests/suggestions.rs`:

```rust
use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{plan_batch, RunInputs, StructuredEdit};
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::report::DiagCode;

struct FakeIdent {
    by_name: HashMap<String, Identification>,
}
impl Identify for FakeIdent {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        let name = path.file_name().unwrap().to_str().unwrap();
        self.by_name.get(name).cloned().ok_or_else(|| IdentifyError::Json("no fixture".into()))
    }
}

fn lang() -> LanguageIndex {
    LanguageIndex::from_rows(&[["English", "eng", "eng", "en"]])
}

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");

// Ambiguous rule: both English SRT subtitle tracks match. A discriminator on
// forced_track resolves it batch-wide with no new diagnostic.
const P_AMBIGUOUS: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles, codec_kind: srt, language: en } }
"#;

fn plan(profile_yaml: &str) -> muxsmith_core::planner::Batch {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert("Show.S01E01.mkv".to_string(), Identification::from_json(SERIES).unwrap());
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(dir);
    batch
}

#[test]
fn ambiguous_rule_gets_a_validated_suggestion() {
    let batch = plan(P_AMBIGUOUS);
    // The conflict is present.
    assert!(batch.files[0].diagnostics.iter().any(|d| d.code == DiagCode::AmbiguousRule));
    // At least one suggestion is emitted, resolving the ambiguity.
    assert!(!batch.suggestions.is_empty(), "expected suggestions");
    assert!(batch.suggestions.iter().all(|s| s.resolves == DiagCode::AmbiguousRule));
    // Every suggestion targets the conflicted rule.
    assert!(batch.suggestions.iter().all(|s| s.config_path.starts_with("tracks[0]")));
    // Capped at 3.
    assert!(batch.suggestions.len() <= 3);
}

#[test]
fn every_suggestion_survives_the_next_dry_run() {
    // The core D6 invariant, as an executable property: applying any emitted
    // suggestion yields a re-plan with the conflict gone and no new diagnostic.
    let batch = plan(P_AMBIGUOUS);
    let base_yaml = P_AMBIGUOUS;
    for s in &batch.suggestions {
        // Apply the edit to tracks[0].match and re-plan.
        let edited = apply_edit_to_first_rule(base_yaml, &s.edit);
        let re = plan(&edited);
        assert!(
            !re.files[0].diagnostics.iter().any(|d| d.code == DiagCode::AmbiguousRule),
            "suggestion {:?} did not resolve the ambiguity",
            s.edit
        );
        // No new error diagnostic appeared for the file.
        assert!(
            !re.files[0].diagnostics.iter().any(|d| d.code == DiagCode::MissingTrack),
            "suggestion {:?} over-narrowed into MissingTrack",
            s.edit
        );
    }
}

// Helper: render an edited profile YAML by inserting the structured edit into
// the single rule's match. Mirrors what the GUI/CLI apply would do.
fn apply_edit_to_first_rule(_base: &str, edit: &StructuredEdit) -> String {
    let inner = match edit {
        StructuredEdit::AddExact { property, value } => {
            format!("exact: {{ type: subtitles, codec_kind: srt, language: en, {property}: {value} }}")
        }
        StructuredEdit::AddNotExact { property, value } => {
            format!("exact: {{ type: subtitles, codec_kind: srt, language: en }}\n      not:\n        - exact: {{ {property}: {value} }}")
        }
        StructuredEdit::AddSubstring { value } => {
            format!("exact: {{ type: subtitles, codec_kind: srt, language: en }}\n      substring: {{ track_name: {value} }}")
        }
        StructuredEdit::AddNotSubstring { value } => {
            format!("exact: {{ type: subtitles, codec_kind: srt, language: en }}\n      not:\n        - substring: {{ track_name: {value} }}")
        }
    };
    format!(
        "profile_version: 1\ninput: {{ pattern: 'S(?<s>\\d{{2}})E(?<e>\\d{{2}})', extensions: [mkv] }}\ntracks:\n  - match:\n      {inner}\n"
    )
}
```

- [ ] **Step 2: Run to verify they fail**

Run: `cargo test -p muxsmith-core --test suggestions`
Expected: FAIL (no suggestions emitted yet).

- [ ] **Step 3: Extract `plan_core` and rewire `plan_batch`**

In `crates/muxsmith-core/src/planner.rs`, change `plan_batch` to discover primaries once, delegate resolution to a new `plan_core`, then run the engine. Replace the current `plan_batch` body with:

```rust
/// Resolution-only planning (no suggestions), over an already-discovered
/// primaries list. The engine re-invokes this on edited profiles to simulate
/// candidate suggestions against the cached identification (spec 5.3, D6).
pub fn plan_core(
    profile: &Profile,
    run: &RunInputs,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> Batch {
    let mut batch_diagnostics = Vec::new();
    validate_language_values(profile, lang, &mut batch_diagnostics);

    let primary_paths: Vec<PathBuf> = primaries.iter().map(|p| p.path.clone()).collect();
    let output_dir = run
        .output
        .clone()
        .or_else(|| profile.output.directory.clone())
        .unwrap_or_else(|| run.source.clone());
    let policy = run.on_collision.unwrap_or(profile.output.on_collision);

    let mut files: Vec<FileReport> = Vec::new();
    for primary in primaries {
        files.push(resolve_file(profile, primary, &primary_paths, &output_dir, id, lang));
    }
    detect_output_collisions(&mut files, policy);
    finalize_plans(&mut files);

    Batch { files, batch_diagnostics, suggestions: Vec::new() }
}

/// Plans the whole batch and attaches batch-validated suggestions (spec 5).
pub fn plan_batch(
    profile: &Profile,
    run: &RunInputs,
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> Batch {
    let (primaries, discovery_diags) = discovery::scan_primaries(&run.source, &profile.input);
    let mut batch = plan_core(profile, run, &primaries, id, lang);
    // scan_primaries diagnostics are batch-level; plan_core re-derives none of
    // them, so fold them in here (once).
    batch.batch_diagnostics.extend(discovery_diags);
    batch.suggestions = suggest(profile, run, &primaries, id, lang, &batch);
    batch
}
```

Note: `plan_core` no longer calls `scan_primaries` (it takes `primaries`), so remove the `scan_primaries` call and the `discovery_diags` fold from the old body; `plan_batch` owns discovery now. Keep `validate_language_values`, `resolve_file`, `detect_output_collisions`, `finalize_plans` unchanged.

- [ ] **Step 4: Implement the engine**

Append to `crates/muxsmith-core/src/planner.rs`:

```rust
use crate::identify::PropValue;

// A candidate refinement plus the concrete Scalar to splice into the profile.
struct Candidate {
    edit: StructuredEdit,
    apply: MatchExpr, // the delta to merge into the rule's match_expr
    rank: (u8, String, String),
}

/// Generates and validates suggestions for every `AmbiguousRule` conflict
/// (spec 5.3, D6). For each conflicted rule: gather the matched (conflicting)
/// tracks across all affected files, derive discriminator candidates, simulate
/// each against the whole batch via [`plan_core`], and keep only those that
/// resolve the ambiguity everywhere with no new diagnostic. Deterministic;
/// capped at 3 per rule. OverlappingRules suggestions and the no-single-fix
/// partition report are deferred (see the task scope note).
fn suggest(
    profile: &Profile,
    run: &RunInputs,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
    baseline: &Batch,
) -> Vec<Suggestion> {
    // Baseline diagnostic signature multiset, to detect "no new diagnostic".
    let base_sig = diag_signature(baseline);

    // Conflicted rule indices, from AmbiguousRule diagnostics.
    let mut conflicted: Vec<usize> = baseline
        .files
        .iter()
        .flat_map(|f| f.diagnostics.iter())
        .filter(|d| d.code == DiagCode::AmbiguousRule)
        .filter_map(|d| rule_index_of(&d.config_path))
        .collect();
    conflicted.sort_unstable();
    conflicted.dedup();

    let mut out = Vec::new();
    for ri in conflicted {
        let Some(rule) = profile.tracks.get(ri) else { continue };
        // Only primary-source rules get suggestions in v1 (external deferred).
        if matches!(rule.source, SourceCfg::External(_)) {
            continue;
        }
        let candidates = candidates_for_rule(profile, ri, primaries, id, lang);
        let mut accepted: Vec<Candidate> = Vec::new();
        for cand in candidates {
            let edited = with_rule_match(profile, ri, &cand.apply);
            let sim = plan_core(&edited, run, primaries, id, lang);
            if resolves_without_regression(&sim, ri, &base_sig) {
                accepted.push(cand);
            }
        }
        accepted.sort_by(|a, b| a.rank.cmp(&b.rank));
        accepted.truncate(3);
        for cand in accepted {
            out.push(Suggestion {
                resolves: DiagCode::AmbiguousRule,
                config_path: format!("tracks[{ri}].match"),
                yaml_fragment: yaml_fragment(ri, &cand.edit),
                edit: cand.edit,
            });
        }
    }
    out
}

// The set of matched track ids per affected file for a rule, and the union of
// their property vectors, from which discriminators are drawn.
fn candidates_for_rule(
    profile: &Profile,
    ri: usize,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> Vec<Candidate> {
    let rule = &profile.tracks[ri];
    let mut raw: Vec<Candidate> = Vec::new();
    let mut seen: std::collections::BTreeSet<(String, String, u8)> = std::collections::BTreeSet::new();

    for primary in primaries {
        let Ok(ident) = id.identify(&primary.path) else { continue };
        let matched: Vec<&crate::identify::Track> = ident
            .tracks
            .iter()
            .filter(|t| matcher::matches(&rule.match_expr, t, lang))
            .collect();
        if matched.len() < 2 {
            continue; // not a conflict in this file
        }
        for t in &matched {
            for (prop, val) in t.properties.iter().chain(std::iter::once((&"type".to_string(), &PropValue::Str(t.kind.clone())))) {
                // Only propose over model-known, matchable properties.
                if capability::matchable_type(prop).is_none() {
                    continue;
                }
                let Some((display, scalar)) = prop_value_as(val) else { continue };
                // Positive exact (select this track's side) and negative exact.
                for (polarity, edit) in [
                    (0u8, StructuredEdit::AddExact { property: prop.clone(), value: display.clone() }),
                    (1u8, StructuredEdit::AddNotExact { property: prop.clone(), value: display.clone() }),
                ] {
                    if seen.insert((prop.clone(), display.clone(), polarity)) {
                        raw.push(Candidate {
                            apply: delta_for(&edit, &scalar),
                            rank: (rank_of(prop, polarity), prop.clone(), display.clone()),
                            edit,
                        });
                    }
                }
            }
            // track_name substring candidates from whitespace tokens.
            if let Some(PropValue::Str(name)) = t.get("track_name") {
                for tok in name.split_whitespace() {
                    for (polarity, edit) in [
                        (0u8, StructuredEdit::AddSubstring { value: tok.to_string() }),
                        (1u8, StructuredEdit::AddNotSubstring { value: tok.to_string() }),
                    ] {
                        let key = ("track_name~".to_string(), tok.to_string(), polarity);
                        if seen.insert(key) {
                            raw.push(Candidate {
                                apply: delta_for(&edit, &Scalar::Str(tok.to_string())),
                                rank: (rank_substring(polarity), "track_name".into(), tok.to_string()),
                                edit,
                            });
                        }
                    }
                }
            }
        }
    }
    raw
}

// Builds the MatchExpr delta a candidate edit represents.
fn delta_for(edit: &StructuredEdit, scalar: &Scalar) -> MatchExpr {
    let mut m = MatchExpr::default();
    match edit {
        StructuredEdit::AddExact { property, .. } => {
            let mut map = BTreeMap::new();
            map.insert(property.clone(), scalar.clone());
            m.exact = Some(map);
        }
        StructuredEdit::AddNotExact { property, .. } => {
            let mut inner = MatchExpr::default();
            let mut map = BTreeMap::new();
            map.insert(property.clone(), scalar.clone());
            inner.exact = Some(map);
            m.not = Some(vec![inner]);
        }
        StructuredEdit::AddSubstring { value } => {
            let mut map = BTreeMap::new();
            map.insert("track_name".to_string(), value.clone());
            m.substring = Some(map);
        }
        StructuredEdit::AddNotSubstring { value } => {
            let mut inner = MatchExpr::default();
            let mut map = BTreeMap::new();
            map.insert("track_name".to_string(), value.clone());
            inner.substring = Some(map);
            m.not = Some(vec![inner]);
        }
    }
    m
}

// Merges a delta into rule `ri`'s match expression, returning an edited profile.
fn with_rule_match(profile: &Profile, ri: usize, delta: &MatchExpr) -> Profile {
    let mut p = profile.clone();
    let expr = &mut p.tracks[ri].match_expr;
    if let Some(add) = &delta.exact {
        expr.exact.get_or_insert_with(BTreeMap::new).extend(add.clone());
    }
    if let Some(add) = &delta.substring {
        expr.substring.get_or_insert_with(BTreeMap::new).extend(add.clone());
    }
    if let Some(add) = &delta.not {
        expr.not.get_or_insert_with(Vec::new).extend(add.clone());
    }
    p
}

// Accept iff rule `ri` has no AmbiguousRule anywhere in the simulation AND no
// diagnostic in the simulation is absent from the baseline (no regression).
fn resolves_without_regression(sim: &Batch, ri: usize, base_sig: &std::collections::BTreeSet<String>) -> bool {
    let still_ambiguous = sim
        .files
        .iter()
        .flat_map(|f| f.diagnostics.iter())
        .any(|d| d.code == DiagCode::AmbiguousRule && rule_index_of(&d.config_path) == Some(ri));
    if still_ambiguous {
        return false;
    }
    let sim_sig = diag_signature(sim);
    sim_sig.iter().all(|s| base_sig.contains(s))
}

// A comparable signature set of all diagnostics in a batch: code + config_path
// + file. Used to detect newly-introduced diagnostics.
fn diag_signature(batch: &Batch) -> std::collections::BTreeSet<String> {
    let mut set = std::collections::BTreeSet::new();
    let all = batch
        .batch_diagnostics
        .iter()
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()));
    for d in all {
        let file = d.file.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
        set.insert(format!("{}|{}|{}", d.code.key(), d.config_path, file));
    }
    set
}

fn rule_index_of(config_path: &str) -> Option<usize> {
    let start = config_path.find("tracks[")? + "tracks[".len();
    let end = config_path[start..].find(']')? + start;
    config_path[start..end].parse().ok()
}

// Rank: typed flags/booleans (0) < language (1) < other exact (2); positive
// exact before its negation at equal property rank.
fn rank_of(prop: &str, polarity: u8) -> u8 {
    let base = match prop {
        "forced_track" | "default_track" | "flag_hearing_impaired" | "flag_visual_impaired"
        | "flag_commentary" | "flag_original" | "enabled_track" => 0,
        "language" | "language_ietf" => 1,
        _ => 2,
    };
    base * 2 + polarity
}

fn rank_substring(polarity: u8) -> u8 {
    // track_name substring ranks below exact conditions.
    6 + polarity
}

fn prop_value_as(v: &PropValue) -> Option<(String, Scalar)> {
    match v {
        PropValue::Bool(b) => Some((b.to_string(), Scalar::Bool(*b))),
        PropValue::Int(i) => Some((i.to_string(), Scalar::Int(*i))),
        PropValue::Str(s) => Some((s.clone(), Scalar::Str(s.clone()))),
        PropValue::Float(_) => None, // floats are poor discriminators; skip
    }
}

fn yaml_fragment(ri: usize, edit: &StructuredEdit) -> String {
    let body = match edit {
        StructuredEdit::AddExact { property, value } => format!("match:\n  exact: {{ {property}: {value} }}"),
        StructuredEdit::AddNotExact { property, value } => {
            format!("match:\n  not:\n    - exact: {{ {property}: {value} }}")
        }
        StructuredEdit::AddSubstring { value } => format!("match:\n  substring: {{ track_name: {value} }}"),
        StructuredEdit::AddNotSubstring { value } => {
            format!("match:\n  not:\n    - substring: {{ track_name: {value} }}")
        }
    };
    format!("# tracks[{ri}] - add:\n{body}")
}
```

Note for the implementer: the `.chain(std::iter::once((&"type".to_string(), ...)))` in `candidates_for_rule` needs matching reference types; if the borrow checker objects, materialize the properties into an owned `Vec<(String, PropValue)>` first (top-level `type` plus `t.properties`), then iterate that. Keep the behavior (offer `type` as a discriminator too). The `_base` parameter in the test helper is intentionally unused (the edited YAML is rebuilt from scratch).

- [ ] **Step 5: Run the suggestion tests and the whole core suite**

Run: `cargo test -p muxsmith-core --test suggestions` then `cargo test -p muxsmith-core`
Expected: PASS. Verify the total test count by re-running; do not trust a reported number.

- [ ] **Step 6: Commit**

```bash
git add crates/muxsmith-core/src/planner.rs crates/muxsmith-core/tests/suggestions.rs
git -c commit.gpgsign=false commit -m "feat(core): batch-validated suggestion engine for AmbiguousRule (D6)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

### Task 12: CLI `dry-run` and `identify`

**Files:**
- Modify: `crates/muxsmith-cli/src/cli.rs` (add subcommands)
- Modify: `crates/muxsmith-cli/src/main.rs` (dispatch)
- Create: `crates/muxsmith-cli/src/commands/dry_run.rs`
- Create: `crates/muxsmith-cli/src/commands/identify.rs`
- Modify: `crates/muxsmith-cli/src/commands/mod.rs`
- Modify: `locales/en/cli.ftl` (report rendering strings)
- Test: `crates/muxsmith-cli/tests/dry_run_cli.rs` (mkvmerge-gated end-to-end)

**Interfaces:**
- Consumes: `muxsmith_core::planner::{plan_batch, RunInputs}`, `identify::{IdentifyCache, LiveIdentifier}`, `capability::runtime::Mkvmerge`, the `i18n::Renderer` (Plan 1).
- Produces: `muxsmith dry-run <profile> [--source DIR] [--output DIR] [--json]` and `muxsmith identify <file> [--json]`, exit codes 0/1/2 mirroring mkvmerge (spec 8.1), rendering the batch report and diagnostics through Fluent.

- [ ] **Step 1: Add the subcommands**

In `crates/muxsmith-cli/src/cli.rs`, extend `Cmd`:

```rust
    /// Plan the batch without muxing: identify sources, resolve rules, and
    /// print the per-file resolution, diagnostics, and suggestions.
    DryRun {
        /// Path to the profile file.
        profile: PathBuf,
        /// Source directory to scan (overrides the profile default).
        #[arg(long)]
        source: Option<PathBuf>,
        /// Output directory (overrides the profile default).
        #[arg(long)]
        output: Option<PathBuf>,
        /// Emit the structured batch report as JSON.
        #[arg(long)]
        json: bool,
        /// Locale for rendered messages (default: system, fallback en).
        #[arg(long)]
        locale: Option<String>,
    },
    /// Identify one source file via mkvmerge and print its tracks.
    Identify {
        /// Path to the media file to identify.
        file: PathBuf,
        /// Emit the structured identification as JSON.
        #[arg(long)]
        json: bool,
        /// Locale for rendered messages (default: system, fallback en).
        #[arg(long)]
        locale: Option<String>,
    },
```

The doc comment on `Cmd` still says only validate/schema exist; update it to include dry-run/identify.

- [ ] **Step 2: Dispatch in `main.rs`**

Add arms to the `match args.command` in `crates/muxsmith-cli/src/main.rs`:

```rust
        cli::Cmd::DryRun { profile, source, output, json, locale } => {
            let renderer = i18n::Renderer::new(locale.as_deref());
            commands::dry_run::run(&profile, source, output, json, &renderer)
        }
        cli::Cmd::Identify { file, json, locale } => {
            let renderer = i18n::Renderer::new(locale.as_deref());
            commands::identify::run(&file, json, &renderer)
        }
```

- [ ] **Step 3: Register the command modules**

In `crates/muxsmith-cli/src/commands/mod.rs`:

```rust
pub mod dry_run;
pub mod identify;
pub mod validate;
```

- [ ] **Step 4: Implement `identify.rs`**

`crates/muxsmith-cli/src/commands/identify.rs`:

```rust
//! `muxsmith identify` (spec 8.1): identify one file via mkvmerge and print
//! its tracks (human-readable, or `--json` passing the structured data
//! through). Exit 0 on success, 2 on any failure to identify.

use std::path::Path;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{IdentifyCache, Identification};

use crate::i18n::Renderer;

/// Runs `muxsmith identify`. Returns the mkvmerge-style exit code.
pub fn run(file: &Path, json: bool, renderer: &Renderer) -> i32 {
    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{}", renderer.msg("mkvmerge-not-found", &[]));
            return 2;
        }
    };
    let mut cache = IdentifyCache::new();
    let id = match cache.get_or_identify(&mkv, file) {
        Ok(id) => id.clone(),
        Err(_) => {
            eprintln!("{}", renderer.msg("identify-failed", &[("file", &file.display().to_string())]));
            return 2;
        }
    };
    if json {
        print_identify_json(&id);
    } else {
        print_identify_human(&id, renderer);
    }
    0
}

fn print_identify_json(id: &Identification) {
    // Re-serialize a compact view; the CLI owns presentation, core owns data.
    let tracks: Vec<serde_json::Value> = id
        .tracks
        .iter()
        .map(|t| {
            serde_json::json!({ "id": t.id, "type": t.kind, "codec": t.codec })
        })
        .collect();
    println!(
        "{}",
        serde_json::json!({
            "file_name": id.file_name,
            "identification_format_version": id.format_version,
            "identifiable": id.is_identifiable(),
            "tracks": tracks,
        })
    );
}

fn print_identify_human(id: &Identification, renderer: &Renderer) {
    if !id.is_identifiable() {
        println!("{}", renderer.msg("identify-not-media", &[("file", &id.file_name)]));
        return;
    }
    for t in &id.tracks {
        let lang = match t.get("language") {
            Some(muxsmith_core::identify::PropValue::Str(s)) => s,
            _ => String::new(),
        };
        println!(
            "{}",
            renderer.msg(
                "identify-track-line",
                &[
                    ("id", &t.id.to_string()),
                    ("type", &t.kind),
                    ("codec", &t.codec),
                    ("language", &lang),
                ],
            )
        );
    }
}
```

- [ ] **Step 5: Implement `dry_run.rs`**

`crates/muxsmith-cli/src/commands/dry_run.rs`:

```rust
//! `muxsmith dry-run` (spec 8.1, 5.5): plan the batch without muxing and print
//! the per-file resolution, diagnostics, and suggestions. Exit code mirrors
//! mkvmerge: 0 clean, 1 worst diagnostic is a warning, 2 an error.

use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{Batch, RunInputs, plan_batch};
use muxsmith_core::profile::load;
use muxsmith_core::report::{Diagnostic, Severity};

use crate::i18n::Renderer;

/// Runs `muxsmith dry-run`. Returns the mkvmerge-style exit code.
pub fn run(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    json: bool,
    renderer: &Renderer,
) -> i32 {
    let profile = match load::from_file(profile_path) {
        Ok(p) => p,
        Err(d) => {
            println!("{}", renderer.diagnostic(&d));
            return 2;
        }
    };
    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{}", renderer.msg("mkvmerge-not-found", &[]));
            return 2;
        }
    };
    let lang = match mkv.list_languages() {
        Ok(l) => l,
        Err(_) => {
            eprintln!("{}", renderer.msg("mkvmerge-query-failed", &[]));
            return 2;
        }
    };
    // Source: flag, else profile has none stored (Plan 2 uses the flag or CWD).
    let source_dir = source.unwrap_or_else(|| PathBuf::from("."));
    let run = RunInputs { source: source_dir, output, on_collision: None };

    let mut ident = LiveIdentifier { cache: IdentifyCache::new(), mkv: &mkv };
    let batch = plan_batch(&profile, &run, &mut ident, &lang);

    if json {
        println!("{}", serde_json::to_string(&batch).unwrap());
    } else {
        print_batch_human(&batch, renderer);
    }
    exit_code(&batch)
}

fn all_diags(batch: &Batch) -> impl Iterator<Item = &Diagnostic> {
    batch
        .batch_diagnostics
        .iter()
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()))
}

fn exit_code(batch: &Batch) -> i32 {
    match all_diags(batch).map(|d| d.severity).max() {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    }
}

fn print_batch_human(batch: &Batch, renderer: &Renderer) {
    for f in &batch.files {
        println!(
            "{}",
            renderer.msg("dry-run-file", &[("file", &f.source.display().to_string()), ("id", &f.identifier)])
        );
        if let Some(plan) = &f.plan {
            for a in &plan.assignments {
                let track = a.track_id.map(|t| t.to_string()).unwrap_or_else(|| "-".into());
                println!(
                    "{}",
                    renderer.msg("dry-run-assignment", &[("rule", &a.rule_index.to_string()), ("track", &track)])
                );
            }
            println!("{}", renderer.msg("dry-run-output", &[("path", &plan.output.display().to_string())]));
        }
        for d in &f.diagnostics {
            println!("{}", renderer.diagnostic(d));
        }
    }
    for d in &batch.batch_diagnostics {
        println!("{}", renderer.diagnostic(d));
    }
    for s in &batch.suggestions {
        println!("{}", renderer.msg("dry-run-suggestion", &[("config_path", &s.config_path)]));
        println!("{}", s.yaml_fragment);
    }
}
```

- [ ] **Step 6: Add the CLI Fluent strings**

Append to `locales/en/cli.ftl`:

```
mkvmerge-not-found = mkvmerge was not found on PATH. Install MKVToolNix or set the mkvmerge path.
mkvmerge-query-failed = Querying mkvmerge failed.
identify-failed = Could not identify { $file }.
identify-not-media = { $file } is not a recognized media file.
identify-track-line = Track { $id }: { $type } [{ $codec }] { $language }
dry-run-file = { $file } (identifier: { $id })
dry-run-assignment =   rule { $rule } -> track { $track }
dry-run-output =   output: { $path }
dry-run-suggestion = Suggestion for { $config_path }:
```

- [ ] **Step 7: Write and run the end-to-end CLI test**

`crates/muxsmith-cli/tests/dry_run_cli.rs`:

```rust
//! End-to-end CLI test, gated on a real mkvmerge (self-skips otherwise).
//! Builds a fixture MKV via mkvmerge, writes a profile, runs `dry-run --json`,
//! and checks the batch report shape and exit code.

use std::process::Command;

use assert_cmd::cargo::CommandCargoExt;

fn have_mkvmerge() -> bool {
    Command::new("mkvmerge").arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
}

#[test]
fn dry_run_plans_a_single_file() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    // Build Show.S01E01.mkv from the committed seeds.
    let wav = concat!(env!("CARGO_MANIFEST_DIR"), "/../muxsmith-core/tests/fixtures/seeds/tone.wav");
    let srt = concat!(env!("CARGO_MANIFEST_DIR"), "/../muxsmith-core/tests/fixtures/seeds/sub.srt");
    let media = dir.path().join("Show.S01E01.mkv");
    let ok = Command::new("mkvmerge")
        .args(["-q", "-o"]).arg(&media)
        .args(["--language", "0:eng"]).arg(wav)
        .args(["--language", "0:eng"]).arg(srt)
        .status().unwrap().success();
    assert!(ok);

    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  - match: { exact: { type: audio } }\n",
    ).unwrap();

    let out = Command::cargo_bin("muxsmith").unwrap()
        .args(["dry-run"]).arg(&profile)
        .args(["--source"]).arg(dir.path())
        .args(["--output"]).arg(dir.path().join("out"))
        .arg("--json")
        .output().unwrap();

    assert!(out.status.success(), "exit: {:?}, stderr: {}", out.status.code(), String::from_utf8_lossy(&out.stderr));
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).expect("json report");
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    assert_eq!(report["files"][0]["identifier"], "S01E01");
    assert!(report["files"][0]["plan"].is_object());
}
```

Add `tempfile` to the CLI crate dev-deps if not present:

```bash
cargo add -p muxsmith-cli --dev tempfile
```

Run: `cargo test -p muxsmith-cli`
Expected: PASS (existing validate tests plus the new dry-run test; the latter self-skips without mkvmerge).

- [ ] **Step 8: Run the whole workspace suite, fmt, and clippy**

Run:
```bash
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
```
Expected: all green. Re-run and confirm counts yourself.

- [ ] **Step 9: Commit**

```bash
git add crates/muxsmith-cli/ locales/en/cli.ftl Cargo.lock
git -c commit.gpgsign=false commit -m "feat(cli): dry-run and identify subcommands rendering the batch report

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>"
```

---

## Post-plan: verification and handoff

- [ ] **Whole-branch review** (strongest model): run the superpowers:requesting-code-review flow over the full Plan 2 diff. Focus areas: matcher correctness (language + codec_kind edge cases), planner uniqueness and collision severity mapping, the suggestion engine's acceptance invariant, and prose-free core (no user strings outside `locales/`).
- [ ] **CI**: push and confirm the Linux-only matrix plus the new `deny` job pass (log the push in `gh-log.md`).
- [ ] **Journal**: append a process-journal entry per `docs/process-journal/PROMPT.md`; archive the SDD artifacts under `docs/process-journal/artifacts/plan-2-sdd/`.
- [ ] **HANDOFF**: record the deferred items so they are not lost: (1) suggestion engine OverlappingRules auto-suggestions and the no-single-fix partition report (Task 11 scope note); (2) attachments/chapters/tags/title resolution + command generation + `run`/executor (Plan 3); (3) extension validation against `--list-types` (no diagnostic code exists yet); (4) platform-standard mkvmerge install-location probing (Plan 4, GUI first-run). Distill durable decisions into a `project` memory or ARCHITECTURE note, then supersede the HANDOFF.
- [ ] **Self-review against the spec** (writing-plans checklist): confirm every spec 5.1-5.5 planning requirement in Plan 2 scope maps to a task, no placeholders remain, and type/name usage is consistent across tasks.

