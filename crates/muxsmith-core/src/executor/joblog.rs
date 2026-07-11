//! Persisted job logs (spec 6, D26): `RunLogger` writes one `job-<index>.json`
//! per job, incrementally as [`JobEvent`]s arrive, plus a `summary.json` (the
//! `report::json::run_document`, taken verbatim) once the batch finishes,
//! under `<platform-data-dir>/muxsmith/runs/<run-id>/`.
//!
//! **Single-threaded by design.** A [`RunLogger`] carries no synchronization
//! (no `Mutex`) because it is meant to be driven from exactly one thread: the
//! caller's own event-drain loop (the CLI's `run` command drains its
//! `mpsc::Receiver<JobEvent>` on a single thread today; a future GUI surface
//! must do the same). Spec 6 requires persistence to be unconditional for
//! every surface that runs jobs, so every such drain loop must tee its events
//! through one `RunLogger` via [`RunLogger::on_event`] -- there is no
//! supported way to share one `RunLogger` across threads instead.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;
use time::OffsetDateTime;
use time::format_description::{BorrowedFormatItem, Component, modifier};

use super::job::{JobSpec, JobState};
use super::queue::JobEvent;

/// Hand-built format description for [`make_run_id`]'s "YYYYMMDD-HHMMSSZ"
/// (UTC) stamp: `Component` values plus literals, assembled directly rather
/// than through the `format_description!` macro or runtime string parsing,
/// since only the `formatting` cargo feature is pinned (not `macros` or
/// `parsing`).
const RUN_ID_FORMAT: &[BorrowedFormatItem<'_>] = &[
    BorrowedFormatItem::Component(Component::CalendarYearFullStandardRange(
        modifier::CalendarYearFullStandardRange::default(),
    )),
    BorrowedFormatItem::Component(Component::MonthNumerical(
        modifier::MonthNumerical::default(),
    )),
    BorrowedFormatItem::Component(Component::Day(modifier::Day::default())),
    BorrowedFormatItem::StringLiteral("-"),
    BorrowedFormatItem::Component(Component::Hour24(modifier::Hour24::default())),
    BorrowedFormatItem::Component(Component::Minute(modifier::Minute::default())),
    BorrowedFormatItem::Component(Component::Second(modifier::Second::default())),
    BorrowedFormatItem::StringLiteral("Z"),
];

/// The D26 default location: `dirs::data_dir()?/muxsmith/runs`. `None` when
/// the platform data dir itself cannot be resolved (e.g. no `HOME`); callers
/// treat that exactly like a [`RunLogger::create`] failure (spec 6: a mux run
/// never dies for a log dir).
pub fn default_runs_root() -> Option<PathBuf> {
    dirs::data_dir().map(|dir| dir.join("muxsmith").join("runs"))
}

/// Formats `now` as a UTC run-id stamp, `"YYYYMMDD-HHMMSSZ"`. Takes the
/// timestamp as a parameter rather than calling `SystemTime::now()`
/// internally, so the caller (`SystemTime::now()` in production) can pass a
/// fixed value in tests.
pub fn make_run_id(now: std::time::SystemTime) -> String {
    OffsetDateTime::from(now)
        .format(RUN_ID_FORMAT)
        .expect("RUN_ID_FORMAT is a fixed, well-formed descriptor; formatting a valid OffsetDateTime never fails")
}

/// The inverse of [`make_run_id`]: parses its fixed `"YYYYMMDD-HHMMSSZ"`
/// (UTC) prefix -- the first 16 bytes, present even on a collision-suffixed
/// name like `"...Z-2"` (see [`RunLogger::create`]) -- back into the
/// instant it encodes. `None` for anything that does not match, including a
/// digit-shaped but out-of-range calendar/clock value (e.g. month `13`):
/// such a name is not one this crate itself produced, exactly like a name
/// that fails the digit-shape check outright -- [`prune_stale_runs`] relies
/// on that to never delete a directory it did not create.
///
/// Moved here from the shell's own `started_at_from_run_id` (D35): both the
/// GUI's run history and this module's pruning need the same parse, so it
/// lives once in core and the shell delegates to it.
pub fn run_id_timestamp(name: &str) -> Option<OffsetDateTime> {
    let prefix = name.get(0..16)?;
    let b = prefix.as_bytes();
    let all_digits = |r: std::ops::Range<usize>| b[r].iter().all(u8::is_ascii_digit);
    if b[8] != b'-' || b[15] != b'Z' || !all_digits(0..8) || !all_digits(9..15) {
        return None;
    }
    let n = |r: std::ops::Range<usize>| prefix[r].parse::<u32>().ok();
    let year = n(0..4)? as i32;
    let month = time::Month::try_from(u8::try_from(n(4..6)?).ok()?).ok()?;
    let day = u8::try_from(n(6..8)?).ok()?;
    let hour = u8::try_from(n(9..11)?).ok()?;
    let minute = u8::try_from(n(11..13)?).ok()?;
    let second = u8::try_from(n(13..15)?).ok()?;
    time::Date::from_calendar_date(year, month, day)
        .and_then(|date| date.with_hms(hour, minute, second))
        .map(time::PrimitiveDateTime::assume_utc)
        .ok()
}

