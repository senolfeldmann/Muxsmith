### Task 3: Property tests (Stream B, after T2)

**Files:**
- Modify: `crates/muxsmith-core/tests/prop_planner.rs`, `crates/muxsmith-core/tests/prop_matcher.rs`, `crates/muxsmith-core/tests/prop_language.rs`, `crates/muxsmith-core/src/planner.rs` (two `#[doc(hidden)] pub` markers only)

**Interfaces:** consumes T2's merged planner.rs state.

- [ ] `prop_planner.rs:42` etc. **idiom** - six builder sites (42, 51, 106, 176, 245, 281) become `BTreeMap::from([(k, v), ...])`; the mut-binding statement chains go.
- [ ] `prop_planner.rs:144, :156` **idiom** - `select(TYPE_VALUES)` directly (proptest 1.11.0's select takes `impl Into<Cow<'static, [T]>>`, verified in vendored source; TYPE_VALUES is `&'static [&str]`).
- [ ] `prop_planner.rs:450-454` **idiom (seed T14-m1)** - both prop_assume! guards become prop_assert! with condition-specific messages ("generator no longer yields an ambiguous batch" / "ambiguous batch yielded no suggestions"); replace the stale "Meaningfulness guard" comment (448-449) with one stating these are by-construction invariants. Full snippet: `docs/process-journal/artifacts/idiomacy-review-sdd/seed-4.md`.
- [ ] `prop_planner.rs:315-337, :360-364` **dup (seed T14-m3)** - mark planner.rs `with_rule_match` (:1796) and `rule_index_of` (:1971) `#[doc(hidden)] pub` (crate is publish=false); shrink apply_suggestion to fragment-parse + `planner::with_rule_match`; delete the local rule_index_of copy; keep diag_sig (deliberate documented variant). Splice semantics stay independently covered by suggestions.rs::with_rule_match_never_widens_an_existing_substring_constraint.
- [ ] `prop_matcher.rs:83-107` **stdlib** - exact_one/substring_one/regex_one one-entry maps via `BTreeMap::from`, inlined into the struct literals.
- [ ] `prop_matcher.rs:179` **stdlib** - `properties: entries.into_iter().collect(),` (FromIterator).
- [ ] `prop_matcher.rs:130` etc. **idiom** - all 14 `select(CONST.to_vec())` sites drop `.to_vec()`; the `select(vec![...])` literal sites elsewhere stay (documented proptest form when no static slice exists).
- [ ] `prop_language.rs:128-130, :139-141` **stdlib** - one-entry maps via `BTreeMap::from`, inlined.
- [ ] Full gate; commit `refactor(tests): property-test idiom cleanup (T14 seeds + select/BTreeMap sweeps)`.

