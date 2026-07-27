# Plan 8 plan review brief (round 1)

Independent reviewer, fresh eyes; you did not author the plan. Artifact
under review: `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md`
(commit a0a8dea, 6 tasks). Ground truth: the owner-approved design
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(D75-D90, R1-R10 acceptance checklist), the controller plan brief
`.superpowers/sdd/plan-8/plan-brief.md`, the house template
`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md`, the ROADMAP
"Ledger hygiene" rider ruling, the Tier-2 house files, and the ACTUAL
TREE incl. `.github/workflows/ci.yml`. Verify, never believe.

## Dimensions

1. **COVERAGE (load-bearing)**: walk D75-D90 and every design section
   (workflow sketch, config blocks, guard scripts, INSTALL.md outline,
   naming/SHA256SUMS, rehearsal checklist, triggers) yourself; name the
   task implementing each; grade the plan's map against your walk. The
   ROADMAP rider (ledger-lint CI wiring + duplicate-key extension) is
   REQUIRED scope - its absence or dilution is a finding.
2. **The rider-vs-D83 adjudication**: the author resolved the candidate
   collision in-plan (D83's "ci.yml is not modified" scoped to the
   release pipeline; Task 5 appends one additive ci.yml job with a
   no-existing-line-changed check). Verify the adjudication's evidence
   yourself: does the design's text actually scope D83 as claimed
   (quote it), and is the additive-only check in the task executable
   and fire-verifiable? If the evidence does not carry the reading,
   that is a finding requiring controller routing, not your own ruling.
3. **Latitude scan, BOTH forms, per task text**; design section
   "what the implementer must not decide" (or its equivalent)
   transmitted undiluted.
4. **Template conformance**: plan-7 structural conventions (no-progress
   header deviation, tracker `.superpowers/sdd/plan-8/progress.md`,
   binding SDD execution-method section, global constraints per the
   brief incl. session-relocation ban, gh-log duties on every
   GitHub-touching step, plan-close pre-registrations: rehearsal-draft
   cleanup, INSTALL.md owner pass, salvage items).
5. **Transcription fidelity**: re-diff every transcribed block (R1-R10,
   G1-G5, config blocks 3.1-3.3, 4.4, leg table, asset set) against the
   design yourself, byte-level modulo indentation.
6. **Dependency graph vs reality**: the 4-stream file-disjointness
   claim checked against the actual files each task touches; the
   rehearsal wave's sequencing constraint (release.yml must be merged
   to master and pushed before workflow_dispatch can run) explicit and
   correctly ordered; merge order + full gate after every merge stated.
7. **Citations and counts**: every :line/:symbol ref at the current
   tree; every count recomputed (walk every numeral - the plan-7 plan
   review's repeated defect class).
8. **Model-tier classification** (proc-03): T1/T2 cheap-tier claims
   hold only if those tasks are transcription-complete in the plan
   (could a transcription-tier implementer execute with zero judgment?
   verify against the design sketches the plan cites); mid elsewhere;
   reviewers mid.
9. **Implementability walk**: each task executable as written by a
   fresh implementer - foreground runs with explicit timeouts on gh
   watches, fire-verifications carried (G1-G5 re-run pre-merge; the
   rider's broken-fixture fire-test), NEEDS_CONTEXT routing stated,
   no background-run-plus-monitor anywhere.
10. **Rehearsal task evaluability**: R1-R10 each evaluated at its named
    emitter in the task steps; the draft-release rehearsal cleanup step
    present; nothing in the task publishes anything (owner publishes
    nothing in this plan).

## Output

Write `.superpowers/sdd/plan-8/plan-review-round-1.md`: verdict APPROVED
or NEEDS FIXES; findings by severity with location and what to change; a
HARVEST section. Final message: verdict word + at most three lines + the
file path.

## Constraints

Read-only except your verdict file; no git writes; never call
EnterWorktree/ExitWorktree or any session-relocation tool; absolute
paths; anything you run, run foreground.
