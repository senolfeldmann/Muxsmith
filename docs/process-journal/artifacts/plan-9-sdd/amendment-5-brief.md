# Amendment 5 - author brief (design half + plan half)

**Role:** fresh author of Plan 9's amendment 5. You write BOTH halves - the
design rider and the plan's one-sentence qualifier - because this amendment adds,
removes and re-cuts no task; it changes the stated observable of one assertion
inside an existing task's step. Model tier: mid (dispatch model: Opus 5). Effort:
xhigh. An independent reviewer grades both halves.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. Absolute paths, **foreground runs only**.
- **You are NOT the only content in this working tree.** Task 6 is mid-flight and
  its finished work sits uncommitted: six modified files under `e2e/` plus the
  untracked `e2e/jobsview-reset.spec.ts`. Do not touch, stage, revert or
  `git add -A` any of it. Two writers in one tree share one git INDEX, so stage
  and commit with an explicit pathspec only:
  `git -c commit.gpgsign=false commit -- docs/... docs/...`
  (ledger `concurrent-writers-need-pathspec-scoped-commits`). Verify with
  `git show --stat` that your commit contains ONLY your two doc files, and that
  `git status --porcelain` still shows the seven `e2e/` entries afterwards.
- A bare `cp` here is aliased interactive; if you mutate anything, restore
  non-interactively and prove it.

## The fork, already routed and already decided

Task 6 returned NEEDS_CONTEXT on D104's item 2, "fresh dispatch rejection
renders the error and clears runActive". Its FIRST assertion (the alert renders
`run-already-active`) passes. Its SECOND - `cancel-batch` is disabled - cannot
execute, and the controller verified this independently at the source before
dispatching you:

- `cancel-batch` (`src/views/JobsView.vue:263`) lives inside
  `<template v-if="jobs.length > 0 || runActive || finishedSummary">`
  (`:258`).
- A FRESH dispatch (`startingFresh = !runActive.value`) resets `jobs = []`,
  `logLines = []`, `finishedSummary = null` before the invoke (`:177-182`).
- On rejection the catch arm sets `startError` and, because the dispatch was
  fresh, `runActive = false` (`:193-196`); `ensureJobsLength` is never reached.
- So all three disjuncts are false in every end state of that scenario and the
  button is not rendered. "Disabled" has no bearer.

**The adjudicated premise is confirmed, not contradicted.** The button's absence
IS the consequence of `runActive` going back to false. Task 6 measured that:
mutating the catch arm to skip the reset makes the button appear (1 element
found where 0 was expected). D104's own wording carries the tell - "disabled
*again*" - because at mount the button does not exist either.

**Decision, taken by the controller and recorded here (not an escalation).** The
options were weighed and three of four are excluded by statements already on
record: changing the view is excluded by D104's opening "No code fix" and by the
task's scope; dropping the assertion is excluded by
`tests-ship-with-the-feature-never-after` and `proc-proposed-safeguard-stays`
(the safeguard was measured NOT redundant); re-shaping the scenario is
impossible, since the fresh branch's reset runs unconditionally before the
invoke. None of the four contested criteria in the deliberation tiers holds, so
this is the controller's call, and it is:

> **Item 2's second assertion asserts the control's ABSENCE, paired with the
> positive bearer.** Concretely, in place of `toBeDisabled()`:
>
> ```ts
> await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
> await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
> ```
>
> `jobs-empty` (`src/views/JobsView.vue:328`) is the `v-else` of the very same
> condition, so with `jobs` empty and `finishedSummary` null its visibility is
> logically equivalent to `runActive === false` - the proposition item 2 exists
> to assert. The pairing is load-bearing: `toHaveCount(0)` alone would also pass
> against a view that never mounted, and the visible placeholder is what rules
> that out.

You are not re-deciding this. You are writing it down correctly, in the two
documents that carry it.

## What to write

### Half 1: the design rider

`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`.

Add an amendment-5 rider to **D104**, in the shape the file already uses for
amendment riders (read amendment 3's rider under D96 and the `## Amendment log`
entries first, and follow that form - this brief does not restate it). It
records:

1. What item 2's stated vehicle was and that it cannot execute, with the render
   condition and the fresh-branch reset as the reason.
2. That the adjudicated behaviour is unchanged and confirmed - `runActive` is
   cleared - and that the button's absence is that clearing's direct
   consequence.
3. The replacement assertion pair verbatim, and why the pairing (not the count
   alone) is the assertion.
4. The measured fact that closes the obvious alternative:
   `expect(locator).not.toBeEnabled()` does NOT pass on a detached element - it
   reports "element(s) not found" and times out exactly like the positive form -
   so no wording-preserving fix exists. Task 6 measured this; say that it was
   measured.
5. That items 1 and 3 are untouched: their scenarios leave the block rendered,
   so the disabled-state vehicle remains correct there.

Append the matching line to the `## Amendment log` at its current state.

### Half 2: the plan qualifier

`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`.

Task 6's Step 2 closes with: "the internal transitions are asserted through the
cancel-batch button's disabled state, as D104 fixes." That sentence stays
literally true for items 1 and 3 and is false for item 2. Give it the qualifier,
minimally, pointing at the rider rather than restating it, and add the
amendment-5 line to the plan's own amendment log in the form the previous four
use.

Change nothing else in either document. Task 6's Files list, its other steps,
its "Must not decide" block and every other task stay exactly as they are.

## Standing rules

- **No design latitude**, in either form. A fork you find returns as
  NEEDS_CONTEXT with a decision memo; it is not resolved at the keyboard.
- **Verify the brief's premises against the tree.** Every line number and quoted
  condition above is the controller's measurement; refuting one is a valid
  completion, not a failed task (`proc-57-briefs-not-ground-truth`).
- **No task or amendment edits any house-knowledge YAML.** Surface
  ledger-worthy observations in your report; the controller is the single writer.
- Counts recomputed from their enumerations; **typography** ASCII hyphens,
  straight quotes, no Unicode ellipsis.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push. Pathspec-scoped as the
preamble requires, `git -c commit.gpgsign=false`, exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line. One commit for both halves is correct here; say in the message that this
is amendment 5 and what it changes.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/amendment-5-report.md`, same
content as your final message: what each half now says, the premises you
verified and any you refuted, divergences and judgment calls, numbered concerns
a reviewer can rule on yes/no, the commit hash with `git show --stat`, and the
proof that the seven `e2e/` entries are still uncommitted and untouched.
