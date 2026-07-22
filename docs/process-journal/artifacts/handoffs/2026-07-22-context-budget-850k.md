# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-22 (session 20 close: Plan 7 mid-wave-2, context-budget cut)
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
`git add -A` (untracked artifacts); stage explicitly.

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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-6 complete. **Plan 7
(help mode + i18n cluster) is EXECUTING**: design and plan four-eyes-approved
and committed; wave 1 fully merged; wave 2 at task 14 of 16.

## Constraints and conventions

The SIs above; the doctrine (SI-1). Spec authoritative over plans
(docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); the Plan 7 design
is docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md (D50-D64,
amended through review round 6); the plan is
docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md (21 tasks, 4 waves).
Nine-part gate per BUILDING.md before any push; full gate after every merge.
New/changed Fluent messages and help topics land bilingual (en+de).
Ledger/Tier-2 YAML by targeted text replacement only; `scripts/ledger-lint.py`
after every edit. Binding session-scale rules in
docs/process-conventions.yaml: **proc-03-model-assignment** (OWNER BOUND:
top model only for controller loop / whole-branch review / four-eyes rounds /
decision documents; mid tier for task implementers and every task reviewer;
cheap tier for plan-carried transcription; EVERY dispatch names its model
explicitly) and **proc-context-budget-session-cut** (above 850k controller
context: no new dispatches, close the session cleanly; raised from 700k
by owner directive, S21). Subagent dispatches
NEVER call session-relocation tools (EnterWorktree/ExitWorktree) - worktrees
are plain directories, absolute paths only (implementer preamble carries the
ban; ledger proc-no-session-relocation-in-subagents).

## Current state (verified)

- master b128596, clean, pushed (== origin). CI green on session pushes.
- **Wave 1 fully merged** (streams A-E, five sequential merges, full gate
  after each): bilingual CLI (D63/D64 incl. the en-pin test funnel), 42
  editor tooltips as Fluent .tooltip attributes, the D55 attribute
  migration (+ corrected e2e parse-guard discriminator), resolvedTrackLabel
  via Fluent, live locale switch (D56), 44 help topics (D54 set).
- **Wave 2 in .worktrees/plan7-f at 18a9801** (branch plan7-f, clean):
  - T11 marked 18.0.7 + topic loader: DONE + APPROVED (c445aa7).
  - T12 help mode (sidebar, single v-html, E3 suppression): DONE +
    APPROVED (906260b); post-approval D52 hover-clear fix DONE + APPROVED
    (f8e7d5d).
  - T13 D54 annotations: DONE + APPROVED (97f707f).
  - T14 field-anchored markers: DONE (18a9801), review **NEEDS FIXES** -
    one reachable double-marker (the rule detail-panel SectionWidget
    mounts at the same `tracks[i]` path as the grid row and lacks
    `suppress-self-anchor`; fix is one line at EditorView.vue ~597 plus a
    fixture extension so the e2e opens a rule carrying a bare `tracks[i]`
    diagnostic). Verdict with full evidence:
    .superpowers/sdd/plan-7/task-14-verdict.md.
- All task verdicts and the run tracker live in .superpowers/sdd/plan-7/
  (progress.md = task table; controller-notes.md = cross-task constraints
  for T19/T20 dispatches, whole-branch review inputs, pending design
  one-liner; owner-surface-pass-inputs.md = collected de-wording items).
- Design amendments committed through round 6 (bd21e85); journal entry for
  session 20 committed (b128596).

## Next steps (priority order)

1. **T14 fix round** - first dispatch of the next session: resume-style
   fix per task-14-verdict.md F1 (one line + fixture), re-review as the
   same-reviewer delta (fresh dispatch reading the verdict file; model:
   mid tier), then T14 is done.
2. **T15** (D58 curated-domain dropdowns; consumes T14's path prop) and
   **T16** (D59 ordinal column) - serial in plan7-f, mid-tier implementers
   and reviewers per proc-03.
3. **Merge wave 2** into master, full gate.
4. **Wave 3**: plan7-g serial T17-T19 (check-i18n chain; the T19 dispatch
   MUST carry the two constraints from controller-notes.md: zero-pipe
   table check, inline-code-span exemption for the raw-HTML check) in
   parallel with plan7-h T20 (IpcError number promotion; string-sink
   null-narrowing constraint applies).
5. **Wave 4**: T21 spec amendments on master (land-together set per the
   design's section 6).
6. **Whole-branch review** (top model per proc-03) with the collected
   inputs in controller-notes.md; then the plan-close gate (roll-up
   funnel, blocked-pool sweep, salvage per SI-2's prompt, owner surface
   pass over owner-surface-pass-inputs.md, journal, HANDOFF).

## Open questions / risks

- Owner veto window still open on the Tasks-8-10 content acceptance
  criteria (markdown subset, h1 opener, 1-3 kB band) - reviewers reported
  zero chafing; closes at the owner surface pass unless exercised earlier.
- Owner decision parked as ROADMAP discussion anchor: the editor cannot
  add or remove track rules (surfaced by the T10 help-content review);
  pre-1.0 vs 1.x timing is the owner's call, not blocking Plan 7.
- The h1-form split across content streams and the collected de coinages
  ride the owner surface pass (owner-surface-pass-inputs.md).
- Wall-clock durations of several wave-1/2 runs are stall-polluted
  (session-relocation prompt waits) - unusable as process timings.
