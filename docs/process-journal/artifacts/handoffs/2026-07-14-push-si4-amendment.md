<!-- Snapshot of HANDOFF.md at the 2026-07-14 push + SI-4 command-shape amendment (SI-5) -->
# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-13 (session 11 close: Plan 5.6 idiomacy fix wave executed and closed)
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
push was lost to this once). The agent cannot edit its own permission
file; any rule change is Şenol's edit. Never `git add -A` (untracked
artifacts); stage explicitly.

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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-5.6 complete (5.6 = the
pre-1.0 idiomacy fix wave, executed and closed this session, whole-branch
verdict READY).

## Constraints and conventions

The SIs above; the doctrine (SI-1) incl. house-knowledge management; spec
authoritative over plans
(docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); nine-part gate per
BUILDING.md before any push; new/changed Fluent messages land bilingual
(en+de). **Standing SDD wiring (doctrine section 7):** implementer briefs
conform to the Tier-2 files; reviewer briefs treat the Tier-2 files as review
ground truth (run the `house` dimension) and harvest observed patterns/
rejections into the ledger; the controller is the single ledger writer.

## Decisions made (and why)

- **Plan 5.6 executed and closed** (commits 0b3149a..a5d506b + close-out):
  64 idiomacy findings + 13 funnel seeds applied across 12 tasks, six
  parallel streams + four serial cross-crate tasks, per-task independent
  reviews, whole-branch review on the strongest model, one final fix wave.
  Zero behavior change except THREE sanctioned interface deltas: ADR D36
  (Diagnostic.claimants, structural, JSON changed on overlapping-rules
  only), the diag_signature tuple key (latent collision removal only), and
  MkvmergeInfo losing the dead meets_minimum field. ADR:
  docs/superpowers/specs/2026-07-13-plan-5.6-decisions.md.
- **Ledger harvest done** (doctrine §7): core-90 promoted to Tier-2
  (doc-hidden-pub cross-crate test access, count 4); core-117 non-decision
  resolved by implementation; six new Tier-1 entries incl. proc-57
  (briefs-not-ground-truth, deliberately contested - the proc-07 scoping
  question, two data points this wave).
- Full record: journal session-11 entry + the salvaged
  docs/process-journal/artifacts/plan-5.6-sdd/ (54 files incl. whole-branch
  verdict).

## Current state (verified via git, updated 2026-07-14)

- **master PUSHED to origin 2026-07-14** (6f03ca9..6e4cf37: Plan 5.6 +
  close-out + the session-start salvage). The session-11 "push blocked"
  state was a command-shape problem, root-caused and recorded in SI-4
  above. Tree clean, stream worktrees removed, full nine-part gate
  controller-verified green on the pushed state.
- The push-fired ROADMAP trigger is BEING CONSUMED: CI run 29320166456 on
  the pushed head must show all three legs green (T8 rewrote the CI
  toolchain step per rustup #4216; this run is its on-runner proof).
  If it fails, the toolchain step is the first suspect. Do not tag or
  release before this run is green and the trigger line is removed.

## Next steps (priority order)

1. **Confirm CI run 29320166456 green on all three legs**, then consume
   the Trigger (remove its ROADMAP line, note the run id).
2. **Routed-items correctness/security/perf review** (the idiomacy pass's 11
   routed-out items; ROADMAP pre-1.0 gate; the '|'-collision item is already
   closed by Plan 5.6 T2 - the task-2 report records it).
3. **Zero-rule-keep passthrough** implementation + documentation (ROADMAP;
   Şenol's scope-timing call).
4. **Mixed-language `allowed`-param polish** (ROADMAP pre-1.0 gate).
5. **Plan 6** (profile editor, help mode, apply-suggestion, packaging) on
   Şenol's go; consume the Plan-6 named inputs incl. the three items folded
   in from the idiomacy triage and the T11 rider (plan_pipeline consumes
   config_diagnostics).
6. **At 1.0:** guide + two blog posts (fresh sessions), README placeholder
   items, requirements-catalog derivation (product-baseline-desktop).

## Open questions / risks

- None blocking. gui-26 (live locale switch) stays a Tier-1 non-decision
  blocked on Plan 6. proc-57 (verify-against-source scope for untagged brief
  claims) is deliberately contested in the ledger - a panel or owner call
  when it recurs.
