<!-- Provenance: snapshot of HANDOFF.md at the session-25 close (2026-07-28), per SI-5. -->
# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-28 (session 25 close)
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
**Plan 9 is RUNNING: tasks 1-4 of 7 are done and approved. Resume at Task 5.**

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict; the Plan 9 design is authoritative over the plan.

- **The plan is the contract**:
  `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`
  (7 tasks, amendments 1-4, all four-eyes reviewed). Its design is
  `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
  (D91-D105, owner-approved, amended). **Every D-entry AND every entry in the
  design's amendment log binds, at the log's state at execution time** - the
  pointer is the contract, not an enumeration of it.
- **Execution is strictly serial, no worktrees**, and the serial ruling binds
  the CONTROLLER's dispatch concurrency too: do not write or commit in this
  tree while a task is live.
- **The gate is TEN parts** per BUILDING.md, foreground, no subsets, before any
  push, with no docs-only exception; `python3 scripts/ledger-lint.py` runs
  before every push as well and is not one of the ten.
- **Model tiering, owner ruling 2026-07-28**: the top model serves ONE role,
  the plan-close whole-branch review and its delta re-reviews. Design and plan
  four-eyes rounds, decision documents, task implementers, task reviewers, fix
  dispatches and recon all run the mid tier; plan-carried transcription runs
  the cheap tier. Every dispatch names its model explicitly.
- **Test-coverage precedence, owner ruling 2026-07-28** (Tier-2
  `tests-ship-with-the-feature-never-after`): at execution time the package
  BUILDS a missing producer rather than routing it when all four hold -
  additive, existing infrastructure, the consequence comes from this package's
  own diff, and it is named in the report. Outside those four the plan's
  enumeration binds and the fork returns.
- **Files-list boundary, owner ruling 2026-07-28**: the enumeration binds over
  FILES; an entry constrains work within its file only where it carries an
  explicit qualifier. Repairing a reference the task's own edit invalidated,
  inside a listed file, is in scope.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip, and never by a script anchored on a repeated
  key pair (that silently edits a neighbouring entry - it happened twice this
  session; anchor on the entry's `- id:` line). 505 entries at this close.
  **No task edits these files** - the controller is the single writer.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- master at `7016a34`, clean. **15+ commits unpushed at the time of writing;
  the session-close push runs the full gate first.**
- **Tasks 1-4 DONE and approved.** Task 1 `9bbe53d` + `fed55be`; Task 2
  `9b2843f`; Task 3 `9e5e112` + fix round `4e73739`; Task 4 `d768657` +
  `3412fcc`. The run's tracker is `.superpowers/sdd/plan-9/progress.md` and it
  carries per-task state, verdicts, amendments and the carried-forward
  constraints.
- **Amendment 3** (the moved `run_batch` rustdoc restated for its core home;
  design `08621cb`, plan `36d8538` + `63fc5b2`) and **amendment 4** (the German
  subprocess test rides a locale-parameterized pinned helper; plan `ba69c36` +
  `4e5daa6`) are both CLOSED, each four-eyes reviewed.

## Next steps (priority order)

1. **Task 5** (central errors-first sort + BatchView code-keyed fetch, D102/D103,
   spec S-7), then 6 and 7. Each: fresh implementer, independent reviewer, the
   plan's model tiers, verdict-arrival gate on every verdict.
   **Anchors moved:** Task 4 inserted its scenario into `e2e/smoke.spec.ts`, so
   everything below shifted by +54 - the apply-flow test is at `:460` and the
   enabled assertion at `:565`. Locate by content; do not transcribe the plan's
   authoring-time numbers.
2. **Plan close actions**, all listed in the plan, plus: the two BUILDING.md
   gate edits (ledger-lint as an eleventh part; `--document-private-items` on
   the doc step at both consuming sites); the roll-up funnel; the promotion
   sweep of the five owner-ruled ledger entries; `core-121`'s `blocked_on`;
   the `core-d49-g1g2-experiment` entry from Task 7's measurement; the
   whole-branch review on the top tier; the SDD salvage with its `diff -r`
   re-check; the journal entry; the HANDOFF snapshot. **New this session:** the
   D64 snapshot claim needs TWO edits, not a recount alone - see the ROADMAP's
   "Docs accuracy" item.
3. **Pre-1.0 gates** after Plan 9, per the ROADMAP.
4. **Archive duty:** session 25 is archived by the NEXT session; session 24 was
   archived at the start of this one, together with its own /tmp artifacts.

## Open questions / risks

- **One owner ruling is pending**, raised by the Task-4 review: whether adding
  a symbol import that a task's own enumerated addition requires survives a
  "nothing else in this file" qualifier. Task 4 did it, disclosed it, and its
  reviewer ruled it correct; the standing entry does not yet say so, and Task 5
  writes the same file under the same qualifier.
- **The controller's own error class remained this session's most frequent**:
  four claims of mine were refuted downstream by measurement (a wrong ledger id
  carried into two briefs, a stale commit hash in a dispatch, a one-file
  measurement compressed into "one call site", and a docs-accuracy item stale
  within the hour). All four are recorded with handles; the rate is the thing
  to watch.
- **No check in this repo can go red on a broken intra-doc link inside a
  `tests/` module** - cargo does not document test targets, and the pending
  private-items flag does not change that.
- **One flaky test, owner-ruled a 1.x fix** (ROADMAP "Test flakiness"):
  `dry_run_json_emits_a_document_when_the_language_query_fails`. It has not
  reappeared.
- Framework-side follow-ups are tracked agent-side.
