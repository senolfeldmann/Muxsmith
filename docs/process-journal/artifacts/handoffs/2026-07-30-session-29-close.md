<!-- Provenance: snapshot of HANDOFF.md at the session-29 close (2026-07-30), the definitive state: both amendments approved and the four late owner rulings folded in, including the Plan 13 definition and the final QA sequencing. Supersedes all earlier same-day snapshots. Taken per SI-5 because HANDOFF.md is git-ignored and superseded in place. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-30 (session 29 close; Plans 11 and 12 authored, amended and fully approved; neither executed)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1 through 10 are closed. Plans
11 and 12 are FULLY APPROVED contracts - owner and independent review, amendments
included - and have NOT been executed. Execution is the next session's first work.

## The gate that changes what "done" means

**Owner ruling, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no 1.0 release is
cut before Şenol has personally run a manual QA and bug-hunting pass on his own
hardware.** Its output is first-class scope input in three shapes he named: real
bugs; behaviour he dislikes even where it matches the spec; and v1.x items he
decides belong in 1.0 after all. **All three shapes have now occurred** - see the
ROADMAP's "OWNER QA PASS, round 3" entry, which is the authoritative record of
that pass and its five rulings.

**His pass is STOPPED and cannot resume on any build that exists.** Round 3, on
Windows, found that the GUI can only OPEN a profile and never create one, so
nothing behind a profile is reachable: not the editor, rule add/remove, suggestion
apply, a dry-run, a run, the jobs view or run history. Plan 12 is what unblocks
it. **Until that pass completes, 1.0 scope is unknown by construction and no
completeness claim about 1.0 may be made.**

**TIMING, final form ruled 2026-07-30: the pass comes AFTER PLAN 12.** That is the
third formulation in two days and supersedes his own earlier same-day ruling that it
came after both Plan 12 and Plan 11; all three versions and what each got wrong are
kept in the ROADMAP rather than replaced. The controller's original recommendation
was exactly this, so it now stands - by his ruling, and with the reasoning it rested
on restored: the round's yield is known BEFORE Plan 13 is designed, which is why
Plan 13's scope is deliberately open until the round returns.

## Current state (verified)

Re-derive rather than trusting these lines: `git log --oneline -1`,
`git status`, `git rev-list origin/master..master`.

