# Plan 8 plan brief (controller-authored)

You author the EXECUTION PLAN for Plan 8 (packaging / release pipeline).
Four-eyes: an independent plan reviewer grades your plan - including a
COVERAGE dimension walking the design section by section - before
execution. You write exactly ONE file:
`docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md`.
Do NOT commit (the controller commits); touch nothing else.

## Contract and template

- **The design is the contract**: docs/superpowers/specs/
  2026-07-22-plan8-packaging-release-design.md (D75-D90, owner-approved
  2026-07-23 after a one-round four-eyes fix loop; the R1-R10 rehearsal
  checklist is the plan's acceptance test). The v1 spec is authoritative
  above it on conflict; flag, do not improvise.
- **The house plan template is the plan-7 plan**: docs/superpowers/plans/
  2026-07-21-plan-7-help-i18n.md. Reproduce its structural conventions
  exactly (see the plan-7.5 sibling brief's list - agentic-workers
  header with the no-progress house deviation, tracker
  `.superpowers/sdd/plan-8/progress.md`, Goal, Architecture, Global
  Constraints, binding execution method
  superpowers:subagent-driven-development, dependency graph,
  design-section coverage map over D75-D90, tasks with exact steps).

## Plan-specific requirements

- **Task cut**: yours, with the dependency graph making it checkable.
  Candidate seams the design implies (decide, do not assume): the
  version-sync guard script + tauri.conf version-key removal; the
  bundle-config changes (D86) + externalBin overlay (D82); INSTALL.md
  (D75); release.yml itself (D83/D85/D77/D79 - one file, likely one
  serial chain); the tar.gz packing step (D88) and naming/SHA256SUMS
  (D89/D90) live inside release.yml's tasks; the REHEARSAL task (below);
  the RIDER task (below). Same-file-region tasks serialize.
- **RIDER TASK (controller ruling, recorded in ROADMAP "Ledger hygiene",
  2026-07-22 S22)**: wire scripts/ledger-lint.py into CI AND extend it
  with the per-entry duplicate-key check (both deferral triggers fired;
  one task). Design latitude does NOT exist here either: the task must
  settle where the lint runs (a cheap ubuntu step; whether in ci.yml's
  existing structure or elsewhere - resolve against D83's "ci.yml is not
  modified" decision, which was scoped to the RELEASE pipeline; if you
  judge the two collide, that is a NEEDS_CONTEXT, not your call),
  Python/PyYAML setup pinned per house pin discipline, and a fire-test
  (a deliberately broken fixture makes the step fail).
- **The REHEARSAL task**: executes the design's acceptance test.
  Sequencing constraint the plan must state: workflow_dispatch requires
  release.yml on the default branch, so the rehearsal runs AFTER the
  workflow lands on master and is pushed (controller pushes; standing
  authorization). The task runs the dispatch and evaluates R1-R10
  against the run, each observable at its named emitter. gh usage rules
  bind (Tier-2/house: every gh interaction on this repo gets a
  gh-log.md entry - command, effect, manual-UI equivalent; nothing that
  costs money; public-repo Actions are free). Wall-clock note: four
  native legs incl. windows-arm64 - the task runs foreground gh watches
  or polls with explicit timeouts, never background-run-plus-monitor.
- **Model-tier classification per task** (proc-03): mid tier default;
  cheap tier ONLY where the plan itself carries the code verbatim
  (candidates: the D86 config block and D87 guard script IF the design's
  sketches are transcription-complete - verify before classifying);
  reviewers mid tier.
- **Global constraints to carry** (adapted from the plan-7 template):
  nine-part gate foreground before any push and after every merge (note:
  this plan changes packaging config, not product code paths - the gate
  still runs in full, no subsets); no new runtime dependencies; any new
  GitHub Action SHA-pinned with version comment per house precedent;
  commits unsigned + trailer + explicit staging; typography; foreground
  runs; counts recomputed; absence checks fire-verified (the design's
  G1-G5 fire-tests re-run pre-merge as it mandates); every fork closed,
  NEEDS_CONTEXT with decision memo on code contact; Tier-2 ground truth
  (name the binding entries, at minimum
  deps-first-party-pinned-over-convenience,
  design-empirical-claims-reproducible, proc-normative-count-recomputed,
  design-acceptance-observables-have-producers, and the SI-4 push rules).
- **Plan-close items to pre-register**: the rehearsal's draft-release
  cleanup (the rehearsal draft is deleted after evaluation - owner
  publishes nothing in this plan), INSTALL.md wording rides the owner's
  rendered-surface pass, and the plan-8 design's own sdd-scratch
  citations move with the salvage per the ruled house pattern (check the
  design for such citations and enumerate them for the close).

## Method duties

Latitude ban both forms; counts recomputed; citations verified at the
current tree; transcribe only the design's hardest-won enumerations
(name which - the R1-R10 checklist and the leg/artifact matrix are the
obvious candidates) and cite the rest by section.

Constraints: read-only except your one output file; no git; never call
EnterWorktree/ExitWorktree or any session-relocation tool (and the plan
carries the implementer-preamble ban verbatim: subagents never call
session-relocation tools, worktrees are plain directories, absolute
paths); anything you run, run foreground.

Final message: at most 3 lines + the plan path + any NEEDS_CONTEXT.
