<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-3  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01Rugz5a394GRa22QGwaKGDH
  agent_id:           aa3e4af188debbf81
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-aa3e4af188debbf81.jsonl
  dispatch_desc:      Review Task 3 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T21:34:02.589Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

I have everything I need. I've read the brief, the report, the full diff, and the two sanctioned out-of-diff files (`job.rs`, `spawn.rs`) that the named risks depend on. Line references below are to the files at HEAD (`b9960c5`); `queue.rs` is entirely new so its line N maps 1:1 to the diff's added lines.

### Spec Compliance
✅ Spec compliant.

- `JobEvent` (queue.rs:44-86): `#[derive(Debug, Clone, PartialEq, Serialize)]`, `#[serde(tag = "event", rename_all = "snake_case")]`, variants Started/Progress/Warning/Error/Finished with exact fields. Verbatim match.
- `QueueOpts { jobs, fail_fast }` (queue.rs:88-96): derive `Debug, Clone, Copy` exactly as the brief's block (no `Default` derive is mandated; "default 1" is the clamp behavior, satisfied by `.max(1)` at queue.rs:117). Clamp present.
- `run_queue(specs, &(dyn Spawn + Sync), opts, &Arc<AtomicBool>, &Sender<JobEvent>) -> Vec<JobOutcome>` (queue.rs:110-116): signature verbatim (`Sender` = `std::sync::mpsc::Sender`, imported queue.rs:37).
- Pinned shape all present: `std::thread::scope` (queue.rs:125), `AtomicUsize` next (118), `AtomicBool` stop (119), killer registry `Mutex<Vec<Option<Killer>>>` one slot per worker (121), watcher polling every 50ms flipping stop + invoking all killers then exiting (136-150), workers loop (154-196), `Failed && fail_fast -> set stop` (187-189).
- Cancelled-backfill after scope (queue.rs:210-218): `state: Cancelled, exit_code: None, warnings/errors empty, duration_ms: 0`, no Started for never-dequeued specs (Started is emitted only post-dequeue at 164). SendError ignored everywhere (`let _ = events.send(...)`).
- std threads only, no async. Core prose-free (events carry pass-through mkvmerge text, not core-authored UI prose). ASCII punctuation throughout the diff. Outcomes index-aligned (fetch_add unique index -> unique slot).
- All six mandated tests present and faithful; several are strengthened beyond the sketch (see below). FakeSpawner extension (`ConcurrencyTracker` spawn.rs:200-230, `with_concurrency_tracker` spawn.rs:140-146) is public, doc-commented, and scoped "Task 3 test support".

⚠️ Cannot verify from diff: `#![deny(missing_docs)]` lives in `lib.rs` (out of diff). Every public item in the diff *is* doc-commented and the reported build/clippy passed (deny-level would fail otherwise), so this is verified-by-compile, not a gap. Controller need not re-check.

### Named-risk findings

**1. Shutdown correctness — PASS.** Watcher has exactly two exits: `done` true -> return (queue.rs:138-140), `cancel` true -> stop+kill+return (141-147). `done` is set at queue.rs:172 only after all worker handles join (170-171); workers always return (index exhausted at 130, or stop/cancel at 126). So `done` is unconditionally reached and the watcher cannot spin forever even when cancel never fires. The `done` flag is not a semantic change to the brief's cancel behavior — it is the termination condition the brief's literal watcher text omits; without it `thread::scope` never returns on any successful run (guaranteed hang). Faithful, and in fact required, extension. Ordering `done`-before-`cancel` is safe: `done` implies all workers joined implies no in-flight job to kill, so skipping the sweep is correct, and a worker blocked on an unkilled real process cannot join, so `done` cannot preempt a needed kill sweep.

