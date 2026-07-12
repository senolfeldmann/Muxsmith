//! FIFO job queue (spec 6, D13/D14): runs a batch of [`JobSpec`]s over a
//! bounded std worker pool, streaming a [`JobEvent`] per state change, and
//! honoring soft fail-fast (D14) and cooperative cancellation (D16).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;

use super::job::{JobOutcome, JobProgress, JobSpec, JobState, run_job};
use super::spawn::{Killer, RunningJob, Spawn, SpawnError};
use crate::report::DiagCode;

/// Serializable job-engine event (D13): the CLI renders it, Plan 5's Tauri
/// shell forwards it, a future --json-events streams it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum JobEvent {
    /// A queued spec was dequeued and handed to the runner.
    Started {
        /// Index into the `specs` slice passed to [`run_queue`].
        index: usize,
        /// The job's rendered output path.
        output: PathBuf,
    },
    /// A parsed `#GUI#progress NN%` line.
    Progress {
        /// Index into the `specs` slice passed to [`run_queue`].
        index: usize,
        /// 0-100.
        percent: u8,
    },
    /// A captured warning line.
    Warning {
        /// Index into the `specs` slice passed to [`run_queue`].
        index: usize,
        /// Tag-stripped warning text.
        text: String,
    },
    /// A captured error line.
    Error {
        /// Index into the `specs` slice passed to [`run_queue`].
        index: usize,
        /// Tag-stripped error text.
        text: String,
    },
    /// A job reached its terminal state.
    Finished {
        /// Index into the `specs` slice passed to [`run_queue`].
        index: usize,
        /// The terminal outcome.
        outcome: JobOutcome,
    },
    /// A raw output line (D24), verbatim, for every line mkvmerge wrote that
    /// was not a `#GUI#progress` tick. Feeds a live log pane and persisted
    /// job logs; warning/error lines appear both here (verbatim) and as
    /// their own tagged [`JobEvent::Warning`]/[`JobEvent::Error`].
    Output {
        /// Index into the `specs` slice passed to [`run_queue`].
        index: usize,
        /// The line, verbatim (tags included).
        line: String,
    },
}

/// Queue policy (spec 6, D14).
#[derive(Debug, Clone, Copy)]
pub struct QueueOpts {
    /// Requested worker count; clamped to >= 1, then further capped at the
    /// batch's spec count (see the private `worker_count` helper) so a `--jobs` far larger
    /// than the batch never spawns idle OS threads. Default 1 (sequential).
    pub jobs: usize,
    /// Soft fail-fast (D14): on the first Failed, dequeue nothing further;
    /// in-flight jobs finish; queued jobs become Cancelled.
    pub fail_fast: bool,
}

/// How often the watcher thread polls the batch-cancel flag.
const CANCEL_POLL: Duration = Duration::from_millis(50);

/// Batch- and per-job cancellation control (D25). Wraps a single shared
/// batch-cancel flag (D16: today's SIGINT semantics, e.g. the CLI's ctrlc
/// handler) alongside one flag per spec index, so a caller can cancel
/// either the whole batch or a single job by its index into the `specs`
/// slice passed to [`run_queue`]. The map of currently in-flight
/// [`Killer`]s, keyed by that same index, is what lets [`Self::cancel_job`]
/// kill an in-flight job synchronously, without waiting for the watcher's
/// next poll.
pub struct QueueControl {
    batch: Arc<AtomicBool>,
    jobs: Vec<AtomicBool>,
    killers: Mutex<HashMap<usize, Killer>>,
}

impl QueueControl {
    /// Builds a fresh control for a batch of `spec_count` jobs, wrapping
    /// `batch` (the caller's own batch-cancel flag - e.g. the CLI's ctrlc
    /// handler shares this same `Arc`, so existing batch-cancel wiring is
    /// unchanged) with one additional per-job flag per spec.
    pub fn new(spec_count: usize, batch: Arc<AtomicBool>) -> Arc<QueueControl> {
        Arc::new(QueueControl {
            batch,
            jobs: (0..spec_count).map(|_| AtomicBool::new(false)).collect(),
            killers: Mutex::new(HashMap::new()),
        })
    }

    /// Requests batch cancellation (D16, unchanged): the watcher thread
    /// stops dequeuing and kills every currently registered in-flight job.
    pub fn cancel_all(&self) {
        self.batch.store(true, Ordering::SeqCst);
    }

    /// Requests cancellation of a single job by its spec `index` (D25):
    /// sets that job's flag, then, if a [`Killer`] is currently registered
    /// for `index` (the job is in flight), invokes it immediately rather
    /// than waiting for the watcher's next poll. A queued job (not yet
    /// dequeued) has no registered killer; setting its flag alone is
    /// enough, since the worker consults [`Self::job_cancelled`] before
    /// spawning (`run_job`'s pre-spawn check). An `index` outside the
    /// batch's spec count is a no-op.
    pub fn cancel_job(&self, index: usize) {
        if let Some(flag) = self.jobs.get(index) {
            flag.store(true, Ordering::SeqCst);
        }
        if let Some(killer) = self.killers().get(&index) {
            killer();
        }
    }