/// D35 (Şenol 2026-07-11, fixed for v1 -- no setting, no CLI flag;
/// configurability parked as IDEAS #7): run logs are kept for 14 days.
const RUN_LOG_RETENTION: time::Duration = time::Duration::days(14);

/// Deletes run directories under `runs_root` older than the fixed 14-day
/// `RUN_LOG_RETENTION` window (D35). Called best-effort by [`RunLogger::create`]
/// before it creates the new run's own leaf directory, so pruning happens
/// on every run through the one shared entry point (D26's
/// core-writes-for-both-surfaces principle) rather than needing each
/// surface to remember it separately.
///
/// Age is decided by the directory NAME alone, via [`run_id_timestamp`],
/// never by filesystem mtime: a name this crate did not itself produce is
/// left alone unconditionally (not this crate's directory to judge), and a
/// name it did produce carries its own age regardless of whatever mtime a
/// copy, backup, or restore might have given the directory since.
///
/// Only actual directories are candidates: `DirEntry::file_type()` does not
/// follow symlinks, so a symlink is excluded outright, never handed to
/// `remove_dir_all` -- which would otherwise delete whatever it points at,
/// possibly well outside `runs_root`.
///
/// Every I/O error along the way (`read_dir` failing, a single entry's
/// `file_type()` failing, a `remove_dir_all` failing) is deliberately
/// IGNORED. Pruning is housekeeping, not part of a run's own contract, and
/// its only failure mode -- old logs surviving a little longer -- is
/// harmless; surfacing the error instead would mean delaying or failing a
/// run over a stale log directory the run itself does not depend on.
pub fn prune_stale_runs(runs_root: &Path, now: std::time::SystemTime) {
    let cutoff = OffsetDateTime::from(now) - RUN_LOG_RETENTION;
    let Ok(entries) = fs::read_dir(runs_root) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        let Some(started_at) = run_id_timestamp(&name) else {
            continue;
        };
        if started_at < cutoff {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

/// Formats the current UTC instant as RFC3339, for a job record's
/// `started_at`/`finished_at` fields.
fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .expect("Rfc3339 formatting of the current UTC time never fails")
}

/// Per-job state accumulated across events until `Finished` writes it out.
/// `argv`/`output` are seeded from the [`JobSpec`] at [`RunLogger::create`]
/// time (a spec that later receives zero events, e.g. never dequeued under
/// batch cancellation, D16, simply keeps its accumulator unwritten -- see
/// [`RunLogger::finish`]'s doc).
struct JobAccumulator {
    argv: Vec<String>,
    output: PathBuf,
    lines: Vec<String>,
    started_at: Option<String>,
}

/// One `job-<index>.json` record (D26 shape); assembled from a finished
/// job's [`JobAccumulator`] plus its terminal outcome fields, carried
/// directly on `JobEvent::Finished`.
#[derive(Serialize)]
struct JobRecord<'a> {
    index: usize,
    output: String,
    argv: &'a [String],
    state: JobState,
    exit_code: Option<i32>,
    warnings: &'a [String],
    errors: &'a [String],
    duration_ms: u64,
    lines: &'a [String],
    started_at: Option<String>,
    finished_at: String,
}

/// Accumulates one run's job logs from a stream of [`JobEvent`]s and writes
/// them to disk (D26). See the module doc for the single-threaded-writer
/// invariant.
pub struct RunLogger {
    dir: PathBuf,
    jobs: HashMap<usize, JobAccumulator>,
    /// Set when any `job-<index>.json` write fails in [`Self::on_event`]
    /// (which has no return channel of its own); [`Self::finish`] surfaces
    /// it so a run whose per-job logs are silently incomplete never reports
    /// clean persistence.
    had_write_error: bool,
}

impl RunLogger {
    /// Creates the run's log directory under `runs_root`, named `run_id`
    /// (`mkdir -p runs_root` first, then the leaf directory itself). A
    /// leaf-name collision (a directory of that name already exists, e.g. a
    /// prior run with the same-second `run_id`) appends a numeric suffix:
    /// `<run_id>-2`, then `<run_id>-3`, and so on, until an unused name is
    /// found.
    ///
    /// Also runs [`prune_stale_runs`] (D35) against `runs_root` first,
    /// best-effort, before the new leaf is created -- see its doc for the
    /// retention window and failure handling.
    ///
    /// `specs` seeds one [`JobAccumulator`] per index (its `argv`/`output`),
    /// so a job's identity is known even if it never receives a single
    /// event before the batch ends.
    pub fn create(runs_root: &Path, run_id: &str, specs: &[JobSpec]) -> io::Result<RunLogger> {
        fs::create_dir_all(runs_root)?;

        prune_stale_runs(runs_root, std::time::SystemTime::now());

        let mut dir = runs_root.join(run_id);
        let mut suffix = 1u32;
        loop {
            match fs::create_dir(&dir) {
                Ok(()) => break,
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                    suffix += 1;
                    dir = runs_root.join(format!("{run_id}-{suffix}"));
                }
                Err(e) => return Err(e),
            }
        }

