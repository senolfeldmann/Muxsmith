//! The shared planning pipeline (spec 5.5, 7): one orchestration -- load the
//! profile, collect the config-time diagnostics through the shared funnel,
//! resolve mkvmerge, query its language index, build the [`RunInputs`],
//! identify and plan -- that every surface calls instead of inlining its own
//! copy. This module is the injectable planner seam.
//!
//! It emits no prose, no documents and no exit codes (spec 5.2, 7, 8.4):
//! [`plan_pipeline`] returns [`PipelineOutcome`] as data and every surface
//! maps it to its own presentation (the CLI's human lines and mkvmerge-style
//! exit codes, the GUI's IPC document shapes). How mkvmerge is resolved (the
//! CLI's PATH-only lookup versus the GUI's override-aware detect ladder, D28)
//! is a parameter, not a branch.

use std::path::{Path, PathBuf};

use crate::capability::runtime::{Mkvmerge, RuntimeError};
use crate::command::command;
use crate::executor::job::JobSpec;
use crate::identify::{IdentifyCache, LiveIdentifier};
use crate::planner::{Batch, RunInputs, plan_batch};
use crate::profile::model::{CollisionPolicy, Profile};
use crate::profile::{load, validate};
use crate::report::Diagnostic;

/// One shared planning pipeline for every surface (spec 5.5, 7): the
/// injectable planner seam. Each variant is data; presentation (documents,
/// exit codes, stderr lines, IPC shapes) stays with the caller.
pub enum PipelineOutcome {
    /// `load::from_file` failed; planning and the mkvmerge lookup never ran.
    LoadFailed {
        /// The single `ParseError` diagnostic the load produced.
        diagnostic: Diagnostic,
    },
    /// The injected resolver produced no usable mkvmerge (D92 defines the
    /// shared meaning); config-time diagnostics were still collected.
    MkvmergeUnavailable {
        /// The config-time diagnostics collected before the lookup ran.
        config_diags: Vec<Diagnostic>,
    },
    /// mkvmerge resolved but `list_languages` failed (broken installation).
    QueryFailed {
        /// The config-time diagnostics collected before the query ran.
        config_diags: Vec<Diagnostic>,
    },
    /// Planning ran. Boxed for the same `large_enum_variant` reason as
    /// src-tauri's `PlanOutcome::Ready`.
    Planned(Box<PlannedPipeline>),
}

/// [`PipelineOutcome::Planned`]'s payload: everything the surfaces need to
/// render a report or start a run.
pub struct PlannedPipeline {
    /// The config-time diagnostics collected before planning (spec 5.5
    /// level 1), which every surface renders alongside the batch.
    pub config_diags: Vec<Diagnostic>,
    /// The planned batch (spec 5.1, 5.3).
    pub batch: Batch,
    /// The loaded profile, returned so callers keep presentation-side
    /// access (`print_batch_human` needs `profile.input.extensions`).
    pub profile: Profile,
    /// The effective source directory (the `.`-default of D95 applied),
    /// returned because `print_batch_human` renders it.
    pub source: PathBuf,
    /// The resolved mkvmerge, returned because the run surfaces need its
    /// path for the spawner.
    pub mkv: Mkvmerge,
}

/// Runs the shared planning pipeline over the profile at `profile_path` and
/// returns the outcome as data (spec 5.5, 7).
///
/// `resolve_mkvmerge` is the divergence that is injected rather than branched
/// on: the CLI passes [`Mkvmerge::locate`] (PATH only, spec 8.1 defines no
/// CLI override flag), the GUI a [`Mkvmerge::detect`]-with-override closure
/// (spec 8.2, D28).
///
/// Spec 5.5: dry-run is a strict superset of `validate`, never a subset. The
/// config-time static pass therefore runs FIRST -- it needs no filesystem
/// access beyond the profile itself -- and its diagnostics travel out on
/// every outcome that reached it, including the two pre-planning failures.
/// Planning itself needs mkvmerge for identification and never runs on those
/// paths, but the superset guarantee is unconditional, so the config-time
/// diagnostics are carried to the caller rather than dropped. A load failure
/// reaches neither the config-time pass nor the mkvmerge lookup, so it
/// carries only its own diagnostic.
///
/// `source` falls back to the current directory here, once for every
/// surface (D95). No natural "current directory" for a bundled desktop app,
/// but kept for parity with the CLI's own fallback (dry_run.rs); in practice
/// the batch view (T10) always supplies an explicit source directory via its
/// dir picker before calling this command.
///
/// The identification cache (spec 5.5) is constructed here and dropped when
/// this call returns: within one call each unchanged file is identified once
/// (planning and the suggestion engine's re-simulation passes share it),
/// while separate calls re-identify.
pub fn plan_pipeline(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    on_collision: Option<CollisionPolicy>,
    resolve_mkvmerge: impl FnOnce() -> Result<Mkvmerge, RuntimeError>,
) -> PipelineOutcome {
    let profile = match load::from_file(profile_path) {
        Ok(p) => p,
        Err(diagnostic) => return PipelineOutcome::LoadFailed { diagnostic },
    };

    let config_diags = validate::config_diagnostics(&profile);

    let mkv = match resolve_mkvmerge() {
        Ok(m) => m,
        Err(_) => return PipelineOutcome::MkvmergeUnavailable { config_diags },
    };
    let lang = match mkv.list_languages() {
        Ok(l) => l,
        Err(_) => return PipelineOutcome::QueryFailed { config_diags },
    };

    let run = RunInputs {
        source: source.unwrap_or_else(|| PathBuf::from(".")),
        output,
        on_collision,
    };

    let mut ident = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &mkv,
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang);

    PipelineOutcome::Planned(Box::new(PlannedPipeline {
        config_diags,
        batch,
        profile,
        source: run.source,
        mkv,
    }))
}

/// Derives the executable job specs from a planned batch: files with an
/// error-severity diagnostic already carry `plan: None` (spec 5.1), so
/// this filter_map is also the "does this file get muxed" gate.
pub fn job_specs(batch: &Batch) -> Vec<JobSpec> {
    batch
        .files
        .iter()
        .filter_map(|f| f.plan.as_ref())
        .map(|p| JobSpec {
            argv: command(p),
            output: p.output.clone(),
        })
        .collect()
}
