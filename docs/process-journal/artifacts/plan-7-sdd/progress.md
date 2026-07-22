# Plan 7 progress (the plan's tracker; the plan document stays untouched)

Plan: docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md (0fea107).
Method: SDD, fresh implementer + independent reviewer per task,
whole-branch review at close. Nine-part gate after every merge.

## Wave 1 (five parallel worktrees from master 0fea107)

| Task | Stream/worktree | Implementer | Review | Merged |
|---|---|---|---|---|
| T1 D64 pin funnel | A .worktrees/plan7-a | DONE a8c5951 | APPROVED (1 info) | d3b8c7e gate green |
| T2 D63 multilang renderer | A (after T1) | DONE_WITH_CONCERNS d6d20ad | APPROVED (Q1 upheld) | d3b8c7e gate green |
| T3 tooltip content 42x2 | B | DONE 923b049 | APPROVED | 41e378d |
| T4 D53 helpId + $ta bindings | B | DONE d194588 | APPROVED | 41e378d |
| T5 D55 migration | B | DONE 3fab82f | APPROVED | 41e378d |
| T6 D60 resolvedTrackLabel | B | DONE 2422a58 | APPROVED | 41e378d |
| T7 D56 live switch | B | DONE 72c07ee | APPROVED | 41e378d |
| T8 view+card topics | C | DONE d76145f+77f0525 | APPROVED | 634db53 |
| T9 editor topics 1-9 | D | DONE 34a5aa7 | APPROVED | e66dbae |
| T10 editor topics 10-18 | E | DONE 20c2586+8d0e860 | APPROVED | c8cf309 |

## Wave 2 (plan7-f, serial, from c8cf309)

| Task | Implementer | Review | Merged |
|---|---|---|---|
| T11 marked+loader | DONE c445aa7 | APPROVED (1 minor->whole-branch) | - |
| T12 D52 sidebar/help-mode | DONE 906260b | APPROVED (Q1-Q3 ruled clean) | - |
| T13 D54 annotations | DONE 97f707f | APPROVED | - |
| T12-fix hoverId clear (D52 r6) | DONE f8e7d5d | APPROVED (1 comment minor -> whole-branch) | - |
| T14 D57 markers | DONE_WITH_CONCERNS 18a9801 | NEEDS FIXES (F1: detail-panel double marker; Q1-Q3 upheld) | - |
| T14-fix F1 suppression + reaching fixture | DONE ff49658 (S21) | APPROVED (delta verdict appended to task-14-verdict.md; harvest mined) | - |
| T15 D58 dropdowns | DONE 9f4aa8a | APPROVED (Q1-Q4 clean; 1 minor -> whole-branch; harvest mined) | - |
| T16 D59 ordinal column | DONE 8f6400f | APPROVED (Q1/Q2 clean; 46-id count verified; harvest mined) | - |

Wave 2 COMPLETE at 8f6400f; merge into master = next step.
## Wave 3 (plan7-g serial T17-T19; plan7-h T20) - RUNNING since S21

Wave 2 merged into master as 7c29957 (no conflicts), nine-part gate green
on the merged state, pushed. Worktrees plan7-g and plan7-h branched from
7c29957.

| Task | Stream/worktree | Implementer | Review | Merged |
|---|---|---|---|---|
| T17 check-i18n extensions | G plan7-g | NEEDS_CONTEXT (premise refuted) -> design amendment round 7 four-eyes APPROVED (1fa2972 + baa8ee2) -> completion DONE_WITH_CONCERNS a838a32 | APPROVED (Q1/Q2 clean; M1 header count -> riding T18; harvest mined) | - |
| T18 D61 presence gate | G plan7-g | DONE 45f0e31 (19 codes gated; M1 header recount folded in) | APPROVED first pass (Q1 taxonomy verified by recount; 1 no-action nit in verdict; harvest = corroboration only, no ledger writes) | - |
| T19 D62 help-topic gate | G plan7-g | DONE_WITH_CONCERNS a653d39 (6 checks vs design's "exhaustive" 4 - the 2 routed constraints; brief regex premise refuted, grammar-constrained to 22 ids; zero-pipe/code-span asymmetry deliberate) | APPROVED (design amendment round 8 + M1 fix four-eyes approved first; Q1-Q3 clean; harvest mined; 2 watch items -> whole-branch) | - |

Stream G COMPLETE at a653d39 (T17 a838a32, T18 45f0e31, T19 a653d39);
merge into master = next step. Then wave 4 (T21 on master), whole-branch.
| T20 D61 ParamValue promotion | H plan7-h | DONE_WITH_CONCERNS 900db87 (lib.rs compile-forced beyond Files list; ~12 test assertions re-typed; no ergonomic PartialEq impls) | APPROVED (Q1-Q3 clean; harvest mined - drop note: over-restriction calibration item is corroboration in the verdict artifact, no strict entry fit) | merging |
## Wave 4 (master): T21 spec amendments - COMPLETE

| Task | Implementer | Review | Notes |
|---|---|---|---|
| T21 v1-spec amendments | DONE_WITH_CONCERNS 4ac8d8b (row-29 echo found+aligned beyond brief) | NEEDS FIXES (F1 brief truncated design 6(b); Low formatting) -> fix cc0e6d7 -> delta APPROVED | execution graded exemplary; defects were plan-transcription-level; harvest mined (2 new entries) |

ALL 21 TASKS COMPLETE at cc0e6d7. Whole-branch review (top model):
NOT READY (I1 + 6 minors) -> single fix wave 13e138c -> delta verdict
READY-subject-to (1) owner ruling on I1's keyboard residual, (2) owner
surface pass. Harvest mined (3 new entries + citation-drift promoted).
Blocked-pool sweep done: i18n-12 + gui-26 stale-resolved (work landed
via T17 / D56), 24 entries correctly blocked, zero fired conditions.
Remaining at close: owner rulings, design one-liner batch (M4/M6/D54
parenthetical + keyboard wording if Option 2), roll-up funnel, salvage
incl. scratch-citation re-pointing, journal, HANDOFF.

## Notes

- Content rules for T8-T10 (markdown subset, h1 opener, 1-3 kB):
  controller-ratified provisional; owner veto window open.
- Merge order: sequential, gate re-run per merge (plan's merge section).
