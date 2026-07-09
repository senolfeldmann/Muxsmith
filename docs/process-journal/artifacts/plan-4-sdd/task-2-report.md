# Task 2 report: per-job runner (progress parse, exit mapping, delete-partial)

## What I implemented

`crates/muxsmith-core/src/executor/job.rs` (new), wired via `pub mod job;` in
`crates/muxsmith-core/src/executor/mod.rs`.

Public interface, matching the brief exactly:

- `JobSpec { argv: Vec<String>, output: PathBuf }`
- `JobState { Ok, Warning, Failed, Cancelled }` (`Debug, Clone, Copy, PartialEq, Eq, Serialize`, `snake_case`)
- `JobOutcome { state, exit_code: Option<i32>, warnings: Vec<String>, errors: Vec<String>, duration_ms: u64 }` (`Serialize`)
- `JobProgress { Percent(u8), WarningLine(String), ErrorLine(String) }`
- `run_job(spawner: &dyn Spawn, spec: &JobSpec, cancel: &AtomicBool, on_progress: &mut dyn FnMut(JobProgress)) -> JobOutcome`

`run_job` flow:

1. `std::fs::create_dir_all(spec.output.parent())` before spawning (best-effort,
   error ignored - a failing mux will surface its own I/O error on the line
   stream / exit code).
2. `spawner.spawn(&spec.argv)`. On `Err(SpawnError(message))` (not exercised
   by the six mandated tests - the brief only mandates FakeSpawner-driven
   success-path spawns): treated as `JobState::Failed`, `exit_code: None`,
   `errors: [message]`, going through the same `finish`/delete-partial path
   as every other failure. This was necessary because `run_job` returns
   `JobOutcome` unconditionally (no `Result`), so the `Err` arm had to
   resolve to *some* terminal state; `Failed` is the only one of the four
   that fits "mkvmerge did not produce output," and it reuses D13's own
   "abnormal exit is Failed" reading rather than inventing a fifth state.
