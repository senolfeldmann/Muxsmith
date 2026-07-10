//! The `Spawn` seam: abstracts starting one mkvmerge process so the job
//! runner and queue (D13) are unit-testable against a scripted fake, and the
//! live implementation carries the empirically confirmed `--gui-mode` line
//! grammar (spec 6, D7). Mirrors the `Identify` trait's live/fake split
//! (`crate::identify`).

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// Why a spawn error is not a job failure: failing to even start mkvmerge
/// (binary vanished, non-UTF-8 path) is an environment problem, reported
/// distinctly from a mux that ran and failed.
#[derive(Debug, Clone, PartialEq)]
pub struct SpawnError(pub String);

/// A clonable handle that kills its job when invoked (idempotent,
/// best-effort). Lives separately from [`RunningJob`] so the queue's
/// cancellation path can kill in-flight jobs it does not own.
pub type Killer = Arc<dyn Fn() + Send + Sync>;

/// Abstracts process execution so the job runner and queue are
/// unit-testable with a scripted fake, mirroring `Identify` (D7/D13).
pub trait Spawn {
    /// Spawns one mux with `argv` (the pure `command(&Plan)` vector; the
    /// live impl prepends `--gui-mode`).
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError>;
}

/// One running (or scripted) mkvmerge process.
pub trait RunningJob: Send {
    /// Next stdout line; `None` at EOF (or after a kill). Blocking.
    fn next_line(&mut self) -> Option<String>;
    /// Waits for exit; `None` exactly when this job's [`Killer`] was
    /// invoked, regardless of what the OS reports for the exit status.
    /// Guaranteed cross-platform: Windows' `TerminateProcess` always yields
    /// a `Some` exit code (unlike Unix's signal death), so an
    /// implementation must track kill state itself rather than trust the
    /// raw process status (D16/D17).
    fn wait(&mut self) -> Option<i32>;
    /// A [`Killer`] for this job.
    fn killer(&self) -> Killer;
}

/// The production [`Spawn`]: runs the resolved mkvmerge with `--gui-mode`
/// prepended for machine-readable progress.
///
/// Observed v100 `--gui-mode` line grammar (confirmed empirically against
/// the installed binary, not assumed from memory; verbatim examples, ready
/// as Task 2 parser-test fixtures):
///
/// - Progress: `#GUI#progress 100%`. Printed one or more times per run; a
///   longer mux also emits intermediate percentages, but the final line on
///   any successful run is always `#GUI#progress 100%`. General form:
///   `#GUI#progress NN%`.
/// - Warning (exit 1, output kept): `` #GUI#warning 'seed.srt': A track
///   with the ID 9 was requested but not found in the file. The
///   corresponding option will be ignored. `` General form: `#GUI#warning
///   '<file>': <message>` (single-quoted source file name, then message).
/// - Error (exit 2, no output produced): `#GUI#error The file
///   'does-not-exist.srt' could not be opened for reading: open file
///   error.`. General form: `#GUI#error <message>` (no leading quoted
///   filename, unlike warning).
pub struct LiveSpawner {
    /// Resolved mkvmerge binary path (from `Mkvmerge::path()`).
    pub mkvmerge: PathBuf,
}

impl Spawn for LiveSpawner {
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
        let mut child = Command::new(&self.mkvmerge)
            .arg("--gui-mode")
            .args(argv)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| SpawnError(e.to_string()))?;
        let stdout = child.stdout.take().expect("stdout was piped");
        Ok(Box::new(LiveJob {
            reader: BufReader::new(stdout),
            child: Arc::new(Mutex::new(child)),
            killed: Arc::new(AtomicBool::new(false)),
        }))
    }
}

/// A real, running mkvmerge child process. Not constructed directly; use
/// [`LiveSpawner::spawn`].
struct LiveJob {
    reader: BufReader<std::process::ChildStdout>,
    child: Arc<Mutex<Child>>,
    /// Set by this job's [`Killer`] before it signals the child (D16/D17):
    /// authoritative for `wait`'s `None`-when-killed contract, since a
    /// Windows `TerminateProcess`-killed child still reports a `Some` exit
    /// code from `Child::wait`, unlike Unix's signal death.
    killed: Arc<AtomicBool>,
}

