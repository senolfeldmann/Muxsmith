# Exact (match)

The `exact` part of a match expression compares track properties for equality. It is a map of property names to values; a track qualifies when every entry holds. All entries are AND-combined, together with any other parts of the expression - the Match topic describes the full algebra.

## Typed equality

`exact` compares each value in the property's own domain, not as raw text:

- Numbers compare numerically: `6` equals `6.0`.
- Languages compare as languages: ISO 639 spellings and BCP-47 tags reduce to one canonical form, so `de` equals `ger` and `pt-Latn-BR` equals `pt-BR`, while real distinctions stay distinct (`pt-BR` is not `pt-PT`).
- Strings compare case-sensitively. For case-insensitive containment use `substring`; for byte-literal pattern matching use `regex`.

## Curated domains

Properties with a closed value set are validated instead of silently never matching: a `type` or `codec_kind` value outside its domain is a config-time error, an unknown `language` a plan-time error checked against your mkvmerge installation. `codec_kind` is a friendly alias (`srt`, `ass`, `pgs`, ...) mapped to sets of `codec_id` values, and is usable only under `exact` - pattern-match `codec_id` instead when you need `substring` or `regex`.

## Booleans: absent means false

mkvmerge reports flags such as `flag_hearing_impaired` only when they are set. `exact` mirrors that: a boolean property missing from a track's identification compares equal to `false`, so `exact: { flag_hearing_impaired: false }` matches tracks that never set the flag.

## The `raw:` bypass

Prefixing a property name with `raw:` (for example `raw:dolby_complexity_index`) matches a property this build's schema does not know yet: no existence, type, or domain check, no language normalization, no absent-means-false shortcut, and no type conversion - a value matches only a reported value of the same kind, so `6` does not match a reported `6.0` here even though it does under typed equality above. Diagnostics flag every bypass. Use it only for properties a newer mkvmerge reports; a typo in a `raw:` name silently never matches.
