# Task 3 Report: FIFO queue, JobEvent stream, soft fail-fast, cancellation

Commit: `b9960c5` on master (base `f394f61`), message per brief, trailer included. Not pushed.

## What was implemented

`crates/muxsmith-core/src/executor/queue.rs` (new):

- `JobEvent` enum, serde `tag = "event", rename_all = "snake_case"`, variants exactly per brief (Started/Progress/Warning/Error/Finished), all variants and fields doc-commented.
- `QueueOpts { jobs, fail_fast }`.
- `run_queue(specs, spawner, opts, cancel, events) -> Vec<JobOutcome>` with the pinned shape:
  - `std::thread::scope`; `AtomicUsize` next-index; `AtomicBool` stop-dequeuing.
  - Killer registry `Mutex<Vec<Option<Killer>>>`, one slot per worker. Registration happens through a private `RegisteringSpawner` decorator around the caller's spawner: it registers `job.killer()` immediately after the inner `spawn` returns, i.e. strictly before `run_job` starts streaming. This is how the queue gets a kill handle without changing `run_job`'s signature.
  - Watcher thread polls `cancel` every 50ms; when set: flips stop, invokes all registered killers, exits. Addition beyond the brief's literal text: a `done` flag flipped after the workers join, so the watcher also exits on a natural (non-cancelled) finish; without it the scope never closes. Costs at most one 50ms poll of shutdown latency.
  - Workers: loop { stop/cancel check -> break; take index; emit Started; run_job with an on_progress closure forwarding Percent/WarningLine/ErrorLine as Progress/Warning/Error events; clear killer slot; on Failed && fail_fast -> set stop; emit Finished; store outcome by index }.
  - After the scope: every spec without an outcome becomes `Cancelled { exit_code: None, warnings: [], errors: [], duration_ms: 0 }`; no Started (and no Finished) event for these - they appear only in the returned vector (documented on `run_queue`).
  - All event `send` results discarded (receiver gone = caller stopped listening).
  - `opts.jobs` clamped via `.max(1)`.

`executor/mod.rs`: `pub mod queue;` added.

`executor/spawn.rs` (sanctioned fake extension only): `ConcurrencyTracker` (public, doc-commented) with `current`/`max` atomics; `FakeSpawner::with_concurrency_tracker(Arc<ConcurrencyTracker>)` increments in `spawn`, decrements in `wait`, `max()` exposes the high-water mark.

## How each concurrency test was made deterministic

- `jobs_n_bounds_concurrency`: a test-local `RendezvousSpawner` wraps `FakeSpawner`; every job's **first `next_line`** waits at a `Barrier::new(2)`. With exactly 2 workers (structural, from `opts.jobs`) and 4 specs (an exact multiple), every rendezvous round is full: overlap of exactly 2 is forced, not hoped for; overlap above 2 is structurally impossible (each worker has at most one in-flight job). Asserts `tracker.max() == 2` (equality, not `<=`). No sleeps.
- `cancel_kills_inflight_and_cancels_queued`: a test-local `GatedFakeSpawner` whose job's `next_line` first sends a readiness signal, then blocks on a condvar until its own `Killer` fires (mimicking a real child that only dies when killed). The test sets `cancel` only after the readiness signal. Ordering argument: readiness is sent from inside `next_line`, which runs strictly after `spawn` returned, which is strictly after the queue registered the killer - so the watcher's one-shot kill sweep can never run before the killer it must invoke exists. The only wait in the test is the watcher's own bounded 50ms poll. The `recv_timeout(10s)` on the readiness channel is a hang-to-failure converter for future regressions, not a race window (in a correct queue the signal always arrives).
- `outcomes_index_aligned` / `no_fail_fast_continues_past_failure`: a test-local `ScriptByIndexSpawner` keys each script off the spec's own index (encoded in `argv[0]` by the test's spec helper) instead of spawn-call order, which is non-deterministic under `jobs: 2`. Index alignment is additionally asserted by content: the scripted warning text must land in `outcomes[1].warnings` and the error text in `outcomes[3].errors`.

