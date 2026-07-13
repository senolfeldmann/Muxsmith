//! `muxsmith validate` subcommand (spec 8.1): loads a profile, runs
//! `muxsmith-core`'s config-time checks, and renders the resulting
//! diagnostics.

use std::path::Path;

use muxsmith_core::profile::validate;
use muxsmith_core::report::json::rendered_diags;
use muxsmith_core::report::{Diagnostic, Severity, worst_severity};

use crate::commands::{severity_exit, severity_sorted};
use crate::i18n::Renderer;

/// Runs `muxsmith validate` (spec 8.1): loads and statically validates the
/// profile, prints diagnostics (human-readable, or as structured JSON with
/// `--json`), and returns the mkvmerge-style exit code (0 success, 1 worst
/// diagnostic is a warning, 2 worst is an error).
pub fn run(profile_path: &Path, json: bool, renderer: &Renderer) -> i32 {
    // Error-first, stable within a severity; both output modes share it.
    let diagnostics: Vec<Diagnostic> = severity_sorted(&collect(profile_path))
        .into_iter()
        .cloned()
        .collect();
    let exit = severity_exit(worst_severity(&diagnostics));

    if json {
        let entries = rendered_diags(&diagnostics, renderer);
        println!("{}", serde_json::json!({ "diagnostics": entries }));
    } else if diagnostics.is_empty() {
        println!("{}", renderer.msg("validate-ok", &[]));
    } else {
        for d in &diagnostics {
            println!("{}", renderer.diagnostic(d));
        }
        println!("{}", render_summary(&diagnostics, renderer));
    }
    exit
}

fn collect(profile_path: &Path) -> Vec<Diagnostic> {
    validate::config_diagnostics_from_file(profile_path)
}

/// The `validate-summary` line: how many of `diagnostics` are each
/// severity, each count through its own CLDR plural selector ("1 error" /
/// "2 errors", T19 #17 step 1). Split out so it is unit-testable without a
/// real profile file.
fn render_summary(diagnostics: &[Diagnostic], renderer: &Renderer) -> String {
    let count = |s: Severity| diagnostics.iter().filter(|d| d.severity == s).count();
    renderer.msg_with_counts(
        "validate-summary",
        &[],
        &[
            ("errors", count(Severity::Error)),
            ("warnings", count(Severity::Warning)),
            ("infos", count(Severity::Info)),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use muxsmith_core::report::DiagCode;

    fn renderer() -> Renderer {
        Renderer::new(Some("en"))
    }

    #[test]
    fn summary_renders_the_singular_form_for_exactly_one_of_each_severity() {
        let diags = vec![
            Diagnostic::error(DiagCode::EmptyExtensions, "input.extensions"),
            Diagnostic::warning(DiagCode::EmptyMatchExpression, "tracks[0].match"),
            Diagnostic::info(DiagCode::IgnoredFile, "input"),
        ];
        assert_eq!(
            render_summary(&diags, &renderer()),
            "1 error, 1 warning, 1 info."
        );
    }

    #[test]
    fn summary_renders_the_plural_form_for_two_or_more() {
        let diags = vec![
            Diagnostic::error(DiagCode::EmptyExtensions, "a"),
            Diagnostic::error(DiagCode::EmptyExtensions, "b"),
            Diagnostic::warning(DiagCode::EmptyMatchExpression, "a"),
            Diagnostic::warning(DiagCode::EmptyMatchExpression, "b"),
        ];
        assert_eq!(
            render_summary(&diags, &renderer()),
            "2 errors, 2 warnings, 0 infos."
        );
    }
}
