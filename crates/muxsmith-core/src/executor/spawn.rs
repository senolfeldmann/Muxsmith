//! The `Spawn` seam: abstracts starting one mkvmerge process so the job
//! runner and queue (D13) are unit-testable against a scripted fake, and the
//! live implementation carries the empirically confirmed `--gui-mode` line
//! grammar (spec 6, D7). Mirrors the `Identify` trait's live/fake split
//! (`crate::identify`).

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
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
    /// Waits for exit; `None` when the process died without a code (killed).
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
        }))
    }
}

/// A real, running mkvmerge child process. Not constructed directly; use
/// [`LiveSpawner::spawn`].
struct LiveJob {
    reader: BufReader<std::process::ChildStdout>,
    child: Arc<Mutex<Child>>,
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
        self.child
            .lock()
            .unwrap()
            .wait()
            .ok()
            .and_then(|s| s.code())
    }
    fn killer(&self) -> Killer {
        let child = Arc::clone(&self.child);
        Arc::new(move || {
            let _ = child.lock().unwrap().kill();
        })
    }
}

/// Scripted fake for unit tests: yields the scripted lines, then EOF, then
/// the scripted exit code; records every argv it was asked to spawn.
pub struct FakeSpawner {
    lines: Vec<String>,
    exit: Option<i32>,
    spawned: Mutex<Vec<Vec<String>>>,
}

impl FakeSpawner {
    /// Builds a fake that, on every `spawn`, yields `lines` in order and
    /// then reports `exit` (`None` mimics a killed process with no code).
    pub fn script(lines: Vec<String>, exit: Option<i32>) -> FakeSpawner {
        FakeSpawner {
            lines,
            exit,
            spawned: Mutex::new(Vec::new()),
        }
    }

    /// The argv of every `spawn` call so far, in call order.
    pub fn spawned(&self) -> Vec<Vec<String>> {
        self.spawned.lock().unwrap().clone()
    }
}

impl Spawn for FakeSpawner {
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
        self.spawned.lock().unwrap().push(argv.to_vec());
        Ok(Box::new(FakeJob {
            lines: self.lines.clone(),
            cursor: 0,
            exit: self.exit,
            killed: Arc::new(AtomicBool::new(false)),
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
}
