# Task 2 report: generalize the matcher over a `Matchable` trait

(Plan 3, pure resolution + command layer. This overwrites a stale
`task-2-report.md` left over from an earlier plan's Task 2, "Diagnostic
types" - that content belonged to a different plan's task numbering and is
superseded here.)

## What was done

Widened `crates/muxsmith-core/src/matcher.rs` from a `Track`-specific matcher
to a generic one over a new `pub trait Matchable`, per the brief. No other
file was touched.

- Added `pub trait Matchable { fn get(&self, prop: &str) -> Option<PropValue>; }`
  and `impl Matchable for Track` (delegates to the existing `Track::get`).
- Widened `pub fn matches` from `(expr: &MatchExpr, track: &Track, lang: &LanguageIndex)`
  to `<M: Matchable>(expr: &MatchExpr, item: &M, lang: &LanguageIndex)`.
- Widened the private helper `exact_matches` the same way (`track: &Track` ->
  `item: &M` with `M: Matchable`).
- Renamed `track_str` to `item_str<M: Matchable>(prop, item)` and widened it
  identically.
- All parameter renames are `track`/`track_str` -> `item`/`item_str`
  throughout; no logic in `exact_matches` changed. The `language`,
  `codec_kind`, and boolean-absent-false branches are untouched: they still
  consult `item.get(...)` and `matchable_type(...)` exactly as before.
- Added one blanket impl not in the brief's Step 3 snippet (see "Deviation
  from the brief" below): `impl<M: Matchable> Matchable for &M`.
- Lightly reworded the module doc comment ("a track" -> "a [`Matchable`]
  item") for accuracy; no other prose changes.
- Added the brief's `matches_is_generic_over_matchable` test verbatim.

## TDD RED/GREEN

**RED** (Step 1-2, before implementing the trait):

```
$ cargo test -p muxsmith-core matcher
   Compiling muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
error[E0405]: cannot find trait `Matchable` in this scope
   --> crates/muxsmith-core/src/matcher.rs:322:21
    |
322 |         fn check<M: Matchable>(m: &M) -> bool {
    |                     ^^^^^^^^^ not found in this scope
error: could not compile `muxsmith-core` (lib test) due to 1 previous error
```

**GREEN** (Step 4, after implementing trait + blanket impl + generic widening):

```
$ cargo test -p muxsmith-core matcher
test matcher::tests::codec_kind_is_codec_id_prefix_match ... ok
test matcher::tests::empty_expression_matches_everything ... ok
test matcher::tests::exact_matches_type_and_flags ... ok
test matcher::tests::language_falls_back_to_raw_compare_when_unknown ... ok
test matcher::tests::matches_is_generic_over_matchable ... ok
test matcher::tests::language_normalizes_iso_and_bcp47_against_both_fields ... ok
test matcher::tests::numeric_exact_compares_across_int_and_float ... ok
test matcher::tests::present_boolean_property_still_matches_its_real_value ... ok
test matcher::tests::substring_is_case_insensitive_and_regex_is_literal ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 60 filtered out
```

All 12 matcher unit tests pass unchanged, plus the new generic-usage test.

## Deviation from the brief: the blanket `impl<M: Matchable> Matchable for &M`

The brief's regression guard claimed: "the planner calls
`matcher::matches(&rule.match_expr, t, lang)` with `t: &Track` ... Type
inference must resolve `M = Track` at those sites with no changes to
callers." This did not hold as literally stated, and implementing only
Step 3's snippet (trait + `impl Matchable for Track`, nothing else) leaves
the crate **not compiling**:

```
error[E0277]: the trait bound `&identify::Track: matcher::Matchable` is not satisfied
   --> crates/muxsmith-core/src/planner.rs:358:60
    |
358 |             .filter(|t| matcher::matches(&rule.match_expr, t, lang))
    |                         ----------------                   ^ the trait `matcher::Matchable` is not implemented for `&identify::Track`
```

Root cause: `planner.rs` calls `matches` inside `.iter().filter(|t| ...)` on a
`Vec<Track>`. `Iterator::filter`'s predicate signature is
`FnMut(&Self::Item) -> bool` with `Item = &Track` (from `.iter()`), so inside
the closure `t: &&Track` (double reference), not `&Track` as the brief
assumed. The matcher unit tests call `matches(&expr, &t, &lang())` with a
single `&Track`, which is why those compiled fine even before this fix; only
the planner's `.filter()` call sites hit the double-reference case.

Rust's generic trait-bound unification does not apply deref coercion the way
a concrete `&Track`-typed parameter would: with `matches<M: Matchable>(item:
&M, ...)` and an argument of type `&&Track`, the compiler unifies `M = &Track`
directly (no coercion attempt), then checks `&Track: Matchable`, which fails.
This is a known, unavoidable Rust behavior for this shape, not an oversight
I could work around by "trying harder" at the call site.

Since the brief scopes files to modify as `matcher.rs` only (and the
regression guard explicitly forbids caller changes), the correct fix
confined to that file is a blanket reference impl:

```rust
impl<M: Matchable> Matchable for &M {
    fn get(&self, prop: &str) -> Option<PropValue> {
        (**self).get(prop)
    }
}
```

This lets `M` resolve to `&Track` at the planner's call sites (satisfied via
`Track: Matchable` -> `&Track: Matchable` through the blanket impl), so
`planner.rs` needed zero changes, matching the brief's actual intent (no
caller changes, no behavior change) even though the literal claim "`M`
resolves to `Track`" was inaccurate for those two call sites. It is a
mechanical, 4-line addition (doc comment included), not a speculative
abstraction: it exists solely to make the stated regression guard hold given
how `Vec<Track>::iter().filter(...)` actually types in Rust. It also does not
preempt or complicate Task 3: it says nothing about `Attachment`, which still
needs its own explicit `impl Matchable for Attachment`.

