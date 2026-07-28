<!-- Snapshot of HANDOFF.md at the session-24 close (2026-07-28), per SI-5: the HANDOFF is git-ignored and superseded in place. Supersedes the earlier same-day snapshot; the gate ruling landed after it. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-28 (session 24 close)
**Active agent:** Peter (~/agents/peter)
**Working directory / repo:** ~/Git/Muxsmith (github.com/senolfeldmann/Muxsmith, public, master tracks origin/master)

## STANDING INSTRUCTIONS - reproduce this ENTIRE section verbatim in every future HANDOFF, including this reproduction rule itself

### SI-1. Process package

This project runs the **software-dev-process** package as its standing
method: the skill (auto-triggers at controller moments) plus the doctrine
at `~/agents/peter/prompts/software-dev-process.md`. The doctrine is the
single source for the execution spine (Superpowers workflow, SDD never
inline in the controller, parallel worktree streams), write-at-creation
rules, the gates (session start/close, plan close, milestone, pre-execution),
controller duties, and - since session 10 - **house-knowledge management
(doctrine section 7)**: the Tier-2 convention files + Tier-1 ledger, the
source x nature promotion matrix, the escalation flow. Do NOT restate
doctrine content here - read it. Şenol's ruling 2026-07-11: this binding is
project-scoped; new projects get an adopt-or-not question at kickoff.

### SI-2. Process journal

Journal duty per `docs/process-journal/PROMPT.md` (git-tracked, read it each
time - it mandates the salvage pass incl. reviewer-verdict files and a
HANDOFF snapshot at EVERY plan close). Entries at every plan completion and
session close.

### SI-3. mkvtoolnix parity audit in all planning and decision-making

When authoring plans or ADRs, or resolving ANY behavioral question, compare
against mkvtoolnix-gui / mkvmerge wherever meaningful. Load-bearing
distinction: mkvtoolnix is INTERACTIVE (pre-fills guesses the user reviews),
Muxsmith is DECLARATIVE BATCH (the profile is the spec). Muxing semantics and
output are parity targets; input-time convenience guesses are NOT
(docs/IDEAS.md 1-2). Method: classify match / justified divergence / genuine
gap; read the source at ~/Downloads/mkvtoolnix (cite file:line); confirm
mkvmerge behavior by running the binary (v100.0), never from memory; surface
gaps and divergences for Şenol; record divergences in the memo. Licensing
boundary (mkvtoolnix GPL, Muxsmith MIT): behavior, facts and interfaces are
fair game; literal code or text passages are never taken; deliberately
modeled wording is recorded as an explicit ADR decision.

### SI-4. Git commits and pushes are STANDING-authorized for this repo

