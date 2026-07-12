# Task 11 report: test-hardening rider, group T (#21)

Commit: `543259b test: close the audited coverage gaps (group T, #21)` on
`plan55-stream-d` (worktree `.worktrees/stream-d`).

## Gap (i): donor-ordering golden for mixed `track_id: None/Some` assignments

**Unpinned:** `command::push_track_order` (drop mode) and
`push_track_order_keep` (D20 keep mode) both `filter_map` over
`plan.assignments`, skipping `None`-track entries. No existing test
exercised a `None` assignment sitting *between* two `Some` ones across
multiple distinct donor sources - the only prior mixed case
(`unmatched_donor_rule_opens_no_input_group`) had a single trailing `None`
after one `Some`, too weak to pin group-index assignment order or
in-place `None` skipping under a denser mix.

**Failing test first:** added
`donor_ordering_drop_mode_with_mixed_none_and_some_assignments` and
`donor_ordering_keep_mode_with_mixed_none_and_some_assignments` in
`crates/muxsmith-core/tests/command.rs`, asserted a deliberately wrong
`--track-order` string, ran `cargo test -p muxsmith-core --test command
donor_ordering`, confirmed both FAILED with the actual computed order shown
in the panic message, then corrected to the actual value.

**Fix:** none needed to `command.rs` - the existing algorithm was already
correct; the test closes the coverage gap. Pinned behavior: `None`
assignments are skipped in place without disturbing surrounding `Some`
order; a group's index is first-Some-appearance order (so a donor first
hit by a later rule gets a higher index than one hit earlier, independent
of an intervening failed/`None` rule on either source); a source with only
`None` assignments opens no input group at all, in both drop and keep mode.

Side note: while iterating on these two tests with `sed`, an identical
substring accidentally also mutated the pre-existing
`keep_unmatched_donor_trails_primary_track_order` test's expected value;
caught by rerunning the full `command.rs` suite and reverted before commit.

## Gap (ii): identify parse-edge tests (identify.rs:225)

