# Task 11 report: `command` - attachments, chapters, tags argv

> Note: this file previously held a stale report for an unrelated,
> differently-numbered "CLI scaffold and `schema` subcommand" task from an
> earlier plan. Overwritten below with the actual Plan 3 Task 11 (the
> `command` module's attachment/chapter/tag argv flags, per
> `.superpowers/sdd/task-11-brief.md` in this repo state).

## Summary

Implemented the per-group flags that close out spec 4.9 item 2 for the
`command` module: `--no-chapters` (2a), `--no-global-tags`/`--no-track-tags`
(2b), and the attachment filter (2c). These slot into `push_group` before
track selection, per the canonical order. Global `--chapters <path>` and
`--attach-file <p>` (Task 9) and track selection/per-track-properties/order
(Tasks 9-10) were not touched.

## Files changed

- `crates/muxsmith-core/src/command.rs`: added `push_group_chapters`,
  `push_group_tags`, `push_group_attachments`; wired all three into
  `push_group` ahead of `push_track_selection`; updated the module and
  `push_group` doc comments (Task 11 is no longer "not yet implemented");
  imported `PrimaryAttachments`.
- `crates/muxsmith-core/tests/command.rs`: added 7 new golden tests plus two
  small helper constructors (`single_group_plan`, `multi_group_plan`); fixed
  one pre-existing Task 10 golden test (see "Note on a pre-existing test"
  below).

## Implementation

```rust
fn push_group(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    push_group_chapters(argv, plan);
    push_group_tags(argv, plan);
    push_group_attachments(argv, plan, source);
    push_track_selection(argv, plan, source);
    push_track_properties(argv, plan, source);

    argv.push("(".to_string());
    argv.push(source.display().to_string());
    argv.push(")".to_string());
}
```

- `push_group_chapters`: `ChapterSource::Keep` emits nothing;
  `Drop`/`External(_)` both emit `--no-chapters` (the global `--chapters
  <path>` for `External` was already emitted by `push_global` in Task 9;
  this only adds the per-group suppression flag).
- `push_group_tags`: two independent `if !keep` checks against
  `plan.tags.global_keep`/`track_keep`, each unconditional per group (no
  primary/donor distinction - tags apply the same way to every group).
- `push_group_attachments`: `source != plan.source.as_path()` -> donor,
  always `--no-attachments` (D10, no exceptions). Otherwise primary:
  `KeepAll` -> nothing; `Subset(ids)` -> `--attachments`, `ids.join(",")`
  (ids trusted pre-sorted from resolution, per the brief - no defensive
  re-sort, unlike `push_track_selection`/`push_track_properties` which do
  sort defensively since their inputs aren't already-sorted-and-documented
  as such); `DropAll` -> `--no-attachments`.

No signature changes: `push_group` already took `plan` and `source`, enough
to distinguish primary vs. donor by path comparison and to read
`plan.chapters`/`plan.tags`/`plan.attachments`.

## TDD

**RED** (`cargo test -p muxsmith-core --test command`, before implementation):
5 of the 7 new tests failed as expected (`attachments_drop_all_on_primary`,
`attachments_subset_on_primary_no_attachments_on_donor`,
`chapters_drop_emits_no_chapters_on_every_group_and_no_global_flag`,
`chapters_external_emits_global_chapters_and_no_chapters_on_every_group`,
`tags_dropped_emit_flags_on_every_group`); 6 passed (the pre-existing Task
9-10 tests, plus `add_files_emit_global_attach_file` and
`attachments_keep_all_emits_no_flag_on_primary`, which pass vacuously since
they assert the *absence* of new flags and Task 9-10's code already
produces that absence).

```
test result: FAILED. 6 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
```

**GREEN** (after implementing the three `push_group_*` helpers): all 7 new
tests passed, but a pre-existing Task 10 test (`per_track_properties_and_multi_group`)
then failed - see next section. After fixing that test's expectation, full
re-run:

