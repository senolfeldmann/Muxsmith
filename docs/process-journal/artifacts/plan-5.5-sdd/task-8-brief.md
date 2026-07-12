### Task 8: Empty-batch summary line, always-print (#8)

**Files:**
- Modify: `crates/muxsmith-cli/src/commands/run.rs:168-179` and the dry-run equivalent (`dry_run.rs`, same pattern), `commands/mod.rs` (`print_batch_human`)
- Modify: `locales/en/cli.ftl` (summary message gains the zero-case or a dedicated `run-empty-batch` key with `$root` and `$extensions` params)
- Test: `crates/muxsmith-cli/tests/run_cli.rs` / `dry_run_cli.rs`

- [ ] Step 1: Failing test: run over a directory with zero matching files, human mode; assert stdout contains the summary line with "0" and the searched root, exit 0 unchanged.
- [ ] Step 2: Implement: `print_batch_human` always prints the batch summary line (not only when non-empty); JSON path unchanged (already emits the zeroed document).
- [ ] Step 3: Full gate; commit `feat(cli): empty batch speaks - always print the batch summary (#8)`.