impl RunningJob for LiveJob {
    fn next_line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(line.trim_end().to_string()),
        }
    }
    fn wait(&mut self) -> Option<i32> {
        let raw_code = self
            .child
            .lock()
            .unwrap()
            .wait()
            .ok()
            .and_then(|s| s.code());
        resolve_wait(self.killed.load(Ordering::SeqCst), raw_code)
    }
    fn killer(&self) -> Killer {
        let child = Arc::clone(&self.child);
        let killed = Arc::clone(&self.killed);
        Arc::new(move || {
            // Set before kill(): a concurrent `wait()` must never observe
            // "the process is gone" without also observing "the flag is
            // set" (D16/D17).
            killed.store(true, Ordering::SeqCst);
            let _ = child.lock().unwrap().kill();
        })
    }
}

/// Combines a job's kill flag with its raw OS exit code into the
/// [`RunningJob::wait`] contract (`None` exactly when killed). Extracted out
/// of [`LiveJob::wait`] so the decision is unit-testable without a real
/// child process: `raw_code` stands in for whatever the OS reports (Unix
/// `None` on signal death, Windows always `Some`), and the flag overrides it
/// either way.
fn resolve_wait(killed: bool, raw_code: Option<i32>) -> Option<i32> {
    if killed { None } else { raw_code }
}

/// Scripted fake for unit tests: yields the scripted lines, then EOF, then
/// the scripted exit code; records every argv it was asked to spawn.
pub struct FakeSpawner {
    lines: Vec<String>,
    exit: Option<i32>,
    spawned: Mutex<Vec<Vec<String>>>,
    tracker: Option<Arc<ConcurrencyTracker>>,
}

impl FakeSpawner {
    /// Builds a fake that, on every `spawn`, yields `lines` in order and
    /// then reports `exit` (`None` mimics a killed process with no code).
    pub fn script(lines: Vec<String>, exit: Option<i32>) -> FakeSpawner {
        FakeSpawner {
            lines,
            exit,
            spawned: Mutex::new(Vec::new()),
            tracker: None,
        }
    }

    /// The argv of every `spawn` call so far, in call order.
    pub fn spawned(&self) -> Vec<Vec<String>> {
        self.spawned.lock().unwrap().clone()
    }

    /// Attaches a [`ConcurrencyTracker`] so every job this fake spawns
    /// increments it on `spawn` and decrements it on `wait` (Task 3 test
    /// support for the queue's `--jobs N` bound).
    pub fn with_concurrency_tracker(mut self, tracker: Arc<ConcurrencyTracker>) -> FakeSpawner {
        self.tracker = Some(tracker);
        self
    }
}

impl Spawn for FakeSpawner {
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
        self.spawned.lock().unwrap().push(argv.to_vec());
        if let Some(tracker) = &self.tracker {
            tracker.enter();
        }
        Ok(Box::new(FakeJob {
            lines: self.lines.clone(),
            cursor: 0,
            exit: self.exit,
            killed: Arc::new(AtomicBool::new(false)),
            tracker: self.tracker.clone(),
        }))
    }
}

/// A scripted [`RunningJob`] backed by [`FakeSpawner`]. Not constructed
/// directly; use `FakeSpawner::spawn`.
struct FakeJob {
    lines: Vec<String>,
    cursor: usize,
    exit: Option<i32>,
    killed: Arc<AtomicBool>,
    tracker: Option<Arc<ConcurrencyTracker>>,
}

impl RunningJob for FakeJob {
    fn next_line(&mut self) -> Option<String> {
        if self.killed.load(Ordering::SeqCst) || self.cursor >= self.lines.len() {
            return None;
        }
        let line = self.lines[self.cursor].clone();
        self.cursor += 1;
        Some(line)
    }
    fn wait(&mut self) -> Option<i32> {
        if let Some(tracker) = &self.tracker {
            tracker.exit();
        }
        if self.killed.load(Ordering::SeqCst) {
            None
        } else {
            self.exit
        }
    }
    fn killer(&self) -> Killer {
        let killed = Arc::clone(&self.killed);
        Arc::new(move || killed.store(true, Ordering::SeqCst))
    }
}

/// Live/max concurrency counter, attached to a [`FakeSpawner`] via
/// [`FakeSpawner::with_concurrency_tracker`] (Task 3 test support): `current`
/// is incremented in `spawn` and decremented in `wait`, mirroring the window
/// during which a real job is in flight, and `max` records the high-water
/// mark so a `--jobs N` bound is observable from a test.
///
/// Test instrumentation only, not a supported API: hidden from rustdoc
/// (pre-go-public decision); kept `pub` because cross-crate tests consume it.
#[doc(hidden)]
#[derive(Default)]
pub struct ConcurrencyTracker {
    current: AtomicUsize,
    max: AtomicUsize,
}

