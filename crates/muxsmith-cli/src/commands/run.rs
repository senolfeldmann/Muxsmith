//! `muxsmith run` (spec 5.5, 8.1): re-plans the batch immediately before
//! executing it (never reuses a stale dry-run), then runs the queue over the
//! real mkvmerge, rendering per-job milestone lines and folding the worst-of
//! diagnostic and job severities into the process exit code (D15).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::command::command;
use muxsmith_core::executor::job::{JobOutcome, JobSpec, JobState};
use muxsmith_core::executor::queue::{JobEvent, QueueOpts, run_queue};
use muxsmith_core::executor::spawn::LiveSpawner;
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::profile::{lint, load, validate};

use crate::commands::{diag_exit_code, dry_run, print_batch_human};
use crate::i18n::Renderer;

/// Progress thresholds a job's cumulative percent is checked against, in
/// ascending order. Each threshold prints at most one milestone line per
/// job, the first time progress reaches or passes it; the printed
/// `$percent` is the threshold itself (not the raw reported value), so a
/// single `Progress` event that jumps past several thresholds at once (a
/// coarse-grained mkvmerge report) still renders one line per threshold
/// crossed, in ascending order.
const MILESTONES: [u8; 3] = [25, 50, 75];

/// Runs `muxsmith run`. Returns the mkvmerge-style exit code: worst-of fold
/// of every planning diagnostic and every job outcome (0 clean / 1 worst is
/// a warning / 2 worst is an error), overridden to 130 if the cancellation
/// flag ended the batch (D16). A `ctrlc` handler installed just before the
/// queue runs flips that flag on the first SIGINT (the queue kills
/// in-flight jobs, partials are deleted, the summary still prints) and
/// force-exits on a second SIGINT during cleanup.
///
/// Spec 5.5 level 3: identical to `dry-run` through `plan_batch` (re-plans
/// from scratch immediately before executing, never reuses a stale
/// dry-run), printing that planning report in exactly dry-run's human
/// format first. If nothing plans cleanly enough to mux, the batch is
/// folded and the function returns exactly like `dry-run` would, without
/// ever touching the queue.
#[allow(clippy::too_many_arguments)]
pub fn run(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    on_collision: Option<CollisionPolicy>,
    jobs: usize,
    fail_fast: bool,
    json: bool,
    renderer: &Renderer,
) -> i32 {
    let profile = match load::from_file(profile_path) {
        Ok(p) => p,
        Err(d) => {
            // Load failure never reaches the config-time validate pass or
            // the mkvmerge lookup, but the `--json` contract still holds:
            // stdout carries exactly one JSON document. json mode folds
            // this single diagnostic into the same config-only,
            // empty-jobs/zeroed-summary shape the mkvmerge-not-found branch
            // below builds (mirrors validate.rs's `Err(d) => vec![d]`
            // fold), minus `mkvmerge_found` itself: the lookup never ran on
            // this path, so the field is absent rather than asserting a
            // fact never established; human mode is unchanged.
            if json {
                println!(
                    "{}",
                    run_json_document(dry_run::config_only_json(&[d], None, renderer), &[], &[])
                );
            } else {
                println!("{}", renderer.diagnostic(&d));
            }
            return 2;
        }
    };

    let mut config_diags = validate::validate(&profile);
    config_diags.extend(lint::provable_overlaps(&profile));

    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            // Planning never runs without mkvmerge, so json mode gets the
            // same superset-of-validate document dry-run builds for this
            // path (D15): config diagnostics surfaced, everything else
            // empty, `mkvmerge_found: false`.
            if json {
                println!(
                    "{}",
                    run_json_document(
                        dry_run::config_only_json(&config_diags, Some(false), renderer),
                        &[],
                        &[],
                    )
                );
            } else {
                for d in &config_diags {
                    println!("{}", renderer.diagnostic(d));
                }
                eprintln!("{}", renderer.msg("mkvmerge-not-found", &[]));
            }
            return 2;
        }
    };
    let lang = match mkv.list_languages() {
        Ok(l) => l,
        Err(_) => {
            // mkvmerge was located but querying it failed (a broken
            // installation): planning never runs here either, so json mode
            // gets the same config-only, empty-jobs/zeroed-summary document
            // shape the locate()-failure branch above builds, but with
            // `mkvmerge_found: true` - the binary WAS found, only the query
            // failed; human mode is unchanged (stderr only).
            if json {
                println!(
                    "{}",
                    run_json_document(
                        dry_run::config_only_json(&config_diags, Some(true), renderer),
                        &[],
                        &[],
                    )
                );
            } else {
                eprintln!("{}", renderer.msg("mkvmerge-query-failed", &[]));
            }
            return 2;
        }
    };
    let source_dir = source.unwrap_or_else(|| PathBuf::from("."));
    let run_inputs = RunInputs {
        source: source_dir,
        output,
        on_collision,
    };

    let mut ident = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &mkv,
    };
    let batch = plan_batch(&profile, &run_inputs, &mut ident, &lang);

    if !json {
        for d in &config_diags {
            println!("{}", renderer.diagnostic(d));
        }
        print_batch_human(&batch, renderer);
    }

    // error-severity files already carry `plan: None` (spec 5.1), so this
    // filter_map is also the "does this file get muxed" gate.
    let specs: Vec<JobSpec> = batch
        .files
        .iter()
        .filter_map(|f| f.plan.as_ref())
        .map(|p| JobSpec {
            argv: command(p),
            output: p.output.clone(),
        })
        .collect();

    if specs.is_empty() {
        // Nothing plans cleanly enough to mux: fold and exit exactly like
        // dry-run, never touching the queue. json callers still get a
        // complete document (D15): the same base dry-run's `--json` would
        // print, with an empty `jobs` array and a zeroed `summary`.
        if json {
            println!(
                "{}",
                run_json_document(
                    dry_run::batch_json(&config_diags, &batch, renderer),
                    &[],
                    &[]
                )
            );
        }
        return diag_exit_code(&config_diags, &batch);
    }

    let total = specs.len();
    let outputs: Vec<String> = specs
        .iter()
        .map(|s| s.output.display().to_string())
        .collect();
    // `outputs` is moved into `MilestoneState` below (human mode only needs
    // it there); json mode needs its own copy to pair with `outcomes` once
    // the queue finishes, so it is cloned once, up front, regardless of
    // mode (cheap: one string per job).
    let json_outputs = outputs.clone();
    let spawner = LiveSpawner {
        mkvmerge: mkv.path().into(),
    };
    let opts = QueueOpts { jobs, fail_fast };
    let cancel = Arc::new(AtomicBool::new(false));

    // Single-level SIGINT (D16): first Ctrl-C requests graceful cancel
    // (queue kills in-flight, partials deleted, summary printed, exit 130);
    // a second Ctrl-C during cleanup force-exits immediately.
    let handler_cancel = Arc::clone(&cancel);
    let _ = ctrlc::set_handler(move || {
        if handler_cancel.swap(true, Ordering::SeqCst) {
            std::process::exit(130);
        }
    });

    let (tx, rx) = mpsc::channel();

    // `run_queue` blocks until the whole batch finishes, so it runs on its
    // own scoped thread; this (the calling) thread drains `rx` concurrently,
    // rendering milestone lines as jobs progress rather than only after the
    // batch completes. The queue thread owns the only `Sender`, so it drops
    // when that thread's closure returns, ending the `for event in rx` loop
    // below deterministically (no explicit `drop` needed on this side).
    let outcomes = std::thread::scope(|scope| {
        let queue_cancel = Arc::clone(&cancel);
        let handle = scope.spawn(move || run_queue(&specs, &spawner, opts, &queue_cancel, &tx));

        let mut milestones = MilestoneState::new(outputs);
        for event in rx {
            if json {
                // --json suppresses human progress lines; Task 9 builds the
                // final document from the returned outcomes instead.
                continue;
            }
            for line in milestones.render(&event, total, renderer) {
                println!("{}", line);
            }
        }

        handle.join().expect("queue worker thread panicked")
    });

    if !json {
        println!("{}", render_summary(&outcomes, renderer));
    } else {
        println!(
            "{}",
            run_json_document(
                dry_run::batch_json(&config_diags, &batch, renderer),
                &outcomes,
                &json_outputs,
            )
        );
    }

    if cancel.load(Ordering::SeqCst) {
        return 130;
    }

    std::cmp::max(
        diag_exit_code(&config_diags, &batch),
        job_exit_code(&outcomes),
    )
}

