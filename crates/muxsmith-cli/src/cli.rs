//! clap argument definitions for the `muxsmith` binary (spec 8.1). Parsing
//! only: no validation or business logic lives here.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Top-level CLI arg parser (spec 8.1): `muxsmith <subcommand> ...`.
#[derive(Parser)]
#[command(name = "muxsmith", version, about)]
pub struct Cli {
    /// The invoked subcommand.
    #[command(subcommand)]
    pub command: Cmd,
}

/// A `muxsmith` subcommand (spec 8.1). Only `validate` and `schema` exist
/// so far; `dry-run`/`run`/`identify` arrive with the planner (Plan 2+).
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
}
