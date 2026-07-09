# Task 1 report: `identify` parses attachments and chapters

## Summary

Extended `crates/muxsmith-core/src/identify.rs` to parse `-J` attachments and
chapters, exactly per the task brief. Added `pub struct Attachment` (with
`get(&self, name: &str) -> Option<PropValue>` mirroring `Track::get`), and
`attachments: Vec<Attachment>` / `chapters: u64` fields on `Identification`,
populated in `Identification::from_json`. Added `fn parse_attachment` mirroring
`parse_track`.

## Files changed

- `crates/muxsmith-core/src/identify.rs` (only file touched)

## TDD evidence

### Step 1/2: RED - failing tests added first

Added the brief's three test cases verbatim (`parses_attachments_with_optional_fields`,
`absent_attachments_and_chapters_default_empty`, `attachment_get_exposes_match_properties`)
to the existing `#[cfg(test)] mod tests` block, before touching any production code.

`cargo test -p muxsmith-core identify` at that point failed to *compile* (the red
state the brief specifies, since `Attachment` and the two new fields did not exist
yet):

```
error[E0609]: no field `attachments` on type `identify::Identification`
   --> crates/muxsmith-core/src/identify.rs:392:23
    |
392 |         assert_eq!(id.attachments[1].content_type, None);
    |                       ^^^^^^^^^^^ unknown field
    |
    = note: available fields are: `file_name`, `format_version`, `container_recognized`, `container_supported`, `tracks`
[... 6 more E0609 errors for attachments/chapters across the 3 new tests ...]
error: could not compile `muxsmith-core` (lib test) due to 9 previous errors
```

### Step 3: implement

Added (in this order, mirroring the existing `Track`/`parse_track` placement):

- `pub struct Attachment { id, file_name, size, content_type, description, uid }`
  with a doc comment on the struct and every field. Per the task context note,
  the schema's attachment `type` field is deliberately not parsed (not
  matchable); the struct doc comment says so.
- `impl Attachment { pub fn get(&self, name: &str) -> Option<PropValue> }`,
  mapping `file_name`/`content_type`/`description` to `Str`, `id`/`size` to
  `Int` (with the `u64 as i64` cast, matching `Track::get`'s cast for its `id`),
  else `None`.
- `Identification.attachments: Vec<Attachment>` and `Identification.chapters: u64`,
  each with a doc comment.
- In `Identification::from_json`: parse `attachments` via
  `arr.iter().filter_map(parse_attachment).collect()` (defaulting to empty),
  and `chapters` via summing each entry's `num_entries` (defaulting to 0).
- `fn parse_attachment(v: &Value) -> Option<Attachment>`: required fields
  `id`/`file_name`/`size` via `?` (drop the entry if missing/wrong-typed,
  exactly like `parse_track`), optional `content_type`/`description` via
  `.map(str::to_string)`, `uid` nested under `properties.uid`.

Also tightened the module's top doc comment, which said "Attachments/chapters
parsing arrives in Plan 3" - now stale since this task is that arrival; updated
it to describe the track/attachment/chapter model in one line instead of
leaving a dangling forward reference.

### Construction-site sweep (brief Step 3 note)

Searched the whole workspace for `Identification {` struct literals:

```
grep -rn "Identification {" --include="*.rs" .
```

Only one hit outside the type/impl declarations themselves: the literal inside
`Identification::from_json` (which I updated directly). Every other reference
in the codebase (`crates/muxsmith-core/tests/planner_resolution.rs`,
`crates/muxsmith-core/tests/suggestions.rs`, `crates/muxsmith-core/src/planner.rs`,
`crates/muxsmith-cli/src/commands/identify.rs`) constructs `Identification` via
`Identification::from_json(...)`, never a struct literal, so `cargo build
--workspace --tests` had nothing else to point out. Confirmed with a clean
`cargo build --workspace --tests` (no errors) after implementing.

### Step 4: GREEN

```
$ cargo test -p muxsmith-core identify
running 8 tests
test identify::tests::absent_attachments_and_chapters_default_empty ... ok
test identify::tests::display_is_a_terse_phrase_not_the_debug_dump ... ok
test identify::tests::parses_attachments_with_optional_fields ... ok
test identify::tests::attachment_get_exposes_match_properties ... ok
test identify::tests::unrecognized_container_is_not_identifiable ... ok
test identify::tests::prop_value_from_json_scalars_only ... ok
test identify::tests::parses_tracks_and_container ... ok
test identify::tests::get_unifies_toplevel_and_nested_properties ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.00s
```

(5 pre-existing `identify` tests + 3 new ones, all green; the pre-existing
`parses_tracks_and_container` test continued to pass unchanged because the
committed fixture `series-s01e01.json` already carries empty `"attachments":
[]` / `"chapters": []`, which the new parsing logic correctly turns into an
empty vec / `chapters: 0`.)

### Step 5: full gate

```
$ cargo test --workspace          # all green, no FAILED anywhere (grep-verified)
$ cargo fmt --all --check         # clean after one `cargo fmt --all` pass (two
                                   # long assert_eq! lines needed wrapping)
$ cargo clippy --workspace --all-targets -- -D warnings   # clean
$ cargo deny check                # advisories ok, bans ok, licenses ok, sources ok
```

## Self-review findings

- Every new `pub` item (the `Attachment` struct, its 6 fields, `Attachment::get`,
  `Identification::attachments`, `Identification::chapters`) carries a doc
  comment, satisfying `#![deny(missing_docs)]`.
- `Attachment` derives `Debug, Clone, PartialEq` - the same set `Track` derives,
  no more, no less.
- `parse_attachment` and `Attachment::get` read as if written by the same
  author as `parse_track` and `Track::get`: same required-field-via-`?` /
  optional-field-via-`.map()` shape, same match-based `get`.
- No stray non-ASCII punctuation introduced (checked visually; all dashes in
  new comments are plain ASCII hyphens, no smart quotes).
- Did not touch `crates/muxsmith-cli/src/commands/identify.rs` (CLI JSON/human
  output) or `identify_live.rs` - out of scope for this task per the brief
  ("unit tests suffice here"; the CLI wiring is presumably a later Plan 3 task
  once command generation actually consumes attachments/chapters).
- Considered adding a live-mkvmerge attachment/chapter case to
  `identify_live.rs`, per the brief's "if a live case fits." Decided against it:
  it would need a new seed fixture plus `--attach-file`/`--chapters` mkvmerge
  arguments, which is scope the brief explicitly makes optional and which
  duplicates unit coverage already exercising the parser precisely. Flagging
  this decision explicitly in case a later task wants that live coverage.
- Note: this file previously held a stale report for a *different* plan's
  "Task 1" (the Plan 1 workspace scaffold, dated 2026-07-08). Overwritten with
  this task's report; `.superpowers/` is controller scratch, not a durable
  archive, so no history was preserved deliberately.

## Concerns

None. The change is additive and isolated to `identify.rs`; no other file
needed touching, and the full gate is green.
