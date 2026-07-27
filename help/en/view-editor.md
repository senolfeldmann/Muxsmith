# Editor view

The Editor edits a profile as a structured model, not as YAML text. Open a profile file, or reopen one from the recent profiles; every field renders as a form control and the track rules as a grid.

## Editing the model

What you edit is the in-memory model parsed from the file, never the file's text. The rule grid summarizes each track rule (source, match, optional, changes); rule order is output track order, and dragging a row reorders the rules. Selecting a row opens a detail panel below the grid with the full fields of that rule. The remaining sections (metadata, input, output, attachments, chapters, tags, title) render as regular form sections.

## Validate on edit

Every edit re-validates the whole profile in the background, so the diagnostics panel always describes the current state of the model. Saving is disabled while any error-severity diagnostic exists; that is the one gate the editor enforces. Warnings and info notices never block saving.

## Save semantics

Saving rewrites the file from the model: comments, key order and formatting of the file on disk are not preserved, and fields left at their default are omitted rather than written back. The output format follows the file's extension, so a YAML profile stays YAML. If a hand-commented profile matters to you, keep a copy or put it under version control before saving over it.

The Apply button on a suggestion card in the Batch view saves the same way: one canonical rewrite, same rules (see the Suggestion card topic).
