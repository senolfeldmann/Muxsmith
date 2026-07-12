<!-- snapshot of HANDOFF.md as written at the session-9 final close, 2026-07-12 (SI-5) -->
# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-12 (session 9 final close: Plan 5.5 done, ponytail adoptions, idiomacy review prepared)
**Active agent:** Peter (~/agents/peter)
**Working directory / repo:** ~/Git/Muxsmith (github.com/senolfeldmann/Muxsmith, public, master tracks origin/master)

## STANDING INSTRUCTIONS - reproduce this ENTIRE section verbatim in every future HANDOFF, including this reproduction rule itself

### SI-1. Process package

This project runs the **software-dev-process** package as its standing
method: the skill (auto-triggers at controller moments) plus the doctrine
at `~/agents/peter/prompts/software-dev-process.md`. The doctrine is the
single source for the execution spine (Superpowers workflow, SDD never
inline in the controller, parallel worktree streams), write-at-creation
rules (deferrals name their vehicle; reviewer verdicts and recon corpora
are files at creation), the gates (session start/close, plan close,
milestone, pre-execution) and controller duties (verify-never-trust,
lossless condensation, empirical grounding). Do NOT restate doctrine
content here - read it. Şenol's ruling 2026-07-11: this binding is
project-scoped; new projects get an adopt-or-not question at kickoff.

### SI-2. Process journal

Journal duty per `docs/process-journal/PROMPT.md` (git-tracked, read it
each time - it now mandates the salvage pass incl. reviewer-verdict files
and a HANDOFF snapshot at EVERY plan close). Entries at every plan
completion and session close.

### SI-3. mkvtoolnix parity audit in all planning and decision-making

When authoring plans or design memos, or resolving ANY behavioral question,
compare against mkvtoolnix-gui / mkvmerge wherever meaningful. Load-bearing
distinction: mkvtoolnix is INTERACTIVE (pre-fills guesses the user reviews),
Muxsmith is DECLARATIVE BATCH (the profile is the spec). Muxing semantics
and output are parity targets; input-time convenience guesses are NOT
(docs/IDEAS.md 1-2). Method: classify match / justified divergence /
genuine gap; read the source at ~/Downloads/mkvtoolnix (cite file:line);
confirm mkvmerge behavior by running the binary (v100.0 currently), never
from memory; surface gaps and divergences for Şenol; record divergences in
the memo. Licensing boundary (mkvtoolnix GPL, Muxsmith MIT): behavior,
facts and interfaces are fair game; literal code or text passages are
never taken; deliberately modeled wording is recorded as an explicit memo
decision.

### SI-4. Git commits and pushes are STANDING-authorized for this repo

Şenol's grant (2026-07-09, "persist indefinitely"): commits AND pushes on
~/Git/Muxsmith are authorized standing; never re-request. Agent commits
are deliberately UNSIGNED as policy (a GPG signature is Şenol's authorship
claim): `git -c commit.gpgsign=false` on every agent commit and merge.
Trailer per convention; log every push in gh-log.md (git-ignored). The
permission mechanics are REALLY solved since 2026-07-11: Şenol added
`Bash(cd /home/senol/Git/Muxsmith && git *)` and
`Bash(git -C /home/senol/Git/Muxsmith *)` to
~/agents/peter/.claude/settings.local.json himself (an earlier HANDOFF
claimed this was done on 2026-07-10; it was not - verify claims on disk).
The agent cannot edit its own permission file (self-escalation, hard-
denied); any future rule change is Şenol's edit. Never `git add -A`
(untracked artifacts); stage explicitly.

### SI-5. HANDOFF lifecycle: snapshot every state, publication-grade always

HANDOFF.md is git-ignored and superseded in place, so any state not
snapshotted dies with its overwrite (eight historical states had to be
mined out of transcripts on 2026-07-10/11; two more - the session-6 and
session-7 closes - were lost again on 2026-07-11 because this rule lived
only in the file's intro prose and PROMPT.md fires it only at plan
closes). Rule: whenever HANDOFF.md is rewritten (plan close, session
close, mid-session supersede etc.), snapshot the NEW state in the same turn to
`docs/process-journal/artifacts/handoffs/<date>-<label>.md` and commit
it. The PROMPT.md plan-close salvage rule is this same mechanism at plan
closes; this SI extends it to every rewrite. Because snapshots are
committed to the public repo, the HANDOFF is written publication-grade at
ALL times: nothing enters this file that could not go public - no secrets
or tokens, no personal or private context, no names or paths beyond the
project's approved-public set.

