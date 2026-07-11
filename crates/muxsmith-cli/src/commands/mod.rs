//! One module per CLI subcommand (spec 8.1); each wraps a core operation
//! and renders its diagnostics via [`crate::i18n::Renderer`].

pub mod dry_run;
pub mod identify;
pub mod run;
pub mod validate;

use std::cmp::Reverse;
use std::path::Path;

use muxsmith_core::planner::Batch;
use muxsmith_core::report::{Diagnostic, Severity};

use crate::i18n::Renderer;

/// Diagnostics in error-first order (`Severity` is `Info < Warning < Error`,
/// so `Reverse` puts errors first), stable within a severity. Matches
/// `validate`'s human/JSON sort so every surface prints the worst first.
/// Returns borrows; the source slice is untouched.
pub(crate) fn severity_sorted(diags: &[Diagnostic]) -> Vec<&Diagnostic> {
    let mut sorted: Vec<&Diagnostic> = diags.iter().collect();
    sorted.sort_by_key(|d| Reverse(d.severity));
    sorted
}

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
    print!("{}", batch_human_report(batch, root, extensions, renderer));
}

/// Builds the dry-run human report as a single string (see
/// [`print_batch_human`] for the format). Split out so it is unit-testable
/// without capturing stdout.
///
/// Per-file diagnostics are rendered file-less: the `dry-run-file` header
/// already names the file, so repeating it on each diagnostic under it is
/// noise. Batch-level diagnostics keep their file, since no header precedes
/// them.
fn batch_human_report(
    batch: &Batch,
    root: &Path,
    extensions: &[String],
    renderer: &Renderer,
) -> String {
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    for f in &batch.files {
        line(renderer.msg(
            "dry-run-file",
            &[
                ("file", &f.source.display().to_string()),
                ("id", &f.identifier),
            ],
        ));
        if let Some(plan) = &f.plan {
            for a in &plan.assignments {
                let track = a
                    .track_id
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "-".into());
                line(renderer.msg(
                    "dry-run-assignment",
                    &[("rule", &a.rule_index.to_string()), ("track", &track)],
                ));
            }
            line(renderer.msg(
                "dry-run-output",
                &[("path", &plan.output.display().to_string())],
            ));
        }
        for d in severity_sorted(&f.diagnostics) {
            line(renderer.diagnostic_no_file(d));
        }
    }
    for d in severity_sorted(&batch.batch_diagnostics) {
        line(renderer.diagnostic(d));
    }
    for s in &batch.suggestions {
        line(renderer.msg("dry-run-suggestion", &[("config_path", &s.config_path)]));
        line(s.yaml_fragment.clone());
    }
    line(renderer.msg(
        "dry-run-summary",
        &[
            ("count", &batch.files.len().to_string()),
            ("root", &root.display().to_string()),
            ("extensions", &extensions.join(", ")),
        ],
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use muxsmith_core::planner::{Batch, FileReport};
    use muxsmith_core::report::{DiagCode, Diagnostic};

    #[test]
    fn per_file_diagnostics_do_not_repeat_the_filename_the_header_prints() {
        let file = "/in/Show.S01E01.mkv";
        let fr = FileReport {
            source: file.into(),
            identifier: "S01E01".into(),
            plan: None,
            diagnostics: vec![
                Diagnostic::error(DiagCode::UnsupportedSource, "input").for_file(file),
            ],
        };
        let batch = Batch {
            files: vec![fr],
            batch_diagnostics: vec![],
            suggestions: vec![],
        };
        let report = batch_human_report(
            &batch,
            Path::new("/in"),
            &["mkv".to_string()],
            &Renderer::new(Some("en")),
        );
        let count = report.matches(file).count();
        assert_eq!(
            count, 1,
            "the filename must appear once (the dry-run-file header), not on each diagnostic:\n{report}"
        );
    }

    #[test]
    fn per_file_diagnostics_print_errors_before_warnings() {
        let file = "/in/Show.S01E01.mkv";
        // Emitted warning-then-error; the human report must print the error first.
        let fr = FileReport {
            source: file.into(),
            identifier: "S01E01".into(),
            plan: None,
            diagnostics: vec![
                Diagnostic::warning(DiagCode::UnknownPropertySkew, "tracks[0].match")
                    .for_file(file)
                    .with("property", "new_prop")
                    .with("found_version", "21")
                    .with("pinned", "20"),
                Diagnostic::error(DiagCode::UnsupportedSource, "input").for_file(file),
            ],
        };
        let batch = Batch {
            files: vec![fr],
            batch_diagnostics: vec![],
            suggestions: vec![],
        };
        let report = batch_human_report(
            &batch,
            Path::new("/in"),
            &["mkv".to_string()],
            &Renderer::new(Some("en")),
        );
        let err_at = report.find("[error]").expect("an error line");
        let warn_at = report.find("[warning]").expect("a warning line");
        assert!(err_at < warn_at, "error must precede warning in:\n{report}");
    }
}
