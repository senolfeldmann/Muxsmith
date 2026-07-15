# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-15 (session 15 close: Plan-6 anchor re-cut into Plans 6-9; the Plan-6 design document written four-eyes and APPROVED; no code)
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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-5.8 complete. Plan 6 has a
design document and no execution plan.

## Constraints and conventions

The SIs above; the doctrine (SI-1) incl. house-knowledge management; spec
authoritative over plans
(docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); nine-part gate per
BUILDING.md before any push; new/changed Fluent messages land bilingual
(en+de). Standing SDD wiring per doctrine section 7 (Tier-2 files are
implementer/reviewer ground truth; controller is the single ledger writer).

## Current state (verified via git)

- master = ecd8e4f, tree clean. **No code touched this session** - crates/,
  src/, e2e/, src-tauri/ are untouched since e107bd8. This was a spec session.
- **Nothing pushed.** The nine-part gate did not run; there is no code to gate.
  A push is a normal next step whenever one is wanted.

## Decisions made (and why)

- **Plan-6 anchor re-cut into Plans 6-9** (Şenol; `docs/ROADMAP.md`, section
  "Plan-6 scope re-cut"). The anchor had accumulated 20 named inputs across four
  independent subsystems; all 20 are distributed and individually numbered so
  the split recounts rather than being asserted. Plan 6 = profile editor +
  apply-suggestion + the schema keyword-domain fix; 7 = help mode + i18n
  cluster; 8 = packaging; 9 = core hoists + planner seam. D22's editor+apply
  pairing survives but NOT on D22's stated reason: that reason assumed
  comment-preserving mutation, which the save-fidelity ruling killed.
- **Plan 6 design: D41-D48**,
  `docs/superpowers/specs/2026-07-15-plan-6-design.md` (1951 lines, commit
  7dac970). Canonical save, comments not preserved (D41, on YAML 1.2.2 section
  6.6: no comment-to-node association exists, so drag-to-reorder would silently
  make comments describe the wrong rule); three new commands (D42); apply
  written fresh in core (D43); ts-rs types, committed bindings, CI drift check
  (D44); hand-built components with a `Record<keyof T, FieldSpec>` registry as
  the forcing function (D45); the four keyword domains projected into the schema
  from one constant set (D46); the schema as a supported user artifact (D47);
  canonical save omits default-valued fields (D48). Rationales, rejected
  alternatives and their steelmen are in the document; do not re-derive them.
- **Written four-eyes and APPROVED**: controller brief -> fresh implementer ->
  independent reviewer, four rounds, twelve findings, none contested. Full
  verdict with all four rounds stacked:
  `.superpowers/sdd/plan-6/design-review-verdict.md` (git-ignored; salvage it at
  the plan close per SI-2).

## Next steps (priority order)

1. **Plan 6 execution planning.** The design is approved; the plan is not
   written. Start with `writing-plans` against the design document. Its
   decisions are settled and reviewed - do not reopen them.
2. **Ledger hygiene** (`docs/ROADMAP.md`, own section): 12 blocked ledger
   entries are stale (condition cleared AND the work visibly in the tree) and 3
   name no observable event at all. Each needs a per-entry owner disposition;
   the controller is the ledger's single writer and does not self-dispose them.
   Report: `docs/process-journal/artifacts/2026-07-15-ledger-blocked-pool-audit.md`.
3. **At 1.0:** guide + two blog posts (fresh sessions), README placeholder
   items, requirements-catalog derivation.

## Open questions / risks

- **The audit's structural question is open.** Zero blocked entries landed in
  "condition fired, work outstanding" - because the conditions do not drive the
  work; plans do, and they sweep up blocked items incidentally while the ledger
  finds out afterwards or never. Whether the `blocked_on` mechanism is
  misdesigned or merely needed the sweep step it now has is undecided.
- `docs/process-conventions.yaml` gained three Tier-2 entries this session:
  proc-latitude-clause-boundary (widened to every artifact an implementer reads,
  and to latitude by OMISSION - an unenumerated set in a normative position);
  proc-no-work-needed-check (where a passage concludes a guard or enumeration is
  unnecessary, the reviewer runs the claim that makes it unnecessary);
  proc-proposed-safeguard-stays (a safeguard the plan proposed goes only after it
  is built and measured redundant). Every implementer and reviewer brief carries
  the Tier-2 files, so these bind from now.
- Spec 8.4 / Renderer rustdoc "English only" staleness, and spec 10 crediting an
  eslint rule that does not exist, are tracked in ROADMAP v1.x for the next
  spec-touching plan.
- During the design review an instruction arrived through a tool-output channel
  claiming to be a meta-instruction and telling the reviewer to stop early. The
  reviewer refused it and flagged it; the repo, the framework and the hooks were
  ruled out as its source by grep. Origin unresolved and outside this repo's
  visibility. If it recurs, record it rather than dismiss it.
- Framework-side follow-ups from this session are tracked agent-side.
