# Match to source

`Match to source` pairs external donor files with the primary file they belong to. The control is a flag that is either set or absent from the profile - there is no explicit `false`; clearing it removes it.

## What it does

When set, a donor file qualifies only if its filename contains the primary's matched identifier - the text the input pattern captured, such as `S03E01`. It is exactly shorthand for a `Match pattern` of `{match}`.

Example: the primary `Show S03E01.mkv` was matched as `S03E01`; in a directory of subtitle files, only names containing `S03E01` (say `Show.S03E01.tr.srt`) pair with it. The comparison ignores case unless `Case sensitive` is set.

## Interactions

- Mutually exclusive with `Match pattern`: set one or the other. When donor names spell the identifier differently than the primaries do (say `staffel3episode1` against `S03E01`), clear this flag and write a pattern instead - see the Match pattern topic.
- File pairing is only the first stage: the rule's match expression then selects exactly one track inside the paired donor.
- Uniqueness holds at the file stage: two donor files pairing with one primary is an ambiguity error; zero is a missing-external error, unless the rule is `Optional`.

## When to use it

Whenever donors follow the same naming scheme as the primaries - the common case for sidecar subtitles or a second release of the same series. Without it (and without a match pattern) every file the locator finds is a candidate, which only works when the locator finds exactly one file.
