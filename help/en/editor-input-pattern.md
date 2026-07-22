# Pattern (input)

The input pattern is a regular expression that Muxsmith searches, unanchored, in the basename of every candidate file. The first match becomes the file's *identifier*: the string that names the episode or unit this file represents within the batch.

The identifier drives two things:

- **Donor pairing.** An external locator with `match_to_source` enabled requires the primary's identifier in the donor file's name, and a locator's `match_pattern` builds its search around the identifier's capture fields. This is how an external subtitle or audio file finds the primary it belongs to; see the Source topic.
- **Template fields.** Named capture groups (for example `(?<season>\d{2})`) and numbered groups become fields such as `{season}` or `{g1}`, available in the output filename template, the title template, and a locator's match pattern; `{match}` carries the whole identifier. See the Template topic.

Diagnostics to know:

- A file whose extension passes but whose basename does not match the pattern is skipped and reported as `IgnoredFile` (info), so a too-narrow pattern stays visible instead of silently shrinking the batch.
- If the pattern matches more than once in a basename, the first match is used and `MultipleIdentifierMatches` (info) is reported.
- Two primaries yielding the same identifier (a 720p and a 1080p copy of the same episode) are both muxed and both attract the same external files; `DuplicateIdentifier` (warning) points this out because output templates may then collide.

When you write one: capture exactly the part of the name that identifies the unit, for example `S(?<season>\d{2})E(?<episode>\d{2})` for a series. Name the groups you intend to use in templates; everything outside the match is ignored for identity.
