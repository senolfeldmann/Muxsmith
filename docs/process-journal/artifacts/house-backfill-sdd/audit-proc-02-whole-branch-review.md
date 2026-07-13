# Audit: proc-02-whole-branch-review (PROMOTION candidate)

**Cluster:** `proc-02-whole-branch-review` (kind: pattern, domain: process, status: settled, count: 4, promoted).
**Statement under audit:** A separate final whole-branch review grades the plan's code against its own constraints after every per-task review passes; it repeatedly caught cross-task drift no task-scoped review could see. At close it runs on the strongest model plus a roll-up funnel over reviewer minors.

**Method:** each occurrence opened against its cited primary artifact (journal entry / verdict file / `git show <hash>`), then cross-checked against the era find file. Verdict is per-occurrence support for "a distinct whole-branch-review event arose here as {occ.kind}".

---

## Occurrence 1 — 2026-07-08, `decided` — journal 2026-07-08 'Moments' (E0[4]) — SURVIVES

**Claim:** final reviewer failed the plan on three counts; "the plan does not grade its own work" proved the load-bearing process rule.

**Verified:** `docs/process-journal.md` 2026-07-08 entry, "Moments" (lines 90-92) verbatim: *"Final reviewer graded the plan's own code against the plan's constraints and failed it on three counts; 'the plan does not grade its own work' proved to be the load-bearing process rule."* The three counts are itemized in the same entry, "What the process caught" (lines 49-53), each tagged `(final review)`: English-prose-leak out of core, spurious `LocatorConflict` on `match_to_source: false`, `UnknownProperty` name collision. Corroborated by find-E0.md E0-45 ("The plan does not grade its own work", process/pattern/**decided**). This is the origin/decision point of the rule. Support: strong, exact.

## Occurrence 2 — 2026-07-09, `reinforced` — journal fix-pass + plan-2-fixes-sdd/FINAL-review.md I1 + commit 59d24c8 (E2[23]) — SURVIVES

**Claim:** FINAL whole-branch review (opus) caught the literal-.mkv empty-stem output every per-task review missed.

**Verified:** `FINAL-review.md` is titled "Plan 2 fix pass - FINAL whole-branch review"; finding **I1** (line 29) documents "template rendering to `.mkv` produces a hidden empty-stem output instead of `EmptyRenderedName` (regression)", and line 4 states the review found "correctness the per-task reviews could not see (F5+F6 post-pass interaction)". Commit `59d24c8` ("fix(core): catch .mkv-literal empty-stem output (I1)") is co-authored by Claude Opus 4.8, confirming the opus attribution and the fix. Journal 2026-07-09 fix-pass entry (line 191) verbatim: *"FINAL whole-branch review (opus) caught what EVERY per-task review missed... This is the whole-branch stage earning its place. Fixed (59d24c8)."* Corroborated by find-E2 RC15. Distinct event from occurrence 1 (different plan, defect, date). Support: strong, exact.

## Occurrence 3 — 2026-07-10, `reinforced` — whole-branch-review-verdict.md + journal Plan 5 (E6[51]) — SURVIVES

**Claim:** caught the start_run override cross-task drift; the single strongest argument this session for the final cross-cutting review.

**Verified:** `plan-5-sdd/verdicts/whole-branch-review-verdict.md` (line 35) documents `start_run` resolving mkvmerge PATH-only and never reading the settings override, ending: *"T8 mirrored the CLI's run.rs verbatim and missed T7's substitution — the exact cross-task drift a task-scoped review cannot see."* (line 87 restates it as "one genuine end-to-end break no task-scoped review could see"). Journal 2026-07-10 Plan 5 entry (line 390) verbatim: *"...(WHOLE-BRANCH review only; T8 mirrored the CLI verbatim and could not see T7's substitution). Origin: cross-task drift; the single strongest argument this session for the final cross-cutting review."* Corroborated by find-E6 item 37 (line 189-190). Distinct event. Support: strong, exact.

## Occurrence 4 — 2026-07-12, `decided` — whole-branch-verdict.md + plan T23 (E7[69]) — SURVIVES

**Claim:** whole-branch review on the strongest model + roll-up funnel: 37 ledger items -> 3 fix-now / 16 defer / 14 discard / 4 resolved.

**Verified:** `plan-5.5/whole-branch-verdict.md` §"Minor triage table (roll-up funnel)" (lines 58-60) verbatim: *"n-in: **37** ledger items. n-out: **3 FIX-NOW / 16 DEFER / 14 DISCARD / 4 RESOLVED in-plan**."* Journal 2026-07-12 Plan 5.5 entry (lines 671-673) restates the identical breakdown. The whole-branch review fed fix wave commit `98e869a` ("fix: whole-branch review wave ... (T23)"), and it caught cross-task drift here too (C1: SchemaDrift severity recorded as warning against the owner's info ruling, "caught only by the whole-branch review", journal 647-650). Corroborated by find-E7 ("whole-branch funnel" referenced throughout). Distinct event. Support for roll-up-funnel: exact. See caveat below on "strongest model".

---

## Caveat (does not drop any occurrence)

The statement's "**At close it runs on the strongest model**" is imprecise. The Plan 5.5 whole-branch reviewer (occ. 4) ran on **Fable 5** (verdict line 14: "Reviewer: final whole-branch (Fable 5)"; journal line 663-664: "whole-branch review + final verification on fable"), and Plan 5 (occ. 3) also ran on fable. Only Plans 2/3/4 ran the whole-branch review on opus. So the accurate generalization is "runs on a top-tier model (opus, or the controller's frontier model fable depending on session)", not literally "the strongest model". This is a wording nuance in the promoted statement, not a fabricated recurrence — the roll-up-funnel refinement and the whole-branch-review event at occ. 4 are both real and exactly documented. Recommend softening the model clause when the convention is written.

## Verdict

**CONFIRMED.** All 4 occurrences survive: four distinct whole-branch-review events on four dates (07-08 decided origin; 07-09, 07-10 reinforced; 07-12 decided refinement), each substantiated verbatim by its cited primary artifact, none fabricated, misattributed, or duplicated. verified_count = 4 (>= 3). Promotion to house-knowledge stands. The recurrence is genuine; the only correction is the minor "strongest model" wording caveat above.