/// The queue's own worst-of fold (spec 8.1, D15): 2 if any job `Failed`, 1
/// if the worst outcome is `Warning`, else 0. Combined with
/// [`diag_exit_code`] via `max` for the process's final exit code.
fn job_exit_code(outcomes: &[JobOutcome]) -> i32 {
    if outcomes.iter().any(|o| o.state == JobState::Failed) {
        2
    } else if outcomes.iter().any(|o| o.state == JobState::Warning) {
        1
    } else {
        0
    }
}

/// The final `run-summary` line: counts of each terminal [`JobState`]
/// across `outcomes`.
fn render_summary(outcomes: &[JobOutcome], renderer: &Renderer) -> String {
    let count = |state: JobState| outcomes.iter().filter(|o| o.state == state).count();
    renderer.msg(
        "run-summary",
        &[
            ("ok", &count(JobState::Ok).to_string()),
            ("warning", &count(JobState::Warning).to_string()),
            ("failed", &count(JobState::Failed).to_string()),
            ("cancelled", &count(JobState::Cancelled).to_string()),
        ],
    )
}

/// Extends a dry-run-shaped `--json` base document (`config_diagnostics`,
/// `files`, `batch_diagnostics`, `suggestions`, and on the mkvmerge-missing
/// path `mkvmerge_found`; both built by [`dry_run::batch_json`] /
/// [`dry_run::config_only_json`]) with `run`'s own two additions (D15): a
/// `jobs` array (one entry per outcome: `index`, `output`, plus every
/// `JobOutcome` field via its existing `Serialize` impl) and a `summary`
/// object with the same worst-of state counts as [`render_summary`]'s human
/// line. `outcomes` and `outputs` must be index-aligned, exactly like
/// `run_queue`'s return value and `run`'s own `outputs` vector always are;
/// an empty pair (the mkvmerge-not-found and nothing-plans-cleanly-enough
/// paths, where the queue never runs) yields an empty `jobs` array and a
/// zeroed `summary`, so json callers always get a complete document.
fn run_json_document(
    mut base: serde_json::Value,
    outcomes: &[JobOutcome],
    outputs: &[String],
) -> serde_json::Value {
    let jobs: Vec<serde_json::Value> = outcomes
        .iter()
        .zip(outputs)
        .enumerate()
        .map(|(index, (outcome, output))| {
            let mut v = serde_json::to_value(outcome).expect("JobOutcome always serializes");
            v["index"] = serde_json::json!(index);
            v["output"] = serde_json::json!(output);
            v
        })
        .collect();
    let count = |state: JobState| outcomes.iter().filter(|o| o.state == state).count();
    base["jobs"] = serde_json::Value::Array(jobs);
    base["summary"] = serde_json::json!({
        "ok": count(JobState::Ok),
        "warning": count(JobState::Warning),
        "failed": count(JobState::Failed),
        "cancelled": count(JobState::Cancelled),
    });
    base
}

