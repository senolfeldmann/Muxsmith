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
