# Task 3 report: attachment matching (`Attachment: Matchable`)

(Plan 3, pure resolution + command layer. This overwrites a stale
`task-3-report.md` left over from an earlier plan's Task 3, "Match
expression model" - that content belonged to a different plan's task
numbering and is superseded here.)

## What was done

Made `Attachment` (Task 1, `crates/muxsmith-core/src/identify.rs`)
implement `Matchable` (Task 2, `crates/muxsmith-core/src/matcher.rs`), so
`matcher::matches` evaluates attachment rules with the same algebra used
for tracks (spec 4.9). Single file touched: `matcher.rs`.

- Added `impl Matchable for Attachment { fn get(&self, prop: &str) ->
  Option<PropValue> { Attachment::get(self, prop) } }`, delegating to the
  existing inherent `Attachment::get` from Task 1. Placed next to `impl
  Matchable for Track`, matching the codebase's existing pattern (all
  `Matchable` impls live in `matcher.rs`).
- Updated the `Matchable` trait doc comment, which said "`Track` is the
  only implementor today; a later item type (e.g. attachments) implements
  it" - now stale since Attachment is that later item. Changed to name
  both implementors.
- Added the attachment-matching test from the brief verbatim
  (`attachment_matching_uses_the_same_algebra`), appended to the existing
  `mod tests` in `matcher.rs`.
- No changes to `identify.rs`, the match algebra, `Attachment`, or the
  `Matchable` trait signature. No new attachment properties.

## TDD evidence

**RED** - test added before the impl, `cargo test -p muxsmith-core
attachment_matching`:
```
error[E0277]: the trait bound `Attachment: matcher::Matchable` is not satisfied
   --> crates/muxsmith-core/src/matcher.rs:391:13
    |
389 |         assert!(!matches(
    |                  ------- required by a bound introduced by this call
390 |             &expr("substring: { content_type: pdf }"),
391 |             &font,
    |             ^^^^^ unsatisfied trait bound
help: the following other types implement trait `matcher::Matchable`
 --> crates/muxsmith-core/src/matcher.rs:23:1
 23 | impl Matchable for Track {
error: could not compile `muxsmith-core` (lib test) due to 5 previous errors
```
(5 errors: one per `matches(...)` call site in the new test, each on
`&font`.)

**GREEN** - after adding the `impl Matchable for Attachment` block:
```
running 1 test
test matcher::tests::attachment_matching_uses_the_same_algebra ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 72 filtered out
```

## Gate results (all before commit)

- `cargo test --workspace`: all green, 0 failed across every test binary
  (lib, integration test files, doc-tests).
- `cargo fmt --all --check`: one diff on first run (the new test's
  multi-line `any:` string literal wrapped differently than rustfmt
  wanted); ran `cargo fmt --all` to apply, `--check` clean afterward.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/src/matcher.rs`:
  - `use` list: added `Attachment` to the `crate::identify::{...}` import.
  - Doc comment on `pub trait Matchable` updated (no signature change).
  - New `impl Matchable for Attachment` block (3 lines of body).
  - New test `attachment_matching_uses_the_same_algebra` in `mod tests`.

## Self-review

- Impl is a pure delegation to `Attachment::get`, identical shape to the
  `Track` impl right above it; no new logic, no new attachment properties,
  no touch to `identify.rs` or the match algebra, matching the task's
  explicit boundaries.
- Considered whether the doc-comment update was in scope (task says "no
  new pub item" needs docs, and this isn't a new item) - did it anyway
  since leaving a doc comment that explicitly says "Track is the only
  implementor today" false the moment `impl Matchable for Attachment`
  exists a few lines below is worse than a one-line edit to keep docs
  honest. Not a behavior change.
- The test contains `use crate::identify::Attachment;` inside the test
  function body, which is redundant with the module-level `use
  crate::identify::{Attachment, ...}` already brought into test scope via
  `use super::*` - kept as-is because the brief's Step 1 quotes the test
  verbatim including that line, and clippy raises no warning for it (the
  glob import and explicit import resolve to the same item; this is not
  an unused-import case).
- Verified `Attachment`'s field set and `Attachment::get`'s exact match
  arms against `identify.rs` before writing the test, rather than trusting
  the brief's inline snippet blind - matched exactly (id/file_name/size/
  content_type/description/uid fields; file_name/content_type/description
  as `PropValue::Str`, id/size as `PropValue::Int`, no arm for `uid`).
- No pub items added beyond the trait impl itself (which needs no method
  docs per the task's own constraint), so `#![deny(missing_docs)]` is
  unaffected.

## Concerns

None. Small, self-contained task; implementation is a two-line delegation
plus one test, both matching the brief exactly.
