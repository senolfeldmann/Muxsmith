//! `muxsmith` binary entry point: parses CLI args ([`muxsmith_cli::cli`])
//! and dispatches to the matching command module, exiting with the
//! mkvmerge-style code the command returns (spec 8.1).

use clap::Parser;
use muxsmith_cli::{cli, commands, i18n};

fn main() {
    let args = cli::Cli::parse();
    let code = match args.command {
        cli::Cmd::Schema => {
            let schema = schemars::schema_for!(muxsmith_core::profile::Profile);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            0
        }
        cli::Cmd::Validate {
            profile,
            json,
            locale,
        } => {
            let renderer = i18n::Renderer::new(locale.as_deref());
            commands::validate::run(&profile, json, &renderer)
        }
        cli::Cmd::DryRun {
            profile,
            source,
            output,
            json,
            locale,
        } => {
            let renderer = i18n::Renderer::new(locale.as_deref());
            commands::dry_run::run(&profile, source, output, json, &renderer)
        }
        cli::Cmd::Identify { file, json, locale } => {
            let renderer = i18n::Renderer::new(locale.as_deref());
            commands::identify::run(&file, json, &renderer)
        }
    };
    std::process::exit(code);
}
