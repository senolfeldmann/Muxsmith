# Task 4 report: worker-panic handling + mutex-poison hygiene (#10)

Commit: `a4ab647` on `plan55-stream-a`
(`fix(executor): report worker panics as failures, recover poisoned mutexes (#10)`)

## Summary

`run_queue`'s final join loop discarded `handle.join()`'s result (`let _ =
handle.join();`). If a worker thread panicked mid-`run_job` (a bug in this
crate, e.g. a bad downstream change - never an mkvmerge failure, which is
`run_job`'s own `Failed` return, not a panic), that job's slot in the
`outcomes: Mutex<Vec<Option<JobOutcome>>>` was never written, so the final
`outcome.unwrap_or(Cancelled)` fallback mislabeled a panicked job
`Cancelled` - indistinguishable from a spec that was never dequeued at all.
Separately, every `killers`/`outcomes` lock site used a bare `.unwrap()`, so
a poisoned mutex (from any future panic while holding one) would cascade
into every other worker's, the watcher's, and the final collection's own
panic - i.e. one job's bug could crash the whole batch.

Fixed both:

1. **Worker-panic reporting.** `handle.join()`'s `Err` is now captured. A
   new per-worker `current_index: Vec<AtomicUsize>` (one sentinel-valued
   slot per worker) records which spec index a worker last dequeued,
   cleared only once that job's outcome is durably written - so it survives
   the unwind and identifies exactly which job died with a panicked worker.
   `recover_panicked_worker` uses it to record the job `Failed` with a new
   `DiagCode::WorkerPanicked` ("worker-panicked") pushed into
   `outcome.errors`, log the panic payload (downcast to `&str`/`String`) to
   stderr for developer triage, and still emit `JobEvent::Finished` (so the
   persisted per-job log and any live listener see it too, matching every
   other terminal-outcome path in this module).
2. **Poison recovery.** Every `killers`/`outcomes` lock site (5 in
   `queue.rs`) and every `AppState.active` lock site (8 in `run.rs`) now
   uses `unwrap_or_else(|poisoned| poisoned.into_inner())` instead of
   `.unwrap()`, centralized behind two new getter functions
   (`QueueControl::killers()`, `lock_outcomes()`) and one (`lock_active()`
   in `run.rs`) rather than duplicating the reasoning at every call site.

## Files changed

- `crates/muxsmith-core/src/executor/queue.rs`: the fix (see below) plus
  two new tests.
- `crates/muxsmith-core/src/report/mod.rs`: new `DiagCode::WorkerPanicked
  => "worker-panicked"` catalog entry.
- `locales/en/diagnostics.ftl`: EN-only Fluent message for the new code
  (per cross-task constraint C2).
- `src-tauri/src/run.rs`: `lock_active()` helper + 8 call-site conversions
  (Step 3).

## Poison-recovery soundness argument (as written into the code)

From `QueueControl::killers()`'s doc comment (the fullest of the three,
`lock_outcomes`/`lock_active` restate the same reasoning for their own
lock):

> Recovery is sound because every write this lock guards is a single
> `HashMap` `insert`/`remove` [or, for `outcomes`/`active`: a single slot
> assignment] - never a multi-step invariant a panic could leave
> half-applied - so whatever the map held right before a panic is exactly
> what it holds right after: still a valid killer registry.

I.e.: state written *before* any panic point is a completed write, not a
half-applied one, because none of these critical sections do more than one
atomic-looking mutation before releasing the lock. The panicked worker's
own slot is handled explicitly (`recover_panicked_worker` writes it before
the final `into_inner()`), so recovering the *other* slots via poison
recovery never has to paper over missing data - it only has to not
propagate a second, unrelated panic. Refusing to recover would be strictly
worse: one worker's bug would make every other (perfectly healthy) worker's
attempt to record its own outcome panic too, and in `run.rs`, would wedge
the whole app (every future `start_run`/`cancel_run`/`cancel_job`/close
dialog) over one earlier bug.

The `poisoned_mutex_recovers_state_written_before_the_panic` test (below)
pins this claim directly against `std::sync::Mutex`, independent of
`run_queue`'s specific scenario.

## TDD evidence

