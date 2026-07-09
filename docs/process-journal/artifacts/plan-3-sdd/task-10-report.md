# Task 10 report: `command` per-track property options + multi-input grouping

(Plan 3, Task 10. Note: this file previously held a stale report from an
earlier plan's unrelated Task 10 ("static overlap lint"); overwritten below.)

## Summary

Task 9 already implemented multi-group input handling (`input_groups`, `group_index`, `push_track_order` mapping each assignment to `group:tid`) and single-group track selection, but only had single-group golden coverage. Task 10's job: add per-track property options (spec 4.9 item 2e) and extend golden coverage to lock the full multi-group + per-track-property contract.

## Files changed

- `crates/muxsmith-core/src/command.rs`: added `push_track_properties` (called from `push_group`, after `push_track_selection`, before the `( source )` bracket) and a private `value_str` helper. Updated the module doc comment (Task 9-10 now both implemented; only Task 11's chapter/tag/attachment-filter flags remain). Added imports for `AppliedChange` and `Scalar`.
- `crates/muxsmith-core/tests/command.rs`: added a `change()` test helper, and two new tests: `per_track_properties_and_multi_group` (the brief's canonical golden case: primary `/m/e.mkv` video+audio with a boolean+string change on the audio track, donor `/m/e.tr.srt` subtitle track with two string changes, asserting the full argv including alphabetical property ordering, group indices, and `--track-order`) and `boolean_and_string_value_encoding` (single-group case isolating boolean `true`/`false` -> `1`/`0` encoding alongside a string value, with mixed-type alphabetical property ordering).

## TDD

**RED** (`cargo test -p muxsmith-core --test command`, after adding the two new tests, before touching `command.rs`):

```
test boolean_and_string_value_encoding ... FAILED
test per_track_properties_and_multi_group ... FAILED
test unmatched_donor_rule_opens_no_input_group ... ok
test global_and_single_video_group ... ok
```

Failure diffs showed exactly the missing slice: the multi-group scaffolding and `--track-order` from Task 9 already produced the correct skeleton (`--video-tracks 0 --audio-tracks 1 --no-subtitles --no-buttons ( /m/e.mkv ) --no-video --no-audio --subtitle-tracks 0 --no-buttons ( /m/e.tr.srt ) --track-order 0:0,0:1,1:0`); only the per-track property options between selection and the `(` bracket were absent, confirming the task boundary described in the context brief.

**GREEN** (after implementing `push_track_properties` + `value_str`):

```
running 4 tests
test global_and_single_video_group ... ok
test unmatched_donor_rule_opens_no_input_group ... ok
test boolean_and_string_value_encoding ... ok
test per_track_properties_and_multi_group ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Implementation notes

- `push_track_properties(argv, plan, source)`: collects `(track_id, &[AppliedChange])` for every assignment in this group with a resolved `track_id` (donors with `track_id: None` already excluded upstream by `input_groups`), sorts by `track_id` ascending, then for each track sorts its changes by property name ascending (defensive re-sort per the brief, even though the planner already emits them BTreeMap-ordered). For each change, looks up `capability::settable(&c.property)` for the mkvmerge option string and pushes `<option>`, `<tid>:<value_str>`.
- `value_str(&Scalar) -> String`: `Bool` -> `"1"`/`"0"`, `Str` -> the raw string (no quoting), `Int`/`Float` -> `to_string()`. `Int`/`Float` arms are currently dead in practice (no settable property in `SETTABLE` is typed `Integer`/`Float` today) but included for exhaustiveness and to match the brief's `value_str` spec verbatim rather than `unreachable!()`-ing on a type the enum still allows.
- Used `.expect("changes carry only validated settable properties")` on the `settable()` lookup, mirroring the existing `group_index` pattern in the file: `changes` only ever contains properties `validate.rs` already confirmed are in `SETTABLE` at config time, so a lookup miss here would indicate a bug upstream (validator regression), not a legitimate runtime case.
- Did not touch `input_groups`, `group_index`, or `push_track_order` -- Task 9 logic, already correct for the multi-group case (confirmed by the RED-state diffs showing the skeleton was already right before any Task 10 code existed).

## Golden test contract locked

For primary `/m/e.mkv` (video 0; audio 1 with `language=de`, `default_track=true`) + donor `/m/e.tr.srt` (subtitles 0 with `language=tr`, `track_name=Turkce`):

```
--output /out/e.mkv
--video-tracks 0 --audio-tracks 1 --no-subtitles --no-buttons
--default-track-flag 1:1 --language 1:de
( /m/e.mkv )
--no-video --no-audio --subtitle-tracks 0 --no-buttons
--language 0:tr --track-name 0:Turkce
( /m/e.tr.srt )
--track-order 0:0,0:1,1:0
```

Property ordering verified alphabetical across mixed types: `default_track` < `language` (primary), `language` < `track_name` (donor), and in the boolean/string test `forced_track` < `language`.

## Self-review

- Re-read `push_track_properties` and `value_str` against the canonical contract (reference item 2e) line by line: track-id ascending, property-name ascending, option lookup via `capability::settable(...).1`, value encoding rules -- all match.
- Confirmed scope discipline: no `--no-chapters`/`--no-*-tags`/attachment-filter flags added; `push_group`'s call order is `push_track_selection` -> `push_track_properties` -> `(` bracket, exactly where the Task 9 comment marked the slot; `input_groups`/`group_index`/`push_track_order` untouched.
- Ran the full per-commit gate (`cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check`) -- all pass, no warnings.
- Checked `#![deny(missing_docs)]`: `push_track_properties` and `value_str` are private (`fn`, not `pub fn`), so no doc comment is required; both carry a `//` explanatory comment anyway, consistent with the rest of the file's style for private helpers.
- Typography: ASCII only in code, comments, and test data (including the brief's literal `Turkce` test value, which is intentionally ASCII per the brief, not a diacritic-stripping violation).

## Concerns

None blocking. One minor observation for whoever picks up Task 11: `push_track_properties` iterates `plan.assignments` once per group via `.filter(|a| a.source.as_path() == source)` (same O(groups x assignments) pattern `push_track_selection` already uses); fine at expected profile sizes, not worth restructuring now (matches [[feedback_scale_appropriate_design]] -- no premature optimization).