    /// Locks `killers`, recovering from poisoning instead of propagating a
    /// second panic (hardening: a worker panicking mid `insert`/`remove`
    /// must not also poison every other worker's, and the watcher's, own
    /// access to this same map, cascading one job's bug into the whole
    /// batch). Recovery is sound because every write this lock guards is a
    /// single `HashMap` `insert`/`remove` -- never a multi-step invariant a
    /// panic could leave half-applied -- so whatever the map held right
    /// before a panic is exactly what it holds right after: still a valid
    /// killer registry.
    fn killers(&self) -> std::sync::MutexGuard<'_, HashMap<usize, Killer>> {
        self.killers
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Whether job `index` is cancelled, either because the whole batch was
    /// ([`Self::cancel_all`]) or because that job specifically was
    /// ([`Self::cancel_job`]). Passed to [`run_job`] as its `cancelled`
    /// closure.
    pub fn job_cancelled(&self, index: usize) -> bool {
        self.batch.load(Ordering::SeqCst)
            || self
                .jobs
                .get(index)
                .is_some_and(|flag| flag.load(Ordering::SeqCst))
    }
}

/// Runs `specs` FIFO over a bounded std worker pool. Returns one outcome
/// per spec, index-aligned. `ctl` (D25) carries both the batch-cancel flag
/// (e.g. the CLI's SIGINT handler) and a per-job cancel flag for each spec
/// index (e.g. a GUI's per-row cancel): batch cancellation stops dequeuing
/// AND kills every in-flight job via its registered [`Killer`]; per-job
/// cancellation ([`QueueControl::cancel_job`]) kills only that job if it is
/// in flight, or, if the spec has not yet been dequeued, is caught the
/// moment a worker picks it up, by `run_job`'s pre-spawn check.
///
/// Specs never dequeued because of BATCH cancellation emit no events (no
/// Started - they never started); their Cancelled outcomes appear only in
/// the returned vector (D16, unchanged). A spec cancelled PER-JOB before a
/// worker dequeues it is an explicit deviation from that silence (D25): it
/// still emits `Finished { outcome: Cancelled }` (never `Started`), because
/// a GUI needs the row-level confirmation that its cancel click took
/// effect, even for a job that never actually started. Event send failures
/// are ignored (receiver gone = caller stopped listening).
pub fn run_queue(
    specs: &[JobSpec],
    spawner: &(dyn Spawn + Sync),
    opts: QueueOpts,
    ctl: &Arc<QueueControl>,
    events: &Sender<JobEvent>,
) -> Vec<JobOutcome> {
    let workers = worker_count(opts.jobs, specs.len());
    let next = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let done = AtomicBool::new(false);
    let outcomes: Mutex<Vec<Option<JobOutcome>>> =
        Mutex::new((0..specs.len()).map(|_| None).collect());
    // One slot per worker, holding the spec index that worker last
    // dequeued; cleared back to `NO_JOB` only once that job's outcome has
    // actually been written to `outcomes` (see the worker loop below). A
    // panic unwinds past every other local inside the worker closure, so
    // this is the one piece of state that survives it: `handle.join()`
    // below reads it, once a worker's thread has died, to recover which
    // job it was running when it did.
    let current_index: Vec<AtomicUsize> = (0..workers).map(|_| AtomicUsize::new(NO_JOB)).collect();

    std::thread::scope(|scope| {
        let next = &next;
        let stop = &stop;
        let done = &done;
        let outcomes = &outcomes;
        let current_index = &current_index;

        // Watcher: polls the batch flag; on batch cancellation flips
        // stop-dequeuing, kills every in-flight job through its registered
        // Killer, and exits. Also exits (without killing) once all workers
        // are done, so the scope can close on a natural finish. Per-job
        // cancellation needs no watcher involvement:
        // QueueControl::cancel_job kills its target synchronously.
        scope.spawn(move || {
            loop {
                if done.load(Ordering::SeqCst) {
                    return;
                }
                if ctl.batch.load(Ordering::SeqCst) {
                    stop.store(true, Ordering::SeqCst);
                    for killer in ctl.killers().values() {
                        killer();
                    }
                    return;
                }
                std::thread::sleep(CANCEL_POLL);
            }
        });

        let handles: Vec<_> = (0..workers)
            .map(|worker_id| {
                scope.spawn(move || {
                    loop {
                        if stop.load(Ordering::SeqCst) || ctl.batch.load(Ordering::SeqCst) {
                            return;
                        }
                        let index = next.fetch_add(1, Ordering::SeqCst);
                        if index >= specs.len() {
                            return;
                        }
                        current_index[worker_id].store(index, Ordering::SeqCst);
                        let spec = &specs[index];

                        // A job already cancelled per-job at dequeue time
                        // never gets a Started event (D25 deviation - see
                        // this function's rustdoc); run_job's own
                        // pre-spawn check still runs below and produces
                        // its Cancelled outcome without touching the
                        // spawner or the filesystem.
                        if !ctl.job_cancelled(index) {
                            let _ = events.send(JobEvent::Started {
                                index,
                                output: spec.output.clone(),
                            });
                        }

                        let registering = RegisteringSpawner {
                            inner: spawner,
                            ctl,
                            index,
                        };
                        let mut on_progress = |progress: JobProgress| {
                            let event = match progress {
                                JobProgress::Percent(percent) => {
                                    JobEvent::Progress { index, percent }
                                }
                                JobProgress::WarningLine(text) => JobEvent::Warning { index, text },
                                JobProgress::ErrorLine(text) => JobEvent::Error { index, text },
                                JobProgress::OutputLine(line) => JobEvent::Output { index, line },
                            };
                            let _ = events.send(event);
                        };
                        let outcome = run_job(
                            &registering,
                            spec,
                            &|| ctl.job_cancelled(index),
                            &mut on_progress,
                        );
                        ctl.killers().remove(&index);

                        if outcome.state == JobState::Failed && opts.fail_fast {
                            stop.store(true, Ordering::SeqCst);
                        }
                        let _ = events.send(JobEvent::Finished {
                            index,
                            outcome: outcome.clone(),
                        });
                        lock_outcomes(outcomes)[index] = Some(outcome);
                        current_index[worker_id].store(NO_JOB, Ordering::SeqCst);
                    }
                })
            })
            .collect();

        for (worker_id, handle) in handles.into_iter().enumerate() {
            if let Err(payload) = handle.join() {
                recover_panicked_worker(worker_id, payload, current_index, ctl, outcomes, events);
            }
        }
        done.store(true, Ordering::SeqCst);
    });

    // Poisoning here is unreachable in practice today (every write above
    // already recovers instead of panicking while holding this lock), but
    // `into_inner()` still surfaces a `PoisonError` if that ever changes;
    // recovering rather than `.unwrap()`-ing keeps a single future
    // regression elsewhere from taking down this final collection step
    // too.
    outcomes
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .into_iter()
        .map(|outcome| {
            outcome.unwrap_or(JobOutcome {
                state: JobState::Cancelled,
                exit_code: None,
                warnings: Vec::new(),
                errors: Vec::new(),
                duration_ms: 0,
            })
        })
        .collect()
}

