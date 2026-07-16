# Verdict (extracted from the reviewer transcript at write-time)

### Spec Compliance

- ✅ Four `pub const KEYWORDS: &'static [&'static str]` associate-consts added, one per enum (`FilenameCfg` `crates/muxsmith-core/src/profile/model.rs:138`, `SourceCfg` `:181`, `ChaptersCfg` `:224`, `TitleCfg` `:267`), placed beside the enum via a fresh inherent `impl` block for `ChaptersCfg`/`TitleCfg` (which previously carried only `impl Default`), matching D46's exact code sample (`docs/superpowers/specs/2026-07-15-plan-6-design.md:1198-1201`).
- ✅ Values byte-identical to D46's measured domains: `["keep"]`, `["primary"]`, `["keep","drop"]`, `["keep","clear"]` (design `:1198-1201`, `:1224-1226`) — confirmed by direct comparison, not by trusting the report.
- ✅ Placement in `profile::model`, not `capability` (design `:1207-1218`).
- ✅ All four validate.rs guards repointed at `<Cfg>::KEYWORDS.contains(&k.as_str())` (`crates/muxsmith-core/src/profile/validate.rs:296,322,344,363`), all four `allowed` literals replaced with `domain_hint(<Cfg>::KEYWORDS)` — `domain_hint` (`validate.rs:430`) reused unchanged.
- ✅ `Keyword(String)` arm keeps its `String` in all four enums (`model.rs:164,253,381,425`) — only a `#[schemars(schema_with = ...)]` attribute was added above each, no retyping.
- ✅ Schema override emits `{"type":"string","enum":[...]}` via a shared `keyword_domain_schema` helper (`model.rs:102-107`), not `oneOf`+`const` — matches D46's rejection of that alternative (design `:1247-1258`).
- ✅ Variant doc comments left untouched (not replaced) so schemars' own metadata-merge produces the combined `description`, per D46 `:1164-1166`; implementer additionally traced the schemars 1.2.1 source (`schemars_derive-1.2.1/src/schema_exprs.rs:600`, `schemars-1.2.1/src/_private/mod.rs:324`) rather than taking the design's empirical claim on faith — good practice, verifiable in principle though I did not re-trace the crate source myself.
- ✅ `schema_json()` extracted in `cli_schema.rs:6-14` from the pre-existing single inline invocation; `schema_prints_json_schema_and_exits_zero` re-pointed at it (`:17-22`); no second invocation path introduced.
- ✅ `keyword_domains_project_as_closed_enums_not_bare_strings` present with the brief's exact logic (`cli_schema.rs:38-78`) — text differs only in rustfmt line-wrapping (two `unwrap_or_else` closures reflow across lines), substance identical.
- ✅ `misspelled_chapters_keyword_is_invalid_keyword_with_const_derived_allowed` (`validate_semantics.rs:397-416`) asserts both `d.code == DiagCode::InvalidKeyword` (via `.find`) and `d.params["allowed"] == "keep, drop"`, matching brief step 6's dual requirement.
- ✅ Rustdoc present on all four public consts and (though not lint-required) on the private helper and four wrapper fns.
- ✅ Scope: exactly the four files the brief named (`git diff --stat` in the review package matches); no stray edits.
- ⚠️ Cannot independently verify from the diff alone: the actual `cargo test --workspace` (476/476) and nine-part gate outcomes, and that the commit is genuinely unsigned with the exact `git add` file list from step 8 — the diff's commit-list header is consistent with the prescribed commit message, and I did not re-run git per instructions.

### Adjudications

**Q1**: Vacuity claim is **correct**, confirmed independently, not weighed.
- Checked (a) myself: grepped all 11 tracked `.snap` files under `crates/muxsmith-cli/tests/snapshots/` for `keyword|allowed|invalid` (case-insensitive). Four matched on "invalid" — `cli_validate__invalid_profile_...`, both `run_cli__bad_regex_profile_...` variants, `dry_run_cli__...mkvmerge_missing` — and reading all four in full, every one renders a regex-parse error, a `forced_track` type-mismatch error, or a rules-overlap warning. None renders `InvalidKeyword` or an `Allowed:` hint, and none touches `output.filename`, `track.source`, `chapters`, or `title` at all — the four fields this diff's guards live on. So no pinned fixture exercises this diff's code paths in any form, not just not the specific `allowed` string.
- Checked (b) against house record rather than guessing: `docs/decision-ledger.yaml:2437` states this project's insta setup is "CI strict compare (default CI=true / no INSTA_UPDATE)", and no `.config/insta.yaml` or `Settings::new()`/force-update call exists anywhere in the tree (searched). Under insta's documented `INSTA_UPDATE=auto` default, that resolves to `no`/`new` mode: a mismatch writes an untracked `.snap.new` and fails the test — it never rewrites the tracked `.snap` in place. So `git diff --exit-code` on the snapshots dir adds zero signal beyond "did `cargo test` already pass": if a change altered a snapshot-exercised rendering, the test would fail before the diff step ever ran; if it didn't, the diff step is trivially clean regardless of what changed elsewhere.
- Separate question — does the check guard something real here that was missed: no. I checked whether any of the touched diagnostics (`InvalidKeyword` on the four fields) appears anywhere across all 11 fixtures, not just the four "invalid"-matching ones (the other 7 render success/info paths per their filenames — `valid_profile`, `warnings_only`, `speaks_on_an_empty_source_dir`, `live_run_muxes_two_sources`); none does. The plan's own proof claim (design `:1226-1229`, "the existing snapshot tests ... prove it") does not hold for this task, and the implementer's deliberate-break exercise (breaking `ChaptersCfg::KEYWORDS`, confirming the CLI suites stayed green and `git diff` stayed clean while the two new unit tests caught it) is real verification of the vacuity, not an unverified assertion.

