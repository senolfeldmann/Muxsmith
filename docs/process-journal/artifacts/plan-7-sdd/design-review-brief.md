# Plan 7 design review brief (round 1)

Independent review of the Plan 7 design document
`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md` (author: a
separate fresh implementer; you have no stake in it). You grade; you do
NOT fix. The governing human reviews only after your APPROVED.

## Ground truth (in order)

1. The v1 spec `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
   (esp. §8.2, 8.3, 8.4, 10) - authoritative.
2. The controller design brief
   `.superpowers/sdd/plan-7/design-brief.md` - scope (11 inputs + rider),
   binding owner decisions, the 14 design questions.
3. The Tier-2 house files: `docs/product-boundaries.yaml`,
   `docs/conventions.yaml`, `docs/process-conventions.yaml`.
4. The real tree and installed packages - verify empirical claims
   yourself, never from memory.

## Dimensions (all mandatory)

- **Coverage**: walk the brief section by section. Every scope input
  (all 11 + rider), every owner decision, every design question 1-14 must
  be decided in the document, and every decision must name its ADR. A
  scope item with no decision, or a decision the brief did not ask for
  and the doc does not justify, is a finding.
- **Latitude, both forms**: explicit implementer-choice clauses
  ("either works", "the implementer may") AND omission latitude - any
  set the design mandates but does not enumerate (a "one per X" without
  the X list, an ellipsis, an unmarked exemplary list). Test every
  normative sentence: must a future implementer invent something it is
  not allowed to invent? The enumerations to check hardest: the D54
  annotated-control set (must classify ALL 42 editor fields and all
  non-editor controls include/exclude with reasons, and recount), the
  D55 migration table (all existing suffixed ids accounted), the D57
  config_path grammar enumeration, the D61 promotion-site and
  plural-message enumerations, the D62 check list.
- **House**: conformance with recorded conventions (Tier-2 files) -
  flag deviations from a recorded convention; also flag NEW patterns
  the design establishes that no entry covers (for harvest, not
  necessarily as defects).
- **No-work-needed check**: wherever the document concludes a guard,
  enumeration, check or piece of work is unnecessary ("already
  covered", "cannot happen", "nothing to build"), verify the claim that
  makes it unnecessary - run it, grep it, read the cited lines. The
  author's report claims several such (e.g. "$ta ships natively in
  fluent-vue 3.8.2", "e2e already real-parses all catalogs", "no
  tooltipKey field needed", "Diagnostic wire format untouched") - these
  are exactly the claims to execute, not weigh. For any check whose
  passing result is an absence (a grep with no hits), confirm the
  pattern against a known-present control first.
- **Empirical/external claims**: registry-verified versions (marked
  18.0.7 exact-pin, its 0-dep/size claims), fluent-vue 3.8.2 API claims
  (bundles setter, $ta), CLDR plural-category carve-out correctness,
  the mkvtoolnix parity citations (spot-check at least three at
  ~/Downloads/mkvtoolnix, incl. one negative claim with a positive
  control). Foreground runs only.
- **Internal consistency**: ADR slots complete (decision / rationale /
  rejected alternatives WITH steelmen / triggers created / interface
  changes); the spec-amendment list complete and consistent with the
  decisions; the escalation section's variants (E1-E3) each carry both
  rulings' consequences; no section contradicts another; the document
  stands alone without the author's chat report.
- **Safeguard rule**: the design must not have argued out any guard it
  itself proposed earlier in the document; proposed guards stay until
  built and measured redundant.

## Verdict

Write your verdict as a FILE:
`.superpowers/sdd/plan-7/design-review-round-1.md`, containing:
- Verdict: APPROVED or NEEDS FIXES.
- Numbered findings, severity-ranked (Blocker / Major / Minor), each
  with file/section citation, what is wrong, and the evidence.
- A **HARVEST** section: observed dominant patterns, repeated
  rejections, any new convention candidates; plus the over-restriction
  watch (flag any stop you judge the structural-conformance grant's
  spirit should have covered).
- A one-paragraph whole-document justification of the verdict.

Your final chat answer: the verdict line, the findings list (compact),
and the harvest - the file is the artifact of record.