- **Nine commits this session.** Two pushed early (the ROADMAP rulings and Plan
  11's first fix round) behind a full green gate; the rest were committed locally
  and pushed at the close behind a second full gate run.
- **The eleven-part gate ran green**, each part's exit code captured separately
  rather than trusting an aggregate: 505 Rust tests over 39 suites, 68 e2e cases,
  `check-i18n` clean over 7 catalogs against a second locale.
- **Plan 11 is APPROVED by owner and by independent reviewer, amendment included.**
  Four review rounds, 23 findings raised, 23 addressed, 0 disputed. Document:
  `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`.
- **Plan 12 is APPROVED by owner and by independent reviewer, amendment included.**
  Four rounds, 0 disputed, two author divergences both upheld on their evidence.
  Document: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`.
- **House knowledge is at 560 entries**, up from 548 at the session start; six
  entries were mined at verdict arrival, and `ledger-lint` is green.
- **No plan was executed and no worktree was created.** Both plans are contracts.
  All SDD scratch for them lives in `.superpowers/sdd/plan-11/` and
  `.superpowers/sdd/plan-12/` and is UNSALVAGED, because salvage belongs to the
  plan close, not to a session close - the reviewer verdict files and the
  consolidated addenda record are there.

## One question is escalated TO the owner, with a recommendation

Nothing blocks on it. **A `raw:` comparison that can never match is not reported at
config time**, and the runtime diagnostics were measured not to carry the signal: no
error severity, no suggestion, no proposed narrowing, an exit code identical on
success (1 for a `raw:` match, 1 for an optional non-match, 2 for a required one),
and a skew warning that fires whether or not the comparison succeeded. The only
signal is the human rendering naming the unmatched rule. D111 therefore leaves
`RawOnKnownProperty`'s scope unchanged and escalates a config-time never-match
guard as its own package, with a recommendation to build it. That figure was
corrected three times before it was right; the ROADMAP entry carries all three
versions and what each got wrong.

## Owner decisions: ALL RULED at the session close, nothing waits on him

**Both plans are APPROVED.** Four further rulings landed after the approvals and
two of them change work that was already contracted. Full context for each is in
the ROADMAP; this is the index.

1. **The close-path residual: option B**, re-read the decision after the confirm
   and prompt again when it changed. It does NOT overrule D109 decision 5, whose
   four-variant table and rejection of sequential prompts concern the case where
   both facts hold at READ time; this is the case where the state changed between
   read and confirm, so B extends decision 5's own one-prompt-per-state principle.
   Cost: one extra evaluation plus a conditional dialog on an exceptional path.
   Vehicle: Plan 12, a text replacement in the paragraph that currently presents
   both options and picks neither.
2. **`raw:` must compare byte-exactly**: the documents were right and the CODE is
   the defect, so that entry inverts from a wording repair into a behaviour fix.
   **He also ruled that Plan 11 must be ADJUSTED rather than shipping the
   accurate-for-today wording**, overruling the controller's recommendation.
   Vehicle: a Plan-11 amendment, and it is the FOUR-ROLE case because it re-cuts
   task A3 from a documentation task into a behaviour change with tests.
   **Read the ROADMAP entry before touching this**: the fix is surgical and must
   not strip the int/float cross arms from `scalar_eq` itself, because those are
   correct and documented for the typed `exact` path. Only the `raw:` arm, which
   shares the function, is wrong.
3. **Transcript-archive dating: option A**, and it is now the written rule. Bundle
   names use the UTC start date; every document date - footers, the generated
   index - uses local time, because the index generator derives last-edit from
   filesystem mtime in local time. The mnemonic: names date the session, footers
   date the file. This is agent-side project material, not repo material.
4. **The editor catalog budget rises 46 to 54** with his approval of Plan 12; the
   plan itemizes every new key in two tables so a strike stays possible later.

## Next steps (priority order)

**The sequence is owner-ruled 2026-07-30. Do not reorder it without him.**

1. **Implement Plan 12.** Nothing is owed before task 1: both amendments the
   post-approval rulings required are authored, independently reviewed and approved
   within session 29, and both plans carry an owner approval on top. Serial tasks in
   one worktree, per its own document. Plan 12 goes first because it is the only
   thing that unblocks the stopped owner QA pass.
2. **He runs a QA round on a draft build of that state.** `workflow_dispatch` on
   release.yml with the draft flag, never a tag, never published. This is the round
   whose yield decides Plan 13's final scope, so it is a gate on authoring Plan 13
   and not merely a milestone.
3. **Author Plan 13, folding in the round's yield, then implement it.** Its floor is
   three members and its scope is OPEN by his ruling - it may grow. See the ROADMAP
   section "Plan 13" for the three and their recorded design questions:
   the example validator (whose design question is the fragment marker), the `raw:`
   never-match guard (with his documentation requirement attached), and deriving a
   profile from a selected container (the largest, needing its own design round, with
   his binding shape: the derived rules populate the profile IN THE EDITOR, unsaved,
   the editor being the review surface).
4. **Plan 11's position in this order is HIS to name and is deliberately unresolved
   here.** He named Plan 12 and Plan 13 for the coming sessions and did not mention
   Plan 11, which is fully approved and blocks nothing. Controller reading, marked as
   a reading: it stays in the queue with after the QA round as the natural slot. The
   consequence worth raising with him: Plan 11 carries the user-visible `raw:`
   behaviour change, so a QA round before it does not exercise that change. **Ask;
   do not assume.**
5. **Read D111** (`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`)
   before Plan 11's task A3. The plan points at it per-site rather than duplicating
   its twelve replacement strings, and that is a ruled decision with a hardening
   clause in A3's Must-not-decide: transcribing those fences into the plan is
   deliberately NOT wanted, because nothing compares the applied text against D111,
   so a drifted duplicate would stay green while the wrong sentence shipped.
6. **The one thing an implementer must not get backwards**, restated because it is
   the expensive mistake: `scalar_eq`'s int/float cross arms STAY - they are
   documented, intended behaviour of the typed `exact` path. Only the `raw:` call
   site re-points, to a same-type comparator. Test T-1 exists to catch a strip, and
   the reviewer measured that none of A3's other eight checks would.
7. **At each plan close**, beyond the standard gate: execute the recorded close
   action moving that plan's per-finding narration out of the plan document into the
   SDD scratch and the journal. Two reviewers recommended it and one measured the
   share at 21 and 23 percent of the respective documents.
8. **Archive duty:** session 29 is archived by the NEXT session. Session 28 was
   archived at the start of this one, verified, with its `/tmp` artefacts caught up.

## Open questions / risks

- **The five defects review caught this session would all have reached code**, and
  three of them were introduced by the repair of another. The pattern worth
  carrying: a fix round is where the sign gets flipped. The save-path guard was
  fixed from armed-forever to disarmed-silently in one round, and the parity check
  built to close an absence-check hole was itself green under both its red states.
- **The controller's own error class stayed the most frequent, again.** Two ROADMAP
  statements were refuted by plan authors against the tree; one
  controller-demanded check would have been green before and after the fix it was
  meant to verify; one controller-fenced figure was moved by the controller's own
  writing in the same session. Every instance was caught by something else.
- **Four brief addenda existed only as subagent messages for hours**, against this
  project's own rule that an instruction existing only as a message is
  unsalvageable. They are on disk now at
  `.superpowers/sdd/plan-12/plan-brief-addenda.md` with an order-of-authority
  header. A brief that grows by addendum needs the addenda written at creation, and
  the reviewer needs a controller-assembled requirement list rather than the
  plan's own self-report.
- **Plan 11 carries one non-gating Minor by decision**: a markup-span count that
  describes the plan author's extractor rather than the tree, and that nothing
  consumes.
- **Two open vulnerability alerts remain open** until Plan 11 executes. The
  `cargo deny` / GitHub disagreement behind them is ANSWERED and measured, not
  hypothesised: the unsound scope defaults to workspace and glib is transitive.
  Interim disposition is ruled; addressing glib properly is a v1.x item with full
  context and an observable trigger in the ROADMAP.
- **Renovate's first dependency PRs are expected 2026-08-01 to 08-03** (its cadence
  is the 1st to 3rd; security updates bypass it). When they land, walk the RUSTSEC
  ignores in `deny.toml` and drop the ones they obsolete, and take the TypeScript-7
  bump when the typescript-eslint ceiling allows. Nothing is owed before then.
- Framework-side follow-ups are tracked agent-side.