**Unpinned:** `parse_attachment`'s doc comment at identify.rs:224-225
("Required fields (id, file_name, size) missing or wrong-typed drop the
entry") and the chapters `num_entries` sum (identify.rs:172-180) had no
test for: a wrong-typed `id` (string instead of number), a non-numeric
`num_entries`, or an attachment JSON with the `properties` key absent
entirely (not just empty).

**Failing test first:** added three unit tests to `identify.rs`'s
`mod tests` with deliberately wrong assertions (`len()==2` instead of `1`,
`chapters==17` instead of `5`, `uid==Some(1)` instead of `None`); ran
`cargo test -p muxsmith-core --lib identify::`, confirmed all three FAILED
against the actual parsed values; corrected.

**Fix:** none needed - `parse_track`/`parse_attachment`/the chapters sum
already behave per their doc comments (`Value::as_u64` returning `None` on
a wrong-typed field, whether via `?` on required fields or `filter_map` on
the chapters sum, correctly drops/skips rather than erroring or panicking).
Tests: `attachment_with_wrong_typed_id_is_dropped`,
`chapters_non_numeric_num_entries_is_skipped_not_erroring`,
`attachment_without_properties_key_has_no_uid`.

## Gap (iii): Plan-4 test gaps

### exit-1 output-kept assertion

**Unpinned:** `exit_one_is_warning_with_captured_lines` in
`executor/job.rs` never wrote an output file before running, so it could
not distinguish "output kept" from "output never produced" - unlike its
exit-0 and exit-2 siblings, which both write a file first and assert
existence/non-existence after.

**Failing test first:** wrote an output file before the run, added
`assert!(spec.output.exists(), ...)`, temporarily inverted it to
`!spec.output.exists()`, ran `cargo test -p muxsmith-core --lib
executor::job::tests::exit_one`, confirmed FAILED, reverted to the correct
assertion. Renamed to
`exit_one_is_warning_with_captured_lines_and_output_kept`.

**Fix:** none needed - `finish()` only deletes on `Failed`/`Cancelled`, so
`Warning` already keeps the output; the test now actually verifies this.

### fail-fast-with-non-first-failing-job queue test

**Unpinned:** `soft_fail_fast_cancels_queued_but_not_inflight` only fails
job index 0 (the first ever dequeued), so it could not distinguish a
`stop.store(true, ...)` genuinely gated on "the job that just finished
failed" from one accidentally gated on "index == 0".

**Failing test first:** added
`fail_fast_triggers_on_a_non_first_failing_job` (job 0 ok, job 1 fails,
job 2 must never spawn) to `executor/queue.rs`, temporarily corrupted the
expected outcome vector, ran `cargo test -p muxsmith-core --lib
executor::queue::tests::fail_fast_triggers_on_a_non_first`, confirmed
FAILED, reverted.

**Fix:** none needed - `stop.store(true, ...)` at queue.rs:256-258 already
fires on any `Failed` outcome regardless of index; job 0's outcome is
`Ok`, job 1's is `Failed`, job 2 is asserted both `Cancelled` and never
`Started`.

### dry_run_cli default-branch severity assertion

**Unpinned:** `diag_exit_code` (`muxsmith-cli/src/commands/mod.rs:33-39`)
has a two-armed match (`Error => 2`, `Warning => 1`) with a `_ => 0`
default arm covering `Severity::Info` and "no diagnostics at all". The
existing `dry_run_on_collision_flag_overrides_default_error_policy` test
(dry_run_cli.rs:283ff) covered the `Error` and `Warning` arms
(`--on-collision` default and `skip`) but never the default arm.

**Empirical grounding for the third case:** `planner.rs:940-944` maps
`CollisionPolicy::Overwrite` to `Severity::Info` for an on-disk output
collision - a natural end-to-end exercise of the default arm via
`--on-collision overwrite`.

**Failing test first:** extended the same test function with a third
`--on-collision overwrite` invocation, asserted exit code and
`severity: "info"`, temporarily changed the expected exit code to `Some(1)`,
ran `cargo test -p muxsmith-cli --test dry_run_cli
dry_run_on_collision_flag_overrides_default_error_policy`, confirmed
FAILED (actual was `Some(0)`, visible in the panic's captured stdout),
reverted to `Some(0)`.

**Fix:** none needed - the `_ => 0` arm already does the right thing; the
gap was purely missing coverage.

## Gap (iv): with-attachments.json 1-based attachment ids

**mkvmerge verification:** built a real MKV with two attachments via
`mkvmerge -q -o out.mkv --attach-file a.ttf --attach-file b.otf
sub.srt` (mkvmerge v100.0, installed at
`/home/linuxbrew/.linuxbrew/bin/mkvmerge`), then ran `mkvmerge -J
out.mkv`. Actual output: attachment ids `1` and `2` (not `0`/`1`); the
sole track's id stayed `0`. Confirms attachment ids are 1-based, track
ids 0-based, in the real `-J` wire format - the fixture had it backwards
for attachments.

**Fixture loader check:** `Identification::from_json` (identify.rs)
parses via manual `serde_json::Value::get(...)` lookups on named keys
only, no `deny_unknown_fields`, no strict struct deserialize anywhere in
the call path (`FakeIdent` in `tests/support/mod.rs` just calls
`Identification::from_json(text).unwrap()`). Confirmed a `_comment`
top-level key is silently ignored, so used it in place of a sibling
README (locality: the note sits directly on the data it explains).

**Change:** `crates/muxsmith-core/tests/fixtures/identify/with-attachments.json`
attachment ids `0,1,2` -> `1,2,3`, plus the `_comment` key naming mkvmerge
v100 as the verified source of truth.

**Failing test first (the breakage was the point):** ran `cargo test -p
muxsmith-core --test planner_resolution` immediately after the fixture
edit, confirmed the predicted breakage:
`attachment_select_rule_keeps_matched_and_unmatched_drop_removes_rest`
(`Subset([0])` -> actual `Subset([1])`) and
`attachment_drop_rule_covers_one_and_unmatched_keep_keeps_the_rest`
(`Subset([0, 1])` -> actual `Subset([1, 2])`) both FAILED.

**Fix:** updated both assertions to the 1-based ids (`Subset(vec![1])`,
`Subset(vec![1, 2])`), plus the fixture-describing comments in
`planner_resolution.rs` (`WITH_ATTACHMENTS` const doc, the two test's own
doc comments).

## Files changed

- `crates/muxsmith-core/tests/command.rs` (+2 golden tests, gap i)
- `crates/muxsmith-core/src/identify.rs` (+3 unit tests, gap ii)
- `crates/muxsmith-core/src/executor/job.rs` (1 test extended, gap iii)
- `crates/muxsmith-core/src/executor/queue.rs` (+1 test, gap iii)
- `crates/muxsmith-cli/tests/dry_run_cli.rs` (1 test extended, gap iii)
- `crates/muxsmith-core/tests/fixtures/identify/with-attachments.json`
  (gap iv)
- `crates/muxsmith-core/tests/planner_resolution.rs` (2 assertions fixed
  for gap iv's breakage)

Net: +9 tests (104 -> 108 lib tests in muxsmith-core; +2 command.rs, +1
queue unit test counted in the lib total; dry_run_cli.rs and job.rs tests
extended in place, not added as new functions).

## Gate results (all 8 parts, from worktree root)

1. `cargo fmt --all --check` - clean (one local diff auto-fixed via
   `cargo fmt --all` before the check, in `job.rs`'s new `assert!` call).
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean, no
   warnings.
3. `cargo test --workspace` - all green, zero failures across every crate
   (muxsmith-core, muxsmith-cli, muxsmith-gui/src-tauri, xtask), including
   doc-tests.
4. `cargo deny check` - exit 0, "advisories ok, bans ok, licenses ok,
   sources ok" (no dependency changes in this task).
5. `pnpm lint` - clean.
6. `pnpm build` - vue-tsc + vite build succeeded.
7. `pnpm check:i18n` - exit 0, "ok" (12 unused-key warnings are
   pre-existing gui-* catalog entries, unrelated to this task's scope).
8. `pnpm test:e2e` - 3/3 Playwright smoke tests passed.

All run in the foreground, sequentially, no background-and-poll.

## Self-review

- Every new/modified assertion went through a genuine RED observation
  (either a deliberately wrong value or, for gap iv, the fixture edit
  itself) before being set to the correct/final value - confirmed via
  actual `cargo test` failure output each time, not just code reading.
- No production code changed in any of gaps i-iii: all three closed pure
  coverage gaps in already-correct behavior. Gap iv's fix is the fixture
  data change itself, which is the task.
- Did not touch `catalog_completeness.rs` (Task 10's file, confirmed via
  `git diff --stat`) or anything outside the brief's five named
  locations.
- Files staged explicitly by name (no `git add -A`/`.`).
- Commit message and trailer match the brief and repo convention exactly.

## Concerns

- None blocking. One minor note: the accidental `sed` cross-contamination
  during gap-i test authoring (caught and fixed before commit, described
  above) is a reminder that identical string literals across nearby tests
  make blind `sed` substitution risky in this file; used targeted `Edit`
  calls with surrounding context for every other fix in this task.
- The `_comment` convention introduced in `with-attachments.json` is new
  to this fixture directory (no prior fixture used it); if a future
  fixture needs the same "note the wire-format source of truth" pattern,
  this establishes the precedent rather than a documented directory-wide
  convention. Left as-is per the brief's narrow scope (this one file);
  worth revisiting only if a second fixture needs the same treatment.
