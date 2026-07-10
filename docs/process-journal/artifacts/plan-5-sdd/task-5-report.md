# Task 5 report: per-job cancel in core (D25) - QueueControl, index-keyed killers, skip-set

Base commit: `b020bb7`. Result commit: `5ea0929` "feat(core): per-job
cancellation via QueueControl (D25) + pre-spawn cancel check".

Note: an earlier, unrelated `.superpowers/sdd/task-5-report.md` (a Plan 4
"tests/support consolidation" report) was overwritten by this file - that
directory is untracked scratch space reused across plans/waves, not a git
history conflict; verified via `git log --follow` returning nothing for
either path before this run.

## What was implemented

Exactly the brief's interfaces, verbatim signatures:

```rust
pub struct QueueControl { /* batch: Arc<AtomicBool>, jobs: Vec<AtomicBool>, killers: Mutex<HashMap<usize, Killer>> */ }
impl QueueControl {
    pub fn new(spec_count: usize, batch: Arc<AtomicBool>) -> Arc<QueueControl>;
    pub fn cancel_all(&self);
    pub fn cancel_job(&self, index: usize);
    pub fn job_cancelled(&self, index: usize) -> bool;
}
pub fn run_queue(specs, spawner, opts, ctl: &Arc<QueueControl>, events) -> Vec<JobOutcome>
```

`run_job`'s `cancel: &AtomicBool` became `cancelled: &dyn Fn() -> bool`;
the exit-code match arm is now `None if cancelled() => JobState::Cancelled`.

### queue.rs

- `QueueControl`: `batch` (shared with the caller's own flag, e.g. the
  CLI's ctrlc `Arc`), `jobs: Vec<AtomicBool>` (one per spec index),
  `killers: Mutex<HashMap<usize, Killer>>` (re-keyed from the old
  worker-slot `Mutex<Vec<Option<Killer>>>`).
  - `cancel_all`: `batch.store(true)`.
  - `cancel_job(index)`: sets `jobs[index]` (via `.get()`, so an
    out-of-range index is a no-op, not a panic - defensive against a
    misbehaving GUI caller), then, if a killer is currently registered
    under `index`, invokes it synchronously (no watcher-poll delay).
  - `job_cancelled(index)`: `batch || jobs[index]` (`.get().is_some_and`,
    same out-of-range-safe pattern).
- `RegisteringSpawner` gained an `index: usize` field (was `slot: usize`
  into the old Vec); on spawn it `.insert(index, killer)`s into the map.
  The worker loop `.remove(&index)`s it once `run_job` returns.
- Worker loop: computes `ctl.job_cancelled(index)` right after dequeuing
  and, only if false, sends `Started`; it then always calls `run_job`
  with `&|| ctl.job_cancelled(index)` as the `cancelled` closure. A job
  already per-job-cancelled at dequeue time therefore never gets
  `Started`, but `run_job`'s own pre-spawn check (job.rs) still produces
  its `Cancelled` outcome and the loop still sends `Finished` - one
  mechanism serves both the queue-level "skip queued" behavior and
  job.rs's own unit-level "pre-spawn check" behavior, no duplicated
  cancellation logic between the two files.
- Watcher thread: unchanged in shape, now polls `ctl.batch` and iterates
  `ctl.killers.lock().unwrap().values()` (map values instead of
  `Vec<Option<_>>::iter().flatten()`).
- `run_queue`'s rustdoc documents the D25 deviation explicitly (per-job
  skip emits `Finished{Cancelled}` without `Started`, unlike batch
  cancel's silence for never-dequeued specs).

### job.rs

- `run_job` now checks `cancelled()` as its very first statement, before
  the parent-dir creation and before calling the spawner; on true it
  returns a `Cancelled` outcome built directly (bypassing `finish`/
  `delete_partial` entirely, mirroring the existing spawn-error
  construction), so a pre-spawn cancel touches the filesystem in exactly
  zero ways - no dir created, nothing deleted, a pre-existing output from
  an earlier run survives untouched.
- `delete_partial` signature changed to `(output: &Path, errors: &mut
  Vec<String>)`: `NotFound` stays silently ignored; any other
  `remove_file` error is pushed as `delete_partial_failed: <io error>`
  into `errors` (uses Rust 1.96's stable let-chains per clippy's own
  suggestion: `if let Err(e) = ... && e.kind() != NotFound`). `finish`
  takes `errors` by value and passes it in as `&mut` before moving it
  into the returned `JobOutcome`. This is the one deliberate exception to
  core's "no user-facing prose" rule (spec 6/7 third-party-detail
  passthrough), consistent with the existing exception already
  documented for the CLI-side i18n layer.

