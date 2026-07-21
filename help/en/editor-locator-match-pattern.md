# Match pattern

`Match pattern` pairs external donor files with a primary by template - for when the donors spell the identifier differently than the primaries do. It is the profile's third text syntax: not a plain regex like the input pattern, not a literal template like the output filename, but a template rendered in regex mode. The two are easy to conflate; the render rule below is the difference.

## How it renders

Template fields interpolate the values captured from the primary's filename - and every value is inserted regex-escaped, as a literal. Only the text you write around the fields is a regular expression. The rendered whole is searched against each candidate donor filename; the search ignores case unless `Case sensitive` is set.

- Fields: `{match}` (the whole identifier), named groups such as `{season}`, numbered groups `{g1}`, `{g2}`.
- Filters: `{season}` keeps the captured spelling (`03`), `{season:int}` strips leading zeros (`3`), `{season:pad2}` / `{season:pad3}` zero-pad.
- `{source_stem}` is not available here; it exists in literal mode only.

## Example

For a primary matched as `S03E01` with groups `season` and `episode`, the pattern `staffel0*{season:int}episode0*{episode:int}` matches `staffel03episode01`, `staffel3episode01`, and `Staffel3Episode1`.

## Not a plain regex

The braces are template fields, not regex syntax, and interpolated values can never act as regex fragments - a captured `S03E01` is escaped, never interpreted. Write regex constructs (`0*`, alternation, classes) only in the surrounding text.

## Interactions

- Mutually exclusive with `Match to source`, which is exactly the shorthand for a pattern of `{match}` - prefer the flag when donors and primaries share one naming scheme.
- After a file is paired, the rule's match expression selects exactly one track inside it. Two pairing files is an ambiguity error; zero is a missing-external error, unless the rule is `Optional`.
