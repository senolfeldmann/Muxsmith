# Plan 7 design review, round 5 (Task-1 NEEDS_CONTEXT re-fold)

Artifact: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`.
Same reviewer; scope is the D64 amendment following execution Task-1's
refutation of the schema-through-the-funnel premise, plus its ripples
(mechanism bullet, Where-it-applies row, section 9). Everything else is
settled by rounds 1-4 and owner approval. All claims verified on the
tree and the built binary; foreground.

## Verdict: APPROVED

No findings. The delta is clean, and the refutation it folds in is
independently confirmed.

## Dispositions

1. **Premise-refutation facts - VERIFIED, all four, independently.**
   - `Schema` is a unit variant with no arguments (`cli.rs:34`: bare
     `Schema,` between the Validate block and DryRun - no fields, no
     `locale` arg; the four flag-bearing subcommands are
     Validate/DryRun/Identify/Run at :31,:55,:66,:93, verified in round
     3 and correctly named by the rewritten mechanism bullet).
   - `muxsmith schema --locale en` measured on the built binary:
     `error: unexpected argument '--locale' found` / `Usage: muxsmith
     schema`, **exit 2** (clean capture) - the funnel conversion would
     indeed turn the schema tests red exactly as the implementer
     measured. Bare `muxsmith schema` emits the JSON Schema, exit 0.
   - The `Schema` arm is renderer-free: `main.rs:11-15` prints
     `schema_for!(Profile)` and returns 0 with no `Renderer::new` (the
     Renderers live only in the other arms, :21+).
   - Permanence: spec 8.4's accepted-exception list names the JSON
     Schema's `description` fields (Rust doc comments, D47) as
     English-only by design - so the schema output's locale-independence
     is a spec position, not an implementation accident. The ADR's
     "permanently, not incidentally" is the correct characterization.
2. **The two-caller exception block - SOUND AND CLOSED.** Caller 1's
   rationale is unchanged from round 4 (verified there: silent
   test-meaning change, measured). Caller 2's rationale chains the four
   verified facts above; both callers share the "no Renderer on either
   path" locale-moot argument, now stated once for both. The closed-ness
   wording is complete: exactly these two callers, a third reopens D64 -
   the reopening rule from round 4 was itself the mechanism by which
   this very amendment happened, which is the rule working as written.
   The rejected add-`--locale`-to-`Schema` alternative is correctly
   argued on both grounds: D64's own retained anti-clap-global argument
   (user-visible syntax widening for zero pinning gain) and the spec's
   English-only-schema position, with no contradiction against the
   retained rejection blocks. Provenance recorded (controller ruling at
   execution, internal technical fork, test-support only) - correctly
   routed.
3. **Sweep - CLEAN.**
   - The refuted "routes through the funnel ... pinned regardless"
     wording survives only inside the corrective quotation in the
     Where-it-applies row (:1401-1402), which names the Task-1
     NEEDS_CONTEXT as the refuter - legitimate historical record. No
     stale one-caller or funnel-covers-schema wording anywhere else.
   - Section 9's D64 line now names both callers and the
     third-caller-reopens rule, consistent with the ADR verbatim.
   - The grep invariant is genuinely unaffected: the bullet is
     byte-identical, and both exception callers route through
     `support::muxsmith_bare()` in `tests/support/mod.rs`, so
     `cargo_bin("muxsmith")` still ends up in exactly one file
     post-sweep.
   - Snapshot recount: `tests/snapshots/` holds exactly **11** files,
     split 3/3/4/1 across cli_validate/dry_run_cli/run_cli/run_live with
     none under cli_schema - so the reworded coverage sentence ("the
     funnel covers all 11 insta snapshots and every locale-sensitive
     assertion; cli_schema's two tests are locale-independent by
     construction") is exactly right, and tighter than the round-3
     version it replaces.
4. **No regression elsewhere in D64.** The other four Where-it-applies
   rows are unchanged and still match my round-3 measurements
   (cli_validate 1/3, dry_run_cli 13/3, run_cli 1/4, run_live 1/1); the
   env-pinning and clap-global rejection blocks, the e2e negative, and
   the not-pinned renderer-tests bullet are all untouched. The
   `schema_json()` cite (`cli_schema.rs:5-15`) is exact - it is the
   file's shared invocation, currently a direct `cargo_bin` +
   `.arg("schema")`, precisely what the sweep converts to the bare
   helper.

## HARVEST

- **The reopening rule paid for itself immediately**: round 4's "a
  second caller reopens D64 rather than riding the helper" is exactly
  what happened - the fork went NEEDS_CONTEXT -> controller ruling ->
  design amendment -> re-review, instead of the implementer quietly
  widening the helper. The exception-with-reopening-trigger shape is now
  validated by one live traversal; keep it as the standard form.
- **Calibration data for the fire-verification convention candidate
  (self-critique included)**: the refuted premise was checkable all
  along - `Schema`'s unit-variant shape sat at cli.rs:34 while D64
  claimed funnel coverage for it, and my round-3/4 measurements verified
  flag-after-positional on `validate` only and let the generalization
  ride. Lesson worth recording with the convention candidate: fire-verify
  per shape class (here: flag-bearing vs unit subcommands), not per
  representative - a claim quantified over N variants needs a probe per
  variant shape, which is the same per-member discipline
  proc-latitude already demands of enumerations.
- **NEEDS_CONTEXT as designed**: the implementer refuted a design
  premise with a measurement, stopped, and the fork came back as a
  recorded ruling - the proc-57 refute-and-record path and the
  proc-latitude no-keyboard-resolution rule composing correctly at
  execution time. Second live confirmation after plan-authoring (round
  4) that the layered gates catch what document review sampled past.
- **Over-restriction watch**: nothing wrongly stopped. The funnel rule
  hit its second legitimate outlier and the resolution again narrowed
  the exception to an enumerated, greppable, reopening-guarded set
  rather than loosening the invariant - and notably the coverage
  sentence got *stronger* in the amendment (locale-sensitive scope
  stated precisely instead of blanket "every assertion").

## Whole-document justification

The delta folds an execution-measured refutation into D64 exactly the
way this document series has established: the refuted premise is quoted
and attributed rather than silently rewritten, the corrected claim
chains four facts that all verify independently on the tree and binary
(unit variant, exit-2 rejection, renderer-free arm, spec-anchored
English-only schema), the exception set stays closed with its rationales
per caller and a reopening trigger that has now demonstrably functioned
once, and the greppable invariant survives untouched because both
exceptions live inside the same support module the invariant names. The
snapshot recount and the other four enumeration rows still match my own
round-3 measurements, so nothing regressed around the edit. Approved
without reservation; the one durable lesson (verify quantified claims
per shape class, not per representative) is recorded in the harvest, on
the reviewer as much as the author.
