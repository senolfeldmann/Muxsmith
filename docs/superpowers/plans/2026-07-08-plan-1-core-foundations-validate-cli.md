# Muxsmith Plan 1: Core Foundations and Validate CLI

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Cargo workspace with `muxsmith-core` (profile model, match algebra, template engine, capability model, semantic validation, static lint) and a `muxsmith` CLI providing `validate` and `schema`, fully i18n-clean.

**Architecture:** All logic and all diagnostics (as code+params data, never prose) live in `muxsmith-core`. The CLI is a thin clap binary that renders diagnostics through Fluent catalogs under `locales/`. Matchable track properties are generated into code by an `xtask` from the upstream mkvmerge identification schema; the schema itself is never committed. This plan is stage 1 of 4 (spec: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`); planning/dry-run, execution, and GUI follow in later plans.

**Tech Stack:** Rust stable (edition 2024), serde + yaml-serde + serde_json + serde_path_to_error, schemars, regex, clap 4 (derive), fluent-bundle + unic-langid, assert_cmd + predicates (dev).

## Global Constraints

- Spec is authoritative: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`. On conflict, the spec wins; flag the conflict instead of improvising.
- `muxsmith-core` emits NO user-facing prose. Diagnostics carry `code` + `params` only (spec 5.2). All human text lives in `locales/*/*.ftl`.
- Unknown keys in profiles are errors, not warnings: `#[serde(deny_unknown_fields)]` on every profile struct (spec 4).
- `profile_version: 1` is the only accepted version.
- User-facing string names in profiles are exactly the mkvmerge identification schema names (spec 4.4).
- License MIT. ASCII only in identifiers and code comments. No em-dashes or curly quotes in any file.
- Every `cargo add` step resolves current versions at execution time; do not hand-pin versions unless a step says so.
- All commands run from the repo root `~/Git/Muxsmith` unless stated otherwise.

---

### Task 1: Workspace scaffold

**Files:**
- Create: `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, `LICENSE`
- Create: `crates/muxsmith-core/Cargo.toml`, `crates/muxsmith-core/src/lib.rs`
- Create: `crates/muxsmith-cli/Cargo.toml`, `crates/muxsmith-cli/src/main.rs`

**Interfaces:**
- Consumes: nothing (first task).
- Produces: compiling workspace; crate names `muxsmith-core` (lib name `muxsmith_core`) and `muxsmith-cli` (binary name `muxsmith`).

- [ ] **Step 1: Create workspace files**

`Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/muxsmith-core", "crates/muxsmith-cli"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/senolf/muxsmith"
```

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

`.gitignore`:

```
/target
```

`LICENSE`: MIT text, `Copyright (c) 2026 Şenol Feldmann` (copy the license body from `~/Git/mkv-batch-tools/LICENSE`, update the year).

`crates/muxsmith-core/Cargo.toml`:

```toml
[package]
name = "muxsmith-core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
```

`crates/muxsmith-core/src/lib.rs`:

```rust
pub mod report;
```

Create `crates/muxsmith-core/src/report.rs` containing only a placeholder unit test so the workspace compiles:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn workspace_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
```

`crates/muxsmith-cli/Cargo.toml`:

```toml
[package]
name = "muxsmith-cli"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "muxsmith"
path = "src/main.rs"

[dependencies]
muxsmith-core = { path = "../muxsmith-core" }
```

`crates/muxsmith-cli/src/main.rs`:

```rust
fn main() {
    // Subcommands arrive in Task 11.
}
```

- [ ] **Step 2: Verify the workspace builds and tests run**

Run: `cargo test --workspace`
Expected: PASS (1 test: `workspace_compiles`)

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "chore: scaffold cargo workspace (muxsmith-core, muxsmith-cli)"
```

---

### Task 2: Diagnostic types (`report` module)

**Files:**
- Modify: `crates/muxsmith-core/src/report.rs` (replace placeholder)
- Modify: `crates/muxsmith-core/Cargo.toml` (add serde)

**Interfaces:**
- Consumes: nothing.
- Produces (used by every later task):
  - `report::Severity` (`Error | Warning | Info`, `Ord`: Info < Warning < Error)
  - `report::DiagCode` (full v1 enum, see below) with `fn key(self) -> &'static str` (kebab-case, Fluent message id)
  - `report::Diagnostic { code, severity, config_path: String, file: Option<PathBuf>, params: BTreeMap<String, String>, suggestion_ref: Option<usize> }`
  - Builders: `Diagnostic::error(code, config_path)`, `::warning(...)`, `::info(...)`, `.with(key, value) -> Self`, `.for_file(path) -> Self`
  - `report::worst_severity(&[Diagnostic]) -> Option<Severity>`

- [ ] **Step 1: Add serde to the core crate**

```bash
cargo add -p muxsmith-core serde --features derive
cargo add -p muxsmith-core serde_json --dev
```

- [ ] **Step 2: Write the failing tests**

Append to `crates/muxsmith-core/src/report.rs` (replacing the placeholder test):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_orders_info_warning_error() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }

    #[test]
    fn diag_code_keys_are_kebab_case() {
        assert_eq!(DiagCode::InvalidRegex.key(), "invalid-regex");
        assert_eq!(DiagCode::AmbiguousRule.key(), "ambiguous-rule");
        assert_eq!(DiagCode::UnknownSettableProperty.key(), "unknown-settable-property");
    }

    #[test]
    fn diagnostic_builder_sets_fields() {
        let d = Diagnostic::error(DiagCode::InvalidRegex, "input.pattern")
            .with("detail", "unclosed group");
        assert_eq!(d.severity, Severity::Error);
        assert_eq!(d.config_path, "input.pattern");
        assert_eq!(d.params["detail"], "unclosed group");
        assert!(d.file.is_none());
    }

    #[test]
    fn worst_severity_picks_error_over_warning() {
        let diags = vec![
            Diagnostic::warning(DiagCode::ProvableOverlap, "tracks[0]"),
            Diagnostic::error(DiagCode::InvalidRegex, "input.pattern"),
        ];
        assert_eq!(worst_severity(&diags), Some(Severity::Error));
        assert_eq!(worst_severity(&[]), None);
    }

    #[test]
    fn diagnostic_serializes_with_snake_case_severity_and_kebab_code() {
        let d = Diagnostic::error(DiagCode::UnknownProperty, "tracks[1].match.exact.foo")
            .with("property", "foo");
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["severity"], "error");
        assert_eq!(json["code"], "unknown-property");
        assert_eq!(json["params"]["property"], "foo");
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core report`
Expected: FAIL (types not defined)

- [ ] **Step 4: Implement the module**

`crates/muxsmith-core/src/report.rs` above the tests:

```rust
//! Diagnostics as data. Core never produces user-facing prose; renderers
//! (CLI, GUI) map `DiagCode::key()` + `params` to Fluent messages.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

macro_rules! diag_codes {
    ($($variant:ident => $key:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum DiagCode {
            $($variant),+
        }

        impl DiagCode {
            pub fn key(self) -> &'static str {
                match self {
                    $(DiagCode::$variant => $key),+
                }
            }
        }
    };
}

diag_codes! {
    // Config-time (validate)
    UnsupportedProfileVersion => "unsupported-profile-version",
    ParseError => "parse-error",
    NoTrackRules => "no-track-rules",
    EmptyMatchExpression => "empty-match-expression",
    EmptyExtensions => "empty-extensions",
    InvalidRegex => "invalid-regex",
    UnknownProperty => "unknown-property",
    NotStringProperty => "not-string-property",
    ValueTypeMismatch => "value-type-mismatch",
    UnknownSettableProperty => "unknown-settable-property",
    InvalidKeyword => "invalid-keyword",
    LocatorConflict => "locator-conflict",
    InvalidTemplate => "invalid-template",
    UnknownTemplateField => "unknown-template-field",
    UnknownTemplateFilter => "unknown-template-filter",
    PathSeparatorInTemplate => "path-separator-in-template",
    AttachmentRuleShape => "attachment-rule-shape",
    ProvableOverlap => "provable-overlap",
    // Planning-time (produced from Plan 2 on; defined now for a stable catalog)
    AmbiguousRule => "ambiguous-rule",
    OverlappingRules => "overlapping-rules",
    MissingTrack => "missing-track",
    MissingExternal => "missing-external",
    AmbiguousExternal => "ambiguous-external",
    OutputCollision => "output-collision",
    SourceOverwrite => "source-overwrite",
    DuplicateIdentifier => "duplicate-identifier",
    DonorIsPrimary => "donor-is-primary",
    IgnoredFile => "ignored-file",
    MultipleIdentifierMatches => "multiple-identifier-matches",
    UnknownPropertySkew => "unknown-property-skew",
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub severity: Severity,
    pub config_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<PathBuf>,
    pub params: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_ref: Option<usize>,
}

impl Diagnostic {
    fn new(code: DiagCode, severity: Severity, config_path: impl Into<String>) -> Self {
        Diagnostic {
            code,
            severity,
            config_path: config_path.into(),
            file: None,
            params: BTreeMap::new(),
            suggestion_ref: None,
        }
    }

    pub fn error(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Error, config_path)
    }

    pub fn warning(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Warning, config_path)
    }

    pub fn info(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Info, config_path)
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn for_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.file = Some(path.into());
        self
    }
}

