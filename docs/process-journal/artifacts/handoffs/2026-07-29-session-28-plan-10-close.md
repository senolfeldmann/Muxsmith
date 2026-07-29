<!-- Snapshot of HANDOFF.md at the session-28 close (Plan 10 executed and closed). The HANDOFF itself is git-ignored and superseded in place; SI-5 requires this snapshot in the same turn as the rewrite. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-29 (session 28 close, Plan 10 closed)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1 through 10 are closed.
**No further product plan is scheduled.** What stands between here and the tag
is listed under "Next steps", and the first item is not a plan.

## The gate that changes what "done" means

**Owner ruling 2026-07-29, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no
1.0 release is cut before Şenol has personally run a manual QA and bug-hunting
pass on his own hardware.** Its output is first-class scope input in three
shapes he named: real bugs; behaviour he dislikes even where it matches the
spec; and v1.x items he decides belong in 1.0 after all.

Round 1 covered the install paths only - three OSes installed and launched, the
documented steps and SHA commands confirmed, one finding (now documented). **The
product itself is still untested by him**: a real dry-run and run over his own
library, the profile editor including rule add and remove, suggestion apply, the
jobs view during a live batch with a mid-run cancel, run history, the locale
switch, help mode. **Until that pass has run, 1.0 scope is unknown by
construction, and no completeness claim about 1.0 may be made** - however short
the remaining list looks. His timing call: the full pass comes once the next
plan is implemented, which is now.

A build for it exists as a rehearsal draft on `a5b63ba` (`workflow_dispatch` on
release.yml with the draft flag; never a tag, never published). A fresh draft
build off current master would be more useful, since Plan 10 changed
`docs/INSTALL.md` and the README.

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict.

- **The gate is what `BUILDING.md` enumerates** - foreground, no subsets, before
  any push. Since Plan 10 Task 1 the file STATES its own total behind a
  `gate-total` marker and `scripts/ledger-lint.py` checks that statement against
  the commands its three `gate-block`-marked blocks enumerate. Anything deriving
  a count still says "per BUILDING.md" and must agree with that file. Two
  boundaries of that check are recorded as ROADMAP triggers: a FOURTH marked
  gate block would be invisible to it, and a command wrapped with a trailing `|`
  or `&&` is not modelled (backslash continuations are, and it refuses on them).
- **Model tiering, owner ruling 2026-07-28**: the top model serves ONE role, the
  plan-close whole-branch review and its delta re-reviews. Everything else -
  design and plan four-eyes rounds, decision documents, task implementers, task
  reviewers, fix dispatches, recon - runs the mid tier; plan-carried
  transcription runs the cheap tier. Every dispatch names its model explicitly.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip, and never by a script anchored on a repeated
  key pair - anchor on the entry's `- id:` line. 545 entries at this close.
  **No task edits these files** - the controller is the single writer. Run
  `ledger-lint` after every batch edit and before every commit.
- **A comment never locates code by line number** (owner ruling 2026-07-29,
  Tier-2 `comments-locate-by-symbol-never-by-line-number`): name the symbol.
  Naming the file stays fine. Plan 10 Task 5 swept the corpus; see the open
  question below about what the ruling's scope covers.
- **A document never cites a line number inside ITSELF** (owner ruling
  2026-07-29, Tier-2 `a-document-never-cites-a-line-number-inside-itself`).
- **Two writers in one tree share one index** (SI-4). This close violated it
  four times and got away with it; the rule is `git commit -- <paths>`, and a
  reviewer's byte-identity proof is per file against blobs, never a clean
  `git status`.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- **master at `ab0a6a3` plus this close's remaining commits, nothing unpushed**
  (checked with `git status` and `git rev-list origin/master..master`). The
  eleven-part gate ran green before the push with every part's exit code
  captured separately; the push-triggered CI run concluded **success** on all
  five jobs - the three OS legs, `deny`, and `ledger-lint`, which now also
  carries the new gate-count check.
- **Plan 10 is EXECUTED AND CLOSED.** Five serial tasks in one tree, each with a
  fresh implementer and an independent reviewer; one task fix round; a
  whole-branch review on the top tier returning READY_WITH_MINORS; one close fix
  wave whose three findings were all verdicted ADDRESSED by the resumed
  reviewer. Archive: `docs/process-journal/artifacts/plan-10-sdd/` (32 files,
  count verified in the commit and in the index, re-salvaged after the close fix
  wave).