Şenol's grant (2026-07-09, "persist indefinitely"): commits AND pushes on
~/Git/Muxsmith are authorized standing; never re-request. Agent commits are
deliberately UNSIGNED as policy (a GPG signature is Şenol's authorship claim):
`git -c commit.gpgsign=false` on every agent commit and merge. Trailer per
convention; log every push in gh-log.md (git-ignored). Permission mechanics
are solved: Şenol added the git allow-rules to the agent-side permission
file himself; they match ONLY pure git command shapes (a cd-into-repo plus
git, or git -C). ANY non-git segment chained into the compound voids the
match and the command falls to a permission classifier that sees only the
global never-push rule and denies. Keep git commands pure; do bookkeeping
separately; a denial of a compound is a denial of the SHAPE, not the
action - re-shape and retry before treating the push as blocked (a day's
push was lost to this once; for push specifically the `git -C` shape is the
one that passes - confirmed deliberate 2026-07-15). The agent cannot edit
its own permission file; any rule change is Şenol's edit. Never
`git add -A` (untracked artifacts); stage explicitly. The harness's
security monitor may falsely flag authorized subagent commits (this grant
is invisible to it): verify the commit's content, name the false alarm,
never revert because of the flag alone. **A dispatch that expects a
subagent to commit RESTATES this grant in the dispatch text** - the
subagent inherits a global never-commit default and cannot see a grant
that lives only here (ledger `dispatch-restates-the-standing-commit-grant`).
**Trailer set, owner-ruled 2026-07-28** (Tier-2 `agent-commit-trailer-set`):
exactly one trailer, `Co-Authored-By: Claude <model> <noreply@anthropic.com>`,
no `Claude-Session` line; the model name is canonical with no context-window
suffix; and the string is DERIVED from the dispatch's model parameter, never
written as a literal in a plan or brief. **Two writers in one working tree
share one git INDEX**, so staging your own paths does not isolate them - a
bare `git commit` takes everything staged. Use pathspec-scoped commits
(`git commit -- <paths>`) or give the second writer its own worktree
(`concurrent-writers-need-pathspec-scoped-commits`).

### SI-5. HANDOFF lifecycle: snapshot every state, publication-grade always

HANDOFF.md is git-ignored and superseded in place, so any state not
snapshotted dies with its overwrite. Rule: whenever HANDOFF.md is rewritten
(plan close, session close, mid-session supersede), snapshot the NEW state in
the same turn to `docs/process-journal/artifacts/handoffs/<date>-<label>.md`
and commit it. Because snapshots are committed to the public repo, the HANDOFF
is written publication-grade at ALL times: nothing enters this file that could
not go public - no secrets, no personal or private context, no names or paths
beyond the project's approved-public set.

(SI-1 through SI-5 are carried forward by the reproduction rule in this
section's heading.)

## Objective

Muxsmith v1: rule-based bulk MKV muxing tool (Rust core + CLI + Tauri 2/Vue 3
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-8 and 8.5 are CLOSED.
**Plan 9 is RUNNING: design and plan are owner-approved, Task 1 of 7 is done
and reviewed. Resume at Task 2.**

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict; the Plan 9 design is authoritative over the plan.

- **The plan is the contract**:
  `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`
  (7 tasks, plus amendments 1 and 2, all four-eyes reviewed). Its design is
  `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
  (D91-D105, owner-approved, amended twice). **Every D-entry AND every entry
  in the design's amendment log binds, at the log's state at execution time** -
  the pointer is the contract, not an enumeration of it.
- **Execution is strictly serial, no worktrees** (the plan's sequencing
  section argues it from the file graph). **The serial ruling binds the
  CONTROLLER's dispatch concurrency too**: do not write or commit anything in
  this tree while a task is live. That rule was violated once in session 24
  with no damage but a live hazard; the occurrence is on
  `a-serial-ruling-binds-dispatch-concurrency-too`.
- **The gate is TEN parts** per BUILDING.md, foreground, no subsets, before
  any push and after every merge - **and it binds EVERY push with no
  docs-only exception** (owner ruling 2026-07-28, on the measurement that a
  full run costs 11 seconds on an unchanged tree, which is what a docs-only
  push is).
- **`python3 scripts/ledger-lint.py` runs before every push too** (Tier-2
  `ledger-lint-runs-before-every-push`). It is NOT one of the ten parts and it
  is the only check a documentation-or-YAML push can turn red. Two edits to
  BUILDING.md's gate block are pending and land together AT THE PLAN CLOSE,
  both tracked in the ROADMAP: this as an eleventh part, and
  `--document-private-items` on the doc step at both consuming sites
  (BUILDING.md and ci.yml), which closes the blind spot where rustdoc skips
  private modules - the GUI shell hides three behind private `mod`
  declarations, so a dangling doc link there passes every run today. Neither
  is done mid-plan: the plan quotes the ten-part gate verbatim and every task
  verifies against that wording, so the plan's own text is updated in the same
  pass.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip; `python3 scripts/ledger-lint.py` after
  every edit; it is a CI job. 492 entries across the four files at this close.
  **No task edits these files** - the controller is the single writer, and a
  task that finds something ledger-worthy surfaces it in its report.
- **Model tiering** per `proc-03-model-assignment`, explicit model parameter at
  EVERY dispatch: top tier for design/plan four-eyes rounds and whole-branch
  reviews, mid tier for the controller loop, task implementers, task reviewers
  and fix dispatches, cheap tier only where a plan carries the work verbatim.
  The plan's own tier table rules no Task-9 task cheap.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- master at `39ed4d9`, clean, pushed. Every push this session had its CI
  conclusion watched to completion and logged.
- **Plan 9 design: owner-approved** after one four-eyes review round (four
  blocking findings, five minor), one fix round, an APPROVED delta review, and
  two closed wording notes. **Amended twice afterwards on owner rulings**,
  each amendment re-reviewed by the same reviewer: amendment 1 (a feature's
  tests ship with the feature; no GUI identification session cache) and
  amendment 2 (the e2e `name()` helper hoist).
- **Plan 9 plan: approved** by the owner and by its own four-eyes loop
  (APPROVED with three minors, fixed, delta APPROVED), then amended twice with
  the same rulings and re-reviewed each time.
- **Task 1 DONE**: commits `9bbe53d` (the seam hoist, 8 files, +288/-246) and
  `fed55be` (doc-only fix round). Review APPROVED; all three findings CLOSED
  by the delta. Behavior preservation verified per divergence, both completion
  greps fire-verified with reachable green states, the CLI's inline queue
  block proven byte-identical for Task 2, 494 tests with real mkvmerge and
  zero skips.

## Next steps (priority order)

1. **Task 2**: `run_batch` hoists into `muxsmith_core::executor`; the
   src-tauri runs-root seam is DELETED, not hoisted (D96, D97). It replaces
   the CLI's inline queue block that Task 1 deliberately left byte-unchanged.
   Then Tasks 3-7 in order. Each: fresh implementer, independent reviewer, the
   plan's own model tiers, verdict-arrival gate on every verdict (route the
   findings AND mine the harvest into the ledger before the next dispatch).
2. **Plan close actions**, all listed in the plan, plus the two BUILDING.md
   gate edits above: the roll-up funnel; the
   promotion sweep of the five owner-ruled ledger entries the ROADMAP anchor
   enumerates, whose statements describe a tree that only exists once the work
   lands; `core-121`'s `blocked_on` clearing; the `core-d49-g1g2-experiment`
   entry written by the controller from Task 7's measurement; the whole-branch
   review on the top tier; the SDD salvage with its `diff -r` re-check; the
   journal entry; the HANDOFF snapshot; and the rustdoc-flag adoption above.
3. **Pre-1.0 gates** after Plan 9, per the ROADMAP: the README's four
   `placeholder(1.0)` comments and its WIP banner at the tag, then the at-1.0
   deliverables (single GUIDE.md, two blog posts, the requirements-catalog
   derivation), each authored in a fresh session.
4. **Archive duty:** session 24 is archived by the NEXT session; session 23
   was archived at the start of this one.

## Open questions / risks

- **The controller's own error class dominated this session's findings.** Four
  claims of mine were refuted by agents downstream - a finding count, a
  trigger's firing condition read to the end of its first clause, a cost
  estimate built on a measurement that answered a narrower question, and an
  enumeration naming one consuming site where there are two. None was a fact
  nobody could check; all four were borrowed claims passed on without
  re-measuring. The handles now exist as house entries; whether the rate falls
  is the thing to watch.
- **Two acceptance consequences are covered by a chain, not end to end.** The
  new e2e scenarios pass on the pre-feature tree, because in both cases the
  behavior exists and only the assertion was missing; the mocked IPC boundary
  supplies the new input by hand. The plan states this explicitly at both
  sites. If a future reader wants one end-to-end proof, that is new work, not
  a correction.
- **One flaky test, owner-ruled a 1.x fix** (ROADMAP "Test flakiness"):
  `dry_run_json_emits_a_document_when_the_language_query_fails`. It has not
  reappeared; four full local gate runs this session were green.
- Framework-side follow-ups are tracked agent-side.
