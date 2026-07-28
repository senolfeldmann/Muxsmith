# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-28 (session 23 close)
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
**Plan 9 (core/orchestration hoists + planner seam) is the last planned
package before the pre-1.0 gates.**

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict.

- **The gate is TEN parts** per BUILDING.md, foreground, no subsets, before
  any push and after every merge. The tenth is a cross-target Windows clippy
  run (`cargo clippy --workspace --all-targets --target
  x86_64-pc-windows-msvc -- -D warnings`), with `rustup target add
  x86_64-pc-windows-msvc` as a documented one-time prerequisite.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip; `python3 scripts/ledger-lint.py` after
  every edit. It is a CI job, so a red ledger blocks a release rehearsal by
  construction. 480 entries across the four files at this close.
- **Model tiering** per `proc-03-model-assignment`, with an explicit model
  parameter at EVERY dispatch: top tier for design/plan four-eyes rounds,
  whole-branch reviews and their deltas; mid tier for the controller loop,
  task implementers, task reviewers and fix dispatches; cheap tier only where
  a plan carries the work verbatim.
- **Serial vs parallel is a judgement with a handle** (doctrine): a worktree
  stream costs a setup, a merge, a full gate run and the choreography around
  both, so it earns its place only when the task's own work exceeds that.
  Doc/config packages go serial. And a serial ruling binds the CONTROLLER's
  dispatch concurrency too, not merely the plan's task order.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- master at `62e1c33`, clean, **one commit ahead of origin at the moment this
  file was written** - the session-close push follows immediately and carries
  it. Every earlier push this session had its CI conclusion watched to
  completion and logged (`proc-push-ci-conclusion-observed`); the last
  observed was run 30315908474 on `9c105d5`, SUCCESS including `ledger-lint`.
- **Three plans closed this session.** Plan 7.5 (track-rule add/remove in the
  editor) and Plan 8 (packaging/release pipeline) arrived executed but
  unclosed and were taken through their close gates. Plan 8.5 (macOS
  packaging fixes) was kicked off, planned four-eyes, executed in four serial
  tasks and closed within the session.
- **Why 8.5 existed, because it is the session's load-bearing lesson:** Plan
  8's rehearsal passed every machine-checkable acceptance item, and the
  owner's first real-hardware walk-through then found three defects, two of
  them 1.0 blockers - the macOS app did not launch at all, its dmg showed a
  mis-rendered licence dialog, and the release body's OS links broke into
  paragraphs. All three are fixed and **owner-accepted on hardware and on the
  rendered page (2026-07-28)**.
- Salvages verified in their commits: plan-7.5 36 files, plan-8 63, plan-8.5
  17. All three re-checked current against their scratch directories at this
  close (`diff -r`, the handle the doctrine gained this session).

## Next steps (priority order)

1. **The one open owner action:** delete the rehearsal draft release
   `REHEARSAL - not a release (run 30312889098)`. Recorded as an open user
   action, never booked as done - no agent in this project may touch a
   release.
2. **Plan 9 kickoff** (core/orchestration hoists + planner seam). Its design
   is NOT written. What exists is a recon inventory at
   `.superpowers/sdd/plan-9/recon-inventory.md` (1119 lines, git-ignored) and
   a RECON block in the ROADMAP's Plan 9 anchor carrying the findings that
   change what the design must decide. **Read both before briefing a
   design** - several of the anchor's long-standing figures were measured
   wrong (the duplication is 260 lines, not the "~100" the ledger carried),
   the four copies differ in seven deliberate and six accidental ways, one
   listed item may be closeable without building anything, and one seam has
   no consumer at all.
3. **Pre-1.0 gates** after Plan 9, per the ROADMAP: the README's four
   `placeholder(1.0)` comments and its WIP banner at the tag, then the at-1.0
   deliverables (single GUIDE.md, two blog posts, the requirements-catalog
   derivation), each authored in a fresh session per the recorded authoring
   pipeline.
4. **Archive duty:** this was session 23 (it ran past midnight into
   2026-07-28). The one-session-offset transcript archival applies as usual -
   **session 23 is archived by the NEXT session**; session 22 was archived at
   the start of this one.

## Open questions / risks

- **One flaky test, owner-ruled a 1.x fix** (ROADMAP "Test flakiness"):
  `dry_run_json_emits_a_document_when_the_language_query_fails` failed once
  under the full gate and passed four times afterwards, including in
  isolation. No Rust changed in the plan that surfaced it, and CI had run the
  identical code green hours earlier. The candidate fix is named and
  explicitly labelled as removing a known race class rather than as a
  confirmed fix for a cause that remains unestablished; the 1.x pass should
  state which of the two it claims.
- **Three process-doctrine amendments were adopted this session** (SI-1's
  doctrine) and mirrored into the shared collection with its manifest rows
  updated: the salvage is the LAST write of a close, with a `diff -r` handle
  at the close and session-close gates; independent verification needs
  independent INSTRUMENTS, not merely an independent context; and a parallel
  stream must earn its overhead, with the shared-index mechanism naming why
  one-tree parallelism is not made safe by staging discipline alone. **The
  latter two carry a deferred revisit** tied to a future branch-and-
  pull-request execution model, with the explicit instruction to CHECK rather
  than assume: worktree and PR isolation separates repo state, while the
  instruments at issue live outside the repo in a shared scratch directory
  that separate worktrees still share.
- **Briefs remain the one artifact four-eyes does not cover by
  construction.** Nine controller-brief defects were found downstream this
  session, every one by the agent receiving it - including a factual claim
  about macOS that Apple's own man page refuted, and a sweep claim that was
  false as phrased and true in substance. Worth watching whether the rate
  falls rather than treating it as settled.
- **Deferred to the next design-touching change:** a clause in the plan-8
  design's amendment A3 about a reviewer figure, that amendment's region
  mapping, and two report-internal citations that do not reproduce. The exact
  replacement text for a dangling `ci.yml` comment citation rides the
  ROADMAP's v1.x mise-removal entry, gated on the next `ci.yml`-touching
  change whichever it turns out to be.
- Framework-side follow-ups are tracked agent-side.
