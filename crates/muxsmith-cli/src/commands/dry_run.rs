//! `muxsmith dry-run` (spec 8.1, 5.5): plan the batch without muxing and print
//! the per-file resolution, diagnostics, and suggestions. Exit code mirrors
//! mkvmerge: 0 clean, 1 worst diagnostic is a warning, 2 an error.

use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::pipeline::{PipelineOutcome, plan_pipeline};
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::report::json::{batch_document, config_only_document};

use crate::commands::{diag_exit_code, print_batch_human, severity_sorted};
use crate::i18n::Renderer;

/// Runs `muxsmith dry-run`. Returns the mkvmerge-style exit code.
///
/// Planning itself is the shared core seam
/// ([`muxsmith_core::pipeline::plan_pipeline`], which carries spec 5.5's
/// superset-of-`validate` guarantee and its rationale); everything below is
/// this surface's presentation of the outcome. The exit code reflects the
/// worst severity across the config-time and planning diagnostics, except on
/// the pre-planning failures, which return the mkvmerge failure (2) outright
/// rather than a severity fold, since there is nothing else to fold in.
pub fn run(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    on_collision: Option<CollisionPolicy>,
    json: bool,
    renderer: &Renderer,
) -> i32 {
    match plan_pipeline(profile_path, source, output, on_collision, Mkvmerge::locate) {
        PipelineOutcome::LoadFailed { diagnostic } => {
            // The `--json` contract still holds on a load failure: stdout
            // carries exactly one JSON document. json mode folds this single
            // diagnostic into the same config-only shape the
            // mkvmerge-not-found branch below builds (mirrors validate.rs's
            // `Err(d) => vec![d]` fold); human mode is unchanged.
            if json {
                println!("{}", config_only_document(&[diagnostic], None, renderer));
            } else {
                println!("{}", renderer.diagnostic(&diagnostic));
            }
            2
        }
        PipelineOutcome::MkvmergeUnavailable { config_diags } => {
            if json {
                println!(
                    "{}",
                    config_only_document(&config_diags, Some(false), renderer)
                );
            } else {
                for d in severity_sorted(&config_diags) {
                    println!("{}", renderer.diagnostic(d));
                }
                eprintln!("{}", renderer.msg("mkvmerge-not-found", &[]));
            }
            2
        }
        PipelineOutcome::QueryFailed { config_diags } => {
            // json mode gets the same config-only document shape the
            // mkvmerge-not-found branch above builds, but with
            // `mkvmerge_found: true` - the binary WAS found, only the query
            // failed.
            if json {
                println!(
                    "{}",
                    config_only_document(&config_diags, Some(true), renderer)
                );
            } else {
                for d in severity_sorted(&config_diags) {
                    println!("{}", renderer.diagnostic(d));
                }
                eprintln!("{}", renderer.msg("mkvmerge-query-failed", &[]));
            }
            2
        }
        PipelineOutcome::Planned(planned) => {
            if json {
                println!(
                    "{}",
                    batch_document(&planned.config_diags, &planned.batch, renderer)
                );
            } else {
                for d in severity_sorted(&planned.config_diags) {
                    println!("{}", renderer.diagnostic(d));
                }
                print_batch_human(
                    &planned.batch,
                    &planned.source,
                    &planned.profile.input.extensions,
                    renderer,
                );
            }
            diag_exit_code(&planned.config_diags, &planned.batch)
        }
    }
}