/// The worker-pool size for a batch of `spec_count` specs: `jobs` clamped to
/// at least 1, then capped at `spec_count` (also at least 1, so an empty
/// batch still gets one idle worker rather than zero). A worker beyond
/// `spec_count` could never dequeue a spec (each fetches a distinct index
/// once), so it would only ever sit idle; an oversized `--jobs` would
/// otherwise spawn that many real OS threads for nothing; at extreme values
/// enough to exhaust the OS and panic inside the scope (`Scope::spawn`
/// panics on a thread-creation failure).
fn worker_count(jobs: usize, spec_count: usize) -> usize {
    jobs.max(1).min(spec_count.max(1))
}

/// Sentinel `current_index` value meaning "this worker is between jobs (or
/// has not dequeued one yet)", never a real spec index.
const NO_JOB: usize = usize::MAX;

/// Locks `outcomes`, recovering from poisoning instead of propagating a
/// second panic. Sound for the same reason as [`QueueControl::killers`]:
/// the one write this lock ever guards (`outcomes[index] = Some(...)`) is a
/// single slot assignment, never a multi-step invariant a panic could leave
/// half-applied -- and refusing to recover would turn one worker's panic
/// into every other, perfectly healthy, worker's inability to ever record
/// its own outcome once its turn to lock this comes up.
fn lock_outcomes(
    outcomes: &Mutex<Vec<Option<JobOutcome>>>,
) -> std::sync::MutexGuard<'_, Vec<Option<JobOutcome>>> {
    outcomes
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Recovers from one worker thread's panic (`handle.join()`'s `Err`, in
/// [`run_queue`]): a bug inside this crate's own job-execution path, never
/// an mkvmerge failure (a failing mux is `run_job`'s own `Failed` return,
/// not a panic). Before this existed, the panic was silently swallowed
/// (`let _ = handle.join();`) and the job's slot in `outcomes` stayed
/// `None` forever, so [`run_queue`]'s final fallback mislabeled it
/// `Cancelled` -- indistinguishable from a job that was never dequeued at
/// all.
///
/// `current_index[worker_id]` still holds the spec index that worker last
/// dequeued (cleared only after a successful outcome write, see the worker
/// loop in [`run_queue`]), so it is the one piece of state that survives
/// the unwind and identifies which job died with it; `NO_JOB` means the
/// panic happened outside any job (no such path exists today, but there is
/// then nothing to recover).
///
/// The panic payload itself (whatever `panic!`/`.unwrap()`/`.expect()` was
/// called with) is arbitrary developer-diagnostic content, not a stable,
/// translatable code -- like `job.rs`'s `delete_partial_failed` detail, it
/// is core's one deliberate prose-free exception (spec 6/7): logged here
/// for triage, never carried past this function into the user-facing
/// [`JobOutcome`] beyond the stable `worker-panicked` [`DiagCode`].
fn recover_panicked_worker(
    worker_id: usize,
    payload: Box<dyn std::any::Any + Send>,
    current_index: &[AtomicUsize],
    ctl: &QueueControl,
    outcomes: &Mutex<Vec<Option<JobOutcome>>>,
    events: &Sender<JobEvent>,
) {
    let index = current_index[worker_id].load(Ordering::SeqCst);
    if index == NO_JOB {
        return;
    }
    let message = payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "<non-string panic payload>".to_string());
    eprintln!("muxsmith: worker thread panicked while running job {index}: {message}");

    // Invoke before dropping: a child spawned just before the panic is
    // still running and would otherwise keep writing to an output this
    // function is about to report Failed (killer is idempotent/best-effort).
    if let Some(killer) = ctl.killers().remove(&index) {
        killer();
    }
    let outcome = JobOutcome {
        state: JobState::Failed,
        exit_code: None,
        warnings: Vec::new(),
        errors: vec![format!("{}: job {index}", DiagCode::WorkerPanicked.key())],
        // The worker died mid-`run_job`, before it could measure its own
        // elapsed time; there is nothing truthful to report here.
        duration_ms: 0,
    };
    let _ = events.send(JobEvent::Finished {
        index,
        outcome: outcome.clone(),
    });
    lock_outcomes(outcomes)[index] = Some(outcome);
}

