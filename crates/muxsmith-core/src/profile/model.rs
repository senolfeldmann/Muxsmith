//! Profile data model (spec 4). Serde-only: semantic validation lives in
//! validate.rs, so this file stays a faithful mirror of the file format.

use std::collections::BTreeMap;
use std::path::PathBuf;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

use super::match_expr::{MatchExpr, Scalar};

/// Shared schema shape for a keyword-domain projection (D46): a closed
/// `enum` of the domain's values, not `oneOf` + per-value `const` (rejected;
/// there is no per-value prose to attach, see D46). Used by the four
/// `Keyword` arm `schema_with` overrides below; the doc comment on the
/// variant itself still merges in as the branch's `description`, so this
/// helper does not set one.
fn keyword_domain_schema(domain: &'static [&'static str]) -> Schema {
    json_schema!({
        "type": "string",
        "enum": domain,
    })
}

/// D48: a canonical save omits a field sitting on its default. Generic
/// predicate for 13 of the 17 defaulted fields, the ones whose serde
/// default *is* `Default::default()`. Not usable for `FilenameCfg` or
/// `SourceCfg` (neither implements `Default`, so instantiation is
/// `E0277`), and wrong for `TracksCfg.unmatched` and `Input.recursive`
/// (both compile against this generic bound but silently omit a
/// non-default value, since their serde default is a named function, not
/// `Default::default()`); those four use the bespoke predicates below
/// instead, one per named default function.
fn is_default<T: Default + PartialEq>(v: &T) -> bool {
    *v == T::default()
}

/// The top-level profile document (spec 4): input matching, output naming,
/// track rules (in output order), and attachments/chapters/tags/title
/// configuration. Deserializes with `deny_unknown_fields` throughout: an
/// unrecognized key is a config error, not a silently ignored typo (spec 4:
/// "Unknown keys are errors, not warnings").
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct Profile {
    /// Format version; must equal `1` in v1 (`UnsupportedProfileVersion` if
    /// not). Incremented only on breaking format changes (spec 4).
    pub profile_version: u32,
    /// Optional descriptive metadata; purely informational, never
    /// interpreted by matching or planning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    /// Primary-file matching rule and directory-walk options (spec 4.2).
    pub input: Input,
    /// Output naming and collision policy (spec 4.8).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(OutputCfg::default()).unwrap()))]
    pub output: OutputCfg,
    /// Track selection/change rules plus the unmatched-track policy
    /// (spec 4.5). Restructured into a block so the policy lives with its
    /// rules, matching `attachments` and `output`/`tags`.
    pub tracks: TracksCfg,
    /// Attachment keep/drop/add rules (spec 4.9).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(AttachmentsCfg::default()).unwrap()))]
    pub attachments: AttachmentsCfg,
    /// Chapters handling: keep, drop, or an external locator (spec 4.9).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(ChaptersCfg::default()).unwrap()))]
    pub chapters: ChaptersCfg,
    /// Global and per-track tag handling (spec 4.9).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(TagsCfg::default()).unwrap()))]
    pub tags: TagsCfg,
    /// Output title handling: keep, clear, or a literal-mode template
    /// (spec 4.9).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(TitleCfg::default()).unwrap()))]
    pub title: TitleCfg,
}

/// Informational profile metadata; not used by matching or planning.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// Human-readable profile name (e.g. shown in the GUI's recent-profiles list).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Prose note for humans reading the profile file; carried through
    /// serialization but never interpreted or rendered by the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Primary-file selection and directory-walk options (spec 4.2).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct Input {
    /// Regex searched (not anchored) against each candidate file's
    /// basename; the first match's span and capture groups become the
    /// identifier (spec 3) and populate template fields (`{match}`,
    /// `{g1}`, named groups).
    pub pattern: String,
    /// Candidate file extensions, matched case-insensitively; not restricted
    /// to MKV. Checked once per batch against the local `mkvmerge
    /// --list-types` (spec 4.2): an entry absent from that list is still
    /// used for matching (so a typo silently excludes candidates) but
    /// raises `UnknownExtension`. Skipped, not raised, when the runtime
    /// capability is unavailable.
    pub extensions: Vec<String>,
    /// Whether the source directory walk descends into subdirectories.
    /// Defaults to `true`.
    #[serde(default = "default_true", skip_serializing_if = "is_default_true")]
    #[schemars(extend("default" = serde_json::to_value(default_true()).unwrap()))]
    pub recursive: bool,
}