### run.rs (CLI)

- `let ctl = QueueControl::new(specs.len(), Arc::clone(&cancel));` right
  after `opts`, wrapping the CLI's own ctrlc `Arc<AtomicBool>` as the
  batch flag (same allocation, not a copy) - the ctrlc handler and the
  final `if cancel.load(...) { return 130; }` exit-code check both still
  read/write the original `cancel`, so SIGINT-driven batch cancel and the
  130 exit path are byte-for-byte unchanged. `run_queue` is now called
  with `&queue_ctl` (an `Arc::clone(&ctl)` moved into the scoped thread,
  the same pattern the old `queue_cancel` clone used).

## TDD evidence

This is a signature-changing refactor across three files, so true
per-behavior RED (a passing compile, failing assertion) isn't reachable
in isolation - the whole crate stops compiling the moment `run_job`'s
signature changes until every call site is updated together. RED is
therefore the natural Rust equivalent: a compile error citing the new
contract, captured before the other files were touched.

**RED** (job.rs already changed to `cancelled: &dyn Fn() -> bool`; queue.rs
still calling it with the old `cancel: &Arc<AtomicBool>`):

```
error[E0277]: expected a `Fn()` closure, found `Arc<Atomic<bool>>`
   --> crates/muxsmith-core/src/executor/queue.rs:167:67
    |
167 |                         let outcome = run_job(&registering, spec, cancel, &mut on_progress);
    |                                                                   ^^^^^^ expected an `Fn()` closure, found `Arc<Atomic<bool>>`
```

**GREEN**, one behavior at a time, all in the final `cargo test --workspace`
run (283 passed, 0 failed):

1. **Skip queued** -
   `executor::queue::tests::cancel_job_before_dequeue_skips_start_but_still_reports_finished`
   (queue.rs): `cancel_job(2)` before `run_queue` even starts; asserts
   `outcomes[2] == Cancelled`, `Started` events are exactly `[0, 1]`
   (never 2), and a `Finished{index:2, Cancelled}` event is present.
2. **Kill in-flight** -
   `executor::queue::tests::cancel_job_kills_exactly_that_job_others_continue`
   (queue.rs, `jobs: 2`): a dedicated `SelectiveGateSpawner` gates only
   spec index 0 (reused `Gate`/`GatedJob` from the existing batch-cancel
   test) while indices 1/2 spawn quick scripted successes; the test
   synchronizes on index 0 reaching its read loop, then calls
   `ctl.cancel_job(0)`. Asserts `outcomes[0] == Cancelled`,
   `outcomes[1]/[2] == Ok`, the killed job's pre-written partial file is
   gone (D17), and `kills == 1` (exactly the targeted job, not the
   others).
3. **Pre-spawn check** -
   `executor::job::tests::pre_spawn_cancellation_skips_spawn_and_deletes_nothing`
   (job.rs): `cancelled = || true`; asserts `Cancelled`, `exit_code:
   None`, `fake.spawned()` empty, and a pre-existing output file at the
   spec's path still exists (nothing deleted).
4. **delete_partial surfacing** -
   `executor::job::tests::delete_partial_failure_surfaces_into_errors`
   (job.rs): output path is a pre-created directory (portable, no-perms
   way to make `remove_file` fail with a non-`NotFound` error on every
   OS), exit code 2 (`Failed`); asserts `outcome.errors` contains an
   entry starting with `delete_partial_failed: `.