        let jobs = specs
            .iter()
            .enumerate()
            .map(|(index, spec)| {
                (
                    index,
                    JobAccumulator {
                        argv: spec.argv.clone(),
                        output: spec.output.clone(),
                        lines: Vec::new(),
                        started_at: None,
                    },
                )
            })
            .collect();

        Ok(RunLogger {
            dir,
            jobs,
            had_write_error: false,
        })
    }

    /// The run's actual log directory (post-collision-suffix).
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Feeds one [`JobEvent`] into the logger: `Started` records the start
    /// timestamp, `Output` appends the raw line (D24: every non-progress-tick
    /// line, verbatim; core attaches no meaning to it), `Finished` writes
    /// `job-<index>.json` and drops that job's accumulator. Every other
    /// variant (`Progress`/`Warning`/`Error`) is a no-op here: `Progress` is
    /// transient and `Warning`/`Error` text already reaches the record two
    /// other ways (verbatim inside `lines`, and structured in the `Finished`
    /// outcome's own `warnings`/`errors`).
    ///
    /// A job that never receives `Finished` at all (never dequeued under a
    /// batch cancel, D16 -- it emits no events whatsoever) never gets a
    /// `job-<index>.json`; only `summary.json` ([`Self::finish`]) covers it,
    /// via the `run_document`'s own `jobs` array (index-aligned to every
    /// outcome, unconditionally).
    ///
    /// A write failure (e.g. disk full) never interrupts the run --
    /// `on_event` returns nothing (the run itself is never at risk over a
    /// log write) -- but it is not lost either: the logger remembers it and
    /// [`Self::finish`] returns `Err`, so the caller can report the run's
    /// logs as incomplete instead of claiming clean persistence.
    pub fn on_event(&mut self, ev: &JobEvent) {
        match ev {
            JobEvent::Started { index, .. } => {
                if let Some(job) = self.jobs.get_mut(index) {
                    job.started_at = Some(now_rfc3339());
                }
            }
            JobEvent::Output { index, line } => {
                if let Some(job) = self.jobs.get_mut(index) {
                    job.lines.push(line.clone());
                }
            }
            JobEvent::Finished { index, outcome } => {
                if let Some(job) = self.jobs.remove(index) {
                    let record = JobRecord {
                        index: *index,
                        output: job.output.display().to_string(),
                        argv: &job.argv,
                        state: outcome.state,
                        exit_code: outcome.exit_code,
                        warnings: &outcome.warnings,
                        errors: &outcome.errors,
                        duration_ms: outcome.duration_ms,
                        lines: &job.lines,
                        started_at: job.started_at,
                        finished_at: now_rfc3339(),
                    };
                    let path = self.dir.join(format!("job-{index}.json"));
                    let written = serde_json::to_vec_pretty(&record)
                        .map_err(io::Error::from)
                        .and_then(|bytes| fs::write(path, bytes));
                    if written.is_err() {
                        self.had_write_error = true;
                    }
                }
            }
            JobEvent::Progress { .. } | JobEvent::Warning { .. } | JobEvent::Error { .. } => {}
        }
    }

    /// Writes `run_document` (the T2 `report::json::run_document`, passed in
    /// verbatim -- this module never rebuilds it) to `summary.json` and
    /// returns the run's log directory. Consumes `self`: once a run has
    /// finished, its accumulators (any left over from jobs that never
    /// received `Finished`, see [`Self::on_event`]) are simply dropped.
    ///
    /// Returns `Err` not only when the `summary.json` write itself fails,
    /// but also when any earlier `job-<index>.json` write failed inside
    /// [`Self::on_event`] (which has no return channel of its own):
    /// `summary.json` is still written first in that case (best-effort:
    /// persist what we can, then signal), so a caller seeing this error
    /// knows the run's log directory exists but is incomplete, and must not
    /// report clean persistence.
    pub fn finish(self, run_document: &serde_json::Value) -> io::Result<PathBuf> {
        fs::write(
            self.dir.join("summary.json"),
            serde_json::to_vec_pretty(run_document)?,
        )?;
        if self.had_write_error {
            return Err(io::Error::other(
                "one or more job-<index>.json writes failed during the run",
            ));
        }
        Ok(self.dir)
    }
}
