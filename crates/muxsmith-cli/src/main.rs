mod cli;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    let code = match args.command {
        cli::Cmd::Schema => {
            let schema = schemars::schema_for!(muxsmith_core::profile::Profile);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            0
        }
        cli::Cmd::Validate { .. } => {
            // Implemented in the next task (i18n renderer required first;
            // no hardcoded strings allowed here).
            2
        }
    };
    std::process::exit(code);
}
