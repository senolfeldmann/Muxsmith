//! Direct core coverage for `report::json` (Plan 5 Task 2): the CLI's
//! `dry_run_cli`/`run_cli` integration tests already prove the assembled
//! documents are byte-identical to what the CLI printed before the hoist;
//! this file adds the one direct-in-core assertion the brief calls for,
//! plus the `run_document` unit tests relocated from
//! `muxsmith-cli/src/commands/run.rs` now that the function they exercise
//! lives here instead.

use muxsmith_core::executor::job::{JobOutcome, JobState};
use muxsmith_core::report::json::run_document;

fn outcome(state: JobState, exit_code: Option<i32>, warnings: usize, ms: u64) -> JobOutcome {
    JobOutcome {
        state,
        exit_code,
        warnings: (0..warnings).map(|i| format!("w{i}")).collect(),
        errors: Vec::new(),
        duration_ms: ms,
        panic: None,
    }
}

/// The brief's Step 2 test: `jobs[]` entries carry `index`, `output`, and
/// `state` (on top of the rest of `JobOutcome`'s own fields), and `summary`
/// carries all four terminal-state counts, even for a single job.
#[test]
fn run_document_jobs_carry_index_output_state_and_summary_carries_all_four_counts() {
    let base = serde_json::json!({
        "config_diagnostics": [],
        "files": [],
        "batch_diagnostics": [],
        "suggestions": [],
    });
    let outcomes = vec![outcome(JobState::Ok, Some(0), 0, 1000)];
    let outputs = vec!["a.mkv".to_string()];

    let doc = run_document(base, &outcomes, &outputs);

    let job = &doc["jobs"][0];
    assert_eq!(job["index"], 0);
    assert_eq!(job["output"], "a.mkv");
    assert_eq!(job["state"], "ok");

    let summary = &doc["summary"];
    assert!(summary.get("ok").is_some());
    assert!(summary.get("warning").is_some());
    assert!(summary.get("failed").is_some());
    assert!(summary.get("cancelled").is_some());
}

#[test]
fn run_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty() {
    let base = serde_json::json!({
        "config_diagnostics": [],
        "files": [],
        "batch_diagnostics": [],
        "suggestions": [],
    });
    let doc = run_document(base, &[], &[]);
    assert_eq!(
        doc,
        serde_json::json!({
            "config_diagnostics": [],
            "files": [],
            "batch_diagnostics": [],
            "suggestions": [],
            "jobs": [],
            "summary": { "ok": 0, "warning": 0, "failed": 0, "cancelled": 0 },
        })
    );
}

#[test]
fn run_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary() {
    let base = serde_json::json!({ "config_diagnostics": [] });
    let outcomes = vec![
        outcome(JobState::Ok, Some(0), 0, 12400),
        outcome(JobState::Warning, Some(1), 1, 500),
        outcome(JobState::Failed, Some(2), 0, 10),
        outcome(JobState::Cancelled, None, 0, 0),
    ];
    let outs: Vec<String> = ["a.mkv", "b.mkv", "c.mkv", "d.mkv"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let doc = run_document(base, &outcomes, &outs);
    assert_eq!(
        doc,
        serde_json::json!({
            "config_diagnostics": [],
            "jobs": [
                {"index": 0, "output": "a.mkv", "state": "ok", "exit_code": 0, "warnings": [], "errors": [], "duration_ms": 12400, "panic": null},
                {"index": 1, "output": "b.mkv", "state": "warning", "exit_code": 1, "warnings": ["w0"], "errors": [], "duration_ms": 500, "panic": null},
                {"index": 2, "output": "c.mkv", "state": "failed", "exit_code": 2, "warnings": [], "errors": [], "duration_ms": 10, "panic": null},
                {"index": 3, "output": "d.mkv", "state": "cancelled", "exit_code": null, "warnings": [], "errors": [], "duration_ms": 0, "panic": null},
            ],
            "summary": { "ok": 1, "warning": 1, "failed": 1, "cancelled": 1 },
        })
    );
}
