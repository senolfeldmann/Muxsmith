<!-- STALE-REPORT NOTE (2026-07-11, docs-tree audit S18): this file is a MISDIRECTED
     foreign report - it describes plan-4 task 7 (richer gated live test), NOT plan-5
     task 7 (Shell IPC read-only commands + settings, see task-7-brief.md). The real
     plan-5 T7 outcome survives only in progress.md and the merge report. Same
     stale-same-named-report trap as T1/T3/T5/T6, here unmarked until this note. -->
# Task 7 report: richer gated live test (attachment + changes round trip)

## What was implemented

Appended one gated live test, `live_attachment_and_changes_round_trip`, to
`crates/muxsmith-core/tests/command_integration.rs` (end of file, after
`live_keep_donor_trails_primary`). It follows the file's established
locate-or-skip idiom (`mkvmerge()` helper at :213-215) and the SRT-fixture
pattern (:232-245).

Flow:

1. Builds a primary MKV directly via real mkvmerge: one SRT-derived subtitle
   track plus one `.txt` file attached with an explicit
   `--attachment-mime-type text/plain --attach-file`.
2. Drives `plan_batch` with a small profile (`ATTACHMENT_PROFILE`) whose one
   rule selects the subtitle track (`exact: { type: subtitles }`) with
   `changes: { track_name: Renamed, default_track: true }`. No
   `attachments:` section is configured, so the profile's default
   (`attachments.unmatched: keep`, no rules) resolves the primary's one
   attachment to `PrimaryAttachments::KeepAll` -- the attachment survives
   without any extra profile config.
3. Renders `command(&plan)` and runs it through real mkvmerge.
4. Re-identifies the output via `-J` and asserts:
   - `out_id.tracks.len() == 1`, `kind == "subtitles"`
   - `track_name(track) == "Renamed"` (reuses the file's existing
     `track_name` helper, originally written for `live_keep_donor_trails_primary`)
   - `track.get("default_track") == Some(PropValue::Bool(true))`
   - `out_id.attachments.len() == 1`, `out_id.attachments[0].file_name == "note.txt"`
     (the original file's basename, confirming the attachment round-tripped
     unmodified)

## mkvmerge probe (SI-3, run before writing the test)

Ran directly against the installed real binary (v100), throwaway temp dir,
before touching any Rust code:

```
$ mkvmerge --version
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit

$ mkvmerge -q -o out_nomime.mkv --attach-file note.txt seed.srt
exit=0

$ mkvmerge -q -o out_mime.mkv --attachment-mime-type text/plain --attach-file note.txt seed.srt
exit=0

$ mkvmerge -J out_nomime.mkv | ...attachments...
[{'content_type': 'text/plain', 'description': '', 'file_name': 'note.txt', 'id': 1, 'properties': {'uid': ...}, 'size': 23}]

$ mkvmerge -J out_mime.mkv | ...attachments...
[{'content_type': 'text/plain', 'description': '', 'file_name': 'note.txt', 'id': 1, 'properties': {'uid': ...}, 'size': 23}]
```

Finding: mkvmerge accepts a plain `.txt` attachment either way (exit 0 in
both cases); with or without the explicit `--attachment-mime-type
text/plain` flag, `-J` reports the identical `content_type: "text/plain"`
(mkvmerge already guesses the MIME type from the `.txt` extension). The
explicit flag changes nothing observable but is what the task brief
specifies, so the fixture-building `Command` in the test keeps it. This is
recorded verbatim in a block comment directly above the test in the source
file (SI-3 discipline: the reasoning is pinned next to the code it justifies,
not just in this report).

No behavior needed adjusting from what the brief assumed; the probe
confirmed the brief's premise (explicit mime type is accepted) directly.

## Test evidence: RED then GREEN

RED: temporarily changed the `track_name` assertion to an impossible value
(`"WRONG_RED_STEP_VALUE"`) and ran the single new test:

```
$ cargo test -p muxsmith-core --test command_integration live_attachment_and_changes_round_trip -- --nocapture
...
thread 'live_attachment_and_changes_round_trip' panicked at crates/muxsmith-core/tests/command_integration.rs:554:5:
assertion `left == right` failed
  left: "Renamed"
 right: "WRONG_RED_STEP_VALUE"
test live_attachment_and_changes_round_trip ... FAILED
```

The left-hand side (`"Renamed"`) is what real mkvmerge actually produced
after running the planned `command(&plan)` argv, i.e. the test genuinely
drives the round trip rather than trivially passing. mkvmerge's own console
output above the panic (`Using the demultiplexer...`, `Multiplexing took 0
seconds`) confirms the real binary ran twice (fixture build + planned
command).

Restored the correct value and reran:

```
$ cargo test -p muxsmith-core --test command_integration -- --nocapture
running 4 tests
test reference_example_end_to_end ... ok
test live_keep_donor_trails_primary ... ok
test live_attachment_and_changes_round_trip ... ok
test live_mkvmerge_accepts_planned_command ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

GREEN, all four tests in the file pass (was 3 before this task).

## Full gate (run before commit)

- `cargo test --workspace`: all 23 test binaries `ok`, 0 failed
  (`command_integration.rs` binary: 4 passed).
- `cargo fmt --all --check`: exit 0, no diff.
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/tests/command_integration.rs` (+109 lines, one new
  test + its profile constant + a doc comment block). No other file touched.

## Self-review

- **Completeness against the brief's three assertions:** present --
  `track_name == "Renamed"` (line 554), `default_track == true` (line 555),
  attachment present with the original `file_name` "note.txt" (line 566).
- **Discipline:** diff touches only `command_integration.rs`
  (`git diff --stat`: 1 file changed, 109 insertions, 0 deletions). No
  changes to `command.rs`, `planner.rs`, or any other source file. Reused
  the file's existing `mkvmerge()` locate-or-skip helper and `track_name()`
  helper rather than duplicating them, per the workspace rule.
- **Test output pristine:** no stray `eprintln!`/debug prints left in; the
  only `eprintln!` is the established skip-message idiom shared with the
  other two live tests. mkvmerge's own stdout (progress lines) appears
  because the two live-mux `Command`s do not suppress stdout, exactly like
  the two pre-existing live tests in this file (not something this task
  introduced or should suppress, since it would depart from the established
  pattern for a cosmetic-only reason).
- **Typography:** grepped the diff for em-dash/en-dash/curly-quote/ellipsis/
  NBSP characters -- none found; the one double-hyphen usage (` -- `)
  matches the file's own pre-existing ASCII em-dash substitution convention
  (five other instances already in the file).
- **Scope:** did not touch `command.rs`'s attachment/`--attach-file`
  handling (confirmed unaffected: `plan.attachments.add_files` is for
  externally *added* attachments via profile `add` locators, a different
  mechanism from the primary's own pre-existing attachments this test
  exercises, which flow through as `PrimaryAttachments::KeepAll`/`Subset`/
  `DropAll`).

## Issues or concerns

None. The brief's premise (explicit `--attachment-mime-type text/plain` is
accepted) held exactly as stated; no adjustment to the planned approach was
needed, and no ambiguity in the brief required escalation.
