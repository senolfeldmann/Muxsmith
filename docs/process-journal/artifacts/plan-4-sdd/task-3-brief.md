### Task 3: FIFO queue, `JobEvent` stream, soft fail-fast, cancellation

**Files:**
- Create: `crates/muxsmith-core/src/executor/queue.rs`
- Modify: `executor/mod.rs` (`pub mod queue;`)
- Test: unit tests in `queue.rs` against `FakeSpawner`

**Interfaces:**
- Consumes: Task 2's `run_job`/`JobSpec`/`JobOutcome`/`JobState`.
- Produces: `executor::queue::{JobEvent, QueueOpts, run_queue}`:

```rust
/// Serializable job-engine event (D13): the CLI renders it, Plan 5's Tauri
/// shell forwards it, a future --json-events streams it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum JobEvent {
    Started { index: usize, output: PathBuf },
    Progress { index: usize, percent: u8 },
    Warning { index: usize, text: String },
    Error { index: usize, text: String },
    Finished { index: usize, outcome: JobOutcome },
}

/// Queue policy (spec 6, D14).
#[derive(Debug, Clone, Copy)]
pub struct QueueOpts {
    /// Worker count; clamped to >= 1. Default 1 (sequential).
    pub jobs: usize,
    /// Soft fail-fast (D14): on the first Failed, dequeue nothing further;
    /// in-flight jobs finish; queued jobs become Cancelled.
    pub fail_fast: bool,
}

/// Runs `specs` FIFO over a bounded std worker pool. Returns one outcome
/// per spec, index-aligned. `cancel` (set by the CLI's SIGINT handler)
/// stops dequeuing AND kills in-flight jobs via their Killers; their
/// partials are deleted by `run_job`, queued specs become Cancelled.
pub fn run_queue(
    specs: &[JobSpec],
    spawner: &(dyn Spawn + Sync),
    opts: QueueOpts,
    cancel: &Arc<AtomicBool>,
    events: &std::sync::mpsc::Sender<JobEvent>,
) -> Vec<JobOutcome>
```

Implementation shape (pin it): `std::thread::scope`; shared `AtomicUsize` next-index + `AtomicBool` stop-dequeuing; killer registry `Mutex<Vec<Option<Killer>>>` (one slot per worker); a watcher thread polls `cancel` every 50ms and, when set, flips stop-dequeuing and invokes all registered killers, then exits; workers: loop { if stop or cancel -> break; take index; register killer; run_job with an `on_progress` closure that forwards as JobEvents; emit Started/Finished; on Failed && fail_fast -> set stop }. After the scope: every spec with no outcome gets `JobOutcome { state: Cancelled, exit_code: None, warnings: vec![], errors: vec![], duration_ms: 0 }` (no Started event - it never started). Event send failures (`SendError`) are ignored (receiver gone = caller stopped listening).

- [ ] **Step 1: Failing tests:**

```rust
#[test] fn sequential_fifo_order_and_events() { /* jobs:1, 3 specs; Started/Finished index order 0,1,2 */ }
#[test] fn jobs_n_bounds_concurrency() { /* FakeSpawner variant with a live-jobs counter (Arc<AtomicUsize> max tracker); jobs:2, 4 specs; max observed <= 2 */ }
#[test] fn soft_fail_fast_cancels_queued_but_not_inflight() { /* jobs:1, 3 specs, first Failed; outcomes [Failed, Cancelled, Cancelled]; exactly one Started */ }
#[test] fn no_fail_fast_continues_past_failure() { /* [Failed, Ok, Ok] */ }
#[test] fn cancel_kills_inflight_and_cancels_queued() { /* long-scripted fake; set cancel after first Started; outcomes [Cancelled, Cancelled, ...]; killer invoked */ }
#[test] fn outcomes_index_aligned() { /* jobs:2 with different-length scripts; outcome[i] matches spec i */ }
```

(The concurrency-tracking fake extension: add to `FakeSpawner` a shared `Arc<AtomicUsize>` current/max counter incremented in `spawn`/decremented in `wait` - extend Task 1's fake here, doc-commented.)

- [ ] **Step 2: RED. Step 3: implement. Step 4: GREEN + full gate. Step 5: Commit** - `feat(executor): FIFO job queue with JobEvent stream, jobs N, soft fail-fast, cancellation (D13/D14)`

---

