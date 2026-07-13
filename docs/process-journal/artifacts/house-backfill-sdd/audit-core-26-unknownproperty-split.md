# Adversarial promotion audit: core-26-unknownproperty-split

- **Cluster id:** `core-26-unknownproperty-split`
- **Kind / domain / status:** pattern / core / settled
- **Claimed count:** 3 (all `violated-corrected`, all 2026-07-08); `promoted: true`, `promoted_at: 3`
- **Audit verdict:** **REJECTED** — demote to Tier 1
- **verified_count (distinct surviving occurrences):** **1**

## Statement under audit

> The code used unknown-property for a config-time typo error while spec 5.2 defined UnknownProperty as the plan-time skew warning; resolved by amending the spec (split into UnknownProperty error + UnknownPropertySkew warning) and keeping the code — a deliberate spec-wins exception because the code was correct.

The topic and approach are real and correctly described. That is not in dispute. The audit turns on whether the three listed occurrences are **three distinct arisings of the pattern** (which would earn promotion to a standing convention) or **three artifact-views of one single episode** (which does not).

## Per-occurrence verification

### Occurrence 1 — "whole-branch verdict Important #2" (violated-corrected)

**Artifact:** `docs/process-journal/artifacts/plan-1-sdd/verdicts/whole-branch-review-verdict.md:30` (also mirrored at `.superpowers/sdd/...` — same file).

Verbatim (Important #2):

> **`DiagCode::UnknownProperty` repurposes a spec-defined code name.** Spec 5.2's table defines `UnknownProperty` as the planning-time version-skew **warning** (section 9.2); the implementation uses `unknown-property` for the config-time typo **error** (`report.rs:45`) and invents `unknown-property-skew` (`report.rs:69`) for the spec's case. [...] Both conditions deserve distinct codes — the code is right, the spec table is now wrong. Decide before catalog freeze: amend the spec 5.2 table (recommended: rename its row to `UnknownPropertySkew`, add a config-time `UnknownProperty` error row) or rename the codes.

Round-2 verdict (`whole-branch-review-verdict-round-2.md:20`): "**#2**: spec 5.2 table and 9.2 amended exactly as recommended; code untouched."

**Verdict: SUPPORTS the topic and approach.** This is the review finding that raised the collision and decided the resolution (amend spec, keep code because code is correct). Genuine, richest of the three. **This is the one surviving distinct occurrence.**

### Occurrence 2 — "journal" (violated-corrected)

**Artifact:** `docs/process-journal.md:52-53`, inside the plan-1 session entry, under the heading "What the process caught":

> - UnknownProperty name collision between spec table and code **(final review)**
>   -> spec amended, code kept.

**Verdict: real artifact, but a DUPLICATE of occurrence 1.** It is not an independent arising of the pattern; it is the journal's one-line narration of the exact same finding. The line self-identifies its source as "(final review)" — i.e. occurrence 1 — and its resolution as "spec amended, code kept" — i.e. occurrence 3. The journal entry literally chains occ 1 → occ 3. **DROP as duplicate.**

### Occurrence 3 — "fix commits cd3f239/f7afa8d" (violated-corrected)

**Artifacts:** `git show cd3f239`, `git show f7afa8d` — both touch only `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`.

- `cd3f239` (Jul 8 10:36): splits the 5.2 table row into `UnknownProperty` (error, config-time) + `UnknownPropertySkew` (warning); updates §9.2 to use `UnknownPropertySkew`.
- `f7afa8d` (Jul 8 11:09): follow-up refinement — `UnknownProperty` covers match conditions only; unknown `changes` keys become `UnknownSettableProperty`.

**Verdict: real artifacts, but a DUPLICATE of occurrence 1.** These commits *are* the correction the round-2 verdict signed off ("All three Important findings verified resolved against cd3f239"). A `violated-corrected` episode inherently consists of the violation being raised (occ 1, the review) and corrected (occ 3, the commits). Splitting the raise and the fix of one finding into two "occurrences" double-counts a single episode. **DROP as duplicate.**

## Core finding: one episode, three records

All three refs are genuine; none is fabricated or misattributed. But they document a **single** violated-corrected episode:

| Ref | Role in the one episode | Date |
|-----|-------------------------|------|
| occ 1 — whole-branch verdict Important #2 | the review **raised** the collision and decided "amend spec, code is right" | 2026-07-08 |
| occ 3 — commits cd3f239/f7afa8d | **implemented** that exact amendment (spec-only edits) | 2026-07-08 |
| occ 2 — journal line 52-53 | **narrated** that same finding-and-fix, explicitly citing "(final review)" and "spec amended, code kept" | 2026-07-08 (plan-1 entry) |

There is exactly one situation: the `UnknownProperty` diagnostic-code / spec-5.2-table name collision, found in the plan-1 whole-branch review on 2026-07-08, fixed by two spec commits the same morning, journaled in the plan-1 close entry. The pattern never recurred in any later plan (grep of all later verdicts/reports shows only settled downstream *usage* of the now-split codes, no second conflict-resolution episode).

A promotion count of 3 is manufactured by counting review + commit + journal as three occurrences. Under the audit rubric these are "duplicate of another listed occurrence." Every incident in this repo has a review, a commit, and a journal line; if that triad cleared the 3-occurrence bar, the threshold would be meaningless — any one-off would auto-promote.

## Conclusion

- Distinct surviving occurrences: **1** (occurrence 1; occurrences 2 and 3 dropped as duplicate records of the same episode).
- 1 < 3 → **REJECTED**. The pattern is a genuine, correctly-described **one-time decision**, not a recurring convention. Demote from promoted house-knowledge (Tier 2 standing convention) back to **Tier 1** (a recorded single instance / decision-ledger entry). It belongs in the decision ledger as "spec-wins exception: UnknownProperty split, code kept," not as a standing rule with a recurrence count.