3. Drains `next_line()` to EOF fully **before** calling `wait()` (binding
   concurrency constraint from Task 1's review), parsing each line:
   - `#GUI#progress NN%` -> `JobProgress::Percent(NN)` via `on_progress`.
   - `#GUI#warning <rest>` -> tag-stripped `<rest>` pushed to `warnings` and
     surfaced via `on_progress(WarningLine)`.
   - `#GUI#error <rest>` -> tag-stripped `<rest>` pushed to `errors` and
     surfaced via `on_progress(ErrorLine)`.
   - Anything else: silently ignored (no other grammar is documented).
4. `wait()` for the exit code, then maps: `Some(0)` -> `Ok`; `Some(1)` ->
   `Warning`; `None` while `cancel.load(Ordering::SeqCst)` -> `Cancelled`;
   everything else (`Some(2)`, any other code, or `None` without `cancel`
   set, i.e. an unexplained process death) -> `Failed`.
5. `finish()` deletes the partial output (`std::fs::remove_file`, error
   ignored - covers `NotFound` per the brief and any other error, since
   `JobOutcome` has no field to report a delete failure) only when the
   state is `Failed` or `Cancelled`, then assembles `JobOutcome` with
   `duration_ms` from an `Instant` taken at the top of `run_job`.

## What I tested and results

All six brief-mandated tests, in `job.rs`'s `#[cfg(test)] mod tests`, driven
through `FakeSpawner`:

- `exit_zero_is_ok_and_output_kept`
- `exit_one_is_warning_with_captured_lines`
- `exit_two_is_failed_and_partial_deleted`
- `killed_under_cancel_is_cancelled_and_partial_deleted`
- `progress_lines_surface_as_percent`
- `parent_dir_created_before_spawn`

Each uses a real `tempfile::tempdir()` so the file-existence assertions
(`spec.output.exists()`, parent-dir creation) exercise the real filesystem,
not a mock.

## TDD evidence

**RED** (module wired, `run_job` body replaced with `todo!()`, everything
else - types, tests - identical to the final version):

```
$ cargo test -p muxsmith-core executor::job::
running 6 tests
test executor::job::tests::parent_dir_created_before_spawn ... FAILED
test executor::job::tests::exit_one_is_warning_with_captured_lines ... FAILED
test executor::job::tests::progress_lines_surface_as_percent ... FAILED
test executor::job::tests::exit_two_is_failed_and_partial_deleted ... FAILED
test executor::job::tests::exit_zero_is_ok_and_output_kept ... FAILED
test executor::job::tests::killed_under_cancel_is_cancelled_and_partial_deleted ... FAILED

thread '...' panicked at crates/muxsmith-core/src/executor/job.rs:84:5:
not yet implemented: RED placeholder for TDD evidence

test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 77 filtered out
```

Expected: with `run_job` unimplemented, every test that calls it panics
immediately, before any assertion runs - confirming all six tests actually
exercise `run_job` and are not vacuously passing.

Before this stub run I also confirmed the true pre-task baseline (module
literally absent - `job.rs` moved aside, `mod.rs` reverted to `pub mod
spawn;` only) builds clean with zero job tests present, matching the brief's
"Step 2: RED - module absent."

**GREEN** (real implementation restored):

```
$ cargo test -p muxsmith-core --lib executor::job::
running 6 tests
test executor::job::tests::exit_one_is_warning_with_captured_lines ... ok
test executor::job::tests::exit_zero_is_ok_and_output_kept ... ok
test executor::job::tests::parent_dir_created_before_spawn ... ok
test executor::job::tests::progress_lines_surface_as_percent ... ok
test executor::job::tests::exit_two_is_failed_and_partial_deleted ... ok
test executor::job::tests::killed_under_cancel_is_cancelled_and_partial_deleted ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 77 filtered out
```

## Full gate (all four, run before the commit)

```
$ cargo test --workspace          # all green, 0 failed across every crate/test binary
$ cargo fmt --all --check         # clean after one `cargo fmt --all` pass (job.rs had
                                   # 4 line-wrap diffs from hand-formatting; not skipped)
$ cargo clippy --workspace --all-targets -- -D warnings   # clean, no warnings
$ cargo deny check                # advisories ok, bans ok, licenses ok, sources ok
```

## Files changed

- `crates/muxsmith-core/src/executor/job.rs` (new, 296 lines: types, `run_job`,
  two private helpers `parse_progress`/`delete_partial`/`finish`, 6 tests)
- `crates/muxsmith-core/src/executor/mod.rs` (+1 line: `pub mod job;`)

Commit: `3f33c10` "feat(executor): per-job runner with gui-mode parse and exit
mapping (D13)"

## Self-review findings

- **Completeness**: all six mandated tests present and each is meaningful
  (verified via the RED run above - every one fails against an
  unimplemented `run_job`, so none is vacuous). Every brief-mandated
  interface item (`JobSpec`, `JobState`, `JobOutcome`, `JobProgress`,
  `run_job`) matches the brief's signatures verbatim.
- **Quality**: every public item has a doc comment, including individual
  enum variants and struct fields (the brief's illustrative snippet omits
  some of these; `#![deny(missing_docs)]` requires them and the build would
  not have passed otherwise). Naming follows the brief and existing crate
  conventions (`planner.rs`/`report.rs` style: doc-commented fields,
  `Debug, Clone, PartialEq[, Eq], Serialize` derive order,
  `#[serde(rename_all = "snake_case")]`).
- **Discipline**: only touched the two files in scope
  (`executor/job.rs`, `executor/mod.rs`). Did not touch `spawn.rs`, did not
  "fix" the `read_line` `Err(_)`-as-EOF behavior there, did not add new
  dependencies (only `serde`, already a core dependency, and
  `tempfile`, already a core dev-dependency). Left the untracked
  `HANDOFF.md` in the working tree alone and out of the commit (outside
  task scope).
- **Testing**: every test drives `run_job` through the real `FakeSpawner`
  (no test mocks `run_job` itself), and the file-existence assertions hit a
  real `tempfile::tempdir()`, not an in-memory stand-in. Output is pristine
  - no `println!`/`dbg!`/commented-out code left in the file.
- **Concurrency constraint**: verified by inspection and by the code shape
  itself - `run_job`'s `while let Some(line) = running.next_line() { ... }`
  loop runs to completion (`None`) before `running.wait()` is ever called;
  there is no code path that calls `wait()` while lines might still be
  pending, satisfying Task 1's binding "drain to EOF, then wait" rule.

## Issues or concerns

- The `SpawnError` arm (mapping a failed `spawner.spawn()` call to
  `JobState::Failed`) is untested by the six mandated tests, since
  `FakeSpawner::spawn` never returns `Err`. I judged this the only
  reasonable reading given `run_job`'s `Result`-free signature (see "What I
  implemented" above), but it's a design call the brief didn't spell out
  and Task 3 (or Şenol) may want to confirm or override once the queue
  layer exists and it's clearer whether a queue-level retry/report path for
  spawn failures is wanted.
- Everything else in the brief was unambiguous; no other guessing was
  needed and the design memo's D13/D17 sections aligned with the brief
  throughout.

## Review fix: spawn failure must not delete a pre-existing output

**Finding (Important, from task review):** the `spawner.spawn()` Err arm
routed through `finish(JobState::Failed, ...)`, whose Failed gate runs
`delete_partial(&spec.output)`. When spawn fails, the process provably never
ran and mkvmerge wrote nothing - there is no partial. Under an
`on_collision: overwrite` plan, a file at `spec.output` is a valid output
from a prior run; a mere environment error (binary vanished, non-UTF-8 path)
silently destroyed it. D17's delete-partial covers partials only.

**Fix:** the Err arm now assembles its `JobOutcome` directly (state
`Failed`, `exit_code: None`, `warnings` empty, `errors: [message]`,
`duration_ms` measured) without touching the filesystem; `finish` keeps the
delete gate but is documented as the path for jobs whose process actually
ran. A comment on the Err arm records the invariant.

**TDD evidence:**

New test `spawn_failure_is_failed_but_keeps_preexisting_output`, using a
test-local `struct FailingSpawner` (implements `Spawn`, always returns
`Err(SpawnError("boom"))`; `FakeSpawner` has no error mode). It pre-creates
a file at `spec.output` and asserts state `Failed`, `exit_code None`,
`errors == ["boom"]`, and that the file still exists.

RED (against the reviewed commit's code, before the fix):

```
$ cargo test -p muxsmith-core --lib executor::job::tests::spawn_failure_is_failed_but_keeps_preexisting_output
test executor::job::tests::spawn_failure_is_failed_but_keeps_preexisting_output ... FAILED
panicked at crates/muxsmith-core/src/executor/job.rs:307:9:
no process ran, so no partial exists; a pre-existing output must survive
test result: FAILED. 0 passed; 1 failed
```

Expected: the buggy Err arm deletes the pre-existing file, so exactly the
file-exists assertion fires - confirming the test captures the reported bug.

GREEN (after the fix):

```
$ cargo test -p muxsmith-core --lib executor::job::
running 7 tests
... all 7 ok (6 original + spawn_failure_is_failed_but_keeps_preexisting_output)
test result: ok. 7 passed; 0 failed
```

**Gate (all four, after the fix):** `cargo test --workspace` all green;
`cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets --
-D warnings` clean; `cargo deny check` advisories/bans/licenses/sources ok.

**Commit:** `f394f61` "fix(executor): keep pre-existing output when spawn
fails (D17 scope)" - follow-up commit on top of `3f33c10`, not an amend.
