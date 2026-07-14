//! `--json` document assembly (spec 5.2, 5.5, 7, D15). Hoisted verbatim from
//! the CLI (Plan 5 Task 2) so "CLI and GUI render the same diagnostic and
//! report structures; neither owns logic" (spec 7) holds by construction:
//! the CLI's `dry-run`/`run` commands and the GUI's future summary/IPC
//! returns all build their `--json` documents through the three functions
//! here.
//!
//! This module still emits no prose of its own, matching the crate-level
//! "core produces codes and params, never text" contract (spec 5.2, 8.4):
//! every document's per-diagnostic `"rendered"` field is filled by an
//! injected [`DiagnosticRenderer`], never synthesized here.

use crate::executor::job::{JobOutcome, JobState};
use crate::planner::{Batch, Plan};
use crate::report::Diagnostic;

/// Renders one [`Diagnostic`] to a single human-readable line, for the
/// `"rendered"` field every document in this module attaches to each
/// diagnostic. Each presentation layer supplies its own implementation
/// (the CLI's Fluent-based `Renderer`, eventually the GUI's own catalog)
/// and passes it in here, so this crate assembles the documents without
/// ever producing prose itself (spec 7, 8.4).
pub trait DiagnosticRenderer {
    /// Renders `diagnostic` as a single human-readable line.
    fn diagnostic(&self, diagnostic: &Diagnostic) -> String;
}

/// Builds the `--json` report (spec 5.2): the raw [`Batch`] plus the
/// config-time diagnostics, with a `"rendered"` message string attached to
/// every diagnostic (config-time, batch-level, and per-file alike).
///
/// Consumed verbatim as the base of `run`'s own `--json` document (spec
/// 5.5, D15; see [`run_document`]), and by dry-run directly; the GUI's
/// equivalent calls reuse it unchanged (spec 7).
pub fn batch_document(
    config_diags: &[Diagnostic],
    batch: &Batch,
    renderer: &dyn DiagnosticRenderer,
) -> serde_json::Value {
    let files: Vec<serde_json::Value> = batch
        .files
        .iter()
        .map(|f| {
            serde_json::json!({
                "source": f.source,
                "identifier": f.identifier,
                "plan": plan_value(&f.plan),
                "diagnostics": rendered_diags(&f.diagnostics, renderer),
            })
        })
        .collect();
    serde_json::json!({
        "config_diagnostics": rendered_diags(config_diags, renderer),
        "files": files,
        "batch_diagnostics": rendered_diags(&batch.batch_diagnostics, renderer),
        "suggestions": batch.suggestions,
    })
}

/// Builds the `--json` report for a path where planning never ran (spec
/// 5.5): `files`/`batch_diagnostics`/`suggestions` stay empty, but whatever
/// config-time diagnostics were collected are still rendered here, keeping
/// this a valid JSON document (not plain text on stderr) and dry-run/run a
/// superset of `validate` even on these paths.
///
/// `mkvmerge_found` flags, for JSON consumers, whether mkvmerge's presence
/// was actually established: `Some(false)` on the mkvmerge-not-found path
/// (the lookup ran and failed, and the JSON consumer cannot otherwise
/// distinguish this from any other error-severity report); `Some(true)` on
/// the mkvmerge-query-failed path (the binary was found, only the
/// subsequent query failed); `None` (the key is absent from the document)
/// on a profile-load failure, where the lookup never ran at all and
/// asserting either value would claim a fact never established.
///
/// Consumed by both dry-run and run for their identical mkvmerge-missing /
/// query-failed / profile-load-failure paths (spec 5.5, D15), which all
/// need this same superset-of-validate guarantee.
pub fn config_only_document(
    config_diags: &[Diagnostic],
    mkvmerge_found: Option<bool>,
    renderer: &dyn DiagnosticRenderer,
) -> serde_json::Value {
    let mut doc = serde_json::json!({
        "config_diagnostics": rendered_diags(config_diags, renderer),
        "files": [],
        "batch_diagnostics": [],
        "suggestions": [],
    });
    if let Some(found) = mkvmerge_found {
        doc["mkvmerge_found"] = serde_json::json!(found);
    }
    doc
}

/// Extends a dry-run-shaped `--json` base document ([`batch_document`] or
/// [`config_only_document`]) with `run`'s own two additions (spec 5.5,
/// D15): a `jobs` array (one entry per outcome: `index`, `output`, plus
/// every [`JobOutcome`] field via its existing `Serialize` impl) and a
/// `summary` object with worst-of-state counts (`ok`/`warning`/`failed`/
/// `cancelled`). `outcomes` and `outputs` must be index-aligned, exactly
/// like `run_queue`'s return value and the caller's own per-job output
/// list always are; an empty pair (a path where the queue never ran)
/// yields an empty `jobs` array and a zeroed `summary`, so callers always
/// get a complete document.
///
/// `jobs[].index` indexes the QUEUE (the job spec slice `run_queue` was
/// given: only the files that planned cleanly enough to mux), not the
/// source-file list `batch.files` enumerates. A file skipped because one of
/// its diagnostics is error-severity has no queue entry, so it has no
/// `jobs[].index` at all; do not treat this index as a `batch.files`
/// offset.
pub fn run_document(
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

/// Serializes `plan` for [`batch_document`]'s per-file `"plan"` field
/// without ever panicking (D40, report/json.rs:44's fix): every `Plan`
/// enum on the wire (`TitleAction`/`ChapterSource`/`PrimaryAttachments`) is
/// now a struct variant under `#[serde(tag = "kind")]`, so `to_value`
/// cannot fail for any `Plan` this crate can construct today (pinned by
/// the per-variant shape tests next to the enums in `planner.rs`). The
/// `unwrap_or` fallback exists purely so a FUTURE non-map newtype variant
/// (the exact defect class this ADR fixes) degrades this one file's `plan`
/// to `null` instead of crashing the whole batch document -- `null` is
/// already `f.plan`'s wire value for an error-severity file with no plan
/// at all (`Option<Plan>`'s ordinary `None` encoding), so this introduces
/// no new field or shape, only an additional (today unreachable) producer
/// of an existing value. Deliberately not a `Result`-returning
/// `batch_document` propagated to callers: that would make a report-
/// building regression exit non-zero after a mux that already completed
/// successfully -- precisely Finding 1's own failure mode, reintroduced by
/// the very code meant to fix it.
fn plan_value(plan: &Option<Plan>) -> serde_json::Value {
    plan.as_ref()
        .map(|p| serde_json::to_value(p).unwrap_or(serde_json::Value::Null))
        .unwrap_or(serde_json::Value::Null)
}

/// Maps each diagnostic to its JSON value with a `"rendered"` field
/// injected (mirrors `validate`'s own `--json` rendering, spec 5.2).
pub fn rendered_diags(
    diags: &[Diagnostic],
    renderer: &dyn DiagnosticRenderer,
) -> Vec<serde_json::Value> {
    diags
        .iter()
        .map(|d| {
            let mut v = serde_json::to_value(d).unwrap();
            v["rendered"] = serde_json::Value::String(renderer.diagnostic(d));
            v
        })
        .collect()
}
