<!-- Provenance: snapshot of HANDOFF.md at the session-16 close (2026-07-16), per SI-5. The HANDOFF itself is git-ignored and superseded in place. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-16 (session 16 close: Plan 6 execution plan written and independently reviewed - NOT approved; D49 closes the apply seam - approved; execution plans joined the four-eyes rule)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-5.8 complete. Plan 6 has an
approved design (D41-D49) and a written-but-NOT-approved execution plan.

## Constraints and conventions

The SIs above; the doctrine (SI-1) incl. house-knowledge management and - new
2026-07-16 - **four-eyes authorship for execution plans**; spec authoritative
over plans (docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); nine-part
gate per BUILDING.md before any push; new/changed Fluent messages land bilingual
(en+de). Standing SDD wiring per doctrine section 7 (Tier-2 files are
implementer/reviewer ground truth; controller is the single ledger writer).

## Current state (verified via git)

- master = a71c251, tree clean. `git ls-remote origin refs/heads/master` returns
  71d564a: **the journal commit a71c251 is NOT yet on the remote** - push it
  first thing. Everything before it is public.
- **No product code touched this session** - crates/, src/, e2e/, src-tauri/ are
  untouched since e107bd8. Spec/plan session; the nine-part gate never ran
  because there was nothing to gate. The next session, which executes, gates
  normally.
- All 389 entries across the four house files satisfy the ledger invariants
  (count == len(occurrences), no empty refs, no tier-2 entry with a null
  promoted_at, no blocked entry without a blocked_on). Verified by an ad-hoc
  validator; see the ledger-lint ROADMAP item, now evidence-backed.

## Decisions made (and why)

- **Execution plans are authored four-eyes, like design documents** (Şenol
  2026-07-16). The doctrine's carve-out ("execution tests plans") covers only
  false CLAIMS, which an implementer is looking at; it cannot catch a MISSING
  task, because an implementer sees only its own. Reviewer plan briefs carry a
  coverage dimension. Briefs stay controller-authored (a brief is the input to
  four-eyes). The controller does not author or revise a plan's content; it
  briefs, routes, and does the mechanical git/salvage work. Progress lives in
  the SDD scratch, never in the plan. A mid-run amendment is authored by the
  plan's own author, resumed, with the original reviewer judging the delta.
  Full rules in the doctrine (SI-1) - not restated here.
- **D49** (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`, commit
  71d564a, four-eyes APPROVED over two rounds): `StructuredEdit::{AddExact,
  AddNotExact}` carry `value: Scalar` instead of `String`; the substring
  variants keep `String` (each variant carries the type its target map holds).
  `delta_for` loses its scalar parameter and stays private. Third `ApplyError`
  variant `EditChangedNothing` detects the silent no-op (Şenol ruling). D49
  **cannot land before D44** (the derive needs `Scalar: TS`).
- **Save failures are `SaveError` mapped to an `IpcError`, not a `Diagnostic`**
  (Şenol 2026-07-16). Tier 2 as `core-124-error-currency-split`. D41's original
  `Result<_, Diagnostic>` signature is superseded; plan Task 1 folds this and
  the one-key save-note ruling into the design document.
- **The editor's tooltips ride Plan 7** (Şenol 2026-07-16): `gui-editor.ftl`
  carries 43 keys in Plan 6 (42 labels + 1 save-surface note); the 42 controls
  get tooltips alongside their help-ids in Plan 7, not as a retrofit. Recorded
  in the ROADMAP's Plan 7 anchor.

## Next steps (priority order)

1. **Push a71c251** (the journal commit is local-only).
2. **Plan 6 fix round.** The plan
   (`docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`, commit 2f9dafe,
   marked DRAFT) has an independent review: **NEEDS FIXES, 1 Critical, 7
   Important, 10 Minor**, verdict at `.superpowers/sdd/plan-6/plan-review-verdict.md`
   (git-ignored). Its Critical is closed by D49; the other 17 stand. A **fresh
   implementer** applies D49 + the findings and becomes the plan's author (the
   controller authored the draft, which the four-eyes rule now forbids; it does
   not adopt the artifact to "just apply the findings"). Then the **resumed
   original reviewer** judges the delta. No task dispatches before the plan is
   APPROVED.
   Known plan defects worth carrying in case the verdict is lost: `cargo test -p
   muxsmith` names no package (it is `muxsmith-gui`); wave-1 streams A and C both
   amend the v1 spec against the plan's own no-shared-files claim; Task 12
   collides with Tasks 10/11 on `e2e/smoke.spec.ts`; `ProfileDocument` is
   self-contradictory (`Vec<Diagnostic>` vs "byte-identical to `validate_profile`",
   which returns an untyped Value keyed `config_diagnostics`); Task 11 is too
   large; two of the plan's own verification steps cannot fire.
3. **Ledger hygiene** (`docs/ROADMAP.md`, own section): 12 blocked entries are
   stale (condition cleared AND the work visibly in the tree) and 3 name no
   observable event at all. Each needs a per-entry owner disposition; the
   controller is the ledger's single writer and does not self-dispose them.
   Report: `docs/process-journal/artifacts/2026-07-15-ledger-blocked-pool-audit.md`.
4. **At 1.0:** guide + two blog posts (fresh sessions), README placeholder items,
   requirements-catalog derivation.

## Open questions / risks

- **`gui-22` vs `exec-44-runlog-14day-autoprune` is a recorded-statement
  collision** in `product-boundaries.yaml`: `gui-22` (:243-252) says v1 keeps all
  run logs with pruning deferred to v1.x; `exec-44` (:15-23) records D35
  reversing that to an automatic 14-day prune, which shipped. `gui-22` still
  carries `status: settled` with no supersession marker. A recorded-statement
  collision is a contested criterion, so it routes to Şenol as a spec question.
  Found while reading Tier 2 for the Plan-6 design; unrelated to Plan 6.
- **Nothing gates `IpcError` codes against `gui-common.ftl`.** `DiagCode` is
  gated exhaustively (an enum match, so a new code cannot compile without its
  message); `IpcError` codes are plain strings and `check-i18n.mjs` warns only.
  Visible cost found by the D49 review: a proposed message read "1 rules" in the
  singular and a Fluent plural selector cannot fix it, because `IpcError.params`
  is `Record<string,string>` at every call site while only `DiagnosticsPanel.vue`
  promotes numbers, keyed by *diagnostic* code. Tracked in ROADMAP with a
  trigger; D49 sidesteps it by rephrasing.
- **The blocked-pool audit's structural question is still open.** Zero blocked
  entries landed in "condition fired, work outstanding" - because the conditions
  do not drive the work; plans do, and they sweep up blocked items incidentally
  while the ledger finds out afterwards or never. Whether `blocked_on` is
  misdesigned or merely needed the sweep step it now has is undecided.
- **Spec 8.4 / Renderer rustdoc "English only" staleness**, and spec 10 crediting
  an eslint rule that does not exist, are tracked in ROADMAP v1.x for the next
  spec-touching plan.
- **D44's "20 model types" are unenumerated** (design :509, :548) - a latitude
  omission in a settled ADR. D49 detected it and binds its own `ts` derive
  explicitly; any other type on that wire has the same hole.
- Three reviewer verdicts from this session live in `.superpowers/sdd/plan-6/`
  (git-ignored): the plan review, and the D49 review with both rounds stacked.
  Salvage them at the plan close per SI-2.
- Framework-side follow-ups from this session are tracked agent-side, and
  include one uncommitted state outside this repo.
