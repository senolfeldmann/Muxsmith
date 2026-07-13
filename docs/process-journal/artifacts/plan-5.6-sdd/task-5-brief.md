### Task 5: CLI crate (Stream D)

**Files:**
- Modify: `crates/muxsmith-cli/src/commands/mod.rs`, `crates/muxsmith-cli/src/commands/validate.rs`, `crates/muxsmith-core/src/report/json.rs` (pub visibility only), `crates/muxsmith-cli/src/i18n.rs`, `crates/muxsmith-cli/tests/cli_validate.rs`, `tests/run_cli.rs`, `tests/dry_run_cli.rs`, `tests/support/mod.rs`, `tests/catalog_completeness.rs`, `locales/en/cli.ftl`, `locales/de/cli.ftl`, `docs/ROADMAP.md` (one resolved deferral line)

**Interfaces:** makes `report::json::rendered_diags` pub (core); renames Fluent key `dry-run-summary` -> `batch-summary` (bilingual, same commit).

- [ ] `commands/mod.rs:32` **yagni** - inline all_diags' chain (`config_diags.iter().chain(&batch.batch_diagnostics).chain(batch.files.iter().flat_map(|f| &f.diagnostics))`) into diag_exit_code, its only caller; move the ordering rationale onto diag_exit_code's doc.
- [ ] `commands/validate.rs:19-34` **dup** - reuse commands::severity_sorted (drop the inline Reverse-sort); make report::json::rendered_diags pub and call it (drop the inline to_value + v["rendered"] map); extract `fn severity_exit(Option<Severity>) -> i32` shared with diag_exit_code (drop the duplicated fold).
- [ ] `i18n.rs:45` **dup** - `pub fn msg(&self, id: &str, args: &[(&str, &str)]) -> String { self.msg_with_counts(id, args, &[]) }`.
- [ ] `tests/cli_validate.rs:3` **yagni** - delete the dead `mod support;`; remove the two `#[allow(dead_code)]` from tests/support/mod.rs (lines 35, 47) - every real consumer binary uses both helpers.
- [ ] `tests/run_cli.rs:498` + `tests/dry_run_cli.rs:576` **dup** - hoist the verbatim-identical fake_mkvmerge_that_fails_queries into tests/support/mod.rs as `pub #[cfg(unix)] fn`; both files call it; delete run_cli's "kept local per this file's existing per-file-helper convention" note (triage settled the share). Distinct from the tracked cross-crate three-copy decision (trigger >3 stands).
- [ ] `tests/catalog_completeness.rs:453` **yagni** - fixture_args and allowlisted_cli_key_args return `Vec<(&'static str, &'static str)>`; pass pairs straight to renderer.msg in render_and_find_leaks; delete string_pairs and the FluentValue import.
- [ ] **seed T8-m2** - rename `dry-run-summary` -> `batch-summary` at all nine occurrences: locales/en/cli.ftl:23 + locales/de/cli.ftl:28 (definitions, same commit - bilingual rule), mod.rs:129 (call site), catalog_completeness.rs:246 (allowlist) + :313 (fixture arm), mod.rs:217/:236 (test fn names -> batch_summary_renders_...), run_cli.rs:358 + dry_run_cli.rs:243 (comments). Snapshots untouched (verified: no .snap carries the key name; they capture rendered text). Then delete the resolved deferral line in docs/ROADMAP.md (grep "dry-run-summary"; the Plan-6 anchor names it under "Further named inputs (2026-07-12, Plan 5.5 roll-up funnel)" as T8-m2).
- [ ] Full gate; commits `refactor(cli): ...` / `i18n(cli): rename dry-run-summary to batch-summary (T8-m2)`.

