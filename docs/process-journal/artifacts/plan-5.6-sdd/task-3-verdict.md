# Task 3 review verdict: property tests (Stream B)

**Base:** 89f346b **Head:** cc49337

## Spec Compliance

- ✅ `prop_planner.rs` six builder sites (`exact_one`, `substring_one`, `video_track`, `arb_track` closure, `arb_nonvideo_track` closure, `arb_ambiguous_ident`'s `sub` closure) -> `BTreeMap::from([...])` literals, mut-binding chains removed. Verified key/value pairs preserved 1:1 against the deleted code in the diff.
- ✅ `prop_planner.rs` `select(TYPE_VALUES.to_vec())` -> `select(TYPE_VALUES)` at both sites (`arb_plan_expr`, `arb_track`). Domain-preservation verified: proptest's `select` takes `impl Into<Cow<'static,[T]>>`; a `&'static [&str]` converts via `Cow::Borrowed`, a `.to_vec()` via `Cow::Owned` - same element set, `Select`'s uniform-pick semantics unaffected by which `Cow` variant backs it.
- ✅ Seed T14-m1: both `prop_assume!` -> `prop_assert!` conversions in `accepted_suggestion_survives_replan`, checked line-for-line against `docs/process-journal/artifacts/idiomacy-review-sdd/seed-4.md` - verbatim match (messages, comment replacement, structure).
- ✅ Seed T14-m3: `with_rule_match`/`rule_index_of` marked `#[doc(hidden)] pub` in `planner.rs:1777-1778,1956-1957` (nothing else touched there, diffstat 13 lines confirms). `apply_suggestion` shrunk to fragment-parse + `with_rule_match(profile, ri, &doc.match_expr)` delegate. Local `rule_index_of` copy deleted (confirmed 0 occurrences of a local `fn rule_index_of` remain in `prop_planner.rs`; both call sites now resolve through the newly-imported `planner::rule_index_of`). `diag_sig` untouched, as mandated.
  **Splice-semantics divergence risk (the task's named central risk) checked directly**: the diff's hunk for `with_rule_match` is cut off mid-function (context window ends before the `not`-handling line and the `p` return). Read `planner.rs:1778-1797` in full to close the gap - the `exact`/`substring`/`not` handling and return are byte-identical to the deleted `apply_suggestion` copy shown in full in the diff. No divergence; the delegate is a faithful reuse, not a behavior change.
- ✅ `prop_matcher.rs:83-107` - `exact_one`/`substring_one`/`regex_one` one-entry maps inlined via `BTreeMap::from`.
- ✅ `prop_matcher.rs:179` - `arb_track`'s loop -> `entries.into_iter().collect()`. `BTreeMap`'s `FromIterator`/`Extend` semantics (later value wins on a duplicate key, insertion order does not affect final content since a `BTreeMap` is always key-sorted) are identical to the deleted `for (k,v) in entries { properties.insert(k,v); }` loop, including the duplicate-key case `arb_prop_entry`'s `0..6` vec can produce.
- ✅ `prop_matcher.rs` 14x `select(CONST.to_vec())` -> `select(CONST)`. Recounted directly against the checked-out file: `grep -c '\.to_vec()' prop_matcher.rs` = 0 (post), matches the claimed 14 sites all converted; the `select(vec![...])` literal sites (no static backing slice) correctly left untouched.
- ✅ `prop_language.rs:128-130,139-141` - `track_with_language`/`language_expr` one-entry maps inlined via `BTreeMap::from`.

Scope discipline confirmed: diffstat touches exactly the four owned files (`planner.rs`, `prop_language.rs`, `prop_matcher.rs`, `prop_planner.rs`), nothing else. Commit message in the diff header matches the mandated string exactly.

## Strengths

- The one risk this review exists to catch - `apply_suggestion`'s delegate diverging from the deleted copy's splice semantics - does not materialize; verified by reading the full (diff-truncated) function body, not just trusting the report's assertion.
- `select()` Cow-conversion reasoning is correct and the domain is provably unchanged, not just asserted.
- Duplicate-key `FromIterator` equivalence for item 6 is the kind of subtle correctness question a lazier pass would have waved through; the report's own reasoning here is verifiably correct.
- `pub` (not `pub(crate)`) is the only visibility that works for this reuse mechanism, since `tests/*.rs` integration tests compile as a separate crate and cannot see `pub(crate)` items - the brief's mandated visibility is technically necessary, not just conventional.
- Zero scope creep: exactly the two markers in `planner.rs`, nothing else in production code.

## Issues

### Critical (Must Fix)

None.

### Important (Should Fix)

None. No narrowed generator domain, no weakened oracle beyond the plan-mandated T14-m1 strengthening (which is a strengthening, not a defect).

### Minor (Nice to Have)

- **Imprecise house-pattern citation.** The report attributes the `#[doc(hidden)] pub` reuse to the `testing-support-helpers` convention (`docs/conventions.yaml:450`, which is about consolidating cross-file helpers into `tests/support/mod.rs`). The actual closer precedent already in the codebase is `crates/muxsmith-core/src/executor/spawn.rs:257`'s `ConcurrencyTracker` (`#[doc(hidden)] pub ... /// Test instrumentation only, not a supported API: hidden from rustdoc ... kept pub because cross-crate tests consume it.`) - the identical mechanism (hide-from-docs + pub for integration-test consumption), not the module-consolidation pattern. Neither `conventions.yaml` nor `process-conventions.yaml` currently tracks "`#[doc(hidden)] pub` for integration-test-only crate-internal reuse" as its own pattern id; this task is a second, uncredited occurrence of an unregistered pattern. Not a code defect - worth a follow-up ledger entry (new pattern id or an occurrence added under an existing one) rather than leaving it implicit.
- **Comment-wording drift.** The new `planner.rs` comments justify the marker via "(crate is publish=false)"; `spawn.rs`'s existing comment for the same mechanism uses different phrasing ("pre-go-public decision" + "cross-crate tests consume it"). Two prose variants of the same rule; harmless, but a shared one-line convention comment would read as more deliberate than independently reinvented wording.

## House dimension

Ran the `house` dimension against `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`.

- **Matched pattern**: `testing-support-helpers` (`conventions.yaml:450`) - cited by the report, genuinely applicable in spirit (dedup a test-code duplicate against a single source of truth) even if the closer literal precedent (`spawn.rs` `ConcurrencyTracker`) went uncited.
- **No deviation found.** `#[doc(hidden)] pub` for integration-test reuse of a crate-internal function already has one precedent (`spawn.rs:257`); this task is consistent with it, not a new departure.
- **Ledger gap (pre-existing, not this task's fault, but this task was an opportunity to close it)**: no pattern id in `conventions.yaml`/`process-conventions.yaml` names "`#[doc(hidden)] pub` for cross-crate-test-only reuse" explicitly; both occurrences (`spawn.rs`, now `planner.rs`) sit uncredited in the ledger. Worth raising to the controller for a ledger update, not a task defect.

## Assessment

**Task quality:** Approved

**Reasoning:** All 8 brief items verified against the diff (and, for the one item with a diff-truncated hunk, against the full file) with no narrowed generator domain and no weakened oracle beyond the plan's own sanctioned T14-m1 strengthening. The two Minor findings are house-ledger bookkeeping, not code defects.