**Q2**: E2e-only coverage is the **plan's own intended structure**, not a gap.
- Brief step 1 (lines 19-66) mandates exactly one CLI-level test with a verbatim body, asserting the schema-projection shape end-to-end through `muxsmith schema`'s stdout. Brief step 6 (lines 87-89) mandates exactly one core-level test, but scoped to `InvalidKeyword` reachability (the guard/`domain_hint` half), not to the schema-projection half. No step asks for a core-level unit test of `keyword_domain_schema` or the four `*_keyword_schema` wrapper fns directly.
- This matches existing house practice, not just this brief: grepping `crates/muxsmith-core/tests/` for `schema_with`/`SchemaGenerator`/`schema_for` turns up nothing — there is no precedent anywhere in this codebase for a core-level unit test asserting a schemars-derived shape; `cli_schema.rs`'s pre-existing sole test (`schema_prints_json_schema_and_exits_zero`) was already CLI-level-only before this task touched it. The task did not introduce a new coverage pattern; it followed the one already in place.
- The implementer's own "Concerns" item 2 flags this without claiming it's a defect — correctly hedged, and correct on the merits.

### Strengths

- Clean single-source refactor: three consumers (guard, `allowed` hint, schema) now genuinely derive from one array per enum, with zero duplicated literals surviving in `validate.rs`.
- The `keyword_domain_schema` shared-helper-plus-thin-wrappers shape avoids reimplementing the `{"type":"string","enum":[...]}` literal four times while still satisfying `schema_with`'s required `fn(&mut SchemaGenerator) -> Schema` signature — mirrors the house's `CODEC_KIND_NAMES` "derived, not hand-re-listed" principle (`capability/mod.rs:125-129`) the design cites as precedent.
- Genuine empirical verification throughout rather than trusting the design doc: traced schemars 1.2.1 source for the description-merge mechanism, deliberately broke a constant to prove the new tests (not the snapshot check) catch the regression, and manually confirmed all four `allowed` values byte-for-byte via a throwaway multi-field-invalid profile before reverting.
- The snapshot-vacuity finding is correctly filed as a process observation (design/plan-owner note) rather than smuggled in as an excuse to skip the prescribed step — the prescribed `git diff --exit-code` was still run and reported.
- No typography violations, no scope creep, no warning suppression anywhere in the diff.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- `model.rs`: the four `*_keyword_schema` wrapper fns are structurally identical one-liners (`keyword_domain_schema(<Cfg>::KEYWORDS)`); a future fifth keyword-bearing enum would add a fifth copy-pasted wrapper. Not worth a macro at n=4 ([[feedback_scale_appropriate_design]] territory generally, and matches this review's own house-scale judgment), just flagging in case a Task 5+ addition changes the count.

### HARVEST

- **Doctrine reinforcement candidate**: this task is a clean second instance of the standing "a check whose passing result is an absence must be proven to fire" principle (already codified per the review brief's own "Standing check" clause) — here applied to a plan-authored proof mechanism (D46's own snapshot claim), not just an implementer-authored one. Worth an ADR/ledger note that D46's "existing snapshot tests prove it" line is now known-false for the keyword-domain `allowed` claim specifically, so a later task (e.g. an emitter or GUI consumer of `KEYWORDS`) doesn't inherit the same false assurance by citing D46 without re-checking fixture coverage.
- **Pattern worth naming**: shared-private-helper-plus-thin-typed-wrappers for schemars `schema_with` fns, where the wrapper only exists to match the trait-required signature and immediately delegates to a plain-data helper. Not novel here, but if plan-6 grows more schema overrides, this is the shape to reuse rather than re-deriving it.
- No rejections, deviations, or contested criteria surfaced in this task; execution matched the brief tightly enough that there is little else to log.

### Assessment
**Task quality:** Approved
**Reasoning:** Every brief-mandated item is present and verified against the diff and D46's measured values; both adjudication questions resolve cleanly in the implementer's favor on independent verification (snapshot vacuity confirmed via fixture grep + insta default semantics; e2e-only coverage confirmed as the brief's own structure, not an omission). No Critical or Important findings.