This mid-implementation determinism rework was forced by evidence: the first draft of the cancel test waited on the *Started event* before setting cancel, which leaves a window (Started is emitted before spawn/killer-registration) where the watcher's one-shot sweep can fire against an empty registry and the gated fake then blocks forever. The very first RED run deadlocked on exactly this (cargo test sat 16+ minutes with no rustc child). The ready-signal design closes the window by construction.

## TDD evidence

RED (with `run_queue` body stubbed to `todo!("Task 3 RED stub")`):

```
$ timeout 280 cargo test -p muxsmith-core --lib executor::queue
failures:
    executor::queue::tests::cancel_kills_inflight_and_cancels_queued
    executor::queue::tests::jobs_n_bounds_concurrency
    executor::queue::tests::no_fail_fast_continues_past_failure
    executor::queue::tests::outcomes_index_aligned
    executor::queue::tests::sequential_fifo_order_and_events
    executor::queue::tests::soft_fail_fast_cancels_queued_but_not_inflight
test result: FAILED. 0 passed; 6 failed; ... finished in 10.00s
```

Five tests panic directly on the todo; the cancel test fails via its readiness-timeout detector ("first job never reached its read loop: Timeout") - i.e. even a totally absent implementation fails cleanly instead of hanging. Expected: no implementation existed.

GREEN (stub removed, real implementation in place):

```
$ timeout 280 cargo test -p muxsmith-core --lib executor::
test result: ok. 15 passed; 0 failed; ... finished in 0.05s   (9 pre-existing + 6 new)
```

Repeated-run evidence (flakiness shakeout):

```
$ for i in $(seq 1 50); do cargo test -q -p muxsmith-core --lib executor::queue | grep "test result"; done | sort | uniq -c
     50 test result: ok. 6 passed; 0 failed; ... finished in 0.05s
```

50/50 identical passes, 0.05s each (no timing-dependent slack anywhere).

## Gate (all four, run before the commit)

- `cargo test --workspace`: all 24 test binaries ok, 0 failures.
- `cargo fmt --all --check`: clean (after applying `cargo fmt`; first check flagged import order and loop-block style, mechanically applied).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: advisories ok, bans ok, licenses ok, sources ok.

## Files changed

- `crates/muxsmith-core/src/executor/queue.rs` (new: implementation + 6 tests + 3 test-local fakes)
- `crates/muxsmith-core/src/executor/mod.rs` (wire `pub mod queue;`)
- `crates/muxsmith-core/src/executor/spawn.rs` (sanctioned: `ConcurrencyTracker` + `FakeSpawner::with_concurrency_tracker`)

## Self-review findings

- All six mandated tests present, named per the brief; the fail-fast test asserts exactly-one-Started AND exactly-one-spawn; index alignment asserted by content, not just by state.
- Interfaces match the brief verbatim; Task 8 can consume `run_queue`/`JobEvent`/`QueueOpts` as specified.
- `#![deny(missing_docs)]` satisfied (enforced by the compile); every public item including enum variants/fields documented.
- Inherited semantics untouched: spawn-failure -> Failed without deletion, delete-partial inside `run_job` only. The queue adds no file handling.
- Scope discipline: only the three sanctioned files; no changes to `job.rs`, no CLI work, no new dependencies.
- ASCII punctuation throughout; core emits no prose (events are data; all strings are pass-through mkvmerge text).

## Issues / concerns

- Known property of the pinned one-shot watcher (not a defect, inherent in the brief's shape): if a worker dequeues a spec in the instant between the kill sweep and its next loop-top stop check, that one job's process is spawned and never killed; it runs to natural completion, its outcome is computed on its own merits, and the worker then exits. Best-effort cancellation semantics, bounded to at most one job per worker. The CLI's exit-code fold (D15) is unaffected.
- The `done` flag on the watcher is a minimal addition beyond the brief's literal watcher description; without it `thread::scope` cannot close on a natural finish. Judged as the intended reading, not a deviation.
- No test asserts the serialized JSON shape of `JobEvent` (the brief mandates the derive, not a serde test); Task 8's renderer tests will exercise it.
