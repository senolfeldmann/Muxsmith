# Verify-22: planner_resolution.rs:619 — `unwrap_or(&String::new())` emptiness test

**Verdict: CONFIRMED**

## Finding under test

`!d.params.get("detail").unwrap_or(&String::new()).is_empty()` at
`crates/muxsmith-core/tests/planner_resolution.rs:619`; proposed replacement
`d.params.get("detail").is_some_and(|s| !s.is_empty())`; claims the file already
uses the idiomatic form at line 741.

## Checks

### (a) Cited code says what the finding claims — yes

Line 619 reads verbatim:

```rust
!d.params.get("detail").unwrap_or(&String::new()).is_empty(),
```

inside an `assert!` whose message ("expected a non-empty detail param") confirms the
intended predicate is exactly "present and non-empty". Line 741 reads:

```rust
unsupported.params.get("donor").is_some_and(|d| d.ends_with("Donor.S01E01.srt")),
```

so the same file does use the `Option::is_some_and` form for the same
params-map-lookup-plus-predicate shape. Claim accurate.

One imprecision in the finding's rationale, noted for the record: `String::new()` is
guaranteed non-allocating (empty `String`, no heap buffer), so "allocates a String" is
technically wrong; it constructs a needless temporary but performs no heap allocation.
This weakens one supporting clause, not the finding: the double-negation/buried-predicate
and in-file-inconsistency arguments stand on their own, and this is a test assertion where
the readability argument is the load-bearing one anyway.

### (b) Replacement is current idiom for the pinned toolchain — yes

Verified against the official std docs via context7 (`/websites/doc_rust-lang_stable_std`):
`Option::is_some_and(self, f: impl FnOnce(T) -> bool) -> bool` is a current, stable,
documented method, with the docs' own examples covering exactly this
present-and-satisfies-predicate use. Semantic equivalence verified empirically on the
pinned toolchain (rustc 1.96.1, edition 2024) with a three-case compile-and-run check
(key absent / present-empty / present-non-empty): old and new forms agree in all cases
(absent → false, empty → false, non-empty → true). The closure receives `&String`;
`|s| !s.is_empty()` type-checks as proposed.

### (c) Duplication difference — none

The reference to line 741 is a style precedent, not a dedup claim; the predicate shape
there (`Option<&String>` from `params.get` + boolean predicate) is structurally identical
to what line 619 needs. No load-bearing difference.

### (d) yagni gate — n/a

Tag is `idiom`, and a concrete construct plus concrete replacement are named regardless.

## Decision guard

Grepped `docs/superpowers/specs/*.md` (D-memos, all eight spec files), `docs/IDEAS.md`,
and `docs/ROADMAP.md` (v1.x candidates, cosmetic-cleanup group K at ROADMAP.md:260-267,
test-hygiene collection at ROADMAP.md:307-321, deferred entries) for
`planner_resolution`, `is_some_and`, `unwrap_or(&String::new`, and the detail-param
construct. The only hit is the v1-design spec's note that third-party error text flows
through a `detail` param (i18n policy, unrelated to this assertion's form). No recorded
decision covers or tracks this construct. Not a conflict, not tracked.

## Conclusion

The construct exists as cited, the replacement is current stable idiom (std docs),
behavior-equivalent on rustc 1.96.1 / edition 2024 in all three input cases, consistent
with the form the same file already uses at line 741, and no decision doc claims the
territory. Minor caveat: the "allocates" wording in the finding is imprecise
(`String::new()` does not heap-allocate); the finding holds without it.
