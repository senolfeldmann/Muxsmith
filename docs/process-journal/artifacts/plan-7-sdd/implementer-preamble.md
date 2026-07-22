# Plan 7 implementer preamble (standing wiring, read before your task)

You are a fresh SDD implementer for ONE task of Muxsmith Plan 7. The plan
is `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md`. Read its
header + Global Constraints + YOUR task only (plus exactly the design
sections your task names in
`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`). Do not
read other tasks except the Interfaces blocks yours references.

## Binding rules

- **Work only in your assigned worktree.** Commit there on its branch
  per the plan's commit steps: `git -c commit.gpgsign=false commit`,
  explicit staging (NEVER `git add -A`), the trailer exactly as the
  snippet shows. NEVER push. Never touch master or another worktree.
  **The worktree already exists - it is a plain directory. NEVER call
  EnterWorktree/ExitWorktree or any session-relocation/worktree-switching
  tool**: that relocates the permission root, which triggers a HUMAN
  permission prompt no auto-mode approves, and your run stalls invisibly
  for hours. Operate with absolute paths only (`git -C <worktree>`, file
  paths under the worktree, `cd` inside a single Bash command is fine).
- **Conform to the Tier-2 house files** (`docs/product-boundaries.yaml`,
  `docs/conventions.yaml`, `docs/process-conventions.yaml`). Surface (do
  not silently resolve) any new pattern you establish or any deliberate
  deviation - list them in your report.
- **Standing structural-conformance grant**: following the touched
  file's existing structural patterns is in-scope without a round-trip
  when the extension has ZERO outward effect - no API/symbol surface, no
  data-format change across a serialization boundary, verification never
  weakened (additive pattern-conforming test extensions are covered; any
  weakening/deleting/skipping/rewording of existing assertions, mutating
  existing fixture values, new test files, or new test infrastructure
  stops), nothing user-visible. The grant fills SILENCE only - an
  explicit enumeration in plan/design/spec wins over it. Presentation
  carve-out: exact colors/widths/spacing within the existing design
  language are yours (`latitude-carveout-presentation-tokens`); semantic
  mappings the plan/design enumerate are not.
- **No keyboard-resolved forks**: a fork with real decision content
  (ripple cost, hidden consumers, colliding invariant, anything
  user-visible the plan does not close) returns as **NEEDS_CONTEXT**
  with a decision memo - the options, their costs against the named
  invariants, your recommendation - BEFORE you resolve it. Never
  decide-then-report. An unenumerated set you would have to invent is
  such a fork.
- **The plan's steps are binding**, TDD order included: failing test
  first, watch it fail, minimal implementation, watch it pass, commit.
  Run every test/gate command in the FOREGROUND (no background runs, no
  monitors). Do not tick the plan's checkboxes or edit the plan/design -
  progress bookkeeping is the controller's.
- **Verify plan premises against the tree** before building on them
  (line anchors may have shifted; within a serial stream, re-locate by
  quoted text). Refuting a premise with evidence is a valid outcome -
  report it, do not improvise around it.
- Empirical claims about tools/crates: confirm at the installed source
  or by running the real binary, never from memory.

## Report format (your final message)

- Verdict: DONE / DONE_WITH_CONCERNS (numbered concerns) /
  NEEDS_CONTEXT (decision memo).
- Per plan step: done/deviated (with evidence for deviations).
- Commits made (hashes, in your worktree branch).
- Test/gate evidence: the actual commands run and their outcomes -
  quote counts, do not round.
- Surfaced items: new patterns, deviations, premise refutations,
  anything the reviewer or controller must know.
