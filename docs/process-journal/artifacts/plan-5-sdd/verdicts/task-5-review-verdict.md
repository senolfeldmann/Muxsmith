<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-5  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01B4r22pX7ydnvYmehrv2y7W
  agent_id:           ae090b7e57668bd9b
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-ae090b7e57668bd9b.jsonl
  dispatch_desc:      Review Task 5 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T12:51:13.906Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

- ✅ **Interfaces match the brief verbatim.** `QueueControl::new/cancel_all/cancel_job/job_cancelled`, `run_queue(..., ctl: &Arc<QueueControl>, ...)`, `run_job`'s `cancelled: &dyn Fn() -> bool`, and the `None if cancelled() => JobState::Cancelled` arm all match the brief's code block exactly (queue.rs:92-146, job.rs:90-158).
- ✅ **D25 skip-queued deviation** is implemented and documented in `run_queue`'s rustdoc (queue.rs:156-163) and pinned by `cancel_job_before_dequeue_skips_start_but_still_reports_finished` (queue.rs, asserts `Started == [0,1]` and a `Finished{2, Cancelled}` event).
- ✅ **Batch-cancel silence preserved.** Verified the un-diffed post-loop fill code (queue.rs:275-289, `outcome.unwrap_or(Cancelled)`) is untouched, and the worker's top-of-loop `stop || ctl.batch` gate still prevents dequeue entirely under batch cancel, so the D25 Started-suppression path is only reachable via `cancel_job`, never via `cancel_all` - the two mechanisms stay correctly separated.
- ✅ **D17 boundary respected.** Pre-spawn cancel (job.rs:98-113) constructs the outcome directly, bypassing `finish`/`delete_partial` entirely; `delete_partial`'s trigger condition (`Failed | Cancelled` from a job that actually ran) is unchanged.
- ✅ **delete_partial error surfacing** matches the specified `"delete_partial_failed: <io error>"` format exactly (job.rs:204-211), `NotFound` still silently ignored.
- ✅ **CLI SIGINT/exit-130 path genuinely unchanged.** Read past the diff hunk into the full `run.rs`: `cancel` (the ctrlc-flipped `Arc`) is the same `Arc` cloned into `QueueControl::new`, and the exit check at run.rs:253 (`if cancel.load(...) { return 130; }`) still reads the original, un-touched `Arc` - not a copy, no drift possible.
- ✅ **Per-job kill is synchronous** inside `cancel_job` (queue.rs:125-131), not deferred to the watcher.
- ✅ **Load-bearing invariants preserved**: drain-to-EOF-before-`wait()` ordering in `run_job` is untouched; the watcher thread still exists for batch cancel.
- ✅ **Core stays prose-free** except the one documented, brief-sanctioned exception.
- ✅ **Flagged test rework verified sound.** `killed_under_cancel_is_cancelled_and_partial_deleted`'s `Cell`-backed closure returns `false` on `run_job`'s first (pre-spawn) call and `true` from then on. Traced `run_job`'s exact call sites: `cancelled()` is invoked exactly twice on this path (pre-spawn check, then the post-`wait()` `None if cancelled()` guard, which only evaluates because the fake's scripted exit is `None`). The rework genuinely reproduces "job spawns, runs, gets killed mid-flight, discovered as cancelled on `wait()`" - a more accurate timeline than the original constant-`true` flag, not a weakened assertion. Final asserts (`Cancelled`, `exit_code: None`, partial deleted) are unchanged from the original.

No missing or extra scope found. ASCII punctuation confirmed clean (grepped the diff for smart quotes/dashes/ellipsis, no hits).

### Strengths

