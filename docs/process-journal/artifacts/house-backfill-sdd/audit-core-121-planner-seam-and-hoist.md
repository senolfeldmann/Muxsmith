# Audit: core-121-planner-seam-and-hoist (PROMOTION candidate)

- **Cluster:** `core-121-planner-seam-and-hoist` (non-decision, core, blocked)
- **Statement:** The four-copy planning pipeline (~100 lines across cli `dry_run.rs`/`run.rs` and src-tauri `lib.rs`/`run.rs`) and the never-decided injectable-planner-seam interface (S4/S5/S6) are one question: a shared core `plan_pipeline()` IS the seam. Left undecided whether to do the hoist as an idiomacy-fix wave now or fold it into the Plan 6 profile-editor design.
- **Claimed count:** 3 (all dated 2026-07-12). Promotion asserts >=3 genuine, distinct occurrences.
- **Verdict: REJECTED** - only **2 of 3** occurrences survive as distinct independent surfacings. The third is the promotion destination itself. Demote to Tier 1.

The topic is real and its two genuine surfacings are correctly attributed. But the count is padded to 3 by listing the **materialized rule (CONVENTIONS.md Non-decisions)** as one of its own supporting occurrences. That is circular self-citation: CONVENTIONS.md is, by the repo's own documented mechanism, the *destination* an item moves to *after* its recurrence count reaches 3 (`docs/decision-ledger.md`: "an item is promoted there when its recurrence count reaches 3"). A promotion target cannot also be one of the counts that authorize the promotion. This is exactly the padding the backfill audits are meant to reject (cf. audit-core-54: "The count is not padded by a self-citation of one artifact").

Underlying-source view: the three listed refs collapse onto only **two** distinct surfacing events -

- **Event A - 2026-07-11 docs-tree sweep:** the injectable-planner-seam interface (S4/S5/S6) surfaced as a never-decided Plan-6 named input (test-harness / untested-orchestration angle). -> Occ 2.
- **Event B - 2026-07-12 idiomacy review:** the four-copy planning-pipeline duplication surfaced, with the insight "a shared `plan_pipeline()` IS the seam" (duplication-cleanup angle). -> Occ 3, and also the ROADMAP idiomacy-triage block (L46-53), the ROADMAP STATUS "FOLDED INTO PLAN 6" entry (L230-234), and process-journal L762-768 - all the same event, none independent.
- **CONVENTIONS.md Non-decisions (Occ 1):** neither event; the materialized promotion, derived from and citing the ROADMAP anchor.

---

## Occ 1 - `CONVENTIONS.md Non-decisions (seam interface)` - kind: deferred - **DROPPED**

- **Artifact:** `docs/CONVENTIONS.md` **## Non-decisions**, "Injectable-planner-seam interface (S4/S5/S6)" bullet (l.53-59): "Blocked on internal progress: the profile-editor design in Plan 6, since a shared `plan_pipeline()` IS the seam and the four-copy planning pipeline hoist is folded into Plan 6. Reactivate when Plan 6 brainstorming starts. (ROADMAP Plan-6 anchor.)"
- **Text present / on-topic?** Yes. The topic is there verbatim, correctly typed as a non-decision (deferred). Not fabricated, not misattributed.
- **Why dropped - not an independent occurrence:** CONVENTIONS.md is the header-declared "materialized current-state view over our decisions," and `decision-ledger.md` defines it as the **promotion destination** reached at count 3. Counting the promotion target as one of the three occurrences that justify the promotion is circular bootstrapping - the rule is cited as evidence for itself. The entry is also substantively **derivative** of the ROADMAP: it explicitly sources itself "(ROADMAP Plan-6 anchor.)" and condenses the same material as the ROADMAP STATUS "FOLDED INTO PLAN 6" record. It is therefore a duplicate of the ROADMAP-recorded material, not a distinct occasion on which the consideration independently arose.
- **Corroborating structural fact:** `docs/decision-ledger.md` Tier-1 entries section reads "(none yet.)" - there is no accumulated Tier-1 trail of three considerations behind this promotion. The backfill reconstructs the third count by pointing at the promoted rule itself.

## Occ 2 - `ROADMAP L37` - kind: deferred - **SURVIVES**

