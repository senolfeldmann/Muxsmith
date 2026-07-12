# Task 6 reviewer verdict (model: sonnet, 2026-07-11)

Diff: d9db161..c09875e, fix a60e9a0 (review-d9db161..a60e9a0.diff)

## Round 1: Needs fixes
Core predicate, D20 handling, batch-visibility plumbing correct and
source-verified (print_batch_human iterates diagnostics unconditionally;
json batch_document serializes the same vec; validate's counter
config-time-only and run's summary JobState-based - both correctly
orthogonal). Error-severity guard judged CORRECT in intent (without it the
message is factually false for finalize-dropped plans) but structurally
unreachable for cross-file passes (push in resolve_file predates them).
IMPORTANT 1: brief's "batch-report test" deliverable silently dropped.
IMPORTANT 2: relocate the check post-finalize_plans gated on
f.plan.is_some() (closes the cross-file gap, subsumes the ad-hoc scan).

## Fix wave (a60e9a0)
Verified: relocation into detect_empty_plans after BOTH finalize passes;
old push site fully removed; f.plan.is_some() a strict superset of the old
guard AND closes a THIRD unenumerated case (CollisionPolicy::Skip nulls
the plan with only a warning - the old guard could never catch the false
EmptyPlan there). Predicate equivalence exact. New CLI test verified
load-bearing (exit 1 genuinely driven by the warning; kebab-case code
enforced by existing serde test; fixture produces no other diagnostic).
Planner tests byte-identical. Doc comment now lifetime-accurate.
Behavioral ripple (recorded, beneficial): suggest() simulations now
include detect_empty_plans, so resolves_without_regression rejects
candidates that would newly produce an empty plan - conservative-correct.

## SI-3 (carried to memo)
mkvtoolnix-gui has no general zero-selected-tracks warning; deliberate
divergence recorded in the plan-5.5 memo.

## Issues (residual)
Minor: no dedicated attachment/chapters-only zero-track test (predicate
provably ignores them; documentation-value only).

## Final Assessment
Spec compliance ✅ (round 2). Task quality: Approved.
