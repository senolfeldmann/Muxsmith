# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Superseded in place; snapshots go to docs/process-journal/artifacts/handoffs/ at plan closes (PROMPT.md salvage rule). Written publication-grade: nothing enters this file that could not go public.

**Date:** 2026-07-11 (session 7 close: audit, walkthrough, doctrine, Plan 5.5)
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

(SI-1 through SI-4 are carried forward by the reproduction rule in this
section's heading.)

## Objective

Muxsmith v1: rule-based bulk MKV muxing tool (Rust core + CLI + Tauri 2/
Vue 3 GUI, MIT, public). Plans 1-5 complete and CI-verified. **Next
milestone: 1.0**, gated by the ROADMAP pre-1.0 gates. The code gates are
covered by **Plan 5.5** (authored this session, awaiting Şenol's execution
go); the non-code gates (CSP, README, guide/blogs, log pruning) each need
their own conversation with Şenol first.

## Decisions this session (rationale in the named artifacts)

- Session 7 was a forensic process audit: 7 auditor reports over six
  session transcripts + docs tree found ~70 lost/orphaned items beyond the
  23-item baseline sweep. Şenol decided every item individually; each
  decision is persisted with provenance in docs/ROADMAP.md (pre-1.0 gates,
  Triggers section, v1.x candidates) and docs/IDEAS.md (#5, #6).
- Root causes and countermeasures are operationalized in the
  software-dev-process package (SI-1); the human-readable distillate +
  audit raw corpus live in Şenol's Nextcloud (Projekte/project-muxsmith/),
  deliberately outside this repo.
- Recovered artifacts are committed: 8 historical HANDOFF states and 78
  reviewer verdicts (byte-verified) under docs/process-journal/artifacts/
  (commit 96f2e84), after Şenol's review plus a dedicated 86/86-file
  public audit (no blockers).
- Plan 5.5 (docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md):
  23 tasks, 4 waves, six parallel wave-1 worktree streams, two design-gated
  packages (D32 skew mitigation, D33 overlap suggestions), i18n chain last
  (plural selectors -> parity check -> de catalogs -> insta snapshots).
  Anchor-precise task specs, a documented and approved deviation from the
  plan-1-3 full-code style.

## Current state (verified via git status at close)

- HEAD d5a0d0d, master in sync with origin/master, working tree clean.
  CI green through 7fa01b3; d5a0d0d run pending (docs-only, expect green).
- HANDOFF.md is now gitignored (this file is never committed live;
  snapshots at plan closes only).
- The eight-part gate becomes nine-part when Plan 5.5 T12 lands
  (cargo doc -D warnings); until then eight.
- Şenol's OPEN actions (outside this repo, not booked as done):
  1. Commit ~/agents: new peter/prompts/software-dev-process.md; edited
     peter/memories/{MEMORY.md, project_muxsmith.md,
     reference_mkvtoolnix_source.md}; DELETED
     peter/memories/feedback_superpowers_throughout.md (content integrated
     into the doctrine); edited _shared/memory-convention.md (ask when
     memory scope/load-mode is not clear-cut); pre-existing
     user_mise_runtimes.md untracked.
  2. Commit ~/dotfiles-private: new skill
     dir-links/claude/skills/software-dev-process/SKILL.md; pre-existing
     settings.json change (cleanupPeriodDays=36500) plus his new git
     allow rules in agents/peter settings.local.json (that file lives
     under ~/agents, part of action 1).

## Next steps (priority order)

1. **Plan 5.5 execution on Şenol's explicit go**, in a fresh session:
   SDD per SI-1, scratch at .superpowers/sdd/plan-5.5/, wave order per the
   plan's dependency graph, wave 1 as parallel worktree streams.
2. Plan 6 (profile editor, help mode, apply-suggestion, packaging) on his
   separate go - starts with brainstorming; consume the named design
   inputs in the ROADMAP Plan-6 anchor (schema keyword domains, GUI test
   harness block, D23 re-check, help-id guard).
3. Non-code pre-1.0 gates: each begins with a Şenol conversation (CSP
   shape, README voice + content anchors, guide/blog format interview -
   sources: process journal + the Nextcloud distillate; log pruning
   decision).

## Open questions / risks

- None blocking. The ROADMAP Triggers section is the register every
  milestone action must consult (doctrine milestone gate).
- Şenol's behavior-as-package statement (his architecture philosophy)
  lives in his Joplin, deliberately not in this repo; ask him for it when
  designing agent/process structures.
