# Plan 7 design review, round 2

Artifact: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
(1609 lines, post fix-round). Same reviewer as round 1; scope is the
delta against the round-1 verdict plus regression watch. Every disposition
below was verified on the artifact and the tree, not from the author's
report; the corrected grammar was re-fire-verified with my own probe
profiles through the real binary.

## Verdict: APPROVED

(One new Minor - a wrong internal section pointer, below - is routed to
the controller's consuming edit alongside triggers 9/10; it is a
typo-class defect that can make no implementer invent anything, and does
not warrant a fix-round dispatch.)

## Per-finding disposition

1. **BLOCKER, D57/§1 grammar enumeration - VERIFIED FIXED.** The table
   was re-derived and now matches my independent round-1 source
   measurements point for point: `input.extensions` (:55-57) present and
   :55 no longer misattributed to input.pattern; bare `tracks` removed
   with the explicit negative ("no bare `tracks` path exists" -
   PassthroughProfile anchors at `tracks.rules`, :70-73, confirmed); bare
   `{p}.any`/`{p}.not` (EmptyMatchList :350-355/:361-366) added; the
   phantom locator `.path` replaced by the real bare-locator
   LocatorConflict (:460-465) with the explicit "no `.path` suffix
   exists"; the phantom "template suffixes" replaced by "every template
   diagnostic anchors at the bare root" (:487-530). The new
   `config_diagnostics` scope citation (validate.rs:189-198) verified
   exact. **Fire-verification independently reproduced**: my own probe
   profile through `./target/release/muxsmith validate --json` returned
   exactly `input.extensions` (empty-extensions), `tracks[0].match.any`
   (empty-match-list), `tracks[0].source.external` (locator-conflict),
   `output.filename.template` (unknown-template-field) and
   `tracks[0].match` (empty-match-expression); a second probe confirmed
   the `tracks.rules` anchor and the absence of any bare `tracks` path.
   D57 now transcribes the three previously-implicit anchor placements
   (ListWidget root for bare any/not, locator SectionWidget for the bare
   locator root, template TextWidget for template paths) and drops the
   dead `tracks` anchor. The new "out of band" load-failure note was
   verified real (`EditorView.vue:190-192` puts `loadProfile`'s
   `config_diagnostics` into the same ref; `load.rs:26` documents the
   load-derived config_path) - it correctly lands in the panel-only
   fallback.
2. **MAJOR, D61 TS-side inventory - VERIFIED FIXED.** Now: five direct
   `$t(x.code, x.params)` sites (incl. `RunHistory.vue:241`, correctly
   flagged as the site rendering the promoted `index`), three ref pairs
   (incl. `EditorView.vue:121-122`), three ref-fed render sites - all
   eight matching my measured inventory exactly. Correction #4 now also
   records the recon list as incomplete rather than merely confirmed.
3. **MAJOR, missing spec amendments - VERIFIED FIXED.** Amendment 6 is
   restructured into (a) additions, (b) **modifications of stated
   mechanics** - the pin-release enumeration gains the view-switch
   condition, the Esc rule gains the settings-dialog qualifier - and (c)
   the E3-ruled activation semantic. Both round-1 deviations are now
   amended, not silently narrowed.
