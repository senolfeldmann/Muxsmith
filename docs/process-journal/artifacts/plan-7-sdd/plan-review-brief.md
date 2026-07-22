# Plan 7 plan-review brief (round 1)

Independent review of the Plan 7 implementation plan
`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md` (1976 lines,
uncommitted; authored four-eyes by a fresh plan author, updated to the
round-4-amended design). You grade; you do NOT fix. Execution starts only
on your APPROVED.

## Ground truth (in order)

1. The v1 spec `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`.
2. The amended, four-round-approved design
   `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
   (D50-D64) - the contract this plan implements.
3. The controller plan brief `.superpowers/sdd/plan-7/plan-brief.md`
   (global constraints, structural requirements).
4. The Tier-2 house files (product-boundaries / conventions /
   process-conventions) and the real tree.

## Dimensions (all mandatory)

- **COVERAGE - the reason plan reviews exist** (an implementer sees only
  its own task and cannot notice a missing one; no self-review catches
  this class): walk the design ADR by ADR (D50-D64), section by section
  (incl. section 2's catalog end-state, section 6's spec amendments with
  the land-together constraint, section 8's controller triggers), and
  name the task implementing each mandated element. A design element
  with no task, or a task element with no design mandate and no
  justification, is a finding. Check the plan's own coverage-mapping
  claim against your independent walk, not instead of it.
- **Latitude, both forms**: implementer-choice clauses AND unenumerated
  sets. The design's enumerations (D55 migration table, D57 config_path
  grammar and sub-grammars, D54 annotation table, D61 promotion sites,
  D64 pinning surface) must appear in the plan complete - transcribed
  or precisely referenced, never sampled. An unmarked list is
  exhaustive.
- **Verification soundness**: every absence-expectation step carries its
  fire-once event (break-and-restore, or the presence-control pairing);
  spot-execute at least three of the plan's verification commands
  including one grep whose pattern you validate against a known-present
  control. Counts recomputed from their enumerations
  (proc-normative-count-recomputed).
- **Stream/wave soundness**: verify the file-disjointness claims behind
  wave 1's five parallel worktrees and wave 3's G/H split against the
  tasks' actual Files blocks; a shared file between parallel streams is
  a finding. Verify the serialization rationale where the plan serializes
  (D53/D57 same widget files; D54/D59 same grid region; D58 consumes
  D57's prop).
- **writing-plans format**: complete code in code steps, exact paths,
  Files/Interfaces blocks with exact signatures, TDD ordering, no
  placeholders ("TBD", "similar to Task N", "add appropriate X"), the
  required header, global constraints verbatim from the brief. Reuse
  mandates name the exact signature with every argument traceable
  (proc-reuse-mandate-names-the-signature).
- **Type consistency**: names/signatures in later tasks match their
  defining tasks' Interfaces blocks.
- **House**: Tier-2 conformance (the 46-key budget, closed-domain
  dropdowns boundary, presentation-token carve-out limits, bilingual
  landing, unsigned-commit trailer conventions in commit steps if
  present).
- **Amended-design fidelity**: the plan matches the ROUND-4 design state
  (muxsmith_bare exception in Task 1, amended 6(a) wording in Task 21,
  amended hover semantics in Task 13); any surviving pre-amendment
  transcription is a finding.

## Verdict

Write `.superpowers/sdd/plan-7/plan-review-round-1.md`: APPROVED or
NEEDS FIXES; numbered severity-ranked findings with citations and
evidence; a HARVEST section (dominant patterns, repeated rejections,
over-restriction watch per the standing reviewer wiring); one-paragraph
whole-plan justification. Foreground probes only; the verdict file is
your only write.
