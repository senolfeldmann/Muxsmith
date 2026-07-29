//! Direct core coverage for `report::json` (Plan 5 Task 2): the CLI's
//! `dry_run_cli`/`run_cli` integration tests already prove the assembled
//! documents are byte-identical to what the CLI printed before the hoist;
//! this file adds the one direct-in-core assertion the brief calls for,
//! plus the `run_document` unit tests relocated from
//! `muxsmith-cli/src/commands/run.rs` now that the function they exercise
//! lives here instead.

use muxsmith_core::executor::job::{JobOutcome, JobState};
use muxsmith_core::planner::{Batch, FileReport};
use muxsmith_core::report::json::{DiagnosticRenderer, batch_document, run_document};
use muxsmith_core::report::{DiagCode, Diagnostic};
use std::path::PathBuf;

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

/// A stub renderer for the document-shape tests below. The `"rendered"`
/// field is not what these tests assert, so it echoes the diagnostic's own
/// catalog key rather than prose - core produces codes and params, never
/// text (spec 5.2, 8.4).
struct KeyRenderer;

impl DiagnosticRenderer for KeyRenderer {
    fn diagnostic(&self, d: &Diagnostic) -> String {
        d.code.key().to_string()
    }
}

/// Three diagnostics in COLLECTION order info, warning, error - deliberately
/// the reverse of the errors-first order, so an array that is sorted and an
/// array that is preserved cannot produce the same code sequence. Every test
/// below reads that discriminating property.
fn mixed_severity() -> Vec<Diagnostic> {
    vec![
        Diagnostic::info(DiagCode::RawProperty, "tracks[0].match.exact.raw:x"),
        Diagnostic::warning(
            DiagCode::RawOnKnownProperty,
            "tracks[1].match.exact.raw:language",
        ),
        Diagnostic::error(DiagCode::InvalidRegex, "tracks[2].match.regex.title"),
    ]
}

/// The `code` sequence of a document array, which is what every ordering
/// assertion in this file compares.
fn codes(array: &serde_json::Value) -> Vec<&str> {
    array
        .as_array()
        .expect("diagnostic array")
        .iter()
        .map(|d| d["code"].as_str().expect("diagnostic code"))
        .collect()
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

/// The preserved-order half of D102 that the mutation measurement found
/// unguarded: widening `batch_document`'s `"batch_diagnostics"` emission to
/// `severity_sorted` left the whole workspace green. The sorted half of the
/// same contract is guarded elsewhere and is not duplicated here - see
/// `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran` in
/// the CLI's `dry_run_cli` integration test.
#[test]
fn batch_document_preserves_batch_diagnostics_collection_order() {
    let batch = Batch {
        files: Vec::new(),
        batch_diagnostics: mixed_severity(),
        suggestions: Vec::new(),
    };

    let doc = batch_document(&[], &batch, &KeyRenderer);

    assert_eq!(
        codes(&doc["batch_diagnostics"]),
        ["raw-property", "raw-on-known-property", "invalid-regex"],
        "D102's scope boundary: only `config_diagnostics` is ordered \
         errors-first, while `batch_diagnostics` keeps collection order \
         (spec section 5.2, Diagnostics)"
    );
}

/// The per-file half of the same boundary, unguarded by the same
/// measurement: sorting `batch_document`'s per-file `"diagnostics"` left the
/// whole workspace green too. Per-file order carries resolution meaning in
/// the human dry-run rendering, which is why `batch_document`'s own rustdoc
/// calls it deliberate.
#[test]
fn batch_document_preserves_per_file_diagnostics_collection_order() {
    let file = FileReport {
        source: PathBuf::from("S01E01.mkv"),
        identifier: "S01E01".to_string(),
        plan: None,
        diagnostics: mixed_severity(),
    };
    let batch = Batch {
        files: vec![file],
        batch_diagnostics: Vec::new(),
        suggestions: Vec::new(),
    };

    let doc = batch_document(&[], &batch, &KeyRenderer);

    assert_eq!(
        codes(&doc["files"][0]["diagnostics"]),
        ["raw-property", "raw-on-known-property", "invalid-regex"],
        "D102's scope boundary: only `config_diagnostics` is ordered \
         errors-first, while per-file `diagnostics` keep collection order \
         (spec section 5.2, Diagnostics)"
    );
}
