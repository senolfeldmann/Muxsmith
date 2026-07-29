<!-- Snapshot of HANDOFF.md at the final session-27 close (SI-5). -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-29 (session 27 close)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1 through 9 are closed.
**Plan 10, the pre-1.0 product package, is AUTHORED AND FOUR-EYES APPROVED but
NOT EXECUTED** - executing it is the next work.

## The gate that changes what "done" means

**Owner ruling 2026-07-29, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no
1.0 release is cut before Şenol has personally run a manual QA and bug-hunting
pass on his own hardware.** Its output is first-class scope input in three
shapes he named: real bugs; behaviour he dislikes even where it matches the
spec; and v1.x items he decides belong in 1.0 after all.

Operative consequence, and the part that is easy to forget: **finishing Plan 10
does not close 1.0 scope**, and no completeness claim about 1.0 may be made
before that pass has run. A build for it exists as a rehearsal draft on
`a5b63ba` (`workflow_dispatch` on release.yml with the draft flag; never a tag,
never published), with all seven per-OS artifacts plus SHA256SUMS.

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict.

- **The gate is what `BUILDING.md` enumerates** - foreground, no subsets, before
  any push, no docs-only exception. `BUILDING.md` is the single authoritative
  enumeration; anything deriving a count says "per BUILDING.md" and must agree
  with that file. Plan 10's Task 1 makes the file state its own total and adds a
  check for it, so the count moves under that task - name the file, never a
  figure.
- **Model tiering, owner ruling 2026-07-28**: the top model serves ONE role, the
  plan-close whole-branch review and its delta re-reviews. Design and plan
  four-eyes rounds, decision documents, task implementers, task reviewers, fix
  dispatches and recon all run the mid tier; plan-carried transcription runs the
  cheap tier. Every dispatch names its model explicitly.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip, and never by a script anchored on a repeated
  key pair - anchor on the entry's `- id:` line. 527 entries at this close.
  **No task edits these files** - the controller is the single writer. Run
  `ledger-lint` after every batch edit and before every commit; it refused a
  commit this session for a count not bumped alongside an appended occurrence.
- **A comment never locates code by line number** (owner ruling 2026-07-29,
  Tier-2 `comments-locate-by-symbol-never-by-line-number`): name the symbol.
  Naming the file stays fine. Plan 10's Task 5 sweeps the existing 24 sites.
  Scoped to SOURCE comments by his ruling the same day, and NOT widened.
- **A document never cites a line number inside ITSELF** (owner ruling
  2026-07-29, Tier-2 `a-document-never-cites-a-line-number-inside-itself`): name
  the container - sentence, table row, fenced block, section. Chosen over an
  update duty on his reasoning that updating gets forgotten, i.e. a rule
  requiring someone to notice is decoration. Citations to ANOTHER file at a
  named commit stay permitted; there the moment is part of the claim.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- **master at `b72d6a7` plus the session-close commit**, nothing unpushed
  (checked with `git status` and `git rev-list origin/master..master`, not from
  memory). Gate green before every push except two doc-only commits that went
  out before it ran; those were gated retroactively green and the lapse is
  recorded in the ledger, because a retroactive green is not the same evidence
  as a gate that ran first.
- **Two vulnerability alerts are OPEN** and the gate disagreement under them is
  the part that outranks both: `cargo deny check` is gate part 5 and is GREEN on
  this tree while GitHub reports a Rust advisory that `deny.toml` does not
  silence. **Until that is measured, neither mechanism may be quoted as
  coverage.** Details and the ruled vehicle are in the ROADMAP's pre-1.0 gates.
- **House knowledge is at 531 entries**, up from 517 at the session start.
  Almost every new one is about the same failure: when a check asserts something
  it did not measure. Two of them are owner-ruled Tier-2 conventions.
- **Plan 10 is authored, reviewed, approved and twice amended**:
  `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`. Five serial
  tasks, one tree, no worktrees. Task 1 the canonical gate total and its check;
  Task 2 the D102 preserved-order producers, measurement-gated across four
  contract halves; Task 3 `renovate.jsonc`; Task 4 the user-facing documentation
  pass; Task 5 the comment line-citation sweep. The 4 -> 5 edge is hard: three
  sweep sites cite `README.md` spans that Task 4 edits.
- **Both amendments are post-approval owner rulings, one pair each** (they add
  steps to Task 4, they do not re-cut the task set). Amendment 1 folds in the
  two false counts in the README's process paragraph. Amendment 2 documents the
  Fedora warning his QA pass found; it renamed Task 4 from a README pass and
  added `docs/INSTALL.md` to its files.
