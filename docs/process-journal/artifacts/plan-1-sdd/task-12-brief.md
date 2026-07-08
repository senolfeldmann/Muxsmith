### Task 12: Fluent catalogs, renderer, and `validate` subcommand

**Files:**
- Create: `locales/en/diagnostics.ftl`, `locales/en/cli.ftl`
- Create: `crates/muxsmith-cli/src/i18n.rs`, `crates/muxsmith-cli/src/render.rs`, `crates/muxsmith-cli/src/commands/validate.rs`, `crates/muxsmith-cli/src/commands/mod.rs`
- Modify: `crates/muxsmith-cli/src/main.rs`
- Create: `crates/muxsmith-cli/tests/fixtures/good.yaml`, `crates/muxsmith-cli/tests/fixtures/bad.yaml`
- Test: `crates/muxsmith-cli/tests/cli_validate.rs`

**Interfaces:**
- Consumes: `validate` + `provable_overlaps` + `load` (Tasks 4, 8-10), `Diagnostic`/`DiagCode::key()`/`worst_severity` (Task 2).
- Produces:
  - `i18n::Renderer::new(locale: Option<&str>) -> Renderer` (embeds `locales/en/*.ftl` via `include_str!`; unknown locale falls back to en; `set_use_isolating(false)` so output has no Unicode isolation marks)
  - `Renderer::diagnostic(&Diagnostic) -> String`, `Renderer::msg(id: &str, args: &[(&str, &str)]) -> String`
  - `muxsmith validate <profile>`: renders diagnostics sorted error-first; exit 0 clean / 1 warnings / 2 errors (spec 8.1). `--json` prints `{"diagnostics": [...]}` where each entry is the serialized `Diagnostic` plus a `"rendered"` string field.

- [ ] **Step 1: Add dependencies**

```bash
cargo add -p muxsmith-cli fluent-bundle unic-langid sys-locale
```

- [ ] **Step 2: Write the catalogs**

`locales/en/diagnostics.ftl` (one message per `DiagCode::key()`; params referenced as Fluent variables):

```ftl
severity-error = error
severity-warning = warning
severity-info = info

unsupported-profile-version = Unsupported profile_version { $found } (supported: { $supported }).
parse-error = The profile could not be parsed at "{ $at }": { $detail }
no-track-rules = The profile defines no track rules; at least one is required.
empty-match-expression = This match expression is empty and would match every track.
empty-extensions = The extensions list must not be empty.
invalid-regex = Invalid regular expression: { $detail }
unknown-property = Unknown property "{ $property }". It is not part of the mkvmerge identification model.
not-string-property = Property "{ $property }" has type { $actual_type }; { $condition } conditions require a string property.
value-type-mismatch = Value for "{ $property }" has type { $found }, expected { $expected }.
unknown-settable-property = "{ $property }" is not a settable track property.
invalid-keyword = Invalid keyword "{ $found }". Allowed: { $allowed }.
locator-conflict = match_to_source and match_pattern are mutually exclusive; set only one.
invalid-template = Invalid template: { $detail }
unknown-template-field = Unknown template field "{ $field }". Available fields: { $allowed }.
unknown-template-filter = Unknown template filter: { $detail }
path-separator-in-template = Filename templates must not contain path separators.
attachment-rule-shape = Each attachment rule needs exactly one of select, drop, add (found { $found }).
provable-overlap = Rules { $rule_a } and { $rule_b } provably overlap: every track matching one also matches the other. Add a distinguishing condition to one of them.
ambiguous-rule = Rule matches { $count } tracks; it must match exactly one.
overlapping-rules = Rules { $rule_a } and { $rule_b } both claim track { $track }.
missing-track = No track matches this non-optional rule.
missing-external = No file matches this external locator.
ambiguous-external = { $count } files match this external locator; exactly one is required.
output-collision = Output path { $path } collides with an existing file or another planned output.
source-overwrite = Output path { $path } would overwrite a source file. This is never allowed.
duplicate-identifier = Files { $file_a } and { $file_b } share the identifier "{ $identifier }".
donor-is-primary = External donor file { $donor } is itself a primary source.
ignored-file = File matches the extension list but not the input pattern.
multiple-identifier-matches = The input pattern matches more than once in "{ $name }"; the first match is used.
unknown-property-skew = Property "{ $property }" is unknown to this Muxsmith build but reported by the local mkvmerge; it is matched untyped.
```

