# Template

The template text renders a string from the fields the input pattern captured. One engine serves all template surfaces: here in *literal mode* for the output filename and the title, and in *regex mode* for a locator's match pattern (that mode has its own topic; see the Match pattern topic).

Fields:

- `{match}`: the whole identifier, exactly as the input pattern matched it.
- `{season}`: a named capture group, raw as captured (a group that matched `03` renders `03`).
- `{g1}`, `{g2}`: numbered capture groups, for patterns without named groups.
- `{source_stem}`: the primary file's basename without its extension. Literal mode only.

Filters, written after a colon inside the braces:

- `{season:int}` strips leading zeros: `03` renders as `3`.
- `{season:pad2}` and `{season:pad3}` zero-pad to two or three digits: `3` renders as `03` or `003`.

The filters make differing source conventions meet in one canonical output: whether an episode was captured as `1` or `01`, `{episode:pad2}` always renders `01`.

In literal mode the field values interpolate as plain text and everything around them is kept verbatim. Example: `Show - S{season:pad2}E{episode:pad2}.mkv` renders `Show - S03E01.mkv` for a file identified as `S03E01`.

Interactions:

- The available fields are exactly the capture groups of the input pattern. Renaming a group there changes the field names every template must use; an unknown field in a template is a validation error, caught before any file is touched.
- For output filenames, the rendered result must not contain a path separator and must not be empty; see the Filename topic for those checks and the collision consequences.
