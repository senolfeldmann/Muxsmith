use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "muxsmith", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Statically validate a profile (YAML or JSON).
    Validate {
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
