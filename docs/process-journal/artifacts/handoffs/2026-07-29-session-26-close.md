# Handoff

<!-- Provenance: snapshot of HANDOFF.md at the session-26 close (2026-07-29), superseding the plan-9-close snapshot, per SI-5. -->

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-29 (session 26 close)
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
GUI, MIT, public). **Next milestone: 1.0.** **Plans 1 through 9 are CLOSED** -
plan 9 was the last planned work package before the pre-1.0 gates. The
ROADMAP's "Pre-1.0 release gates" section is the next initiative.

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict.

- **The gate is ELEVEN parts** per BUILDING.md, foreground, no subsets, before
  any push, with no docs-only exception. It gained two things at the plan-9
  close: `python3 scripts/ledger-lint.py` as a gate part rather than a
  side-duty, and `--document-private-items` on the rustdoc step at BOTH
  consuming sites. `BUILDING.md` is the single authoritative enumeration and it
  now comes to eleven (six Rust, four frontend, one house-knowledge); anything
  deriving a count says "per BUILDING.md" and must agree with that file.
- **Model tiering, owner ruling 2026-07-28**: the top model serves ONE role,
  the plan-close whole-branch review and its delta re-reviews. Design and plan
  four-eyes rounds, decision documents, task implementers, task reviewers, fix
  dispatches and recon all run the mid tier; plan-carried transcription runs
  the cheap tier. Every dispatch names its model explicitly.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip, and never by a script anchored on a repeated
  key pair - anchor on the entry's `- id:` line, and reproduce that line if the
  anchor spans an entry boundary. 516 entries at this close. **No task edits
  these files** - the controller is the single writer. Run `ledger-lint` after
  every batch edit and before every commit; it caught two structural edit slips
  this session within seconds.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- master at the close-pass fix round plus the controller's own follow-up
  writes, clean. Several commits unpushed at the time of writing; the
  session-close push runs the full eleven-part gate first.
- **Plan 9 is CLOSED.** Seven tasks, five amendments, four task fix rounds and
  a whole-branch fix round. The whole-branch review (top tier) returned
  NEEDS_FIXES on two rustdoc claims that spec amendment S-8 had falsified, and
  **READY** after the fix wave.
- Close actions DONE: promotion sweep of the five owner-ruled entries into
  their nature files (two needed a tense rewrite); `core-121` settled;
  `core-d49-g1g2-experiment` written with a controller-composed statement that
  discloses itself as such; blocked-pool sweep over 18 entries (none settled,
  one premise-stale and corrected); ROADMAP anchor marked executed with its IN
  items named against their commits and both D49 triggers dispositioned; SDD
  salvage of 53 files, count verified in the commit, `diff -r` clean; journal
  entry.

## Next steps (priority order)

1. **Pre-1.0 gates** per the ROADMAP's "Pre-1.0 release gates" section. Plan 9
   was the last planned work package before them, and its close is complete:
   the text-corrections pass and both BUILDING.md gate edits landed in
   `9dc3a4d` with their fix round `c8dfc6d`, each independently reviewed.
2. **Archive duty:** session 26 is archived by the NEXT session; session 25 was
   archived at the start of this one.

## Open questions / risks

- **Both owner questions from this close are RULED.** The D49 question stays
  open until after 1.0 rather than being re-run now, and the registered trigger
  was re-aimed at the event that would make it answerable: a re-fenced
  experiment mutating the applier site only. Until such a run happens all three
  guards stay. Nothing is scheduled for it.
- **The controller's own error class stayed the most frequent, again.** Two
  claims of mine were refuted downstream by measurement (a commit count in a
  review brief, a ledger occurrence that widened a literal-phrase measurement
  into a claim about the class), and two YAML edits of mine damaged the house
  files structurally and were caught by `ledger-lint` seconds later. The rate
  is the thing to watch; the lint is doing its job.
- **A defect class this plan hit twice: a design's substance right, its stated
  form impossible.** Amendment 4's pinned invocation, amendment 5's bearerless
  assertion. Both were caught on code contact by the executing agent, neither
  by design review. Ledger
  `a-state-assertion-presupposes-the-control-renders-so-walk-the-render-chain`.
- **One flaky test, owner-ruled a 1.x fix** (ROADMAP "Test flakiness"):
  `dry_run_json_emits_a_document_when_the_language_query_fails`. It has not
  reappeared.
- Framework-side follow-ups are tracked agent-side.
