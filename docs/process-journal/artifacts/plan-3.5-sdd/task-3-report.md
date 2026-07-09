# Task 3 report: gated live test for `keep` track-order (D20)

(Plan 3.5, mkvtoolnix parity. This overwrites a stale `task-3-report.md`
left over from Plan 3's Task 3, "attachment matching" -- that content
belongs to Plan 3's own artifact trail, not here; task numbers are reused
per-plan.)

## What was built

Added `live_keep_unmatched_orders_only_listed_track` to
`crates/muxsmith-core/tests/command_integration.rs`, mirroring the file's
existing gated-live-test pattern exactly:

- Same `mkvmerge()` locate-or-self-skip guard as `live_mkvmerge_accepts_planned_command`
  (`eprintln!` + early `return` when `Mkvmerge::locate()` fails).
- Same fixture-generation approach (spawn real mkvmerge to build the source),
  extended to 3 single-track SRT inputs merged as separate (non-grouped)
  files, each tagged with a distinguishing `--track-name` (ALPHA/BRAVO/CHARLIE)
  so the output can be matched back to its source track after re-identification.
- Same re-identify-via-`-J` verification step (`Mkvmerge::identify_json` +
  `Identification::from_json`) on both the fixture source (to confirm ids 0/1/2
  == ALPHA/BRAVO/CHARLIE in source order, not assumed) and the muxed output.

Unlike the sibling test, the `Plan` here is constructed directly (not via
`plan_batch`/a profile), matching the pattern already used by the pure golden
tests in `crates/muxsmith-core/tests/command.rs` (e.g.
`keep_unmatched_suppresses_primary_selection_flags`): `keep_unmatched: true`,
a single `Assignment` whose `track_id` is the *second* source track (BRAVO,
id 1), `track_kind: "subtitles"`, no changes. This is the minimal `Plan` that
exercises exactly the D20 mechanic: `command()` emits no primary
track-selection flags (so mkvmerge keeps all 3 tracks by default) and a
`--track-order` containing only `"0:1"` (the one assignment), never mentioning
tracks 0 or 2.

The test asserts:
1. The 3-track fixture is built as expected (id 0/1/2 = ALPHA/BRAVO/CHARLIE),
   verified via `-J`, not assumed.
2. The rendered argv passed to real mkvmerge exits 0 and produces the output
   file.
3. Re-identifying the output: all 3 source tracks are present (kept), despite
   only one being in `--track-order`.
4. The D20 assumption itself: output track order (ascending output id) is
   `BRAVO, ALPHA, CHARLIE` -- the explicitly-ordered track first, then the two
   unlisted tracks in their original source-relative order.

## Empirical verification (before writing the Rust test)

Manually reproduced the exact scenario with the installed mkvmerge v100 in a
scratch directory, to confirm the assumption against the binary per SI-3, not
from memory:

```
$ mkvmerge -q -o source.mkv --track-name 0:ALPHA alpha.srt \
    --track-name 0:BRAVO bravo.srt --track-name 0:CHARLIE charlie.srt
$ mkvmerge -J source.mkv | ...
[(0, 'ALPHA'), (1, 'BRAVO'), (2, 'CHARLIE')]

$ mkvmerge -q --output out.mkv "(" source.mkv ")" --track-order 0:1
exit=0
$ mkvmerge -J out.mkv | ...
[(0, 'BRAVO'), (1, 'ALPHA'), (2, 'CHARLIE')]
```

**Observed: the D20 assumption holds.** With only track 1 (BRAVO) named in
`--track-order`, mkvmerge keeps all 3 tracks (no selection flags given) and
places BRAVO first (the explicit order entry), then ALPHA and CHARLIE
afterward in their original source-relative order (0 before 2). No
interleaving, no reordering, no error. The Rust test encodes exactly this
observation and passed on the first run using the real `command()` output run
through real mkvmerge (not a hand-typed argv).

## Gate results

- `cargo test -p muxsmith-core --test command_integration keep -- --nocapture`: PASS (the new test ran live, not skipped -- mkvmerge v100.0 is installed).
- `cargo fmt --all --check`: clean (after `cargo fmt --all` reformatted one multi-line `assert_eq!`).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo test --workspace`: all green (every test binary, 0 failed).
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/tests/command_integration.rs`: added the module-doc
  bullet for the new test, imports (`PropValue`, `Track` from `identify`;
  `Assignment`, `AttachmentPlan`, `ChapterSource`, `Plan`, `PrimaryAttachments`,
  `TagFlags`, `TitleAction` from `planner`), a small `track_name` helper, and
  the `live_keep_unmatched_orders_only_listed_track` test itself.

## Commit

`342ea42` -- `test(command): gated live mkvmerge guard for keep track-order (D20)`

Only this one file was staged and committed. Two unrelated untracked files
present in the working tree (`HANDOFF.md`, `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`,
apparently controller-level artifacts from Plan 4 groundwork) were left
untouched -- out of this task's scope, and `git add -A` would have swept them
in unintentionally.

## Concerns

None. The empirical result matches the assumption exactly; no planner change
is needed as a consequence of this task. The test is a genuine standing guard:
if a future mkvmerge version changes this behavior, or if `command.rs`'s
`--track-order`/selection-flag emission changes, this test will fail loudly
rather than silently drift from the documented D20 assumption.
