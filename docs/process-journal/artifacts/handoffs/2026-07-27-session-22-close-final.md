# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-27 (session 22 close, superseded in-session: rehearsal GREEN, owner rulings executed)
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
four-eyes APPROVED and owner-approved; execution is COMPLETE for both
plans including plan 8's rehearsal (green after the Windows-msi codepage
fix). Neither plan is CLOSED yet; the owner wording pass for 7.5 is
already EXECUTED (in-session rulings).

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
per dispatch; RE-INSTANTIATED by owner ruling at this close: the
CONTROLLER LOOP runs Opus 5 - the owner sets it as the session model -
and Fable 5 serves ONLY design/plan four-eyes rounds, whole-branch
reviews + deltas, and four-eyes decision documents; mid tier Opus 5 for
task implementers+reviewers/fixes/recon; Sonnet 5 for plan-carried
transcription) and proc-context-budget-session-cut (this close is an
instance). Subagents never call session-relocation tools; worktrees are
plain directories. Notable Tier-2 additions this session (all binding):
design-empirical-claims-reproducible, e2e-filter-invokes-playwright-directly,
proc-noninteractive-file-ops-in-agents, proc-wrapped-prose-quote-grep
(content-anchored extraction), proc-sweep-surface-completeness,
help-topic-h1-scheme.

## Current state (verified)

- master f97776e at the final close, clean, pushed. Watched CI
  conclusions: 30261258685 (dbd0dc3), 30268338147 (663b6ec) and
  30272193732 (c999090) all SUCCESS; the runs on the last close pushes
  (628b738 through f97776e, docs/yaml-only) are the OPEN conclusion
  observations the next session checks first
  (proc-push-ci-conclusion-observed; gh-log carries the entries).
- Session-close gate ran in full: three journal entries for session 22
  (main, continuation, tiering re-instantiation), all deferrals homed in
  the two progress trackers, the gate/ci-watch logs the trackers cite
  secured from /tmp scratch into .superpowers/sdd/plan-8/gate-logs/
  (11 files).
- Plan 7.5: tasks 1-4 complete, whole-branch verdict READY, AND the
  owner wording pass is executed (commit 406e91b, reviewed APPROVED -
  all in-session owner rulings). Remaining: the plan CLOSE only
  (roll-up funnel, blocked-pool sweep, salvage incl. the registered
  citation re-pointing trigger, journal plan entry; one residual
  owner-pass candidate: de "das Thema zur Vorschlagskarte", 2 sites).
- Plan 8: wave 1 merged A-D; the Windows blocker is FIXED (07c0255:
  WiX localization file, code page 1254, the publisher keeps its
  correct orthography - the diagnosis proved the ASCII fallback would
  NOT have fixed the build, LICENSE text is a third sink) and the
  rehearsal is GREEN: runs 30272619000 + 30273529210, all four legs,
  8/8 machine observables PASS. R8 (draft inspection) and R10 (draft
  deletion) are OWNER actions by design - draft "rehearsal-30273529210"
  is preserved as his input. Remaining: whole-branch review + plan
  close (close-batch item list in the progress tracker).
- Two latent pre-session defects fixed via reviewed dispatches: the
  joblog calendar-bomb fixture and the Windows-only clippy red (five
  unobserved failure CI runs; watch discipline now ledgered).

## Next steps (priority order)

1. **Plan 8 whole-branch review** (top model; path-scoped like 7.5's -
   the plans interleave on master) over the full wave-1 + fixes union,
   triaging the deferred list in the progress tracker (BUILDING.md
   two-site update + gate-part addition [cross-target windows clippy,
   owner-APPROVED], yaml.ReaderError try-scope minor, R1/R6 wording
   one-liners, D86 fallback superseded-on-merits bookkeeping, design/plan
   stale "language" list sites, bundler-2.9.4 citation sweep). Then the
   plan-8 CLOSE per doctrine.
2. **Plan 7.5 close** per doctrine plan-close gate (whole-branch READY
   and wording pass already done; the H1 supersession one-liner and the
   de "Thema zur Vorschlagskarte" residual ride the close).
3. **Owner actions, anytime**: R8 - inspect draft release
   "rehearsal-30273529210" (assets, SHA256SUMS, body); R10 - delete the
   draft at the plan-8 close; commit the agent-side memory update
   (he does this himself).
4. Session-numbering note for the archive: this was session 22
   (claude-session 956e4bb0 continued as 5c2bfd67); the one-session-offset
   transcript-archival duty applies as usual (S22 is archived by the NEXT
   session; S21 was archived this session).

## Open questions / risks

- Wall-clock timings of this session's reviewer runs and the rehearsal
  are interruption-polluted (model-limit + server-overload windows) -
  unusable as process data.
- The draft-cleanup rule: the PLAN text governs (the owner deletes the
  run-B draft at plan close); a controller dispatch line contradicted it
  once and was caught - do not repeat.
- The license dialog's rendering of the publisher's Ş on a real Windows
  install is unverified (upstream tauri-bundler hardcodes ansicpg1252 in
  the license RTF header) - cosmetic, recorded in the wix-fix report.
- Mid-tier model resolution is Opus 5 by owner correction (proc-03
  updated); resumptions continue on spawn models.
