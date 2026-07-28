//! Persisted job logs (`executor::joblog`, D26, Task 6): `RunLogger` writes
//! one `job-<index>.json` per job (incrementally, as events arrive) plus a
//! `summary.json` (the T2 `run_document`, taken verbatim) when the run
//! finishes. All tests use a tempdir as `runs_root`, never the real platform
//! data dir.

use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use muxsmith_core::executor::job::{JobOutcome, JobSpec, JobState};
use muxsmith_core::executor::joblog::{
    RunLogger, default_runs_root, make_run_id, prune_stale_runs, run_id_timestamp,
};
use muxsmith_core::executor::queue::JobEvent;

fn spec(argv: &[&str], output: &str) -> JobSpec {
    JobSpec {
        argv: argv.iter().map(|s| s.to_string()).collect(),
        output: PathBuf::from(output),
    }
}

fn outcome(state: JobState, exit_code: Option<i32>) -> JobOutcome {
    JobOutcome {
        state,
        exit_code,
        warnings: Vec::new(),
        errors: Vec::new(),
        duration_ms: 42,
        panic: None,
    }
}

/// `make_run_id` (UTC "YYYYMMDD-HHMMSSZ" via the `time` crate, from an
/// injected `SystemTime` rather than an internal `now()` call, so it is
/// deterministic here): the Unix epoch itself, and an offset from it that
/// exercises zero-padding on every single-digit component.
#[test]
fn make_run_id_formats_utc_as_compact_stamp() {
    assert_eq!(make_run_id(SystemTime::UNIX_EPOCH), "19700101-000000Z");
    assert_eq!(
        make_run_id(SystemTime::UNIX_EPOCH + Duration::new(3723, 0)),
        "19700101-010203Z",
        "1h2m3s after the epoch must zero-pad every component"
    );
}

/// `default_runs_root` (D26 location): `dirs::data_dir()?/muxsmith/runs`.
/// Lenient on `dirs::data_dir()` itself returning `None` (sandboxed CI with
/// no resolvable data dir), but whenever it resolves, the suffix must be
/// exactly `muxsmith/runs`.
#[test]
fn default_runs_root_appends_muxsmith_runs() {
    if let Some(root) = default_runs_root() {
        assert!(
            root.ends_with(PathBuf::from("muxsmith").join("runs")),
            "expected a muxsmith/runs suffix, got: {root:?}"
        );
    }
}

/// Step 1, case 1: `create` -> `on_event(Started/Output/Finished)` ->
/// `finish` yields exactly `job-0.json` + `summary.json`, with the full
/// field set the brief specifies.
#[test]
fn full_lifecycle_writes_job_and_summary_files() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    let specs = vec![spec(&["--output", "out.mkv", "a.srt"], "out.mkv")];

    let mut logger = RunLogger::create(&runs_root, "20260710-120000Z", &specs).unwrap();
    logger.on_event(&JobEvent::Started {
        index: 0,
        output: PathBuf::from("out.mkv"),
    });
    logger.on_event(&JobEvent::Output {
        index: 0,
        line: "#GUI#progress 100%".to_string(),
    });
    logger.on_event(&JobEvent::Output {
        index: 0,
        line: "some real output line".to_string(),
    });
    logger.on_event(&JobEvent::Finished {
        index: 0,
        outcome: outcome(JobState::Ok, Some(0)),
    });

    let run_document = serde_json::json!({ "jobs": [], "summary": {} });
    let written_dir = logger.finish(&run_document).unwrap();

    let mut entries: Vec<String> = std::fs::read_dir(&written_dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(
        entries,
        vec!["job-0.json".to_string(), "summary.json".to_string()],
        "exactly these two files, nothing else"
    );

    let job: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(written_dir.join("job-0.json")).unwrap())
            .unwrap();
    assert_eq!(job["index"], 0);
    assert_eq!(job["output"], "out.mkv");
    assert_eq!(
        job["argv"],
        serde_json::json!(["--output", "out.mkv", "a.srt"])
    );
    assert_eq!(job["state"], "ok");
    assert_eq!(job["exit_code"], 0);
    assert_eq!(job["warnings"], serde_json::json!([]));
    assert_eq!(job["errors"], serde_json::json!([]));
    assert_eq!(job["duration_ms"], 42);
    assert!(
        job.as_object().unwrap().contains_key("panic"),
        "`panic` is always on the wire (D98), null for a job that did not \
         panic: {job}"
    );
    assert_eq!(
        job["lines"],
        serde_json::json!(["#GUI#progress 100%", "some real output line"]),
        "lines accumulate every Output line verbatim, without inspecting the \
         text (D24) -- the progress-shaped line here is synthetic, proving the \
         writer does not filter by content; in the real pipeline run_job's \
         parser turns progress ticks into Progress events, so they never \
         arrive as Output at all"
    );
    assert!(job["started_at"].is_string(), "job: {job}");
    assert!(job["finished_at"].is_string(), "job: {job}");

    let summary: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(written_dir.join("summary.json")).unwrap())
            .unwrap();
    assert_eq!(
        summary, run_document,
        "summary.json is the T2 run_document, written verbatim"
    );
}