pub fn worst_severity(diags: &[Diagnostic]) -> Option<Severity> {
    diags.iter().map(|d| d.severity).max()
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core report`
Expected: PASS (5 tests)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(core): diagnostic types with stable kebab-case catalog keys"
```

---

### Task 3: Match expression model

**Files:**
- Create: `crates/muxsmith-core/src/profile/mod.rs`
- Create: `crates/muxsmith-core/src/profile/match_expr.rs`
- Modify: `crates/muxsmith-core/src/lib.rs`

**Interfaces:**
- Consumes: nothing from earlier tasks.
- Produces:
  - `profile::match_expr::Scalar` (untagged: `Bool(bool) | Int(i64) | Float(f64) | Str(String)`) with `fn type_name(&self) -> &'static str` (`"boolean" | "integer" | "float" | "string"`)
  - `profile::match_expr::MatchExpr { exact: Option<BTreeMap<String, Scalar>>, substring: Option<BTreeMap<String, String>>, regex: Option<BTreeMap<String, String>>, any: Option<Vec<MatchExpr>>, not: Option<Vec<MatchExpr>> }` with `fn is_empty(&self) -> bool`
  - Both derive `Deserialize, Serialize, JsonSchema, Debug, Clone, PartialEq`; `MatchExpr` is `deny_unknown_fields`.

- [ ] **Step 1: Add dependencies**

```bash
cargo add -p muxsmith-core schemars
cargo add -p muxsmith-core --dev yaml-serde
```

(If `cargo add yaml-serde` fails because the crates.io name differs, check https://github.com/yaml/yaml-serde for the published crate name and use that; it is the YAML org's maintained fork of serde-yaml. Import path in code below assumes `yaml_serde`.)

- [ ] **Step 2: Write the failing tests**

`crates/muxsmith-core/src/profile/match_expr.rs`, tests at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_expression() {
        let y = r#"
exact: { type: subtitles, forced_track: true }
substring: { track_name: SDH }
"#;
        let e: MatchExpr = yaml_serde::from_str(y).unwrap();
        assert_eq!(
            e.exact.as_ref().unwrap()["type"],
            Scalar::Str("subtitles".into())
        );
        assert_eq!(e.exact.as_ref().unwrap()["forced_track"], Scalar::Bool(true));
        assert_eq!(e.substring.as_ref().unwrap()["track_name"], "SDH");
        assert!(e.any.is_none() && e.not.is_none());
        assert!(!e.is_empty());
    }

    #[test]
    fn parses_nested_any_and_not() {
        let y = r#"
exact: { type: subtitles }
not:
  - substring: { track_name: SDH }
  - exact: { flag_hearing_impaired: true }
any:
  - regex: { track_name: '(?i)forced' }
"#;
        let e: MatchExpr = yaml_serde::from_str(y).unwrap();
        assert_eq!(e.not.as_ref().unwrap().len(), 2);
        assert_eq!(e.any.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn rejects_unknown_keys() {
        let y = "exactt: { type: video }";
        assert!(yaml_serde::from_str::<MatchExpr>(y).is_err());
    }

    #[test]
    fn empty_expression_reports_empty() {
        let e: MatchExpr = yaml_serde::from_str("{}").unwrap();
        assert!(e.is_empty());
    }

    #[test]
    fn scalar_type_names() {
        assert_eq!(Scalar::Bool(true).type_name(), "boolean");
        assert_eq!(Scalar::Int(3).type_name(), "integer");
        assert_eq!(Scalar::Float(1.5).type_name(), "float");
        assert_eq!(Scalar::Str("x".into()).type_name(), "string");
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core match_expr`
Expected: FAIL (types not defined)

- [ ] **Step 4: Implement**

`crates/muxsmith-core/src/profile/match_expr.rs` above the tests:

```rust
//! Match algebra (spec 4.3): conjunction of exact/substring/regex maps
//! plus recursive `any` (at least one holds) and `not` (none hold).

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Scalar {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

impl Scalar {
    pub fn type_name(&self) -> &'static str {
        match self {
            Scalar::Bool(_) => "boolean",
            Scalar::Int(_) => "integer",
            Scalar::Float(_) => "float",
            Scalar::Str(_) => "string",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MatchExpr {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<BTreeMap<String, Scalar>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substring: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<MatchExpr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Vec<MatchExpr>>,
}

impl MatchExpr {
    pub fn is_empty(&self) -> bool {
        self.exact.as_ref().is_none_or(|m| m.is_empty())
            && self.substring.as_ref().is_none_or(|m| m.is_empty())
            && self.regex.as_ref().is_none_or(|m| m.is_empty())
            && self.any.as_ref().is_none_or(|v| v.is_empty())
            && self.not.as_ref().is_none_or(|v| v.is_empty())
    }
}
```

`crates/muxsmith-core/src/profile/mod.rs`:

```rust
pub mod match_expr;
```

`crates/muxsmith-core/src/lib.rs`:

```rust
pub mod profile;
pub mod report;
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core match_expr`
Expected: PASS (5 tests)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(core): match expression model with recursive any/not"
```

---

### Task 4: Full profile model and loader

**Files:**
- Create: `crates/muxsmith-core/src/profile/model.rs`
- Create: `crates/muxsmith-core/src/profile/load.rs`
- Create: `crates/muxsmith-core/tests/fixtures/reference.yaml`
- Modify: `crates/muxsmith-core/src/profile/mod.rs`
- Test: `crates/muxsmith-core/tests/profile_load.rs`

**Interfaces:**
- Consumes: `MatchExpr`, `Scalar` (Task 3); `Diagnostic`, `DiagCode` (Task 2).
- Produces (exact names used by Tasks 8-12):
  - `profile::Profile { profile_version: u32, meta: Option<Meta>, input: Input, output: OutputCfg, tracks: Vec<TrackRule>, attachments: AttachmentsCfg, chapters: ChaptersCfg, tags: TagsCfg, title: TitleCfg }`
  - `Meta { name: Option<String>, description: Option<String> }`
  - `Input { pattern: String, extensions: Vec<String>, recursive: bool /* default true */ }`
  - `OutputCfg { directory: Option<PathBuf>, filename: FilenameCfg, on_collision: CollisionPolicy }` with `Default` (None, Keyword("keep"), Error)
  - `FilenameCfg` / `TitleCfg` / `ChaptersCfg` / `SourceCfg`: untagged enums `Keyword(String)` or a struct variant (`Template { template: String }` for filename/title, `External { external: Locator }` for chapters/source)
  - `CollisionPolicy { Error, Skip, Overwrite }` (lowercase serde)
  - `KeepDrop { Keep, Drop }` (lowercase serde)
  - `TrackRule { source: SourceCfg /* default Keyword("primary") */, match_expr: MatchExpr /* serde rename "match" */, optional: bool, changes: Option<BTreeMap<String, Scalar>> }`
  - `Locator { path: String, recursive: bool /* default false */, extensions: Vec<String>, match_to_source: Option<bool>, match_pattern: Option<String>, case_sensitive: bool /* default false */ }`
  - `AttachmentsCfg { unmatched: KeepDrop /* default Keep */, rules: Vec<AttachmentRule> }`
  - `AttachmentRule { select: Option<MatchExpr>, drop: Option<MatchExpr>, add: Option<Locator> }`
  - `TagsCfg { global: KeepDrop, track: KeepDrop }` (both default Keep)
  - `profile::load::from_file(path: &Path) -> Result<Profile, Diagnostic>` and `from_str(text: &str, format: Format) -> Result<Profile, Diagnostic>` with `Format { Yaml, Json }`; parse failures return a `ParseError` diagnostic with params `detail` and `at` (serde path).
  - All model types derive `Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema`; all structs are `deny_unknown_fields`.

- [ ] **Step 1: Add dependencies**

```bash
cargo add -p muxsmith-core yaml-serde serde_json serde_path_to_error
```

(Move `yaml-serde` from dev-dependencies to dependencies if Task 3 left it dev-only.)

- [ ] **Step 2: Create the reference fixture**

`crates/muxsmith-core/tests/fixtures/reference.yaml` is the spec 4.1 example, completed with the German rules the spec elides. Copy the spec example verbatim, replace the `# analogous forced / plain / SDH rules for language: de omitted for brevity` comment with the three `de` rules (same shape as the three `en` subtitle rules with `language: de`, names `German forced` / `German` / `German SDH`), and set `directory: null` under `output`.

- [ ] **Step 3: Write the failing tests**

`crates/muxsmith-core/tests/profile_load.rs`:

```rust
use muxsmith_core::profile::model::{ChaptersCfg, FilenameCfg, KeepDrop, SourceCfg, TitleCfg};
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::report::DiagCode;

const REFERENCE: &str = include_str!("fixtures/reference.yaml");

#[test]
fn reference_profile_parses() {
    let p = from_str(REFERENCE, Format::Yaml).unwrap();
    assert_eq!(p.profile_version, 1);
    assert_eq!(p.input.pattern, r"S(?<season>\d{2})E(?<episode>\d{2})");
    assert!(p.input.recursive);
    assert_eq!(p.tracks.len(), 10); // 1 video + 2 audio + 6 subs + 1 external
    assert!(matches!(p.output.filename, FilenameCfg::Keyword(ref k) if k == "keep"));
    assert!(matches!(p.chapters, ChaptersCfg::Keyword(ref k) if k == "keep"));
    assert!(matches!(p.title, TitleCfg::Keyword(ref k) if k == "clear"));
    assert_eq!(p.tags.global, KeepDrop::Drop);
    assert_eq!(p.tags.track, KeepDrop::Keep);
    assert_eq!(p.attachments.unmatched, KeepDrop::Keep);

    let last = p.tracks.last().unwrap();
    match &last.source {
        SourceCfg::External { external } => {
            assert_eq!(external.path, ".");
            assert_eq!(external.extensions, vec!["srt"]);
            assert_eq!(external.match_to_source, Some(true));
            assert!(external.match_pattern.is_none());
        }
        other => panic!("expected external source, got {other:?}"),
    }
    let changes = last.changes.as_ref().unwrap();
    assert!(changes.contains_key("language") && changes.contains_key("track_name"));
}

#[test]
fn json_profile_parses_identically_to_yaml() {
    let yaml = r#"
profile_version: 1
input: { pattern: 'S(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
"#;
    let json = r#"{
  "profile_version": 1,
  "input": { "pattern": "S(\\d+)", "extensions": ["mkv"] },
  "tracks": [ { "match": { "exact": { "type": "video" } } } ]
}"#;
    let a = from_str(yaml, Format::Yaml).unwrap();
    let b = from_str(json, Format::Json).unwrap();
    assert_eq!(a, b);
}

#[test]
fn defaults_apply_when_sections_absent() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(matches!(p.output.filename, FilenameCfg::Keyword(ref k) if k == "keep"));
    assert_eq!(p.tags.global, KeepDrop::Keep);
    assert!(matches!(p.tracks[0].source, SourceCfg::Keyword(ref k) if k == "primary"));
    assert!(!p.tracks[0].optional);
}

#[test]
fn unknown_key_is_parse_error_with_path() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
    optionall: true
"#;
    let err = from_str(y, Format::Yaml).unwrap_err();
    assert_eq!(err.code, DiagCode::ParseError);
    assert!(err.params["detail"].contains("optionall"));
}
```

- [ ] **Step 4: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core --test profile_load`
Expected: FAIL (modules not defined)

- [ ] **Step 5: Implement the model**

`crates/muxsmith-core/src/profile/model.rs`:

```rust
//! Profile data model (spec 4). Serde-only: semantic validation lives in
//! validate.rs, so this file stays a faithful mirror of the file format.

use std::collections::BTreeMap;
use std::path::PathBuf;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::match_expr::{MatchExpr, Scalar};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub profile_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub input: Input,
    #[serde(default)]
    pub output: OutputCfg,
    pub tracks: Vec<TrackRule>,
    #[serde(default)]
    pub attachments: AttachmentsCfg,
    #[serde(default)]
    pub chapters: ChaptersCfg,
    #[serde(default)]
    pub tags: TagsCfg,
    #[serde(default)]
    pub title: TitleCfg,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub pattern: String,
    pub extensions: Vec<String>,
    #[serde(default = "default_true")]
    pub recursive: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OutputCfg {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<PathBuf>,
    #[serde(default = "FilenameCfg::keep")]
    pub filename: FilenameCfg,
    #[serde(default)]
    pub on_collision: CollisionPolicy,
}

impl Default for OutputCfg {
    fn default() -> Self {
        OutputCfg {
            directory: None,
            filename: FilenameCfg::keep(),
            on_collision: CollisionPolicy::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum FilenameCfg {
    Template {
        template: String,
    },
    Keyword(String),
}

impl FilenameCfg {
    pub fn keep() -> Self {
        FilenameCfg::Keyword("keep".into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CollisionPolicy {
    #[default]
    Error,
    Skip,
    Overwrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum KeepDrop {
    Keep,
    Drop,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TrackRule {
    #[serde(default = "SourceCfg::primary")]
    pub source: SourceCfg,
    #[serde(rename = "match")]
    pub match_expr: MatchExpr,
    #[serde(default)]
    pub optional: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<BTreeMap<String, Scalar>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum SourceCfg {
    External {
        external: Locator,
    },
    Keyword(String),
}

impl SourceCfg {
    pub fn primary() -> Self {
        SourceCfg::Keyword("primary".into())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Locator {
    pub path: String,
    #[serde(default)]
    pub recursive: bool,
    pub extensions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_to_source: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<String>,
    #[serde(default)]
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttachmentsCfg {
    #[serde(default = "keep")]
    pub unmatched: KeepDrop,
    #[serde(default)]
    pub rules: Vec<AttachmentRule>,
}

fn keep() -> KeepDrop {
    KeepDrop::Keep
}

impl Default for AttachmentsCfg {
    fn default() -> Self {
        AttachmentsCfg {
            unmatched: KeepDrop::Keep,
            rules: Vec::new(),
        }
    }
}

/// Exactly one of `select` / `drop` / `add` must be set; enforced in
/// validate.rs (AttachmentRuleShape) so the parse error stays readable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttachmentRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select: Option<MatchExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<MatchExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Locator>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum ChaptersCfg {
    External {
        external: Locator,
    },
    Keyword(String),
}

impl Default for ChaptersCfg {
    fn default() -> Self {
        ChaptersCfg::Keyword("keep".into())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TagsCfg {
    #[serde(default = "keep")]
    pub global: KeepDrop,
    #[serde(default = "keep")]
    pub track: KeepDrop,
}

impl Default for TagsCfg {
    fn default() -> Self {
        TagsCfg {
            global: KeepDrop::Keep,
            track: KeepDrop::Keep,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum TitleCfg {
    Template {
        template: String,
    },
    Keyword(String),
}

impl Default for TitleCfg {
    fn default() -> Self {
        TitleCfg::Keyword("keep".into())
    }
}
```

`crates/muxsmith-core/src/profile/load.rs`:

```rust
use std::fs;
use std::path::Path;

use crate::report::{DiagCode, Diagnostic};

use super::model::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Yaml,
    Json,
}

pub fn from_str(text: &str, format: Format) -> Result<Profile, Diagnostic> {
    match format {
        Format::Yaml => {
            let de = yaml_serde::Deserializer::from_str(text);
            serde_path_to_error::deserialize(de).map_err(|e| parse_error(&e))
        }
        Format::Json => {
            let mut de = serde_json::Deserializer::from_str(text);
            serde_path_to_error::deserialize(&mut de).map_err(|e| parse_error(&e))
        }
    }
}

pub fn from_file(path: &Path) -> Result<Profile, Diagnostic> {
    let format = match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Format::Json,
        _ => Format::Yaml, // .yaml, .yml, and anything else tries YAML
    };
    let text = fs::read_to_string(path).map_err(|e| {
        Diagnostic::error(DiagCode::ParseError, "")
            .for_file(path)
            .with("detail", e.to_string())
            .with("at", "")
    })?;
    from_str(&text, format).map_err(|d| d.for_file(path))
}

fn parse_error<E: std::fmt::Display>(err: &serde_path_to_error::Error<E>) -> Diagnostic {
    Diagnostic::error(DiagCode::ParseError, err.path().to_string())
        .with("detail", err.inner().to_string())
        .with("at", err.path().to_string())
}
```

`crates/muxsmith-core/src/profile/mod.rs`:

```rust
pub mod load;
pub mod match_expr;
pub mod model;

pub use model::Profile;
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core --test profile_load`
Expected: PASS (4 tests)

Note: if the untagged enums (`FilenameCfg` etc.) fail to roundtrip a bare string, check variant ORDER: the struct variant must be declared before `Keyword(String)` so maps are not swallowed by the string variant. The code above already orders them correctly.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(core): full profile model with loader and reference fixture"
```

---

### Task 5: Capability code generator (xtask)

**Files:**
- Create: `crates/xtask/Cargo.toml`, `crates/xtask/src/main.rs`, `crates/xtask/src/gen.rs`
- Create: `crates/xtask/tests/fixtures/mini-schema.json`
- Modify: `Cargo.toml` (workspace members)
- Test: `crates/xtask/tests/gen.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `cargo run -p xtask -- gen-capability <schema.json> <out.rs>` writing a Rust source file containing `pub static MATCHABLE_PROPERTIES: &[(&str, PropType)]`. Task 6 commits its output as `crates/muxsmith-core/src/capability/generated.rs`.
- The upstream schema file is an INPUT only. It is never committed and never shipped (spec 9 / decision log).

- [ ] **Step 1: Scaffold the xtask crate**

Add `"crates/xtask"` to workspace `members` in the root `Cargo.toml`.

`crates/xtask/Cargo.toml`:

```toml
[package]
name = "xtask"
version.workspace = true
edition.workspace = true
license.workspace = true
publish = false

[dependencies]
serde_json = "1"
```

- [ ] **Step 2: Create the mini-schema fixture**

`crates/xtask/tests/fixtures/mini-schema.json` (synthetic, mirrors the upstream structure without copying it):

```json
{
  "title": "synthetic mini schema for generator tests",
  "type": "object",
  "properties": {
    "tracks": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "codec": { "type": "string" },
          "id": { "type": "integer" },
          "type": { "type": "string" },
          "properties": {
            "type": "object",
            "properties": {
              "audio_channels": { "type": "integer" },
              "default_track": { "type": "boolean" },
              "display_dimensions": { "type": "string" },
              "forced_track": { "type": "boolean" },
              "language": { "type": "string" },
              "track_name": { "type": "string" }
            }
          }
        }
      }
    }
  }
}
```

- [ ] **Step 3: Write the failing test**

`crates/xtask/tests/gen.rs`:

```rust
use xtask::gen::generate;

#[test]
fn generates_matchable_table_from_schema() {
    let schema = include_str!("fixtures/mini-schema.json");
    let out = generate(schema).unwrap();
    // Track-level fields injected by the generator:
    assert!(out.contains(r#"("type", PropType::String)"#));
    assert!(out.contains(r#"("codec", PropType::String)"#));
    assert!(out.contains(r#"("id", PropType::Integer)"#));
    // Properties from the schema:
    assert!(out.contains(r#"("audio_channels", PropType::Integer)"#));
    assert!(out.contains(r#"("default_track", PropType::Boolean)"#));
    assert!(out.contains(r#"("language", PropType::String)"#));
    // Header marker so humans know not to edit:
    assert!(out.contains("GENERATED FILE"));
}

#[test]
fn rejects_schema_without_track_properties() {
    assert!(generate("{}").is_err());
}
```

Make xtask a lib+bin so the test can import it: add to `crates/xtask/Cargo.toml`:

```toml
[lib]
path = "src/lib.rs"
```

and create `crates/xtask/src/lib.rs`:

```rust
pub mod gen;
```

- [ ] **Step 4: Run test to verify it fails**

Run: `cargo test -p xtask`
Expected: FAIL (`gen` not defined)

- [ ] **Step 5: Implement the generator**

`crates/xtask/src/gen.rs`:

```rust
use serde_json::Value;

/// Extract matchable track property names and types from the mkvmerge
/// identification output schema. Only derived FACTS are emitted; the
/// schema text itself is never redistributed (spec 9).
pub fn generate(schema_json: &str) -> Result<String, String> {
    let schema: Value =
        serde_json::from_str(schema_json).map_err(|e| format!("invalid JSON: {e}"))?;

    let track_props = schema
        .pointer("/properties/tracks/items/properties/properties/properties")
        .and_then(Value::as_object)
        .ok_or("schema has no tracks.items.properties.properties.properties object")?;

    let mut entries: Vec<(String, &'static str)> = vec![
        // Track-level fields outside the nested properties object.
        ("type".into(), "String"),
        ("codec".into(), "String"),
        ("id".into(), "Integer"),
    ];

    for (name, def) in track_props {
        let prop_type = match def.get("type").and_then(Value::as_str) {
            Some("boolean") => "Boolean",
            Some("integer") => "Integer",
            Some("number") => "Float",
            // Strings, unions and anything exotic degrade to String:
            // matching still works, only exact-type checks get looser.
            _ => "String",
        };
        entries.push((name.clone(), prop_type));
    }
    entries.sort();
    entries.dedup_by(|a, b| a.0 == b.0);

    let mut out = String::new();
    out.push_str("// GENERATED FILE - do not edit.\n");
    out.push_str("// Regenerate: cargo run -p xtask -- gen-capability <schema.json> <this file>\n");
    out.push_str("// Source: mkvmerge identification output schema (facts only, not the schema).\n\n");
    out.push_str("use super::PropType;\n\n");
    out.push_str("pub static MATCHABLE_PROPERTIES: &[(&str, PropType)] = &[\n");
    for (name, ty) in &entries {
        out.push_str(&format!("    (\"{name}\", PropType::{ty}),\n"));
    }
    out.push_str("];\n");
    Ok(out)
}
```

`crates/xtask/src/main.rs`:

```rust
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("gen-capability") if args.len() == 4 => {
            let schema = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("cannot read {}: {e}", args[2]);
                process::exit(1);
            });
            let code = xtask::gen::generate(&schema).unwrap_or_else(|e| {
                eprintln!("generation failed: {e}");
                process::exit(1);
            });
            fs::write(&args[3], code).unwrap_or_else(|e| {
                eprintln!("cannot write {}: {e}", args[3]);
                process::exit(1);
            });
            eprintln!("wrote {}", args[3]);
        }
        _ => {
            eprintln!("usage: cargo run -p xtask -- gen-capability <schema.json> <out.rs>");
            process::exit(2);
        }
    }
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test -p xtask`
Expected: PASS (2 tests)

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(xtask): capability table generator from identification schema"
```

---

### Task 6: Capability module with generated table

**Files:**
- Create: `crates/muxsmith-core/src/capability/mod.rs`
- Create: `crates/muxsmith-core/src/capability/generated.rs` (via xtask, then committed)
- Modify: `crates/muxsmith-core/src/lib.rs`
- Test: inline `#[cfg(test)]` in `capability/mod.rs`

**Interfaces:**
- Consumes: xtask generator (Task 5).
- Produces (used by Tasks 8-10 and by Plan 2's matcher):
  - `capability::PropType { String, Boolean, Integer, Float }`
  - `capability::matchable_type(name: &str) -> Option<PropType>` (generated table, plus the virtual property `codec_kind` typed String)
  - `capability::settable(name: &str) -> Option<(PropType, &'static str)>` returning type and mkvmerge option
  - `capability::codec_kind_prefixes(kind: &str) -> Option<&'static [&'static str]>`
  - `capability::ATTACHMENT_PROPERTIES: &[(&str, PropType)]` (`file_name`, `content_type`, `description`, all String; `id`, `size` Integer)

- [ ] **Step 1: Generate the real table**

Download the newest published identification schema to the scratch area (NOT into the repo). Start at v21 and decrement on HTTP 404; v18 is a known-good floor:

```bash
for v in 21 20 19 18; do
  curl -fsSL -o /tmp/mkvmerge-schema.json \
    "https://mkvtoolnix.download/doc/mkvmerge-identification-output-schema-v$v.json" && \
    echo "got v$v" && break
done
cargo run -p xtask -- gen-capability /tmp/mkvmerge-schema.json \
  crates/muxsmith-core/src/capability/generated.rs
```

Record the obtained version number in a comment at the top of `capability/mod.rs` (e.g. `// Generated from identification schema v20.`).

- [ ] **Step 2: Write the failing tests**

Tests at the bottom of the new `crates/muxsmith-core/src/capability/mod.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matchable_types_from_generated_table() {
        assert_eq!(matchable_type("language"), Some(PropType::String));
        assert_eq!(matchable_type("forced_track"), Some(PropType::Boolean));
        assert_eq!(matchable_type("audio_channels"), Some(PropType::Integer));
        assert_eq!(matchable_type("type"), Some(PropType::String));
        assert_eq!(matchable_type("no_such_property"), None);
    }

    #[test]
    fn codec_kind_is_virtual_matchable() {
        assert_eq!(matchable_type("codec_kind"), Some(PropType::String));
        assert!(codec_kind_prefixes("srt").unwrap().contains(&"S_TEXT/UTF8"));
        assert!(codec_kind_prefixes("pgs").unwrap().contains(&"S_HDMV/PGS"));
        assert!(codec_kind_prefixes("h264").unwrap().contains(&"V_MPEG4/ISO/AVC"));
        assert!(codec_kind_prefixes("nope").is_none());
    }

    #[test]
    fn settable_maps_to_mkvmerge_options() {
        assert_eq!(
            settable("track_name"),
            Some((PropType::String, "--track-name"))
        );
        assert_eq!(
            settable("default_track"),
            Some((PropType::Boolean, "--default-track-flag"))
        );
        assert_eq!(
            settable("forced_track"),
            Some((PropType::Boolean, "--forced-display-flag"))
        );
        assert_eq!(settable("codec_kind"), None); // matchable only, never settable
    }

    #[test]
    fn attachment_properties_are_defined() {
        assert!(ATTACHMENT_PROPERTIES
            .iter()
            .any(|(n, t)| *n == "content_type" && *t == PropType::String));
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core capability`
Expected: FAIL (module not defined)

- [ ] **Step 4: Implement the module**

`crates/muxsmith-core/src/capability/mod.rs`:

```rust
//! mkvtoolnix capability model (spec 4.4 / 9). Matchable properties are
//! generated from the identification schema; settable properties and
//! codec_kind aliases are curated here. Runtime queries (--list-types,
//! --list-languages, --version) arrive in Plan 2.
// Schema version: recorded during generation (Task 6 step 1 wrote the
// actual version number here, e.g. "v20").

mod generated;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropType {
    String,
    Boolean,
    Integer,
    Float,
}

pub fn matchable_type(name: &str) -> Option<PropType> {
    if name == "codec_kind" {
        return Some(PropType::String);
    }
    generated::MATCHABLE_PROPERTIES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, t)| *t)
}

/// (profile name, value type, mkvmerge option) - spec 4.4 table.
pub static SETTABLE: &[(&str, PropType, &str)] = &[
    ("language", PropType::String, "--language"),
    ("track_name", PropType::String, "--track-name"),
    ("default_track", PropType::Boolean, "--default-track-flag"),
    ("forced_track", PropType::Boolean, "--forced-display-flag"),
    ("flag_hearing_impaired", PropType::Boolean, "--hearing-impaired-flag"),
    ("flag_visual_impaired", PropType::Boolean, "--visual-impaired-flag"),
    ("flag_commentary", PropType::Boolean, "--commentary-flag"),
    ("flag_original", PropType::Boolean, "--original-flag"),
    ("enabled_track", PropType::Boolean, "--track-enabled-flag"),
    ("sub_charset", PropType::String, "--sub-charset"),
];

pub fn settable(name: &str) -> Option<(PropType, &'static str)> {
    SETTABLE
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, t, o)| (*t, *o))
}

/// codec_kind aliases -> codec_id prefixes (prefix match at plan time).
pub static CODEC_KINDS: &[(&str, &[&str])] = &[
    ("srt", &["S_TEXT/UTF8"]),
    ("ass", &["S_TEXT/ASS", "S_TEXT/SSA"]),
    ("pgs", &["S_HDMV/PGS"]),
    ("vobsub", &["S_VOBSUB"]),
    ("webvtt", &["S_TEXT/WEBVTT"]),
    ("aac", &["A_AAC"]),
    ("ac3", &["A_AC3"]),
    ("eac3", &["A_EAC3"]),
    ("dts", &["A_DTS"]),
    ("truehd", &["A_TRUEHD"]),
    ("flac", &["A_FLAC"]),
    ("opus", &["A_OPUS"]),
    ("mp3", &["A_MPEG/L3"]),
    ("h264", &["V_MPEG4/ISO/AVC"]),
    ("h265", &["V_MPEGH/ISO/HEVC"]),
    ("av1", &["V_AV1"]),
    ("vp9", &["V_VP9"]),
];

pub fn codec_kind_prefixes(kind: &str) -> Option<&'static [&'static str]> {
    CODEC_KINDS
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, p)| *p)
}

/// Attachment match properties (spec 4.9); not part of the track schema.
pub static ATTACHMENT_PROPERTIES: &[(&str, PropType)] = &[
    ("content_type", PropType::String),
    ("description", PropType::String),
    ("file_name", PropType::String),
    ("id", PropType::Integer),
    ("size", PropType::Integer),
];
```

Add `pub mod capability;` to `crates/muxsmith-core/src/lib.rs`.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core capability`
Expected: PASS (4 tests)

- [ ] **Step 6: Commit (generated.rs IS committed; the downloaded schema is not)**

```bash
git add -A
git status   # confirm /tmp schema is not in the tree
git commit -m "feat(core): capability model with generated matchable table and curated settable set"
```

---

### Task 7: Template engine

**Files:**
- Create: `crates/muxsmith-core/src/template.rs`
- Modify: `crates/muxsmith-core/src/lib.rs`
- Modify: `crates/muxsmith-core/Cargo.toml` (add regex)

**Interfaces:**
- Consumes: nothing.
- Produces (used by Tasks 9 and Plans 2-3):
  - `template::Filter { Raw, Int, Pad2, Pad3 }`
  - `template::Template` with `fn parse(text: &str) -> Result<Template, TemplateError>`, `fn field_names(&self) -> Vec<&str>`, `fn render_literal(&self, ctx: &Ctx) -> String`, `fn render_regex_pattern(&self, ctx: &Ctx, case_sensitive: bool) -> String`
  - `template::Ctx` wrapping `BTreeMap<String, String>` with `Ctx::new()`, `.set(name, value)`; missing fields render as empty string (validation prevents that from being reachable)
  - `template::TemplateError { UnclosedBrace { pos: usize }, EmptyField { pos: usize }, UnknownFilter { name: String } }`
  - Escapes: `{{` and `}}` are literal braces.

- [ ] **Step 1: Add regex dependency**

```bash
cargo add -p muxsmith-core regex
```

- [ ] **Step 2: Write the failing tests**

Tests at the bottom of the new `crates/muxsmith-core/src/template.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(pairs: &[(&str, &str)]) -> Ctx {
        let mut c = Ctx::new();
        for (k, v) in pairs {
            c.set(k, v);
        }
        c
    }

    #[test]
    fn parses_fields_and_filters() {
        let t = Template::parse("Show - S{season}E{episode:pad2}.mkv").unwrap();
        assert_eq!(t.field_names(), vec!["season", "episode"]);
    }

    #[test]
    fn rejects_unknown_filter_and_unclosed_brace() {
        assert!(matches!(
            Template::parse("{season:frobnicate}"),
            Err(TemplateError::UnknownFilter { .. })
        ));
        assert!(matches!(
            Template::parse("S{season"),
            Err(TemplateError::UnclosedBrace { .. })
        ));
        assert!(matches!(
            Template::parse("S{}"),
            Err(TemplateError::EmptyField { .. })
        ));
    }

    #[test]
    fn renders_literal_with_filters() {
        let t = Template::parse("S{season:int}E{episode:pad3} of {show}").unwrap();
        let c = ctx(&[("season", "03"), ("episode", "1"), ("show", "X")]);
        assert_eq!(t.render_literal(&c), "S3E001 of X");
    }

    #[test]
    fn int_filter_keeps_single_zero() {
        let t = Template::parse("{n:int}").unwrap();
        assert_eq!(t.render_literal(&ctx(&[("n", "000")])), "0");
    }

    #[test]
    fn double_braces_are_literal() {
        let t = Template::parse("a{{b}}c").unwrap();
        assert_eq!(t.render_literal(&Ctx::new()), "a{b}c");
    }

    #[test]
    fn regex_mode_matches_spec_examples() {
        // Spec 4.7: primary matched as S03E01, targets use staffel naming.
        let t = Template::parse("staffel0*{season:int}episode0*{episode:int}").unwrap();
        let c = ctx(&[("season", "03"), ("episode", "01")]);
        let pattern = t.render_regex_pattern(&c, false);
        let re = regex::Regex::new(&pattern).unwrap();
        assert!(re.is_match("staffel03episode01"));
        assert!(re.is_match("staffel3episode01"));
        assert!(re.is_match("Staffel3Episode1"));
        assert!(!re.is_match("staffel4episode01"));
    }

    #[test]
    fn regex_mode_escapes_interpolated_values() {
        let t = Template::parse("{m}").unwrap();
        let c = ctx(&[("m", "a.b(c)")]);
        let pattern = t.render_regex_pattern(&c, true);
        let re = regex::Regex::new(&pattern).unwrap();
        assert!(re.is_match("xa.b(c)y"));
        assert!(!re.is_match("aXb(c)"));
    }

    #[test]
    fn case_sensitive_flag_controls_inline_i() {
        let t = Template::parse("abc").unwrap();
        assert!(t.render_regex_pattern(&Ctx::new(), false).starts_with("(?i)"));
        assert!(!t.render_regex_pattern(&Ctx::new(), true).starts_with("(?i)"));
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core template`
Expected: FAIL

- [ ] **Step 4: Implement**

`crates/muxsmith-core/src/template.rs` above the tests:

```rust
//! Template engine (spec 4.7). One parser, two render modes:
//! literal (output filenames, title) and regex pattern (external
//! locator match_pattern; interpolated values are regex-escaped).

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Raw,
    Int,
    Pad2,
    Pad3,
}

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    Literal(String),
    Field { name: String, filter: Filter },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateError {
    UnclosedBrace { pos: usize },
    EmptyField { pos: usize },
    UnknownFilter { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    segments: Vec<Segment>,
}

#[derive(Debug, Default)]
pub struct Ctx {
    values: BTreeMap<String, String>,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx::default()
    }

    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }

    fn get(&self, name: &str) -> &str {
        self.values.get(name).map(String::as_str).unwrap_or("")
    }
}

impl Template {
    pub fn parse(text: &str) -> Result<Template, TemplateError> {
        let mut segments = Vec::new();
        let mut literal = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '{' if chars.get(i + 1) == Some(&'{') => {
                    literal.push('{');
                    i += 2;
                }
                '}' if chars.get(i + 1) == Some(&'}') => {
                    literal.push('}');
                    i += 2;
                }
                '{' => {
                    let close = chars[i + 1..]
                        .iter()
                        .position(|&c| c == '}')
                        .ok_or(TemplateError::UnclosedBrace { pos: i })?;
                    let inner: String = chars[i + 1..i + 1 + close].iter().collect();
                    if inner.is_empty() {
                        return Err(TemplateError::EmptyField { pos: i });
                    }
                    let (name, filter) = match inner.split_once(':') {
                        None => (inner.as_str(), Filter::Raw),
                        Some((n, "int")) => (n, Filter::Int),
                        Some((n, "pad2")) => (n, Filter::Pad2),
                        Some((n, "pad3")) => (n, Filter::Pad3),
                        Some((_, f)) => {
                            return Err(TemplateError::UnknownFilter { name: f.to_string() })
                        }
                    };
                    if name.is_empty() {
                        return Err(TemplateError::EmptyField { pos: i });
                    }
                    if !literal.is_empty() {
                        segments.push(Segment::Literal(std::mem::take(&mut literal)));
                    }
                    segments.push(Segment::Field {
                        name: name.to_string(),
                        filter,
                    });
                    i += close + 2;
                }
                c => {
                    literal.push(c);
                    i += 1;
                }
            }
        }
        if !literal.is_empty() {
            segments.push(Segment::Literal(literal));
        }
        Ok(Template { segments })
    }

    pub fn field_names(&self) -> Vec<&str> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                Segment::Field { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

    pub fn render_literal(&self, ctx: &Ctx) -> String {
        self.render(ctx, false)
    }

    /// Render as a regex pattern: literal segments pass through as regex
    /// source, field values are escaped. Prefixes (?i) unless case_sensitive.
    pub fn render_regex_pattern(&self, ctx: &Ctx, case_sensitive: bool) -> String {
        let body = self.render(ctx, true);
        if case_sensitive {
            body
        } else {
            format!("(?i){body}")
        }
    }

    fn render(&self, ctx: &Ctx, escape_fields: bool) -> String {
        let mut out = String::new();
        for seg in &self.segments {
            match seg {
                Segment::Literal(l) => out.push_str(l),
                Segment::Field { name, filter } => {
                    let v = apply_filter(ctx.get(name), *filter);
                    if escape_fields {
                        out.push_str(&regex::escape(&v));
                    } else {
                        out.push_str(&v);
                    }
                }
            }
        }
        out
    }
}

fn apply_filter(value: &str, filter: Filter) -> String {
    match filter {
        Filter::Raw => value.to_string(),
        Filter::Int => {
            let stripped = value.trim_start_matches('0');
            if stripped.is_empty() {
                "0".to_string()
            } else {
                stripped.to_string()
            }
        }
        Filter::Pad2 => format!("{value:0>2}"),
        Filter::Pad3 => format!("{value:0>3}"),
    }
}
```

Add `pub mod template;` to `crates/muxsmith-core/src/lib.rs`.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core template`
Expected: PASS (8 tests)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(core): template engine with int/pad filters and dual render modes"
```

---

### Task 8: Semantic validation of match expressions and changes

**Files:**
- Create: `crates/muxsmith-core/src/profile/validate.rs`
- Modify: `crates/muxsmith-core/src/profile/mod.rs`
- Test: `crates/muxsmith-core/tests/validate_semantics.rs`

**Interfaces:**
- Consumes: `Profile` model (Task 4), `capability` (Task 6), `report` (Task 2).
- Produces: `profile::validate::validate(&Profile) -> Vec<Diagnostic>`. Task 9 EXTENDS this same function (input/locator/template checks); Task 12's CLI calls it. `config_path` strings use the shape `tracks[3].match.exact.language`, `tracks[9].source.external`, `attachments.rules[0]`.

- [ ] **Step 1: Write the failing tests**

`crates/muxsmith-core/tests/validate_semantics.rs`:

```rust
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::{DiagCode, Severity};

fn profile(tracks_yaml: &str) -> muxsmith_core::profile::Profile {
    let y = format!(
        r#"
profile_version: 1
input: {{ pattern: 'S(?<season>\d{{2}})E(?<episode>\d{{2}})', extensions: [mkv] }}
tracks:
{tracks_yaml}
"#
    );
    from_str(&y, Format::Yaml).unwrap()
}

fn codes(p: &muxsmith_core::profile::Profile) -> Vec<DiagCode> {
    validate(p).into_iter().map(|d| d.code).collect()
}

#[test]
fn reference_profile_validates_clean() {
    let text = include_str!("fixtures/reference.yaml");
    let p = from_str(text, Format::Yaml).unwrap();
    let errors: Vec<_> = validate(&p)
        .into_iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert_eq!(errors, vec![], "reference profile must have zero errors");
}

#[test]
fn wrong_profile_version_is_rejected() {
    let mut p = profile("  - match: { exact: { type: video } }");
    p.profile_version = 2;
    assert!(codes(&p).contains(&DiagCode::UnsupportedProfileVersion));
}

#[test]
fn empty_tracks_list_is_rejected() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::NoTrackRules));
}

#[test]
fn unknown_match_property_is_flagged_with_path() {
    let p = profile("  - match: { exact: { colour_depth: 10 } }");
    let diags = validate(&p);
    let d = diags.iter().find(|d| d.code == DiagCode::UnknownProperty).unwrap();
    assert_eq!(d.config_path, "tracks[0].match.exact.colour_depth");
    assert_eq!(d.params["property"], "colour_depth");
}

#[test]
fn substring_on_boolean_property_is_flagged() {
    let p = profile("  - match: { substring: { forced_track: 'yes' } }");
    assert!(codes(&p).contains(&DiagCode::NotStringProperty));
}

#[test]
fn exact_value_type_mismatch_is_flagged() {
    let p = profile("  - match: { exact: { forced_track: 'yes' } }");
    assert!(codes(&p).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn integer_accepted_for_float_property_but_not_reverse() {
    // audio_sampling_frequency is number (Float) in the schema.
    let ok = profile("  - match: { exact: { audio_sampling_frequency: 48000 } }");
    assert!(!codes(&ok).contains(&DiagCode::ValueTypeMismatch));
    let bad = profile("  - match: { exact: { audio_channels: 5.1 } }");
    assert!(codes(&bad).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn invalid_condition_regex_is_flagged() {
    let p = profile("  - match: { regex: { track_name: '([' } }");
    assert!(codes(&p).contains(&DiagCode::InvalidRegex));
}

#[test]
fn nested_any_and_not_are_validated_recursively() {
    let p = profile(
        "  - match:\n      any:\n        - exact: { nonexistent_prop: 1 }",
    );
    let diags = validate(&p);
    let d = diags.iter().find(|d| d.code == DiagCode::UnknownProperty).unwrap();
    assert_eq!(d.config_path, "tracks[0].match.any[0].exact.nonexistent_prop");
}

#[test]
fn empty_match_expression_is_warning() {
    let p = profile("  - match: {}");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::EmptyMatchExpression)
        .unwrap();
    assert_eq!(d.severity, Severity::Warning);
}

#[test]
fn unknown_change_property_is_flagged() {
    let p = profile(
        "  - match: { exact: { type: video } }\n    changes: { bitrate: 5000 }",
    );
    assert!(codes(&p).contains(&DiagCode::UnknownSettableProperty));
}

#[test]
fn change_value_type_mismatch_is_flagged() {
    let p = profile(
        "  - match: { exact: { type: video } }\n    changes: { default_track: 'yes' }",
    );
    assert!(codes(&p).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn attachment_rule_must_have_exactly_one_action() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
attachments:
  rules:
    - select: { substring: { content_type: font } }
      drop: { substring: { file_name: cover } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::AttachmentRuleShape));
}

#[test]
fn attachment_match_uses_attachment_property_set() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
attachments:
  rules:
    - select: { exact: { language: en } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    // "language" is a track property, not an attachment property.
    assert!(codes(&p).contains(&DiagCode::UnknownProperty));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core --test validate_semantics`
Expected: FAIL (`validate` not defined)

The `include_str!("fixtures/reference.yaml")` path resolves relative to the test file, so Task 4's fixture is already in place; nothing to copy.

- [ ] **Step 3: Implement**

`crates/muxsmith-core/src/profile/validate.rs`:

```rust
//! Semantic validation (spec 5.4 static checks, config-time part).
//! Task 9 extends this file with input/locator/template validation.

use std::collections::BTreeMap;

use crate::capability::{self, PropType};
use crate::report::{DiagCode, Diagnostic};

use super::match_expr::{MatchExpr, Scalar};
use super::model::{AttachmentRule, Profile};

pub fn validate(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if profile.profile_version != 1 {
        diags.push(
            Diagnostic::error(DiagCode::UnsupportedProfileVersion, "profile_version")
                .with("found", profile.profile_version.to_string())
                .with("supported", "1"),
        );
    }

    if profile.tracks.is_empty() {
        diags.push(Diagnostic::error(DiagCode::NoTrackRules, "tracks"));
    }

    for (i, rule) in profile.tracks.iter().enumerate() {
        let base = format!("tracks[{i}]");
        if rule.match_expr.is_empty() {
            diags.push(Diagnostic::warning(
                DiagCode::EmptyMatchExpression,
                format!("{base}.match"),
            ));
        }
        validate_expr(
            &rule.match_expr,
            &format!("{base}.match"),
            track_prop_type,
            &mut diags,
        );
        if let Some(changes) = &rule.changes {
            validate_changes(changes, &format!("{base}.changes"), &mut diags);
        }
    }

    for (i, rule) in profile.attachments.rules.iter().enumerate() {
        let base = format!("attachments.rules[{i}]");
        validate_attachment_rule(rule, &base, &mut diags);
    }

    diags
}

fn track_prop_type(name: &str) -> Option<PropType> {
    capability::matchable_type(name)
}

fn attachment_prop_type(name: &str) -> Option<PropType> {
    capability::ATTACHMENT_PROPERTIES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, t)| *t)
}

fn validate_attachment_rule(rule: &AttachmentRule, base: &str, diags: &mut Vec<Diagnostic>) {
    let actions = [
        rule.select.is_some(),
        rule.drop.is_some(),
        rule.add.is_some(),
    ]
    .iter()
    .filter(|b| **b)
    .count();
    if actions != 1 {
        diags.push(
            Diagnostic::error(DiagCode::AttachmentRuleShape, base.to_string())
                .with("found", actions.to_string()),
        );
    }
    if let Some(expr) = &rule.select {
        validate_expr(expr, &format!("{base}.select"), attachment_prop_type, diags);
    }
    if let Some(expr) = &rule.drop {
        validate_expr(expr, &format!("{base}.drop"), attachment_prop_type, diags);
    }
    // rule.add locator validation arrives with Task 9.
}

fn validate_expr(
    expr: &MatchExpr,
    path: &str,
    prop_type: fn(&str) -> Option<PropType>,
    diags: &mut Vec<Diagnostic>,
) {
    if let Some(exact) = &expr.exact {
        for (prop, value) in exact {
            let p = format!("{path}.exact.{prop}");
            match prop_type(prop) {
                None => diags.push(unknown_property(&p, prop)),
                Some(t) => {
                    if !scalar_fits(value, t) {
                        diags.push(
                            Diagnostic::error(DiagCode::ValueTypeMismatch, p)
                                .with("property", prop.clone())
                                .with("expected", type_label(t))
                                .with("found", value.type_name()),
                        );
                    }
                }
            }
        }
    }
    for (map, kind) in [(&expr.substring, "substring"), (&expr.regex, "regex")] {
        if let Some(map) = map {
            for (prop, value) in map.iter() {
                let p = format!("{path}.{kind}.{prop}");
                match prop_type(prop) {
                    None => diags.push(unknown_property(&p, prop)),
                    Some(PropType::String) => {}
                    Some(t) => diags.push(
                        Diagnostic::error(DiagCode::NotStringProperty, p.clone())
                            .with("property", prop.clone())
                            .with("actual_type", type_label(t))
                            .with("condition", kind.to_string()),
                    ),
                }
                if kind == "regex" {
                    if let Err(e) = regex::Regex::new(value) {
                        diags.push(
                            Diagnostic::error(DiagCode::InvalidRegex, p)
                                .with("detail", e.to_string()),
                        );
                    }
                }
            }
        }
    }
    if let Some(any) = &expr.any {
        for (i, sub) in any.iter().enumerate() {
            validate_expr(sub, &format!("{path}.any[{i}]"), prop_type, diags);
        }
    }
    if let Some(not) = &expr.not {
        for (i, sub) in not.iter().enumerate() {
            validate_expr(sub, &format!("{path}.not[{i}]"), prop_type, diags);
        }
    }
}

fn validate_changes(
    changes: &BTreeMap<String, Scalar>,
    path: &str,
    diags: &mut Vec<Diagnostic>,
) {
    for (prop, value) in changes {
        let p = format!("{path}.{prop}");
        match capability::settable(prop) {
            None => diags.push(
                Diagnostic::error(DiagCode::UnknownSettableProperty, p)
                    .with("property", prop.clone()),
            ),
            Some((t, _option)) => {
                if !scalar_fits(value, t) {
                    diags.push(
                        Diagnostic::error(DiagCode::ValueTypeMismatch, p)
                            .with("property", prop.clone())
                            .with("expected", type_label(t))
                            .with("found", value.type_name()),
                    );
                }
            }
        }
    }
}

fn unknown_property(path: &str, prop: &str) -> Diagnostic {
    Diagnostic::error(DiagCode::UnknownProperty, path.to_string()).with("property", prop)
}

fn scalar_fits(value: &Scalar, t: PropType) -> bool {
    matches!(
        (value, t),
        (Scalar::Str(_), PropType::String)
            | (Scalar::Bool(_), PropType::Boolean)
            | (Scalar::Int(_), PropType::Integer)
            | (Scalar::Int(_), PropType::Float)
            | (Scalar::Float(_), PropType::Float)
    )
}

fn type_label(t: PropType) -> &'static str {
    match t {
        PropType::String => "string",
        PropType::Boolean => "boolean",
        PropType::Integer => "integer",
        PropType::Float => "float",
    }
}
```

Add `pub mod validate;` to `crates/muxsmith-core/src/profile/mod.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core --test validate_semantics`
Expected: PASS (14 tests)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(core): semantic validation of match expressions, changes, attachment rules"
```

---

### Task 9: Input, locator, keyword and template validation

**Files:**
- Modify: `crates/muxsmith-core/src/profile/validate.rs`
- Test: `crates/muxsmith-core/tests/validate_structure.rs`

**Interfaces:**
- Consumes: everything from Task 8 (extends the same `validate` function), `template::Template` (Task 7).
- Produces: complete config-time validation. After this task, `validate` covers every check in spec 5.4's config-time scope except the overlap lint (Task 10).

- [ ] **Step 1: Write the failing tests**

`crates/muxsmith-core/tests/validate_structure.rs`:

```rust
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::DiagCode;

fn parse(y: &str) -> muxsmith_core::profile::Profile {
    from_str(y, Format::Yaml).unwrap()
}

fn codes(y: &str) -> Vec<DiagCode> {
    validate(&parse(y)).into_iter().map(|d| d.code).collect()
}

const BASE: &str = r#"
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
"#;

#[test]
fn invalid_input_pattern_is_flagged() {
    let y = BASE.replace(r"S(?<season>\d{2})E(?<episode>\d{2})", "([");
    assert!(codes(&y).contains(&DiagCode::InvalidRegex));
}

#[test]
fn empty_extensions_flagged_for_input_and_locator() {
    let y = BASE.replace("extensions: [mkv]", "extensions: []");
    assert!(codes(&y).contains(&DiagCode::EmptyExtensions));
}

#[test]
fn locator_with_both_match_options_is_conflict() {
    let y = format!(
        "{BASE}  - source:\n      external: {{ path: '.', extensions: [srt], match_to_source: true, match_pattern: '{{match}}' }}\n    match: {{ exact: {{ type: subtitles }} }}\n"
    );
    assert!(codes(&y).contains(&DiagCode::LocatorConflict));
}

#[test]
fn match_pattern_with_unknown_field_is_flagged() {
    let y = format!(
        "{BASE}  - source:\n      external: {{ path: '.', extensions: [srt], match_pattern: 'x{{volume}}y' }}\n    match: {{ exact: {{ type: subtitles }} }}\n"
    );
    let c = codes(&y);
    assert!(c.contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn match_pattern_may_not_use_source_stem() {
    // source_stem is literal-mode only (spec 4.7).
    let y = format!(
        "{BASE}  - source:\n      external: {{ path: '.', extensions: [srt], match_pattern: '{{source_stem}}' }}\n    match: {{ exact: {{ type: subtitles }} }}\n"
    );
    assert!(codes(&y).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn filename_template_fields_checked_against_pattern_groups() {
    let good =
        BASE.to_string() + "output:\n  filename: { template: 'X S{season}E{episode:pad2}.mkv' }\n";
    assert!(!codes(&good).contains(&DiagCode::UnknownTemplateField));

    let bad = BASE.to_string()
        + "output:\n  filename: { template: 'X {show}.mkv' }\n";
    assert!(codes(&bad).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn filename_template_with_path_separator_is_flagged() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'sub/dir{season}.mkv' }\n";
    assert!(codes(&y).contains(&DiagCode::PathSeparatorInTemplate));
}

#[test]
fn bad_template_syntax_is_invalid_template() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'S{season' }\n";
    assert!(codes(&y).contains(&DiagCode::InvalidTemplate));
}

#[test]
fn unknown_keywords_are_flagged() {
    for (snippet, _section) in [
        ("chapters: discard\n", "chapters"),
        ("title: wipe\n", "title"),
    ] {
        let y = BASE.to_string() + snippet;
        assert!(
            codes(&y).contains(&DiagCode::InvalidKeyword),
            "expected InvalidKeyword for: {snippet}"
        );
    }
    let y = BASE.replace(
        "- match: { exact: { type: video } }",
        "- source: secondary\n    match: { exact: { type: video } }",
    );
    assert!(codes(&y).contains(&DiagCode::InvalidKeyword));
}

#[test]
fn numbered_group_fields_are_accepted() {
    let y = r#"
profile_version: 1
input: { pattern: 'S(\d{2})E(\d{2})', extensions: [mkv] }
output:
  filename: { template: 'S{g1}E{g2}.mkv' }
tracks:
  - match: { exact: { type: video } }
"#;
    assert!(!codes(y).contains(&DiagCode::UnknownTemplateField));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core --test validate_structure`
Expected: FAIL (new checks missing; some tests may pass incidentally, at least pattern/locator/template ones must fail)

- [ ] **Step 3: Extend `validate`**

Add to `crates/muxsmith-core/src/profile/validate.rs` (new code, integrated into the existing `validate` function; new imports at top):

```rust
use crate::template::{Template, TemplateError};

use super::model::{ChaptersCfg, FilenameCfg, Locator, SourceCfg, TitleCfg};
```

Inside `validate`, after the version check, add input validation and compute the allowed template fields:

```rust
    // input.pattern must compile; its groups define the template fields.
    let mut template_fields: Vec<String> = vec!["match".into()];
    match regex::Regex::new(&profile.input.pattern) {
        Err(e) => diags.push(
            Diagnostic::error(DiagCode::InvalidRegex, "input.pattern")
                .with("detail", e.to_string()),
        ),
        Ok(re) => {
            for (i, name) in re.capture_names().enumerate() {
                if i == 0 {
                    continue; // group 0 is the whole match
                }
                template_fields.push(format!("g{i}"));
                if let Some(n) = name {
                    template_fields.push(n.to_string());
                }
            }
        }
    }

    if profile.input.extensions.is_empty() {
        diags.push(Diagnostic::error(DiagCode::EmptyExtensions, "input.extensions"));
    }
```

In the per-track loop, add source validation:

```rust
        match &rule.source {
            SourceCfg::Keyword(k) if k == "primary" => {}
            SourceCfg::Keyword(k) => diags.push(
                Diagnostic::error(DiagCode::InvalidKeyword, format!("{base}.source"))
                    .with("found", k.clone())
                    .with("allowed", "primary"),
            ),
            SourceCfg::External { external } => {
                validate_locator(external, &format!("{base}.source.external"), &template_fields, false, &mut diags);
            }
        }
```

After the attachments loop, add section keyword/template validation:

```rust
    // output.filename
    match &profile.output.filename {
        FilenameCfg::Keyword(k) if k == "keep" => {}
        FilenameCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "output.filename")
                .with("found", k.clone())
                .with("allowed", "keep"),
        ),
        FilenameCfg::Template { template } => {
            let mut fields = template_fields.clone();
            fields.push("source_stem".into());
            validate_template(template, "output.filename.template", &fields, true, &mut diags);
        }
    }

    match &profile.chapters {
        ChaptersCfg::Keyword(k) if k == "keep" || k == "drop" => {}
        ChaptersCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "chapters")
                .with("found", k.clone())
                .with("allowed", "keep, drop"),
        ),
        ChaptersCfg::External { external } => {
            validate_locator(external, "chapters.external", &template_fields, false, &mut diags);
        }
    }

    match &profile.title {
        TitleCfg::Keyword(k) if k == "keep" || k == "clear" => {}
        TitleCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "title")
                .with("found", k.clone())
                .with("allowed", "keep, clear"),
        ),
        TitleCfg::Template { template } => {
            let mut fields = template_fields.clone();
            fields.push("source_stem".into());
            validate_template(template, "title.template", &fields, false, &mut diags);
        }
    }
```

In `validate_attachment_rule`, replace the `rule.add` comment with locator validation (pass `template_fields` through as a parameter; adjust the function signature and its call site accordingly):

```rust
    if let Some(locator) = &rule.add {
        validate_locator(locator, &format!("{base}.add"), template_fields, false, diags);
    }
```

New helper functions at the bottom of the file:

```rust
fn validate_locator(
    locator: &Locator,
    path: &str,
    template_fields: &[String],
    _reserved: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if locator.extensions.is_empty() {
        diags.push(Diagnostic::error(
            DiagCode::EmptyExtensions,
            format!("{path}.extensions"),
        ));
    }
    if locator.match_to_source.is_some() && locator.match_pattern.is_some() {
        diags.push(Diagnostic::error(DiagCode::LocatorConflict, path.to_string()));
    }
    if let Some(pattern) = &locator.match_pattern {
        // source_stem is literal-mode only: do NOT add it here (spec 4.7).
        validate_template(
            pattern,
            &format!("{path}.match_pattern"),
            template_fields,
            false,
            diags,
        );
    }
}

fn validate_template(
    text: &str,
    path: &str,
    allowed_fields: &[String],
    forbid_path_separators: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let template = match Template::parse(text) {
        Ok(t) => t,
        Err(e) => {
            let (code, detail) = match e {
                TemplateError::UnknownFilter { name } => (
                    DiagCode::UnknownTemplateFilter,
                    format!("unknown filter: {name}"),
                ),
                TemplateError::UnclosedBrace { pos } => {
                    (DiagCode::InvalidTemplate, format!("unclosed brace at {pos}"))
                }
                TemplateError::EmptyField { pos } => {
                    (DiagCode::InvalidTemplate, format!("empty field at {pos}"))
                }
            };
            diags.push(Diagnostic::error(code, path.to_string()).with("detail", detail));
            return;
        }
    };
    for field in template.field_names() {
        if !allowed_fields.iter().any(|f| f == field) {
            diags.push(
                Diagnostic::error(DiagCode::UnknownTemplateField, path.to_string())
                    .with("field", field)
                    .with("allowed", allowed_fields.join(", ")),
            );
        }
    }
    if forbid_path_separators && (text.contains('/') || text.contains('\\')) {
        diags.push(Diagnostic::error(
            DiagCode::PathSeparatorInTemplate,
            path.to_string(),
        ));
    }
}
```

Note the `validate_expr` / `validate_attachment_rule` signatures change to thread `&template_fields` where needed; update the call sites shown in Task 8 accordingly.

- [ ] **Step 4: Run all validation tests**

Run: `cargo test -p muxsmith-core --test validate_structure --test validate_semantics`
Expected: PASS (all tests, both files)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(core): input, locator, keyword and template validation"
```

---

### Task 10: Static overlap lint

**Files:**
- Create: `crates/muxsmith-core/src/profile/lint.rs`
- Modify: `crates/muxsmith-core/src/profile/mod.rs`
- Test: inline `#[cfg(test)]` in `lint.rs`

**Interfaces:**
- Consumes: `Profile`, `MatchExpr`, `Scalar` (Tasks 3-4), `report` (Task 2).
- Produces: `profile::lint::provable_overlaps(&Profile) -> Vec<Diagnostic>` emitting `ProvableOverlap` warnings. CLI (Task 12) appends these to `validate`'s output.

- [ ] **Step 1: Write the failing tests**

Tests at the bottom of new `crates/muxsmith-core/src/profile/lint.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::load::{from_str, Format};
    use crate::report::DiagCode;

    fn lint(y: &str) -> Vec<crate::report::Diagnostic> {
        provable_overlaps(&from_str(y, Format::Yaml).unwrap())
    }

    #[test]
    fn subset_conditions_are_provable_overlap() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio } }
  - match: { exact: { type: audio, language: en } }
"#;
        let diags = lint(y);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagCode::ProvableOverlap);
        assert_eq!(diags[0].params["rule_a"], "0");
        assert_eq!(diags[0].params["rule_b"], "1");
    }

    #[test]
    fn identical_exact_rules_are_provable_overlap() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: en } }
  - match: { exact: { type: audio, language: en } }
"#;
        assert_eq!(lint(y).len(), 1);
    }

    #[test]
    fn disjoint_exact_values_are_not_flagged() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: en } }
  - match: { exact: { type: audio, language: de } }
"#;
        assert!(lint(y).is_empty());
    }

    #[test]
    fn rules_with_negations_or_regex_are_skipped() {
        // Not exact-only: undecidable statically, planner handles it.
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles } }
  - match:
      exact: { type: subtitles }
      not:
        - substring: { track_name: SDH }
"#;
        assert!(lint(y).is_empty());
    }

    #[test]
    fn external_source_rules_are_skipped() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles } }
  - source:
      external: { path: '.', extensions: [srt], match_to_source: true }
    match: { exact: { type: subtitles } }
"#;
        assert!(lint(y).is_empty());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core lint`
Expected: FAIL

- [ ] **Step 3: Implement**

`crates/muxsmith-core/src/profile/lint.rs` above the tests:

```rust
//! Static overlap lint (spec 5.4): flags PROVABLE overlaps only.
//! Decidable case: two primary-source rules whose expressions are
//! exact-only, where one condition map is a subset of the other. Any
//! track matching the superset rule then necessarily matches the
//! subset rule. Everything else is left to the planner's dry run.

use crate::report::{DiagCode, Diagnostic};

use super::match_expr::MatchExpr;
use super::model::{Profile, SourceCfg};

pub fn provable_overlaps(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let exact_only: Vec<(usize, &MatchExpr)> = profile
        .tracks
        .iter()
        .enumerate()
        .filter(|(_, r)| matches!(&r.source, SourceCfg::Keyword(k) if k == "primary"))
        .filter(|(_, r)| is_exact_only(&r.match_expr))
        .map(|(i, r)| (i, &r.match_expr))
        .collect();

    for (ai, (a_idx, a)) in exact_only.iter().enumerate() {
        for (b_idx, b) in exact_only.iter().skip(ai + 1) {
            if subset_of(a, b) || subset_of(b, a) {
                diags.push(
                    Diagnostic::warning(
                        DiagCode::ProvableOverlap,
                        format!("tracks[{b_idx}]"),
                    )
                    .with("rule_a", a_idx.to_string())
                    .with("rule_b", b_idx.to_string()),
                );
            }
        }
    }
    diags
}

fn is_exact_only(e: &MatchExpr) -> bool {
    e.substring.is_none() && e.regex.is_none() && e.any.is_none() && e.not.is_none()
        && e.exact.as_ref().is_some_and(|m| !m.is_empty())
}

/// True if every condition in `a` also exists identically in `b`:
/// then any track matching `b` also matches `a`.
fn subset_of(a: &MatchExpr, b: &MatchExpr) -> bool {
    let (Some(a), Some(b)) = (&a.exact, &b.exact) else {
        return false;
    };
    a.iter().all(|(k, v)| b.get(k) == Some(v))
}
```

Add `pub mod lint;` to `crates/muxsmith-core/src/profile/mod.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core lint`
Expected: PASS (5 tests)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(core): static lint for provable rule overlaps"
```

---

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

### Task 13: Catalog completeness guard and CI

**Files:**
- Test: `crates/muxsmith-cli/tests/catalog_completeness.rs`
- Create: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: `DiagCode` (Task 2), catalogs (Task 12).
- Produces: the spec 10 guard "CI fails on diagnostic codes without message templates", plus the CI matrix. (The help-id guard arrives with the GUI plan.)

- [ ] **Step 1: Write the failing-or-passing completeness test**

`crates/muxsmith-cli/tests/catalog_completeness.rs`:

```rust
//! Spec 10: every DiagCode must have a message template in the English
//! catalog. This test IS the CI guard.

const CATALOG: &str = include_str!("../../../locales/en/diagnostics.ftl");

// Keep in sync with report::DiagCode::key() - the test fails loudly when a
// new code is added without a catalog entry, which is exactly its job.
const ALL_KEYS: &[&str] = &[
    "unsupported-profile-version",
    "parse-error",
    "no-track-rules",
    "empty-match-expression",
    "empty-extensions",
    "invalid-regex",
    "unknown-property",
    "not-string-property",
    "value-type-mismatch",
    "unknown-settable-property",
    "invalid-keyword",
    "locator-conflict",
    "invalid-template",
    "unknown-template-field",
    "unknown-template-filter",
    "path-separator-in-template",
    "attachment-rule-shape",
    "provable-overlap",
    "ambiguous-rule",
    "overlapping-rules",
    "missing-track",
    "missing-external",
    "ambiguous-external",
    "output-collision",
    "source-overwrite",
    "duplicate-identifier",
    "donor-is-primary",
    "ignored-file",
    "multiple-identifier-matches",
    "unknown-property-skew",
];

#[test]
fn every_diag_code_has_a_catalog_message() {
    let missing: Vec<&str> = ALL_KEYS
        .iter()
        .filter(|key| {
            !CATALOG
                .lines()
                .any(|l| l.starts_with(&format!("{key} =")))
        })
        .copied()
        .collect();
    assert_eq!(missing, Vec::<&str>::new(), "missing catalog entries");
}
```

- [ ] **Step 2: Run the test**

Run: `cargo test -p muxsmith-cli --test catalog_completeness`
Expected: PASS (Task 12 wrote all entries; if it fails, add the missing lines to the catalog now)

- [ ] **Step 3: Create the CI workflow**

`.github/workflows/ci.yml`:

```yaml
name: ci
on:
  push:
    branches: [master]
  pull_request:

jobs:
  test:
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all --check
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo test --workspace
```

- [ ] **Step 4: Verify locally what CI will run**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
Expected: all three PASS (fix any fmt/clippy fallout now)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "ci: test matrix, lint gates, and diagnostic catalog completeness guard"
```

---

## Plan 1 exit criteria

- `cargo test --workspace` green on Linux (CI proves Windows/macOS).
- `muxsmith validate docs/examples/reference.yaml` style invocation works against the test fixture: exit 0.
- `muxsmith validate` on a profile with an ambiguity-prone ruleset renders actionable, localized messages with exact config paths; exit codes 0/1/2 correct.
- `muxsmith schema` emits the generated JSON Schema.
- No user-facing prose anywhere in `muxsmith-core` (review gate: grep string literals in core for sentence-like text).
- `generated.rs` committed; no upstream schema file in the tree.

## Deferred to later plans

- Plan 2: `identify` (mkvmerge -J + cache), `matcher` evaluation against real tracks, language normalization via `--list-languages`, `planner` + batch report + suggestion engine, `dry-run` and `identify` subcommands, runtime capability queries (`--version`, `--list-types`), file discovery (pattern scan, external locator resolution).
- Plan 3: `command` argv generation, `executor` with `--gui-mode` progress, job queue, `run` subcommand, integration tests against real mkvmerge fixtures.
- Plan 4: Tauri shell, React GUI (profile editor, batch view, job queue), help mode with help-id completeness guard, frontend Fluent wiring, packaging.