fn default_true() -> bool {
    true
}

/// D48: `Input.recursive`'s serde default is `true`, not `bool::default()`
/// (`false`), so the generic `is_default` would omit an explicit
/// `recursive: false` and reload it inverted. Named after, and calling,
/// the same `default_true` its own `#[serde(default = ...)]` names.
fn is_default_true(b: &bool) -> bool {
    *b == default_true()
}

/// Output naming and collision handling (spec 4.8).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct OutputCfg {
    /// Output directory; profile default, usually overridden per run via
    /// CLI/GUI run inputs (spec 3, 4.8).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<PathBuf>,
    /// `keep` (source basename, `.mkv` extension enforced) or a literal-mode
    /// template (spec 4.7); defaults to `keep`.
    #[serde(
        default = "FilenameCfg::keep",
        skip_serializing_if = "is_keep_filename"
    )]
    #[schemars(extend("default" = serde_json::to_value(FilenameCfg::keep()).unwrap()))]
    pub filename: FilenameCfg,
    /// Policy when the rendered output path already exists or two planned
    /// outputs collide. An output path equal to any input path is always a
    /// hard `SourceOverwrite` error regardless of this policy (spec 4.8, 5.2).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(CollisionPolicy::default()).unwrap()))]
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
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct TemplateBlock {
    /// The template source text (spec 4.7); rendered in literal or regex
    /// mode depending on where the block appears.
    pub template: String,
}

/// Shared `{ external: ... }` block; standalone for the same
/// `deny_unknown_fields` reason as [`TemplateBlock`].
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct ExternalBlock {
    /// The external locator (spec 4.6) resolved to find the donor file(s).
    pub external: Locator,
}

/// `output.filename` value: either a literal-mode template or the `keep`
/// keyword (spec 4.8). `#[serde(untagged)]`: matches whichever variant
/// deserializes first, so a bare string becomes `Keyword` and a
/// `{ template: ... }` map becomes `Template`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(untagged)]
pub enum FilenameCfg {
    /// Literal-mode template (spec 4.7); `.mkv` appended if missing, path
    /// separators in the rendered name are errors (spec 4.8).
    Template(TemplateBlock),
    /// Keyword form; the only accepted value is `"keep"` (`validate.rs`
    /// rejects anything else as `InvalidKeyword`).
    #[schemars(schema_with = "filename_keyword_schema")]
    Keyword(String),
}

impl FilenameCfg {
    /// Closed keyword domain for `output.filename` (spec 4.8): the single
    /// source for the `validate.rs` guard, its `InvalidKeyword` `allowed`
    /// hint, and this enum's schema projection (D46).
    pub const KEYWORDS: &'static [&'static str] = &["keep"];

    /// The `keep` keyword variant: reuse the source basename (`.mkv`
    /// extension enforced). Also the serde default for `output.filename`.
    pub fn keep() -> Self {
        FilenameCfg::Keyword("keep".into())
    }
}

/// D48: `FilenameCfg` has no `Default` impl (only [`FilenameCfg::keep`]),
/// so the generic `is_default` cannot be instantiated for it (`E0277`).
/// Named after, and calling, the same `FilenameCfg::keep` its own
/// `#[serde(default = ...)]` names.
fn is_keep_filename(f: &FilenameCfg) -> bool {
    *f == FilenameCfg::keep()
}