/// Acceptance 4's persisted half (D98): the panic payload a recovered
/// outcome carries reaches `job-<index>.json` as the `panic` key, so a
/// panicked run stays triageable from the record once the run is over -
/// the record is what replaced the deleted stderr line.
#[test]
fn panicked_outcome_persists_its_payload_on_the_job_record() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    let specs = vec![spec(&["--output", "out.mkv", "a.srt"], "out.mkv")];

    let mut logger = RunLogger::create(&runs_root, "20260710-120000Z", &specs).unwrap();
    let mut panicked = outcome(JobState::Failed, None);
    panicked.panic = Some("scripted worker panic for job 0".to_string());
    logger.on_event(&JobEvent::Finished {
        index: 0,
        outcome: panicked,
    });
    let written_dir = logger.finish(&serde_json::json!({})).unwrap();

    let job: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(written_dir.join("job-0.json")).unwrap())
            .unwrap();
    assert_eq!(
        job["panic"], "scripted worker panic for job 0",
        "the persisted record must carry the panic payload (D98), got: {job}"
    );
}

/// Step 1, case 2: a collision on `runs_root/<run_id>` (pre-created) makes
/// `create` fall back to `<run_id>-2`; a further collision on that falls
/// back to `<run_id>-3`.
#[test]
fn collision_appends_a_numeric_suffix() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    // Derived from the clock, never a literal stamp: `create` prunes run
    // dirs outside D35's 14-day window BEFORE it looks for a collision, so
    // an absolute stamp stops colliding the day the calendar passes it
    // (ledger: test-fixture-dates-outside-retention-windows). Absolute
    // stamps belong only in fixtures that TEST the aging path, e.g.
    // `create_prunes_run_dirs_older_than_14_days_by_name_only` below.
    let run_id = make_run_id(SystemTime::now());
    std::fs::create_dir_all(runs_root.join(&run_id)).unwrap();

    let logger = RunLogger::create(&runs_root, &run_id, &[]).unwrap();
    assert_eq!(logger.dir(), runs_root.join(format!("{run_id}-2")));

    std::fs::create_dir_all(runs_root.join(format!("{run_id}-2"))).unwrap();
    let logger2 = RunLogger::create(&runs_root, &run_id, &[]).unwrap();
    assert_eq!(logger2.dir(), runs_root.join(format!("{run_id}-3")));
}

/// Step 1, case 3 (D25 skipped-job case): a job cancelled per-job before
/// dequeue emits `Finished{Cancelled}` WITHOUT a prior `Started`. The writer
/// must still produce a `job-<index>.json` record for it, with an empty
/// `lines` array and a `null` (absent) `started_at`.
#[test]
fn skipped_job_without_started_still_writes_a_record_with_empty_lines() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    let specs = vec![spec(&["a.mkv"], "a.mkv"), spec(&["b.mkv"], "b.mkv")];

    let mut logger = RunLogger::create(&runs_root, "run", &specs).unwrap();
    // Job 1 is skipped per-job before ever being dequeued (D25): no Started,
    // no Output, straight to Finished{Cancelled}.
    logger.on_event(&JobEvent::Finished {
        index: 1,
        outcome: outcome(JobState::Cancelled, None),
    });
    let written_dir = logger.finish(&serde_json::json!({})).unwrap();

    let job: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(written_dir.join("job-1.json")).unwrap())
            .unwrap();
    assert_eq!(job["state"], "cancelled");
    assert_eq!(job["lines"], serde_json::json!([]));
    assert!(
        job["started_at"].is_null(),
        "a job that never started must not claim a started_at, got: {job}"
    );
    assert!(
        !written_dir.join("job-0.json").exists(),
        "job 0 never received any event (never dequeued under a hypothetical \
         batch cancel) and must not get a file; only the run_document \
         (summary.json) covers it"
    );
}

