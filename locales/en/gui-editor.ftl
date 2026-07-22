# T9-T13: the profile editor (spec 8.2 view 4, D45). One labelKey per
# EditableField across the 13 field registries (src/editor/registries.ts),
# grouped below by the struct each registry covers, plus the save-surface
# note (D41). Every label message carries its tooltip as a .tooltip
# attribute (Plan 7, D53/D55); there are no suffixed tooltip ids.
# Widget facets (select/keywordOrBlock options) render from the domain
# arrays (profile-format tokens), not from this catalog.

## Profile (top-level sections)

editor-profile-meta = Metadata
    .tooltip = Optional descriptive fields for this profile: a display name and free-text notes. They never affect muxing.
editor-profile-input = Input
    .tooltip = How source files are found: the identifier pattern, the accepted file extensions, and whether subdirectories are scanned.
editor-profile-output = Output
    .tooltip = Where muxed files are written, how their filenames are formed, and what happens when an output file already exists.
editor-profile-tracks = Tracks
    .tooltip = Which tracks each output contains and in what order: the policy for unmatched tracks plus the ordered track rules.
editor-profile-attachments = Attachments
    .tooltip = What happens to attachments such as fonts: the policy for unmatched ones plus select, drop and add rules.
editor-profile-chapters = Chapters
    .tooltip = What happens to the source chapters: keep copies them over, drop removes them, and an external locator loads chapters from a separate file.
editor-profile-tags = Tags
    .tooltip = Whether the global and per-track tags of the source files are kept or dropped.
editor-profile-title = Title
    .tooltip = Container title of each output: keep retains the source title, clear removes it, a template renders a new one.

## Meta

editor-meta-name = Name
    .tooltip = Human-readable name of this profile. Free text; it never affects muxing.
editor-meta-description = Description
    .tooltip = Free-text notes on what this profile is for and how it is meant to be used. Never affects muxing.

## Input

editor-input-pattern = Pattern
    .tooltip = Regular expression searched in each source file's basename; the first match becomes the file's identifier, and capture groups become template fields for filenames and donor matching.
editor-input-extensions = Extensions
    .tooltip = File extensions that enter the batch, matched case-insensitively and validated against what the local mkvmerge can read.
editor-input-recursive = Recursive
    .tooltip = Also scan the subdirectories of the source directory, not only its top level.

## Output (OutputCfg)

editor-output-directory = Directory
    .tooltip = Default directory the muxed files are written to. The output directory picked in the Batch view overrides it per run.
editor-output-filename = Filename
    .tooltip = How each output file is named: keep reuses the source basename with an .mkv extension, a template renders a new name from the captured fields.
editor-output-on-collision = On collision
    .tooltip = What happens when the rendered output path already exists on disk: error refuses the file, skip omits it with a warning, overwrite replaces it.

## TemplateBlock

editor-template-block-template = Template
    .tooltip = Template text rendered per source file. The whole identifier, named and numbered capture groups and the source basename are available as fields; filters can strip or pad leading zeros.

## ExternalBlock

editor-external-block-external = External locator
    .tooltip = Locator for donor files: where to search, which file extensions count, and how each donor file is paired with its source file.

## TrackRule

editor-track-rule-source = Source
    .tooltip = Where the track is taken from: primary reads the batch source file itself, an external locator pulls it from a donor file.
editor-track-rule-match-expr = Match
    .tooltip = Conditions the track must meet; all listed parts must hold, and each rule must match exactly one track in its source.
editor-track-rule-optional = Optional
    .tooltip = Lets the rule match no track at all: it is then skipped instead of failing the file. Two matching candidates remain an error.
editor-track-rule-changes = Changes
    .tooltip = Property changes applied to the matched track, for example language, track_name or default_track.

## Locator

editor-locator-path = Path
    .tooltip = Directory searched for donor files: relative to the primary file's directory, or absolute.
editor-locator-recursive = Recursive
    .tooltip = Also search the subdirectories of the locator path, not only the directory itself.
editor-locator-extensions = Extensions
    .tooltip = File extensions considered when searching for donor files, validated against what the local mkvmerge can read.
editor-locator-match-to-source = Match to source
    .tooltip = Pairs donor files with source files by requiring the donor's basename to contain the identifier captured from the source. Shorthand for a match pattern of exactly that identifier.
editor-locator-match-pattern = Match pattern
    .tooltip = Template matched against donor basenames as a regular expression; interpolated field values match literally. Mutually exclusive with match_to_source.
editor-locator-case-sensitive = Case-sensitive
    .tooltip = Match donor filenames case-sensitively; by default the pattern ignores letter case.

## Attachments (AttachmentsCfg)

editor-attachments-unmatched = Unmatched
    .tooltip = What happens to attachments no rule matches: keep copies them into the output, drop discards them. Dropped fonts silently break ASS subtitle rendering.
editor-attachments-rules = Rules
    .tooltip = Ordered select, drop and add rules for attachments. Rules resolve in list order, the first match wins, and one rule may match several attachments.

## Tracks (TracksCfg)

editor-tracks-unmatched = Unmatched
    .tooltip = What happens to primary-file tracks no rule matches: keep copies them into the output, drop discards them.
editor-tracks-rules = Rules
    .tooltip = Ordered track rules; each must match exactly one track, and the list order defines the output track order.

## AttachmentRule

editor-attachment-rule-select = Select
    .tooltip = Match expression for attachments to keep; every matching attachment is copied into the output.
editor-attachment-rule-drop = Drop
    .tooltip = Match expression for attachments to discard; every matching attachment is left out of the output.
editor-attachment-rule-add = Add
    .tooltip = Locator for external files to attach to each output, searched the same way as donor files.

## Tags (TagsCfg)

editor-tags-global = Global
    .tooltip = Whether the container-wide tags of the source files are kept or dropped.
editor-tags-track = Track
    .tooltip = Whether the per-track tags of the source files are kept or dropped.

## MatchExpr

editor-match-expr-exact = Exact
    .tooltip = Properties compared by typed equality: numbers compare numerically and languages across equivalent spellings. Several entries must all hold.
editor-match-expr-substring = Substring
    .tooltip = Case-insensitive containment test on string properties. Several entries must all hold.
editor-match-expr-regex = Regex
    .tooltip = Regular expression searched in string properties, case-sensitive as written. Several entries must all hold.
editor-match-expr-any = Any
    .tooltip = Sub-expressions of which at least one must hold: the OR part of a match.
editor-match-expr-not = Not
    .tooltip = Sub-expressions of which none may hold: excludes tracks that would otherwise match.

## Save surface (D41)

editor-save-note = Saving rewrites the file from the model: comments, key order and formatting are not preserved, and fields left at their default are not written back.

## Generic list/map actions

editor-action-add = Add
editor-action-remove = Remove

## Rule grid ordinal (D59)

# Presentation-only 1-based index column; not a registry label, so no
# .tooltip. The digit itself is locale-neutral data rendered in the cell.
editor-track-rule-order = Order
