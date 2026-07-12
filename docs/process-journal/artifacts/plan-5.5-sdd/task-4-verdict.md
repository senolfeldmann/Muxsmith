# Task 4 reviewer verdict (model: opus, 2026-07-11)

Diff: 3353028..a4ab647 on plan55-stream-a (review-3353028..a4ab647.diff)

## Spec Compliance
✅ all mandated points: Failed + worker-panicked (not Cancelled), RED
proves old mislabel; join Err captured, remaining jobs complete, final
collection never panics; poison recovery centralized behind
killers()/lock_outcomes()/lock_active() getters with soundness docs;
payload downcast flows ONLY into eprintln (never errors/outcome/JSON,
grep-verified); catalog key EN-only; C1 satisfied at the
catalog-completeness level now, render fixture deferred to stream-D per
plan; TDD meaningful (index-keyed scripting, interleaving-independent).

## Concurrency verification (named risks, all clean)
(a) soundness holds at EVERY site: panic originates in run_job with no
guarded lock held; critical sections are single map-ops that cannot
unwind mid-operation; backfill slot claimed once via fetch_add, dead
worker died before its own write. (b) payload containment verified.
(c) no double-record possible: monotonic claiming + distinct slot under
poison-safe lock; test pins outcomes[0]=Failed/[1][2]=Ok.

## Concern judgments
1. eprintln in core: precedent (delete_partial_failed) only partially
   analogous (data channel vs I/O side effect); FIRST direct stderr I/O
   in core. Defensible interim (brief asked for a log line, no facade
   exists, payload stays off user surfaces). IMPORTANT non-blocking.
2. Scope expansion (5+8 lock sites): justified thoroughness, uniform
   idiom, test-only sites deliberately left bare (poisoned test lock
   should fail loudly).
3. duration_ms:0 not load-bearing (verified all consumers; Cancelled
   already uses the same convention).
4. Literal spec met (DiagCode + ftl exist), but the rich EN message
   renders on NO live surface: CLI-human shows generic run-job-failed,
   GUI generic Failed chip; only JSON carries the token. Matches the
   pre-existing raw-errors design, not a regression. IMPORTANT
   observation for the whole-branch review.

## Issues
Critical: none.
Important (non-blocking, tracked):
- I1: eprintln-in-core -> idiomatic fix is a log/tracing facade the
  binaries route; in bundled GUI stderr is invisible today.
- I2: worker-panicked rich message unreachable from live surfaces; fix =
  routing JobOutcome.errors codes through the diagnostics catalog
  (cross-cutting, whole-branch item).
Minor:
- m1: lock_active doc over-claims "single assignment" for the
  abort/cancel arms (recovery still sound).
- m2: child-process leak on a panic AFTER successful spawn (killer
  removed without invoking; Child not killed on drop). Invoke the
  idempotent killer before removing. Edge-of-edge.

## Assessment
Spec compliance ✅. Task quality: Approved.