**2. Cancel-during-registration race — the window exists, outcome stays correct, plan-mandated.** Trace: a worker passes its loop-top `stop||cancel` check (queue.rs:126, both false), emits Started, enters `run_job`; if `cancel` is set and the watcher's one-shot sweep runs before that worker's `spawn` registers its killer (RegisteringSpawner, queue.rs:205), the sweep misses this job and then exits. The job spawns, runs unkilled: `run_job` does not poll `cancel` in its read loop (job.rs:111-123), so it drains to natural EOF; `wait()` returns a real code (not None), so the state maps to Ok/Warning/Failed, not Cancelled (job.rs:126-131). So yes — a job can run to completion despite `cancel`. It is bounded to at most one job per worker (stop is set before the sweep, so every worker's next loop-top check breaks), the outcome is recorded on its merits, index alignment holds, and nothing hangs (a real mkvmerge reaches EOF on its own; the queue closes normally). This is inherent to the brief's pinned one-shot watcher; the implementer flagged it. The cancel test dodges the window by construction (readiness signal is sent from `next_line`, i.e. strictly after registration, and the test sets `cancel` only then), so the test is honest about testing the in-registry case, not the race. See Minor item for the design note.

**3. Index alignment under jobs>=2 — PASS.** `index = next.fetch_add(1, SeqCst)` (queue.rs:129) hands each worker-iteration a unique index; `outcomes.lock()[index] = Some(...)` (164) therefore never collides, and the `Mutex` serializes writes (no data race). `index >= specs.len()` guard (130) means every write is in-bounds. Final construction `into_inner().unwrap().into_iter().map(unwrap_or(Cancelled))` (206-219) cannot panic on out-of-order completion and always yields `specs.len()` slots. `outcomes_index_aligned` (queue.rs:491) asserts by *content* (warning text in slot 1, error text in slot 3) via an index-keyed fake, which is robust to spawn-order nondeterminism — stronger than the sketch.

**4. Fail-fast exactly-one-Started — PASS, correct beyond the first-fails case.** `soft_fail_fast_cancels_queued_but_not_inflight` (queue.rs:382) asserts `[Failed, Cancelled, Cancelled]`, `started_count == 1`, AND `fake.spawned().len() == 1` (queue.rs:411). The spawn-count assertion genuinely pins "dequeue nothing further" — not just final states. Semantics when the failing job is not first are also correct by construction (stop is set only after a Failed at 187-189; earlier successful jobs are unaffected, later ones break at the loop top), though jobs:1-not-first is not covered by a test (Minor).

**5. Determinism of concurrency tests — PASS, no correctness-carrying timing.** `jobs_n_bounds_concurrency`: `ConcurrencyTracker::enter` runs inside `spawn` (spawn.rs:152-154) *before* the `Barrier::new(2)` wait in `next_line` (queue.rs:333-336); both of 2 workers must enter before either passes the barrier, so `max == 2` is recorded deterministically. 4 specs / 2 workers pair evenly across two barrier rounds — no odd-pairing deadlock, since a worker blocked at the barrier cannot grab a second index. No sleeps; asserts equality, not `<=`. `cancel_kills_inflight_and_cancels_queued`: the only wait is the watcher's 50ms poll, which is *latency*, not correctness — the assertions (all Cancelled, `started == [0]`, `kills >= 1`) depend on the readiness-ordering (registration-before-cancel), not on the poll interval. No residual timing dependence found; the report's 50/50 is consistent with the code.

### Strengths
- Interfaces match the brief verbatim; all six tests present, faithful, several strengthened (equality `== 2` not `<= 2`; explicit spawn-count assertion for fail-fast; content-based index-alignment).
- The determinism engineering is real, not decorative: barrier rendezvous forces exact overlap; the readiness-gated cancel closes the empty-registry race that a naive "wait for the Started event" would open (the report's RED-deadlock discovery is a genuine finding, and the fix is by-construction, not a timing bet); index-keyed scripting removes spawn-order nondeterminism.
- Lock discipline is clean: `killers`/`outcomes` mutex hold times are minimal; no lock-order cycle among `killers`, the child mutex inside the live killer, and the test gate/barrier (verified — nobody holds `killers` while blocking).
- Per-index event ordering is guaranteed: one worker owns an index start-to-finish and sends Started -> Progress/Warning/Error (stream order) -> Finished sequentially on one channel.
- Honest self-review: the best-effort kill window and the `done`-flag addition are both disclosed rather than buried.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. (The cancellation window in named-risk 2 is plan-mandated and does not violate any invariant — no hang, correct outcomes, correct index alignment — so I calibrate it Minor, not Important. Flagging here for visibility that I considered and rejected Important for it.)

#### Minor (Nice to Have)
- **Best-effort cancellation window (plan-mandated, queue.rs:136-150 one-shot watcher).** A job that registers its killer after the sweep runs to natural completion under `cancel` and is recorded as Ok/Warning/Failed instead of Cancelled. Correct-and-bounded, inherent to the brief's pinned one-shot design. If stronger cancellation is ever wanted, the watcher would loop-and-re-sweep instead of one-shot — a design change for the human to weigh, not a bug in this implementation of the pinned design.
- **Worker panics are swallowed (queue.rs:171, `let _ = handle.join()`).** A panicking worker leaves its slot None -> backfilled Cancelled silently; if the panic poisoned `killers`/`outcomes`, the later `into_inner().unwrap()` (207) panics instead. Inconsistent failure mode, but `run_job` does not panic on valid input, so this is defensive-robustness only.
- **`ConcurrencyTracker::exit` underflow (spawn.rs:227-229).** `current.fetch_sub(1)` is balanced only because `run_job` calls `wait()` exactly once; a future test calling `wait()` twice on a tracked `FakeJob` wraps `current` to `usize::MAX`. Test-helper fragility, not production code.
- **Coverage gap:** no test for fail-fast where the failing job is not the first (jobs:1), and no serde-shape test for `JobEvent` (report defers to Task 8). The derive is present and correct (internally-tagged enum, all struct variants — no serde newtype-variant limitation applies), so this is coverage breadth only.

### Assessment
**Task quality:** Approved
**Reasoning:** Interfaces and pinned shape match the brief verbatim, all six mandated tests are present and in several cases stronger than the sketch, and every named risk resolves correctly — the only cancellation imperfection is the brief's own one-shot-watcher best-effort window, which preserves every invariant (no hang, correct index-aligned outcomes) and is disclosed. Remaining items are Minor robustness and coverage notes, none blocking.