- **The package has no design document**, owner-approved. The controller brief
  at `.superpowers/sdd/plan-10/plan-brief.md` stands in as coverage ground
  truth; the plan says so in its header.
- Review record in `.superpowers/sdd/plan-10/plan-review-round-1.md`: round 1
  (8 Important, 6 Minor, coverage complete) plus four delta rounds. Every round
  found something, including the last.

## Next steps (priority order)

1. **Execute Plan 10. The owner approved it and gave the go on 2026-07-29.**
   Five serial tasks, one tree, no worktrees, fresh implementer and independent
   reviewer per task, per the plan's own model-tier table. Start in a FRESH
   session: the context that authored it is spent. Nothing else blocks this.
2. **The vulnerability vehicle** (ROADMAP, pre-1.0 gates): its own one-task
   plan, owner-ruled rather than a Plan-10 rider. Bump `postcss` past 8.5.17
   through the lockfile; MEASURE the `cargo deny` disagreement in the same task
   rather than restating the hypothesis; INVESTIGATE `glib` only, and if it
   cannot move independently of Tauri's tree, say so and give it its own
   vehicle. Unscheduled against Plan 10 on purpose - neither touches the other's
   files.
3. **Renovate: both owner actions are DONE and the onboarding PR is closed**
   (PR #1, 2026-07-29). The alert feed is live and already produced the two
   findings above. **One check remains, and it belongs to whoever lands Task 3:**
   the vendor documents a config commit as a route to onboarding but does not
   say in so many words that it overrides an ALREADY-CLOSED onboarding PR. After
   `renovate.jsonc` reaches master, confirm Renovate actually starts - the
   observable is a dependency-dashboard issue appearing. If it stays silent, the
   documented fallback is renaming the closed PR.
4. **The owner's full product QA pass**, which is what actually closes 1.0
   scope. He ruled the timing on 2026-07-29: it comes later, probably once the
   next plan is implemented. Round 1 covered install paths only; the product is
   untested. Until that pass has run, 1.0 scope is unknown by construction.
5. **Archive duty:** session 27 is archived by the NEXT session; session 26 was
   archived at the start of this one.

## Open questions / risks

- **Both owner questions from the close are RULED** and both landed as changes:
  the README counts came into Task 4 as amendment 1, and the locate-by-symbol
  ruling stays scoped to source comments while self-citation inside one document
  became its own Tier-2 prohibition.
- **The README's review count is a unit trap, and the plan defends against it by
  mechanism rather than by warning.** Measured: at the commit that introduced the
  sentence both candidate units returned 78, and they forked later when plans
  stopped using the `verdicts/` subdirectory. Today it is 219 by basename and 78
  by directory, frozen - so a re-measurement under the original unit reproduces
  the README's figure and reads as confirmation. The neighbouring figure carries
  the other shape: the decision series is not contiguous, 103 numbers reaching
  D105, so a range claim would state a false count as a side effect.
- **`BUILDING.md`'s positional gate ordinals** ("part 6", "parts 1-4") were
  surfaced as OUT of Task 1 and ROUTED at this close to whichever package next
  edits those gate blocks after Task 1 lands. They are Rust-block-local
  positions rather than totals, and they become newly ambiguous once the file
  states a total, since "part 6" then has a second possible referent.
- **A coverage fact now tracked in the ROADMAP** (D102 paragraph, Plan 9 anchor), not this package's problem: the guard
  for the sorted half of the D102 contract is `have_mkvmerge()`-gated, so that
  half is unguarded on any machine without mkvmerge.
- **The controller's own error class stayed the most frequent, again, and every
  instance was caught by something other than the controller.** Enumerated
  rather than counted from memory: a corpus count whose search enumerated cited
  file extensions from recall and omitted one; a push-log commit count written
  without counting; an unmeasured "fourth time this session" in a commit
  message, amended pre-push; a review brief conflating measurements a reviewer
  can reproduce with fires prescribed against an unbuilt deliverable; an
  amendment brief asserting HOW a stale figure came to be wrong, refuted by
  measuring at the commit that introduced it; a tracker line reading "during the
  rpm install" that meant the package and reads as the tool; the interactive
  `cp` alias blocking two consecutive steps on an operation an existing entry
  already names; and two doc-only commits pushed before the gate ran rather than
  after. Several became ledger entries; one near-miss on a stretched promotion
  was caught by re-reading the statement-fit rule rather than by anything
  mechanical, which is the one with no handle yet.
- **One flaky test, owner-ruled a 1.x fix** (ROADMAP "Test flakiness"):
  `dry_run_json_emits_a_document_when_the_language_query_fails`. It has not
  reappeared.
- Framework-side follow-ups are tracked agent-side.
