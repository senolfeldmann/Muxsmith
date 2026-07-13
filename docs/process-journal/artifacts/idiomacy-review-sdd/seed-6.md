# Seed [T13-m1] - partition best=None invariant comment

**Verdict: CONFIRMED**

- **File:** `crates/muxsmith-core/src/planner.rs`
- **Line:** 1521 (`if let Some(cand) = best {` in `partition_for_rule`)
- **Tag:** doc

## Current state at HEAD

`partition_for_rule` (lines 1482-1549) groups affected files by their top-ranked per-file resolving candidate. The per-file loop (1509-1527) computes `best: Option<&Candidate>`; a file whose `best` stays `None` is silently dropped from the partition - the `if let Some(cand) = best` at line 1521 has no else branch and no comment explaining the skip. Nothing at the site records that the case is unreachable under v1 id-uniqueness or that the drop is deliberate defensive behavior.

Contrast: the sibling `partition_for_overlap` has its emits-nothing (`best=None -> Vec::new()`) case fully documented in the function comment (lines 1559-1564). Only the per-rule site lacks the invariant note.

Original finding context (task-13-verdict.md Minor 1, task-13-report.md point 3): `id` is unique per track, so the id discriminator always resolves a single file in isolation; a "no per-file fix" file therefore cannot occur in v1, and the code skips it defensively. The DEFER decision (whole-branch-verdict.md, T13-m1 row) asked for an invariant comment or an "unresolvable" group at exactly this skip.

## Replacement

Add an invariant comment above line 1521 (the comment-only option; the "unresolvable" group alternative would add a new report shape for an unreachable case, which the scale has not earned):

```rust
// `best` is `None` only for a file no candidate resolves even in
// isolation - unreachable in v1: `id` is unique per track, so the id
// discriminator always resolves a single file (task-13 report, D6).
// Skipped defensively rather than fabricating a group without a fix;
// if id-uniqueness ever relaxes, this needs an "unresolvable" group.
if let Some(cand) = best {
```

- **lines_cut:** -5 (pure addition, no code removed)
- **deps_cut:** 0

## Reason

Still applies at HEAD: no invariant comment exists at the `best=None` skip in `partition_for_rule`; grep for `best` in planner.rs shows the site unchanged since the T13 merge, and the documented counterpart in `partition_for_overlap` proves the codebase's own convention is to state the emits-nothing invariant at the site.
