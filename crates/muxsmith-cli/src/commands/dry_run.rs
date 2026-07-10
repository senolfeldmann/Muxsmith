//! `muxsmith dry-run` (spec 8.1, 5.5): plan the batch without muxing and print
//! the per-file resolution, diagnostics, and suggestions. Exit code mirrors
//! mkvmerge: 0 clean, 1 worst diagnostic is a warning, 2 an error.

use std::path::{Path, PathBuf};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report::json::{batch_document, config_only_document};

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
                println!("{}", config_only_document(&[d], None, renderer));
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
                println!(
                    "{}",
                    config_only_document(&config_diags, Some(false), renderer)
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
            // gets the same config-only document shape the locate()-failure
            // branch above builds, but with `mkvmerge_found: true` - the
            // binary WAS found, only the query failed; human mode is
            // unchanged (stderr only).
            if json {
                println!(
                    "{}",
                    config_only_document(&config_diags, Some(true), renderer)
                );
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
        println!("{}", batch_document(&config_diags, &batch, renderer));
    } else {
        for d in &config_diags {
            println!("{}", renderer.diagnostic(d));
        }
        print_batch_human(&batch, renderer);
    }
    diag_exit_code(&config_diags, &batch)
}
