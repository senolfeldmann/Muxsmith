# Plan 7 plan-authoring brief

Controller brief for the four-eyes plan phase (S20, 2026-07-21). You are
the PLAN AUTHOR: a fresh implementer who writes the executable Plan 7
implementation plan. An independent reviewer will grade it (coverage
against the design, latitude in both forms, house conformance) before
execution. Verify every premise of this brief against the tree; refuting
a premise with evidence is a valid outcome.

## Contract and ground truth

- **The design is the contract**: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
  (D50-D64, owner-approved after three review rounds). The plan implements
  it - every design decision, every mandated enumeration, every spec
  amendment in its section 6. The v1 spec
  (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) stays
  authoritative above both; flag conflicts, do not improvise.
- Tier-2 house files bind: `docs/product-boundaries.yaml` (note
  `cli-multilang-rendering`, `editor-generic-action-keys` at its 46-key
  budget, `gui-closed-domain-dropdowns`), `docs/conventions.yaml` (note
  the newly promoted `core-derive-dont-restate`),
  `docs/process-conventions.yaml` (note
  `latitude-carveout-presentation-tokens`,
  `latitude-carveout-zero-content-structural-forks`,
  `proc-proposed-safeguard-stays`, `proc-normative-count-recomputed`).
- House plan format: invoke the `superpowers:writing-plans` skill and
  follow its structure (header with the REQUIRED SUB-SKILL note, Global
  Constraints verbatim, per-task Files/Interfaces blocks, bite-sized
  checkbox steps with complete code, no placeholders, self-review).
  Study `docs/superpowers/plans/` for the plan-6 document as the house
  instance of that format. House deviation from the skill text: progress
  NEVER enters the plan document (no ticked boxes; the SDD scratch's
  progress.md is the tracker) - the checkbox syntax is structure, not a
  tracking surface.
- Deliverable: `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md`.
  Do not commit; report back.

## Global constraints the plan must carry verbatim

- Nine-part gate per BUILDING.md green before any push; full gate after
  every merge.
- Every new/changed Fluent message and every help topic lands bilingual
  (en+de); de terminology rides to the owner surface pass at the plan
  close (house precedent), the plan states this.
- Registry-verified pins: marked =18.0.7 (D50, exact); no other new
  dependencies.
- D64 locale pinning: every CLI-output-asserting test goes through the
  `support::muxsmith(args)` funnel appending `--locale en`; the post-sweep
  invariant (cargo_bin only in tests/support/mod.rs) is a stated sweep
  target with its completion check.
- Implementer briefs get foreground test runs only (no
  background-run-plus-monitor inside subagents) - the plan's steps are
  written accordingly.
- Generated-bindings changes (D58's settables.ts additions) go through
  `cargo test -p muxsmith-core --features ts` + the existing CI drift
  gate; committed generated output, never hand-edited.

## Structural requirements

- **Dependency graph first**: draw the task dependency graph and mark
  independent streams for parallel worktree execution (house standing
  method; serial-in-one-tree only for genuinely dependent or
  same-file-region tasks). Candidate stream cuts the design suggests -
  verify, do not transcribe: CLI multilang (D63+D64) is orthogonal to all
  GUI work; check-i18n/gate extensions (D55 rules, D61 presence gate,
  D62) form a tooling stream; the editor comfort items (D57 markers, D58
  dropdowns, D59 ordinal) touch EditorView/registries and may collide
  with the tooltip/attribute pass (D53/D55) - decide explicitly.
- **Task right-sizing** per the skill; every task ends independently
  testable, TDD steps with real code in the plan.
- **Content tasks are tasks**: the 22 help topics x 2 locales (44
  markdown files) and the 42 tooltip attributes x 2 locales are authored
  content with their own tasks, acceptance criteria (D54's per-control
  justifications are the topic scope), and the D62 gate as their
  completeness check.
- **Spec amendments are a task** (or explicit steps in the tasks that
  land them): design section 6 lists them with the land-together
  constraint (amendments 1/2 with D63's code, not before).
- **Latitude-free**: every set enumerated (the design carries the
  enumerations - transcribe them exactly or reference them precisely,
  never sample them); no implementer-choice clauses; unmarked lists are
  exhaustive. The presentation-token carve-out covers visual CSS tokens
  only - semantic mappings the design enumerates (severity classes,
  hover-vs-pin distinction) appear in the tasks that build them.
- The plan states the execution method: subagent-driven development,
  fresh implementer per task, independent reviewer per task, whole-branch
  review at the close (house standing; pre-execution gate makes the
  stated method binding).

## Known traps (from this project's record)

- A count in the plan is recomputed from its enumeration
  (`proc-normative-count-recomputed`); the plan-6 record shows counts
  drifting from their own tables as the commonest defect class.
- A verification step whose expected result is an absence must be
  fire-verified once (`proc-verification-step-must-be-falsifiable`);
  prefer the presence-control pairing template where applicable.
- A reuse mandate names the exact signature and traces every argument
  (`proc-reuse-mandate-names-the-signature` in the ledger).
- The D57 config_path table and the D55 migration table were the design's
  hardest-won enumerations - transcribe them without abbreviation.

## Report back

Your final report: the task list with the dependency graph and stream
cut, which design sections map to which tasks (coverage self-check), any
design defect you found while planning (the plan-6 record shows
plan-authoring is where design defects surface - report them, do not
silently fix), and open questions routed to the controller. The plan
document is the deliverable.
