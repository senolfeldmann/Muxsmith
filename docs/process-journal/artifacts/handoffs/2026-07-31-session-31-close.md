<!-- Snapshot of HANDOFF.md at the session-31 close (Plan 12 tasks 1-3 of 7 closed). The live HANDOFF is git-ignored and superseded in place; this is its committed state per SI-5. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-31 (session 31 CLOSED; Plan 12 tasks 1-3 of 7 executed, reviewed and closed; tasks 4-7 next, in a fresh session)
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
**Plan 12 is IN EXECUTION: tasks 1, 2 and 3 of 7 are closed; tasks 4 to 7
remain.**

## The gate that changes what "done" means

**Owner ruling, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no 1.0 release is
cut before Şenol has personally run a manual QA and bug-hunting pass on his own
hardware.** Its output is first-class scope input in three shapes he named: real
bugs; behaviour he dislikes even where it matches the spec; and v1.x items he
decides belong in 1.0 after all. All three shapes have occurred - see the
ROADMAP's "OWNER QA PASS, round 3" entry, the authoritative record of that pass.

**His pass is STOPPED and cannot resume on any build that exists.** Round 3, on
Windows, found that the GUI can only OPEN a profile and never create one, so
nothing behind a profile is reachable. **Plan 12 is what unblocks it, and task 3
built the New action** - but the pass resumes on a build carrying the whole
package, not on a mid-plan state. Until that pass completes, 1.0 scope is
unknown by construction and no completeness claim about 1.0 may be made.

**Timing, owner-ruled 2026-07-30: the pass comes AFTER PLAN 12.**

## Current state (verified)

Re-derive rather than trusting these lines: `git log --oneline -1`,
`git status`, `git rev-list --count origin/master..master`.

- **Working tree clean; the branch is ahead of `origin/master` and nothing has
  been pushed this session.** The single push is a plan-close action per Plan
  12's own close section; the work is durable in git meanwhile. **No count is
  stated here on purpose:** the close's own commits - this snapshot among them -
  change it, so any number written into this file is falsified by the act of
  committing the file. Count it with `git rev-list --count origin/master..master`
  when you need it.
- **Plan 12 tasks 1-3 closed**, each with an independent review and its fix
  rounds: task 1 (the spec amendment plus ADRs D106-D110) one fix round; task 2
  (the three-state language control) two; task 3 (New creates a blank profile)
  two. Every delta re-review went to the RESUMED original reviewer.
- **The full eleven-part gate is green on the committed state** - 507 Rust tests
  over 39 suites, 79 e2e cases, all eleven parts exit 0, exit codes captured per
  command rather than through a pipeline.
- **House knowledge is at 570 entries**, up from 566, `ledger-lint` green. One
  entry was PROMOTED to tier 2 on its third strict-fit event
  (`gitignored-paths-need-command-grep`).
- **Plan 12's scratch holds 32 artifacts** including two controller rulings and
  one parked owner-decision memo. Not yet salvaged - salvage is a plan-close
  action and this plan is mid-flight.

## What tasks 4 to 7 inherit

- **Task 4 (undo/redo) and task 5 (the discard guards) both build on task 3's
  `createBlank`.** Two constraints are MEASURED and must not be re-derived from
  the plan's prose, which is wrong about both: the statement order inside the
  body is NOT load-bearing (both orderings measured green), but the RELATIVE
  order of the session gate and the model assignment IS - the gate goes first,
  which makes the function await-proof, and the reverse order fails three cases
  as soon as an `await` lands between them. Task 5 makes that function async.
  The shipped comment on `createBlank` carries the measurements.
- **A forward note from task 3's reviewer, not measured and not a finding:**
  making that funnel async also makes concurrent entry possible, and the
  existing busy guard does not cover a second call into it. Task 5's ground.
- **Tasks 4 and 5 append to the same catalogs under a placement clause that is
  under-determined** once a further section sits between; their dispatches carry
  an explicit placement rather than inheriting the ambiguity.
- **The plan's own counts diverged from its enumerations twice** in task 3's
  brief (four seeds where its source enumerates five; seven tests where the step
  enumerates six). Where a count and an enumeration disagree, the enumeration
  governs - and later tasks should expect the same shape.

## Owner decision PARKED, not blocking

**What the editor shows after a profile fails to parse.** It currently displays
"Selected profile: `<path>`" and "no profile open" at the same time, and the
recents list - hidden after any open before task 3 - comes back. Three options
are costed in the plan scratch's `owner-decision-failed-load-empty-state.md`.
The deciding measurement: the rendered parse error carries a detail and NOT the
file path, so the path line is the only place the failing file is named. Task 5
touches the same surface and is the natural vehicle. Recommendation on record:
keep the path line, hide the other two after a failed open.

## Open questions / risks

- **Two plan defects reached execution through four eyes and an owner
  approval**, both of a class reading cannot catch: a fenced code block that
  does not type-check, and a prescribed test case that is always-red against a
  correct implementation because the fixture cannot reach the state it asserts.
  Both were found by implementers at code contact and routed rather than
  patched. Expect the same class in tasks 4 to 7, which carry more fenced code
  than the first three.
- **Three controller defects this session, each found downstream:** an inverted
  consequence that propagated into a committed source comment citing the ruling
  as its authority; a review package built over a range containing the
  controller's own commit, which would have shown a reviewer a house-knowledge
  file inside a task's file list; and a build's exit status read through a
  pipeline one hour after that exact trap was recorded in the ledger.
- **Renovate's first dependency PRs were expected 2026-08-01 to 08-03** and no
  open PR exists as of this close. When they land, walk the commented RUSTSEC
  ignores in `deny.toml` and drop the ones they obsolete, and take the
  TypeScript-7 bump when the typescript-eslint ceiling allows. The count is
  deliberately unstated in the tracker.
- **`cargo deny check` will fail naming `RUSTSEC-2024-0429` at its exact line**
  once `glib` is fixed or leaves the tree. That is the tool asking for the
  obsolete ignore entry to be deleted - delete those lines, never revert the
  `unused-ignored-advisory` key. Both `deny.toml`'s comment and the ROADMAP
  trigger say so.
- **Six worktrees from Plans 7.5 and 8 were never torn down.** All six have
  clean trees and all six branches are ancestors of master. The safe form is
  `git worktree remove` for the six while keeping their branches.
- Framework-side follow-ups are tracked agent-side.
