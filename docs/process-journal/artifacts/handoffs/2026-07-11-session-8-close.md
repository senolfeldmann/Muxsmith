# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-11 (session 8 close: non-code pre-1.0 gates)
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
close, mid-session supersede), snapshot the NEW state in the same turn to
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
Vue 3 GUI, MIT, public). Plans 1-5 complete and CI-verified. **Next
milestone: 1.0**, gated by the ROADMAP pre-1.0 gates. Code gates from the
sweep: covered by **Plan 5.5** (authored, awaiting Şenol's execution go).
The four conversation-gated items (CSP, log pruning, README, guide/blog
formats) were all resolved in session 8; what remains before 1.0 is
execution work (Plan 5.5, Plan 6, D35 implementation, idiomacy review)
plus the at-1.0 deliverables (guide, two blog posts, README placeholders).

## Decisions this session (rationale in the named artifacts)

- **D34 CSP** (memo docs/superpowers/specs/2026-07-11-pre-1.0-design-
  decisions.md): strict explicit-directive production CSP set in
  tauri.conf.json; no devCsp (source-verified inert with devUrl);
  verified on a Linux debug production build (rendered shell + IPC boot
  gate). ROADMAP gate consumed.
- **D35 run-log retention** (same memo): auto-prune run dirs older than
  14 days, fixed, no v1 config; parity MATCH with mkvtoolnix defaults;
  Şenol overruled the keep-forever + prune-facility recommendation.
  Implementation pending (ROADMAP pre-1.0 entry; vehicle decided at the
  Plan 5.5 go). Configurability parked as IDEAS #7.
- **README v1 shipped** (62aaf61, live on the repo front page): sell-tone
  per Şenol's register override (case-scoped exception to the neutral
  writeup voice), WIP banner until 1.0, full CLI usage reference,
  human-AI collaboration story. Four `placeholder(1.0)` comments (GIF,
  dry-run snippet, release artifacts, screenshot) resolve at the tag.
- **Guide/blog formats decided** (ROADMAP entry): GUIDE.md single file,
  EN, maximal scope Şenol prunes; two cross-linked posts written at 1.0
  (EN + DE) into the blog project folder; authoring via three fresh
  sessions fed journal+repo+git+artifacts. The recovered R3 rationale
  (decay rates, Betriebsblindheit, disjoint audiences) is at
  docs/process-journal/artifacts/r3-journal-blog-rationale.md.
- **New pre-1.0 gate: whole-codebase idiomacy review** (ROADMAP): after
  Plans 5.5/6, before release-facing gates - unidiomatic constructs,
  reuse violations, hand-rolled-vs-stdlib, inverse dependency sweep.
  Motivated by the new shared-conventions idiomacy directive (framework
  side, see open user actions), which only governs code written after it
  existed.
- **product-baseline skill split** (framework side): renamed to
  product-baseline-saas; a product-baseline-desktop gets DERIVED from
  this repo's registers/memos/spec at 1.0 (ROADMAP Near-1.0 entry; D34 a
  named input; both descriptions must be mutually exclusive and mutually
  pointing so skill selection never rests on inference).

## Current state (verified via git status at close)

- Master in sync with origin/master, working tree clean. CI green
  through 4b4d9f8; later session-8 commits are docs-only (journal,
  handoff snapshots), expect green. Session-8 commits: d5a0d0d..HEAD,
  including the recovery of the lost session-6/7 HANDOFF states
  (e3e1205) and this state's own snapshot per SI-5.
- CSP is live in src-tauri/tauri.conf.json; the eight-part gate ran green
  this session (32 test binaries, 370 tests).
- .superpowers/sdd/ scratch verified byte-identical to the salvaged
  artifacts under docs/process-journal/artifacts/; Plan 5.5 uses the
  namespaced .superpowers/sdd/plan-5.5/ when it starts.
- Şenol's OPEN actions (outside this repo, not booked as done):
  1. Commit ~/agents: modified _shared/conventions.md (idiomacy
     directive: ecosystem-idiom check, reuse-before-writing,
     dependencies-are-earned), modified peter/prompts/
     software-dev-process.md (product-baseline-* wording), renamed
     _shared/prompts/product-baseline.md -> product-baseline-saas.md.
  2. Commit ~/dotfiles-private: renamed skill dir product-baseline ->
     product-baseline-saas (frontmatter + title + pointer updated),
     edited repo-deep-scan/SKILL.md and software-dev-process/SKILL.md
     references, pre-existing settings.json change from session 7.

## Next steps (priority order)

1. **Plan 5.5 execution on Şenol's explicit go**, in a fresh session:
   SDD per SI-1, scratch at .superpowers/sdd/plan-5.5/, wave order per
   the plan's dependency graph, wave 1 as parallel worktree streams. At
   that go, Şenol decides the **D35 auto-prune implementation vehicle**
   (added wave-1 task vs standalone small task).
2. **Plan 6** (profile editor, help mode, apply-suggestion, packaging) on
   his separate go - starts with brainstorming; consume the named design
   inputs in the ROADMAP Plan-6 anchor.
3. **Whole-codebase idiomacy review** after Plans 5.5/6 land (ROADMAP
   pre-1.0 gate; multi-agent per the process package).
4. **At 1.0**: guide + two blog posts via three fresh sessions (formats
   and pipeline in the ROADMAP entry), README placeholder(1.0) comments +
   WIP banner drop, requirements-catalog derivation into
   product-baseline-desktop (ROADMAP Near-1.0).

## Open questions / risks

- None blocking. The ROADMAP Triggers section is the register every
  milestone action must consult (doctrine milestone gate).
- Şenol's behavior-as-package statement (his architecture philosophy)
  lives in his Joplin, deliberately not in this repo; ask him for it when
  designing agent/process structures.
