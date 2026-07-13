# Audit: core-44-suggestion-no-clobber (PROMOTION candidate)

**Cluster:** `core-44-suggestion-no-clobber` - "never widen an existing match"
**Claimed:** kind `pattern`, status `settled`, count **3**, `promoted: true`
**Promotion bar:** count >= 3 -> becomes a standing house convention, so the count must be real.
**Audit date:** 2026-07-13

**Statement under audit:** `with_rule_match` must not overwrite an existing exact/substring key (`or_insert` semantics); a clobbering candidate becomes a no-op then rejected by the acceptance sim; not-list append stays additive. Bug C: the inline engine merged via `BTreeMap::extend`, widening an existing `track_name` substring (violating D6 "never relax").

---

## Occurrence-by-occurrence verification

### Occ 1 - `2026-07-09 violated-corrected (independent review bug C)` -> SURVIVES

- **Ref:** `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`
- **Found (line 20, controller triage):** "C. Suggestion engine `with_rule_match` uses `BTreeMap::extend` -> an `AddSubstring` candidate overwrites an existing `track_name` substring (widens, violates D6 'never relax')."
- **Found (line 69, Reviewer 3 verbatim):** "CONFIRMED planner.rs:754 - `with_rule_match` merges `AddSubstring` via `BTreeMap::extend` -> overwrites an existing track_name substring, widening not narrowing (violates D6 'never relax'); acceptance check does not catch it when the widened match happens not to collide. [C]"
- **Verdict:** The artifact states the exact (topic, approach) verbatim: `with_rule_match`, `BTreeMap::extend`, widened `track_name` substring, D6 "never relax", the acceptance-check miss. This is the genuine **find** event. Not fabricated, not misattributed. **Kept.**

### Occ 2 - `2026-07-09 violated-corrected (F7 review (a))` -> SURVIVES

- **Ref:** `docs/process-journal/artifacts/plan-2-fixes-sdd/F7-review.md`, section **(a)** "`with_rule_match` no-clobber" (lines 5-21).
- **Content:** Independent review *of commit `68ec6aa`* against the F7 task text and D6. Section (a) confirms the fix merges `exact`/`substring` via `map.entry(k).or_insert_with(...)` instead of `BTreeMap::extend`, `not` stays `extend`; traces the motivating clobber scenario (`AddSubstring{"Director"}` collides with existing `substring:{track_name:"Foo"}` -> no-op -> rejected by `resolves_without_regression`); independently reverts `planner.rs` and reproduces the RED test. Verdict "correct."
- **Distinctness check:** This is a *different document*, a *different process stage*, and a *different reviewer pass* from Occ 1. Occ 1 is the Plan-2 retrofit review that discovered bugs A-K; Occ 2 is the post-fix review of the F7 implementation. Not the same event -> not a duplicate of Occ 1.
- **Distinctness vs Occ 3:** The F7 review is the per-fix **review verdict**; commit `68ec6aa` is the **implementation**. The cluster's own counting rule (cluster-core.md line 5) explicitly lists "a per-task review verdict" and "an implementation commit-set" as separate valid attestation types and keeps find-vs-fix distinct. A fix-review verdict is not collapsed into the commit it reviews. Not a duplicate of Occ 3.
- **Verdict:** Correctly attributed to the no-clobber (topic, approach); F7 item (a) = bug C per the fix plan (see cross-check below). **Kept.**

### Occ 3 - `2026-07-09 violated-corrected (commit 68ec6aa)` -> SURVIVES

- **Ref:** `git show 68ec6aa` (`fix(core): suggestion engine no-clobber, valid YAML fragments, cap logging`).
- **Message:** "F7 (D6): `with_rule_match` used `BTreeMap::extend` for exact/substring deltas, which overwrites an existing key ... Switch to insert-only-if-absent (`entry().or_insert()`) so a colliding key becomes a no-op the acceptance simulation correctly rejects."
- **Actual diff (`crates/muxsmith-core/src/planner.rs`), not just the message:**
  - `exact`: `.extend(add.clone())` -> `map.entry(k.clone()).or_insert_with(|| v.clone())`
  - `substring`: same `.extend` -> `or_insert_with` change
  - `not`: stays `expr.not...extend(add.clone())` (additive, correct)
  - In-code comment (added line): "`exact`/`substring` use insert-only-if-absent semantics (bug C)".
- **Verdict:** The **fix** event. Code change matches the statement exactly. Not fabricated, not misattributed. **Kept.**

---

## Cross-checks

- **F7 = bug C mapping is explicit.** `docs/superpowers/plans/2026-07-09-plan-2-fixes.md` line 43: "F7: suggestion engine - no-clobber, valid YAML, cap logging (bugs C, D, D6 nit)"; line 45 states the no-clobber requirement (`or_insert` semantics, clobber becomes no-op, `not` append fine). `F7-report.md` section (a) header: "`with_rule_match`: insert-only-if-absent for `exact`/`substring` (bug C)". So `F7 review (a)` correctly resolves to bug C, and `F7 review (b)` (cited by sibling core-45) to bug D.
- **Sibling consistency.** core-45 (yaml, bug D) and core-34 (cap) cite the same triple shape (independent review find / F7 review / commit 68ec6aa). The find/review/fix decomposition is applied uniformly across the suggestion-engine clusters, not inflated specifically for core-44.
- **No mis-cite of F7-report vs F7-review.** The occurrence cites "F7 review", and `F7-review.md` (the independent review of the fix) exists and matches; it is not the implementer's `F7-report.md`.

---

## Verdict: CONFIRMED (3 of 3 survive; promotion stands)

**verified_count = 3.** Every cited ref exists, names the exact (topic, approach) - `with_rule_match` / `BTreeMap::extend` clobber / `or_insert` no-op / D6 "never relax" - and none is fabricated, misattributed, or a duplicate of another listed occurrence. The three are the distinct find (independent review bug C) -> fix (commit 68ec6aa) -> review-of-fix (F7 review a) attestation points the cluster methodology (cluster-core.md line 5) explicitly counts as separate.

### Honest caveat (not grounds to drop, but flag for the promotion bar's intent)

These three are **one incident attested at three SDD process stages**, all on 2026-07-09, all about the same code site (`planner.rs::with_rule_match`), all under work item F7. They are **not** three temporally-separate recurrences of the anti-pattern across the project's life. The count of 3 is valid strictly under the cluster's stated "one occurrence = one distinct attestation point, find/fix/review distinct" rule. If Şenol's promotion bar is meant to require genuine recurrence across >= 3 *independent incidents/contexts* (rather than >= 3 process-stage attestations of a single incident), this candidate would not clear that stricter bar - but that is a question of how to read the promotion threshold, not a defect in the occurrence records. Under the drop criteria I was given (fabricated / misattributed / duplicate), nothing fires.
