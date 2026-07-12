# Task 5 reviewer verdict (model: opus, 2026-07-11)

Diff: 374005a..d9db161 on plan55-stream-b (review-374005a..d9db161.diff)

## Spec Compliance
✅ on all brief points, verified against code: once-per-batch semantics
(plan_core:242-243; suggest's per-candidate sim batches never merge into
batch_diagnostics, so the warning surfaces once; IdentifyCache memoizes the
spawn); degrade-with-warning satisfied at system level (true mkvmerge
absence dies upstream at dry_run.rs:78/:98 exit 2; the in-plan None path
only covers list_types failing after list_languages succeeded); DiagCode
UnknownExtension warning with $extension+$known EN-only; model.rs:73-75 doc
now true; batch continues; list_types() consumed not reimplemented; TDD
credible.

## Design deviation: judged sound
known_extensions as Identify trait default method + IdentifyCache memo
instead of a plan_core param: zero production edits (a param would break
~20 call sites and collide with parallel streams). Costs named as minors.

## Locator.extensions verdict
Spec §4.6:198 DOES mandate locator-extension validation -> real open
requirement, but materially distinct work (recursive walk like
walk_exact_languages + own tests), correctly out of T5's brief scope.
ROUTED: new plan Task 5.9 (stream B, after T7), including the
model.rs:254-256 false-parity doc fix.

## Issues
Critical: none. Important: none blocking (locator gap routed).
Minor:
1. model.rs:254-256 "validated ... like input.extensions" now actively
   false -> folded into Task 5.9.
2. Trait seam: capability query on Identify is a mild contract stretch;
   sharper point is the default-None inversion (sole production impl
   overrides; a future production impl would silently get vacuous
   validation). Fix: make the method required, FakeIdent returns None
   explicitly. Acceptable at scale; T23 funnel item.

## Assessment
Task quality: Approved.
