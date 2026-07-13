# Seed 5 [T14-m3]: test-side logic mirrors in property tests

**Verdict: CONFIRMED** (tag: dup)

## Current state at HEAD (2f17880)

The mirror block still exists, unchanged, in
`crates/muxsmith-core/tests/prop_planner.rs` (section comment line 304). Three
test-side helpers re-derive private production logic from
`crates/muxsmith-core/src/planner.rs`:

| Test helper (prop_planner.rs) | Production original (planner.rs) | Relationship |
|---|---|---|
| `apply_suggestion` (315-337) | `with_rule_match` (1796-1815) | splice body **verbatim** copy; only the YAML-fragment parse front-end is genuinely new |
| `diag_sig` (342-358) | `diag_signature` (1952-1969) | **deliberate variant**: basename-scoped (tempdir comparability), set instead of multiset; documented |
| `rule_index_of` (360-364) | `rule_index_of` (1971-1975) | **verbatim** 5-line copy |

Consumer: the D6 property `accepted_suggestion_survives_replan`
(prop_planner.rs:440), which applies each emitted `yaml_fragment` and replans.

## Assessment

- The verbatim copies buy no oracle independence: a copy-pasted splice
  replicates any production bug by construction. The genuinely independent
  behavioral oracle already exists in
  `tests/suggestions.rs::with_rule_match_never_widens_an_existing_substring_constraint`
  (hand-built edited YAML, public-API-only, asserts resolution outcomes), so the
  mirrors' only remaining value is letting the integration test compile against
  a private function - at the cost of a manually synced 20-line copy
  ("mirrors planner::with_rule_match" comment is the sync obligation).
- `diag_sig` is NOT part of the finding: its divergence (basename scoping,
  set-valued) is load-bearing for tempdir-based property runs and documented.
  Reusing production `diag_signature` would break the test. Keep.
- The 3-line inline `still_ambiguous` check (462-464) mirrors half of
  `resolves_without_regression`; too small to act on.

## Replacement

Expose the two verbatim helpers to integration tests and delete the copies.
`muxsmith-core` is `publish = false`, so visibility cost is nil:

1. In `planner.rs`: mark `with_rule_match` and `rule_index_of`
   `#[doc(hidden)] pub` (or re-export via a `#[doc(hidden)] pub mod test_internals`),
   one-line doc comment "exposed for property tests".
2. In `prop_planner.rs`: `apply_suggestion` shrinks to parse-fragment +
   `planner::with_rule_match(profile, ri, &delta)`; delete local `rule_index_of`
   and import the production one.
3. Keep `diag_sig` as the deliberate basename-scoped variant.

The property loses nothing: its independent content (fragment round-trip,
replan-from-scratch, no-new-diagnostic comparison) survives; splice semantics
stay independently covered by the suggestions.rs behavioral test.

**Estimates:** lines_cut ~16 net (~20 test lines removed, ~4 exposure lines
added), deps_cut 0.

**Severity note for ranking:** low. The duplication is deliberate, documented,
and drift is mostly self-announcing (a stale mirror makes the property fail
loudly, not pass silently). The counterargument - integration tests should stay
black-box and not pierce crate privacy - is real; if the merge stage prefers
the black-box doctrine over deduplication, rejecting this item is defensible.
The copies-buy-no-independence argument is why it stays CONFIRMED rather than
blessed.
