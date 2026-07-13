# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-13 (session 10 close: idiomacy review executed + triaged; house-knowledge mechanism designed, built, and history-bootstrapped)
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
are solved: Şenol added the git allow-rules to
~/agents/peter/.claude/settings.local.json himself. The agent cannot edit its
own permission file; any rule change is Şenol's edit. Never `git add -A`
(untracked artifacts); stage explicitly.

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
GUI, MIT, public). **Next milestone: 1.0.** Plans 1-5.5 complete. Since
session 10 the project also runs the **house-knowledge mechanism** as its
standing convention layer (see below).

## Constraints and conventions

The SIs above; the doctrine (SI-1) incl. house-knowledge management; spec
authoritative over plans
(docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md); nine-part gate per
BUILDING.md before any push; new/changed Fluent messages land bilingual
(en+de). **Standing SDD wiring (doctrine section 7):** implementer briefs
conform to the Tier-2 files; reviewer briefs treat the Tier-2 files as review
ground truth (run the `house` dimension) and harvest observed patterns/
rejections into the ledger; the controller is the single ledger writer.
"D-memo" is now called **ADR** in forward docs.

## Decisions made (and why)

- **House-knowledge mechanism built (session 10).** Two tiers: Tier 2 =
  always-checked convention files split by NATURE
  (`docs/product-boundaries.yaml` product-scope, `docs/conventions.yaml`
  technical-code, `docs/process-conventions.yaml` process/operational);
  Tier 1 = `docs/decision-ledger.yaml`. A count is a list of cited
  occurrences (count == len; an occurrence is a distinct EVENT, not a
  document). Promotion by the source x nature matrix (user-decree/ADR -> 1,
  agent-emergent -> 3, agent-emergent x product-scope -> escalate). Full
  mechanism: doctrine section 7. Full vision + reasoning: the non-repo
  automated-software-department dossier.
- **Bootstrapped from history** by a reconstruction sweep (549 records -> 358
  clusters -> 105 Tier-2 / 252 Tier-1, zero overlap). Corpus + scripts
  salvaged to docs/process-journal/artifacts/house-backfill-sdd/.
- **Idiomacy review executed + triaged.** 70 findings; routing in ROADMAP;
  artifacts salvaged to docs/process-journal/artifacts/idiomacy-review-sdd/.
  The actual fix wave is NOT yet done.
- **5 escalations resolved:** three stack/model boundaries ratified, the
  zero-rule-keep passthrough decided (legal when unmatched=keep - ROADMAP
  follow-up to implement + document), the live locale switch deferred to
  Plan 6.

## Current state (verified via git status at close)

- Master in sync with origin at the session-close commit, tree clean, CI green.
- House-knowledge files live and committed; ledger has zero tier overlap.
- **UNCOMMITTED, framework-side (Şenol's own commit, NOT this repo):**
  doctrine section 7 additions, the `_shared/conventions.md` "Match the house
  pattern" bullet, and the Nextcloud dossier + the two staged skills. These
  are in ~/agents and ~/Nextcloud, edited but awaiting Şenol's commit.

## Next steps (priority order)

1. **Şenol commits the framework-side changes** (doctrine section 7,
   conventions.md bullet) in ~/agents; the dossier + staged skills are
   Nextcloud (no repo).
2. **Pre-1.0 idiomacy fix wave** - the triaged findings become their own SDD
   plan; findings report in the non-repo project material.
3. **Routed-items correctness/security/perf review** (the idiomacy pass's 11
   routed-out items; ROADMAP gate).
4. **Zero-rule-keep passthrough** implementation + documentation (ROADMAP;
   Şenol's scope-timing call).
5. **Plan 6** (profile editor, help mode, apply-suggestion, packaging) on
   Şenol's go; consume the Plan-6 named inputs.
6. **At 1.0:** guide + two blog posts (fresh sessions), README placeholder
   items, requirements-catalog derivation (product-baseline-desktop; the
   house-knowledge files are now an input).

## Open questions / risks

- None blocking. gui-26 (live locale switch) is a Tier-1 non-decision blocked
  on Plan 6. The ROADMAP Triggers section is the register every milestone
  action must consult.
- The `.superpowers/sdd/` root holds loose plan-5.5-era files (review diffs,
  task briefs) that may be un-salvaged remnants from session 9 - worth a check
  next session, not blocking.