- **Artifact:** `docs/ROADMAP.md` L34-44, "Further named inputs (2026-07-11, docs-tree sweep)"; the cited L37 is the clause "the never-decided injectable-planner-seam interface question ('to raise at merge time'; the merge gate passed without it) (S4/S5/S6)."
- **Supports statement?** Yes, the seam-interface half. It records the injectable-planner-seam (S4/S5/S6) as an open, never-decided Plan-6 named input, surfaced from the untested-`start_run`-orchestration angle.
- **Kind correct?** Yes. "never-decided ... the merge gate passed without it" is a deferred item.
- **Distinct?** Yes. Primary record of the 2026-07-11 docs-tree sweep (Event A) - a real surfacing event, a different date, source-block, and angle from the idiomacy review. Does not mention the four-copy pipeline or `plan_pipeline()`; it is the seam-interface strand the cluster later unifies with the hoist.

## Occ 3 - `idiomacy-review-findings (four-copy hoist)` - kind: deferred - **SURVIVES**

- **Artifact:** `.superpowers/sdd/idiomacy-review/find-X1.md`, finding **X1-1** ("the planning pipeline exists in FOUR copies"): the four sites (`muxsmith-cli/src/commands/dry_run.rs:38-120`, `run.rs:60-192`; `src-tauri/src/lib.rs:188-232`, `run.rs:249-350`), the `planner::plan_pipeline(...)` replacement, "~100" lines cut, and the explicit link "ROADMAP notes the injectable-planner-seam question for `start_run` as 'never-decided', so no recorded decision blocks this."
- **Supports statement?** Yes, the four-copy-hoist half, and it is where the unifying insight ("a shared `plan_pipeline()` IS the seam") originates. (verify-39.md, the other repo file touching S4/S5/S6, is a *different* finding - a `?`-vs-hand-rolled-match nit in `start_run` - and only name-drops S4/S5/S6 in its decision-guard grep; not this topic.)
- **Kind correct?** Acceptable. The review surfaced it as a recommendation whose disposition is deferral - "Şenol to decide idiomacy-wave vs fold into Plan 6" (process-journal L767) - so the consideration's outcome is deferred/open.
- **Distinct?** Yes vs Occ 2 (different date, standalone finding artifact vs ROADMAP tracker, duplication vs test-harness angle). Note that this finding, the ROADMAP idiomacy-triage block (L46-53), the ROADMAP STATUS entry (L230-234) and process-journal L762-768 are all the **same** 2026-07-12 event - they corroborate but add no further independent count.

---

## Summary

| # | Ref | Kind | Artifact | Result |
|---|-----|------|----------|--------|
| 1 | CONVENTIONS.md Non-decisions (seam interface) | deferred | `docs/CONVENTIONS.md` §Non-decisions l.53-59 | **DROPPED** (promotion destination / self-citation; derivative of ROADMAP) |
| 2 | ROADMAP L37 | deferred | `docs/ROADMAP.md` l.34-44 (docs-tree sweep, Event A) | SURVIVES |
| 3 | idiomacy-review-findings (four-copy hoist) | deferred | `.superpowers/sdd/idiomacy-review/find-X1.md` X1-1 (Event B) | SURVIVES |

**verified_count = 2** distinct surviving occurrences (Event A: seam-interface never-decided; Event B: four-copy hoist = the seam).

**Verdict: REJECTED.** The topic is genuine and worth tracking, but it has surfaced on only two independent occasions, not three. The claimed third occurrence is the CONVENTIONS.md Non-decisions entry - the materialized promotion destination, derived from the ROADMAP - which cannot count toward its own promotion without circularity. Below the threshold of 3. **Demote to Tier 1** (`docs/decision-ledger.md`) at count 2, `deferred(blocked-on: Plan 6 brainstorming)`; it re-promotes legitimately if a third *independent* surfacing occurs (e.g. Plan 6 brainstorming reopens it, or a new report resurfaces the seam question).

**Secondary note (not decisive for the count):** the cluster statement frames the item as "left undecided ... idiomacy-fix wave now or fold into Plan 6" (status `blocked`), yet CONVENTIONS.md l.57-58 and ROADMAP STATUS l.230-234 already record it as "FOLDED INTO PLAN 6." The materialized artifacts assert a settled disposition the cluster still treats as open; process-journal L767 ("Şenol to decide") sides with the cluster. This tension is worth resolving when the item is re-recorded at Tier 1, independent of the count verdict.