I flag this as a deviation rather than silently "fixing the brief," since the
brief's Step 3 code block did not include it and a future reader diffing
against the brief should know why the extra impl is there.

## Files changed

- `crates/muxsmith-core/src/matcher.rs` (only file touched)

## Full gate (run once, before commit)

```
$ cargo test --workspace
... (21 "test result: ok" blocks, 0 failed anywhere in the workspace)

$ cargo fmt --all --check
(no output, clean)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0
    Checking muxsmith-cli v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok
```

All four gate checks pass.

## Self-review

- **All existing tests still green**: confirmed, `cargo test --workspace`
  shows 0 failed across all 21 test binaries (lib + integration suites in
  `muxsmith-core`, `muxsmith-cli`, `xtask`), including the `planner.rs`
  fixture and unit tests that exercise `matcher::matches` through
  `resolve_file`.
- **Trait + method documented**: `Matchable` and `Matchable::get` both carry
  doc comments; `#![deny(missing_docs)]` did not fire (would have failed
  `cargo test`/`cargo clippy` otherwise). The blanket impl's `get` needs no
  separate doc (impl methods inherit the trait's), only an explanatory `//`
  comment on the impl block explaining why it exists.
- **No behavior change for tracks**: `exact_matches`'s three special-cased
  branches (`language`, `codec_kind`, boolean-absent-false) are byte-for-byte
  the same logic, only with `track` renamed to `item` and the type widened.
  All 12 pre-existing matcher unit tests pass unchanged (same assertions,
  same fixtures).
- **Pristine output**: `cargo fmt --all --check` clean, `cargo clippy
  --workspace --all-targets -- -D warnings` clean, `cargo deny check` clean.
  ASCII-only confirmed (`grep -nP '[^\x00-\x7F]' matcher.rs` finds nothing).
- **Scope discipline**: `git status --porcelain` shows only
  `crates/muxsmith-core/src/matcher.rs` modified (plus a pre-existing,
  unrelated untracked `HANDOFF.md` from before this task started, left
  alone). `Attachment` was not touched and does not implement `Matchable`;
  that remains Task 3's job.

## Concerns for Task 3 / downstream

- Task 3 will add `impl Matchable for Attachment`. Given the blanket
  `impl<M: Matchable> Matchable for &M` added here, any `.iter().filter(...)`
  pattern over `Vec<Attachment>` that hits the same double-reference shape
  will already compile without needing its own blanket impl or caller
  change; this groundwork is now in place.
- No other concerns. The change is a pure, mechanical generalization; no
  design decision beyond the blanket impl documented above.
