# Audit: core-54-reuse-plan2-machinery (PROMOTION candidate)

- **Cluster:** `core-54-reuse-plan2-machinery` (pattern, core, settled)
- **Statement:** Every new Plan-3 resolution site reuses a Plan-2 mechanism (locator machinery, template engine, `LanguageIndex`, existing `DiagCode` variants) rather than introducing a new one; add no new DiagCodes; defaults match spec 4.9 so an omitted section never silently drops data.
- **Claimed count:** 3 (all 2026-07-09). Promotion asserts >=3 genuine, distinct occurrences.
- **Verdict: CONFIRMED** - 3 of 3 occurrences survive. Promotion stands.

The three refs point to three distinct work products from the Plan 3 session, each independently supporting the (topic, approach): the design memo that *decided* reuse, the plan-level constraint that *decided* "no new DiagCodes," and the independent whole-branch review that *reinforced* it against the real implementation. No fabrication, no misattribution, no duplicate.

---

## Occ 1 - `memo D10 (Rationale)` - kind: decided - SURVIVES

- **Artifact:** `docs/superpowers/specs/2026-07-09-plan-3-design-decisions.md` section **D10: Resolution semantics** (lines 101-142).
- **Supports statement?** Yes, directly. The D10 body wires each resolution site to a pre-existing Plan 2 mechanism: chapters reuse "the track-donor locator machinery and its `MissingExternal`/`AmbiguousExternal` diagnostics" (l.123-125); title templates "render in literal mode via the existing template engine" (l.130); settable `language` values are validated "reusing Plan 2's `LanguageIndex` and the `InvalidPropertyValue` code (D2) at a new emission site" (l.134-135). The **Rationale** names it verbatim: "reuse over new mechanism throughout - the locator machinery, the template engine, the language index, and the existing diagnostic codes all already exist from Plan 2; Plan 3 wires them to the new resolution sites" (l.138-140). Defaults matching spec 4.9 are covered in-body (`unmatched: keep|drop`, chapters `keep` = mkvmerge default, l.108-121).
- **Kind correct?** Yes. A design-decision memo section is a `decided` event.
- **Distinct?** Yes. This is the originating design decision; no other listed occurrence points to this file/section.

## Occ 2 - `Plan 3 Global Constraints` - kind: decided - SURVIVES

- **Artifact:** `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md` section **## Global Constraints** (l.11-20).
- **Supports statement?** Yes. Line 20 is the "no new DiagCodes" clause, decided at plan level: "Diagnostics are data: ... reuse existing `DiagCode` variants, **do not add new ones in this plan** (all needed codes exist)." This is a binding constraint every Plan 3 task inherits, not a mere restatement of D10.
- **Kind correct?** Yes. A plan-level global constraint is a `decided` event (a binding rule set for the whole plan).
- **Distinct?** Yes. Separate artifact (plan doc, not the design memo) and a distinct framing: D10 argues the general reuse rationale; the Global Constraints section pins the specific "add no new DiagCodes" rule as a per-commit-enforced invariant. Not a duplicate of Occ 1.

## Occ 3 - `whole-branch verdict (Strengths)` - kind: reinforced - SURVIVES

- **Artifact:** `docs/process-journal/artifacts/plan-3-sdd/verdicts/whole-branch-review-verdict.md`, **### Strengths**, bullet at l.30.
- **Supports statement?** Yes, near-verbatim: "**Reuse over new mechanism throughout** (locator machinery, template engine, `LanguageIndex`, existing `DiagCode`s), exactly as D10 intended. Defaults (`unmatched: keep`, `chapters: keep`, `tags: keep/keep`) match spec 4.9, so an omitted section never silently drops data."
- **Kind correct?** Yes. Listed as a review Strength that the *implemented* branch follows the decided pattern - a `reinforced` event, not a fresh decision.
- **Distinct?** Yes. This is an independent whole-branch review, a separate work product from both the design memo (Occ 1) and the plan constraint (Occ 2). The file header (l.1-14) documents its provenance: salvaged byte-faithfully from a Plan 3 review subagent's final message (session `2b4312c5-...`, `final_message_ts: 2026-07-09T13:27:30Z`); the reviewer read the full diff and drove real mkvmerge v100. It adds independent evidence (the reuse was verified correct against the running binary, e.g. `-J` attachment-id numbering), so it is genuine reinforcement rather than an echo.
- **Caveat (noted, not disqualifying):** the bullet says "exactly as D10 intended," so its framing is not blind to Occ 1. But the audit standard applied elsewhere in this backfill (core-34, core-31) treats the review verdict as a distinct product even when it cites the design memo, and here the verdict carries its own independent verification. Distinctness holds.

---

## Summary

| # | Ref | Kind | Artifact | Result |
|---|-----|------|----------|--------|
| 1 | memo D10 (Rationale) | decided | plan-3 design-decisions §D10 | SURVIVES |
| 2 | Plan 3 Global Constraints | decided | plan-3 plan doc §Global Constraints (l.20) | SURVIVES |
| 3 | whole-branch verdict (Strengths) | reinforced | plan-3 whole-branch-review-verdict §Strengths (l.30) | SURVIVES |

**verified_count = 3** distinct surviving occurrences (2 decided + 1 reinforced), all 2026-07-09.

**Verdict: CONFIRMED.** All three occurrences are real, correctly attributed, correctly typed, and mutually distinct. The count is not padded by a self-citation of one artifact. Promotion to standing house convention stands.