/// Narrows `FilenameCfg::Keyword`'s schema to [`FilenameCfg::KEYWORDS`] (D46).
fn filename_keyword_schema(_generator: &mut SchemaGenerator) -> Schema {
    keyword_domain_schema(FilenameCfg::KEYWORDS)
}

/// `output.on_collision` policy (spec 4.8): applies to an existing rendered
/// output path and to two planned outputs rendering to the same path. Does
/// not override `SourceOverwrite` (output path equal to an input path),
/// which is always a hard error regardless of this policy (spec 5.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(rename_all = "lowercase")]
pub enum CollisionPolicy {
    /// Treat the collision as an error; the default policy.
    #[default]
    Error,
    /// Skip (do not produce) the colliding output.
    Skip,
    /// Replace the existing file. Only reaches files that are not inputs:
    /// the `SourceOverwrite` check runs first and is never overridden.
    Overwrite,
}

/// Binary keep-or-drop toggle shared by `attachments.unmatched`,
/// `tags.global` and `tags.track` (spec 4.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(rename_all = "lowercase")]
pub enum KeepDrop {
    /// Copy the corresponding structure from the sources into the output
    /// (mkvmerge's default behavior, left untouched).
    #[default]
    Keep,
    /// Exclude the corresponding structure from the output (mapped to the
    /// matching `--no-*` mkvmerge option at command generation).
    Drop,
}

/// One track rule (spec 4.5): selects exactly one track (subject to
/// strict independent uniqueness, spec 2) and optionally applies property
/// changes to it.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct TrackRule {
    /// Where the rule resolves against: the primary file (default) or an
    /// external donor located via a [`Locator`] (spec 4.5, 4.6).
    #[serde(default = "SourceCfg::primary", skip_serializing_if = "is_primary")]
    #[schemars(extend("default" = serde_json::to_value(SourceCfg::primary()).unwrap()))]
    pub source: SourceCfg,
    /// The match expression a track of `source` must satisfy; serialized
    /// under the profile key `match` (renamed here because `match` is a
    /// Rust keyword).
    #[serde(rename = "match")]
    pub match_expr: MatchExpr,
    /// Tolerates zero matching tracks (spec 5.1). Two candidates on an
    /// optional rule remain an `AmbiguousRule` error; `optional` covers
    /// only the zero-candidate case, never widening uniqueness.
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(bool::default()).unwrap()))]
    pub optional: bool,
    /// Settable-property changes to apply to the resolved track (spec 4.4
    /// table); `None` leaves the track's properties untouched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<BTreeMap<String, Scalar>>,
}

/// `track.source` value: the primary file, or an external donor (spec 4.5).
/// `#[serde(untagged)]`, same variant-resolution rule as [`FilenameCfg`].
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(untagged)]
pub enum SourceCfg {
    /// Resolve against an external donor file located by this locator
    /// (spec 4.6).
    External(ExternalBlock),
    /// Keyword form; the only accepted value is `"primary"` (the rule's own
    /// primary file).
    #[schemars(schema_with = "source_keyword_schema")]
    Keyword(String),
}

impl SourceCfg {
    /// Closed keyword domain for `track.source` (spec 4.5): the single
    /// source for the `validate.rs` guard, its `InvalidKeyword` `allowed`
    /// hint, and this enum's schema projection (D46).
    pub const KEYWORDS: &'static [&'static str] = &["primary"];

    /// The `primary` keyword variant: resolve against the rule's own
    /// primary file. Also the serde default for `track.source`.
    pub fn primary() -> Self {
        SourceCfg::Keyword("primary".into())
    }
}

/// D48: `SourceCfg` has no `Default` impl (only [`SourceCfg::primary`]),
/// so the generic `is_default` cannot be instantiated for it (`E0277`).
/// Named after, and calling, the same `SourceCfg::primary` its own
/// `#[serde(default = ...)]` names.
fn is_primary(s: &SourceCfg) -> bool {
    *s == SourceCfg::primary()
}

