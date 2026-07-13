# Verify-9: planner.rs:1971 `rule_index_of` hand-rolls find + index arithmetic (tag: stdlib, slice F1a)

**Verdict: CONFIRMED**

## Finding under test

`rule_index_of` (crates/muxsmith-core/src/planner.rs:1971-1975) hand-rolls `str::find` plus manual index arithmetic to extract the bracketed rule index; `str::split_once` is the stdlib-normal parse. Proposed replacement body:
`config_path.split_once("tracks[")?.1.split_once(']')?.0.parse().ok()`

## Checks

### (a) Cited code says what the finding claims — YES

Read at HEAD (2f17880). The function is exactly the described pattern:

```rust
fn rule_index_of(config_path: &str) -> Option<usize> {
    let start = config_path.find("tracks[")? + "tracks[".len();
    let end = config_path[start..].find(']')? + start;
    config_path[start..end].parse().ok()
}
```

Two `find` calls, `+ len()` / `+ start` offset arithmetic, manual slicing. Matches the claim verbatim.

### (b) Replacement is current idiom for the pinned toolchain — YES

- Verified against current official stable stdlib docs via context7 (`/websites/doc_rust-lang_stable_std`): `str::split_once<P>(&self, delimiter: P) -> Option<(&str, &str)>` is stable, takes any `Pattern` including `&str`, splits on the first occurrence, returns `None` when absent — precisely the "parse between two delimiters" idiom.
- Compiled and ran the replacement on the exact pinned toolchain (rustc 1.96.1, edition 2024). It compiles clean; the `?` chaining works inside the `Option<usize>`-returning function; `parse()` type-infers from the return type.
- Empirical equivalence test: original vs replacement on 17 edge cases (normal paths like `profile.tracks[0].match`, missing `tracks[`, missing `]`, empty index, non-numeric, negative, leading zeros, multiple occurrences, `]` before `[`, usize overflow, embedded whitespace, empty string). **All 17 identical** ("ALL EQUIVALENT"). Both take the first `tracks[` and the first `]` after it, both fail to `None` on any parse failure.

### (c) Duplication with load-bearing difference — N/A

The finding claims no duplication; tag is `stdlib`. (Side observation, outside this finding's scope: a byte-identical test-side mirror of `rule_index_of` exists at tests/prop_planner.rs:360; the ROADMAP idiomacy-review entry already names "test-side logic mirrors (T14-m3)" as a review input, so that facet is covered elsewhere.)

### (d) yagni without concrete construct — N/A

Tag is `stdlib`, and both construct and replacement are concretely named anyway.

### Decision guard — no conflict, not separately tracked

- Grepped `docs/superpowers/specs/*.md` (D-memos), `docs/IDEAS.md`, `docs/ROADMAP.md` for `rule_index_of`, `split_once`, `config_path` parsing: no decision covers this helper's implementation.
- The ROADMAP "Whole-codebase idiomacy review" entry (lines 161-201) is the charter of this very review pass; its NAMED INPUTS list does not contain this construct. A finding produced by the tracked review is not itself "already tracked".
- The `config_path` mentions in the D-memos (2026-07-08 v1 design §diagnostic shape, plan-2 grouping) define the path format `tracks[N]`; the replacement preserves that parse contract exactly.

## Metadata nit (not refutation-relevant)

`lines_cut: 1` is conservative: the 3-line body collapses to 1 expression, so the real cut is 2 lines (or 1-2 after rustfmt wrapping). Direction of error is harmless.

## Conclusion

The finding is accurate on all four refutation axes and conflicts with no recorded decision. `split_once` chaining is the documented, stable, current-toolchain idiom for this parse, and the replacement is behaviorally identical to the hand-rolled original across all tested edge cases.
