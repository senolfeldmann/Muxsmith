<!-- Provenance: snapshot of HANDOFF.md at session 16 close (2026-07-16), Plan 6 plan APPROVED, per SI-5. Corrected to the final HEAD after the journal entry landed. The HANDOFF itself is git-ignored and superseded in place. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-16 (session 16, extended: Plan 6 execution plan APPROVED four-eyes; D49 approved; ledger hygiene resolved; plan-close gate gained a blocked-pool sweep; execution plans joined the four-eyes rule)
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
`git add -A` (untracked artifacts); stage explicitly.

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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-5.8 complete. **Plan 6 has
an APPROVED execution plan and has NOT been executed** - executing it is the
next session's work.

## Constraints and conventions

The SIs above; the doctrine (SI-1) incl. house-knowledge management,
**four-eyes authorship for execution plans** (new 2026-07-16: a plan is
authored by a fresh implementer against a controller brief and graded by an
independent reviewer with a coverage dimension; the controller briefs and
routes, never authors or edits a plan's content; progress lives in the SDD
scratch, not the plan; a mid-run amendment is authored by the plan's own
author, resumed), and the **plan-close blocked-pool sweep** (doctrine §3
step 1b). Spec authoritative over plans
(docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); nine-part gate per
BUILDING.md before any push; new/changed Fluent messages land bilingual
(en+de). Ledger/Tier-2 YAML files are edited by targeted text replacement,
never a serializer round-trip (doctrine §7); `scripts/ledger-lint.py` checks
the invariants.

## Current state (verified via git)

- master = 437703f, tree clean, and pushed (`git ls-remote origin
  refs/heads/master` returns 437703f). Everything this session produced is on
  the public remote. (The approved Plan 6 plan is commit 12c002e; three later
  commits are this HANDOFF's own snapshot and the session-close journal entry.)
- **No product code touched this session** - crates/, src/, e2e/, src-tauri/
  are untouched since e107bd8. This was a spec/plan/ledger session; the
  nine-part gate never ran because there was nothing to gate. The next
  session, which executes Plan 6, gates normally.
- All 389 entries across the four house-knowledge files satisfy the ledger
  invariants (`scripts/ledger-lint.py` green).

## Decisions made (and why)

- **Plan 6 execution plan APPROVED** (`docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`,
  commit 12c002e). Authored four-eyes over two review rounds. Round 1 found a
  Critical (the draft's Task 6 mandated reuse of a StructuredEdit->MatchExpr
  seam that does not exist in reusable form), closed by ADR D49. Round 2
  APPROVED. 14 tasks, four waves. Controller/owner decisions folded in so no
  design question reaches an implementer: ProfileDocument is
  `config_only_document` + a `profile` key (core-85, not a bespoke struct);
  the editor's spec-8.3 tooltips ride Plan 7 (owner ruling), gui-editor.ftl
  stays 43 keys; the D49 wire change (StructuredEdit carries Scalar) is a
  one-way Task 6 -> Task 5 dependency, no cycle.
- **D49** (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`, commit
  71d564a, four-eyes APPROVED): apply carries the typed Scalar on the wire
  (StructuredEdit::{AddExact,AddNotExact} take `value: Scalar`); the silent
  no-op is detected by a third ApplyError variant `EditChangedNothing` (owner
  ruling). D49 cannot land before D44 (the derive needs `Scalar: TS`).
- **Save failures are `SaveError` -> `IpcError`, not a `Diagnostic`** (Şenol),
  Tier-2 `core-124-error-currency-split`. The save-surface note is one Fluent
  key. Both fold into the plan's Task 1.
- **Execution plans joined the four-eyes rule** (Şenol 2026-07-16) with all
  its follow-ups (scoping to authoring, mid-run amendments authored by the
  plan's author, the plan is not a progress tracker). Recorded in the
  doctrine (agent-side).
- **Ledger hygiene resolved** (commit e24759b): the 2026-07-15 blocked-pool
  audit's dispositions are done - 12 closed, 2 re-pointed (core-56, core-66 ->
  `v1.x planning`), 1 reclassified (exec-23 -> settled restraint), gui-22
  superseded by D35 (exec-44). Blocked pool in the ledger 27 -> 14.
- **The plan-close gate gained a blocked-pool sweep** (Şenol, doctrine §3
  step 1b): the audit found zero entries had FIRED their condition, so
  `blocked_on` does not drive the work - plans do, and the ledger learns late
  or never; the sweep re-reads the pool where staleness is created.

## Next steps (priority order)

1. **Execute Plan 6.** The plan is approved
   (`docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`). Follow its
   own structure: Task 1 (design amendments) lands first, then wave 1 (streams
   A/B/C in worktrees), the join at Task 5, wave 2 (Task 8), wave 3 (frontend
   9-13), wave 4 (Task 14). SDD per the doctrine: fresh implementer + reviewer
   per task, nine-part gate after every merge. **D49 cannot land before D44**
   (Task 5). Do not re-open the plan's or the design's settled decisions.
2. **Plan 6 close** (after execution): roll-up funnel, the new blocked-pool
   sweep step (1b), salvage `.superpowers/sdd/plan-6/` to the journal
   artifacts dir (the D49 review + the plan review, both git-ignored, both
   with two rounds stacked), journal entry, HANDOFF supersede.
3. **At 1.0:** guide + two blog posts (fresh sessions), README placeholder
   items, requirements-catalog derivation.

## Open questions / risks

- **Nothing gates `IpcError` codes against `gui-common.ftl`** (ROADMAP, own
  entry with a trigger). `DiagCode` is gated exhaustively; `IpcError` codes
  are plain strings and `check-i18n.mjs` warns only. Two separable pieces: a
  presence gate for IpcError codes, and number promotion for IpcError params.
- **The other three house files carry 12 unswept blocked entries**
  (product-boundaries 6, conventions 3, process-conventions 3) - the audit was
  ledger-only. The new plan-close sweep covers them going forward; a one-off
  sweep is a small owner-disposition task, not blocking.
- **The deeper `blocked_on` redesign** (blocked entries naming a vehicle/plan
  instead of a condition) was offered and declined for now; revisit if the new
  sweep keeps finding much stale.
- **Spec 8.4 / Renderer rustdoc "English only" staleness**, and spec 10
  crediting an eslint rule that does not exist, are tracked in ROADMAP v1.x for
  the next spec-touching plan.
- **D44's "20 model types" are unenumerated** (design :509, :548) - a latitude
  omission in a settled ADR; D49 binds its own `ts` derive explicitly, any
  other type on that wire has the same hole.
- Framework-side follow-ups from this session are tracked agent-side, and
  include one uncommitted state outside this repo.
