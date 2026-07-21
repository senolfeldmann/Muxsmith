# Title

`Title` controls the container title of every output - the name players display for the file, distinct from the name of any single track. The control takes a keyword or a template.

## Keywords

- `keep` retains the source file's container title.
- `clear` empties it.

## Template

Switch the control to a template to compose a fresh title per file. Templates render in literal mode: fields interpolate as plain text, nothing is escaped or pattern-matched.

- Fields: `{match}` (the whole matched identifier), named capture groups from the input pattern such as `{season}`, numbered groups `{g1}`, and `{source_stem}` (the primary's filename without extension).
- Filters: `{season}` keeps the captured spelling (`03`), `{season:int}` strips leading zeros, `{season:pad2}` / `{season:pad3}` zero-pad.

Example: `Show S{season}E{episode}` renders `Show S03E01` for a primary matched as `S03E01`.

## Interactions

- Template fields come from the capture groups of `Input.pattern` - name groups there to use them here. See the Pattern topic.
- Source titles frequently carry release-group or encoder noise; `clear` gives every output a clean slate, `keep` is right only when the sources are curated, and a template standardizes titles across the whole batch.
