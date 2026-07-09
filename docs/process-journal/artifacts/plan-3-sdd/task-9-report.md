# Task 9 report: `command` module - global section + track-order skeleton

## What was done

Implemented `pub fn command(plan: &Plan) -> Vec<String>` in a new
`crates/muxsmith-core/src/command.rs`, covering exactly the Task 9 slice of
the canonical argv contract:

- Global section: `--output <output>`, title (`Clear` -> `--title ""`,
  `Set(s)` -> `--title <s>`, `Keep` -> nothing), `--chapters <path>` for
  `ChapterSource::External`, `--attach-file <path>` per `add_files` entry.
- Input-group computation (`input_groups`): `[plan.source]` then every
  distinct `Assignment::source` not already present, in first-appearance
  order across `plan.assignments` -- including a donor source whose
  assignment has `track_id: None` (an unmatched optional external rule),
  since the canonical reference derives groups from `assignment.source`
  unconditionally, not filtered by whether a track resolved. This is a
  deliberate read of the brief's algorithm, not an incidental side effect;
  flagged in Concerns below since it is untested by the Task 9 golden case.
- Track selection per group (`push_track_selection`), fixed category order
  video/audio/subtitles/buttons, each emitting `--<cat>-tracks id,id`
  (ascending) if the group has any assigned track of that kind, else the
  category's `--no-*` flag. The select/no-flag name pairs are spelled out in
  a `Category` table rather than derived, since the no-flag names are not a
  mechanical transform of the category name (`--no-video`, not
  `--no-video-tracks`; `--no-subtitles`/`--no-buttons` plural where the
  select flags `--subtitle-tracks`/`--button-tracks` are singular).
- `--track-order g:tid,...` (`push_track_order`): one entry per assignment
  with a resolved `track_id`, in assignment order, `g` = that source's group
  index via `group_index`. Omitted entirely when no assignment has a track.

Per-track property options (Task 10) and the attachment-filter /
`--no-chapters` / `--no-*-tags` flags (Task 11) are explicitly NOT emitted.
`push_group` is structured with a comment marking exactly where each lands
(chapter/tag/attachment flags before `push_track_selection`, per-track
property options after it), so neither later task needs to reorder anything
already here.

## Files changed

- `crates/muxsmith-core/src/command.rs` (new): the module.
- `crates/muxsmith-core/src/lib.rs`: added `pub mod command;` (alphabetical,
  between `capability` and `discovery`).
- `crates/muxsmith-core/tests/command.rs` (new): the golden test from the
  brief, verbatim (source `/m/e.mkv`, one video-track assignment, all-default
  chapters/tags/attachments/title=Clear), asserting the full `Vec<String>`.

## TDD RED/GREEN

RED (`cargo test -p muxsmith-core --test command`, module absent):

```
error[E0433]: cannot find `command` in `muxsmith_core`
  --> crates/muxsmith-core/tests/command.rs:36:24
```

GREEN (after implementation):

```
running 1 test
test global_and_single_video_group ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Gate (run before commit)

- `cargo test --workspace`: all green (no failures across every crate/test
  binary; the new `command` test included).
- `cargo fmt --all --check`: clean (one auto-format pass applied to
  `command.rs` before the final check; a line-wrap on the joined-ids push
  that `rustfmt` collapsed to one line).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Self-review

- Doc comments: module `//!` doc present; `pub fn command` has a doc
  comment. Every other item in the module is private (no `pub`), so
  `#![deny(missing_docs)]` does not require docs on them; I added `//`
  (non-doc) comments anyway for the non-obvious ones (`group_index`'s
  `expect` invariant, the `Category` table's non-mechanical flag names, the
  `push_group` slot-in markers for Tasks 10/11).
- Path comparisons use `&Path` parameters (not `&PathBuf`), matching the
  codebase's existing convention (`discovery.rs`, `identify.rs`) and
  avoiding a clippy `ptr_arg` finding; comparisons go through `.as_path()`
  explicitly rather than relying on `PathBuf`/`Path` cross-type `PartialEq`
  impls, to keep the comparison obviously correct without checking std's
  `impl_cmp!` macro expansions.
