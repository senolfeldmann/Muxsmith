# Task 4 report: enrich `Plan` and `Assignment` with resolution fields (defaults only)

(Plan 3, Task 4. This file previously held a stale report from an earlier
plan's differently-numbered Task 4 — profile model/loader; superseded here.)

## What was done

Added the six new public types from the reference block to
`crates/muxsmith-core/src/planner.rs` (`AppliedChange`, `ChapterSource`,
`TagFlags`, `TitleAction`, `PrimaryAttachments`, `AttachmentPlan`), widened
`Assignment` with `track_kind: Option<String>` and `changes:
Vec<AppliedChange>`, and widened `Plan` with `attachments: AttachmentPlan`,
`chapters: ChapterSource`, `tags: TagFlags`, `title: TitleAction`. Every new
pub type/field/variant carries the exact doc comment from the reference
block (crate has `#![deny(missing_docs)]`, confirmed still enforced and the
build is clean under it).

Updated `resolve_file`:
- The matched-track collection changed from `Vec<u64>` (ids only) to
  `Vec<(u64, String)>` (id + `Track::kind`), so the matched branch can set
  `track_kind: Some(tkind)` alongside `track_id: Some(tid)`.
- All five other `Assignment` literals (missing-external early-continue,
  unidentifiable-donor early-continue, ambiguous-external early-continue,
  zero-matched, ambiguous-matched) get `track_kind: None, changes: vec![]`.
- The single `Plan` literal gets the four new fields at the exact defaults
  specified: `attachments: AttachmentPlan { primary: KeepAll, add_files:
  vec![] }`, `chapters: ChapterSource::Keep`, `tags: TagFlags { global_keep:
  true, track_keep: true }`, `title: TitleAction::Keep`.

No resolution logic was implemented (attachments/chapters/tags/title/changes
stay hardcoded defaults; Tasks 5-8 fill them in). The only real logic change
is capturing `track.kind` alongside `track.id` in the match collection.

## TDD RED/GREEN

**Step 1 (RED):** extended `crates/muxsmith-core/tests/planner_resolution.rs`
with a new test `plan_and_assignment_carry_resolution_field_defaults`,
asserting the exact defaults from the brief's Step 1 snippet (attachments,
chapters, tags, title, `assignments[0].track_kind == Some("video")`,
`assignments[0].changes.is_empty()`), run against the existing
`P_VIDEO_AUDIO` single-video/audio profile fixture already used by
`resolves_each_rule_to_one_track`.

`cargo test -p muxsmith-core --test planner_resolution` failed to compile
with 11 errors: `AttachmentPlan`/`ChapterSource`/`TagFlags`/`TitleAction` not
found in `muxsmith_core::planner`, and `no field attachments/chapters/tags/
title on type &Plan`, `no field track_kind/changes on type Assignment`.

**Step 3 (implement):** added the types and updated every construction site
as above.

**Step 4 (GREEN):** `cargo test -p muxsmith-core --test planner_resolution`
-> 24 passed, 0 failed (23 pre-existing + the 1 new test).

## Gate (run before commit)

- `cargo build --workspace --tests` -> clean; confirmed no other
  `Plan`/`Assignment` construction sites exist outside `planner.rs` (grepped
  the whole repo for `Assignment {` and `Plan {` beforehand: only the struct
  defs and the 7 construction sites in `planner.rs`; `tests/suggestions.rs`
  and `muxsmith-cli` only consume `Batch`, never construct `Plan`/
  `Assignment`, so neither needed a literal update).
- `cargo test --workspace` -> all green (planner_resolution: 24 passed;
  every other test binary unaffected, 0 failed anywhere in the workspace).
- `cargo fmt --all --check` -> clean.
- `cargo clippy --workspace --all-targets -- -D warnings` -> clean.
- `cargo deny check` -> advisories ok, bans ok, licenses ok, sources ok.

## Files changed

- `crates/muxsmith-core/src/planner.rs`: new types + widened `Assignment`/
  `Plan` + all seven construction-site updates + `track.kind` capture.
- `crates/muxsmith-core/tests/planner_resolution.rs`: new defaults-assertion
  test.

## Self-review

- Every new pub type, pub field, and pub enum variant has a doc comment
  copied verbatim from the reference block; `missing_docs` is `deny`-level
  crate-wide and the build is clean, so this is enforced, not just claimed.
- Field order in `Assignment` and `Plan` matches the reference block exactly
  (`track_kind`/`changes` appended after `track_id`; `attachments`/
  `chapters`/`tags`/`title` appended after `assignments`).
- All pre-existing `planner_resolution.rs` tests still pass with the widened
  types (no literal in that file constructs `Plan`/`Assignment` directly, so
  none needed updating beyond the new test).
- No real resolution logic added: `changes` is `vec![]` everywhere, and the
  `Plan` literal's four new fields are the fixed defaults from the brief,
  not conditioned on profile content. The only behavioral change is the
  `track_kind` capture, which the brief explicitly scoped into this task.
- Typography: ASCII punctuation throughout the new code/comments/doc
  comments.
- `cargo build --workspace --tests` before touching anything beyond
  `planner.rs`/`planner_resolution.rs` confirmed there was nothing else to
  fix; the workspace still builds and tests clean now.

## Concerns

None. Scope stayed within Task 4; Tasks 5-8 have a clean, documented set of
types to populate for real, and the `command` crate (Tasks 9-11) has
`track_kind`/`changes`/`attachments`/`chapters`/`tags`/`title` available on
every `Plan`/`Assignment` to consume.
