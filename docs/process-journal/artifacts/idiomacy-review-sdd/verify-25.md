# Verify-25: `select(CONST_SLICE.to_vec())` in prop_matcher.rs

**Verdict: CONFIRMED**

## Finding under test

14 sites in `crates/muxsmith-core/tests/prop_matcher.rs` call proptest's
`sample::select(CONST_SLICE.to_vec())` although `select` in pinned proptest
=1.11.0 takes `impl Into<Cow<'static, [T]>>`, which the `&'static` slices
satisfy directly. Replacement: pass the slice, delete every `.to_vec()`.

## Checks performed

### (a) Cited code says what the finding claims - YES

`grep -n "to_vec()"` on the file at HEAD returns exactly 14 `select(....to_vec())`
sites: lines 130, 140, 142, 143, 145, 147, 148, 149 (two calls on that line:
`STRING_PROPS` and `REGEXES`), 162, 164, 166, 173, 174. The finding lists the
last pair as 172/173 instead of 173/174 (the tuple opens at 172); count and
construct are exact, the one-line offset on the final pair is immaterial.

All ten constants involved are `&'static [&'static str]`:

- Local in the test file: `STRING_PROPS` (l.34), `STRING_PROPS_TRACK` (l.42),
  `BOOL_PROPS` (l.43), `INT_PROPS` (l.52), `INT_PROPS_TRACK` (l.54),
  `CODEC_POOL` (l.55), `STRING_POOL` (l.59), `REGEXES` (l.77) - all
  `const X: &[&str]` (elided lifetimes in const items are `'static`).
- Imported: `TYPE_VALUES` and `CODEC_KIND_NAMES` in
  `crates/muxsmith-core/src/capability/mod.rs` (l.53, l.126), both
  `pub static ... : &[&str]`.

### (b) Replacement is current idiom for the pinned toolchain - YES

Checked against the pinned version's own source, not training memory:
`~/.cargo/registry/src/index.crates.io-.../proptest-1.11.0/src/sample.rs`,
`pub fn select` at line 156:

```rust
/// `values` should be a `&'static [T]` or a `Vec<T>`, or potentially another
/// type that can be coerced to `Cow<'static,[T]>`.
pub fn select<T: Clone + fmt::Debug + 'static>(
    values: impl Into<Cow<'static, [T]>>,
) -> Select<T> {
```

The doc comment itself names `&'static [T]` as the first-listed intended input.
`T = &'static str` satisfies `Clone + Debug + 'static`. Both forms produce the
same `Select<&'static str>` yielding `&'static str`, so every downstream
`.prop_map(...)` is unchanged - "identical downstream types" holds.

Empirically compile-verified in a scratch crate (outside the repo) pinned to
`proptest = "=1.11.0"`, edition 2024:
`select(STRING_PROPS).prop_map(|s| s.to_string())` with
`const STRING_PROPS: &[&str]` compiles and runs. The `.to_vec()` is a pure
extra allocation per strategy construction and non-idiomatic per the crate's
own documentation.

### (c) Duplication with load-bearing difference - N/A

Not a duplication finding.

### (d) yagni without concrete construct - N/A

Tag is `idiom`, and both construct and replacement are concrete.

## Decision guard - no hit

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for
`prop_matcher`, `to_vec`, `select(`, `proptest`. Hits are only the Plan-5.5
T14 DONE entry (ROADMAP l.145) and the design memo's "property-based tests
(proptest)" mandate (v1 design spec l.408) - neither records a decision about
the `.to_vec()` call style, and neither cosmetic-cleanup group K nor the
test-hygiene collection tracks it. Not TRACKED, no DECISION_CONFLICT.
