# Optional (track rule)

Optional covers exactly one case: the rule may match *zero* tracks without failing the file. Nothing else changes - two or more matching tracks remain an `AmbiguousRule` error even on an optional rule. Optional means "may be absent", never "loosely matched".

- **Off** (the default): the rule is required. Zero matches is a `MissingTrack` error, and its hint lists near-misses - tracks of the same type or language, with the condition each one failed - so you see whether the rule is wrong or the track is genuinely missing.
- **On**: zero matches simply omits this track from that file's output, with no diagnostic. With an external source, a locator that finds no donor file is tolerated the same way (a required rule would raise `MissingExternal`).

When to use: tracks that legitimately exist only in some files of the batch - forced subtitles that only some episodes carry, a commentary track present on selected releases. Keep a rule required wherever absence would be a defect you want surfaced: a missing main audio track should fail that file loudly, not produce a silently thinner output.

Interactions:

- Optional does not relax uniqueness, so it is never a fix for ambiguity. When a rule matches two tracks, the suggestion engine proposes narrowing refinements instead; and when no single refinement resolves the conflict for the whole batch, the report partitions the affected files into groups by the per-file fix each group needs (`SuggestionPartition`). Toggling Optional changes none of that.
- An optional rule's Changes apply only when a track actually matched; on zero matches nothing is applied and the output track order simply closes the gap.
