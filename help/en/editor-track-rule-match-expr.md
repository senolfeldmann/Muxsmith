# Match (track rule)

The match expression selects, from the rule's source, the one track this rule handles. It is a conjunction of up to five parts; every part that is present must hold:

- **`exact`**: property equality, typed - numbers compare numerically, languages as languages (`de` equals `ger`). Typed equality, the curated value domains and the `raw:` bypass have their own topic; see the Exact topic.
- **`substring`**: case-insensitive containment, string properties only - the usual way to catch `SDH` or `Commentary` in a track name.
- **`regex`**: regex search, taken as written; use `(?i)` for case-insensitive matching. String properties only.
- **`any`**: a list of sub-expressions of which at least one must hold (OR).
- **`not`**: a list of sub-expressions of which none may hold (exclusion).

Rules of the algebra:

- Several properties inside one part are AND: `exact` with `type: audio` and `language: en` requires both on the same track.
- `any` and `not` contain full expressions and nest to arbitrary depth; typical profiles stay flat.
- A present-but-empty `any` or `not` list is a config-time error (`EmptyMatchList`) - an empty OR or exclusion group is always an unfinished edit, never a meaningful "no constraint". Omit the key instead.

The uniqueness contract: each rule must match exactly one track of its source. Zero matches on a required rule is `MissingTrack` (its hint lists near-misses: tracks of the same type or language and which condition each failed); two or more is `AmbiguousRule`; one track claimed by two rules is `OverlappingRules`. The Optional toggle covers the legitimate zero-match case. For ambiguity, the suggestion engine proposes narrowing refinements you can apply with one click - see the suggestion card in the Batch view.
