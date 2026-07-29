/**
 * The editor's field registries (D45): one `Record<keyof T, FieldSpec>` per
 * edited struct, the forcing function that fails the *build* (not a test)
 * when a field added to the Rust model has no label-and-widget decision
 * recorded against it. All 13 structs reachable from `Profile` are
 * registered; there is no subset, because registering only some of them
 * would silently unnotice a field added to an unregistered struct -- the
 * exact failure this mechanism exists to close.
 *
 * 42 of the 43 fields are `EditableField`; the sole `FixedField` is
 * `Profile.profile_version` (spec 4 pins it at 1). Widget choices come
 * straight from the 43-row table in D45; do not re-derive them here.
 */
import type {
  AttachmentRule,
  AttachmentsCfg,
  CollisionPolicy,
  ExternalBlock,
  Input,
  KeepDrop,
  Locator,
  MatchExpr,
  Meta,
  OutputCfg,
  Profile,
  TagsCfg,
  TemplateBlock,
  TrackRule,
  TracksCfg,
} from "../bindings/profile";
import {
  CHAPTERS_KEYWORDS,
  FILENAME_KEYWORDS,
  SOURCE_KEYWORDS,
  TITLE_KEYWORDS,
} from "../bindings/keywords";
import type { FieldSpec } from "./fieldSpec";

// --- Option arrays, with a compile-time completeness guard -------------
//
// D45's own `never`-arm principle applied to a value list rather than a
// switch: `satisfies` checks the array against the Rust-derived union, and
// the `Exclude<...> extends never` check below catches a variant ADDED to
// the union that this array has not caught up with (`satisfies` alone only
// catches an array member NOT in the union, the opposite direction).
//
// The four keyword arrays (FILENAME_KEYWORDS, SOURCE_KEYWORDS,
// CHAPTERS_KEYWORDS, TITLE_KEYWORDS) get no such guard here: they are
// generated into src/bindings/keywords.ts from the Rust constants (D46),
// because the untagged enums they cover project to `Block | string` in TS
// -- the keyword domain is not in the type at all, so no `satisfies` guard
// could see it. Imported above, never hand-written.

export const COLLISION_POLICIES = [
  "error",
  "skip",
  "overwrite",
] as const satisfies readonly CollisionPolicy[];
type _CollisionPoliciesComplete =
  Exclude<CollisionPolicy, (typeof COLLISION_POLICIES)[number]> extends never ? true : never;
const _collisionPoliciesComplete: _CollisionPoliciesComplete = true;
void _collisionPoliciesComplete;

export const KEEP_DROP = ["keep", "drop"] as const satisfies readonly KeepDrop[];
type _KeepDropComplete = Exclude<KeepDrop, (typeof KEEP_DROP)[number]> extends never ? true : never;
const _keepDropComplete: _KeepDropComplete = true;
void _keepDropComplete;

// --- Registries, one per edited struct (13) -----------------------------

export const profileFields: Record<keyof Profile, FieldSpec> = {
  profile_version: {
    fixed: true,
    why: "Format version; spec 4 pins it at 1 and increments it only on a breaking format change. The editor never writes this field.",
  },
  meta: { labelKey: "editor-profile-meta", widget: { kind: "section", of: "meta", optional: true } },
  input: { labelKey: "editor-profile-input", widget: { kind: "section", of: "input", optional: false } },
  output: { labelKey: "editor-profile-output", widget: { kind: "section", of: "output", optional: false } },
  tracks: { labelKey: "editor-profile-tracks", widget: { kind: "section", of: "tracks", optional: false } },
  attachments: {
    labelKey: "editor-profile-attachments",
    widget: { kind: "section", of: "attachments", optional: false },
  },
  chapters: {
    labelKey: "editor-profile-chapters",
    helpId: "editor-profile-chapters",
    widget: { kind: "keywordOrBlock", keywords: CHAPTERS_KEYWORDS, block: "externalBlock" },
  },
  tags: { labelKey: "editor-profile-tags", widget: { kind: "section", of: "tags", optional: false } },
  title: {
    labelKey: "editor-profile-title",
    helpId: "editor-profile-title",
    widget: { kind: "keywordOrBlock", keywords: TITLE_KEYWORDS, block: "templateBlock" },
  },
};

export const metaFields: Record<keyof Meta, FieldSpec> = {
  name: { labelKey: "editor-meta-name", widget: { kind: "text", syntax: "plain", multiline: false } },
  description: {
    labelKey: "editor-meta-description",
    widget: { kind: "text", syntax: "plain", multiline: true },
  },
};

export const inputFields: Record<keyof Input, FieldSpec> = {
  pattern: {
    labelKey: "editor-input-pattern",
    helpId: "editor-input-pattern",
    widget: { kind: "text", syntax: "regex", multiline: false },
  },
  extensions: { labelKey: "editor-input-extensions", helpId: "editor-input-extensions", widget: { kind: "stringList" } },
  recursive: { labelKey: "editor-input-recursive", widget: { kind: "bool" } },
};

export const outputFields: Record<keyof OutputCfg, FieldSpec> = {
  directory: { labelKey: "editor-output-directory", widget: { kind: "directoryPath", optional: true } },
  filename: {
    labelKey: "editor-output-filename",
    helpId: "editor-output-filename",
    widget: { kind: "keywordOrBlock", keywords: FILENAME_KEYWORDS, block: "templateBlock" },
  },
  on_collision: {
    labelKey: "editor-output-on-collision",
    helpId: "editor-output-on-collision",
    widget: { kind: "select", options: COLLISION_POLICIES },
  },
};