`locales/en/cli.ftl`:

```ftl
validate-ok = Profile is valid.
validate-summary = { $errors } error(s), { $warnings } warning(s), { $infos } info(s).
diagnostic-line = [{ $severity }] { $config_path }: { $message }
```

- [ ] **Step 3: Write the failing tests**

`crates/muxsmith-cli/tests/fixtures/good.yaml`:

```yaml
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { type: audio, language: en } }
```

`crates/muxsmith-cli/tests/fixtures/bad.yaml`:

```yaml
profile_version: 1
input: { pattern: '([', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio } }
  - match: { exact: { type: audio, language: en } }
  - match: { substring: { forced_track: 'x' } }
```

`crates/muxsmith-cli/tests/cli_validate.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

fn muxsmith() -> Command {
    Command::cargo_bin("muxsmith").unwrap()
}

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn valid_profile_exits_zero_with_ok_message() {
    muxsmith()
        .args(["validate", &fixture("good.yaml")])
        .assert()
        .success()
        .stdout(predicate::str::contains("Profile is valid."));
}

#[test]
fn invalid_profile_exits_two_and_renders_messages() {
    muxsmith()
        .args(["validate", &fixture("bad.yaml")])
        .assert()
        .code(2)
        .stdout(
            predicate::str::contains("Invalid regular expression")
                .and(predicate::str::contains("input.pattern"))
                .and(predicate::str::contains("forced_track")),
        );
}

#[test]
fn warnings_only_exits_one() {
    // good.yaml plus an overlap warning: audio subset rule.
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio } }
  - match: { exact: { type: audio, language: en } }
"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("warn.yaml");
    std::fs::write(&path, y).unwrap();
    muxsmith()
        .args(["validate", path.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("provably overlap"));
}

#[test]
fn json_output_is_machine_readable() {
    let out = muxsmith()
        .args(["validate", &fixture("bad.yaml"), "--json"])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let diags = v["diagnostics"].as_array().unwrap();
    assert!(!diags.is_empty());
    let first = &diags[0];
    assert!(first["code"].is_string());
    assert!(first["severity"].is_string());
    assert!(first["rendered"].is_string());
}

#[test]
fn missing_file_is_parse_error_exit_two() {
    muxsmith()
        .args(["validate", "/nonexistent/profile.yaml"])
        .assert()
        .code(2);
}
```

Add the dev-dependency used above:

```bash
cargo add -p muxsmith-cli --dev tempfile
```

- [ ] **Step 4: Run tests to verify they fail**

Run: `cargo test -p muxsmith-cli --test cli_validate`
Expected: FAIL (validate is a stub)

- [ ] **Step 5: Implement renderer and command**

`crates/muxsmith-cli/src/i18n.rs`:

