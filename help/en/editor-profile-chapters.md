# Chapters

`Chapters` controls the chapter track of every output: a keyword, or an external donor file.

## Keywords

- `keep` passes the source file's chapters through unchanged.
- `drop` removes them from the output.

## External donor chapters

Instead of a keyword, switch the control to an external locator: chapters then come from a separate file on disk, one per primary. The locator works exactly like an external track source - a path (relative to the primary's directory, or absolute), an extension list, and pairing via `Match to source` or a `Match pattern` (see those topics).

- The locator must resolve to exactly one chapters file per primary: zero is a missing-external error, two or more an ambiguity error.
- The file's content is anything mkvmerge accepts as chapters: Matroska chapter XML or the simple chapter format.

## When to use which

`keep` when the sources carry correct chapters; `drop` when their chapters are wrong or unwanted - the profile does not edit chapter content, it only routes it. The donor route serves the case where chapters live outside the video (downloaded, generated, or authored per episode) and should be muxed in during the same pass.
