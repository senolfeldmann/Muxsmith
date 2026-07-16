# Verdict (extracted from the reviewer transcript at write-time)

### Spec Compliance

- ✅ All four commands present with exact brief signatures — verified against `src-tauri/src/lib.rs:454,466,481,495-501` (diff lines 382-430): `load_profile(path: String) -> Result<Value, IpcError>`, `save_profile(path: String, profile: Profile) -> Result<(), IpcError>`, `validate_profile_model(profile: Profile) -> Result<Value, IpcError>`, `apply_suggestion(profile: Profile, config_path: String, edit: StructuredEdit) -> Result<Profile, IpcError>`.
- ✅ All four registered in `invoke_handler` (diff lines 477-493), inserted after `detect_mkvmerge`, before `get_settings`.
- ✅ All four `async` and wrapped in `on_blocking`; the two-reason distinction (disk vs. CPU-bound-per-keystroke) is stated correctly in the doc comments and matches D42's own text almost verbatim.
- ✅ Err-path restriction correct: `load_profile`/`validate_profile_model` wrap their body's `serde_json::Value` in `Ok(...)` inside the closure (so `Err` can only come from `on_blocking`'s own `spawn_blocking` panic-mapping to `internal-task-failed`, `lib.rs:75-81`); `save_profile`/`apply_suggestion` pass their body's own `Result` straight through.
- ✅ `load_profile_body` mirrors `validate_profile_body`, returns no bespoke struct, injects `"profile"` (model on `Ok`, `Null` on `ParseError`) — verified against `report::json::config_only_document`'s actual signature (`&[Diagnostic]`, `Option<bool>`, `&dyn DiagnosticRenderer`) and `load::from_file`'s actual return type (`Result<Profile, Diagnostic>`); both type-correct.
- ✅ `validate_profile(path)` untouched: no `+`/`-` line inside its doc comment or body in the diff; `BatchView.vue` is absent from the diff's 4-file change list (`src-tauri/src/error.rs`, `src-tauri/src/lib.rs`, `locales/{en,de}/gui-common.ftl`).
- ✅ `From<SaveError>`/`From<ApplyError>` exhaustively match the real enum variants (`SaveError::{Io,Serialize}` at `profile/save.rs:21-28`, `ApplyError::{UnparsableConfigPath,RuleIndexOutOfRange,EditChangedNothing}` at `planner.rs:1933-1949`); codes/params match D49 `:597-616` and the brief verbatim.
- ✅ Five catalog entries, both locales, byte-identical to the brief's blocks (I read both committed files directly and diffed by eye); single commit `997666a`.
- ✅ `apply-rule-index-out-of-range` uses the labelled value `(rule count: { $rules })`/`(Regelanzahl: { $rules })`; grep for `[one]`/`*[other]` across the whole diff returns nothing (only unrelated `-> Result<...>` Rust arrows matched `->`).
- ✅ de catalog register: existing IpcError entries in `gui-common.ftl` are consistently passive/descriptive ("konnte nicht ... werden"), not du-imperative (du-imperative and GUI-infinitive are reserved for `cli.ftl`/`gui-batch.ftl` per their own headers); the five new de lines follow the same passive register as their neighbors. Straight ASCII quotes used throughout; no hardcoded config keyword needing literal preservation appears in the new strings (only placeables).
- ✅ de file carries no per-block `##` headers anywhere (confirmed by grep), so omitting one for the new block matches existing convention; en file's new `## D42/D43` header matches the file's existing per-block-header convention.
- ✅ Fixture reuse: `LOADABLE_INVALID_PROFILE` extracted as a module-level const, the pre-existing test repointed to it, the new load-shape test reuses the same const plus the pre-existing missing-file fixture shape — no new literal invented.
- ✅ Both error-mapping tests and the load-shape test present with the brief's exact assertions (see Q2).
- ✅ Envelope identity assertions present (`config_diagnostics`/`files` equality, `profile` present/null, `validate_profile_body` carries no `profile` key).
- ✅ Explicit staging of exactly the four brief-named files, single commit, unsigned (`git show -s --format=%G?` returns `N`) — matches gate requirement.
- ✅ No forbidden typography glyphs anywhere in the diff (checked with a positive control against a real em-dash to confirm the check fires).

⚠️ Not independently reverified: the nine-part gate's actual pass/fail output (RED/GREEN counts, clippy/doc/deny/lint/build/e2e results) — trusted from the report per task instructions, since nothing in the diff gave cause for a targeted rerun.

### Adjudications

**Q1**: (a) Yes, genuinely consistent, exceptionless. Pre-existing `lib.rs` has a 4-for-4 `_body`-twin pattern: `validate_profile_body`/`validate_profile`, `dry_run_body`/`dry_run`, `identify_body`/`identify`, `detect_mkvmerge_body`/`detect_mkvmerge` — every existing `#[tauri::command]` has a private, separately-testable free-function twin, no exception. Extending this to the three new commands matches `conventions.yaml`'s match-the-house-pattern doctrine (a lone deviation — giving only `load_profile` a twin — would itself have been the outlier). (b) No unrequested behavior/surface: `save_profile_body`/`validate_profile_model_body`/`apply_suggestion_body` are all private (`fn`, not `pub fn`), add no public API surface, no wire-format change, and the diff adds exactly three new `#[test]`s total (`save_errors_map_to_distinct_codes`, `apply_errors_map_to_distinct_codes`, `load_profile_body_matches_validate_profile_diagnostics_and_adds_the_model`) — none calling the three extra twins directly.