/// Human-mode progress rendering for one `run` invocation: tracks, per job
/// index, the highest [`MILESTONES`] threshold already printed (so
/// `Progress` events between crossings render nothing), plus every job's
/// output path (only `Started` carries it on the event itself; every other
/// variant needs it looked up by index, precomputed here from the same
/// `specs` the queue was given so it never drifts).
struct MilestoneState {
    outputs: Vec<String>,
    last_milestone: Vec<u8>,
}

impl MilestoneState {
    /// A fresh tracker for a batch whose job `index`es are `0..outputs.len()`.
    fn new(outputs: Vec<String>) -> MilestoneState {
        let last_milestone = vec![0; outputs.len()];
        MilestoneState {
            outputs,
            last_milestone,
        }
    }

    /// Renders zero or more human-mode lines for one [`JobEvent`]. `Started`,
    /// `Warning`/`Error` (both -> `run-job-notice`), and `Finished` each
    /// render exactly one line; `Progress` renders one line per newly
    /// crossed [`MILESTONES`] threshold (often zero, when still strictly
    /// between two of them).
    fn render(&mut self, event: &JobEvent, total: usize, renderer: &Renderer) -> Vec<String> {
        match event {
            JobEvent::Started { index, .. } => vec![renderer.msg(
                "run-job-start",
                &[
                    ("index", &(index + 1).to_string()),
                    ("total", &total.to_string()),
                    ("output", &self.outputs[*index]),
                ],
            )],
            JobEvent::Progress { index, percent } => {
                let mut lines = Vec::new();
                for &threshold in &MILESTONES {
                    if *percent >= threshold && self.last_milestone[*index] < threshold {
                        self.last_milestone[*index] = threshold;
                        lines.push(renderer.msg(
                            "run-job-progress",
                            &[
                                ("index", &(index + 1).to_string()),
                                ("total", &total.to_string()),
                                ("output", &self.outputs[*index]),
                                ("percent", &threshold.to_string()),
                            ],
                        ));
                    }
                }
                lines
            }
            JobEvent::Warning { index, text } | JobEvent::Error { index, text } => {
                vec![renderer.msg(
                    "run-job-notice",
                    &[
                        ("index", &(index + 1).to_string()),
                        ("total", &total.to_string()),
                        ("output", &self.outputs[*index]),
                        ("text", text),
                    ],
                )]
            }
            JobEvent::Finished { index, outcome } => {
                vec![self.render_finished(*index, outcome, total, renderer)]
            }
        }
    }

