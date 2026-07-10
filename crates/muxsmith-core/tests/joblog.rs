//! Persisted job logs (`executor::joblog`, D26, Task 6): `RunLogger` writes
//! one `job-<index>.json` per job (incrementally, as events arrive) plus a
//! `summary.json` (the T2 `run_document`, taken verbatim) when the run
//! finishes. All tests use a tempdir as `runs_root`, never the real platform
//! data dir.

use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use muxsmith_core::executor::job::{JobOutcome, JobSpec, JobState};
use muxsmith_core::executor::joblog::{RunLogger, default_runs_root, make_run_id};
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
    assert_eq!(
        job["lines"],
        serde_json::json!(["#GUI#progress 100%", "some real output line"]),
        "lines accumulate every raw Output line verbatim, progress ticks included \
         (D24: core attaches no meaning to the text; only run.rs's milestone \
         renderer treats #GUI#progress specially)"
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

/// Step 1, case 2: a collision on `runs_root/<run_id>` (pre-created) makes
/// `create` fall back to `<run_id>-2`; a further collision on that falls
/// back to `<run_id>-3`.
#[test]
fn collision_appends_a_numeric_suffix() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    std::fs::create_dir_all(runs_root.join("20260710-120000Z")).unwrap();

    let logger = RunLogger::create(&runs_root, "20260710-120000Z", &[]).unwrap();
    assert_eq!(logger.dir(), runs_root.join("20260710-120000Z-2"));

    std::fs::create_dir_all(runs_root.join("20260710-120000Z-2")).unwrap();
    let logger2 = RunLogger::create(&runs_root, "20260710-120000Z", &[]).unwrap();
    assert_eq!(logger2.dir(), runs_root.join("20260710-120000Z-3"));
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
