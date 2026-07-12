# Task 18 reviewer verdict (model: opus, 2026-07-12)

Diff: 23d3125..f68e5d7 on plan55-t18

## Deviation adjudications
1. TC-A rank-0 (default_track vs forced_track): ILLUSTRATIVE, not a spec
   miss. Both flags rank-equal; the memo's binding tiebreak
   (broader-rule-then-index) is scoped to different-rule narrowings and
   says nothing within-rule/within-rank; id2 carries both flags. The
   "honor the illustration" fix would need a contested-property selection
   dimension - which D33's no-new-mechanism constraint forbids. Alphabetical
   within-rank tiebreak deterministic (stable sort over BTreeSet-seeded
   list). Memo wording corrected (see below).
2. Partition group-branch unreachability: sound for the D33 cases (traced:
   an isolation-resolving batch-safe narrowing becomes a suggestion, so
   the partition never runs there); the report's "effectively never fires"
   OVERSTATES - a contrived multi-file batch does reach the branch, which
   the kept defensive implementation then handles correctly. TC-C rewrite
   is FAITHFUL AND STRONGER (pins why rejection happens: batch collateral).

## Spec Compliance
✅ TC-A/B/C/D all pass through the correct mechanisms (acceptance filter +
isolation check; grep-confirmed no hard-coded two-required early-out).
Forced-list constraints honored; no new edit variant/DiagCode/Fluent
message (tripwire never fired). Named risks: (a) (file,track)-keyed
acceptance sound - narrowing is monotone, cannot mint overlaps, multiset
guard backstops; (b) genuinely symmetric incl. claimant 0; (c) TC-B via
filter, not early-out; (d) tiebreak total order, breadth map total.

## Issues
Critical/Important: none.
Minor:
1. Report's "never fires" phrased as theorem; memo note should say "not
   in the D33 cases". Corrected controller-side.
2. Dead fixture string in TC-A test (let _ = edited). T23 funnel.
3. SuggestionsCapped keyed on lowest claimant while edits target another
   rule - consistent with the diag's filing convention, one comment line
   would help. T23 funnel.
4. TC-B emits no SuggestionPartition at all (leans on standing
   OverlappingRules) - within the memo's marked wording latitude.

## Assessment
Spec compliance ✅. Task quality: Approved.
