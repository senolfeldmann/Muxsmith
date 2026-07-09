//! clap argument definitions for the `muxsmith` binary (spec 8.1). Parsing
//! only: no validation or business logic lives here.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Top-level CLI arg parser (spec 8.1): `muxsmith <subcommand> ...`.
#[derive(Parser)]
#[command(name = "muxsmith", version, about)]
pub struct Cli {
    /// Selected operation; every command shares the exit-code contract
    /// 0 clean / 1 warnings / 2 errors (spec 8.1).
    #[command(subcommand)]
    pub command: Cmd,
}

/// A `muxsmith` subcommand (spec 8.1): `validate`, `schema`, `dry-run`, and
/// `identify`. `run` (actual muxing) arrives with the executor (Plan 3).
#[derive(Subcommand)]
pub enum Cmd {
    /// Statically validate a profile (YAML or JSON).
    Validate {
        /// Path to the profile file to validate.
        profile: PathBuf,
        /// Emit the structured report as JSON.
        #[arg(long)]
        json: bool,
        /// Locale for rendered messages (default: system, fallback en).
        #[arg(long)]
        locale: Option<String>,
    },
    /// Print the profile JSON Schema.
    Schema,
    /// Plan the batch without muxing: identify sources, resolve rules, and
    /// print the per-file resolution, diagnostics, and suggestions.
    DryRun {
        /// Path to the profile file.
        profile: PathBuf,
        /// Source directory to scan (overrides the profile default).
        #[arg(long)]
        source: Option<PathBuf>,
        /// Output directory (overrides the profile default).
        #[arg(long)]
        output: Option<PathBuf>,
        /// Collision policy override (spec 4.2 run input); falls back to the
        /// profile's `output.on_collision` when unset.
        #[arg(long, value_enum)]
        on_collision: Option<CollisionArg>,
        /// Emit the structured batch report as JSON.
        #[arg(long)]
        json: bool,
        /// Locale for rendered messages (default: system, fallback en).
        #[arg(long)]
        locale: Option<String>,
    },
    /// Identify one source file via mkvmerge and print its tracks.
    Identify {
        /// Path to the media file to identify.
        file: PathBuf,
        /// Emit the structured identification as JSON.
        #[arg(long)]
        json: bool,
        /// Locale for rendered messages (default: system, fallback en).
        #[arg(long)]
        locale: Option<String>,
    },
}

/// CLI value for the collision-policy override (spec 4.2 run input). Maps
/// to core's CollisionPolicy; a CLI-local type so core stays clap-free.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CollisionArg {
    /// Refuse the colliding output (default policy).
    Error,
    /// Skip the colliding output with a warning.
    Skip,
    /// Replace the pre-existing file.
    Overwrite,
}

impl CollisionArg {
    /// The core policy this argument selects.
    pub fn policy(self) -> muxsmith_core::profile::model::CollisionPolicy {
        match self {
            CollisionArg::Error => muxsmith_core::profile::model::CollisionPolicy::Error,
            CollisionArg::Skip => muxsmith_core::profile::model::CollisionPolicy::Skip,
            CollisionArg::Overwrite => muxsmith_core::profile::model::CollisionPolicy::Overwrite,
        }
    }
}