- **What landed:** the gate-count invariant in `BUILDING.md` +
  `scripts/ledger-lint.py`; the two D102 preserved-order producers, selected by a
  four-mutation measurement rather than assumed; `renovate.jsonc`;
  the user-facing documentation pass over `README.md` and `docs/INSTALL.md`; the
  comment line-citation sweep (24 lines across 16 files, 21 comments).
- **Two vulnerability alerts are still OPEN** with their own ruled one-task
  vehicle in the ROADMAP's pre-1.0 gates, unscheduled against Plan 10 on purpose.
  The `cargo deny` / GitHub disagreement under them is still unmeasured, and
  until it is, neither mechanism may be quoted as coverage.
- **House knowledge is at 545 entries**, up from 531 at the session start. One
  entry promoted to Tier 2 on its third occurrence.

## Next steps (priority order)

1. **The owner's full product QA pass.** It is what actually closes 1.0 scope,
   his timing condition ("once the next plan is implemented") is now met, and
   nothing else on this list gates the tag the way this does. It needs a current
   build on his hardware; producing that draft build is the preparatory step.
2. **The vulnerability vehicle** (ROADMAP, pre-1.0 gates): its own one-task
   plan. Bump `postcss` past 8.5.17 through the lockfile; MEASURE the
   `cargo deny` disagreement in the same task rather than restating the
   hypothesis; INVESTIGATE `glib` only, and if it cannot move independently of
   Tauri's tree, say so and give it its own vehicle.
3. **Renovate activation: two OWNER actions, in order** - the config is already
   on `master`, which is what suppresses the vendor's onboarding PR; then
   install the app against this repository, and enable the dependency graph and
   Dependabot alerts. Whoever observes activation confirms Renovate actually
   starts (the observable is a dependency-dashboard issue appearing); the
   documented fallback is renaming the closed onboarding PR. The ROADMAP trigger
   stays NOT FIRED until then.
4. **Three routed items waiting on a package that touches their file**: the
   surviving `ci.yml` line-number citation, the `raw:` "byte-exact" wording (in
   the spec, the matcher comment and the README at once), and the v1 spec's 8.1
   synopsis omitting `validate`'s flags. All three carry vehicles in the
   ROADMAP's "Docs accuracy" section.
5. **Archive duty:** session 28 is archived by the NEXT session; session 27 was
   archived at the start of this one.

## Open questions / risks

- **THREE OWNER QUESTIONS are waiting, all recorded in the tracker so they
  cannot evaporate.** (a) Does the comment line-number ruling reach CI and
  config comments? It was scoped to source comments and explicitly not widened,
  and its form list names `//`, `///`, `//!`, `/* */` but not `#` - which is
  where the one surviving member sits. (b) The runner-image exclusion in
  `renovate.jsonc` (`github-runner` disabled, on D85's glibc-floor grounds) came
  from the plan fence rather than from a named ruling of his, and
  `ci-04-dependabot-cadence` never mentioned it; recorded as an occurrence
  pending his confirmation. (c) The count-versus-salvage treadmill: repairing
  the README's verdict figure re-arms it, since the next plan's salvage
  falsifies the new number identically. Two options are on the table - a
  standing re-measure duty at the salvage step, which the whole-branch reviewer
  recommends, or growth-proof phrasing, which is a register call on his README.
- **The comment-citation class is closed WITHIN its corpus selector, not
  tree-wide.** Any sentence saying Plan 10 closed that class names the selector -
  source files in six extensions - or it is false.
- **The controller's own error class stayed the most frequent, again**, and again
  every instance was caught by something else: four commits into a tree with a
  live product writer, using `git add` plus a bare commit rather than the
  pathspec-scoped form; a review brief demanding byte-identity against a commit
  the controller then moved; one verdict harvest mined after the next dispatch
  instead of before it; and one occurrence written from recall (one commit) that
  a measurement corrected to four. All four are ledgered.
- Framework-side follow-ups are tracked agent-side.
