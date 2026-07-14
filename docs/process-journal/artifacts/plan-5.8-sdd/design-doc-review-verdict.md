# Independent review verdict: plan-5.8 decisions document

Artifact: `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md`
Reviewer: independent (did not author). Every cited file:line and quoted
string below was re-verified against the current working tree this
session; the implementer report was treated as unverified claims and
checked item by item. `git status` confirms the artifact is the only
tracked-tree addition.

## Verification performed

- All file:line citations re-read at source: validate.rs:61-63 and
  :274-289, model.rs:306-307/:311/:314-316, planner.rs:440-448 (param
  :447), :811-824 (param :822, comment :819-821), report/mod.rs:42,
  locales en:7/:43 and de:14/:50, catalog_completeness.rs:32-47/:39-47/
  :63-67/:387-395/:397, validate_semantics.rs:54,
  diagnosticFluentParams.ts:19-27/:25-26/:46-48, ROADMAP.md:85-87/
  :114-121/:186-192/:369, v1-design.md:178-192/:255/:266/:281/:297-299/
  :393-397, plan-3.5 decisions :156-160, product-boundaries.yaml:388-402.
  All correct, including every divergence-from-brief the implementer
  flagged (5.4 has no "at least one rule" prose; the 5.2
  InvalidPropertyValue row at :281 names no params; comment spans
  :819-821 with the param at :822; the fixture_args doc comment spans
  :32-47).
- core-83 statement quoted verbatim (character-identical to
  product-boundaries.yaml:394); planner comment and all four current
  catalog strings quoted verbatim.
- Emitter inventory confirmed complete: exactly three production
  `DiagCode::InvalidPropertyValue` sites (validate.rs:283,
  planner.rs:442, planner.rs:813; the report/mod.rs hits are a unit
  test). All non-language emissions set `allowed`, so the `*[other]` arm
  cannot leak.
- All four proposed Fluent bodies machine-validated with the repo's own
  @fluent/bundle 0.19.1 (parse without junk entries; select on
  `$property` resolves: language arm renders without `$allowed`, default
  arm renders the list, en and de). Variant indentation matches the
  catalog's existing select style (invalid-template, non-utf8-path).
  De register checked: du-imperative ("füge ... hinzu", "setze"),
  "Spurregeln", "Primärquelle", literal config keywords per the de
  catalog header; `*[other]` arms keep the current en/de wording
  verbatim.
- House conventions checked: self-contradiction sweep named per
  process-conventions.yaml:78 (spec-is-contract); SI-3 notes present in
  both ADRs per process-conventions.yaml:111; "pre-1.0 gate" is the
  ROADMAP's own section terminology ("Pre-1.0 release gates", :93).
  No deviation from a recorded house convention found.
- Publication hygiene: grep-clean for em/en dashes, curly quotes,
  Unicode ellipsis, NBSP, Unicode minus; no home paths, no
  agent-framework or private-project material; all references in-repo
  and following the existing house pattern (ADR numbers, session-10
  escalation, owner ruling, routed-items item).
- Internal ADR consistency: no slot contradicts another; D38 carries
  Supersedes + Spec amendments, D39 carries Spec amendments; slot order
  consistent across both ADRs and modeled on plan-5.7; D38's "no
  existing code's wire shape changes" is correctly scoped to D38 and
  does not conflict with D39's param removal.

## Findings

1. **Minor** - artifact lines 67-68: the path
   `crates/muxsmith-core/tests/validate_semantics.rs` is broken across a
   line boundary without a code span, so rendered Markdown shows
   "crates/muxsmith-core/tests/ validate_semantics.rs" with a spurious
   space. Every other path in the document stays on one line. Fix: keep
   the path unbroken (or backtick it whole).