```rust
//! Fluent-based rendering. The ONLY place where diagnostic codes and
//! params become human text on the CLI side (spec 8.4).

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

const EN_DIAGNOSTICS: &str = include_str!("../../../locales/en/diagnostics.ftl");
const EN_CLI: &str = include_str!("../../../locales/en/cli.ftl");

pub struct Renderer {
    bundle: FluentBundle<FluentResource>,
}

impl Renderer {
    /// v1 ships English only; `locale` is accepted for interface stability
    /// and falls back to en for any unknown tag (spec 8.4).
    pub fn new(locale: Option<&str>) -> Renderer {
        let requested = locale
            .map(str::to_owned)
            .or_else(|| sys_locale::get_locale())
            .unwrap_or_else(|| "en".into());
        let langid: LanguageIdentifier = requested
            .parse()
            .unwrap_or_else(|_| "en".parse().unwrap());
        let mut bundle = FluentBundle::new(vec![langid]);
        // No Unicode isolation marks around placeables: CLI output must be
        // plain grep-able text.
        bundle.set_use_isolating(false);
        for source in [EN_DIAGNOSTICS, EN_CLI] {
            let res = FluentResource::try_new(source.to_owned())
                .expect("embedded catalog must parse");
            bundle.add_resource_overriding(res);
        }
        Renderer { bundle }
    }

    pub fn msg(&self, id: &str, args: &[(&str, &str)]) -> String {
        let Some(message) = self.bundle.get_message(id) else {
            // Missing catalog entry: fall back to the raw id so the
            // problem is visible instead of hidden. CI guards this case.
            return id.to_string();
        };
        let Some(pattern) = message.value() else {
            return id.to_string();
        };
        let mut fargs = FluentArgs::new();
        for (k, v) in args {
            fargs.set(*k, *v);
        }
        let mut errors = Vec::new();
        self.bundle
            .format_pattern(pattern, Some(&fargs), &mut errors)
            .into_owned()
    }

    pub fn diagnostic(&self, d: &muxsmith_core::report::Diagnostic) -> String {
        let params: Vec<(&str, &str)> = d
            .params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let message = self.msg(d.code.key(), &params);
        let severity = self.msg(severity_key(d.severity), &[]);
        self.msg(
            "diagnostic-line",
            &[
                ("severity", &severity),
                ("config_path", &d.config_path),
                ("message", &message),
            ],
        )
    }
}

fn severity_key(s: muxsmith_core::report::Severity) -> &'static str {
    match s {
        muxsmith_core::report::Severity::Error => "severity-error",
        muxsmith_core::report::Severity::Warning => "severity-warning",
        muxsmith_core::report::Severity::Info => "severity-info",
    }
}
```

`crates/muxsmith-cli/src/commands/mod.rs`:

```rust
pub mod validate;
```

`crates/muxsmith-cli/src/commands/validate.rs`:

```rust
use std::path::Path;

use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report::{worst_severity, Diagnostic, Severity};

use crate::i18n::Renderer;

pub fn run(profile_path: &Path, json: bool, renderer: &Renderer) -> i32 {
    let diagnostics = collect(profile_path);
    let exit = match worst_severity(&diagnostics) {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    };

    if json {
        let entries: Vec<serde_json::Value> = diagnostics
            .iter()
            .map(|d| {
                let mut v = serde_json::to_value(d).unwrap();
                v["rendered"] = serde_json::Value::String(renderer.diagnostic(d));
                v
            })
            .collect();
        println!(
            "{}",
            serde_json::json!({ "diagnostics": entries })
        );
    } else if diagnostics.is_empty() {
        println!("{}", renderer.msg("validate-ok", &[]));
    } else {
        let mut sorted = diagnostics.clone();
        sorted.sort_by_key(|d| std::cmp::Reverse(d.severity));
        for d in &sorted {
            println!("{}", renderer.diagnostic(d));
        }
        let count = |s| diagnostics.iter().filter(|d| d.severity == s).count().to_string();
        println!(
            "{}",
            renderer.msg(
                "validate-summary",
                &[
                    ("errors", &count(Severity::Error)),
                    ("warnings", &count(Severity::Warning)),
                    ("infos", &count(Severity::Info)),
                ],
            )
        );
    }
    exit
}

fn collect(profile_path: &Path) -> Vec<Diagnostic> {
    match load::from_file(profile_path) {
        Err(d) => vec![d],
        Ok(profile) => {
            let mut diags = validate::validate(&profile);
            diags.extend(lint::provable_overlaps(&profile));
            diags
        }
    }
}
```

Update `crates/muxsmith-cli/src/main.rs`:

```rust
mod cli;
mod commands;
mod i18n;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    let code = match args.command {
        cli::Cmd::Schema => {
            let schema = schemars::schema_for!(muxsmith_core::profile::Profile);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            0
        }
        cli::Cmd::Validate { profile, json, locale } => {
            let renderer = i18n::Renderer::new(locale.as_deref());
            commands::validate::run(&profile, json, &renderer)
        }
    };
    std::process::exit(code);
}
```

(Delete `crates/muxsmith-cli/src/render.rs` from the Files list if not needed; rendering fits inside `i18n.rs` and `commands/validate.rs`.)

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test -p muxsmith-cli`
Expected: PASS (all CLI tests, including Task 11's)

- [ ] **Step 7: Run the full workspace test suite**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "feat(cli): validate subcommand with Fluent-rendered diagnostics and exit codes"
```

---

