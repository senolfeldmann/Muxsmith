# Task 1 report: Raw output events (D24) + JobEvent serde golden test

(Note: this path previously held a Plan 4 Task 1 report - "Executor spawn
seam" - stale content from a prior plan's task numbering; overwritten with
this plan's Task 1.)

## Status: DONE

## What was implemented

- `crates/muxsmith-core/src/executor/job.rs`: `JobProgress` gains
  `OutputLine(String)`. In `run_job`'s line loop, every line that is not a
  `#GUI#progress` tick now emits `OutputLine(line.clone())` before the
  existing tag-stripped handling, so tagged warning/error lines emit both
  `OutputLine` (verbatim, tags included) and their existing `WarningLine`/
  `ErrorLine` (tag stripped); plain untagged lines emit only `OutputLine`.
- `crates/muxsmith-core/src/executor/queue.rs`: `JobEvent` gains
  `Output { index: usize, line: String }`. The worker's `on_progress`
  closure maps `JobProgress::OutputLine(line) => JobEvent::Output { index,
  line }`, alongside the existing three mappings.
- `crates/muxsmith-cli/src/commands/run.rs`: `MilestoneState::render`'s
  match over `JobEvent` was exhaustive (no wildcard); added a minimal
  no-op arm `JobEvent::Output { .. } => Vec::new()` with a one-line
  comment. No other change to that file.
- `crates/muxsmith-core/tests/executor_events.rs` (new): golden test for
  all six `JobEvent` variants' exact `serde_json::to_string` output, plus
  a behavior test for `JobProgress::OutputLine`/`WarningLine`/`Percent`
  interaction.

## TDD evidence

**RED** - `cargo test -p muxsmith-core --test executor_events` before the
variants existed:

```
error[E0599]: no variant named `Output` found for enum `JobEvent`
  --> crates/muxsmith-core/tests/executor_events.rs:70:24
error[E0599]: no variant, associated function, or constant named `OutputLine` found for enum `JobProgress`
   --> crates/muxsmith-core/tests/executor_events.rs:108:26
error[E0599]: no variant, associated function, or constant named `OutputLine` found for enum `JobProgress`
   --> crates/muxsmith-core/tests/executor_events.rs:109:26
error: could not compile `muxsmith-core` (test "executor_events") due to 3 previous errors
```

(An earlier attempt hit a self-inflicted raw-string-delimiter bug, not the
intended RED: `r#"..."#GUI#..."#"#` self-terminates on the embedded `"#`
inside the literal. Fixed by switching that one literal to a
`r##"..."##` delimiter, then re-ran to get the real RED above.)

**GREEN** - after implementing the variants, the line-loop emission, and
the queue mapping:

```
$ cargo test -p muxsmith-core --test executor_events
running 2 tests
test job_event_wire_shapes ... ok
test output_line_captures_every_non_tick_line_verbatim ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo build --workspace` then caught the CLI's non-exhaustive match on
`JobEvent` (expected, per brief Step 4):

```
error[E0004]: non-exhaustive patterns: `&JobEvent::Output { .. }` not covered
   --> crates/muxsmith-cli/src/commands/run.rs:354:15
```

fixed with the no-op arm; rebuild clean.

## Full gate (foreground, run after the complete diff was in place)

- `cargo test --workspace`: all green, 0 failed (muxsmith-core lib 95
  passed, `executor_events` 2 passed, muxsmith-cli lib 21 passed,
  `run_cli`/`run_live`/`dry_run_cli` unaffected and green, 187 total tests
  across the workspace, 0 failed).
- `cargo fmt --all --check`: clean, no output.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/src/executor/job.rs`
- `crates/muxsmith-core/src/executor/queue.rs`
- `crates/muxsmith-cli/src/commands/run.rs`
- `crates/muxsmith-core/tests/executor_events.rs` (new)

Commit: `d6725c8 feat(core): raw output-line job events (D24) + JobEvent
wire golden test`.

## Self-review

- **Completeness against brief:** all 5 steps done as specified
  (variant/field names, wire shape, emission-order semantics, minimal CLI
  arm, commit message verbatim).
- **Quality:** doc comments added on every new public item
  (`JobProgress::OutputLine`, `JobEvent::Output` + its two fields) per
  `#![deny(missing_docs)]`; ASCII-only checked via `grep -P '[^\x00-\x7F]'`
  over the diff and the new test file, no hits.
- **YAGNI:** did not add a queue-level (`run_queue`) content test for the
  `JobProgress::OutputLine -> JobEvent::Output` mapping beyond what
  compiles and the golden test's structural coverage of `JobEvent::Output`
  itself. This mirrors the existing, already-established coverage depth
  for the sibling `Progress`/`Warning`/`Error` mappings in `queue.rs`,
  which are also not independently content-asserted at the event level
  (only via `JobOutcome.warnings`/`errors` and `Started`/`Finished` index
  lists) - not a gap this task introduces, just the pattern already in the
  file. Flagging it here rather than silently deciding to expand scope.
- **Test rigor:** golden test asserts exact JSON for all 6 variants
  including field order (tag first, then declaration-order fields - a
  serde internally-tagged-enum property, verified empirically against the
  compiled output, not assumed); behavior test asserts the exact ordered
  sequence of `JobProgress` values for a tick + plain line + tagged-warning
  script, covering both "verbatim on tagged lines" and "no `OutputLine`
  for the tick."
- **Pristine test output:** full-workspace run has 0 failures, 0 ignored,
  no stray `eprintln!`/dbg! noise in the new test file.
- **Scope note:** left `docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md`
  untouched (its Task 1 checkboxes still read `- [ ]`). The T0-done commit
  history (`c822a17`) shows Şenol/the controller ticks these post-merge
  with a `(DONE date, commit sha)` annotation from outside the per-task
  worktree; editing the shared plan file from within a task-scoped
  worktree risks conflicting with parallel task worktrees editing the same
  file. Left for the controller to reconcile at merge time.

## Concerns

None blocking. The YAGNI/scope notes above are judgment calls surfaced
for visibility, not blockers.
