# Task 16 reviewer verdict (model: opus, 2026-07-12)

Diff: 23d3125..27c8b79 on plan55-t16

## Adjudication: removed general skew warning
The old planner.rs:445 emission was a per-file SCHEMA-DRIFT ADVISORY
(fires on every newer-schema file, independent of profile content; 50
identical warnings on a 50-file batch restating one per-run fact; and it
announced untyped matching that pre-D32 never happened - mislabeled).
D32's "was dead code" premise was FALSE, so its text cannot be read as a
deliberate removal decision; but D32's param reshape makes keeping both
under one DiagCode impossible - it forces a choice it does not make.
VERDICT: open-question-for-Şenol, narrowly scoped: should a schema-drift
advisory (once per batch, own diagnostic) survive, or is dropping it
acceptable? Removal-as-implemented is defensible (crude, mis-scoped
signal); the decision is his, not the implementer's.

## Spec Compliance
✅ B-1..B-7, B-9..B-11 exactly per the binding table (B-1 hard-reject
byte-unchanged; raw: strip before type/domain checks; skew fires per
consumed raw: prop incl. nested any/not via collect_raw_props; B-10 at
found==pinned; B-11 alongside MissingTrack). Spec sweep independently
confirmed complete (no residual automatic-on-skew wording; §4.3/4.4/5.2/
5.4/9.2 consistent). Fixture lockstep faithful for all three codes. Both
marked assumptions consistent with B-2/B-4.
⚠️->Şenol: B-8 memo row self-contradictory; implemented single-field
verbatim (raw:language reads only `language`; de vs ger = no match). The
literal row implies dual-field (match via language_ietf's literal de).
Single-field is the more defensible engineering reading (raw: opts out of
exactly that dual-field magic); ratification needed.

## Issues
Critical: none.
Important: the two Şenol ratification items above (recorded in memo +
ROADMAP with vehicle; NOT blocking merge - reviewer explicit).
Minor: bare `raw:` (empty name) accepted, yields RawProperty with
property="" and never matches; no panic on any path; cosmetic wart.

## Assessment
Spec compliance ✅ (with the two flagged human decisions). Task quality:
Approved conditional on ratification; merged as-implemented.