/// A `job-<index>.json` write failure mid-run must not vanish: `on_event`
/// keeps its no-return signature (it cannot surface anything itself), but
/// `finish` must return `Err` so the caller knows the run's logs are
/// incomplete -- while still writing `summary.json` first (best-effort:
/// persist what we can, then signal). A pre-created DIRECTORY at the
/// `job-0.json` path is the same portable, no-perms way to force the write
/// to fail on every OS that `delete_partial_failure_surfaces_into_errors`
/// (job.rs) already uses.
#[test]
fn a_failed_job_file_write_makes_finish_err_but_summary_still_writes() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    let specs = vec![spec(&["a.mkv"], "a.mkv")];

    let mut logger = RunLogger::create(&runs_root, "run", &specs).unwrap();
    std::fs::create_dir(logger.dir().join("job-0.json")).unwrap();
    logger.on_event(&JobEvent::Finished {
        index: 0,
        outcome: outcome(JobState::Ok, Some(0)),
    });

    let written_dir = logger.dir().to_path_buf();
    let run_document = serde_json::json!({ "jobs": [], "summary": {} });
    logger
        .finish(&run_document)
        .expect_err("a lost job-0.json write must surface as a finish error");

    let summary: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(written_dir.join("summary.json")).unwrap())
            .unwrap();
    assert_eq!(
        summary, run_document,
        "summary.json is still written (best-effort) before the error is returned"
    );
}

/// A job that is never dequeued under BATCH cancellation (D16, unchanged)
/// gets no events at all -- not even `Finished` (its outcome appears only in
/// the queue's returned vector, never as an event). The writer must not
/// write a file for it: `on_event` simply never fires for that index.
#[test]
fn a_job_with_zero_events_never_gets_a_file() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    let specs = vec![spec(&["a.mkv"], "a.mkv")];

    let logger = RunLogger::create(&runs_root, "run", &specs).unwrap();
    let written_dir = logger.finish(&serde_json::json!({})).unwrap();

    let entries: Vec<_> = std::fs::read_dir(&written_dir).unwrap().collect();
    assert_eq!(
        entries.len(),
        1,
        "only summary.json; no job file for a spec that received no events at all"
    );
}

/// D35: `create` prunes run dirs older than 14 days, best-effort, before it
/// creates the new run's own leaf. Seeds `runs_root` with a stale dir, a
/// stale collision-suffixed dir, a fresh run dir (named via `make_run_id`,
/// so it looks exactly like one of ours but is recent), a non-run dir, and
/// a plain file -- only the two stale run dirs must disappear; everything
/// else, including the dir `create` itself produces, must survive. The
/// stale dirs get a fresh mtime (just created by this test) but an old
/// NAME, so their deletion also proves the age decision reads the name,
/// not the filesystem timestamp.
#[test]
fn create_prunes_run_dirs_older_than_14_days_by_name_only() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    std::fs::create_dir_all(&runs_root).unwrap();

    std::fs::create_dir(runs_root.join("20200101-000000Z")).unwrap();
    std::fs::create_dir(runs_root.join("20200101-000000Z-2")).unwrap();

    let fresh_name = make_run_id(SystemTime::now());
    std::fs::create_dir(runs_root.join(&fresh_name)).unwrap();

    std::fs::create_dir(runs_root.join("keep-me")).unwrap();
    std::fs::write(runs_root.join("notes.txt"), b"hello").unwrap();

    RunLogger::create(&runs_root, "this-run", &[]).unwrap();

    let mut entries: Vec<String> = std::fs::read_dir(&runs_root)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();

    let mut expected = vec![
        fresh_name,
        "keep-me".to_string(),
        "notes.txt".to_string(),
        "this-run".to_string(),
    ];
    expected.sort();

    assert_eq!(
        entries, expected,
        "the two stale run dirs must be pruned; the fresh run dir, the \
         non-run dir, the plain file, and the newly created run dir must \
         all survive"
    );
}

