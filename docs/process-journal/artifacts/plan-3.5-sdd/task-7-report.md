# Task 7 report: keep-mode `--track-order` - primary leads, donors trail (D20 revision B)

Note: this report file previously held stale content from an unrelated
earlier "Task 7" (chapters resolution, an earlier plan cycle). Overwritten
with this task's (Plan 3.5 Task 7) report below.

## Summary

Implemented decision B from the updated D20 memo
(`docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md`): under
`tracks.unmatched: keep`, `--track-order` now lists ALL primary tracks
first, in the primary's own source order, then donor tracks in
`tracks.rules` order. This reverses the Task 2-3 behavior, where
`push_track_order` listed only matched-assignment track ids, which let a
donor-only rule's track land ahead of the primary's own tracks in the
output (the whole-branch review's flagged bug on exactly the additive use
case `keep` exists for: "add a German sub, keep the rest").

## What changed

1. **`crates/muxsmith-core/src/planner.rs`**
   - `Plan` gains `pub primary_track_ids: Vec<u64>`, doc-commented: the
     primary's full track-id list in source order (as `-J` reports it).
   - Populated at the `Plan` construction site in `resolve_file` from
     `ident.tracks.iter().map(|t| t.id).collect()` (the already-resolved
     `Identification` for the primary, still in scope/borrowed, not moved,
     at that point in the function).

2. **`crates/muxsmith-core/src/command.rs`**
   - `push_track_order` now branches on `plan.keep_unmatched`:
     - **`keep`**: delegates to new `push_track_order_keep`, which emits
       `0:<id>` for every id in `plan.primary_track_ids` (group 0, since
       `input_groups` always puts the primary there), then extends with
       every DONOR assignment (`a.source != plan.source`, `track_id` is
       `Some`) via the existing `group_index` lookup, in assignment order.
       Matched-primary assignments are deliberately NOT emitted a second
       time (already covered by `primary_track_ids`).
     - **`drop`** (unchanged): the original logic, one entry per assignment
       with a resolved `track_id`, `group_index` of its source.
   - Empty-entries -> omit `--track-order` entirely, in both branches
     (unchanged invariant).
   - `push_track_properties` and `push_track_selection` untouched, as
     specified: matched-primary assignments still get their `changes`; the
     keep-mode primary-group selection-flag skip from Task 2 is unaffected.

3. **`crates/muxsmith-core/src/profile/validate.rs`**
   - `NoTrackRules`'s `config_path` changed from the literal `"tracks"` to
     `"tracks.rules"` (the empty check now tests `.rules`, not the whole
     block). No test asserted the old path (verified via grep); one test
     (`empty_tracks_list_is_rejected` in `validate_semantics.rs`) only
     asserts the diagnostic code, unaffected.

4. **`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`** (spec 4.5)
   - Added: keep-mode order is primary-first/source-order, donors trailing
     in rule order (D20); an explicit primary rule under `keep` applies its
     `changes` but does not reposition (reordering the primary is a
     `drop`-mode operation).

