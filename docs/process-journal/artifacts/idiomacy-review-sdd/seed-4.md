# Seed [T14-m1]: prop_assume -> prop_assert in D6 suggestion property test

**Verdict: CONFIRMED**

- **File:** `crates/muxsmith-core/tests/prop_planner.rs`
- **Line:** 450 (both guards: 450-453 and 454)
- **Tag:** idiom
- **Origin:** Plan 5.5 task-14 verdict, Minor #1; deferred to idiomacy review in `whole-branch-verdict.md` line 73.

## What

The D6 property `accepted_suggestion_survives_replan` guards its two preconditions with `prop_assume!`:

```rust
prop_assume!(batch.files.iter().any(|f| f
    .diagnostics
    .iter()
    .any(|d| d.code == DiagCode::AmbiguousRule)));
prop_assume!(!batch.suggestions.is_empty());
```

Both conditions are guaranteed by construction: the generator `arb_ambiguous_ident()` (lines 266-302) documents "guaranteed ambiguous" (two subtitle tracks vs. `exact: { type: subtitles }`) and "guarantees at least one resolving discriminator exists" (distinct languages via `off in 1..3`). `prop_assume` is proptest's tool for filtering inputs a generator cannot cheaply avoid producing; there is no legitimate rejection here. As written:

- A **total** regression (generator or engine stops producing ambiguity/suggestions) dies opaquely as proptest's "Too many global rejects" abort, with no pointer to which condition failed and no shrunken input.
- A **partial** regression silently skips exactly the regressed cases and the test stays green - silent coverage loss.

`prop_assert!` localizes either failure to the specific condition with a shrunken counterexample.

## Replacement

```rust
// By construction (arb_ambiguous_ident): the scenario is ambiguous and a
// resolving discriminator exists, so suggestions must be emitted. Assert,
// don't assume: a miss here is a generator or engine regression.
prop_assert!(
    batch.files.iter().any(|f| f
        .diagnostics
        .iter()
        .any(|d| d.code == DiagCode::AmbiguousRule)),
    "generator no longer yields an ambiguous batch"
);
prop_assert!(
    !batch.suggestions.is_empty(),
    "ambiguous batch yielded no suggestions"
);
```

Also replace the stale "Meaningfulness guard" comment (lines 448-449), which describes assume semantics.

## Cost

- lines_cut: 0 (like-for-like swap; net +4 lines for assertion messages)
- deps_cut: 0
