# Task 11 review verdict: config_diagnostics helper

### Spec Compliance
- ✅ Two new pub helpers in `crates/muxsmith-core/src/profile/validate.rs`: `config_diagnostics(profile: &Profile) -> Vec<Diagnostic>` and `config_diagnostics_from_file(path: &Path) -> Vec<Diagnostic>`, signatures match the brief's interfaces exactly.
- ✅ Meaning-stating rustdoc on both: `config_diagnostics` names the two-step contract and its two callers ("CLI `validate`, GUI `validate_profile`"); `config_diagnostics_from_file` states the short-circuit-on-load-failure semantics. Neither is a name-echo.
- ✅ Zero behavior change verified by direct comparison. CLI `collect()`'s deleted body and src-tauri `validate_profile_body()`'s deleted body are structurally identical (`match load::from_file(..) { Err(d) => vec![d], Ok(profile) => { let mut diags = validate::validate(&profile); diags.extend(lint::provable_overlaps(&profile)); diags } }`, differing only in the parameter name used, `profile_path` vs `path`, which is not a body difference). The new `config_diagnostics_from_file` reproduces this exactly, delegating the `Ok` arm to `config_diagnostics`, which itself reproduces the `validate` + `lint::provable_overlaps` sequence in the same order. Both original call sites are byte-identical pre-hoist, confirming the implementer's claim.
- ✅ Exactly two consumers switched: `crates/muxsmith-cli/src/commands/validate.rs::collect()` and `src-tauri/src/lib.rs::validate_profile_body()`. Both diff hunks confirm this and nothing else in either file.
- ✅ Scope boundary held. Authorized grep for `provable_overlaps` across `crates/` and `src-tauri/` shows exactly: the definition (`lint.rs:18`), its unit test (`lint.rs:71`), the new helper's internal call (`validate.rs:183`, plus a doc-comment mention at `:177`), and the four untouched planning-pipeline two-liners (`muxsmith-cli/src/commands/run.rs:85`, `muxsmith-cli/src/commands/dry_run.rs:60`, `src-tauri/src/run.rs:262`, `src-tauri/src/lib.rs:208` = `dry_run_body`). No unexpected call sites, no fifth site touched.
- ✅ One rider line added to `docs/ROADMAP.md`'s Plan-6 anchor: appended to the end of the "Further named inputs (2026-07-12, idiomacy review triage)" paragraph, the correct plan_pipeline/Plan-6 anchor. Diff shows it as two changed lines only because of the paragraph's existing line-wrap width; it is one added sentence, matching the brief's exact wording ("plan_pipeline consumes profile::validate::config_diagnostics (landed Plan 5.6 T11)").
- ✅ Files touched match the brief's file list exactly (4 files, no scope creep): `crates/muxsmith-core/src/profile/validate.rs`, `crates/muxsmith-cli/src/commands/validate.rs`, `src-tauri/src/lib.rs`, `docs/ROADMAP.md`.
- ⚠️ "Unsigned commit, explicit staging, ran directly on master (wave-2 serial)" is a procedural claim from the report not independently checkable from the diff or the one authorized grep (no git commands used in this review). Not contradicted by anything in scope; flagged as unverified rather than confirmed.

### Strengths
- The consumer-side doc comment on `validate_profile_body` was rewritten, not just left stale: it now correctly names `config_diagnostics_from_file` as the shared funnel instead of describing inline steps that no longer exist in that function, while preserving the surrounding rationale about the GUI's document-shape choice untouched.
- Import hygiene: CLI's `use muxsmith_core::profile::{lint, load, validate}` was trimmed to `profile::validate` since neither `lint::` nor `load::` remained referenced in that file; src-tauri's equivalent import was correctly left untouched because `dry_run_body` still calls `load::from_file`/`validate::validate`/`lint::provable_overlaps` directly.
- Naming (`config_diagnostics`, `_from_file` suffix) matches existing codebase idiom (`load::from_file` already uses this suffix convention).

### Issues
No critical or important issues found.

#### Minor
- None load-bearing. The ROADMAP rider reads slightly dense as one appended sentence directly after a closing paren ("...check-i18n parity). plan_pipeline consumes..."), but this matches the paragraph's existing style of using bare code identifiers as sentence subjects and is a wording nit, not a defect.

### House dimension
The change follows the "one funnel every call routes through" house pattern already established in this file's surrounding rustdoc style: meaning-stating comments that name the contract and callers rather than restate the signature, intra-doc links (`` [`validate`] ``, `` [`lint::provable_overlaps`] ``, `` [`config_diagnostics`] ``) consistent with the rest of `validate.rs`, and a query-style naming convention (`_from_file`) reused from the existing `load::from_file`. The four planning-pipeline sites were deliberately left as their own two-liner per the brief rather than folded into the new helper, correctly treating this as a distinct future consolidation (Plan 6) rather than pulling it in now, consistent with the earned-abstraction principle (don't hoist beyond the two config-time consumers this task actually owns).

### Assessment
**Task quality:** Approved
**Reasoning:** All binding constraints verified directly: the two deleted bodies are structurally identical to each other and to the new helpers' bodies (true zero-behavior-change hoist), exactly two consumers switched, the scope-boundary grep shows the four pipeline sites and nothing else affected, and the ROADMAP rider lands in the correct anchor with the exact specified wording.
