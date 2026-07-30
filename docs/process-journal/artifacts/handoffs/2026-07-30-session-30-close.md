# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-30 (session 30 CLOSED; Plans 11 and 11.5 executed, closed and pushed; Plan 12 next, in a fresh session)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1 through 11 are closed.
**Plan 12 is a FULLY APPROVED contract - owner and independent review, amendment
included - and has NOT been executed.** It is the next work.

## The gate that changes what "done" means

**Owner ruling, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no 1.0 release is
cut before Şenol has personally run a manual QA and bug-hunting pass on his own
hardware.** Its output is first-class scope input in three shapes he named: real
bugs; behaviour he dislikes even where it matches the spec; and v1.x items he
decides belong in 1.0 after all. All three shapes have occurred - see the
ROADMAP's "OWNER QA PASS, round 3" entry, the authoritative record of that pass.

**His pass is STOPPED and cannot resume on any build that exists.** Round 3, on
Windows, found that the GUI can only OPEN a profile and never create one, so
nothing behind a profile is reachable. **Plan 12 is what unblocks it.** Until
that pass completes, 1.0 scope is unknown by construction and no completeness
claim about 1.0 may be made.

**Timing, owner-ruled 2026-07-30: the pass comes AFTER PLAN 12.**

## Current state (verified)

Re-derive rather than trusting these lines: `git log --oneline -1`,
`git status`, `git rev-list origin/master..master`.

- **Plan 11 executed, closed and pushed** (`5378264..71cce6a`, plus the close
  commits after it). Two streams in separate worktrees, merged A then B, both
  worktrees torn down.
- **The full eleven-part gate ran three times** - stream A's own worktree, the
  state after merging A, the state after merging B - with each part's exit code
  captured separately rather than trusting an aggregate. All green: **507 Rust
  tests over 39 suites, 68 e2e cases, zero failures.** The count is an
  independent check on the behaviour change: 505 before, +2 for the two added
  tests, the third having replaced an existing one in place.
- **The `postcss` alert CLEARED on the push.** One alert remains open, `glib`
  `GHSA-wrw7-89jp-8q8g`, and it stays open by decision: the advisory is IGNORED
  with its reason and drop condition recorded in `deny.toml`, not fixed.
  Dismissing it is an owner action and was not taken.
- **`cargo deny check` now evaluates the unsound advisory class** at scope `all`,
  which it did not before. A change in what a gate part covers is owner-visible
  even when the owner ordered it.
- **`raw:` compares without type conversion** as of this plan (ADR D111). The
  typed `exact` path still cross-compares int and float, and one test exists
  solely to catch a future change that strips that.
- **House knowledge is at 566 entries**, up from 560 at session start;
  `ledger-lint` green. Five new entries and eight occurrences were mined at
  verdict arrival.
- **Plan 11's SDD scratch is SALVAGED** to `docs/process-journal/artifacts/plan-11-sdd/`,
  47 files verified in the commit, `diff -r` clean after it.
- **Plan 11.5 executed and closed after Plan 11**, one task: the owner ruled in
  `unused-ignored-advisory = "deny"` in `deny.toml`, so an ignore entry that
  matches nothing fails the check instead of sitting there suppressing nothing.
  His reason, which replaced the controller's framing: we want security-relevant
  findings in transitive dependencies too, not only in our direct ones. **The
  practical consequence to know before it happens:** when `glib` is eventually
  fixed or leaves the tree, `cargo deny check` fails naming
  `RUSTSEC-2024-0429` at its exact line. **That is the tool asking for the
  obsolete ignore entry to be deleted - the correct reaction is to delete those
  lines, never to revert the key.** Both `deny.toml`'s own comment and the
  ROADMAP trigger entry say so. Renovate's first dependency PRs (expected
  2026-08-01 to 08-03) are the most likely cause.
- **The full gate ran a fourth time** on the Plan-11.5 state, all eleven parts
  green, 507 Rust tests.

## What Plan 12 needs before its first dispatch

**CONTROLLER OBLIGATION, created by the owner-ruled ordering and NOT yet
discharged:** Plan 12 was authored and measured against the tree BEFORE Plan 11
landed, and both plans amend the v1 spec (Plan 11 in 4.3, 4.4, 7, 8.1 and 9.2;
Plan 12 in 8.2). So before its pre-execution gate: **re-verify that every fenced
OLD string in Plan 12 still occurs exactly once, and re-run every tree-measured
figure in it.** Different sections means no textual conflict is EXPECTED;
expecting is not measuring. Plan 11 supplies the sharper reason: its own fenced
placement instruction, correct when written, became unperformable against the
state a later step actually held, and two reviewers had to fire both readings to
establish it.

Plan 12's document: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`.
Its execution method is serial tasks in ONE worktree, per its own document.

## Owner decisions: BOTH RESOLVED at the session close, nothing waits on him

1. **The `cargo deny` guard key: RULED IN and built** (Plan 11.5, `937ae42`).
2. **The `raw:` comparison vocabulary: no work owed, and it was never a real
   question.** The controller had presented the seven retained wordings as an
   open owner decision on the strength of a reviewer's classification it had not
   read itself. Read at the artifact: every one of the seven names its property
   in its own sentence (`language`, `codec_kind`, both strings), and the two
   user-visible diagnostics interpolate the property name at runtime. Nothing
   anywhere claims `raw:` is byte-exact in general - those were the twelve sites
   Plan 11 repaired. The owner's standard (no formulation may be false standing
   alone) is met.

## Open questions / risks

- **Four controller defects this session, every one caught downstream and none by
  the controller**: a brief instruction that would have discarded a deliverable;
  an unmeasured set-size claim in an amendment brief; both merge commits written
  without the mandatory trailer; and a mechanism claim borrowed from a subagent's
  first report and written into the ledger unverified. The pattern is stable
  across sessions and the mechanisms that catch it are working.
- **Two pasted grep outputs in one task report were not what their commands
  return**, one labelled as a full enumeration over a list short by one. The
  claims were independently true, so nothing failed. The whole-branch review
  spot-re-ran the load-bearing pastes of the other tasks and found the pattern did
  not extend to them.
- **A harness trap worth knowing before it costs a session:** `grep` here is a
  wrapper that skips binary files, so ONE NUL byte anywhere in a file makes every
  later search over it return nothing at exit 1 - byte-identical to an honest
  no-match. The cheap handle is one `file` call after generating report text
  through the shell. Ledger: `echo-expands-what-printf-passes-through`.
- **Renovate's first dependency PRs are expected 2026-08-01 to 08-03.** When they
  land, walk the commented RUSTSEC ignores in `deny.toml` and drop the ones they
  obsolete, and take the TypeScript-7 bump when the typescript-eslint ceiling
  allows. The count is deliberately unstated in the tracker.
- **Six worktrees from Plans 7.5 and 8 were never torn down.** All six have clean
  trees and all six branches are ancestors of master. Left in place; the safe
  form is `git worktree remove` for the six while keeping their branches.
- Framework-side follow-ups are tracked agent-side.
