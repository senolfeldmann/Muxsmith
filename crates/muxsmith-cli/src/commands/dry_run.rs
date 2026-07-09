//! `muxsmith dry-run` (spec 8.1, 5.5): plan the batch without muxing and print
//! the per-file resolution, diagnostics, and suggestions. Exit code mirrors
//! mkvmerge: 0 clean, 1 worst diagnostic is a warning, 2 an error.

use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{Batch, RunInputs, plan_batch};
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report::Diagnostic;

use crate::commands::{diag_exit_code, print_batch_human};
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
            // Load failure never reaches the config-time validate pass or
            // the mkvmerge lookup, but the `--json` contract still holds:
            // stdout carries exactly one JSON document. json mode folds
            // this single diagnostic into the same config-only shape the
            // mkvmerge-not-found branch below builds (mirrors validate.rs's
            // `Err(d) => vec![d]` fold); human mode is unchanged.
            if json {
                println!("{}", config_only_json(&[d], renderer));
            } else {
                println!("{}", renderer.diagnostic(&d));
            }
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
            // mkvmerge was located but querying it failed (a broken
            // installation): planning never runs here either, so json mode
            // gets the same config-only document the locate()-failure
            // branch above builds; human mode is unchanged (stderr only).
            if json {
                println!("{}", config_only_json(&config_diags, renderer));
            } else {
                eprintln!("{}", renderer.msg("mkvmerge-query-failed", &[]));
            }
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
    diag_exit_code(&config_diags, &batch)
}

/// Builds the `--json` report (spec 5.2): the raw `Batch` plus the
/// config-time diagnostics, with a `rendered` message string attached to
/// every diagnostic (config-time, batch-level, and per-file alike).
///
/// `pub(crate)`: `run --json` (D15) reuses this verbatim as the base of its
/// own document, extending it with `jobs`/`summary`.
pub(crate) fn batch_json(
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
///
/// `pub(crate)`: `run --json` (D15) reuses this for its own
/// mkvmerge-not-found path, which needs the identical superset-of-validate
/// guarantee.
pub(crate) fn config_only_json(
    config_diags: &[Diagnostic],
    renderer: &Renderer,
) -> serde_json::Value {
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