That said, `docs/process-conventions.yaml`'s `proc-latitude-clause-boundary` (tier 2, settled, count 7, twice `violated-corrected`) explicitly rejects "disclosed-then-decided" as a substitute for routing a fork as NEEDS_CONTEXT before resolving it, and the brief's Files line names only "a `load_profile_body`" as the new twin. Read strictly, this was a fork the implementer should have routed rather than resolved-and-disclosed. Materially it differs from the rule's own cited violations (a missing struct-registry enumeration, a missing user-facing DiagCode) in a way that matters: those had multiple plausible answers with real product/wire-format content; this fork had zero decision content, mechanically derived from an unbroken local precedent, with no wire-format/API/behavior implication. **Ruling: correct conformance on the merits; not a scope-creep defect. Recorded as a Minor process note, not grounds for rejection** — flagged to HARVEST since the doctrine as worded gives no exemption for this class of zero-content structural fork.

**Q2**: Verified line-by-line against the brief's three blocks. `save_errors_map_to_distinct_codes` is byte-identical, unchanged even by fmt. `apply_errors_map_to_distinct_codes` differs only in fmt line-wrapping (the `unparsable`/`noop` `let` bindings got re-wrapped); every literal (`"not-a-rule-path"`, `7`, `1`, `0`, `"forced_track"`), every `assert_eq!`/`assert_ne!` call, and the struct-literal fields are unchanged. `load_profile_body_matches_validate_profile_diagnostics_and_adds_the_model` is likewise byte-identical modulo fmt-driven wrapping of the long `assert_eq!`/`assert!` calls, and both inline comments inside the test body are verbatim. **One exception outside the report's "line-wrapping only" framing**: the comment attached to `const LOADABLE_INVALID_PROFILE` (outside any test body) was substantively rewritten — the brief's "Extracted from lib.rs:569-572 (was inline); the loadable-invalid test above is repointed here too." was replaced with different prose citing `testing-support-helpers`. This is not something fmt does, and wasn't disclosed as a deviation beyond the fmt note. The const's own string literal is untouched.

### Strengths

- Rustdoc is extensive and every citation I spot-checked resolves to a real, matching ledger entry: `core-85-report-json-dry`, `core-124-error-currency-split` (including the exact `SaveError{Io,Serialize}` mapping ruling text), `core-33-suggestion-narrow-only`, `core-44-suggestion-no-clobber`, `testing-support-helpers`.
- `From<SaveError>`/`From<ApplyError>` match arms are exhaustive against the actual enum definitions in `muxsmith-core` — no wildcard papering over a future variant.
- The on_blocking placement doc comments correctly restate and apply D42's "touches disk" vs. "could stall the webview" distinction to all four commands, not just paraphrase it.
- `load_profile_body` avoids double-parsing the file (single `load::from_file` call feeds both diagnostics and model), matching the brief's literal Step 3 pseudocode exactly.
- Explicit `git add` of exactly the four brief-named files; single unsigned commit; diff stat matches the review package exactly.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
1. **Q1 process note** (`src-tauri/src/lib.rs:322-353`): per `docs/process-conventions.yaml`'s `proc-latitude-clause-boundary`, the three extra `_body` twins were a fork the implementer resolved and disclosed rather than routing as NEEDS_CONTEXT before resolving. Zero material/behavioral impact; correct on the merits (see Adjudications).
2. `src-tauri/src/lib.rs:507-512` (const `LOADABLE_INVALID_PROFILE`): the leading comment was substantively rewritten from the brief's verbatim text, not merely reformatted by `cargo fmt` as the report's "line-wrapping only" claim implies. Harmless — the literal value is untouched and the new comment is accurate — but the report's characterization of what the fmt pass changed is not fully precise.

### HARVEST

- `proc-latitude-clause-boundary` gives no exemption for a zero-decision-content structural choice that mechanically follows an already-exceptionless local pattern (a private testable-twin extraction with no wire-format/API/test-surface effect). Both of the rule's own cited `violated-corrected` occurrences involved genuine multi-answer forks with real product/wire-format content (an unenumerated struct list; a missing user-facing DiagCode); this task's fork had neither. Candidate: an explicit carve-out distinguishing "internal, zero-observable-effect code-structure choices that mirror an unbroken existing local convention" (already governed by match-the-house-pattern, no routing needed) from "forks with any wire-format, API-surface, test-surface, or multiple-plausible-answer content" (route as NEEDS_CONTEXT). Without it, the rule as currently worded would require halting for essentially every private-helper-extraction decision — not what its violation history is actually about.
- Minor brief-writing pattern: a Files/Interfaces line naming only one deliverable function (e.g., "a `load_profile_body`") reads, under current doctrine, as an implicit enumeration boundary. If the intent was narrower ("the other three commands don't need standalone unit tests, so don't extract them"), stating that explicitly removes this exact ambiguity at zero cost.

### Assessment
**Task quality:** Approved
**Reasoning:** Every binding point in the brief and D42/D43/D49 is implemented and independently verified against the diff and the committed files (signatures, on_blocking split with correct rationale, Err-path restriction, fixture reuse, bilingual catalog entries byte-identical in both locales, no plural selector, `validate_profile` untouched). The one genuine process question (Q1's extra `_body` twins) resolves in the implementer's favor on the merits with zero observable risk, recorded as a Minor process/HARVEST note rather than a blocking defect.
