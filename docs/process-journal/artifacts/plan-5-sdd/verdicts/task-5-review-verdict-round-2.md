<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-5  (round 2 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01B4r22pX7ydnvYmehrv2y7W
  agent_id:           ae090b7e57668bd9b
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-ae090b7e57668bd9b.jsonl
  dispatch_desc:      Review Task 5 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-10T12:59:54.342Z
  continuation_trigger: The coordinator sent a message while you were working: Re-review request for your Task 5 f
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance (re-review of fix commit e06bda0)

- ✅ **(1) The window is closed from both sides.** The two orderings form a correct store/check handshake. `cancel_job` does flag-store (SeqCst) *then* map-lookup; `RegisteringSpawner::spawn` does map-insert *then* flag-re-check (queue.rs, fix hunk). For the cancel to be lost, cancel_job's lookup would have to precede the insert in the killers-mutex total order *and* spawn's re-check would have to miss the flag. But lookup-before-insert means cancel_job's flag store (program-ordered before its lookup, published by the mutex release) happens-before the insert, which is program-ordered before the re-check's SeqCst load - the load must then observe the store. Contradiction: at least one path always kills. Both firing is a double invocation, safe under `Killer`'s idempotent best-effort contract (verified in spawn.rs: FakeJob killer = AtomicBool store; LiveJob killer = flag store + `let _ = kill()`). No interleaving loses the cancel.
- ✅ **(2) The new test genuinely pins the window.** `MidSpawnGateSpawner` is the *inner* spawner under `RegisteringSpawner`, so while it is parked: run_job's pre-spawn check has provably passed (spawn is only reached after it returned false) and the killer insert provably has not happened (it runs after `inner.spawn` returns, on the same single worker thread - `jobs: 1`). Crucially, the test cannot pass for a weaker reason: `cancel_job(0)` finds no killer in the map (insert hasn't run), and the flag alone cannot yield `Cancelled` - the `None if cancelled()` arm requires `wait() == None`, which for `FakeJob` requires an actual kill (verified spawn.rs:212-221: `wait` returns `self.exit`, i.e. `Some(0)`, unless the killed flag is set). Only the post-insert re-check can rescue it. The reported RED output (job completes `Ok`) is exactly the failure mode this mechanics forces on unfixed code. Deterministic channel handshake, no sleeps.
- ✅ **(3) Removal-side no-op claim holds.** After the worker's `.remove(&index)`, `run_job` has returned, which by construction means next_line was drained to EOF and `wait()` returned: the process has already exited. A late `cancel_job` sets a flag nothing reads again for that index and finds no killer - correct "cancel of a finished job does nothing" semantics, not a dropped request. The sub-window between run_job returning and the remove (killer still in the map, invoked on an exited process) is also harmless: LiveJob's `kill()` on a reaped child errors and is ignored; the outcome is already decided. No index reuse (`fetch_add` hands out each index once), so no stale-killer/wrong-process hazard.
- ✅ **(4) No regressions found.** The re-check kill runs *after* the map guard is dropped (the lock guard is a temporary of the insert statement), so no lock is held while invoking the killer on that path - no new lock-ordering edge; the only killers-mutex-held killer invocations remain the pre-existing `cancel_job`/watcher shape. The drain-to-EOF-before-`wait()` invariant is untouched (the new kill goes through the `Killer`, never through `wait`). Started/Finished semantics for the mid-spawn case are consistent with in-flight cancel (Started was already emitted, Finished{Cancelled} follows). The pre-spawn test extension correctly pins zero-filesystem-touch with a separate nested spec (needed because the first spec's pre-written file requires an existing parent), and `fake.spawned().is_empty()` now covers both calls.

### Strengths

- The fix also closes the *analogous batch-cancel gap* for free: the watcher's one-shot kill sweep misses a job mid-spawn at sweep time, but the re-check reads `job_cancelled(index)` = `batch || jobs[index]`, so a batch-cancelled job registering its killer after the sweep now kills itself. An unclaimed improvement, worth knowing for the whole-branch review.
- The comment at the fix site states the two-sided ordering argument precisely ("earlier than this check is seen via the flag; later than it finds the killer in the map") - the invariant is documented where a future refactor would break it.
- TDD discipline was real: the RED run against unfixed code reproduces the predicted silent-`Ok`, and the GREEN depends on verified `FakeJob` kill semantics, not on assertion softening.

### Issues

#### Critical (Must Fix)

None.

#### Important (Should Fix)

None. The previous Important finding is closed by e06bda0.

#### Minor (Nice to Have)

- The double-invocation path (`cancel_job` racing the re-check, both killing) remains untested directly; it rests on the documented idempotent `Killer` contract, which both existing implementations satisfy by construction. Acceptable documented reliance, unchanged from my first review.
- `cancel_job` still invokes a killer while holding the killers mutex (pre-existing pattern shared with the watcher, not introduced here). A `Killer` that blocks (LiveJob's child mutex held by a concurrent `wait()`) would briefly hold the map lock; bounded because `wait()` after EOF returns promptly. Note for the whole-branch review, no action needed in this task.

### Assessment

**Task quality:** Approved
**Reasoning:** The fix closes the lost-cancellation window with a correct two-sided ordering argument, the new test deterministically pins the exact window and cannot pass for a weaker reason, the removal-side no-op claim verifies out, and no regression or new concurrency hazard is introduced; the prior Minor (parent-dir assertion) is also closed.