# Source (track rule)

Where this rule takes its track from. The default, `primary`, matches against the tracks of the primary file itself. The alternative is an *external locator*: the rule pulls its track from a donor file found near the primary - the mechanism behind "take the Turkish subtitles from the `.srt` files next to the episodes" or "take the German audio from a second release".

The locator's parts:

- `path`: where to search, relative to the primary file's directory or absolute. Its own `recursive` flag (off by default) and its own `extensions` list decide which files are candidates - independent of the input extensions, which gate primaries only.
- `match_to_source` or `match_pattern`: how a candidate is paired with its primary. `match_to_source` requires the primary's identifier in the donor's name; a match pattern is a template in regex mode for names that encode the identifier differently. The two are mutually exclusive; each has its own topic.

Selection is two-staged: the locator selects candidate *files*, then the rule's Match expression selects exactly one *track* inside the located file. Donor files are full containers - an external MKV with a match on audio and language `de` is the supported way to pull matching German audio out of a parallel release.

Uniqueness holds at both stages: two matching donor files raise `AmbiguousExternal`, two matching tracks inside the donor raise `AmbiguousRule`. A donor that finds no file fails the rule with `MissingExternal` unless the rule is Optional. A donor that is itself a primary of the batch is flagged `DonorIsPrimary` (warning): it will be muxed as its own output *and* donate tracks.

When to use: whenever a track you want in the output ships outside the primary container. For tracks already inside the primary, leave the source at `primary`.
