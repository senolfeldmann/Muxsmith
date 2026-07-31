# Editor view

The Editor edits a profile as a structured model, not as YAML text. Start a new profile, open a profile file, or reopen one from the recent profiles; every field renders as a form control and the track rules as a grid.

## Creating a profile

New starts a profile with one candidate extension and one empty rule; validation flags the empty rule as a warning rather than an error, so the profile is incomplete, not wrong.

The editor holds one profile at a time. Replacing it, whether by creating another or by opening one, warns first while the current profile has unsaved changes; declining leaves it untouched. Switching to another view and back never touches it. Quitting the app with unsaved changes warns as well.

Every edit can be undone. Undo and Redo sit in the action row, and the same actions also respond to keyboard shortcuts: Ctrl+Z (Cmd+Z on macOS) for Undo, and Ctrl+Shift+Z or Ctrl+Y (Cmd+Shift+Z or Cmd+Y on macOS) for Redo.

## Editing the model

What you edit is the in-memory model parsed from the file, never the file's text. The rule grid summarizes each track rule (source, match, optional, changes); rule order is output track order, and dragging a row reorders the rules. Selecting a row opens a detail panel below the grid with the full fields of that rule. The remaining sections (metadata, input, output, attachments, chapters, tags, title) render as regular form sections.

## Validate on edit

Every edit re-validates the whole profile in the background, so the diagnostics panel always describes the current state of the model. Saving is disabled while any error-severity diagnostic exists; that is the one gate the editor enforces. Warnings and info notices never block saving.

## Save semantics

Nothing is written to disk until you save. On a profile with no path yet, the first save opens a dialog asking where to put the file; every later save on that profile writes there directly, with no dialog.

Saving rewrites the file from the model: comments, key order and formatting of the file on disk are not preserved, and fields left at their default are omitted rather than written back. The output format follows the file's extension, so a YAML profile stays YAML. If a hand-commented profile matters to you, keep a copy or put it under version control before saving over it.

The Apply button on a suggestion card in the Batch view saves the same way: one canonical rewrite, same rules (see the Suggestion card topic).