2. **Minor** - D39's comment-update bullet names only the stale
   planner.rs:819-821 emitter comment, but the same change strands two
   further narrations the ADR itself cites: the
   catalog_completeness.rs:43-47 limitation text ("The one known
   instance -- `resolve_changes` emitting `InvalidPropertyValue` without
   `allowed` ... is fixed and pinned by ...") and the leak-test doc
   comment :392-395 ("Before the fix that added `.with(\"allowed\",
   ...)` ..."). After D39 both describe an inverted reality (the param is
   deliberately gone from both emitters; the leak test then pins the
   `[language]` arm). Not brief-mandated (R2.4 names only the planner
   comment), hence Minor and non-blocking; the execution plan should
   sweep these two doc comments in the same diff. Suggested fix: one
   sentence in the D39 test-topology bullet extending the comment update
   to the two catalog_completeness.rs narrations.

No Critical or Important findings.

## Verdict (a): requirement compliance

- R1.1 met - legality ruling stated, drop stays NoTrackRules, core-83
  quoted verbatim with correct :388-402 cite.
- R1.2 met - keep-only lift at validate.rs:61-63, new info
  `DiagCode::PassthroughProfile` (enum home report/mod.rs:42),
  config_path `tracks.rules`, no params, validate time,
  accidental-edit visibility; the `rules: []`-must-be-explicit note
  (model.rs:311) is verified and correct.
- R1.3 met - current en:7/de:14 quoted exactly; en+de replacements
  proposed, marked PROPOSAL, register-correct de.
- R1.4 met - passthrough-profile en+de proposed, marked PROPOSAL,
  no placeables (consistent with "no params").
- R1.5 met - placement named and marked PROPOSAL (end of "How it
  works", README.md:23, before :67); three use cases; GUIDE deferred
  to 1.0.
- R1.6 met - validate_semantics both shapes (existing :54 correctly
  identified as default-drop), catalog_completeness fixture via the
  exhaustive match, e2e dry-run + run over the D20 path with the spec
  5.2 EmptyPlan exemption (:266) cited.
- R1.7 met - Supersedes slot cites plan-3.5 :156-160 (quote matches
  source) and the annotation as a plan task.
- R1.8 met - 4.5 (:178-192) and new 5.2 row named; 5.4 checked against
  the actual tree (no "at least one rule" clause exists; recorded as
  no-edit-required with the policy-dependent conditional);
  self-contradiction sweep named. Verified independently: the spec
  nowhere states an unconditional rules requirement (no NoTrackRules
  row exists in 5.2).
- R1.9 met - ruling date/venue, core-83 recording, D20 executor path,
  "MUST be documented and hinted" all cited.
- R1.10 met - all four rejected alternatives with the required whys.
- R1.11 met - SI-3 MATCH, mkvmerge default-copy, gui trivial base case,
  declarative idiom.
- R2.1 met - select on `$property`, `[language]` arm without `$allowed`,
  `*[other]` keeps current wording verbatim; current strings quoted
  exactly; bodies proposed and marked PROPOSAL; machine-validated.
- R2.2 met - both planner emitters cited with verified param lines;
  closed-domain emitter explicitly unchanged with the locale-neutral
  rationale.
- R2.3 met - param REMOVAL, language-emissions-only, type/codec_kind
  unchanged, pre-1.0 no compatibility promise.
- R2.4 met - fixture switches to `*[other]` (property "type"), leak test
  pins `[language]`, single-fixture limitation respected (:39-47),
  planner comment update recorded with exact quote. (See finding 2 for
  a non-mandated adjacent gap.)
- R2.5 met - ROADMAP :85-87 trigger recorded as fired-and-consumed in
  the Triggers slot, both fix options, overclaim explained with the
  verified Number("1e3") mechanism against :25-26 and :46-48.
- R2.6 met - I2 (ROADMAP:186-192), root cause, spec 5.2:255 sentence,
  8.4 exception-list absence (:397), no-new-wire-element argument.
- R2.7 met - all four rejected alternatives with the required whys.
- R2.8 met - 5.2 row (:281) checked, names no `allowed`, no edit;
  8.4 no-change stated with the prose-leaves-core reasoning.
- R2.9 met - SI-3 MATCH against mkvtoolnix's localized catalogs.
- R3 met - all three out-of-scope items with correct refs (:369,
  GUIDE at 1.0, :114-121).
- R4 met - English, plan-5.7 register, typography grep-clean, every
  cited line verified correct against the tree, quotes exact, latitude
  passages marked PROPOSAL, public-repo clean.

No requirement unmet.

## Verdict (b): quality

At or above the plan-5.7 standard. Same slot architecture and register;
denser but justified citation coverage; the brief-vs-tree divergences
(5.4 premise, 5.2 row text, comment span, doc-comment span) were caught
and resolved per the brief's own check-the-actual-wording instructions
instead of transcribed; the proposals are syntactically valid Fluent
consistent with the existing catalog style in both locales; two
verified-value additions beyond the brief (the `rules: []` no-serde-
default note, the ROADMAP:187 staleness flag in the report) are correct.
The two Minor findings are polish-level. Passes.

## Harvest

- **Comment-coupling sweep pattern**: comments that narrate cross-site
  param coupling (planner.rs:819-821, catalog_completeness.rs:43-47,
  :392-395) form a set that goes stale as a group when the param
  story changes; a wire-param change should sweep every comment naming
  that param, not just the emitter-site one. Candidate for the
  convention ledger alongside the existing spec self-contradiction
  sweep (process-conventions.yaml:78) - same principle, comment layer.
- **ROADMAP line-number rot**: ROADMAP.md:187 carries stale
  planner.rs:428/:841 cites (current :440-448/:811-824). Dominant
  pattern worth recording: ROADMAP/backlog entries should anchor on
  symbols (fn names, DiagCodes), not raw line numbers, which rot on
  every refactor; specs/ADRs are point-in-time documents where line
  cites are fine.
- **Machine-checkable .ftl proposals**: proposed catalog bodies were
  validated pre-merge in a 30-line script against the repo's own
  @fluent/bundle. The de catalog header says selector/placeable parity
  is "reviewed manually, not machine-checked" - extending
  scripts/check-i18n.mjs to assert placeable-set and selector-structure
  parity per message id would close that gap cheaply and would have
  auto-guarded exactly the D39 class of change.

APPROVED

## Re-review round 1 (fix round 1)

Delta re-verified against the current working tree; the rest of the
document is textually identical to the approved round-0 version (checked
by full re-read against the reviewed copy, not by trusting the fix
report). `git status` shows the artifact still as the only new file; the
`docs/ROADMAP.md` / `docs/decision-ledger.yaml` modifications are the
controller's harvest bookkeeping, out of review scope per the
coordinator, and were ignored.

- Finding 1 resolved: the validate_semantics.rs path is now a single
  unbroken backticked span
  (`crates/muxsmith-core/tests/validate_semantics.rs`, artifact line 68).
  The surrounding rewrap changes no content; line 70 wraps a little
  short, cosmetic only.
- Finding 2 resolved: the D39 test-topology bullet (artifact lines
  205-213) now extends the comment sweep to the two
  catalog_completeness.rs narrations, correctly identified and cited:
  the limitation text at :43-47 and the leak-test doc comment at
  :392-395. Both spans and both elided quote fragments ("The one known
  instance ... is fixed and pinned by ..."; `.with("allowed", ...)`)
  re-verified verbatim against the test source. The stated post-D39
  reality (param deliberately absent from both emitters, leak test pins
  the `[language]` arm, sweep in the same diff) is accurate and
  consistent with the rest of the ADR.
- No new defect introduced: the touched passages are glyph-clean
  (grep re-run: no em/en dashes, curly quotes, ellipsis, NBSP), contain
  no out-of-repo references, and contradict no other slot.

Both findings closed. No findings open.

APPROVED
