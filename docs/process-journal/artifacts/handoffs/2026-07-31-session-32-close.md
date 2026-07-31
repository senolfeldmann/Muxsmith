# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-31 (session 32 CLOSED; **Plan 12 EXECUTED AND CLOSED**, all seven tasks, two amendments, whole-branch review READY, pushed. **QA artifacts built and waiting for the owner's round-4 pass, which runs between sessions.**)
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

**Addition, session 29 (2026-07-30), from a defect this duty produced:** a
borrowed precedent carries the CONDITIONS that licensed it, and the commonest
silently-dropped condition is synchronicity. Plan 12 cited mkvtoolnix marking
clean state at completion; that is sound there only because the handler is
fully synchronous, and the translation has two awaits with a live editing
surface, so the same line would have marked the editor clean against a profile
the write never used. When citing a precedent, state the condition that
licensed it, not only its shape. Ledger:
`a-synchronous-parity-precedent-loses-its-licence-when-translated-into-async`.

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
written as a literal in a plan or brief. **A MERGE COMMIT IS AN AGENT COMMIT
AND CARRIES THE TRAILER TOO** - added session 30, after both of Plan 11's
merge commits were written without it and the whole-branch review found them;
they were not rewritten, because the SHAs were already cited in the tracker,
the ledger and the journal. **Two writers in one working tree
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
GUI, MIT, public). **Next milestone: 1.0. Plans 1 through 12 are closed. No
plan is in execution.**

## The gate that decides what happens next

**Owner ruling, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no 1.0 release is
cut before Şenol has personally run a manual QA and bug-hunting pass on his own
hardware.** Its output is first-class scope input in three shapes he named: real
bugs; behaviour he dislikes even where it matches the spec; and v1.x items he decides
belong in 1.0 after all.

**His round-3 pass stopped on Windows because the GUI could open a profile and never
create one, so nothing behind a profile was reachable. Plan 12 was the package that
unblocks it, and it is now closed and pushed - so the pass can resume on a build
carrying the whole package.** That resumption is the next thing this project does;
Plan 13 must NOT be authored before it returns, because that plan's scope is
deliberately open until the pass is done (owner ruling 2026-07-30).

**The build he needs exists and was made at his request at this session's end.** Draft
`rehearsal-30620332948`, run 30620332948, six of six jobs green, eight assets (two msi,
one dmg, deb/rpm/AppImage/tar.gz, SHA256SUMS). Produced through the pipeline's
by-hand path (`workflow_dispatch` with the rehearsal switch), which is the route Plan 8
designed for this and which deliberately creates NO tag. **Its head SHA was compared
against master rather than assumed: both `1b2d623`.** He runs QA round 4 between
sessions and brings the findings to the Plan-13 kickoff.

**The first thing that pass checks is whether New profile exists and puts an editable
profile in the editor**, because that is the precondition every remaining round-3 item
was blocked on.

## Current state (verified)

Re-derive rather than trusting these lines: `git log --oneline -1`,
`git status`, `git rev-list --count origin/master..master`.

- **Working tree clean, master pushed, nothing local ahead.** The plan-close push
  happened at this close; before it the branch carried 58 local commits over two
  sessions, which was deliberate (Plan 12 reserved its single push as a close action).
- **All seven Plan-12 tasks closed**, each with an independent review and its fix
  rounds, plus **two four-eyes plan amendments** (three fix rounds between them) and a
  **whole-branch review that returned NEEDS_FIXES with eight blocking items, then READY
  after one fix wave**. Every delta re-review went to the RESUMED original reviewer.
- **The full eleven-part gate is green on the pushed state** - 507 Rust workspace
  tests, 88 in the GUI crate, 103 Playwright e2e cases, all eleven parts exit 0.
- **House knowledge is at 586 entries**, `ledger-lint` green, up from 566 at the start
  of session 31. One entry was promoted to tier 2 on its third strict-fit event; one
  tier-1 entry written this session was later REFUTED by measurement and rewritten,
  with the refutation kept as a dated occurrence.
- **Plan 12's SDD workspace is salvaged** to `docs/process-journal/artifacts/plan-12-sdd/`,
  83 artifacts, count verified in the commit rather than in the working tree.

## What the next session inherits

- **Nothing is mid-flight.** No task is open, no agent needs resuming, no fix loop is
  running.
- **The next session opens with the QA round-4 findings**, which are Plan 13's scope
  input. Take them first; the plan is authored after them, never before.
- **OPEN OWNER ACTION, not done and not bookable by an agent:** the rehearsal draft
  `rehearsal-30620332948` is a throwaway artifact and is meant to be deleted once the
  QA pass is finished. Deleting it is a Releases-page action and involves no tag. Two
  older rehearsal drafts from 2026-07-29 are still there as well and are equally
  disposable.
- **Two acceptance-map rows in the retired Plan 12 are knowingly false**, with their
  dispositions recorded in the ROADMAP's Plan-12 close block and in the journal. Neither
  is a coverage gap; both were measured.
- **Plan 13's floor grew to five members**; its scope stays open until the QA pass
  returns.
- **Two new triggers** are registered in the Triggers section: a second caller of the
  editor's confirm component (it has no reentrancy guard, unreachable today by real
  input), and a test needing save-then-reopen without a page reload.
- **Two docs-accuracy items** on editor widget comments, one of them a dead design
  premise rather than a stale figure.
- **Six worktrees from Plans 7.5 and 8 were never torn down.** Verified this session:
  all six branches are ancestors of master. The safe form is `git worktree remove` for
  the six while keeping their branches.
- **Renovate's first dependency PRs** were expected 2026-08-01 to 08-03 and none exists
  as of this close (checked with a control). When they land, walk the commented RUSTSEC
  ignores in `deny.toml` and drop the ones they obsolete, and take the TypeScript-7 bump
  when the typescript-eslint ceiling allows. The count is deliberately unstated.
- **`cargo deny check` will fail naming `RUSTSEC-2024-0429` at its exact line** once
  `glib` is fixed or leaves the tree. That is the tool asking for the obsolete ignore
  entry to be deleted - delete those lines, never revert the `unused-ignored-advisory`
  key. Both `deny.toml`'s comment and the ROADMAP trigger say so.
- Framework-side follow-ups are tracked agent-side.

## Two process facts worth carrying, both earned this session

- **A controller ruling that adds a member to a set falsifies every enumeration of that
  set, and the controller who rules it owns the sweep.** One such miss cost a full
  plan amendment with two fix rounds.
- **A sweep answers "what else says this"; only a pair comparison answers "do the two
  things I just wrote still agree".** Where one statement lives in two documents, no
  expression can find the disagreement - both halves are present and only the content
  differs.
