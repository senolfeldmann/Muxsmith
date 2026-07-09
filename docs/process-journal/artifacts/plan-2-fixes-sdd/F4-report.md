# F4 report: absent boolean-typed property compares equal to `false` for exact matching (decision #1)

## Status

DONE

## Commit

213e1e9 -- fix(core): absent boolean flags compare equal to false in exact matching

## What changed

`crates/muxsmith-core/src/matcher.rs`, in `exact_matches`'s fallback arm
(the `_ =>` branch handling all properties other than the special-cased
`language`/`codec_kind`): when `track.get(prop)` returns `None`, look up
`matchable_type(prop)`. If it is `Some(PropType::Boolean)`, compare `want`
against `PropValue::Bool(false)` via the existing `scalar_eq` helper --
mirroring mkvmerge, which only emits the vanity flags
(`flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`,
`flag_original`, etc.) when set, and Matroska's spec that these flags are
false-when-absent. For any other type (or an unknown property), the
absent case still returns `false`, unchanged from before.

Added imports `use crate::capability::{PropType, matchable_type};` (already
public in `capability::mod`). No changes to the `language`/`codec_kind`
special cases or to substring/regex, per the task's explicit scope.

## Test-first

Added to `crates/muxsmith-core/src/matcher.rs`'s `tests` module:

- `absent_boolean_property_compares_equal_to_false`: a track without
  `flag_hearing_impaired` matches `exact: { flag_hearing_impaired: false }`
  and does not match `exact: { flag_hearing_impaired: true }`.
- `present_boolean_property_still_matches_its_real_value`: a track with
  `flag_hearing_impaired: true` still matches `true` and not `false`
  (guards against breaking the non-absent path).
- `absent_non_boolean_property_still_does_not_match`: a track without
  `track_name` (a `String`-typed property) does not match
  `exact: { track_name: X }` -- absence stays "no match" for non-boolean
  types.

Confirmed red first: ran `cargo test --package muxsmith-core matcher::`
before implementing. `absent_boolean_property_compares_equal_to_false`
failed on `assert!(matches(&expr("exact: { flag_hearing_impaired: false }"),
&t, &lang()))` (the fallback arm's unconditional `None => false`); the other
two new tests passed trivially against the pre-change code (they assert
behavior that already held). After implementing, all 11 tests in
`matcher::tests` pass (8 pre-existing + 3 new).

## Verification

- `cargo test --workspace`: all crates green, no failures in any test binary.
- `cargo fmt --all --check`: clean after running `cargo fmt --all` (it
  reordered the new `use` line -- `PropType` before `matchable_type` --
  and collapsed one test's multi-line `track(...)` call to a single line;
  both are pure formatting, no semantic change).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- Diff is ASCII-only (`grep -n '[^\x00-\x7F]'` over the changed file: no
  matches).
- No new public items; `#![deny(missing_docs)]` unaffected.

## Scope notes

`HANDOFF.md` was present as an untracked file in the working tree at task
start (unrelated to F4) and was left untouched; only
`crates/muxsmith-core/src/matcher.rs` was staged and committed.
