# Verify-18: select(TYPE_VALUES.to_vec()) in prop_planner.rs

**Verdict: CONFIRMED**

## Finding under test

`select(TYPE_VALUES.to_vec())` at `crates/muxsmith-core/tests/prop_planner.rs:144` and `:156` allocates a `Vec` for nothing; `select(TYPE_VALUES)` is the drop-in idiom because proptest 1.11.0's `select` takes `impl Into<Cow<'static, [T]>>` and `TYPE_VALUES` is `&'static [&'static str]`.

## Checks performed

### (a) Cited code says what the finding claims — yes

- `prop_planner.rs:144`: `select(TYPE_VALUES.to_vec()).prop_map(|t| exact_one("type", Scalar::Str(t.to_string())))`
- `prop_planner.rs:156`: `select(TYPE_VALUES.to_vec()),` (inside the `arb_track` tuple strategy)
- `src/capability/mod.rs:53`: `pub static TYPE_VALUES: &[&str] = &["audio", "buttons", "subtitles", "video"];` — references in a `static` item are `'static`, so the type is `&'static [&'static str]`.

### (b) Replacement is current idiom for the pinned toolchain — yes

Verified against the pinned crate source itself, not training memory. `Cargo.toml` pins `proptest = "=1.11.0"`, `Cargo.lock` confirms 1.11.0. In `proptest-1.11.0/src/sample.rs`, `pub fn select` sits at line 156 exactly as the finding cites:

```rust
pub fn select<T: Clone + fmt::Debug + 'static>(
    values: impl Into<Cow<'static, [T]>>,
) -> Select<T>
```

Its own doc comment names the static slice as the primary form: "`values` should be a `&'static [T]` or a `Vec<T>`". `std::borrow::Cow` has `impl<'a, T: Clone> From<&'a [T]> for Cow<'a, [T]>`, so `select(TYPE_VALUES)` converts via `Cow::Borrowed` with `T = &'static str` (unambiguous: no other `From` impl applies to a `&[T]` argument). The produced `Value` type is `&'static str` in both variants, so the downstream `.prop_map(|t| t.to_string())` and the tuple consumer are untouched — a true drop-in.

Compile-checked in isolation (scratchpad crate, edition 2024, `proptest =1.11.0`): `select(TYPE_VALUES).prop_map(|t| t.to_string())` builds cleanly with no inference errors.

The finding's exemption of the `select(vec![...])` literal sites is also correct: `Vec<T>` is the doc-named second form, and a literal has no static slice to borrow (a `&'static` promotion would be possible but is not the documented default; not part of this finding).

### (c) Duplication with load-bearing difference — N/A

Not a duplication finding.

### (d) yagni without concrete construct/replacement — N/A

Tag is `idiom`; construct and replacement are both concrete.

## Decision guard

Grepped `docs/superpowers/specs/*.md` (D1-D35), `docs/IDEAS.md`, `docs/ROADMAP.md` for `prop_planner`, `TYPE_VALUES`, `select(`, `to_vec`: zero hits. Read the ROADMAP cosmetic-cleanup group K and the test-hygiene collection (docs-tree B-minors) in full: both enumerate unrelated items (dead `at` param, module docs, yaml_fragment fidelity, etc.); nothing covers proptest strategy construction. The only proptest entry is the DONE line for Plan 5.5 T14 (adoption of proptest =1.11.0), which does not pin the `to_vec` form. Not tracked, no conflicting decision.

## Note (outside the finding's scope, no bearing on the verdict)

The identical pattern exists at `crates/muxsmith-core/tests/prop_matcher.rs:142` and `:173`. The finding is scoped to `prop_planner.rs` (slice F2a); if applied, the same one-token fix applies there.