impl ConcurrencyTracker {
    /// A fresh tracker starting at zero concurrent jobs.
    pub fn new() -> Arc<ConcurrencyTracker> {
        Arc::new(ConcurrencyTracker::default())
    }

    /// The highest concurrent-job count observed so far.
    pub fn max(&self) -> usize {
        self.max.load(Ordering::SeqCst)
    }

    fn enter(&self) {
        let now = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        self.max.fetch_max(now, Ordering::SeqCst);
    }

    fn exit(&self) {
        self.current.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_spawner_scripts_lines_and_exit() {
        let fake = FakeSpawner::script(vec!["#GUI#progress 50%".into()], Some(0));
        let mut job = fake.spawn(&["--output".into(), "x.mkv".into()]).unwrap();
        assert_eq!(job.next_line().as_deref(), Some("#GUI#progress 50%"));
        assert_eq!(job.next_line(), None);
        assert_eq!(job.wait(), Some(0));
        assert_eq!(
            fake.spawned(),
            vec![vec!["--output".to_string(), "x.mkv".into()]]
        );
    }

    #[test]
    fn fake_killer_ends_stream_and_wait_returns_none() {
        let fake = FakeSpawner::script(vec!["line".to_string(); 100], Some(0));
        let mut job = fake.spawn(&[]).unwrap();
        let kill = job.killer();
        job.next_line();
        kill();
        assert_eq!(job.next_line(), None);
        assert_eq!(job.wait(), None);
    }

    // D16/D17 Windows fix: `TerminateProcess` always yields a `Some` exit
    // code (unlike Unix's signal death), so `LiveJob::wait` cannot trust the
    // raw OS status alone. `resolve_wait` is the extracted flag+raw-code
    // decision, unit-tested directly here since a real child process cannot
    // be driven into that state deterministically (confirmed empirically:
    // `std::process::Child::kill` sends SIGKILL on Unix, so the process
    // always dies by signal here regardless of the flag - see
    // `live_killer_then_wait_returns_none` below for the live-wiring check).

    #[test]
    fn resolve_wait_returns_none_when_killed_even_if_the_os_reports_a_code() {
        assert_eq!(resolve_wait(true, Some(1)), None);
        assert_eq!(resolve_wait(true, Some(0)), None);
        assert_eq!(resolve_wait(true, None), None);
    }

    #[test]
    fn resolve_wait_passes_the_raw_code_through_when_not_killed() {
        assert_eq!(resolve_wait(false, Some(0)), Some(0));
        assert_eq!(resolve_wait(false, Some(2)), Some(2));
        assert_eq!(resolve_wait(false, None), None);
    }

    /// D16/D17 regression, live wiring: a killed [`LiveJob`] must report
    /// `wait() == None`. Uses a scripted fake `mkvmerge` (not a real one) so
    /// the test is deterministic and self-contained; the script sleeps
    /// after printing one line so the process is provably alive at kill
    /// time, mirroring `crates/muxsmith-cli/tests/run_cli.rs`'s
    /// `fake_mkvmerge_that_fails_queries` stub pattern. Cannot itself
    /// distinguish the fixed code from the pre-fix code on Unix (kill()
    /// sends SIGKILL, an untrappable signal death that already yields
    /// `None` from the raw exit status alone) - `resolve_wait`'s unit tests
    /// above are what pin the flag's semantics; this test guards that the
    /// flag is actually wired into `LiveJob`.
    #[cfg(unix)]
    #[test]
    fn live_killer_then_wait_returns_none() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("fake-mkvmerge");
        std::fs::write(&script, "#!/bin/sh\necho started\nsleep 30\n").unwrap();
        let mut perms = std::fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script, perms).unwrap();

        let spawner = LiveSpawner { mkvmerge: script };
        let mut job = spawner.spawn(&[]).unwrap();

        // Proves the process is genuinely alive (not a spawn race) before
        // it is killed.
        assert_eq!(job.next_line().as_deref(), Some("started"));

        let kill = job.killer();
        kill();

        assert_eq!(job.wait(), None);
    }
}