(SI-1 through SI-5 are carried forward by the reproduction rule in this
section's heading.)

## Objective

Muxsmith v1: rule-based bulk MKV muxing tool (Rust core + CLI + Tauri 2/
Vue 3 GUI, MIT, public). **Next milestone: 1.0.** Plans 1-5.5 complete;
the remaining pre-1.0 gates are the whole-codebase idiomacy review, the
mixed-language `allowed`-param polish, Plan 6, and the at-1.0
deliverables - all anchored in docs/ROADMAP.md.

## Constraints and conventions

The SIs above; the doctrine (SI-1); spec authoritative over plans
(docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); nine-part gate
per BUILDING.md before any push; new/changed Fluent messages land
bilingual (en+de) since the German locale shipped.

## Decisions made (and why)

- Plan 5.5 closed 2026-07-12: 30 tasks, whole-branch verdict READY after
  one fix wave. Rationale trail: per-task verdicts + whole-branch review
  in docs/process-journal/artifacts/plan-5.5-sdd/.
- D32 raw: opt-in (incl. resolved sub-decisions: schema-drift notice
  rebuilt once-per-batch as SchemaDrift info; B-8 single-field - raw:
  reads exactly the literally named property), D33 symmetric overlap
  suggestions, D35 14-day auto-prune: all implemented; memo
  docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md.
- German terminology Şenol-reviewed (Stapel, Starten, Probelauf,
  Meldungen, Verweis; endonym locale labels); the settings locale-hint
  texts were owner-approved as shipped.
- Idiomacy review sharpened 2026-07-12 by mining the ponytail rule set
  (Şenol-approved): two new review axes (yagni over-abstraction, native
  platform reinvention) + a one-line-per-finding output contract in the
  ROADMAP entry; two shared-convention rules adopted framework-side
  (native-platform-before-dependency clause, comprehension-gate on
  minimalism). The full mining analysis incl. the benchmark-evidence
  assessment is kept with the project's non-repo material.
- Rejected knowingly (do not relitigate): ponytail's decision ladder
  wholesale (redundant with the existing directive + scale rule) and its
  laziest-thing-that-works framing (our order stays correctness >
  precision > maintainability, then simplicity).

## Current state (verified via git status at close)

- Master in sync with origin at the session-close commit, tree clean, CI
  green on all three OSes with live mkvmerge on every leg.
- Gate is NINE parts; suite 500+ tests incl. 18 property tests and 11
  redacted insta snapshots (CI strict).
- docs/ROADMAP.md: pre-1.0 gates consumed to DONE one-liners; idiomacy
  entry carries six dimensions + 13 named inputs + the output contract;
  Plan-6 anchor carries five new named inputs; funnel: 37 minors -> 3
  fixed / 16 deferred with vehicles / 14 discarded / 4 resolved.
- Salvage complete (115 files, plan-5.5-sdd); journal has the plan entry
  + session-close addendum; this HANDOFF state is snapshotted.

## Next steps (priority order)

1. **Whole-codebase idiomacy review** on Şenol's go (pre-1.0 gate; the
   ROADMAP entry is the complete dispatch spec: six dimensions, 13 named
   inputs, output contract, correctness/security out of scope).
   Multi-agent per the process package.
2. **Mixed-language `allowed`-param polish** (pre-1.0, small; ROADMAP).
3. **Plan 6** (profile editor, help mode, apply-suggestion, packaging) on
   his separate go - starts with brainstorming; consume the ROADMAP
   Plan-6 anchor incl. the funnel-added named inputs.
4. **At 1.0**: guide + two blog posts (three fresh sessions), README
   placeholder items + WIP banner drop, requirements-catalog derivation
   (product-baseline-desktop).

## Open questions / risks

- None blocking. The ROADMAP Triggers section is the register every
  milestone action must consult (doctrine milestone gate).
- Framework-side follow-ups are tracked agent-side, outside this repo.
