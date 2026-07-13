# Verify-19: BTreeMap mut+insert chains in prop_planner.rs (slice F2a)

**Verdict: CONFIRMED**

## Finding under test

Six builder sites in `crates/muxsmith-core/tests/prop_planner.rs` (lines 42, 51, 106, 176, 245, 281) construct fixed-entry BTreeMaps via `let mut map = BTreeMap::new(); map.insert(...)` statement chains; replacement `BTreeMap::from([(k, v), ...])`.

## Checks

### (a) Cited code says what the finding claims — PASS

Read at HEAD (`2f17880`). All six sites verified:

| Line | Context | Entries | Keys |
|------|---------|---------|------|
| 42 | `exact_one` | 1 | `prop.to_string()` (parameter) |
| 51 | `substring_one` | 1 | `prop.to_string()` (parameter) |
| 106 | `video_track` | 1 | literal `"codec_id"` |
| 176 | `arb_track` prop_map closure | 5 | literal strings |
| 245 | `arb_nonvideo_track` prop_map closure | 2 | literal strings |
| 281 | `sub` closure in `arb_ambiguous_ident` | 4 | literal strings |

Every site is a straight-line fixed-entry insert chain: no conditional inserts, no loops, no duplicate keys. One imprecision that is not load-bearing: at lines 42/51 the key is a function parameter, not a literal, so "key set is a fixed literal" is loose there; the entry *set* is fixed at every site, and `BTreeMap::from` takes arbitrary expressions, so the replacement applies identically. Not refutation-grade.

### (b) Replacement is current idiom for Rust 1.96.1 / edition 2024 — PASS

Verified via context7 against current stable std docs (doc.rust-lang.org/stable):

- `impl<K: Ord, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V>` exists on stable.
- The official `BTreeMap` docs themselves present `BTreeMap::from([...])` as the way to initialize "a BTreeMap with a known list of items" (solar_distance example).

Type-check of the concrete replacements: `(String, Scalar)` at 42/51, `(String, PropValue)` at 106/176/245/281 (mixed `PropValue::Str`/`::Bool` variants are the same type, so the tuple array is homogeneous). `String: Ord`. All compile-valid. No duplicate keys at any site, so the From overwrite semantics are irrelevant.

Repo consistency note: no existing `BTreeMap::from`/`HashMap::from` usage in `crates/`, 27 `BTreeMap::new()` sites repo-wide — the finding correctly scopes to the six fixed-entry sites in this file (slice F2a); dynamically populated `new()` sites elsewhere are out of scope and legitimately stay `new()`.

### (c) Duplication with load-bearing difference — N/A

Not a duplication/merge finding; six independent sites with the same idiom defect, each replaceable in isolation.

### (d) yagni without concrete construct — N/A

Tag is `idiom`, and both construct and replacement are concrete.

## Decision guard — no hit

Grepped `docs/superpowers/specs/*.md` (8 memos), `docs/IDEAS.md`, `docs/ROADMAP.md` for `BTreeMap`, `prop_planner`, map-literal/insert-chain wording. Zero hits. Read ROADMAP's cosmetic-cleanup group K and the test-hygiene collection (docs-tree B-minors) in full: both enumerate concrete items, none concerning prop_planner.rs or map-construction style. Not tracked, no recorded decision contradicted.

## Conclusion

The pattern exists as claimed, the replacement is the documented stdlib constructor for fixed entry lists on the pinned toolchain, and nothing in the decision record covers or contradicts it. CONFIRMED.
