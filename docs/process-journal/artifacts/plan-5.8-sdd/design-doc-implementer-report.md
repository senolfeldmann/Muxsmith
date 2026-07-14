# Implementer self-review: plan-5.8 decisions document

Artifact: `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md` (the
only file created; `git status` confirms no other tracked file touched).
Every cited file:line below was verified by reading the current working
tree this session.

## Per-requirement

- R1.1: Decision opens with the legality ruling (keep+zero legal, drop+zero
  stays NoTrackRules) and quotes core-83's statement verbatim in full,
  including the trailing provenance sentence (product-boundaries.yaml:394).
- R1.2: validate.rs:61-63 lift for keep only; new info
  `DiagCode::PassthroughProfile` (enum home report/mod.rs:42), config_path
  `tracks.rules`, no params, validate time, accidental-edit visibility
  stated; also notes `rules: []` must be explicit (model.rs:311, no serde
  default).
- R1.3: current en:7/de:14 strings quoted exactly; replacement en+de hint
  strings proposed, marked PROPOSAL, de matches catalog register
  ("Spurregeln", du-imperative "füge ... hinzu / setze").
- R1.4: `passthrough-profile` en+de bodies proposed, marked PROPOSAL
  (de uses the catalog's "Primärquelle" register).
- R1.5: placement named (new `###` at end of "How it works", README.md:23,
  before "What you get" :67), marked PROPOSAL; content = unmatched:keep +
  rules:[] idiom plus the three use cases; GUIDE deferred to 1.0.
- R1.6: all three test groups carried: validate_semantics keep+zero->info /
  drop+zero->error (existing :54 assertion noted as the default-drop shape),
  catalog_completeness fixture (exhaustive match :32-47), e2e dry-run+run
  via the D20 machinery with the spec 5.2 EmptyPlan exemption (:266) cited.
- R1.7: Supersedes slot names the D20 "Open mechanics" assumption
  (plan-3.5 file :156-160, H12) and the "superseded by D38" annotation as a
  plan task.
- R1.8: Spec amendments slot names 4.5 (:178-192) and the new 5.2 row;
  for 5.4 I checked the actual wording as instructed - the "at least one
  rule" clause does NOT exist in 5.4 (:299) or anywhere in the spec; the
  wording lives only in the catalog message (en ftl:7). Recorded as
  no-5.4-edit-required, with the policy-dependent condition if the plan
  adds the check to the enumeration; self-contradiction sweep named.
- R1.9: Rationale cites the 2026-07-13 owner ruling (escalation resolution,
  session 10; core-83 :388-402), the 2026-07-14 approval of the decision
  set, D20 executor path, and core-83's "MUST be documented and hinted".
- R1.10: all four rejected alternatives with the given reasons.
- R1.11: SI-3 MATCH with mkvmerge default-copy semantics, gui trivial base
  case, unmatched:keep+zero-rules as the declarative idiom.
- R2.1: Fluent select on `$property`, [language] arm without `$allowed`
  using the "must be a valid ISO 639 or BCP-47 language code" wording
  (localized in de), `*[other]` arm keeps current wording verbatim; current
  en:43/de:50 quoted exactly; both proposed bodies given, marked PROPOSAL.
- R2.2: both planner.rs emitters cited (walk_exact_languages :440-448 param
  :447; resolve_changes :811-824 param :822) drop `allowed`; validate.rs
  :274-289 closed-domain emitter explicitly unchanged (locale-neutral
  domain_hint tokens).
- R2.3: wire slot records param REMOVAL for property=language emissions,
  type/codec_kind unchanged, pre-1.0 no compatibility promise.
- R2.4: fixture_args (:63-67) switches to the `*[other]` arm (property
  "type", real list); leak test (:397, doc :387-395) pins the language arm;
  single-fixture-per-code limitation (:39-47) respected; stale comment
  :819-821 (quoted exactly) update recorded.
- R2.5: ROADMAP trigger (:85-87) recorded as fired-and-consumed by Plan
  5.8, same-diff fix of the diagnosticFluentParams.ts:19-27 overclaim
  (Number("1e3") === 1000 passes the :46-48 check) or the /^\d+$/ guard.
- R2.6: rationale cites whole-branch I2 (ROADMAP:186-192), root cause =
  English prose in a param, spec 5.2:255 sentence, absence from the 8.4
  exception list (:397), and selection-on-existing-param = no new wire
  element.
- R2.7: all four rejected alternatives with the given reasons.
- R2.8: 5.2 row text checked (:281) - names no params, no edit required;
  8.4 explicitly needs NO change because the prose leaves core.
- R2.9: SI-3 MATCH against mkvtoolnix's fully localized catalogs.
- R3: closing "Deliberately out of scope" section names all three items
  (D18 remainder ROADMAP:369; GUIDE at 1.0; D32-addendum README anchor
  ROADMAP:114-121 untouched).
- R4: English; grep-verified free of em/en dashes, curly quotes, Unicode
  ellipsis, NBSP; every code claim carries verified file:line; catalog
  strings and the planner comment quoted exactly from the working tree;
  wording-latitude passages marked PROPOSAL; only in-repo references.

## Flags

1. Brief-vs-tree divergences, all resolved by the brief's own
   "check the actual wording/row" instructions and recorded in the ADRs:
   spec 5.4 contains no "at least one rule" prose (R1.8); spec 5.2's
   InvalidPropertyValue row names no `allowed` (R2.8); the planner comment
   spans :819-821 with the param at :822 (brief said :819-822); the
   fixture_args doc comment spans :32-47 (brief said :41-47) - cited
   ranges follow the verified tree.
2. ROADMAP.md:187 carries stale emitter line numbers (planner.rs:428/:841
   vs current :440-448/:811-824). Out of my scope (no other file
   modified); the Plan 5.8 execution may refresh that entry when it
   consumes the gate.
3. Catalog string proposals (no-track-rules hint, passthrough-profile,
   invalid-property-value selector) and the README placement await owner
   wording review, as the brief mandates - marked PROPOSAL in the
   document.

## Fix round 1 (post-approval Minor findings)

1. Broken path render: the `crates/muxsmith-core/tests/validate_semantics.rs`
   path in the D38 test bullet was split across a line boundary without a
   code span; now whole on one line in backticks (wrap of the surrounding
   sentence tidied).
2. Stranded narrations: the D39 test-topology bullet now extends the
   comment update beyond planner.rs:819-821 to the two
   catalog_completeness.rs narrations the ADR cites - the fixture_args
   limitation text (:43-47) and the leak-test doc comment (:392-395) -
   which would otherwise describe an inverted reality after the param
   removal; the execution plan sweeps both in the same diff.

Typography re-verified clean after both edits. Note: `git status` now also
shows `docs/ROADMAP.md` and `docs/decision-ledger.yaml` modified - not by
me (my ROADMAP access was read-only; I never opened decision-ledger.yaml);
these appeared between rounds, presumably controller-side. My changes
remain confined to the artifact and this report.

DONE
