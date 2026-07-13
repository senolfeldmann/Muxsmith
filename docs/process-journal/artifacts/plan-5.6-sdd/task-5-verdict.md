# Task 5 review verdict: CLI crate (Stream D)

Base `0b3149a`, Head `c877e4f`.

## Spec Compliance

- ✅ `commands/mod.rs:32` yagni - `all_diags` inlined into its sole caller `diag_exit_code`; ordering rationale correctly relocated onto `diag_exit_code`'s doc comment.
- ✅ `commands/validate.rs:19-34` dup (three sub-fixes) - `validate::run` now calls `commands::severity_sorted` (inline `Reverse`-sort dropped); `report::json::rendered_diags` made `pub` and called directly (inline `to_value`/`v["rendered"]` map dropped); `severity_exit(Option<Severity>) -> i32` extracted and shared by `diag_exit_code` and `validate::run` (duplicated fold dropped).
- ✅ `i18n.rs:45` dup - `msg` now delegates to `msg_with_counts(id, args, &[])`.
- ✅ `tests/cli_validate.rs:3` yagni - dead `mod support;` deleted; both `#[allow(dead_code)]` removed from `tests/support/mod.rs` (both helpers now have a real consumer via `insta_settings_with_tmp`'s internal call chain).
- ✅ `tests/run_cli.rs:498` + `tests/dry_run_cli.rs:576` dup - `fake_mkvmerge_that_fails_queries` hoisted verbatim into `tests/support/mod.rs` as `#[cfg(unix)] pub fn`; both call sites switched to `support::fake_mkvmerge_that_fails_queries()`; run_cli's now-obsolete "kept local per convention" note deleted.
- ✅ `tests/catalog_completeness.rs:453` yagni - `fixture_args`/`allowlisted_cli_key_args` return `Vec<(&'static str, &'static str)>` directly; `render_and_find_leaks` takes that shape; `string_pairs` and the `FluentValue` import are gone (`FluentArgs` import also correctly dropped as a necessary consequence, not scope creep - it would otherwise fail `-D warnings` as unused).
- ✅ Seed T8-m2 rename, all nine occurrences present and verified directly against the diff: `locales/en/cli.ftl` (message body byte-identical, only the id changed), `locales/de/cli.ftl` (same), `commands/mod.rs:129` call site, `catalog_completeness.rs` allowlist entry, `catalog_completeness.rs` fixture arm (`"batch-summary" => vec![("count","3"),("root","/in"),("extensions","mkv, mp4")]`, values preserved), `mod.rs` both test fn renames (`batch_summary_renders_the_singular_form_...` / `..._plural_form_...`), one comment each in `run_cli.rs`/`dry_run_cli.rs`. No `.snap` file appears in the diff - snapshots untouched, as the plan required.
- ⚠️ Cannot verify from diff alone: whether the bilingual rename (`en`+`de`) actually landed together in the single commit `c877e4f` rather than split across `0e8d048`/`c877e4f` - the reviewed artifact is the merged two-commit diff, which cannot attribute a hunk to one commit vs. the other, and per-commit attribution needs `git show`/`git log`, out of this review's scope (no git commands permitted). The report's own "Commit-splitting mechanics" section describes a plausible, deliberate procedure for exactly this (temporarily reverting the 7 occurrences in the 4 mixed files, committing group 1, reapplying, re-gating, then committing group 2), but that is the implementer's claim, not something I independently confirmed.
- ⚠️ Cannot verify from diff alone: unsigned commits / explicit per-file staging (`git -c commit.gpgsign=false commit`, no `git add -A`) - git-mechanics claims with no trace in file content.

Global constraints, all diff-verifiable and holding: one core visibility change only (`fn` -> `pub fn` on `rendered_diags`, nothing else touched in `report/json.rs`); JSON document shape unchanged (`{"diagnostics": entries}`, `entries` built by the same `to_value` + `rendered`-field injection as before); diagnostic order preserved (`severity_sorted`'s `sort_by_key(Reverse(severity))` over `diags.iter().collect()` is the same stable sort over the same input order as the original `Vec<Diagnostic>::sort_by_key`, so relative order for equal severities is unchanged); `diag_exit_code`'s exit-code mapping is bit-for-bit the same match, just factored through `severity_exit`; files touched match the brief's owned-file list exactly (verified against the diff's own file-stat header - 12 files, no `planner.rs`/`report/mod.rs`/frontend/`src-tauri` touch).

## Strengths

- Rename is complete and precise: all nine named sites checked individually against the diff, message text in both locales byte-identical except the id.
- The one core touch is exactly the single visibility keyword the brief authorized - verified by reading the full `json.rs` hunk, which contains nothing else.
- `severity_exit` extraction is a faithful, non-behavior-changing refactor in both call sites (`diag_exit_code`'s chain order and `match` arms are unchanged, just relocated).
- The `fixture_args`/`allowlisted_cli_key_args` rewrite (the largest hunk, ~350 lines) was spot-checked across a dozen arms of varying shape (0-param, 1-param, multi-param, the renamed `batch-summary` arm) - every value transcribed verbatim. Incidental improvement: the conversion moves the "these are all string values" guarantee from a runtime panic (`string_pairs`) to the compiler, since the deleted helper is no longer needed at all.
- House pattern `testing-support-helpers` (docs/conventions.yaml) correctly cited, including its exact idiomacy-finding reference (`run_cli.rs:L498`) and its distinction from the separate cross-crate 3x-duplication trigger.
- `docs/ROADMAP.md` edit is surgical: only the T8-m2 clause is removed from a comma-separated multi-stream enumeration; every sibling stream's pending item (T2-m1, T5-m2, T14-m1, etc.) is untouched, and the report flags the resulting merge-conflict risk across the six parallel wave-1 worktrees as a heads-up rather than trying to resolve it unilaterally.
- The disclosed judgment call (`severity_sorted(...).into_iter().cloned().collect()`) is accurate and correctly scoped: it is the forced consequence of two brief constraints intersecting (reuse `severity_sorted`, which is borrow-returning, while `rendered_diags`'s signature may not change beyond visibility), not a shortcut the implementer introduced by choice.

## Issues

### Critical (Must Fix)
None.

### Important (Should Fix)
None.

### Minor (Nice to Have)
- `commands/validate.rs` (the `severity_sorted(&collect(...)).into_iter().cloned().collect()` line): two `Vec<Diagnostic>`-shaped allocations occur (the original from `collect`, then the cloned owned copy) where a single in-place `sort_by_key` would have needed one. Negligible at this scale (config-diagnostic count, not a hot path) and structurally forced by the brief's own reuse mandate plus the "core visibility only" constraint - noted for completeness, not something to change within this task's scope.

## House dimension

- `testing-support-helpers` (docs/conventions.yaml) applied correctly, not newly discovered: hoisting `fake_mkvmerge_that_fails_queries` on 2 duplicates (not the tracked cross-crate 3x trigger) is explicitly brief-mandated, and the report's citation matches the ledger entry verbatim.
- `core-85-report-json-dry` (docs/conventions.yaml: "the batch/config/run JSON report documents... are hoisted from the CLI into core::report::json... neither surface owns document logic") is reinforced by this task: `validate.rs` now calls the shared `rendered_diags` instead of duplicating its logic, one step further toward the pattern's stated intent. `report/json.rs`'s stale-but-untouched doc comment ("mirrors `validate`'s own `--json` rendering") is a pre-existing, now slightly inaccurate wording (it describes independent duplication, not the current shared call) - correctly left alone given the "visibility only, no other core touch" constraint would forbid editing it here.
- No deviations from house convention found; no new pattern or rejection to harvest beyond what the report already surfaced (the ROADMAP shared-paragraph merge-conflict risk across the six wave-1 streams, and the ledger citations above).

## Assessment

**Task quality:** Approved
**Reasoning:** All seven brief items are implemented completely and correctly; the three named risks (rename completeness, validate.rs order/JSON-shape preservation, exit-code semantics) hold up against the diff; the one disclosed judgment call is sound and scope-forced rather than a shortcut.
