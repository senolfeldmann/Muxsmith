# Task 7 implementer brief - Plan 9

**Role:** fresh implementer for Plan 9, Task 7 - the D49 G1/G2 removal
experiment (D105). This task **measures**; it does not change the product.
Model tier: mid (dispatch model: Opus 5). Effort: xhigh. An independent reviewer
grades your work; the controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.**
- A bare `cp` here is aliased interactive and blocks on overwrite, leaving a
  mutated tree behind a hung command. Restore with `git checkout --`, never a
  bare `cp`, and take your byte baseline BEFORE mutating.

## What to read first

1. The design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`):
   **D105 in full**. Its protocol, decision rule and recording are design-fixed;
   this task transcribes and observes.
2. The plan (`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`):
   Global Constraints, then **Task 7** in full - Steps 1-6 and "Must not decide".
3. The four house-knowledge YAMLs as ground truth alongside them.

## Scope, stated as three prohibitions because this task's Files list is empty

- **You modify NO file permanently.** The mutation is applied and fully
  reverted inside the task; the tree ends byte-identical to its start.
- **You commit nothing.** Not the mutation, not a revert, nothing.
- **You write no ledger entry and no repo file.** The
  `core-d49-g1g2-experiment` entry is controller-written at the plan close, from
  your reported measurement, with the design-fixed statement text of whichever
  branch you measured. Your report is the input to that; do not pre-empt it and
  do not compose the entry text.

The only file you create is your report, in the SDD scratch.

## The measurement

Follow the plan's Steps 1-6 exactly. The shape, so you can see where the rigour
has to sit:

- **Step 1 is the green control.** A mutation experiment whose baseline nobody
  ran cannot distinguish "the guard caught it" from "the suite was already red"
  (`proc-check-green-state-reachable`). Paste the pass.
- **Step 2 is the exact mutation D105 fixes**, in `delta_for`'s `AddExact` arm.
  The plan's line numbers date from authoring - locate by the `AddExact` match
  arm, not by number, and re-derive before editing
  (`proc-57-briefs-not-ground-truth`). Nothing else is touched.
- **Step 3 records per guard** - G1, G2, G3, each named in the plan - whether it
  went red, with the pasted output. Per guard, not per suite: an aggregate
  "1 failed" tells the decision rule nothing.
- **Step 4 applies D105's decision rule verbatim.** All three fail -> the guards
  are load-bearing and stay for good. Only G3 fails -> G1/G2 are recorded as
  removal candidates. **ANY other outcome is an anomaly**: no removal in any
  direction, and you return NEEDS_CONTEXT to the controller with the pasted
  runs. Do not reason your way from an unexpected pattern to one of the two
  clean branches; the anomaly branch exists precisely because that reasoning is
  where a measurement turns into a story.
- **Step 5 restores and proves it.** The revert, the suite green again, and
  both `git status --porcelain` and `git diff --stat` empty - each of those two
  is an absence check, so paste their non-empty state from during the mutation
  as the fire. Add a byte proof: `sha256sum` of the touched file taken before
  the mutation, `sha256sum -c` after the restore.
- **Step 6 is the report**, with the three pasted runs, the per-guard table, the
  selected branch, and the reminder about who writes the ledger entry. If the
  only-G3 branch is measured, name ROADMAP trigger "Plan-9 design trigger 4" as
  now live for the owner's future ruling.

In every branch **the guards stay in the tree at this plan's end**. This task
removes nothing.

## Standing rules

- **No design latitude**, in either form. A fork found on code contact returns
  as NEEDS_CONTEXT with a decision memo, never resolved at the keyboard.
- **Report what you measured, not what the design expects.** D105 names two
  clean branches; if the tree gives you a third pattern, that is the finding,
  and it is a valuable one. A measurement that agrees with its own hypothesis
  gets more scrutiny from you, not less.
- Counts recomputed from their enumerations; every observed value pasted from
  its run; **typography** ASCII hyphens, straight quotes, no Unicode ellipsis.
- The frontend-rebuild rule does not bind you: `cargo test` compiles what it
  runs. No `pnpm` leg is required unless something you do reaches `src/` or
  `e2e/`, which it should not.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-7-report.md`, same
content as your final message (read as data): status (DONE /
DONE_WITH_CONCERNS / NEEDS_CONTEXT); the three pasted runs; the per-guard
red/green table; the selected branch of D105's decision rule with the reasoning
from the observed pattern to that branch; the restore proof including the fire
for both absence checks and the byte comparison; anything you surface for the
controller; and an explicit statement that you committed nothing and that the
tree is byte-identical to its start.
