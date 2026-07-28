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

/// The one error-first ordering definition, hoisted to core (D102) and
/// re-exported here so every `crate::commands::severity_sorted` call site
/// -- this crate's human printing paths and `validate`'s own `--json`
/// envelope -- is unchanged.
pub(crate) use muxsmith_core::report::severity_sorted;

/// Maps the worst severity present (`None` for no diagnostics at all) to
/// the mkvmerge-style exit code (spec 5.1/8.1): 2 for an error, 1 for a
/// warning, 0 otherwise. Shared by `diag_exit_code` and `validate::run`.
pub(crate) fn severity_exit(worst: Option<Severity>) -> i32 {
    match worst {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    }
}

/// The worst-of diagnostic fold (spec 8.1): 2 if any diagnostic in
/// `config_diags`/`batch` is error-severity, 1 if the worst present is
/// warning-severity, else 0. `dry-run` uses this as its exit code directly;
/// `run` combines it with its own job-outcome fold via `max` (D15). Folds
/// over every diagnostic belonging to the planned [`Batch`] plus the
/// config-time set collected before it ran: config-time, then batch-level,
/// then per-file, in that order (order is irrelevant to the fold itself,
/// but matches `dry-run`'s human report so the two stay easy to compare).
pub(crate) fn diag_exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
    let all = config_diags
        .iter()
        .chain(batch.batch_diagnostics.iter())
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()));
    severity_exit(all.map(|d| d.severity).max())
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
    line(renderer.msg_with_counts(
        "batch-summary",
        &[
            ("root", &root.display().to_string()),
            ("extensions", &extensions.join(", ")),
        ],
        &[("count", batch.files.len())],
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

    fn file_report(source: &str) -> FileReport {
        FileReport {
            source: source.into(),
            identifier: "id".into(),
            plan: None,
            diagnostics: vec![],
        }
    }

    #[test]
    fn batch_summary_renders_the_singular_form_for_one_matched_file() {
        let batch = Batch {
            files: vec![file_report("/in/a.mkv")],
            batch_diagnostics: vec![],
            suggestions: vec![],
        };
        let report = batch_human_report(
            &batch,
            Path::new("/in"),
            &["mkv".to_string()],
            &Renderer::new(Some("en")),
        );
        assert!(
            report.contains("1 file matched (searched /in, extensions mkv)"),
            "{report}"
        );
    }

    #[test]
    fn batch_summary_renders_the_plural_form_for_two_or_more_matched_files() {
        let batch = Batch {
            files: vec![file_report("/in/a.mkv"), file_report("/in/b.mkv")],
            batch_diagnostics: vec![],
            suggestions: vec![],
        };
        let report = batch_human_report(
            &batch,
            Path::new("/in"),
            &["mkv".to_string()],
            &Renderer::new(Some("en")),
        );
        assert!(
            report.contains("2 files matched (searched /in, extensions mkv)"),
            "{report}"
        );
    }
}
