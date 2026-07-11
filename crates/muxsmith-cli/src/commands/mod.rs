//! One module per CLI subcommand (spec 8.1); each wraps a core operation
//! and renders its diagnostics via [`crate::i18n::Renderer`].

pub mod dry_run;
pub mod identify;
pub mod run;
pub mod validate;

use std::path::Path;

use muxsmith_core::planner::Batch;
use muxsmith_core::report::{Diagnostic, Severity};

use crate::i18n::Renderer;

/// Every diagnostic belonging to a planned [`Batch`] plus the config-time
/// set collected before it ran: config-time, then batch-level, then
/// per-file, in that order. Shared by `dry-run` and `run` (spec 5.5): both
/// re-plan via `plan_batch` and fold the same diagnostic set into their
/// exit code.
pub(crate) fn all_diags<'a>(
    config_diags: &'a [Diagnostic],
    batch: &'a Batch,
) -> impl Iterator<Item = &'a Diagnostic> {
    config_diags
        .iter()
        .chain(batch.batch_diagnostics.iter())
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()))
}

/// The worst-of diagnostic fold (spec 8.1): 2 if any diagnostic in
/// `config_diags`/`batch` is error-severity, 1 if the worst present is
/// warning-severity, else 0. `dry-run` uses this as its exit code directly;
/// `run` combines it with its own job-outcome fold via `max` (D15).
pub(crate) fn diag_exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
    match all_diags(config_diags, batch).map(|d| d.severity).max() {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    }
}

/// Prints a planned [`Batch`] in dry-run's human format (spec 5.5): per file
/// its identifier, its assignments and output path when it has a plan, then
/// its diagnostics; then batch-level diagnostics and suggestions; then a
/// trailing batch summary line naming how many files matched, `root`, and
/// `extensions` (ROADMAP "Empty-batch human output" gap, ticket #8).
/// Unconditional: the empty batch (zero files, zero diagnostics, zero
/// suggestions) still prints this one line ("0 files matched (searched
/// ..., extensions ...)"), so human mode never exits clean and silent the
/// way it did before. Shared verbatim by `dry-run` and `run`: spec 5.5
/// requires `run` to re-plan and print exactly this report before
/// executing.
pub(crate) fn print_batch_human(
    batch: &Batch,
    root: &Path,
    extensions: &[String],
    renderer: &Renderer,
) {
    for f in &batch.files {
        println!(
            "{}",
            renderer.msg(
                "dry-run-file",
                &[
                    ("file", &f.source.display().to_string()),
                    ("id", &f.identifier),
                ],
            )
        );
        if let Some(plan) = &f.plan {
            for a in &plan.assignments {
                let track = a
                    .track_id
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "-".into());
                println!(
                    "{}",
                    renderer.msg(
                        "dry-run-assignment",
                        &[("rule", &a.rule_index.to_string()), ("track", &track)],
                    )
                );
            }
            println!(
                "{}",
                renderer.msg(
                    "dry-run-output",
                    &[("path", &plan.output.display().to_string())]
                )
            );
        }
        for d in &f.diagnostics {
            println!("{}", renderer.diagnostic(d));
        }
    }
    for d in &batch.batch_diagnostics {
        println!("{}", renderer.diagnostic(d));
    }
    for s in &batch.suggestions {
        println!(
            "{}",
            renderer.msg("dry-run-suggestion", &[("config_path", &s.config_path)])
        );
        println!("{}", s.yaml_fragment);
    }
    println!(
        "{}",
        renderer.msg(
            "dry-run-summary",
            &[
                ("count", &batch.files.len().to_string()),
                ("root", &root.display().to_string()),
                ("extensions", &extensions.join(", ")),
            ],
        )
    );
}