5. **Batch semantics unchanged** - every pre-existing queue/job/CLI test
   updated to wrap the bare `Arc<AtomicBool>` in `QueueControl::new` (or a
   `cancelled` closure for job.rs's direct `run_job` calls) and rerun
   green, in particular
   `executor::queue::tests::cancel_kills_inflight_and_cancels_queued` (now
   drives cancellation via `ctl.cancel_all()` instead of
   `cancel.store(true, ...)`) and
   `executor::queue::tests::soft_fail_fast_cancels_queued_but_not_inflight`,
   plus the full `run_cli.rs`/`run_live.rs` CLI suites (exit-code and
   `--json` contract tests) unmodified and passing.

One regression caught and fixed mid-implementation: the pre-existing
`killed_under_cancel_is_cancelled_and_partial_deleted` test used a
constant `cancel = AtomicBool::new(true)` to simulate "the process was
killed while already cancelled" - but that same constant-true flag now
also trips the *new* pre-spawn check, so the process never spawned and
the pre-written partial survived, failing the test's `!exists()`
assertion. Fixed by replacing the constant with a `Cell<bool>`-backed
closure (`cancelled_after_spawn.replace(true)`) that reads `false` on its
first call (the pre-spawn check, letting the job actually run) and `true`
from then on (the post-`wait()` check), correctly modeling the real
timeline of a job cancelled only after it started.

## Full gate (final run, all green)

- `cargo build --workspace`: clean (includes `src-tauri`/`muxsmith-gui`,
  no changes needed there).
- `cargo test --workspace`: **283 passed, 0 failed, 0 ignored** across 31
  test binaries (up from the 269 recorded in HANDOFF.md before this task
  - net +14, expected: this task added 4 new tests directly, and Wave 1's
  merged T1-T3 accounted for the rest already on `master`).
- `cargo fmt --all --check`: clean (one `cargo fmt --all` pass was needed
  first, purely mechanical line-wrapping of a multi-call chain and one
  `assert_eq!` call, no semantic change).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean. One
  clippy-driven fix during development: `delete_partial`'s nested
  `if let Err(e) = ... { if e.kind() != NotFound { ... } }` triggered
  `collapsible_if`; collapsed into a let-chain
  (`if let Err(e) = ... && e.kind() != NotFound`) per clippy's own
  suggested rewrite, confirming stable let-chains are fine on this
  toolchain (rustc/cargo 1.96.1, edition 2024).
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`
  (exit 0; no new dependencies were added by this task).
- pnpm/frontend gates: not run - core-only change, no `src/`, `package.json`,
  or `src-tauri` frontend-facing files touched; unaffected per the task's
  own instructions.

All four gate commands run in the foreground per the standing FOREGROUND
constraint (no `run_in_background`, no backgrounded `cargo`).

## Files changed

- `crates/muxsmith-core/src/executor/queue.rs` (+/-, `QueueControl`,
  re-keyed registry, worker/watcher wiring, 2 new tests, existing tests
  updated)
- `crates/muxsmith-core/src/executor/job.rs` (`run_job` signature +
  pre-spawn check, `delete_partial` error surfacing, 2 new tests,
  existing tests updated, one test logic fix)
- `crates/muxsmith-core/tests/executor_events.rs` (`run_job` call site
  updated to the closure signature)
- `crates/muxsmith-cli/src/commands/run.rs` (constructs `QueueControl`
  wrapping the existing ctrlc `Arc<AtomicBool>`)

## Self-review findings

- **Interface fidelity:** every produced signature (`QueueControl::new`,
  `cancel_all`, `cancel_job`, `job_cancelled`, `run_queue`'s `ctl`
  parameter, `run_job`'s `cancelled` parameter, the exact `None if
  cancelled() => JobState::Cancelled` match arm) matches the brief
  verbatim, checked by direct comparison against `task-5-brief.md`'s
  code block, not from memory.
- **D17 boundary respected:** delete-partial still fires only for a
  process that actually ran and ended `Failed`/`Cancelled`
  (`finish`/`delete_partial`, unchanged trigger condition); the new
  pre-spawn-cancel path and the pre-existing spawn-error path both bypass
  `finish` entirely and delete nothing, verified by dedicated tests for
  both (`pre_spawn_cancellation_skips_spawn_and_deletes_nothing`,
  pre-existing `spawn_failure_is_failed_but_keeps_preexisting_output`).
- **No duplicated cancellation logic:** the queue-level "skip queued"
  behavior (test 1) and the job-level "pre-spawn check" behavior (test 3)
  are the same underlying mechanism (`run_job`'s own `cancelled()` check)
  observed from two altitudes - the queue only additionally gates the
  `Started` event, it does not re-implement the skip/cancel decision.
- **CLI behavior genuinely unchanged, not just re-tested:** confirmed by
  reading the diff - `cancel` (the `Arc<AtomicBool>` the ctrlc handler
  flips and the final exit-code check reads) is untouched; `QueueControl`
  wraps a *clone of the same `Arc`*, not a new flag, so there is no
  possible drift between "what SIGINT sets" and "what the queue's batch
  flag sees." `run_cli.rs`/`run_live.rs` pass unmodified.
- **Killer idempotency relied on, not re-verified:** `cancel_job` invokes
  a registered killer without removing it from the map (the worker
  removes it after `run_job` returns); this relies on `Killer`'s existing
  documented idempotent/best-effort contract (spawn.rs) rather than
  adding new synchronization. No new test targets a double-invocation
  race specifically (e.g. `cancel_job` called twice, or `cancel_job` and
  the batch watcher both firing on the same index) - existing `Killer`
  implementations (`FakeJob`, `LiveJob`) are idempotent by construction
  (`AtomicBool` kill flags), so this is a documented reliance, not an
  untested gap in new logic.
- **Out-of-range `index` defensiveness:** `cancel_job`/`job_cancelled` use
  `.get()` rather than direct indexing, so a GUI passing a stale or bad
  index cannot panic core. Not explicitly required by the brief's 5 test
  behaviors; added because Task 8 (GUI IPC) will call `cancel_job` with a
  caller-supplied `usize` and a panic there would be a bad failure mode
  for a one-line guard. Not covered by a dedicated test (the behavior is
  a straightforward `Option::get` no-op, judged not to need one).

## Concerns

None blocking. Two small forward notes for whoever picks up Task 6/T8:

- `QueueControl` is `Arc`-returned from `new` specifically so Task 8 can
  hold onto it across the run lifecycle (start/cancel_all/cancel_job from
  separate Tauri command invocations) without the CLI's scoped-thread
  pattern; this task's own CLI usage only needed a local clone, so this
  is exercised by the type signature and the existing `Arc::clone`
  pattern in the tests, not by a cross-invocation integration test here
  (out of this task's scope - T8 owns that).
- The brief's HANDOFF backlog explicitly named both `delete_partial`
  error surfacing and the pre-spawn cancel check as items this task
  should close; both are closed and covered by the test suite added
  here, so no residual backlog item remains for either.

---

# Review-fix addendum: lost-cancellation race + pre-spawn filesystem pin

Review verdict on `5ea0929`: one Important (concurrency gap), one Minor.
Both fixed in commit `e06bda0` "fix(core): close D25 lost-cancellation
race in the mid-spawn window".

## Important: lost-cancellation race (fixed)

**The gap.** `cancel_job(index)` firing in the window between `run_job`'s
pre-spawn `cancelled()` check and `RegisteringSpawner`'s killer insert
found an empty registry: the per-job flag was set but no killer existed
to invoke, and since a normally-exiting process yields `Some(code)` from
`wait()`, the `None if cancelled() => Cancelled` arm never fires - the
cancel request was silently dropped and the job completed
`Ok`/`Warning`/`Failed`.

**The fix** (queue.rs, `RegisteringSpawner::spawn`): after inserting the
killer into the registry, re-check `ctl.job_cancelled(self.index)`; if
set, invoke the just-created killer directly (a local clone, no second
lock acquisition). The ordering closes the window from both sides: a
`cancel_job` earlier than the re-check is observed via the flag; one
later than it finds the killer already in the map and kills through
that path. Both paths firing (a `cancel_job` racing the re-check
itself) is a harmless double invocation - `Killer` is idempotent and
best-effort by its documented contract (spawn.rs), so no new
synchronization was added. `RegisteringSpawner`'s `killers` field became
`ctl: &'a QueueControl` to give it access to `job_cancelled`.

**TDD, RED first.** New test
`executor::queue::tests::cancel_job_during_spawn_window_is_not_lost`,
written and run against the unfixed code before touching the
implementation. A `MidSpawnGateSpawner` wrapper parks inside `spawn()` -
provably after the pre-spawn check (spawn is only reached once it
returned false) and provably before the killer insert (insertion happens
only after spawn returns) - signals the test over a `ready` channel,
and blocks on a `release` channel until the test has called
`cancel_job(0)` inside the window. Deterministic: no sleeps, no timing
bets. Asserts the outcome is `Cancelled` and the pre-written partial is
deleted (D17).

RED (`cargo test -p muxsmith-core --lib cancel_job_during_spawn_window_is_not_lost`
against `5ea0929`'s code):

```
thread '...cancel_job_during_spawn_window_is_not_lost' panicked at crates/muxsmith-core/src/executor/queue.rs:969:9:
assertion `left == right` failed: a cancel_job landing mid-spawn must not be silently dropped
  left: Ok
 right: Cancelled
test result: FAILED. 0 passed; 1 failed
```

Exactly the reviewer's predicted failure shape: the job completed `Ok`.

GREEN (same command, after the fix):

```
test executor::queue::tests::cancel_job_during_spawn_window_is_not_lost ... ok
test result: ok. 1 passed; 0 failed
```

## Minor: pre-spawn zero-filesystem-touch pinned directly (fixed)

`executor::job::tests::pre_spawn_cancellation_skips_spawn_and_deletes_nothing`
now additionally runs a second pre-spawn-cancelled job whose output is
nested (`never/created/out.mkv`) under a parent that does not exist, and
asserts `!output.parent().exists()` afterward - pinning that the
`create_dir_all` sits behind the cancel check, not just that nothing was
deleted. (A separate spec is needed because the first spec's pre-written
output file requires an existing parent; the `fake.spawned().is_empty()`
assertion now covers both calls.)

## Contract verification (all FOREGROUND)

- Covering tests, all green in one `cargo test -p muxsmith-core --lib
  'executor::'` run (25 passed, 0 failed):
  `cancel_job_during_spawn_window_is_not_lost` (new),
  `cancel_job_kills_exactly_that_job_others_continue`,
  `cancel_job_before_dequeue_skips_start_but_still_reports_finished`,
  plus every other executor queue/job/spawn test.
- `cargo test --workspace`: **284 passed, 0 failed** across 31 binaries
  (283 + the 1 new race test; the Minor folded into an existing test).
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: exit 0, `advisories ok, bans ok, licenses ok,
  sources ok`.

## Files changed (addendum commit)

- `crates/muxsmith-core/src/executor/queue.rs` (RegisteringSpawner
  re-check fix + `MidSpawnGateSpawner` + race test)
- `crates/muxsmith-core/src/executor/job.rs` (pre-spawn test extension)

## Residual concern

None on the fixed window itself. One deliberate scope note: the
symmetric-looking moment on the *removal* side (`cancel_job` firing
after the worker's `.remove(&index)` but before/after `Finished` is
emitted) is not a gap - by then `run_job` has returned, the process has
already exited, and there is nothing left to kill; the flag-set is a
no-op on a finished job, which is correct semantics (cancel of an
already-finished job does nothing), not a dropped request.