### RED (original `queue.rs`, new tests added)

Reverted `queue.rs` to HEAD (`git checkout`), re-added only the import and
the two new tests (not the fix), ran:

```
cargo test -p muxsmith-core --lib executor::queue -- --test-threads=4
```

```
test executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled ... FAILED

---- executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled stdout ----

thread '<unnamed>' (702656) panicked at crates/muxsmith-core/src/executor/queue.rs:604:17:
scripted worker panic for job 0

thread 'executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled' (702652) panicked at crates/muxsmith-core/src/executor/queue.rs:645:9:
assertion `left == right` failed: a panicked worker must be reported Failed, not silently Cancelled
  left: Cancelled
 right: Failed

test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 94 filtered out; finished in 0.15s
```

Exactly the bug: `Cancelled` instead of `Failed`. (The independent
`poisoned_mutex_recovers_state_written_before_the_panic` test passed even
here, as expected - it tests `std::sync::Mutex` semantics directly, not
`run_queue`, so it isn't a symptom of this bug; it's separate regression
coverage for the recovery pattern itself.)

### GREEN (fix restored)

```
cargo test -p muxsmith-core --lib executor::queue -- --test-threads=4
```

```
running 13 tests
test executor::queue::tests::cancel_job_before_dequeue_skips_start_but_still_reports_finished ... ok
test executor::queue::tests::cancel_job_during_spawn_window_is_not_lost ... ok
test executor::queue::tests::cancel_job_kills_exactly_that_job_others_continue ... ok
test executor::queue::tests::cancel_kills_inflight_and_cancels_queued ... ok
test executor::queue::tests::jobs_far_exceeding_spec_count_still_completes_with_correct_outcomes ... ok
test executor::queue::tests::poisoned_mutex_recovers_state_written_before_the_panic ... ok
test executor::queue::tests::jobs_n_bounds_concurrency ... ok
test executor::queue::tests::no_fail_fast_continues_past_failure ... ok
test executor::queue::tests::outcomes_index_aligned ... ok
test executor::queue::tests::worker_count_is_capped_at_spec_count ... ok
test executor::queue::tests::soft_fail_fast_cancels_queued_but_not_inflight ... ok
test executor::queue::tests::sequential_fifo_order_and_events ... ok
test executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 94 filtered out; finished in 0.15s
```

`worker_panic_is_reported_as_failed_not_cancelled` uses `jobs: 2` with 3
specs so a second, healthy worker is provably still alive to finish what
the dead one would have (index-keyed scripting, so this is deterministic
regardless of which physical thread grabs which index - verified this by
reasoning through `next.fetch_add`'s monotonic-claim semantics: every index
0..3 is claimed by *some* live worker no matter the interleaving, since
only the worker that draws the panicking index ever dies). Asserts: index 0
is `Failed` with an `errors` entry starting with
`DiagCode::WorkerPanicked.key()`; indices 1 and 2 are `Ok`; and the test
function itself completing without aborting is the proof `run_queue` no
longer propagates the panic.

## Gate results

All from the worktree root, foreground, after the full implementation:

- `cargo fmt --all --check` - one diff found (my hand-formatted
  `unwrap_or_else` lines exceeded line width); ran `cargo fmt --all`, `--check` clean afterward.
- `cargo clippy --workspace --all-targets -- -D warnings` - clean.
- `cargo test --workspace` - all green (muxsmith-core 107 lib tests incl.
  the 2 new ones, muxsmith-cli 19 + integration suites incl.
  `catalog_completeness` 1/1, muxsmith-gui 78 incl. all `run::tests::*`
  unaffected by the `lock_active` refactor); zero failures.
- `cargo deny check` - exit 0, `advisories ok, bans ok, licenses ok, sources ok`.
- `pnpm lint` - clean.
- `pnpm build` - `vue-tsc --noEmit && vite build` succeeded.
- `pnpm check:i18n` - `ok (16 source files scanned, 172 catalog ids, 12
  unused warning(s))`; the 12 warnings are pre-existing and unrelated
  (`diagnostics.ftl`'s `worker-panicked` isn't in the gui-* scan scope this
  script covers).