/// Narrows `SourceCfg::Keyword`'s schema to [`SourceCfg::KEYWORDS`] (D46).
fn source_keyword_schema(_generator: &mut SchemaGenerator) -> Schema {
    keyword_domain_schema(SourceCfg::KEYWORDS)
}

/// External file locator (spec 4.6): finds candidate donor files; the
/// owning rule's `match` then selects exactly one track inside the located
/// file. Uniqueness applies at both stages: two or more matching files is
/// `AmbiguousExternal`, two or more matching tracks inside the file is
/// `AmbiguousRule`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct Locator {
    /// Directory to search, relative to the primary file's directory, or
    /// absolute. A filesystem path; serialized as a plain string, so the
    /// profile format is unchanged. Use forward slashes in profiles for
    /// portability (Windows accepts them).
    pub path: PathBuf,
    /// Whether the search descends into subdirectories of `path`. Defaults
    /// to `false`, deliberately asymmetric with `input.recursive`
    /// (default `true`).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(bool::default()).unwrap()))]
    pub recursive: bool,
    /// Candidate extensions, validated against `mkvmerge --list-types`
    /// like `input.extensions`.
    pub extensions: Vec<String>,
    /// Sugar for `match_pattern: '{match}'`: the candidate's basename must
    /// match the primary's identifier. Mutually exclusive with
    /// `match_pattern` (`LocatorConflict` if both are set); the only valid
    /// value is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_to_source: Option<bool>,
    /// Regex-mode template (spec 4.7) the candidate's basename must match;
    /// mutually exclusive with `match_to_source`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<String>,
    /// Whether `match_pattern` matching is case-sensitive. Defaults to
    /// `false` (the rendered pattern is prefixed `(?i)`, spec 4.7).
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(bool::default()).unwrap()))]
    pub case_sensitive: bool,
}

/// Attachment handling (spec 4.9).
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct AttachmentsCfg {
    /// Policy for attachments no `rules` entry selects. Defaults to `keep`:
    /// deliberate asymmetry with tracks (unmatched tracks are always
    /// dropped), since dropping fonts silently breaks ASS subtitle
    /// rendering.
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(KeepDrop::default()).unwrap()))]
    pub unmatched: KeepDrop,
    /// Ordered select/drop/add rules (spec 4.9). Unlike track rules, not
    /// uniqueness-constrained: a `select`/`drop` expression may match
    /// several attachments (fonts come in sets); rules apply in list order,
    /// first matching rule wins per attachment.
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(<Vec<AttachmentRule>>::default()).unwrap()))]
    pub rules: Vec<AttachmentRule>,
}

/// Track handling: the unmatched-track policy plus the ordered rules
/// (spec 4.5). Parallel in shape to [`AttachmentsCfg`], but `unmatched`
/// defaults to `drop`: only rule-matched tracks survive unless the profile
/// opts into `keep` (spec 4.9 asymmetry note).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct TracksCfg {
    /// Policy for PRIMARY-file tracks no `rules` entry matches. Defaults to
    /// `drop` (the declarative default). `keep` passes them through
    /// untouched; consumed by `command` (Task 2). Donor tracks are
    /// unaffected: a donor contributes only its rule-selected track.
    #[serde(default = "drop_policy", skip_serializing_if = "is_drop_policy")]
    #[schemars(extend("default" = serde_json::to_value(drop_policy()).unwrap()))]
    pub unmatched: KeepDrop,
    /// Ordered track rules; list order defines the output `--track-order`
    /// (spec 4.5). Uniqueness-constrained (spec 2): each rule resolves to
    /// exactly one track.
    pub rules: Vec<TrackRule>,
}

fn drop_policy() -> KeepDrop {
    KeepDrop::Drop
}

/// D48: `tracks.unmatched`'s serde default is `drop` (`drop_policy`), the
/// opposite of `KeepDrop::default()` (`keep`, spec 4.9's deliberate
/// asymmetry with `attachments.unmatched`). The generic `is_default` would
/// compile against this field (both sides are `KeepDrop`) but compare
/// against the wrong default, silently omitting an explicit `unmatched:
/// keep` and reloading it as `drop` - the owner-ruled-legal `core-83`
/// pure-passthrough profile, destroyed. Named after, and calling, the same
/// `drop_policy` its own `#[serde(default = ...)]` names.
fn is_drop_policy(k: &KeepDrop) -> bool {
    *k == drop_policy()
}

