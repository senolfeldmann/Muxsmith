# Plan 7.5 plan review brief (round 1)

Independent reviewer, fresh eyes; you did not author the plan. Artifact
under review: `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md`
(commit cfe10a4, 4 tasks). Ground truth: the owner-approved design
`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`
(D65-D72), the controller plan brief
`.superpowers/sdd/plan-7.5/plan-brief.md`, the house template
`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md`, the Tier-2
house files, and the ACTUAL TREE. The plan's claims are verified, never
believed.

## Dimensions

1. **COVERAGE (the load-bearing dimension)**: walk the design section by
   section and D-entry by D-entry (D65-D72, sections 0-9 incl. the spec
   amendments, e2e plan, triggers). Name the task implementing each. A
   design element with no implementing task is a FINDING; the plan's own
   coverage map is graded against your independent walk, not accepted.
2. **Latitude scan, BOTH forms, per task text**: explicit permissions
   and omission latitude (unenumerated sets in normative positions). The
   test: must a fresh implementer invent anything it is not licensed to
   invent? Design section 9's implementer-must-not-decide list must be
   transmitted, not diluted.
3. **Template conformance**: the plan-7 structural conventions (header
   note with the no-progress house deviation, binding execution-method
   section naming SDD with fresh implementer + independent reviewer per
   task + whole-branch review, tracker path, global-constraints set per
   the brief incl. the session-relocation ban, plan-close
   pre-registrations per the brief).
4. **Transcription fidelity**: the author claims four transcribed blocks
   match the design byte-for-byte modulo indentation - re-diff them
   yourself (the ledgered plan-7 T21 truncation defect is the class this
   guards).
5. **Dependency graph vs reality**: check each edge and each
   parallelism claim against the actual file graph (which files do the
   tasks touch; is the 2-stream cut collision-free; does anything
   parallel share a file region).
6. **Citations and counts**: every :line/:symbol ref re-verified at the
   current tree; every count recomputed from its enumeration
   (proc-normative-count-recomputed - counts were the plan-7 plan
   review's repeated defect class; walk every numeral).
7. **Model-tier classification** (proc-03): the cheap-tier claim on the
   transcription task holds ONLY if the plan carries that task's content
   completely (verify: could a transcription-tier implementer execute it
   with zero judgment?); mid-tier defaults elsewhere; reviewers mid.
8. **e2e mapping**: the design's nine cases map to enumerated steps,
   with the zero-new-help-ids/zero-new-topics invariant intact (a task
   step that would create a help-id is a finding).
9. **Implementability walk**: for each task, read its steps as the fresh
   implementer would - executable without invention, foreground runs,
   gates named, fire-verifications carried where the design mandates
   them, NEEDS_CONTEXT routing stated.

## Output

Write `.superpowers/sdd/plan-7.5/plan-review-round-1.md`: verdict
APPROVED or NEEDS FIXES; findings by severity with location and what to
change; a HARVEST section. Final message: verdict word + at most three
lines + the file path.

## Constraints

Read-only except your verdict file; no git writes; never call
EnterWorktree/ExitWorktree or any session-relocation tool; absolute
paths; anything you run, run foreground.