    /// One `run-job-{ok,warning,failed,cancelled}` line for a job's terminal
    /// outcome.
    fn render_finished(
        &self,
        index: usize,
        outcome: &JobOutcome,
        total: usize,
        renderer: &Renderer,
    ) -> String {
        let index_s = (index + 1).to_string();
        let total_s = total.to_string();
        let output = &self.outputs[index];
        let seconds = format!("{:.1}", outcome.duration_ms as f64 / 1000.0);
        match outcome.state {
            JobState::Ok => renderer.msg(
                "run-job-ok",
                &[
                    ("index", index_s.as_str()),
                    ("total", total_s.as_str()),
                    ("output", output),
                    ("seconds", seconds.as_str()),
                ],
            ),
            JobState::Warning => renderer.msg_with_count(
                "run-job-warning",
                &[
                    ("index", index_s.as_str()),
                    ("total", total_s.as_str()),
                    ("output", output),
                    ("seconds", seconds.as_str()),
                ],
                "count",
                outcome.warnings.len(),
            ),
            JobState::Failed => {
                let code = outcome
                    .exit_code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "n/a".to_string());
                renderer.msg(
                    "run-job-failed",
                    &[
                        ("index", index_s.as_str()),
                        ("total", total_s.as_str()),
                        ("output", output),
                        ("code", code.as_str()),
                    ],
                )
            }
            JobState::Cancelled => renderer.msg(
                "run-job-cancelled",
                &[
                    ("index", index_s.as_str()),
                    ("total", total_s.as_str()),
                    ("output", output),
                ],
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn renderer() -> Renderer {
        Renderer::new(Some("en"))
    }

    fn outputs(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn outcome(state: JobState, exit_code: Option<i32>, warnings: usize, ms: u64) -> JobOutcome {
        JobOutcome {
            state,
            exit_code,
            warnings: (0..warnings).map(|i| format!("w{i}")).collect(),
            errors: Vec::new(),
            duration_ms: ms,
        }
    }

    #[test]
    fn progress_prints_only_at_25_50_75_crossings() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let percents = [10, 25, 26, 40, 50, 51, 74, 75, 80, 100];
        let mut lines = Vec::new();
        for p in percents {
            lines.extend(state.render(
                &JobEvent::Progress {
                    index: 0,
                    percent: p,
                },
                1,
                &r,
            ));
        }
        assert_eq!(
            lines.len(),
            3,
            "expected exactly the 3 milestone lines, got: {lines:?}"
        );
        assert!(lines[0].contains("25%"), "{}", lines[0]);
        assert!(lines[1].contains("50%"), "{}", lines[1]);
        assert!(lines[2].contains("75%"), "{}", lines[2]);
    }

    #[test]
    fn a_jump_past_several_thresholds_renders_each_one_in_order() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Progress {
                index: 0,
                percent: 90,
            },
            1,
            &r,
        );
        assert_eq!(lines.len(), 3, "{lines:?}");
        assert!(lines[0].contains("25%"));
        assert!(lines[1].contains("50%"));
        assert!(lines[2].contains("75%"));

        // Nothing left below 100 to newly cross.
        let more = state.render(
            &JobEvent::Progress {
                index: 0,
                percent: 100,
            },
            1,
            &r,
        );
        assert!(more.is_empty(), "{more:?}");
    }

