# Verify-26: prop_matcher.rs one-entry BTreeMap hand-roll

**Finding:** `crates/muxsmith-core/tests/prop_matcher.rs:84` (tag: stdlib) - `exact_one` / `substring_one` / `regex_one` (lines 83-107) each hand-roll a one-entry map via `let mut map = BTreeMap::new(); map.insert(...)`. Replacement: `BTreeMap::from([(prop.to_string(), val)])` inlined into the struct literal.

**Verdict: CONFIRMED**

## Checks

### (a) Code says what the finding claims - YES

Read at HEAD, lines 83-107: all three constructors use the two-line `let mut map` + `insert` pattern followed by a struct literal wrapping `Some(map)`. Exactly as described. The file-local comment at line 81 ("struct-update to dodge field_reassign_with_default") justifies only the `..Default::default()` struct-update pattern, which the replacement preserves untouched - it is not a recorded rationale for the map construction style.

### (b) Replacement is current idiom for the pinned toolchain - YES

- **Types check:** `MatchExpr` fields are `exact: Option<BTreeMap<String, Scalar>>`, `substring: Option<BTreeMap<String, String>>`, `regex: Option<BTreeMap<String, String>>` (src/profile/match_expr.rs:51-72). The single-element array form type-checks for all three.
- **Compile probe on the pinned toolchain:** a stand-in replicating the exact replacement shape (all three constructors, `BTreeMap::from([...])` inlined into the struct literal with `..Default::default()`) compiles and runs under `rustc 1.96.1 (31fca3adb 2026-06-26)` with `--edition 2024`. Output: `ok`.
- **Official docs (doc.rust-lang.org, fetched 2026-07-12):** `impl<K: Ord, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V>` is stable since **1.56.0**, and the BTreeMap page's own construction example uses `BTreeMap::from([...])`. This is the documented canonical construction for a literal map, not a fringe alternative.
- **lines_cut = 6** is accurate: 2 lines removed per constructor x 3.

### (c) Duplication with load-bearing difference - N/A

No duplication claim in this finding.

### (d) yagni without concrete construct - N/A

Tag is `stdlib`; construct and replacement are both concrete.

## Decision guard - no hit

- `grep prop_matcher` and `grep 'BTreeMap'` across `docs/superpowers/specs/*.md` (D1-D35 memos), `docs/IDEAS.md`, `docs/ROADMAP.md`: zero hits for this file/construct.
- ROADMAP "Whole-codebase idiomacy review" entry is the process producing this finding; its NAMED INPUTS mention prop_matcher only for unrelated items (T14-m1 prop_assume->prop_assert, T14-m3 test-side logic mirrors), not this construct.
- Cosmetic-cleanup group K, test-hygiene collection (B-minors), and v1.x candidates list unrelated items only.

Not tracked anywhere; contradicts no recorded decision.
