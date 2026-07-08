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