- `pnpm test:e2e` - 3/3 Playwright smoke tests passed.

`node_modules` already existed in this worktree; no `pnpm install` needed.

## Self-review

- **Completeness**: all 4 brief steps done - failing test, implementation
  (panic capture + `WorkerPanicked` DiagCode + poison recovery on
  killers/outcomes), Step 3 (`AppState.active`, all 8 production call
  sites converted via `grep -rn "active" src-tauri/src/ | grep -i
  "lock\|mutex"`), full gate, commit with the exact specified message +
  trailer.
- **Scope discipline on the poison-recovery sites**: the brief's audit
  anchors were `queue.rs:270,276` (the join loop and final `into_inner()`),
  but Step 2's instruction text said "on the killers/outcomes mutexes"
  (plural, both mutexes broadly). I converted every production lock site
  touching either mutex (5 total: `cancel_job`, the watcher, the worker
  loop's `killers().remove`/`outcomes` write, `RegisteringSpawner::spawn`'s
  insert, and the final `into_inner()`), not just the two anchors -
  reasoned this is "owning file-level correctness" rather than scope creep,
  since a poisoned `killers` mutex reachable from `cancel_job` (a public
  API a caller can invoke at any time, not just from inside the worker
  loop) would otherwise still crash the app. Test-only `.lock().unwrap()`
  sites (in `queue.rs`'s and `run.rs`'s `#[cfg(test)]` modules, on
  test-local synchronization primitives or `state.active` in test
  assertions) were deliberately left untouched: a poisoned lock during a
  test is a real bug that should fail loudly, not be silently recovered.
- **Emitting `JobEvent::Finished` for the recovered panic**: not explicitly
  asked by the brief, but I judged it necessary for internal consistency -
  without it, the panicked job's `job-<index>.json` (written only on
  `Finished`, per `joblog.rs`) would never exist even though `summary.json`
  reports it `Failed`, and any live listener (GUI job row, CLI transcript)
  would never see the row resolve. This mirrors the module's own documented
  precedent (D25's per-job-cancel-before-dequeue deviation gets the same
  "still emits Finished for GUI confirmation" treatment) rather than adding
  a new, undocumented silent case.
- **`errors` string format**: `JobOutcome.errors` is `Vec<String>` with no
  structured params map (unlike `report::Diagnostic`). I pushed
  `"worker-panicked: job {index}"` - the stable code as a `starts_with`-
  greppable prefix (mirroring `job.rs`'s existing `delete_partial_failed:
  <detail>` convention) plus the index, satisfying the brief's literal
  "(params: job index)" even though there's no separate structured field to
  put it in.
- **`eprintln!` in core for the panic log line**: `muxsmith-core` has no
  logging crate dependency and no existing `eprintln!` precedent (core is
  otherwise silent; the CLI/GUI layers already use `eprintln!` freely for
  developer-facing text). Given the brief explicitly asks for "a log line"
  and there's no logging facade to plug into, I used `eprintln!` directly,
  documented as core's "one deliberate prose-free exception" mirroring
  `job.rs`'s own documented `delete_partial_failed` exception (same
  category: raw, unstructured, non-templatable third-party/developer text,
  never a `DiagCode`-templated user message). Flagging this as the one
  genuinely judgment-call decision in this task, in case project convention
  wants a different sink.
- **Rustdoc quality**: every new item (public and private) carries a
  contract/rationale doc comment, not a name echo, per BUILDING.md's
  standard; `#![deny(missing_docs)]` (verified active on
  `muxsmith-core::lib`) only gates `pub` items but I held every new item to
  the same bar.
- **Pristine output**: `git status --short` after the commit is clean; only
  the 4 intended files are in the commit (staged explicitly by path, no
  `-A`/`.`).

## Concerns

- None blocking. The `eprintln!`-for-panic-logging choice (noted above) is
  the one design decision worth a second look if the project later adds a
  real logging facade - trivial to swap then.
- `duration_ms: 0` for a worker-panicked outcome is a deliberate, documented
  placeholder (the worker died before it could measure its own elapsed
  time); no test asserts a specific value here, and nothing downstream
  currently treats a `Failed` outcome's `duration_ms` as load-bearing.