5. **Tests**
   - `crates/muxsmith-core/tests/command.rs`:
     - `keep_unmatched_suppresses_primary_selection_flags`: added
       `primary_track_ids: vec![0, 1, 2]` (video/audio-matched/subtitle-
       unmatched-but-kept); replaced the old "just check `--track-order` is
       present" assertion with an exact check,
       `assert_eq!(track_order, Some("0:0,0:1,0:2"))` - now a real
       regression guard for D20, not just a presence check.
     - Added `primary_track_ids` to the 6 other `Plan { .. }` literals /
       helper functions the compiler flagged (all `keep_unmatched: false`,
       so the field is inert there; values set to what a real `-J` of that
       fixture's primary would report, for fixture realism).
   - `crates/muxsmith-core/tests/command_integration.rs`:
     - Restructured `live_keep_unmatched_orders_only_listed_track` into
       `live_keep_donor_trails_primary`: builds a real 2-track primary
       (`PA`, `PB`, via two single-track SRTs merged as separate inputs,
       same technique as the sibling fixture) and a real, separate external
       donor file (`DONOR`, one subtitle track), runs a `keep_unmatched`
       Plan with one donor rule (no primary-track assignment at all) through
       the real installed mkvmerge v100, `-J`s the output, and asserts the
       track order is `[PA, PB, DONOR]` - the donor trails every primary
       track. File-level doc comment and the section-header comment above
       the test both updated to describe the new intent and note this test
       replaces the Task 3 version.

## TDD RED/GREEN

- Added the field to `Plan` + the exact-order assertion in
  `keep_unmatched_suppresses_primary_selection_flags` before touching
  `command.rs`'s behavior.
- Confirmed RED formally: stashed only the `command.rs` behavioral change
  (kept the field/tests) and reran the golden test -
  `left: Some("0:1") / right: Some("0:0,0:1,0:2")` - i.e. it failed against
  the old (matched-only) `push_track_order`, as expected.
- Restored the `command.rs` change (`git stash pop`); the same test then
  passed (GREEN), along with the rest of `tests/command.rs` (12/12).
- The live test's expected order was hand-verified against real mkvmerge
  (raw argv in bash) before finalizing the Rust assertion, per SI-3, rather
  than assumed; the Rust test then reproduced the same observed order.

## OBSERVED mkvmerge -J order (SI-3, primary+donor case)

Hand-verified (bash, real mkvmerge v100, before writing the Rust
assertion) and reproduced by the Rust live test itself:

```
mkvmerge -q -o out.mkv ( primary.mkv ) --no-attachments --no-video --no-audio \
  --subtitle-tracks 0 --no-buttons ( donor.mkv ) --track-order 0:0,0:1,1:0
```

primary.mkv tracks: `[(0, PA), (1, PB)]`; donor.mkv tracks: `[(0, DONOR)]`.

**Output `-J` order: `[(0, PA), (1, PB), (2, DONOR)]`** - donor trails both
primary tracks, confirming decision B exactly as specified. No surprises;
nothing to report as a concern from this verification step.

## Files changed

- `crates/muxsmith-core/src/planner.rs`
- `crates/muxsmith-core/src/command.rs`
- `crates/muxsmith-core/src/profile/validate.rs`
- `crates/muxsmith-core/tests/command.rs`
- `crates/muxsmith-core/tests/command_integration.rs`
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

## Gate (all four green, post-implementation and post-commit)

- `cargo test --workspace`: all green (unit + integration + doc tests
  across `muxsmith-core`, `muxsmith-cli`, `xtask`); the two live
  (mkvmerge-gated) tests in `command_integration.rs` ran for real (mkvmerge
  v100 present, not skipped) and passed.
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Self-review

- Scope held to exactly the D20-B change: `push_track_selection` and
  `push_track_properties` untouched, as required; no primary-reordering
  machinery added under `keep` (explicitly out of scope per the task).
- Double-listing avoided: the keep-mode donor loop filters out any
  assignment whose `source == plan.source`, so a matched-primary rule's
  track_id is never repeated after `primary_track_ids` already listed it.
- Empty case preserved: `push_track_order` still omits `--track-order`
  entirely when there is nothing to order in either mode (checked after
  branching, single `entries.is_empty()` gate for both paths).
- `primary_track_ids` documented as "unused when `keep_unmatched` is
  `false`" rather than left undocumented for that case, since
  `#![deny(missing_docs)]` requires the doc comment anyway and the
  behavioral scope is worth stating explicitly.
- The `NoTrackRules` config_path fix is a one-line, test-unobserved change;
  confirmed via grep that no test or downstream code depends on the old
  `"tracks"` string.
- Left `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md`,
  `HANDOFF.md`, and the untracked `2026-07-09-plan-4-design-decisions.md`
  out of this commit: they were already modified/untracked in the working
  tree before this task started (part of the review/handoff context this
  task was dispatched from), not part of Task 7's own deliverable list, so
  they're left for whatever process is tracking that separately.

## Concerns

None. The SI-3 verification matched the design memo's stated decision
exactly on the first real run; no rigging of the assertion was needed.

## Addendum: review fix - fast unconditional guard for the keep-mode donor branch

The review of this task flagged an Important coverage gap: the keep-mode
`--track-order` primary+donor branch (`push_track_order_keep`'s donor
half) was covered only by `command_integration.rs`'s
`live_keep_donor_trails_primary`, which is gated on `Mkvmerge::locate()`
and silently skips (prints and returns) when mkvmerge is absent, e.g. in
CI. The core D20 logic itself had no deterministic guard.

### Fix

Added `keep_unmatched_donor_trails_primary_track_order` to
`crates/muxsmith-core/tests/command.rs`, right after the sibling
`keep_unmatched_suppresses_primary_selection_flags` test (same
constructor style: literal `Plan { .. }`, `p()`/`change()` helpers). Builds
a `Plan` with `keep_unmatched: true`, `primary_track_ids: vec![0, 1]`
(primary has two tracks), and exactly one donor `Assignment` (`source` a
different path, `track_id: Some(0)`, `track_kind: Some("subtitles")`, no
`changes`); the remaining fields set as the sibling keep test does
(`AttachmentPlan::KeepAll` no add-files, `ChapterSource::Keep`,
`TagFlags { global_keep: true, track_keep: true }`, `TitleAction::Keep`).
Locates `--track-order` in the argv and asserts the following element is
exactly `"0:0,0:1,1:0"`: primary group 0 tracks 0 and 1 in source order,
then donor group 1 track 0.

No production code changed; this is purely a coverage addition for logic
already implemented and already covered by the live test.

### Confirming it is a real guard (not rigged)

1. Ran it alone against the current (correct) code -
   `cargo test --test command keep_unmatched_donor_trails_primary_track_order`
   - passed.
2. Scratch-reverted `push_track_order_keep` in
   `crates/muxsmith-core/src/command.rs` twice, in-place, each time
   reran just this test, then restored the original body (confirmed via
   `git diff crates/muxsmith-core/src/command.rs` showing no diff
   afterwards):
   - **Donor-first regression** (donor entries before
     `primary_track_ids`): test failed -
     `left: Some("1:0,0:0,0:1") / right: Some("0:0,0:1,1:0")`.
   - **Dropped-primary-id regression** (`primary_track_ids` truncated by
     one via `.iter().rev().skip(1).rev()`): test failed -
     `left: Some("0:0,1:0") / right: Some("0:0,0:1,1:0")`.
3. Restored the original `push_track_order_keep` body; `git diff` on
   `command.rs` confirmed clean (no production-code change survives).

Both failure modes the task called out (donor listed before primary, a
primary id dropped) are caught by the new assertion.

### Gate (post-fix)

- `cargo test --workspace`: all green, including `command.rs`'s 13 tests
  (was 12; the new test is the addition), and the untouched
  `command_integration.rs` live tests (ran for real, mkvmerge v100
  present).
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

### Covering-test output

```
running 13 tests
test add_files_emit_global_attach_file ... ok
test attachments_keep_all_emits_no_flag_on_primary ... ok
test attachments_drop_all_on_primary ... ok
test boolean_and_string_value_encoding ... ok
test keep_unmatched_donor_trails_primary_track_order ... ok
test chapters_drop_emits_no_chapters_on_every_group_and_no_global_flag ... ok
test chapters_external_emits_global_chapters_and_no_chapters_on_every_group ... ok
test attachments_subset_on_primary_no_attachments_on_donor ... ok
test global_and_single_video_group ... ok
test unmatched_donor_rule_opens_no_input_group ... ok
test keep_unmatched_suppresses_primary_selection_flags ... ok
test per_track_properties_and_multi_group ... ok
test tags_dropped_emit_flags_on_every_group ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Files changed (this addendum)

- `crates/muxsmith-core/tests/command.rs` (new test only)
- `.superpowers/sdd/task-7-report.md` (this addendum)
