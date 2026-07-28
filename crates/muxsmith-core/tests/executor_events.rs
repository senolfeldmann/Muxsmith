//! `JobEvent` wire-shape golden test (D24) plus raw output-line behavior:
//! Plan 5's Tauri shell forwards `JobEvent` verbatim to a frontend, so the
//! serialized field names, tag layout, and variant shape are a contract, not
//! an implementation detail a refactor may silently drift.

use std::path::PathBuf;

use muxsmith_core::executor::job::{JobOutcome, JobProgress, JobSpec, JobState, run_job};
use muxsmith_core::executor::queue::JobEvent;
use muxsmith_core::executor::spawn::FakeSpawner;

/// Exact `serde_json::to_string` output for every `JobEvent` variant. A
/// silent field rename, reorder, or shape change here is a wire-contract
/// break the Tauri frontend (a later task) would only discover at runtime.
#[test]
fn job_event_wire_shapes() {
    let ev = JobEvent::Started {
        index: 0,
        output: PathBuf::from("out.mkv"),
    };
    assert_eq!(
        serde_json::to_string(&ev).unwrap(),
        r#"{"event":"started","index":0,"output":"out.mkv"}"#
    );

    let ev = JobEvent::Progress {
        index: 0,
        percent: 42,
    };
    assert_eq!(
        serde_json::to_string(&ev).unwrap(),
        r#"{"event":"progress","index":0,"percent":42}"#
    );

    let ev = JobEvent::Warning {
        index: 0,
        text: "'x.srt': track ignored.".into(),
    };
    assert_eq!(
        serde_json::to_string(&ev).unwrap(),
        r#"{"event":"warning","index":0,"text":"'x.srt': track ignored."}"#
    );

    let ev = JobEvent::Error {
        index: 0,
        text: "boom".into(),
    };
    assert_eq!(
        serde_json::to_string(&ev).unwrap(),
        r#"{"event":"error","index":0,"text":"boom"}"#
    );

    let ev = JobEvent::Finished {
        index: 0,
        outcome: JobOutcome {
            state: JobState::Ok,
            exit_code: Some(0),
            warnings: Vec::new(),
            errors: Vec::new(),
            duration_ms: 1234,
            panic: None,
        },
    };
    assert_eq!(
        serde_json::to_string(&ev).unwrap(),
        r#"{"event":"finished","index":0,"outcome":{"state":"ok","exit_code":0,"warnings":[],"errors":[],"duration_ms":1234,"panic":null}}"#
    );

    let index = 0;
    let ev = JobEvent::Output {
        index,
        line: "#GUI#warning hello".into(),
    };
    assert_eq!(
        serde_json::to_string(&ev).unwrap(),
        r##"{"event":"output","index":0,"line":"#GUI#warning hello"}"##
    );
}

/// A plain line and a tagged warning line both surface verbatim as
/// `OutputLine`, in addition to the tagged line's existing tag-stripped
/// `WarningLine`; the progress tick surfaces only as `Percent`, never as an
/// `OutputLine`.
#[test]
fn output_line_captures_every_non_tick_line_verbatim() {
    let dir = tempfile::tempdir().unwrap();
    let spec = JobSpec {
        argv: vec!["ignored".to_string()],
        output: dir.path().join("out.mkv"),
    };
    let fake = FakeSpawner::script(
        vec![
            "#GUI#progress 50%".to_string(),
            "a plain diagnostic line".to_string(),
            "#GUI#warning 'x.srt': track ignored.".to_string(),
        ],
        Some(0),
    );
    let cancelled = || false;
    let mut collected = Vec::new();

    run_job(&fake, &spec, &cancelled, &mut |p| collected.push(p));

    assert_eq!(
        collected,
        vec![
            JobProgress::Percent(50),
            JobProgress::OutputLine("a plain diagnostic line".to_string()),
            JobProgress::OutputLine("#GUI#warning 'x.srt': track ignored.".to_string()),
            JobProgress::WarningLine("'x.srt': track ignored.".to_string()),
        ],
        "plain and tagged lines both surface verbatim as OutputLine; the \
         tick never does"
    );
}
