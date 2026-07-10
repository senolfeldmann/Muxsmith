### Task 5: Per-job cancel in core (D25): QueueControl, index-keyed killers, skip-set

**Files:**
- Modify: `crates/muxsmith-core/src/executor/queue.rs` (QueueControl, RegisteringSpawner re-key, worker loop, watcher), `crates/muxsmith-core/src/executor/job.rs` (cancel closure + pre-spawn check + delete_partial error surfacing), `crates/muxsmith-cli/src/commands/run.rs` (construct QueueControl)
- Test: `crates/muxsmith-core/tests/executor_events.rs` / queue unit tests (append)

**Interfaces:**
- Produces:

```rust
pub struct QueueControl { /* batch: Arc<AtomicBool>, jobs: Vec<AtomicBool>, killers: Mutex<HashMap<usize, Killer>> */ }
impl QueueControl {
    pub fn new(spec_count: usize, batch: Arc<AtomicBool>) -> Arc<QueueControl>;
    pub fn cancel_all(&self);                       // sets batch flag (watcher kills in-flight, as today)
    pub fn cancel_job(&self, index: usize);         // sets jobs[index]; if a killer is registered for index, invokes it NOW
    pub fn job_cancelled(&self, index: usize) -> bool; // batch || jobs[index]
}
pub fn run_queue(specs, spawner, opts, ctl: &Arc<QueueControl>, events) -> Vec<JobOutcome>  // signature change
```

- `run_job` signature change: `cancel: &AtomicBool` becomes `cancelled: &dyn Fn() -> bool` (queue passes `|| ctl.job_cancelled(index)`); job.rs:129 becomes `None if cancelled() => JobState::Cancelled`.
- Consumed by T8 (`cancel_run` -> `cancel_all`, `cancel_job(index)`), CLI (wraps its ctrlc flag: `QueueControl::new(specs.len(), Arc::clone(&cancel))`).

- [ ] **Step 1: TDD, one behavior per failing test:**
  1. *Skip queued:* cancel_job(2) before job 2 dequeues -> outcome[2] Cancelled, `Finished{index:2, Cancelled}` event emitted, NO `Started{index:2}` (deviation from never-dequeued silence - the GUI needs the confirmation; document in run_queue's rustdoc).
  2. *Kill in-flight:* with a blocking FakeSpawner job, cancel_job(index) kills exactly that job -> Cancelled, partial deleted (D17); other jobs unaffected, batch continues.
  3. *Pre-spawn check* (HANDOFF backlog): cancelled flag set before spawn -> Cancelled, spawner never called, nothing deleted.
  4. *delete_partial surfacing* (HANDOFF backlog): a failing partial delete pushes `"delete_partial_failed: <io error>"` into `outcome.errors` (third-party detail passthrough exception).
  5. *Batch semantics unchanged:* existing cancel tests pass with the CLI's flag wrapped in QueueControl.
- [ ] **Step 2:** Implement: registry re-key `Mutex<HashMap<usize, Killer>>` (RegisteringSpawner gains `index`, registers under it, worker removes after run_job); watcher unchanged except it iterates the map's values on batch cancel.
- [ ] **Step 3:** Update CLI run.rs construction; full gate green. **Commit** `feat(core): per-job cancellation via QueueControl (D25) + pre-spawn cancel check`

---

