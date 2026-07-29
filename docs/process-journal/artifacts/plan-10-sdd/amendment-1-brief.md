# Plan 10 amendment 1 brief

Controller-authored brief for one amendment to the approved Plan 10. You are the
plan's author, resumed. The plan's original reviewer, resumed, judges the delta.

**Scale, so you do not over-build it:** this ADDS A STEP to an existing task. It
does not add, remove or re-cut a task, so it is a one-pair amendment - you and
the reviewer, no separate design round. The plan is approved and unexecuted; this
is a pre-execution change, not a mid-run one.

## The ruling

The owner ruled on 2026-07-29, after the plan was approved: **the two false
counts in the README's "How this got built" paragraph come into Task 4.** They
had been surfaced by you during authoring and correctly left out, because they
sit outside the ruled Task-4 scope (CLI reference, exact-typed-matching
paragraph, matching-magic anchor). The owner has now widened that scope by
exactly these two items and nothing else.

## What the amendment must do

Add a step to **Task 4** covering both counts. Both are measurable and both must
be MEASURED at execution time rather than transcribed from this brief - the
figures below are the controller's, they were true when written, and the tree
moves:

1. The paragraph states the decision series as running to roughly D35. The
   series reaches far beyond that; derive the current highest decision number
   from the authoritative source rather than from this sentence.
2. The paragraph states a review count around 78. The repo holds far more
   verdict files than that; derive the count from the tree, and state in the
   step WHICH set is being counted, because "reviews" and "verdict files" are
   not obviously the same unit and the sentence has to be true about the unit it
   names.

**The unit question is the substance of this amendment, not a detail.** The
existing sentence went stale in two different ways - one number simply grew, the
other may never have counted what the prose claims. Getting a fresh wrong number
into the README would be worse than the current stale one, because it would look
freshly checked.

## Constraints

- **Nothing else in Task 4 changes**, and no other task is touched. The four
  `placeholder(1.0)` comments and the WIP banner stay out of scope, per the
  owner ruling that put them there.
- The step is written to the same standard as the rest of the plan: what to
  measure, with which command, what the acceptance observable is, and where it
  lands in the acceptance map. If it earns an acceptance row, add it and sweep
  the stated row count with it - the plan currently states nineteen in two
  places.
- The register stays the README's own, per the ROADMAP's recorded sell-tone
  override.
- Write only the plan document. No git commands. Nothing else in the repo.
- Log the amendment where the plan's own structure expects it, matching how this
  repo's earlier plans recorded amendments.

## Report

What you added, the measured figures with the commands that produced them, the
unit you chose for the review count and why, and any count elsewhere in the plan
that moved as a result.
