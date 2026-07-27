# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-27 (session 22 close: plans 7.5 + 8 executed to the rehearsal block)
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
never revert because of the flag alone.

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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-7 closed. **Plans 7.5
(track-rule add/remove) and 8 (packaging/release pipeline) ran IN PARALLEL
this session** (owner call): both designs and both execution plans are
four-eyes APPROVED and owner-approved; execution is complete except plan 8's
rehearsal, which is BLOCKED on a Windows bundling defect (below). Neither
plan is CLOSED yet.

## Constraints and conventions

The SIs above; the doctrine (SI-1). Spec authoritative over plans; the two
initiative designs (2026-07-22-plan75-track-rule-add-remove-design.md
D65-D72 + witness amendment; 2026-07-22-plan8-packaging-release-design.md
D75-D90 + amendment A1) are the contracts; the two plans in
docs/superpowers/plans/ (2026-07-23-*) bind execution, incl. their
progress trackers `.superpowers/sdd/plan-7.5/progress.md` and
`.superpowers/sdd/plan-8/progress.md` (READ BOTH - they carry the exact
open-fork queues and all deferred minors). Nine-part gate per BUILDING.md
before any push and after every merge. Ledger/Tier-2 YAML by targeted text
replacement only; `scripts/ledger-lint.py` after every edit (now also
duplicate-key-checking, and wired as a CI job). Binding session-scale rules
in docs/process-conventions.yaml: proc-03-model-assignment (explicit model
per dispatch; top model only controller/whole-branch/four-eyes/decision
docs; mid for task implementers+reviewers; cheap for plan-carried
transcription) and proc-context-budget-session-cut (this close is an
instance). Subagents never call session-relocation tools; worktrees are
plain directories. Notable Tier-2 additions this session (all binding):
design-empirical-claims-reproducible, e2e-filter-invokes-playwright-directly,
proc-noninteractive-file-ops-in-agents, proc-wrapped-prose-quote-grep
(content-anchored extraction), proc-sweep-surface-completeness,
help-topic-h1-scheme.

## Current state (verified)

- master ef1f2f3 at this writing (journal entry committed; the close
  commit with this snapshot follows), clean; last watched CI run
  30261258685 (dbd0dc3) SUCCESS - first full-matrix green since 07-22.
  Later local commits are pushed with the close; their CI conclusion is
  an OPEN observation for the next session (proc-push-ci-conclusion-
  observed).
- Plan 7.5: tasks 1-4 complete, streams merged, spec amendments landed,
  whole-branch verdict READY (whole-branch-verdict.md). Remaining: the
  plan CLOSE (roll-up funnel, blocked-pool sweep, salvage incl. the
  registered citation re-pointing trigger, journal plan entry, owner
  rendered-surface pass - the wording agenda is with the owner).
- Plan 8: wave 1 (T1-T5) complete and merged A-D, all gates green;
  rehearsal (T6) BLOCKED: run 30263340264 - both Windows legs fail in
  WiX light (stderr discarded by the bundler; unverified hypothesis:
  U+015E in publisher/copyright vs Windows-1252), run B deliberately not
  dispatched. Full fork queue with routing in the plan-8 progress
  tracker. Whole-branch review runs only after a green rehearsal.
- Two latent pre-session defects fixed via reviewed dispatches: the
  joblog calendar-bomb fixture and the Windows-only clippy red (five
  unobserved failure CI runs; watch discipline now ledgered).

## Next steps (priority order)

1. **Rehearsal fork queue** (plan-8 progress tracker, verbatim): (a) one
   debug dispatch surfacing WiX light's stderr to confirm/refute the
   U+015E hypothesis; (b) if confirmed, design amendment on D86 (its
   fallback trigger is unreachable - gated on a rendering observable a
   build failure never reaches; copyright carries the same character) plus
   an OWNER ruling on transliterating the publisher/copyright strings
   (it is the owner's name - product-visible, his call); (c) re-run the
   rehearsal to green, then run B; (d) owner authorization pending for
   `sudo dnf install dpkg msitools` (R6 local inspections) or a
   substitute-tool ruling (ar/7z/bsdtar exist).
2. **Plan 7.5 close** per doctrine plan-close gate, incl. the owner
   rendered-surface pass (agenda presented to the owner in-session:
   de cross-reference normalization at three sites, Remove-sentence
   pronoun, warning-location naming, the earlier output-directory tooltip
   wording, two spec one-clause candidates).
3. **Plan 8**: after a green rehearsal - whole-branch review (its
   deferred-item list incl. the BUILDING.md two-site update, the
   yaml.ReaderError try-scope minor, the cross-target-clippy gate-part
   candidate, R1 wording alignment), then the plan close.
4. Session-numbering note for the archive: this was session 22
   (claude-session 956e4bb0 continued as 5c2bfd67); the one-session-offset
   transcript-archival duty applies as usual (S22 is archived by the NEXT
   session; S21 was archived this session).

## Open questions / risks

- The U+015E hypothesis is UNVERIFIED until light's stderr is captured -
  do not amend D86 on the hypothesis alone.
- Wall-clock timings of this session's reviewer runs and the rehearsal
  are interruption-polluted (model-limit + server-overload windows) -
  unusable as process data.
- The rehearsal draft-cleanup rule: the PLAN text governs (the owner
  deletes the run-B draft at plan close); a controller dispatch line
  contradicted it once and was caught - do not repeat.
- Owner wording rulings (7.5 agenda) and the dpkg/msitools authorization
  are pending owner input; neither blocks the rehearsal debug dispatch.