4. **MAJOR, count contradictions - VERIFIED FIXED.** D55 headline now "27
   ids fold away, 1 is renamed in place" with the correct 21+1/6
   breakdown; §0 postscript now "20 `:title` sites - 19 static bindings
   plus `BatchView.vue:507`'s computed `runTooltip`". The author's two
   additional self-found defects of the same class check out against my
   measurements: the D55 steelman now says 28 ids / 20 `:title` sites +
   6 hint render sites (all three numbers recompute correctly), and the
   interface-changes line now counts 20 + 6 (previously "19 template
   sites", which also ignored the hint sites). No stale count found
   anywhere else in the delta.
5. **MAJOR, CSS latitude clause - RESOLVED BY OWNER RULING, VERIFIED
   CONFORMANT.** `latitude-carveout-presentation-tokens`
   (`process-conventions.yaml:456`) read: presentation tokens
   implementer-owned, semantic-carrying mappings enumeration-bound. D52
   now cites the carve-out and enumerates exactly the semantic layer the
   entry excludes from it: the two-class structure, help-hover=faint vs
   help-pinned=prominent with required distinguishability,
   outline-not-border. D57's severity-class mapping was already
   enumerated. The clause is now a recorded carve-out application, not a
   latitude clause; the header's no-latitude claim names it.
6. **Minor, `code` param - VERIFIED FIXED** (listed with the correct
   `"signal"`-fallback rationale, error.rs:87-91).
7. **Minor, Tier-2 citation - VERIFIED FIXED.** `editor-generic-action-keys`
   (:404) cited correctly in Grounding and D59; D59 now surfaces the
   45 -> 46 **budget revision** explicitly (attributes add zero ids, the
   46th id is uncovered growth, ROADMAP anchor as substantive cover,
   43 -> 45 owner-ruling precedent) and creates trigger 10 assigning the
   entry update to the controller.
8. **Minor, D62 check 1(c) - VERIFIED FIXED.** Dedicated scan
   (`/['"](view-[a-z-]+)['"]/g` over `src/help/state.ts`), the false
   shape-(a) premise explicitly corrected, the redundancy with shape (b)
   named and justified; `VIEW_TOPICS` is now part of D52's declared
   module contents ("the module's complete contents").
9. **Minor, correction #4 measurement - VERIFIED FIXED** (":46-48, line
   template, severity, code").
10. **Minor, D54 app-shell classification - VERIFIED FIXED.** The method
    sentence now names app-shell chrome and all 20 `:title` sites; four
    new exclusion rows (settings-open button, nav tabs with verified cite
    App.vue:73-106, help toggle with the self-reference rationale,
    SuggestionCard copy/apply buttons) each carry a reason. I mapped all
    20 `:title` sites to classification rows - none is unclassified now.

## Owner rulings, conformance verified

- **E1 (CLI English-only)**: `cli-english-only`
  (`product-boundaries.yaml:433`) read. Ground truth §1, out-of-scope §5
  ("a decree, not a v1.x deferral"), §7, §9 (new bullet: no de embedding,
  no locale rendering path, only amendment 3's rustdoc wording) and
  amendments 1/2/3/5 all carry the ruled shape; amendment 5 removes the
  dangling `--locale` promise; amendment 3 explicitly avoids a
  pending-content note. Full-document scan for leftover
  pending/variant/escalation language: only historical or negated uses
  remain.
- **E2 (three spec views)**: ruled as recommended; D54's 22/44 totals
  stand; FirstRun/settings rows and D52's toggle paragraph cite the
  ruling.
- **E3 (global suppression + allowlist)**: ruled as designed; D52's
  decision and its rejected alternative both marked ruled; amendment
  6(c) carries the semantic into spec 8.3.
- **Section 7** is now "Escalations: ruled", analysis retained, losing
  variants marked moot; trigger 1 marked CONSUMED with numbering kept
  stable; trigger 10 added. The header's "Every fork in this document is
  closed" claim survives the round-1 latitude standards applied to the
  delta.

## New findings in the delta

1. **Minor - wrong internal cross-reference (pre-existing, missed in
   round 1, not an author regression):** line 106, "mkvtoolnix (SI-3
   source facts, full audit in **section 5**)" - the parity audit is
   **section 3**; section 5 is the out-of-scope list. A full sweep of
   every "section N" reference in the document found no other mismatch
   (the sibling defect at :157, "trigger (section 9)", was silently and
   correctly fixed to section 8 in this round). One-word fix; routed to
   the controller's consuming edit together with triggers 9 and 10.

No regressions found: sections not named in the fix report (D50, D51,
D53, D56, D58, D60, section 2, section 3, section 4) were spot-checked
against my round-1 reading and are unchanged where they should be; the
section-2 arithmetic still recomputes correctly against the fixed D55.

## HARVEST

- **Fire-verification of corrected enumerations**: the fix round did not
  just re-cite the grammar, it built a probe profile and demonstrated
  each previously-wrong member firing through the real binary - and the
  probe is cheaply reproducible (I re-ran it independently and got the
  same members). Strong candidate for a recorded convention: *a
  ground-truth enumeration corrected after a review finding is
  demonstrated by execution, not only re-cited* - the natural extension
  of the house's verified-negative-with-control rule to positive
  enumeration claims.
- **Trigger-numbering stability**: marking a consumed trigger CONSUMED
  while keeping its number so cross-references stay stable is a tidy
  micro-pattern worth keeping.
- **Recorded-redundancy pattern**: D62 check 1(c) keeps a deliberately
  redundant check and says so with the failure mode each copy catches -
  the honest alternative to silently deleting a "duplicate" guard.
- **Round-1 harvest items now closed by Tier-2 entries**: the
  presentation-token fork (ruled: `latitude-carveout-presentation-tokens`)
  and the CLI-localization question (ruled: `cli-english-only`). Still
  open and carried: the IpcError render-funnel candidate (eight scattered
  render sites; Plan-9 neighborhood) and the D62 URL ban's
  plain-text-URL over-breadth (correctly kept per the safeguard rule;
  measure after build).
- **Over-restriction watch**: nothing in this round was stopped that the
  structural-conformance grant's spirit should have covered. The
  presentation-token ruling is itself the round-1 watch item resolved in
  the loosening direction, with the semantic/token boundary drawn where
  the round-1 verdict suggested; first calibration data for the new
  carve-out should come from the plan-7 implementation tasks.

## Whole-document justification

Every round-1 finding is fixed on the artifact, and fixed the right way:
the blocker was resolved by re-derivation plus a reproducible execution
proof rather than prose adjustment, the enumeration fixes match the
measured tree exactly (not merely the report's claims), the two spec
deviations became explicit amendments in a restructured
additions-vs-modifications form, and the process-level finding was
resolved above the document by owner rulings now recorded in Tier-2,
which the document cites and conforms to precisely - including
enumerating the semantic layer the new carve-out explicitly refuses to
cover. The author additionally found and correctly fixed two count
defects of the class I had flagged, in places I had not listed, and the
fix round introduced no regression I could find. The one residue is a
single wrong internal section pointer that predates the fix round and
can mislead no implementer into inventing anything; it rides the
controller's consuming edit. The document now does what it claims: every
mandated set is enumerated and measured, every fork is closed or ruled,
and a plan can be written from it without keyboard-side invention.
