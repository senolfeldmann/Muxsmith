# Changes (track rule)

Changes lists the property edits applied to the one track this rule matched. Properties not listed pass through unchanged; a rule without changes copies its track as-is.

The settable properties are a curated, closed set, each mapped to an mkvmerge option:

- `language`, `track_name`, `sub_charset` - string-valued. `language` accepts ISO 639-2 (`ger`) and BCP-47 (`de`) spellings; `sub_charset` is passed through to mkvmerge leniently.
- `default_track`, `forced_track`, `enabled_track` - the boolean track flags.
- `flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`, `flag_original` - the boolean accessibility and provenance flags.

Values are typed per property: the flags take `true` or `false`, not strings. An unknown key here is a config-time error (`UnknownSettableProperty`). The `raw:` prefix that the match side accepts is a matching opt-in only; it is not accepted in changes.

Note the asymmetry with matching: the matchable set is much larger. You can *match* on `codec_id`, `audio_channels` or `pixel_dimensions`, but only the properties listed above can be *set* - everything else is decided by the source track.

Relation to suggestions and one-click apply: when a rule is ambiguous, the suggestion engine proposes refinements to the rule's *Match*, never to its changes (the narrow-only guarantee). Applying a suggestion narrows which track the rule selects; the changes ride along unchanged and then apply to the single remaining track.

Typical use: naming subtitle variants (`track_name` set to `English SDH` with `flag_hearing_impaired` set to `true`), promoting one audio track with `default_track`, or tagging a donor track with its real `language` when the source file left it unset.
