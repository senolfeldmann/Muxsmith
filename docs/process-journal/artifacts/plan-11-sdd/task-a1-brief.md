# Task A1 implementer brief - Plan 11, stream A

**Role:** fresh implementer for Plan 11, Task A1 (W5: `BUILDING.md` loses its
three positional gate ordinals and its one over-80 prose line, in one edit).
Model tier: mid (dispatch model: Opus 5). Effort: xhigh. An independent
reviewer grades your work afterwards; the controller re-runs your claims.

You are the FIRST task of stream A. Three further tasks (A2, A3, A4) follow
you serially in this same worktree; a separate stream B runs concurrently in a
DIFFERENT worktree and never touches your files.

## Preamble (binding)

- **Work in `/home/senol/Git/muxsmith-plan11-a`** (branch `plan-11-stream-a`).
  Never on `master`, never in the main worktree. Absolute paths throughout.
- **Never call session-relocation tools** (EnterWorktree/ExitWorktree or any
  equivalent). Do not run `git worktree` at all - the controller owns worktree
  lifecycle.
- **Foreground runs only.** No background-run-plus-monitor pattern.
- You are the only writer in your worktree while you run.
- **Read the files, not a commit hash.** Grade and edit the current tree.
- Shell hazard this project has hit twice: a bare `cp` is aliased interactive
  here and blocks on overwrite, leaving a mutated tree behind a hung command.
  Your Step-3 fire mutates `BUILDING.md` and must restore it. Take the baseline
  BEFORE mutating, restore with `git checkout -- BUILDING.md`, and PROVE the
  restoration in the diff check the step prescribes.

  > **ERRATUM, controller, 2026-07-30, written after the implementer refuted
  > this instruction and before the review was dispatched.** The prescribed
  > restore mechanism is WRONG and the brief, not the implementer, carries the
  > defect. `git checkout -- BUILDING.md` restores from HEAD, and at Step 3 the
  > Step-2 replacements are still uncommitted (the commit is Step 5), so the
  > instruction would have silently discarded the deliverable. It also
  > contradicts the plan's own Step 3, which requires the post-restore diff to
  > still show both edited regions. The correct baseline for a fire at that
  > point is the EDITED file's own content, captured before the mutation and
  > restored to it afterwards. This erratum corrects the ground truth; how the
  > implementer's substitute mechanism is graded is the reviewer's call, not
  > something this note settles.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash - in the file you edit and in your report.

## What to read first

1. The plan,
   `/home/senol/Git/muxsmith-plan11-a/docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`:
   the **Global Constraints** section in full, **Task A1** in full (its Read-first
   list, its EXHAUSTIVE Files list, Steps 1 through 5, and its "Must not decide"
   line), and acceptance rows **W5-a through W5-e** in the acceptance map, which
   are what your evidence has to satisfy. Also the **Authoring-time verification**
   section's block "Item 5's corpus: three ordinals and one long line, all
   reproduced" - those are the measurements you re-run.
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 5.
3. `docs/ROADMAP.md`, the section **"Gate-count derivation has no check"** in
   full, including its MEASURED block, its NARROWED FORM block, its DONE block
   and the "A neighbouring class" paragraph that routes this item.
4. **`BUILDING.md` in full** - it is both the input and the subject of the edit.
   The four commands your replacement text names must be read out of its own
   Rust gate block, not taken from the plan.
5. `scripts/ledger-lint.py` in full - Step 3's fire has to know which lines the
   check reads.
6. Tier-2 entries in `docs/process-conventions.yaml`:
   `ledger-lint-runs-before-every-push`, `proc-wrapped-prose-quote-grep`,
   `gate-includes-cross-target-lint-for-the-unrun-os` (the statement Step 4
   surfaces), `proc-verification-step-must-be-falsifiable`,
   `proc-check-green-state-reachable`.

## The one thing that decides this task

Both fenced replacements are **verbatim**. If your Step-1 re-measurement returns
a set different from the authoring run's three ordinal lines and one over-80
line, **that set is the ground truth**, you say so in the report, and you
re-check the fenced replacements against the text actually present before
applying either. A hit the plan does not fence returns as **NEEDS_CONTEXT**,
never rewritten at the keyboard. The same applies if your own reading of the
Rust gate block finds a different first four commands: NEEDS_CONTEXT with both
readings pasted, not an adjusted fence.

## Exit bar before you commit

The five checks of Step 3, every output pasted: absence check O with its fire
and its soundness control; absence check L with its fire and its
threshold-lowered control; `python3 scripts/ledger-lint.py` green AND made to
fire and restored; the `git diff -U0 -- BUILDING.md` scope check in both states
(dirty while the fire is live, clean after the restore); the weighed test duty.

You do **not** run the full gate - the stream runs it once before merge, and
that is the controller's dispatch, not yours.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner for this repo; you do not ask.
Agent commits are deliberately UNSIGNED. Use exactly the fenced commands in
Step 5, and the trailer

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

exactly one trailer, no `Claude-Session` line, no context-window suffix. Stage
explicitly; never `git add -A`. Do not push - the controller pushes once, at the
plan close.

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a1-report.md`
(note: the MAIN repo path, not your worktree - the scratch is shared and
git-ignored). It carries: every command you ran with its pasted output, the
Step-4 surfacing list with quoted clauses, your commit SHA, and anything you
noticed but did not touch.

Return to the controller only: status
(`DONE` / `DONE_WITH_CONCERNS` / `NEEDS_CONTEXT` / `BLOCKED`), the commit SHA,
a one-line verification summary, and concerns. Not the report body.