/// Wraps the caller's spawner so a successful spawn registers the new job's
/// [`Killer`] into the control's registry under its own spec `index` (D25)
/// before `run_job` starts streaming, giving [`QueueControl::cancel_job`]
/// and the watcher (batch cancel) a handle to an in-flight job neither owns.
/// The worker loop removes the entry once `run_job` returns.
struct RegisteringSpawner<'a> {
    inner: &'a (dyn Spawn + Sync),
    ctl: &'a QueueControl,
    index: usize,
}

impl Spawn for RegisteringSpawner<'_> {
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
        let job = self.inner.spawn(argv)?;
        let killer = job.killer();
        self.ctl.killers().insert(self.index, Arc::clone(&killer));
        // Closes the D25 lost-cancellation window: a cancel_job landing
        // between run_job's pre-spawn check and the insert above found no
        // killer to invoke, and a normally-exiting process never reaches
        // the `None if cancelled()` arm - the request would be silently
        // dropped. Re-check now that the killer is registered and kill the
        // fresh process ourselves if so. The ordering makes the window
        // airtight: a cancel_job earlier than this check is seen via the
        // flag here; one later than it finds the killer in the map. A
        // double invocation (both paths firing) is harmless - Killer is
        // idempotent and best-effort by contract (spawn.rs).
        if self.ctl.job_cancelled(self.index) {
            killer();
        }
        Ok(job)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::{Barrier, Condvar};

    use crate::executor::spawn::{ConcurrencyTracker, FakeSpawner};

    /// A [`JobSpec`] whose argv encodes `index` in `argv[0]`, so a
    /// per-spec-differentiated fake (see [`ScriptByIndexSpawner`]) can
    /// script it deterministically regardless of dequeue/spawn-call order
    /// under `jobs > 1`.
    fn spec(index: usize, output: std::path::PathBuf) -> JobSpec {
        JobSpec {
            argv: vec![index.to_string()],
            output,
        }
    }

    #[test]
    fn sequential_fifo_order_and_events() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(outcomes.len(), 3);
        for outcome in &outcomes {
            assert_eq!(outcome.state, JobState::Ok);
        }

        let started: Vec<usize> = events
            .iter()
            .filter_map(|e| match e {
                JobEvent::Started { index, .. } => Some(*index),
                _ => None,
            })
            .collect();
        let finished: Vec<usize> = events
            .iter()
            .filter_map(|e| match e {
                JobEvent::Finished { index, .. } => Some(*index),
                _ => None,
            })
            .collect();
        assert_eq!(started, vec![0, 1, 2], "FIFO dequeue order under jobs:1");
        assert_eq!(finished, vec![0, 1, 2]);
    }

    /// Wraps a [`FakeSpawner`] so every job's first `next_line` rendezvous
    /// at a [`Barrier`] sized to the worker count before proceeding: this
    /// forces genuine overlap deterministically instead of hoping OS
    /// scheduling produces it. Safe from deadlock because `run_queue`
    /// spawns exactly `opts.jobs` workers, each with at most one in-flight
    /// job, and a spec count that is an exact multiple of `jobs` makes
    /// every rendezvous round full.
    struct RendezvousSpawner<'a> {
        inner: &'a FakeSpawner,
        barrier: Arc<Barrier>,
    }

    impl Spawn for RendezvousSpawner<'_> {
        fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            let inner = self.inner.spawn(argv)?;
            Ok(Box::new(RendezvousJob {
                inner,
                barrier: Arc::clone(&self.barrier),
                waited: false,
            }))
        }
    }

    struct RendezvousJob {
        inner: Box<dyn RunningJob>,
        barrier: Arc<Barrier>,
        waited: bool,
    }

    impl RunningJob for RendezvousJob {
        fn next_line(&mut self) -> Option<String> {
            if !self.waited {
                self.waited = true;
                self.barrier.wait();
            }
            self.inner.next_line()
        }
        fn wait(&mut self) -> Option<i32> {
            self.inner.wait()
        }
        fn killer(&self) -> Killer {
            self.inner.killer()
        }
    }

    #[test]
    fn jobs_n_bounds_concurrency() {
        let dir = tempfile::tempdir().unwrap();
        let specs: Vec<JobSpec> = (0..4)
            .map(|i| spec(i, dir.path().join(format!("out{i}.mkv"))))
            .collect();
        let tracker = ConcurrencyTracker::new();
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0))
            .with_concurrency_tracker(Arc::clone(&tracker));
        let spawner = RendezvousSpawner {
            inner: &fake,
            barrier: Arc::new(Barrier::new(2)),
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 2,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &spawner, opts, &ctl, &tx);
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(outcomes.len(), 4);
        for outcome in &outcomes {
            assert_eq!(outcome.state, JobState::Ok);
        }
        assert_eq!(
            tracker.max(),
            2,
            "two workers must rendezvous at exactly 2 concurrent jobs"
        );
    }

    #[test]
    fn soft_fail_fast_cancels_queued_but_not_inflight() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let fake = FakeSpawner::script(vec!["#GUI#error boom".to_string()], Some(2));
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: true,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(
            outcomes.iter().map(|o| o.state).collect::<Vec<_>>(),
            vec![JobState::Failed, JobState::Cancelled, JobState::Cancelled]
        );
        let started_count = events
            .iter()
            .filter(|e| matches!(e, JobEvent::Started { .. }))
            .count();
        assert_eq!(started_count, 1, "queued specs must never emit Started");
        assert_eq!(fake.spawned().len(), 1, "queued specs must never spawn");
    }

    /// A [`Spawn`] fake that scripts each spec independently, keyed by the
    /// spec's own index (encoded into `argv[0]` by the test's [`spec`]
    /// helper) rather than by call order: under `jobs > 1` worker
    /// interleaving makes spawn-call order non-deterministic, so
    /// index-keying is what makes per-spec differentiated outcomes
    /// deterministic regardless of concurrency.
    struct ScriptByIndexSpawner {
        scripts: Vec<(Vec<String>, Option<i32>)>,
    }

    impl Spawn for ScriptByIndexSpawner {
        fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            let index: usize = argv[0].parse().expect("test spec index encoded in argv[0]");
            let (lines, exit) = self.scripts[index].clone();
            Ok(Box::new(ScriptedJob {
                lines,
                cursor: 0,
                exit,
            }))
        }
    }

    struct ScriptedJob {
        lines: Vec<String>,
        cursor: usize,
        exit: Option<i32>,
    }

    impl RunningJob for ScriptedJob {
        fn next_line(&mut self) -> Option<String> {
            if self.cursor >= self.lines.len() {
                return None;
            }
            let line = self.lines[self.cursor].clone();
            self.cursor += 1;
            Some(line)
        }
        fn wait(&mut self) -> Option<i32> {
            self.exit
        }
        fn killer(&self) -> Killer {
            Arc::new(|| {})
        }
    }

    #[test]
    fn no_fail_fast_continues_past_failure() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let fake = ScriptByIndexSpawner {
            scripts: vec![
                (vec!["#GUI#error boom".to_string()], Some(2)),
                (vec!["#GUI#progress 100%".to_string()], Some(0)),
                (vec!["#GUI#progress 100%".to_string()], Some(0)),
            ],
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(
            outcomes.iter().map(|o| o.state).collect::<Vec<_>>(),
            vec![JobState::Failed, JobState::Ok, JobState::Ok]
        );
    }

    /// A [`Spawn`] fake whose `spawn` panics for one specific spec index
    /// (encoded into `argv[0]`, [`ScriptByIndexSpawner`]'s convention) and
    /// scripts every other index as an ordinary success -- the harness for
    /// [`worker_panic_is_reported_as_failed_not_cancelled`].
    struct PanicOnIndexSpawner {
        panic_index: usize,
    }

    impl Spawn for PanicOnIndexSpawner {
        fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            let index: usize = argv[0].parse().expect("test spec index encoded in argv[0]");
            if index == self.panic_index {
                panic!("scripted worker panic for job {index}");
            }
            Ok(Box::new(ScriptedJob {
                lines: vec!["#GUI#progress 100%".to_string()],
                cursor: 0,
                exit: Some(0),
            }))
        }
    }

    /// A worker thread panicking mid-job (a bug in this crate, never an
    /// mkvmerge failure) used to mislabel that job `Cancelled` --
    /// indistinguishable from a spec that was never dequeued at all --
    /// because the panic was silently swallowed (`let _ = handle.join();`)
    /// before the job's slot in `outcomes` was ever written. This pins the
    /// fix: the panicked job must be reported `Failed` with the
    /// `worker-panicked` diagnostic code, the surviving worker must still
    /// finish the rest of the batch (`jobs: 2` so a second, healthy worker
    /// is still alive to pick up what the dead one would have), and
    /// `run_queue` itself must not propagate the panic -- this test
    /// function would abort the whole process if it did.
    #[test]
    fn worker_panic_is_reported_as_failed_not_cancelled() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let fake = PanicOnIndexSpawner { panic_index: 0 };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 2,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(
            outcomes[0].state,
            JobState::Failed,
            "a panicked worker must be reported Failed, not silently Cancelled"
        );
        assert!(
            outcomes[0]
                .errors
                .iter()
                .any(|e| e.starts_with(DiagCode::WorkerPanicked.key())),
            "expected a worker-panicked entry, got: {:?}",
            outcomes[0].errors
        );
        assert_eq!(
            outcomes[1].state,
            JobState::Ok,
            "a surviving worker must still finish the rest of the batch"
        );
        assert_eq!(outcomes[2].state, JobState::Ok);
    }

    /// Direct regression test for the `unwrap_or_else(|p| p.into_inner())`
    /// poison-recovery pattern now used throughout this module (`killers`,
    /// `outcomes`): a panic while holding a `MutexGuard` poisons the
    /// `Mutex`, but a plain assignment made *before* the panic is a
    /// completed write, not a half-applied one -- so recovering via
    /// `into_inner()` must yield exactly that state, never lose or reset
    /// it. This is the soundness claim documented on
    /// [`QueueControl::killers`] and [`lock_outcomes`], pinned here so a
    /// future change to the pattern itself cannot silently regress it.
    #[test]
    fn poisoned_mutex_recovers_state_written_before_the_panic() {
        let mutex = Mutex::new(vec![1, 2, 3]);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut guard = mutex.lock().unwrap();
            guard.push(4);
            panic!("simulated poison, deliberately while still holding the guard");
        }));
        assert!(mutex.is_poisoned());

        let recovered = mutex
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        assert_eq!(
            *recovered,
            vec![1, 2, 3, 4],
            "recovery must yield the state as it stood right before the panic"
        );
    }

    #[test]
    fn outcomes_index_aligned() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
            spec(3, dir.path().join("d.mkv")),
        ];
        let fake = ScriptByIndexSpawner {
            scripts: vec![
                (vec!["#GUI#progress 100%".to_string()], Some(0)),
                (
                    vec!["#GUI#warning 'x.srt': track ignored.".to_string()],
                    Some(1),
                ),
                (
                    vec![
                        "#GUI#progress 50%".to_string(),
                        "#GUI#progress 100%".to_string(),
                    ],
                    Some(0),
                ),
                (vec!["#GUI#error boom".to_string()], Some(2)),
            ],
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 2,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(outcomes.len(), 4);
        assert_eq!(outcomes[0].state, JobState::Ok);
        assert_eq!(outcomes[1].state, JobState::Warning);
        assert_eq!(
            outcomes[1].warnings,
            vec!["'x.srt': track ignored.".to_string()],
            "the warning must land in spec 1's slot"
        );
        assert_eq!(outcomes[2].state, JobState::Ok);
        assert_eq!(outcomes[3].state, JobState::Failed);
        assert_eq!(
            outcomes[3].errors,
            vec!["boom".to_string()],
            "the error must land in spec 3's slot"
        );
    }

    /// Test-local synchronization for the cancel test (Task 3 determinism
    /// rule): a job's `next_line` first signals readiness over `ready`,
    /// then blocks on a condvar until its own [`Killer`] fires, mimicking a
    /// real mkvmerge child that only dies when actually killed. The test
    /// sets `cancel` only after the readiness signal, which the job sends
    /// strictly after `spawn` returned - i.e. after the queue registered
    /// the job's killer - so the watcher's one-shot kill sweep can never
    /// run before the killer it must invoke exists. No sleeps, no timing
    /// bets; the only wait is the watcher's own bounded 50ms poll.
    struct Gate {
        killed: Mutex<bool>,
        cvar: Condvar,
    }

    impl Gate {
        fn new() -> Gate {
            Gate {
                killed: Mutex::new(false),
                cvar: Condvar::new(),
            }
        }

        fn block_until_killed(&self) {
            let mut killed = self.killed.lock().unwrap();
            while !*killed {
                killed = self.cvar.wait(killed).unwrap();
            }
        }

        fn kill(&self) {
            *self.killed.lock().unwrap() = true;
            self.cvar.notify_all();
        }
    }

    struct GatedFakeSpawner {
        kills: Arc<AtomicUsize>,
        ready: Sender<()>,
    }

    impl Spawn for GatedFakeSpawner {
        fn spawn(&self, _argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            Ok(Box::new(GatedJob {
                gate: Arc::new(Gate::new()),
                kills: Arc::clone(&self.kills),
                ready: self.ready.clone(),
            }))
        }
    }

    struct GatedJob {
        gate: Arc<Gate>,
        kills: Arc<AtomicUsize>,
        ready: Sender<()>,
    }

    impl RunningJob for GatedJob {
        fn next_line(&mut self) -> Option<String> {
            let _ = self.ready.send(());
            self.gate.block_until_killed();
            None
        }
        fn wait(&mut self) -> Option<i32> {
            None
        }
        fn killer(&self) -> Killer {
            let gate = Arc::clone(&self.gate);
            let kills = Arc::clone(&self.kills);
            Arc::new(move || {
                kills.fetch_add(1, Ordering::SeqCst);
                gate.kill();
            })
        }
    }

    #[test]
    fn cancel_kills_inflight_and_cancels_queued() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let kills = Arc::new(AtomicUsize::new(0));
        let (ready_tx, ready_rx) = mpsc::channel();
        let fake = GatedFakeSpawner {
            kills: Arc::clone(&kills),
            ready: ready_tx,
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: false,
        };

        let outcomes = std::thread::scope(|scope| {
            let handle = scope.spawn(|| run_queue(&specs, &fake, opts, &ctl, &tx));
            // recv_timeout is a hang-to-failure converter, not a race
            // window: in a correct queue the signal always arrives; the
            // ceiling only makes a regression fail instead of deadlock.
            ready_rx
                .recv_timeout(Duration::from_secs(10))
                .expect("first job never reached its read loop");
            ctl.cancel_all();
            handle.join().expect("run_queue thread panicked")
        });
        drop(tx);
        let events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(
            outcomes.iter().map(|o| o.state).collect::<Vec<_>>(),
            vec![
                JobState::Cancelled,
                JobState::Cancelled,
                JobState::Cancelled
            ]
        );
        let started: Vec<usize> = events
            .iter()
            .filter_map(|e| match e {
                JobEvent::Started { index, .. } => Some(*index),
                _ => None,
            })
            .collect();
        assert_eq!(
            started,
            vec![0],
            "only the in-flight job may start once cancelled"
        );
        assert!(kills.load(Ordering::SeqCst) >= 1, "killer must be invoked");
    }

    /// D25 behavior 1 ("skip queued"): a job cancelled per-job before a
    /// worker ever dequeues it must still finish `Cancelled` and still emit
    /// `Finished` - unlike the never-dequeued-under-batch-cancel silence
    /// (D16), it must NOT emit `Started`, since a GUI needs the row-level
    /// confirmation that its cancel click took effect.
    #[test]
    fn cancel_job_before_dequeue_skips_start_but_still_reports_finished() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        ctl.cancel_job(2);
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(outcomes[0].state, JobState::Ok);
        assert_eq!(outcomes[1].state, JobState::Ok);
        assert_eq!(outcomes[2].state, JobState::Cancelled);

        let started: Vec<usize> = events
            .iter()
            .filter_map(|e| match e {
                JobEvent::Started { index, .. } => Some(*index),
                _ => None,
            })
            .collect();
        assert_eq!(started, vec![0, 1], "job 2 must never emit Started");

        let finished_2_cancelled = events.iter().any(|e| {
            matches!(
                e,
                JobEvent::Finished { index: 2, outcome } if outcome.state == JobState::Cancelled
            )
        });
        assert!(
            finished_2_cancelled,
            "job 2 must still emit Finished{{Cancelled}} for GUI confirmation"
        );
        assert_eq!(
            fake.spawned().len(),
            2,
            "the cancelled job must never spawn"
        );
    }

    /// A [`Spawn`] used by [`cancel_job_kills_exactly_that_job_others_continue`]:
    /// spec index 0 gates on a [`Gate`] like [`GatedFakeSpawner`]'s job (so
    /// the test can synchronize on it being genuinely in flight before
    /// cancelling it); every other index spawns a quick scripted success,
    /// so the rest of the batch is unaffected by killing index 0
    /// specifically.
    struct SelectiveGateSpawner {
        kills: Arc<AtomicUsize>,
        ready: Sender<()>,
    }

    impl Spawn for SelectiveGateSpawner {
        fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            let index: usize = argv[0].parse().expect("test spec index encoded in argv[0]");
            if index == 0 {
                Ok(Box::new(GatedJob {
                    gate: Arc::new(Gate::new()),
                    kills: Arc::clone(&self.kills),
                    ready: self.ready.clone(),
                }))
            } else {
                Ok(Box::new(ScriptedJob {
                    lines: vec!["#GUI#progress 100%".to_string()],
                    cursor: 0,
                    exit: Some(0),
                }))
            }
        }
    }

    /// D25 behavior 2 ("kill in-flight"): [`QueueControl::cancel_job`] on an
    /// in-flight job kills exactly that job and only that job; a
    /// concurrently running job and a still-queued job are unaffected and
    /// the batch continues (unlike batch cancel, D16), and the killed job's
    /// partial output is deleted (D17).
    #[test]
    fn cancel_job_kills_exactly_that_job_others_continue() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        std::fs::write(&specs[0].output, b"partial").unwrap();

        let kills = Arc::new(AtomicUsize::new(0));
        let (ready_tx, ready_rx) = mpsc::channel();
        let fake = SelectiveGateSpawner {
            kills: Arc::clone(&kills),
            ready: ready_tx,
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 2,
            fail_fast: false,
        };

        let outcomes = std::thread::scope(|scope| {
            let handle = scope.spawn(|| run_queue(&specs, &fake, opts, &ctl, &tx));
            ready_rx
                .recv_timeout(Duration::from_secs(10))
                .expect("gated job never reached its read loop");
            ctl.cancel_job(0);
            handle.join().expect("run_queue thread panicked")
        });
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(outcomes[0].state, JobState::Cancelled);
        assert_eq!(outcomes[1].state, JobState::Ok);
        assert_eq!(outcomes[2].state, JobState::Ok);
        assert!(
            !specs[0].output.exists(),
            "the killed job's partial must be deleted (D17)"
        );
        assert_eq!(
            kills.load(Ordering::SeqCst),
            1,
            "exactly one kill, for the targeted job only"
        );
    }

    /// Wraps a [`FakeSpawner`] so `spawn` parks in the exact D25 race
    /// window: after `run_job`'s pre-spawn check has passed (`run_job`
    /// only calls `spawn` at all once that check returned false) and
    /// before [`RegisteringSpawner`] can insert the killer (insertion
    /// happens only after this returns). It signals the test over `ready`,
    /// then blocks until `release` fires, so the test can call
    /// [`QueueControl::cancel_job`] inside the window deterministically -
    /// no sleeps, no timing bets.
    struct MidSpawnGateSpawner<'a> {
        inner: &'a FakeSpawner,
        ready: Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }

    impl Spawn for MidSpawnGateSpawner<'_> {
        fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            let job = self.inner.spawn(argv)?;
            let _ = self.ready.send(());
            let _ = self.release.lock().unwrap().recv();
            Ok(job)
        }
    }

    /// D25 lost-cancellation race: `cancel_job` fired while the process is
    /// mid-spawn (pre-spawn check already passed, killer not yet
    /// registered) finds no killer to invoke; without a post-registration
    /// re-check the flag alone changes nothing, because the process exits
    /// normally (`Some(0)`) and the `None if cancelled()` arm never fires -
    /// the cancel request would be silently dropped and the job would
    /// complete `Ok`. The fix re-checks `job_cancelled` in
    /// [`RegisteringSpawner::spawn`] right after inserting the killer and
    /// kills the fresh process itself if set.
    #[test]
    fn cancel_job_during_spawn_window_is_not_lost() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![spec(0, dir.path().join("a.mkv"))];
        std::fs::write(&specs[0].output, b"partial").unwrap();
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let spawner = MidSpawnGateSpawner {
            inner: &fake,
            ready: ready_tx,
            release: Mutex::new(release_rx),
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: false,
        };

        let outcomes = std::thread::scope(|scope| {
            let handle = scope.spawn(|| run_queue(&specs, &spawner, opts, &ctl, &tx));
            ready_rx
                .recv_timeout(Duration::from_secs(10))
                .expect("spawn never reached the race window");
            // The killer for job 0 is provably not registered yet (the
            // spawner is still parked inside spawn), so this cancel_job
            // finds nothing to kill - the exact lost-cancellation window.
            ctl.cancel_job(0);
            release_tx.send(()).expect("run_queue thread hung up early");
            handle.join().expect("run_queue thread panicked")
        });
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(
            outcomes[0].state,
            JobState::Cancelled,
            "a cancel_job landing mid-spawn must not be silently dropped"
        );
        assert!(
            !specs[0].output.exists(),
            "the killed job's partial must be deleted (D17)"
        );
    }

    // A tiny batch with a wildly oversized `--jobs` must not spawn one OS
    // thread per requested worker: `worker_count` is the extracted, pure cap
    // decision (no threads involved), unit-tested directly here.

    #[test]
    fn worker_count_is_capped_at_spec_count() {
        assert_eq!(worker_count(100_000, 2), 2);
        assert_eq!(worker_count(1, 2), 1);
        assert_eq!(
            worker_count(0, 2),
            1,
            "jobs is clamped to >= 1 before the cap is applied"
        );
        assert_eq!(
            worker_count(3, 0),
            1,
            "an empty batch still gets one (idle) worker slot"
        );
    }

    #[test]
    fn jobs_far_exceeding_spec_count_still_completes_with_correct_outcomes() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
        ];
        let tracker = ConcurrencyTracker::new();
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0))
            .with_concurrency_tracker(Arc::clone(&tracker));
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 100_000,
            fail_fast: false,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let _events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(outcomes.len(), 2);
        for outcome in &outcomes {
            assert_eq!(outcome.state, JobState::Ok);
        }
        assert!(
            tracker.max() <= 2,
            "at most one worker per spec can ever be concurrently in \
             flight, observed max concurrency {}",
            tracker.max()
        );
    }

    /// `soft_fail_fast_cancels_queued_but_not_inflight` only exercises a
    /// failure at spec index 0 -- the FIRST spec ever dequeued -- so it
    /// cannot tell a `stop.store(true, ...)` genuinely gated on "the most
    /// recently finished job failed" apart from one accidentally gated on
    /// "index == 0". Job 0 succeeds, job 1 (the second dequeued, not the
    /// first) fails, job 2 must never spawn.
    #[test]
    fn fail_fast_triggers_on_a_non_first_failing_job() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![
            spec(0, dir.path().join("a.mkv")),
            spec(1, dir.path().join("b.mkv")),
            spec(2, dir.path().join("c.mkv")),
        ];
        let fake = ScriptByIndexSpawner {
            scripts: vec![
                (vec!["#GUI#progress 100%".to_string()], Some(0)),
                (vec!["#GUI#error boom".to_string()], Some(2)),
                (vec!["#GUI#progress 100%".to_string()], Some(0)),
            ],
        };
        let ctl = QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)));
        let (tx, rx) = mpsc::channel();
        let opts = QueueOpts {
            jobs: 1,
            fail_fast: true,
        };

        let outcomes = run_queue(&specs, &fake, opts, &ctl, &tx);
        drop(tx);
        let events: Vec<JobEvent> = rx.iter().collect();

        assert_eq!(
            outcomes.iter().map(|o| o.state).collect::<Vec<_>>(),
            vec![JobState::Ok, JobState::Failed, JobState::Cancelled],
            "fail-fast must trigger on the failing job regardless of its \
             position in the queue, not only when it is dequeued first"
        );
        let started: Vec<usize> = events
            .iter()
            .filter_map(|e| match e {
                JobEvent::Started { index, .. } => Some(*index),
                _ => None,
            })
            .collect();
        assert_eq!(
            started,
            vec![0, 1],
            "job 2, queued after the non-first failure, must never start"
        );
    }
}