- Verified the golden test asserts the FULL `Vec<String>` via `assert_eq!`
  against a literal vector (not a partial/contains check), as the task
  required.
- Verified ASCII-only content in the new files (no umlauts/dashes needed
  here; this module has no German text).
- Traced `group_index`'s `.expect(...)` panic path: it can only fire if
  `input_groups` were inconsistent with the assignments it was built from,
  which is structurally impossible given `push_track_order` and
  `input_groups` both walk `plan.assignments` the same way. Kept as an
  `expect` (not a silent fallback) so a future refactor that breaks this
  invariant fails loudly in tests rather than emitting a wrong group index.
- Noticed `.superpowers/sdd/task-9-report.md` already existed as a stale
  Plan 1 artifact (task numbering was reused across plans); confirmed it was
  already salvaged to `docs/process-journal/artifacts/plan-1-sdd/task-9-report.md`
  before overwriting it with this report.

## Concerns

- **Untested edge case, not golden-test-covered in Task 9**: a donor source
  from an external track rule that resolved to a file but matched zero
  tracks (`track_id: None`, `source: <donor path>`) still becomes its own
  input group under my reading of the brief's `input_groups` algorithm
  (`groups = [plan.source]` then distinct `assignment.source` values,
  unconditionally). That donor group would render as
  `--no-video --no-audio --no-subtitles --no-buttons ( <donor> )` with no
  tracks actually used from it. This follows the letter of the appended
  canonical reference ("then donor sources in first-appearance order across
  `assignments`", no track_id filter mentioned) and the brief's Step 3
  instruction verbatim, but it is not exercised by any Task 9 golden test in
  this brief. Flagging for the controller/reviewer to confirm this reading
  is intended before Task 12's real-mkvmerge round trip, since an unused
  input file in the argv is harmless to mkvmerge but is a slightly
  surprising argv shape if unintended.
- No other deviations from the brief's Step 3 instructions.

## Fix: input_groups group set

The reviewer confirmed the Concerns entry above as an Important finding:
`input_groups` included every distinct `assignment.source`, including a donor
whose only assignment has `track_id: None` (an external rule that found the
donor file but matched no track inside it). That yielded an empty input group
rendered as `--no-video --no-audio --no-subtitles --no-buttons ( <donor> )`,
contributing nothing to the mux and risking a real mkvmerge rejection in
Task 12.

**Fix** (`crates/muxsmith-core/src/command.rs`, `input_groups`): added
`a.track_id.is_some() &&` to the loop condition, so the group set is now the
primary (`plan.source`) always, plus only those donor sources that have at
least one assignment with a resolved `track_id`. This preserves
`group_index`'s invariant: every assignment `push_track_order` looks up has
`track_id = Some`, and its source is now guaranteed present in `groups`. Doc
comment above the function updated to state the new rule and the reason
(avoiding a mkvmerge-rejectable empty group).

**Regression test** (`crates/muxsmith-core/tests/command.rs`,
`unmatched_donor_rule_opens_no_input_group`): a `Plan` with the primary's
video assignment (`track_id: Some(0)`, source = primary) plus a second,
unmatched optional external-rule assignment (`track_id: None`,
`track_kind: None`, `source: /m/e.tr.srt`, `changes: vec![]`). Asserts the
argv contains no `/m/e.tr.srt` token (the donor never gets an input group)
and that `--track-order` is exactly `0:0` (only the primary's video track).

Covering test:

```
$ cargo test -p muxsmith-core --test command

running 2 tests
test unmatched_donor_rule_opens_no_input_group ... ok
test global_and_single_video_group ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Full gate re-run after the fix, all green:

- `cargo test --workspace`: all pass (every crate/test binary, including the
  new regression test).
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

Scope: only `input_groups` (the loop condition and its doc comment) and the
new test changed. No per-track property, attachment, chapter, or tag flags
touched (Tasks 10-11 remain untouched); no other files modified.
