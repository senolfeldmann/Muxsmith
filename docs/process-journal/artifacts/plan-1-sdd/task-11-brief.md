### Task 11: CLI scaffold and `schema` subcommand

**Files:**
- Modify: `crates/muxsmith-cli/Cargo.toml`, `crates/muxsmith-cli/src/main.rs`
- Create: `crates/muxsmith-cli/src/cli.rs`
- Test: `crates/muxsmith-cli/tests/cli_schema.rs`

**Interfaces:**
- Consumes: `Profile` (Task 4, needs the `JsonSchema` derives already in place).
- Produces: binary `muxsmith` with clap parsing; subcommands `validate { profile: PathBuf, --json, --locale <tag> }` and `schema`. Task 12 fills in `validate`; this task wires it to exit code 2 with a stub message on stderr is NOT allowed (no hardcoded prose): instead `validate` is simply absent until Task 12; only `schema` exists here.

- [ ] **Step 1: Add dependencies**

```bash
cargo add -p muxsmith-cli clap --features derive
cargo add -p muxsmith-cli serde_json schemars
cargo add -p muxsmith-cli --dev assert_cmd predicates
```

- [ ] **Step 2: Write the failing test**

`crates/muxsmith-cli/tests/cli_schema.rs`:

```rust
use assert_cmd::Command;

#[test]
fn schema_prints_json_schema_and_exits_zero() {
    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let schema: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let text = schema.to_string();
    assert!(text.contains("profile_version"));
    assert!(text.contains("tracks"));
}

#[test]
fn no_args_shows_usage_and_fails() {
    Command::cargo_bin("muxsmith").unwrap().assert().failure();
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test -p muxsmith-cli --test cli_schema`
Expected: FAIL (no subcommands)

- [ ] **Step 4: Implement**

`crates/muxsmith-cli/src/cli.rs`:

```rust
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
```

`crates/muxsmith-cli/src/main.rs`:

```rust
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
```

(clap's own usage/help text is library-generated, not our prose; localizing clap output is out of v1 scope.)

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test -p muxsmith-cli --test cli_schema`
Expected: PASS (2 tests)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(cli): clap scaffold and schema subcommand"
```

---

