//! `muxsmith dry-run` (spec 8.1, 5.5): plan the batch without muxing and print
//! the per-file resolution, diagnostics, and suggestions. Exit code mirrors
//! mkvmerge: 0 clean, 1 worst diagnostic is a warning, 2 an error.

use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{Batch, RunInputs, plan_batch};
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report::{Diagnostic, Severity};

use crate::i18n::Renderer;

/// Runs `muxsmith dry-run`. Returns the mkvmerge-style exit code.
///
/// Spec 5.5: dry-run is a strict superset of `validate`, never a subset. It
/// runs the config-time static validate pass FIRST (the same
/// `validate::validate` + `lint::provable_overlaps` collection `validate`
/// runs), then a full planning pass, and folds both diagnostic sets into one
/// report; the exit code reflects the worst severity across all of them.
/// Exception: if mkvmerge cannot be located, planning never runs (it needs
/// mkvmerge for identification). The config-time diagnostics are still
/// surfaced even then, so the superset-of-validate guarantee holds
/// unconditionally, but the exit code is the mkvmerge-not-found failure (2)
/// outright rather than a severity fold, since there is nothing else to
/// fold in.
pub fn run(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    on_collision: Option<CollisionPolicy>,
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

    // Config-time validate pass (spec 5.5, level 1); needs no filesystem
    // access beyond the profile itself, so it runs before the mkvmerge
    // lookup below.
    let mut config_diags = validate::validate(&profile);
    config_diags.extend(lint::provable_overlaps(&profile));

    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            // mkvmerge missing blocks the planning pass entirely, but the
            // config-time pass above already ran; spec 5.5 requires dry-run
            // to stay a strict superset of `validate` even on this path, so
            // those diagnostics are surfaced here rather than dropped.
            if json {
                println!("{}", config_only_json(&config_diags, renderer));
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
            eprintln!("{}", renderer.msg("mkvmerge-query-failed", &[]));
            return 2;
        }
    };
    let source_dir = source.unwrap_or_else(|| PathBuf::from("."));
    let run = RunInputs {
        source: source_dir,
        output,
        on_collision,
    };

    let mut ident = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &mkv,
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang);

    if json {
        println!("{}", batch_json(&config_diags, &batch, renderer));
    } else {
        for d in &config_diags {
            println!("{}", renderer.diagnostic(d));
        }
        print_batch_human(&batch, renderer);
    }
    exit_code(&config_diags, &batch)
}

fn all_diags<'a>(
    config_diags: &'a [Diagnostic],
    batch: &'a Batch,
) -> impl Iterator<Item = &'a Diagnostic> {
    config_diags
        .iter()
        .chain(batch.batch_diagnostics.iter())
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()))
}

fn exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
    match all_diags(config_diags, batch).map(|d| d.severity).max() {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    }
}

/// Builds the `--json` report (spec 5.2): the raw `Batch` plus the
/// config-time diagnostics, with a `rendered` message string attached to
/// every diagnostic (config-time, batch-level, and per-file alike).
fn batch_json(
    config_diags: &[Diagnostic],
    batch: &Batch,
    renderer: &Renderer,
) -> serde_json::Value {
    let files: Vec<serde_json::Value> = batch
        .files
        .iter()
        .map(|f| {
            serde_json::json!({
                "source": f.source,
                "identifier": f.identifier,
                "plan": f.plan,
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

/// Builds the `--json` report for the mkvmerge-not-found path (spec 5.5):
/// planning never ran, so `files`/`batch_diagnostics`/`suggestions` stay
/// empty, but the config-time diagnostics collected before the mkvmerge
/// lookup are still rendered here, keeping this a valid JSON document (not
/// plain text on stderr) and dry-run a superset of `validate` even when
/// mkvmerge is missing. `mkvmerge_found: false` flags the condition itself
/// for JSON consumers, since the exit code alone (2) does not distinguish
/// this from any other error-severity report.
fn config_only_json(config_diags: &[Diagnostic], renderer: &Renderer) -> serde_json::Value {
    serde_json::json!({
        "config_diagnostics": rendered_diags(config_diags, renderer),
        "files": [],
        "batch_diagnostics": [],
        "suggestions": [],
        "mkvmerge_found": false,
    })
}

/// Maps each diagnostic to its JSON value with a `rendered` field injected
/// (mirrors `validate.rs`'s `--json` rendering).
fn rendered_diags(diags: &[Diagnostic], renderer: &Renderer) -> Vec<serde_json::Value> {
    diags
        .iter()
        .map(|d| {
            let mut v = serde_json::to_value(d).unwrap();
            v["rendered"] = serde_json::Value::String(renderer.diagnostic(d));
            v
        })
        .collect()
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
