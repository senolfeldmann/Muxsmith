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

/// Shared `{ template: ... }` block; a standalone struct (not an inline
/// enum variant) because serde ignores `deny_unknown_fields` on struct
/// variants of untagged enums.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TemplateBlock {
    pub template: String,
}

/// Shared `{ external: ... }` block; standalone for the same
/// `deny_unknown_fields` reason as [`TemplateBlock`].
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExternalBlock {
    pub external: Locator,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum FilenameCfg {
    Template(TemplateBlock),
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
    External(ExternalBlock),
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
    External(ExternalBlock),
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
    Template(TemplateBlock),
    Keyword(String),
}

impl Default for TitleCfg {
    fn default() -> Self {
        TitleCfg::Keyword("keep".into())
    }
}