```
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Note on a pre-existing test (in-scope golden-test update, not a Task 10 logic change)

`per_track_properties_and_multi_group` (written for Task 10, before Task 11
existed) has a donor group (`/m/e.tr.srt`) and asserted the full argv
*without* a `--no-attachments` flag on that donor group. Task 11's canonical
contract (brief reference, item 2c) requires every donor group to always get
`--no-attachments`, independent of `PrimaryAttachments` - so this is now a
correct, required emission, and the old test's literal was stale relative to
the (not-yet-implemented-at-the-time) full contract. I updated only the
expected `Vec<String>` literal (inserted `"--no-attachments"` between the
primary group's closing `")"` and the donor group's `"--no-video"`); the
plan under test, the assertions about track selection/properties, and
`push_track_selection`/`push_track_properties`/`input_groups`/`group_index`
themselves are untouched. This is a golden-test lock update explicitly
anticipated by the file's own header comment ("Task 9-11 progressively lock
the canonical argv contract"), not a change to Task 9-10 grouping logic.

## Golden tests added

1. `attachments_subset_on_primary_no_attachments_on_donor` - `Subset(vec![0,
   2])` on the primary with a donor group present -> primary gets
   `--attachments 0,2`, donor gets `--no-attachments`.
2. `attachments_drop_all_on_primary` - `DropAll` -> primary group
   `--no-attachments` (single-group plan).
3. `attachments_keep_all_emits_no_flag_on_primary` - `KeepAll` -> no
   attachment flag anywhere (single-group plan).
4. `add_files_emit_global_attach_file` - `add_files: [p("/m/x.ttf")]` ->
   global `--attach-file /m/x.ttf` (re-confirms Task 9 behavior still
   holds after this change).
5. `chapters_drop_emits_no_chapters_on_every_group_and_no_global_flag` -
   `ChapterSource::Drop` -> `--no-chapters` on both the primary and donor
   group; no global `--chapters`.
6. `chapters_external_emits_global_chapters_and_no_chapters_on_every_group`
   - `ChapterSource::External(p("/m/e.xml"))` -> global `--chapters
   /m/e.xml` plus `--no-chapters` on both groups.
7. `tags_dropped_emit_flags_on_every_group` - `TagFlags { global_keep:
   false, track_keep: false }` -> `--no-global-tags --no-track-tags` on
   both groups.

Tests 5-7 use a new `multi_group_plan` helper (primary video track + donor
subtitle track) specifically so "every group" is actually exercised across
two groups, not just the primary. Tests 2-4 use a new `single_group_plan`
helper where a second group isn't needed to demonstrate the behavior.

## Gate (full, run twice: once before commit, once after as final sanity)

- `cargo test --workspace`: all suites green (workspace total unaffected
  outside `command.rs`'s own crate; `muxsmith-core` test binary `command`:
  11/11 passed).
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Self-review

- Verified `push_group`'s new call order matches the brief's canonical
  order exactly: chapters (a) -> tags (b) -> attachments (c) -> [existing]
  track selection (d) -> per-track properties (e) -> bracketed source (f).
- Verified the primary/donor distinction reuses the same idiom already used
  elsewhere in the file (`a.source.as_path() == source`), just inverted and
  applied to `plan.source` instead of an assignment's source.
- Verified scope discipline by diffing: only `push_group`'s three new
  callees were added; `input_groups`, `group_index`, `push_track_selection`,
  `push_track_properties`, `push_track_order`, and `push_global` are
  byte-for-byte unchanged (confirmed via `git diff`).
- Confirmed no signature changes were needed to `push_group` - it already
  received `plan` and `source`.
- Confirmed `missing_docs` compliance: new helpers are private, and the
  `#![deny(missing_docs)]` gate (via clippy/build) passed without needing
  doc comments on them; I still gave each a `//` comment for readability,
  consistent with the file's existing style for private helpers.
- Considered whether `Subset(ids)` should defensively re-sort before
  joining (as `push_track_selection`/`push_track_properties` do for their
  own inputs). Brief item 2c explicitly documents ids as "already sorted
  ascending from resolution," so I did not add a redundant sort - matching
  scale-appropriate design (no unearned defensive code) rather than
  blindly mirroring the other two functions' pattern, which sort because
  *their* inputs (filtered from `plan.assignments`, an unordered vec) are
  not already guaranteed sorted.

## Concerns

None. The one deviation from a literal "don't touch Task 10 tests" reading
is documented above and is, in my judgment, squarely in scope: it's a
golden-test literal update forced by a newly-locked contract item (donor
`--no-attachments`), not a change to Task 10's grouping/selection logic or
assertions about it.
