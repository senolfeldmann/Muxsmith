### Task 11: Test-hardening rider, group T (#21)

**Files:**
- Modify: `crates/muxsmith-core/tests/` (donor-ordering golden; identify parse-edge tests near `identify.rs:225`'s documented cases), `crates/muxsmith-core/src/executor/` tests (exit-1 output-kept assertion; fail-fast-with-non-first-failing-job), `crates/muxsmith-cli/tests/dry_run_cli.rs:283ff` (default-branch severity assertion), `crates/muxsmith-core/tests/fixtures/identify/with-attachments.json` (attachment ids 0,1,2 -> 1,2,3 + a fixture comment naming the real tool as source of truth)

- [ ] One failing test per gap, then minimal fixes (the fixture change may break id-hardcoded assertions - fix those to be id-agnostic or 1-based, which is the point).
- [ ] Full gate; commit `test: close the audited coverage gaps (group T, #21)`.

