# Task 1 implementer brief - Plan 10

**Role:** fresh implementer for Plan 10, Task 1 (W3: `BUILDING.md` states the
pre-push gate's total once, canonically, and `scripts/ledger-lint.py` checks
that statement against the commands the file's three marked gate blocks
enumerate). Model tier: mid (dispatch model: Opus 5). Effort: xhigh. An
independent reviewer grades your work afterwards; the controller re-runs your
claims.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only** (no background-run-plus-monitor).
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.** House commits land between dispatches;
  grade and edit the current tree.
- Shell hazard this project has hit twice: a bare `cp` is aliased interactive
  here and blocks on overwrite, leaving a mutated tree behind a hung command.
  Every one of your five fires mutates a file and must restore it. Take the
  baseline BEFORE mutating (`sha256sum`, or `git stash`-free
  `git diff` capture), restore with `git checkout --` or `command cp -f`, and
  PROVE the restoration (`git status --porcelain`, `git diff --stat`).

## What to read first

1. The plan,
   `/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`:
   the **Global Constraints** section, the **Authoring-time verification**
   section (its `BUILDING.md` measurements are your starting point and must be
   re-measured), and **Task 1** in full - the Files list, the "Why the two extra
   regions are in scope" paragraph, Steps 1 through 6, and "Must not decide".
   Also acceptance rows W3-a through W3-f in the acceptance map, which are what
   your evidence has to satisfy.
2. `.superpowers/sdd/plan-10/plan-brief.md`, section 4's **W3** item - the work
   item's own statement of purpose, which the plan implements.
3. `docs/ROADMAP.md`, the section **"Gate-count derivation has no check"** in
   full, including its MEASURED and NARROWED FORM blocks. The NARROWED FORM is
   what you are building; the MEASURED block is why the cross-file lint form
   stays killed.
4. `BUILDING.md` in full - it is both an input and the subject of the edit.
5. `scripts/ledger-lint.py` in full - the check goes INSIDE this script.
6. `.github/workflows/ci.yml`'s `ledger-lint` job (its leading comment block and
   its step `name:` value; nothing else in that file).
7. The Tier-2 entries `ledger-lint-runs-before-every-push` and
   `latitude-carveout-zero-content-structural-forks` in
   `docs/process-conventions.yaml` / `docs/conventions.yaml` (grep the id).

## Scope

Exactly Task 1's **Files list (EXHAUSTIVE)** - `BUILDING.md`,
`scripts/ledger-lint.py`, `.github/workflows/ci.yml` - and within those files
exactly the regions the task names. Steps 1 through 6, and the "Must not decide"
list, are the contract.

What the plan hands you as fixed text, to transcribe rather than compose: the
canonical gate-total sentence and its marker (Step 1a), the three
`gate-block` marker lines (Step 1c), both replacement blocks in Step 1(d), the
one replacement string in Step 1(e), the counting rule and its continuation
guard (Step 2), the summary-line wording, the five fires (Step 4) and the
commit command (Step 6). Character for character.

**One fork is pre-routed and you do not resolve it:** if your own recount of the
three gate blocks disagrees with the plan's `11 / 6 / 4 / 1`, that is not a
licence to adjust the fenced sentence. The tree moved since authoring. Return
**NEEDS_CONTEXT** with both counts pasted and the commands that produced them.

**Step 1(f) is a report item, not an edit:** `BUILDING.md`'s two positional
ordinals ("part 6" at the cross-target clippy paragraph, "Rust-gate parts 1-4"
at the CI paragraph) are surfaced to the controller with the task's reasoning
and are NOT edited.

## Standing rules

- **No design latitude**, in either form - an explicit permission, or an
  omission (an unenumerated set in a normative position, a name or string you
  would have to invent, a list that ends open). A fork found on code contact
  returns as **NEEDS_CONTEXT with a decision memo** (the options, their costs
  against the named invariants, a recommendation), routed by the controller,
  never resolved at the keyboard.
- **Structural-conformance grant** (`latitude-carveout-zero-content-structural-forks`,
  read the entry): following the touched file's existing structural patterns is
  in scope where the extension has zero outward effect, and repairing a
  reference your OWN enumerated edit invalidated inside a LISTED file is in
  scope - which is exactly why `ci.yml`'s two self-descriptions and
  `BUILDING.md`'s CI parenthetical are named regions in the Files list rather
  than findings to file. Everything beyond a named region stops and returns.
- **No task edits any house-knowledge YAML** (`docs/decision-ledger.yaml`,
  `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
  `docs/process-conventions.yaml`), and no task edits `docs/ROADMAP.md` or
  `docs/process-journal.md`. Ledger-worthy observations go into your report; the
  controller is the single writer.
- **Counts are recomputed from their enumerations**, never from the plan's
  authoring snapshot alone: re-run the three block counts and the continuation
  check yourself and paste them.
- **Absence-shaped checks are fire-verified AND reach a green state on the
  intended end state**, never on the pre-state. Task 1 is the task where this
  bites hardest: five fires, each with its pasted red output, then the restored
  green run.
- **Every observed value in your report is pasted from the run that produced
  it**, never recalled, and never attributed to a command that was not the one
  run.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, in every
  file you touch and in your report.
- No new dependency, no new gate part, no new CI job, no new script. The check
  goes into the existing `scripts/ledger-lint.py`, which is NOT renamed. New
  imports are limited to `re` (plus the already-imported `Path`).

## Verification bar

1. The five fires of Step 4, each with the pasted red output and the restore
   proven. F3, F4 and F5 have exact expected-violation shapes stated in the plan
   - meet them, do not approximate them.
2. `python3 scripts/ledger-lint.py` exits 0 on the end state and prints the
   widened summary line; `git status --porcelain` after all restores shows only
   this task's intended edits (paste it).
3. **The full gate as `BUILDING.md` enumerates it**, foreground, no subsets,
   green, before the commit. Your own change is part of it now.
4. `git diff --stat` limited to the three files in the Files list. Anything else
   is a defect signal -> NEEDS_CONTEXT, not a local fix.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply here. You commit; you do NOT push (the
single push is a controller close action).

- `git -c commit.gpgsign=false commit ...` - agent commits are deliberately
  unsigned, as policy.
- Stage explicitly by name, **never `git add -A`**.
- The commit command and message are fenced in Step 6.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
  No `Claude-Session` line, no context-window suffix.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-1-report.md` with
the same content as your final message would carry if it were unlimited (your
final message itself stays under 15 lines):

- Status: DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT.
- Per-file changes against the Files list, region by region.
- The re-measurement of the three gate blocks and the continuation check, with
  commands and pasted output, against the plan's `11 / 6 / 4 / 1`.
- The five fires: for each, the mutation, the exact command, the pasted output,
  the restore proof.
- The green end state: `ledger-lint` summary line, full gate result,
  `git status --porcelain`, `git diff --stat`.
- Step 1(f)'s two surfaced ordinals with their line context.
- Divergences and judgment calls, each named.
- Numbered concerns a reviewer can rule on yes/no.
- What you surface for the controller (ledger candidates, anything the plan did
  not anticipate).
- Commit hash and `git show --stat`.