    #[test]
    fn thresholds_track_independently_per_job_index() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv", "b.mkv"]));
        let lines0 = state.render(
            &JobEvent::Progress {
                index: 0,
                percent: 30,
            },
            2,
            &r,
        );
        let lines1 = state.render(
            &JobEvent::Progress {
                index: 1,
                percent: 10,
            },
            2,
            &r,
        );
        assert_eq!(lines0.len(), 1, "job 0 crossed 25%: {lines0:?}");
        assert!(
            lines1.is_empty(),
            "job 1 has not reached 25% yet: {lines1:?}"
        );
    }

    #[test]
    fn repeated_or_regressing_percent_does_not_reprint() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let first = state.render(
            &JobEvent::Progress {
                index: 0,
                percent: 25,
            },
            1,
            &r,
        );
        let second = state.render(
            &JobEvent::Progress {
                index: 0,
                percent: 25,
            },
            1,
            &r,
        );
        let third = state.render(
            &JobEvent::Progress {
                index: 0,
                percent: 20,
            },
            1,
            &r,
        );
        assert_eq!(first.len(), 1);
        assert!(second.is_empty());
        assert!(third.is_empty());
    }

    #[test]
    fn started_line_renders_1_based_index_total_and_output() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["out/a.mkv"]));
        let lines = state.render(
            &JobEvent::Started {
                index: 0,
                output: "out/a.mkv".into(),
            },
            3,
            &r,
        );
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("[1/3]"), "{}", lines[0]);
        assert!(lines[0].contains("out/a.mkv"), "{}", lines[0]);
        assert!(lines[0].contains("start"), "{}", lines[0]);
    }

    #[test]
    fn warning_and_error_events_render_a_notice_line_with_the_text() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let w = state.render(
            &JobEvent::Warning {
                index: 0,
                text: "careful".to_string(),
            },
            1,
            &r,
        );
        let e = state.render(
            &JobEvent::Error {
                index: 0,
                text: "boom".to_string(),
            },
            1,
            &r,
        );
        assert!(w[0].contains("careful"), "{}", w[0]);
        assert!(e[0].contains("boom"), "{}", e[0]);
    }

    #[test]
    fn finished_ok_renders_duration() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Finished {
                index: 0,
                outcome: outcome(JobState::Ok, Some(0), 0, 1500),
            },
            1,
            &r,
        );
        assert!(lines[0].contains("ok"), "{}", lines[0]);
        assert!(lines[0].contains("1.5s"), "{}", lines[0]);
    }

    #[test]
    fn finished_warning_renders_count_and_duration() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Finished {
                index: 0,
                outcome: outcome(JobState::Warning, Some(1), 2, 2000),
            },
            1,
            &r,
        );
        assert!(lines[0].contains("warning"), "{}", lines[0]);
        assert!(lines[0].contains("2 warnings"), "{}", lines[0]);
        assert!(lines[0].contains("2.0s"), "{}", lines[0]);
    }

    #[test]
    fn finished_warning_with_exactly_one_warning_renders_singular() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Finished {
                index: 0,
                outcome: outcome(JobState::Warning, Some(1), 1, 2000),
            },
            1,
            &r,
        );
        assert!(
            lines[0].contains("(1 warning,"),
            "expected singular '1 warning' in: {}",
            lines[0]
        );
        assert!(
            !lines[0].contains("1 warnings"),
            "did not expect plural '1 warnings' in: {}",
            lines[0]
        );
    }

    #[test]
    fn finished_failed_renders_exit_code() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Finished {
                index: 0,
                outcome: outcome(JobState::Failed, Some(2), 0, 500),
            },
            1,
            &r,
        );
        assert!(lines[0].contains("failed"), "{}", lines[0]);
        assert!(lines[0].contains("exit 2"), "{}", lines[0]);
    }

    #[test]
    fn finished_failed_without_an_exit_code_falls_back_to_n_a() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Finished {
                index: 0,
                outcome: outcome(JobState::Failed, None, 0, 10),
            },
            1,
            &r,
        );
        assert!(lines[0].contains("exit n/a"), "{}", lines[0]);
    }

    #[test]
    fn finished_cancelled_renders_without_duration_or_code() {
        let r = renderer();
        let mut state = MilestoneState::new(outputs(&["a.mkv"]));
        let lines = state.render(
            &JobEvent::Finished {
                index: 0,
                outcome: outcome(JobState::Cancelled, None, 0, 300),
            },
            1,
            &r,
        );
        assert!(lines[0].contains("cancelled"), "{}", lines[0]);
    }

    #[test]
    fn job_exit_code_folds_worst_state() {
        assert_eq!(job_exit_code(&[outcome(JobState::Ok, Some(0), 0, 0)]), 0);
        assert_eq!(
            job_exit_code(&[
                outcome(JobState::Ok, Some(0), 0, 0),
                outcome(JobState::Warning, Some(1), 1, 0),
            ]),
            1
        );
        assert_eq!(
            job_exit_code(&[
                outcome(JobState::Warning, Some(1), 1, 0),
                outcome(JobState::Failed, Some(2), 0, 0),
            ]),
            2
        );
        assert_eq!(
            job_exit_code(&[outcome(JobState::Cancelled, None, 0, 0)]),
            0,
            "a lone Cancelled outcome folds to 0 on its own; the 130 override \
             comes from the cancel flag, not from job_exit_code"
        );
    }

    #[test]
    fn run_json_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty() {
        let base = serde_json::json!({
            "config_diagnostics": [],
            "files": [],
            "batch_diagnostics": [],
            "suggestions": [],
        });
        let doc = run_json_document(base, &[], &[]);
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
    fn run_json_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary() {
        let base = serde_json::json!({ "config_diagnostics": [] });
        let outcomes = vec![
            outcome(JobState::Ok, Some(0), 0, 12400),
            outcome(JobState::Warning, Some(1), 1, 500),
            outcome(JobState::Failed, Some(2), 0, 10),
            outcome(JobState::Cancelled, None, 0, 0),
        ];
        let outs = outputs(&["a.mkv", "b.mkv", "c.mkv", "d.mkv"]);
        let doc = run_json_document(base, &outcomes, &outs);
        assert_eq!(
            doc,
            serde_json::json!({
                "config_diagnostics": [],
                "jobs": [
                    {"index": 0, "output": "a.mkv", "state": "ok", "exit_code": 0, "warnings": [], "errors": [], "duration_ms": 12400},
                    {"index": 1, "output": "b.mkv", "state": "warning", "exit_code": 1, "warnings": ["w0"], "errors": [], "duration_ms": 500},
                    {"index": 2, "output": "c.mkv", "state": "failed", "exit_code": 2, "warnings": [], "errors": [], "duration_ms": 10},
                    {"index": 3, "output": "d.mkv", "state": "cancelled", "exit_code": null, "warnings": [], "errors": [], "duration_ms": 0},
                ],
                "summary": { "ok": 1, "warning": 1, "failed": 1, "cancelled": 1 },
            })
        );
    }

    #[test]
    fn summary_line_counts_every_state() {
        let r = renderer();
        let outcomes = vec![
            outcome(JobState::Ok, Some(0), 0, 0),
            outcome(JobState::Ok, Some(0), 0, 0),
            outcome(JobState::Warning, Some(1), 1, 0),
            outcome(JobState::Failed, Some(2), 0, 0),
            outcome(JobState::Cancelled, None, 0, 0),
        ];
        assert_eq!(
            render_summary(&outcomes, &r),
            "2 ok, 1 warning, 1 failed, 1 cancelled"
        );
    }
}
