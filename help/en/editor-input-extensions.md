# Extensions (input)

The extension list gates what enters the batch: only files whose extension appears here are considered as primary candidates at all. Matching is case-insensitive, so `mkv` also covers `MKV`.

- The list is not restricted to MKV. Any container mkvmerge can read is a legal source (`mp4`, `avi`, and so on); the output container is always Matroska.
- Every entry is validated at runtime against the local mkvmerge's `--list-types` output. An entry mkvmerge does not know is still used for matching but reported as `UnknownExtension` (warning), because a typo here would otherwise silently exclude candidates from the batch.
- The Recursive checkbox next to this field controls whether subdirectories of the source directory are searched too (on by default).

Interactions with other settings:

- This list gates *primary* files only. Each external locator carries its own `extensions` list and its own `recursive` flag for the donor search; see the Source topic. External `.srt` subtitles, for example, belong in the locator's list, not here - listing `srt` here would make every subtitle file a primary with its own output.
- A file that passes the extension gate but does not match the input pattern is reported as `IgnoredFile` (info), so the two filters stay distinguishable: the extension list decides what is considered, the pattern decides what is identified.

When to edit: list exactly the source containers you want muxed, typically `mkv` alone or `mkv` plus `mp4`. A broader list widens the batch; a narrower one is the cheapest way to keep unrelated files in the same directory out of a run.
