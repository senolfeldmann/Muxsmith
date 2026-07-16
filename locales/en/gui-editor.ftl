# T9-T13: the profile editor (spec 8.2 view 4, D45). One labelKey per
# EditableField across the 13 field registries (src/editor/registries.ts),
# grouped below by the struct each registry covers, plus the save-surface
# note (D41). No tooltip keys: the editor ships without tooltips (Plan 7).
# Widget facets (select/keywordOrBlock options) render from the domain
# arrays (profile-format tokens), not from this catalog.

## Profile (top-level sections)

editor-profile-meta = Metadata
editor-profile-input = Input
editor-profile-output = Output
editor-profile-tracks = Tracks
editor-profile-attachments = Attachments
editor-profile-chapters = Chapters
editor-profile-tags = Tags
editor-profile-title = Title

## Meta

editor-meta-name = Name
editor-meta-description = Description

## Input

editor-input-pattern = Pattern
editor-input-extensions = Extensions
editor-input-recursive = Recursive

## Output (OutputCfg)

editor-output-directory = Directory
editor-output-filename = Filename
editor-output-on-collision = On collision

## TemplateBlock

editor-template-block-template = Template

## ExternalBlock

editor-external-block-external = External locator

## TrackRule

editor-track-rule-source = Source
editor-track-rule-match-expr = Match
editor-track-rule-optional = Optional
editor-track-rule-changes = Changes

## Locator

editor-locator-path = Path
editor-locator-recursive = Recursive
editor-locator-extensions = Extensions
editor-locator-match-to-source = Match to source
editor-locator-match-pattern = Match pattern
editor-locator-case-sensitive = Case-sensitive

## Attachments (AttachmentsCfg)

editor-attachments-unmatched = Unmatched
editor-attachments-rules = Rules

## Tracks (TracksCfg)

editor-tracks-unmatched = Unmatched
editor-tracks-rules = Rules

## AttachmentRule

editor-attachment-rule-select = Select
editor-attachment-rule-drop = Drop
editor-attachment-rule-add = Add

## Tags (TagsCfg)

editor-tags-global = Global
editor-tags-track = Track

## MatchExpr

editor-match-expr-exact = Exact
editor-match-expr-substring = Substring
editor-match-expr-regex = Regex
editor-match-expr-any = Any
editor-match-expr-not = Not

## Save surface (D41)

editor-save-note = Saving rewrites the file from the model: comments, key order and formatting are not preserved, and fields left at their default are not written back.

## Generic list/map actions

editor-action-add = Add
editor-action-remove = Remove
