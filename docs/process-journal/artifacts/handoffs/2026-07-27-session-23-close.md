# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-27 (session 23 close)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-8 CLOSED, including 7.5
(track-rule add/remove in the editor) and 8 (packaging/release pipeline),
both closed this session. **Plan 9 (core/orchestration hoists + planner
seam) is the last planned package before the pre-1.0 gates**; its anchor and
named inputs are in docs/ROADMAP.md.

## Constraints and conventions

The SIs above; the doctrine (SI-1). Spec authoritative over plans and
designs on conflict. Nine-part gate per BUILDING.md before any push and after
every merge - **now TEN parts**: a cross-target Windows clippy run joined it
this session by owner ruling (`cargo clippy --workspace --all-targets
--target x86_64-pc-windows-msvc -- -D warnings`, with
`rustup target add x86_64-pc-windows-msvc` as a documented one-time
prerequisite; Tier-2 `gate-includes-cross-target-lint-for-the-unrun-os`).
Ledger/Tier-2 YAML by targeted text replacement only, never a serializer
round-trip; `scripts/ledger-lint.py` after every edit (it is a CI job now, so
a red ledger blocks a release rehearsal by construction). Model tiering per
`proc-03-model-assignment`, explicit model parameter at EVERY dispatch:
controller loop and every implementer/task reviewer/fix dispatch on the mid
tier, top tier reserved for design/plan four-eyes rounds, whole-branch
reviews and their deltas, and four-eyes decision documents. Context-budget
session cut per `proc-context-budget-session-cut`. Subagents never call
session-relocation tools; worktrees are plain directories.

## Current state (verified)

- master at the session close, clean, pushed; the push-triggered CI run's
  conclusion is observed before the session ends
  (`proc-push-ci-conclusion-observed`, gh-log carries the entries). The
  earlier run of this session (adb0f6e) was SUCCESS including the new
  ledger-lint job.
- **Plan 7.5 CLOSED**: whole-branch READY, the owner's wording pass executed,
  citations re-pointed with the salvage, salvage verified at 36 files in the
  commit, journal entry written.
- **Plan 8 CLOSED**: whole-branch review returned NEEDS FIXES (one shipped
  regression plus the deferred documentation debt), the fix wave discharged
  all 18 edit sites, the resumed reviewer's delta returned READY. The owner's
  rendered-surface pass ran on top with its own review, one fix round and a
  second delta, all APPROVED. Salvage verified at 63 files in the commit,
  with the pre-registered citation sweep run in the same change - recounted
  at ten plan-8 refs against the plan's dated snapshot of eight, and extended
  to nineteen so that no house-YAML citation names a bare scratch filename
  any more.
- Both plans' rules converged: a frozen transcription target is never
  rewritten; its drift is carried by a supersession note. Applied to the
  plan-7.5 design and plan, the plan-8 design (amendments A2 and A3) and the
  plan-8 plan's frozen fences.
- House knowledge: 476 entries across the four files, all invariants green.
  Nine new entries this session, eight existing ones touched.

## Next steps (priority order)

1. **Owner actions, open** (recorded as open user actions, never booked as
   done): R8 - inspect the preserved draft release `rehearsal-30273529210`
   (assets, SHA256SUMS, rendered body), including the addendum to check that
   the Windows installer's license dialog renders the publisher name
   correctly; R10 - delete that draft; and the one wording item he reserved
   for himself (whether two continuation lines in the release-body template
   render inline - the same inspection answers it, and it now applies to
   three wrapped regions in that file rather than one).
2. **Plan 9 kickoff** (core/orchestration hoists + planner seam): the last
   planned package. Its anchor in docs/ROADMAP.md carries the named design
   inputs; the ROADMAP Triggers entry "Plan 7, 8 or 9 starts -> consume the
   named design inputs in that plan's anchor" fires at that kickoff. A
   registered trigger also fires there: the D49 G1/G2 removal experiment runs
   on the next core/planner-touching plan.
3. **Pre-1.0 gates** after Plan 9, per the ROADMAP: the README's four
   `placeholder(1.0)` comments and its WIP banner at the tag, then the
   at-1.0 deliverables (single GUIDE.md, two blog posts, the
   requirements-catalog derivation), each authored in a fresh session per the
   recorded authoring pipeline.
4. **Session-numbering note for the archive**: this was session 23; the
   one-session-offset transcript-archival duty applies as usual (session 23
   is archived by the NEXT session; session 22 was archived this session, one
   strand, no tool-results component).

## Open questions / risks

- Two process-doctrine amendments were ADOPTED this session on the owner's
  authorization, both from this session's reviews, both mirrored into the
  shared collection with their manifest rows updated:
  - **Plan-close step 3** now names the salvage the LAST write into the
    scratch rather than a numbered step, adds a re-salvage duty for
    close-generated work, and carries a `diff -r` of scratch against salvaged
    copy as the handle at the plan-close and session-close gates. The check
    was fire-verified here (planted file -> DIFFERS, removed -> current) and
    both closed plans read current.
  - **Controller duty, §4**: independent verification needs independent
    INSTRUMENTS, not just an independent context - a reviewer reproducing an
    implementer's empirical claim builds its harness where the implementer
    could not have written.
  Both carry the SAME deferred revisit, on the owner's instruction: a later
  branch-and-pull-request execution model may change or void them, and the
  revisit must CHECK rather than assume - worktree and PR isolation separates
  repo state, while the instruments in question lived outside the repo in a
  shared scratch directory that separate worktrees still shared. The
  structural alternative for the salvage class (track the scratch from the
  start instead of snapshotting it) is recorded as deferred to that same
  discussion, because it forces a commit-cadence decision for half-written
  subagent artifacts during a run.
- Briefs are the one artifact four-eyes does not cover by construction, and
  five controller-brief defects were found downstream this session (a site
  enumeration off by one, a file count contradicting its own enumeration, a
  review brief contradicting itself, an artifact filename that exists
  nowhere, and a factual claim about macOS refuted at Apple's own man page).
  Every one was caught by the receiving agent. Worth watching whether the
  rate falls.
- The plan-8 close batch left three advisories on the next design-touching
  change: a clause in amendment A3 about a reviewer figure, that amendment's
  region mapping, and two report-internal citations that do not reproduce.
  The exact replacement text for the one ci.yml comment fix rides the
  ROADMAP v1.x mise-removal entry, gated on the next ci.yml-touching change
  whichever it turns out to be.
