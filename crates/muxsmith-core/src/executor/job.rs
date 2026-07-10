//! Per-job runner (spec 6, D13): spawns one mkvmerge invocation behind the
//! [`Spawn`] seam, parses the observed `--gui-mode` line grammar into
//! [`JobProgress`], and maps the process exit into a terminal [`JobState`].
//! Delete-partial (D17) removes the output on `Failed`/`Cancelled` only,
//! deliberately diverging from mkvtoolnix-gui's opt-in default-off behavior.

use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::Serialize;

use super::spawn::{Spawn, SpawnError};

/// What to execute: the pure argv plus the output path the argv writes
/// (needed for parent-dir creation and delete-partial).
#[derive(Debug, Clone, PartialEq)]
pub struct JobSpec {
    /// `command(&Plan)` vector (no program name, no `--gui-mode`).
    pub argv: Vec<String>,
    /// The plan's rendered output path.
    pub output: PathBuf,
}

/// Terminal job state (spec 6; mirrors mkvtoolnix-gui DoneOk/DoneWarnings/
/// Failed/Aborted, D13).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    /// mkvmerge exited 0: clean mux, output kept.
    Ok,
    /// mkvmerge exited 1: mux completed with warnings, output kept.
    Warning,
    /// mkvmerge exited 2, or any other non-zero or abnormal exit: mux
    /// failed, partial output deleted (D17).
    Failed,
    /// Killed while the caller's cancellation flag was set; partial output
    /// deleted (D17).
    Cancelled,
}

/// One finished job.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JobOutcome {
    /// Terminal state.
    pub state: JobState,
    /// mkvmerge's exit code; `None` when killed (or when the process never
    /// started, see [`Spawn::spawn`] errors).
    pub exit_code: Option<i32>,
    /// Captured warning lines (tag stripped).
    pub warnings: Vec<String>,
    /// Captured error lines (tag stripped).
    pub errors: Vec<String>,
    /// Wall-clock duration of the run, in milliseconds.
    pub duration_ms: u64,
}

/// Mid-job signal surfaced to the caller (the queue re-emits as JobEvent).
#[derive(Debug, Clone, PartialEq)]
pub enum JobProgress {
    /// Parsed `#GUI#progress NN%` line.
    Percent(u8),
    /// A captured `#GUI#warning` line, tag stripped.
    WarningLine(String),
    /// A captured `#GUI#error` line, tag stripped.
    ErrorLine(String),
    /// Any line mkvmerge wrote that is not a `#GUI#progress` tick, verbatim
    /// (tags included, so warning/error lines appear here too, in addition
    /// to their own tag-stripped [`JobProgress::WarningLine`] /
    /// [`JobProgress::ErrorLine`]). Feeds a live log pane and persisted job
    /// logs (D24); core attaches no meaning to the text itself.
    OutputLine(String),
}

/// Runs one job to completion: checks for pre-spawn cancellation first
/// (D25 - nothing is touched: no parent dir created, no spawn, no delete),
/// otherwise ensures the output's parent dir exists (D13), spawns, streams
/// lines through the gui-mode parser, and maps the exit code (0 ok / 1
/// warning, output kept / 2 or abnormal failed, partial deleted / killed
/// while `cancelled()` reports true = cancelled, partial deleted).
///
/// `cancelled` is a closure re-evaluated at each check point rather than a
/// single flag read once (D25): this lets a caller like the queue check one
/// specific job's own cancel state by index, instead of only a single
/// shared batch flag.
///
/// Drains `next_line` to EOF before calling `wait` (never the reverse): the
/// live `RunningJob::wait` holds the child mutex across a blocking waitpid,
/// so calling it while a concurrent `Killer` might still be needed to end a
/// streaming process would stall until natural exit.
pub fn run_job(
    spawner: &dyn Spawn,
    spec: &JobSpec,
    cancelled: &dyn Fn() -> bool,
    on_progress: &mut dyn FnMut(JobProgress),
) -> JobOutcome {
    let start = Instant::now();

    if cancelled() {
        // Cancelled before this job ever ran: like a spawn failure, no
        // process means no partial exists, and D25 additionally requires
        // that a pre-spawn cancel delete nothing at all - a pre-existing
        // output from an earlier run must survive untouched.
        return JobOutcome {
            state: JobState::Cancelled,
            exit_code: None,
            warnings: Vec::new(),
            errors: Vec::new(),
            duration_ms: start.elapsed().as_millis() as u64,
        };
    }

    if let Some(parent) = spec.output.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let mut running = match spawner.spawn(&spec.argv) {
        Ok(running) => running,
        // The process never ran, so mkvmerge wrote nothing: no partial
        // exists and nothing may be deleted (a file at `spec.output` is a
        // valid output from a prior run, e.g. under `on_collision:
        // overwrite`). Assembled directly, bypassing the deleting
        // [`finish`] path (D17 covers partials only).
        Err(SpawnError(message)) => {
            return JobOutcome {
                state: JobState::Failed,
                exit_code: None,
                warnings: Vec::new(),
                errors: vec![message],
                duration_ms: start.elapsed().as_millis() as u64,
            };
        }
    };

    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    while let Some(line) = running.next_line() {
        if let Some(pct) = parse_progress(&line) {
            on_progress(JobProgress::Percent(pct));
        } else {
            on_progress(JobProgress::OutputLine(line.clone()));
            if let Some(text) = line.strip_prefix("#GUI#warning ") {
                let text = text.to_string();
                warnings.push(text.clone());
                on_progress(JobProgress::WarningLine(text));
            } else if let Some(text) = line.strip_prefix("#GUI#error ") {
                let text = text.to_string();
                errors.push(text.clone());
                on_progress(JobProgress::ErrorLine(text));
            }
        }
    }

    let exit_code = running.wait();
    let state = match exit_code {
        Some(0) => JobState::Ok,
        Some(1) => JobState::Warning,
        None if cancelled() => JobState::Cancelled,
        _ => JobState::Failed,
    };

    finish(state, exit_code, warnings, errors, &spec.output, start)
}