/// Exactly one of `select` / `drop` / `add` must be set; enforced in
/// validate.rs (AttachmentRuleShape) so the parse error stays readable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct AttachmentRule {
    /// Select (keep) attachments matching this expression over
    /// `file_name`/`content_type`/`description` (spec 4.9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub select: Option<MatchExpr>,
    /// Drop attachments matching this expression; same property set as
    /// `select`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<MatchExpr>,
    /// Add an external file as an attachment, located the same way as a
    /// track's external source (spec 4.6, 4.9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Locator>,
}

/// `chapters` value: keep, drop, or an external locator (spec 4.9).
/// `#[serde(untagged)]`, same variant-resolution rule as [`FilenameCfg`]/[`SourceCfg`].
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(untagged)]
pub enum ChaptersCfg {
    /// Resolve exactly one chapters file (XML or simple format, as
    /// mkvmerge accepts) via this locator.
    External(ExternalBlock),
    /// Keyword form: `"keep"` or `"drop"`.
    #[schemars(schema_with = "chapters_keyword_schema")]
    Keyword(String),
}

impl ChaptersCfg {
    /// Closed keyword domain for `chapters` (spec 4.9): the single source
    /// for the `validate.rs` guard, its `InvalidKeyword` `allowed` hint, and
    /// this enum's schema projection (D46).
    pub const KEYWORDS: &'static [&'static str] = &["keep", "drop"];
}

impl Default for ChaptersCfg {
    fn default() -> Self {
        ChaptersCfg::Keyword("keep".into())
    }
}

/// Narrows `ChaptersCfg::Keyword`'s schema to [`ChaptersCfg::KEYWORDS`] (D46).
fn chapters_keyword_schema(_generator: &mut SchemaGenerator) -> Schema {
    keyword_domain_schema(ChaptersCfg::KEYWORDS)
}

/// Global and per-track tag handling (spec 4.9), mapped to
/// `--no-global-tags`/`--no-track-tags`.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(deny_unknown_fields)]
pub struct TagsCfg {
    /// Global (container-level) tags: keep or drop. Defaults to `keep`.
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(KeepDrop::default()).unwrap()))]
    pub global: KeepDrop,
    /// Per-track tags: keep or drop. Defaults to `keep`.
    #[serde(default, skip_serializing_if = "is_default")]
    #[schemars(extend("default" = serde_json::to_value(KeepDrop::default()).unwrap()))]
    pub track: KeepDrop,
}

/// `title` value: keep, clear, or a literal-mode template (spec 4.9).
/// `#[serde(untagged)]`, same variant-resolution rule as the other
/// keyword-or-block enums above.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(untagged)]
pub enum TitleCfg {
    /// Literal-mode template rendering the output title (spec 4.7).
    Template(TemplateBlock),
    /// Keyword form: `"keep"` or `"clear"`.
    #[schemars(schema_with = "title_keyword_schema")]
    Keyword(String),
}

impl TitleCfg {
    /// Closed keyword domain for `title` (spec 4.9): the single source for
    /// the `validate.rs` guard, its `InvalidKeyword` `allowed` hint, and
    /// this enum's schema projection (D46).
    pub const KEYWORDS: &'static [&'static str] = &["keep", "clear"];
}

impl Default for TitleCfg {
    fn default() -> Self {
        TitleCfg::Keyword("keep".into())
    }
}

/// Narrows `TitleCfg::Keyword`'s schema to [`TitleCfg::KEYWORDS`] (D46).
fn title_keyword_schema(_generator: &mut SchemaGenerator) -> Schema {
    keyword_domain_schema(TitleCfg::KEYWORDS)
}
