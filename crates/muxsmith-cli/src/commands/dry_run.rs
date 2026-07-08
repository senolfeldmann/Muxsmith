//! `muxsmith dry-run` (spec 8.1, 5.5): plan the batch without muxing and print
//! the per-file resolution, diagnostics, and suggestions. Exit code mirrors
//! mkvmerge: 0 clean, 1 worst diagnostic is a warning, 2 an error.

use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{Batch, RunInputs, plan_batch};
use muxsmith_core::profile::load;
use muxsmith_core::report::{Diagnostic, Severity};

use crate::i18n::Renderer;

/// Runs `muxsmith dry-run`. Returns the mkvmerge-style exit code.
pub fn run(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    json: bool,
    renderer: &Renderer,
) -> i32 {
    let profile = match load::from_file(profile_path) {
        Ok(p) => p,
        Err(d) => {
            println!("{}", renderer.diagnostic(&d));
            return 2;
        }
    };
    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{}", renderer.msg("mkvmerge-not-found", &[]));
            return 2;
        }
    };
    let lang = match mkv.list_languages() {
        Ok(l) => l,
        Err(_) => {
            eprintln!("{}", renderer.msg("mkvmerge-query-failed", &[]));
            return 2;
        }
    };
    let source_dir = source.unwrap_or_else(|| PathBuf::from("."));
    let run = RunInputs {
        source: source_dir,
        output,
        on_collision: None,
    };

    let mut ident = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &mkv,
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang);

    if json {
        println!("{}", serde_json::to_string(&batch).unwrap());
    } else {
        print_batch_human(&batch, renderer);
    }
    exit_code(&batch)
}

fn all_diags(batch: &Batch) -> impl Iterator<Item = &Diagnostic> {
    batch
        .batch_diagnostics
        .iter()
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()))
}

fn exit_code(batch: &Batch) -> i32 {
    match all_diags(batch).map(|d| d.severity).max() {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    }
}

fn print_batch_human(batch: &Batch, renderer: &Renderer) {
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
}