- Interface fidelity is exact, including the deliberately-narrow doc-comment nuance around why `cancelled` is a re-evaluated closure rather than a flag read once.
- Clean separation of concerns: the queue only gates the `Started` event; the actual skip/cancel decision lives once, in `run_job`'s own pre-spawn check, reused by both the queue-level and job-level behaviors (no duplicated cancellation logic).
- `delete_partial_failure_surfaces_into_errors` uses a genuinely portable failure trigger (a directory at the output path) rather than a permissions hack that would behave differently as root or on Windows.
- `cancel_job_kills_exactly_that_job_others_continue` correctly isolates the target (`SelectiveGateSpawner` gates only index 0) and asserts both the positive (`outcomes[0] == Cancelled`, partial deleted) and negative (`kills == 1`, others `Ok`) sides.

### Issues

#### Important (Should Fix)

**Lost-cancellation race between `cancel_job` and killer registration** - `queue.rs:125-131` (`cancel_job`), `queue.rs:315-320` (`RegisteringSpawner::spawn`), `job.rs:98` / `job.rs:158` (`run_job`'s two `cancelled()` checkpoints).

`cancel_job(index)` sets the per-job flag, then does a single, non-retried lookup in `ctl.killers`: if no `Killer` is registered for `index` yet, it silently does nothing further. A `Killer` is only inserted *after* the underlying spawn call returns successfully (`RegisteringSpawner::spawn`, after `self.inner.spawn(argv)?`). If `cancel_job(index)` is invoked in the window between the worker's own pre-spawn `cancelled()` read (already `false`) and that insert landing - i.e. while the process is actually being spawned - the flag gets set but no kill is issued, and nothing re-checks the flag afterward except `None if cancelled()` in the post-`wait()` match, which only fires when the process's exit code is genuinely `None` (i.e., it was actually killed). Since it was never killed, `wait()` returns a normal exit code and the job silently completes as `Ok`/`Warning`/`Failed` - the explicit cancel request is dropped with no error and no observable trace.

Neither new "kill in-flight" test can catch this: both `cancel_job_kills_exactly_that_job_others_continue` and the pre-existing batch-cancel test deliberately synchronize on a ready-channel signaled from inside the gated job's read loop, which is only reachable *after* `spawn()` (and hence killer registration) has already completed. The window this task's own review brief calls out by name ("the window where a job is dequeued but its killer not yet registered") is real, unmitigated, and untested.

This mirrors the pre-existing batch-cancel watcher's one-shot kill-then-exit pattern (queue.rs:190-204, unchanged here, which carries an analogous gap for jobs mid-spawn at the moment `cancel_all` is observed) - so the underlying architectural shape isn't new. But `cancel_job` is materially more exposed: it is a single synchronous call with zero retries backing it up, whereas the watcher at least polls every 50ms before the batch flag flips. For a per-job cancel triggered by, say, a GUI click on a row that just started, this is not a corner case - it's the most likely moment to click cancel.

A cheap fix: after `RegisteringSpawner::spawn` inserts the killer, re-check `ctl.job_cancelled(self.index)` and invoke the just-inserted killer immediately if true. `Killer`'s documented idempotent, best-effort contract (`spawn.rs`, confirmed: `killed.store` + best-effort `kill()`, safe under multiple invocations) makes this safe to add without new synchronization primitives.

### Minor (Nice to Have)

- `pre_spawn_cancellation_skips_spawn_and_deletes_nothing` (job.rs) doesn't assert the parent directory was never created, only that spawn didn't happen and the pre-existing output survives; the "zero filesystem touch" claim in the report is structurally guaranteed by code order but not directly pinned by this test.
- No dedicated test for double-invocation of the same `Killer` (`cancel_job` called twice, or `cancel_job` racing the batch watcher on the same index) or for the out-of-range `.get()` no-op paths; both are self-acknowledged in the report as low-risk reliance on existing contracts rather than gaps in new logic, and I agree with that risk assessment on its own merits.

### Assessment

**Task quality:** Needs fixes
**Reasoning:** Interfaces, D25 semantics, D17 boundary, CLI wiring, and the flagged test rework all verified correct against the brief. One Important, unaddressed concurrency gap remains: `cancel_job` can silently fail to cancel a job caught spawning at the exact moment it's called, with no error and no test coverage of that window - undermining the reliability of this task's core deliverable (per-job kill of an in-flight job).