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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-8 and 8.5 CLOSED. This
session closed 7.5 (track-rule add/remove), 8 (packaging/release pipeline)
and 8.5 (macOS packaging fixes - the three defects the owner's first
real-hardware walk-through found, all owner-accepted on 2026-07-28).
**Plan 9 (core/orchestration hoists + planner seam) is the last planned
package before the pre-1.0 gates.** Its ROADMAP anchor now carries a
1119-line recon inventory
(`docs/process-journal/artifacts/`... not salvaged: it lives at
`.superpowers/sdd/plan-9/recon-inventory.md`) plus the corrections that
recon made to the anchor's own figures - read the anchor's RECON block
before designing, because several of its long-standing numbers were wrong.

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

- master clean and pushed; the push-triggered CI conclusion is observed
  before the session ends (`proc-push-ci-conclusion-observed`; gh-log carries
  every entry).
- **Plan 8.5 CLOSED 2026-07-28.** Four serial tasks on one tree (no
  worktrees, argued in the plan: three config/doc tasks do not amortize four
  worktree setups plus a merge gate each). One fix round on Task 1, one
  mid-run plan amendment, whole-branch verdict READY with nothing blocking.
  Rehearsal run 30312889098 green, 6/6 jobs, all six machine halves with
  fired controls and two differential measurements against the original
  defect artifact. Salvage verified at 17 files in the commit.
- **The owner accepted all three rulings at their acceptance surface**
  (2026-07-28): the installer shows the unidentified-developer path instead
  of "damaged", the dmg mounts with no licence dialog, the rendered draft
  body carries the three OS links on one line, and `docs/INSTALL.md`'s macOS
  section matches the flow he walked.
- Plans 7.5 and 8 also closed this session, each with its own salvage,
  citation re-pointing and journal entry.
- House knowledge: 480 entries across the four files, lint green.

## Next steps (priority order)

1. **Owner action, the only one open**: delete the rehearsal draft
   `REHEARSAL - not a release (run 30312889098)`. Recorded as an open user
   action, never booked as done. No agent in this project may touch a
   release.
2. **Plan 9 kickoff** (core/orchestration hoists + planner seam) - the last
   planned package before the pre-1.0 gates. Its design is NOT written; the
   recon inventory at `.superpowers/sdd/plan-9/recon-inventory.md` (1119
   lines) is, and the ROADMAP anchor's RECON block carries the findings that
   change what the design must decide. Read both before briefing a design:
   the anchor's own long-standing figures were measured wrong (the
   duplication is 260 lines, not the ledger's "~100"), the four copies differ
   in seven deliberate and six accidental ways, one candidate item may be
   closeable without building anything, and one seam has no consumer at all.
3. **Pre-1.0 gates** after Plan 9, per the ROADMAP: the README's four
   `placeholder(1.0)` comments and its WIP banner at the tag, then the
   at-1.0 deliverables (single GUIDE.md, two blog posts, the
   requirements-catalog derivation), each authored in a fresh session per the
   recorded authoring pipeline.
4. **Session-numbering note for the archive**: this was session 23 (it ran
   past midnight into 2026-07-28). The one-session-offset transcript-archival
   duty applies as usual - session 23 is archived by the NEXT session;
   session 22 was archived at the start of this one.

## Open questions / risks

- **One flaky test, owner-ruled a 1.x fix** (`ROADMAP` "Test flakiness"):
  `dry_run_json_emits_a_document_when_the_language_query_fails` failed once
  under the full gate and passed four times after, including in isolation. No
  Rust changed in the plan that surfaced it and CI had run the identical code
  green hours earlier. The candidate fix is named and explicitly labelled as
  removing a known race class rather than as a confirmed fix for a cause that
  remains unestablished - the 1.x pass should say which of the two it claims.
- **Three doctrine amendments were adopted this session** and mirrored into
  the shared collection with its manifest rows updated: the salvage is the
  last write of a close (with a `diff -r` handle at the close and
  session-close gates), independent verification needs independent
  INSTRUMENTS rather than only an independent context, and a parallel stream
  must earn its overhead (with the shared-index mechanism naming why one-tree
  parallelism is not made safe by staging discipline alone). The first two
  carry a deferred revisit tied to a future branch-and-pull-request execution
  model; that revisit must CHECK rather than assume, since worktree isolation
  separates repo state while the instruments at issue live outside the repo.
- **Briefs remain the one artifact four-eyes does not cover by
  construction.** Nine controller-brief defects were found downstream this
  session, every one by the agent receiving it - including a factual claim
  about macOS that Apple's own man page refuted and a sweep claim that was
  false as phrased and true in substance. Worth watching whether the rate
  falls.