export const templateBlockFields: Record<keyof TemplateBlock, FieldSpec> = {
  template: {
    labelKey: "editor-template-block-template",
    helpId: "editor-template-block-template",
    widget: { kind: "text", syntax: "templateLiteral", multiline: false },
  },
};

export const externalBlockFields: Record<keyof ExternalBlock, FieldSpec> = {
  external: {
    labelKey: "editor-external-block-external",
    widget: { kind: "section", of: "locator", optional: false },
  },
};

// `match_expr` below is the Rust field name (D45's own table names it that
// way, since the serialized/TS key `match` shadows the `match` keyword);
// the object key here is `match`, the actual `keyof TrackRule` member --
// ts-rs respects TrackRule's `#[serde(rename = "match")]` when projecting
// this binding (src/bindings/profile.ts), so `match_expr` is not a key of
// the generated type and would fail Record<keyof TrackRule, FieldSpec>.
export const trackRuleFields: Record<keyof TrackRule, FieldSpec> = {
  source: {
    labelKey: "editor-track-rule-source",
    helpId: "editor-track-rule-source",
    widget: { kind: "keywordOrBlock", keywords: SOURCE_KEYWORDS, block: "externalBlock" },
  },
  match: {
    labelKey: "editor-track-rule-match-expr",
    helpId: "editor-track-rule-match-expr",
    widget: { kind: "section", of: "matchExpr", optional: false },
  },
  optional: { labelKey: "editor-track-rule-optional", helpId: "editor-track-rule-optional", widget: { kind: "bool" } },
  changes: {
    labelKey: "editor-track-rule-changes",
    helpId: "editor-track-rule-changes",
    widget: { kind: "propertyMap", properties: "settable", values: "scalar" },
  },
};

export const locatorFields: Record<keyof Locator, FieldSpec> = {
  path: { labelKey: "editor-locator-path", widget: { kind: "directoryPath", optional: false } },
  recursive: { labelKey: "editor-locator-recursive", widget: { kind: "bool" } },
  extensions: { labelKey: "editor-locator-extensions", widget: { kind: "stringList" } },
  match_to_source: {
    labelKey: "editor-locator-match-to-source",
    helpId: "editor-locator-match-to-source",
    widget: { kind: "optionalFlag" },
  },
  match_pattern: {
    labelKey: "editor-locator-match-pattern",
    helpId: "editor-locator-match-pattern",
    widget: { kind: "text", syntax: "templateRegex", multiline: false },
  },
  case_sensitive: { labelKey: "editor-locator-case-sensitive", widget: { kind: "bool" } },
};

export const attachmentsFields: Record<keyof AttachmentsCfg, FieldSpec> = {
  unmatched: {
    labelKey: "editor-attachments-unmatched",
    helpId: "editor-attachments-unmatched",
    widget: { kind: "select", options: KEEP_DROP },
  },
  rules: {
    labelKey: "editor-attachments-rules",
    helpId: "editor-attachments-rules",
    widget: { kind: "list", item: "attachmentRule", reorderable: true },
  },
};

export const tracksFields: Record<keyof TracksCfg, FieldSpec> = {
  unmatched: {
    labelKey: "editor-tracks-unmatched",
    helpId: "editor-tracks-unmatched",
    widget: { kind: "select", options: KEEP_DROP },
  },
  rules: {
    labelKey: "editor-tracks-rules",
    helpId: "editor-tracks-rules",
    widget: { kind: "list", item: "trackRule", reorderable: true },
  },
};

export const attachmentRuleFields: Record<keyof AttachmentRule, FieldSpec> = {
  select: {
    labelKey: "editor-attachment-rule-select",
    widget: { kind: "section", of: "matchExpr", optional: true },
  },
  drop: {
    labelKey: "editor-attachment-rule-drop",
    widget: { kind: "section", of: "matchExpr", optional: true },
  },
  add: { labelKey: "editor-attachment-rule-add", widget: { kind: "section", of: "locator", optional: true } },
};

export const tagsFields: Record<keyof TagsCfg, FieldSpec> = {
  global: { labelKey: "editor-tags-global", widget: { kind: "select", options: KEEP_DROP } },
  track: { labelKey: "editor-tags-track", widget: { kind: "select", options: KEEP_DROP } },
};

export const matchExprFields: Record<keyof MatchExpr, FieldSpec> = {
  exact: {
    labelKey: "editor-match-expr-exact",
    helpId: "editor-match-expr-exact",
    widget: { kind: "propertyMap", properties: "matchable", values: "scalar" },
  },
  substring: {
    labelKey: "editor-match-expr-substring",
    widget: { kind: "propertyMap", properties: "matchable", values: "string" },
  },
  regex: {
    labelKey: "editor-match-expr-regex",
    widget: { kind: "propertyMap", properties: "matchable", values: "string" },
  },
  // `any` is a logical OR and `not` a logical NOR (spec 4.3) over a set of
  // sub-expressions; order carries no meaning for either, unlike
  // tracks.rules (output order) and attachments.rules (first-match-wins).
  any: { labelKey: "editor-match-expr-any", widget: { kind: "list", item: "matchExpr", reorderable: false } },
  not: { labelKey: "editor-match-expr-not", widget: { kind: "list", item: "matchExpr", reorderable: false } },
};