/// Parses a `#GUI#progress NN%` line into `NN`; `None` for any other line.
fn parse_progress(line: &str) -> Option<u8> {
    line.strip_prefix("#GUI#progress ")
        .and_then(|rest| rest.strip_suffix('%'))
        .and_then(|digits| digits.parse().ok())
}

/// Assembles the terminal [`JobOutcome`] for a job whose process actually
/// ran, deleting the partial output (D17) when the state is `Failed` or
/// `Cancelled`. Spawn failures do not come through here: no process means
/// no partial, so nothing may be deleted.
fn finish(
    state: JobState,
    exit_code: Option<i32>,
    warnings: Vec<String>,
    mut errors: Vec<String>,
    output: &Path,
    start: Instant,
) -> JobOutcome {
    if matches!(state, JobState::Failed | JobState::Cancelled) {
        delete_partial(output, &mut errors);
    }
    JobOutcome {
        state,
        exit_code,
        warnings,
        errors,
        duration_ms: start.elapsed().as_millis() as u64,
    }
}

/// Best-effort removal of a partial output. `NotFound` (nothing was ever
/// written) is expected and silently ignored. Any other failure is
/// surfaced into `errors` as `delete_partial_failed: <io error>`: the
/// deliberate exception to core staying prose-free (spec 6/7), a
/// third-party I/O detail passed through verbatim rather than invented core
/// prose, since core otherwise has no channel back to the caller for a
/// delete failure on a mux that itself ran to completion (or was
/// cancelled).
fn delete_partial(output: &Path, errors: &mut Vec<String>) {
    if let Err(e) = std::fs::remove_file(output)
        && e.kind() != std::io::ErrorKind::NotFound
    {
        errors.push(format!("delete_partial_failed: {e}"));
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use crate::executor::spawn::{FakeSpawner, RunningJob};

    /// A [`Spawn`] whose `spawn` always fails, for the spawn-error path
    /// ([`FakeSpawner`] has no error mode).
    struct FailingSpawner;

    impl Spawn for FailingSpawner {
        fn spawn(&self, _argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
            Err(SpawnError("boom".to_string()))
        }
    }

    fn spec(output: PathBuf) -> JobSpec {
        JobSpec {
            argv: vec![
                "--output".to_string(),
                output.to_string_lossy().into_owned(),
            ],
            output,
        }
    }

    #[test]
    fn exit_zero_is_ok_and_output_kept() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.mkv");
        std::fs::write(&output, b"muxed").unwrap();
        let spec = spec(output);
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        let cancelled = || false;

        let outcome = run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Ok);
        assert_eq!(outcome.exit_code, Some(0));
        assert!(spec.output.exists());
    }

    #[test]
    fn exit_one_is_warning_with_captured_lines() {
        let dir = tempfile::tempdir().unwrap();
        let spec = spec(dir.path().join("out.mkv"));
        let fake = FakeSpawner::script(
            vec![
                "#GUI#warning 'seed.srt': A track with the ID 9 was requested but not found."
                    .to_string(),
            ],
            Some(1),
        );
        let cancelled = || false;

        let outcome = run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Warning);
        assert_eq!(outcome.exit_code, Some(1));
        assert_eq!(
            outcome.warnings,
            vec!["'seed.srt': A track with the ID 9 was requested but not found.".to_string()]
        );
    }

    #[test]
    fn exit_two_is_failed_and_partial_deleted() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.mkv");
        std::fs::write(&output, b"partial").unwrap();
        let spec = spec(output);
        let fake = FakeSpawner::script(
            vec!["#GUI#error The file 'missing.srt' could not be opened for reading.".to_string()],
            Some(2),
        );
        let cancelled = || false;

        let outcome = run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Failed);
        assert_eq!(outcome.exit_code, Some(2));
        assert!(!spec.output.exists());
    }

    #[test]
    fn killed_under_cancel_is_cancelled_and_partial_deleted() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.mkv");
        std::fs::write(&output, b"partial").unwrap();
        let spec = spec(output);
        let fake = FakeSpawner::script(vec!["#GUI#progress 50%".to_string()], None);
        // Not cancelled yet at the pre-spawn check (so the process actually
        // runs); becomes cancelled only afterward, mirroring the real
        // timeline of a job that starts, then gets killed mid-flight
        // (`Cell::replace` returns the old value, so the first call - the
        // pre-spawn check - reads `false`, and every call from then on -
        // the post-`wait()` check - reads `true`).
        let cancelled_after_spawn = Cell::new(false);
        let cancelled = || cancelled_after_spawn.replace(true);

        let outcome = run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Cancelled);
        assert_eq!(outcome.exit_code, None);
        assert!(!spec.output.exists());
    }

    #[test]
    fn progress_lines_surface_as_percent() {
        let dir = tempfile::tempdir().unwrap();
        let spec = spec(dir.path().join("out.mkv"));
        let fake = FakeSpawner::script(
            vec![
                "#GUI#progress 25%".to_string(),
                "#GUI#progress 50%".to_string(),
                "#GUI#progress 100%".to_string(),
            ],
            Some(0),
        );
        let cancelled = || false;
        let mut collected = Vec::new();

        run_job(&fake, &spec, &cancelled, &mut |p| collected.push(p));

        assert_eq!(
            collected,
            vec![
                JobProgress::Percent(25),
                JobProgress::Percent(50),
                JobProgress::Percent(100),
            ]
        );
    }

    #[test]
    fn spawn_failure_is_failed_but_keeps_preexisting_output() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.mkv");
        std::fs::write(&output, b"valid output from a prior run").unwrap();
        let spec = spec(output);
        let cancelled = || false;

        let outcome = run_job(&FailingSpawner, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Failed);
        assert_eq!(outcome.exit_code, None);
        assert_eq!(outcome.errors, vec!["boom".to_string()]);
        assert!(
            spec.output.exists(),
            "no process ran, so no partial exists; a pre-existing output must survive"
        );
    }

    #[test]
    fn parent_dir_created_before_spawn() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("nested/sub/out.mkv");
        let spec = spec(output);
        let fake = FakeSpawner::script(Vec::new(), Some(0));
        let cancelled = || false;

        run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert!(spec.output.parent().unwrap().is_dir());
    }

    /// D25 pre-spawn check (HANDOFF backlog item): a job whose `cancelled`
    /// closure already reports true before `run_job` ever calls the
    /// spawner must become `Cancelled` without spawning, without creating
    /// the output's parent dir, and - critically - without deleting
    /// anything (a pre-existing output from an earlier run must survive,
    /// exactly like the spawn-failure case above).
    #[test]
    fn pre_spawn_cancellation_skips_spawn_and_deletes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.mkv");
        std::fs::write(&output, b"valid output from a prior run").unwrap();
        let nested = spec(dir.path().join("never/created/out.mkv"));
        let spec = spec(output);
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        let cancelled = || true;

        let outcome = run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Cancelled);
        assert_eq!(outcome.exit_code, None);
        assert!(outcome.errors.is_empty());
        assert!(
            spec.output.exists(),
            "pre-spawn cancel must delete nothing (D25)"
        );

        // Zero filesystem touch, pinned directly: with a nested output
        // whose parent does not exist, a pre-spawn cancel must not even
        // create that parent directory (create_dir_all sits after the
        // check).
        let outcome = run_job(&fake, &nested, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Cancelled);
        assert!(
            !nested.output.parent().unwrap().exists(),
            "pre-spawn cancel must not create the output's parent dir"
        );
        assert!(
            fake.spawned().is_empty(),
            "the spawner must never be called once already cancelled"
        );
    }

    /// D25 delete_partial error surfacing (HANDOFF backlog item): a
    /// `remove_file` failure other than `NotFound` is pushed into
    /// `outcome.errors` as a `delete_partial_failed: <detail>` third-party
    /// passthrough, the one deliberate exception to core staying
    /// prose-free. A directory at the output path is a portable, no-perms
    /// way to force `remove_file` to fail with a non-`NotFound` error on
    /// every OS this runs on.
    #[test]
    fn delete_partial_failure_surfaces_into_errors() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("out.mkv");
        std::fs::create_dir(&output).unwrap();
        let spec = spec(output);
        let fake = FakeSpawner::script(
            vec!["#GUI#error The file 'missing.srt' could not be opened for reading.".to_string()],
            Some(2),
        );
        let cancelled = || false;

        let outcome = run_job(&fake, &spec, &cancelled, &mut |_| {});

        assert_eq!(outcome.state, JobState::Failed);
        assert!(
            outcome
                .errors
                .iter()
                .any(|e| e.starts_with("delete_partial_failed: ")),
            "expected a delete_partial_failed entry, got: {:?}",
            outcome.errors
        );
    }
}