/// `run_id_timestamp` is [`make_run_id`]'s inverse: round-trips any instant
/// it produced, and tolerates a collision `-N` suffix (see
/// `RunLogger::create`'s numeric-suffix fallback) identically to the bare
/// name.
#[test]
fn run_id_timestamp_round_trips_make_run_id_and_tolerates_the_collision_suffix() {
    let now = SystemTime::UNIX_EPOCH + Duration::new(3723, 0);
    let expected = time::OffsetDateTime::from(now);
    let id = make_run_id(now);

    assert_eq!(run_id_timestamp(&id), Some(expected));
    assert_eq!(
        run_id_timestamp(&format!("{id}-2")),
        Some(expected),
        "a collision-suffixed name must parse identically to the bare one"
    );
}

/// `run_id_timestamp` returns `None` not only for digit-shape mismatches
/// (garbage, too short) but also for a digit-shaped, out-of-range
/// calendar/clock value (month 13, hour 99) -- stricter than the shell's
/// original hand-rolled string-slicing parser, which would have happily
/// emitted a nonsensical RFC3339 string for either.
#[test]
fn run_id_timestamp_rejects_garbage_and_out_of_range_calendar_values() {
    assert_eq!(run_id_timestamp("not-a-run-id"), None);
    assert_eq!(run_id_timestamp(""), None);
    assert_eq!(run_id_timestamp("short"), None);
    assert_eq!(
        run_id_timestamp("20260113-999999Z"),
        None,
        "digit-shaped but out-of-range hour/minute/second must not parse"
    );
    assert_eq!(
        run_id_timestamp("20261399-120000Z"),
        None,
        "digit-shaped but out-of-range month/day must not parse"
    );
}

/// D35 boundary: exercised directly against `prune_stale_runs` (`create`
/// always calls `SystemTime::now()` internally, so the cutoff itself is
/// only deterministically testable here). A run one second inside the
/// 14-day window survives; one second past it is pruned.
#[test]
fn prune_stale_runs_boundary_is_exactly_14_days() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    std::fs::create_dir_all(&runs_root).unwrap();

    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(20 * 24 * 60 * 60);
    let fourteen_days = Duration::from_secs(14 * 24 * 60 * 60);
    let just_inside = make_run_id(now - (fourteen_days - Duration::from_secs(1)));
    let just_outside = make_run_id(now - (fourteen_days + Duration::from_secs(1)));
    std::fs::create_dir(runs_root.join(&just_inside)).unwrap();
    std::fs::create_dir(runs_root.join(&just_outside)).unwrap();

    prune_stale_runs(&runs_root, now);

    assert!(
        runs_root.join(&just_inside).exists(),
        "13d 23h 59m 59s old must survive"
    );
    assert!(
        !runs_root.join(&just_outside).exists(),
        "14d 0h 0m 1s old must be pruned"
    );
}

/// D35 safety: `prune_stale_runs` must never delete through a symlink, even
/// one named and dated exactly like a stale run directory -- `file_type()`
/// does not follow symlinks, so `remove_dir_all` is never handed one; a
/// symlinked directory outside `runs_root` (as this one is) would otherwise
/// have its contents wiped out from under it.
#[cfg(unix)]
#[test]
fn prune_stale_runs_leaves_a_stale_named_symlink_and_its_target_untouched() {
    use std::os::unix::fs::symlink;

    let target_dir = tempfile::tempdir().unwrap();
    std::fs::write(target_dir.path().join("keep.txt"), b"x").unwrap();

    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    std::fs::create_dir_all(&runs_root).unwrap();
    let link = runs_root.join("20200101-000000Z");
    symlink(target_dir.path(), &link).unwrap();

    prune_stale_runs(&runs_root, SystemTime::now());

    assert!(
        std::fs::symlink_metadata(&link)
            .expect("the symlink entry itself must still be there")
            .file_type()
            .is_symlink(),
        "must still be a symlink, not resolved or replaced"
    );
    assert!(
        target_dir.path().join("keep.txt").exists(),
        "the symlink's target directory must be untouched"
    );
}
