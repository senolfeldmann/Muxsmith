# Audit: core-35-overlap-auto-suggestions (PROMOTION candidate)

**Cluster:** `core-35-overlap-auto-suggestions` (kind `pattern`, domain `core`)
**Claim:** count 3, promoted at count 3, status `settled`.
**Statement under audit:** OverlappingRules auto-narrowing suggestions - generate
narrowing candidates for ALL overlap claimants symmetrically (including claimant
0); `resolves_without_regression` selects the feasible ones; feasibility, not a
precedence guess, decides which rule narrows.

**Method:** each cited occurrence opened in its source artifact and checked against
"this (topic, approach) arose here as {occ.kind}". Drop if fabricated, misattributed,
or a duplicate of another listed occurrence.

**Verdict: CONFIRMED** - 3 of 3 occurrences survive; the recurrence count is real;
promotion to standing house convention stands.

---

## Occurrence 1 - deferred, 2026-07-09, "journal Plan 2 (Open threads)" -> SURVIVES

**Artifact:** `docs/process-journal.md`, entry "2026-07-09 | Plan 2 written and
implemented | session 2", **Open threads** paragraph (lines 173-178).

**Verbatim support (line 174):**
> Deferred: OverlappingRules auto-suggestions and the no-single-fix partition
> report (D6 remainder)

Exact-match support. The overlap auto-suggestion feature (the D6 remainder from the
Plan 2 design decisions) was explicitly deferred out of Plan 2 on 2026-07-09. Topic
and kind (`deferred`) both correct. Not fabricated, not misattributed.

## Occurrence 2 - decided, 2026-07-11, "memo D33 (Şenol)" -> SURVIVES

**Artifact:** `docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md`,
section "D33: OverlappingRules auto-suggestions - symmetric, acceptance-filtered
(policy 3)" (lines 85-150). File header (line 3): "Decisions D32-D33 ... decided by
Şenol 2026-07-11".

**Verbatim support (lines 87-90):**
> **Decision (Şenol 2026-07-11):** generate narrowing candidates for ALL overlap
> claimants symmetrically; the existing acceptance criterion
> (`resolves_without_regression`, gated on the target overlap INSTANCE
> disappearing) selects the feasible ones

**Rationale (lines 97-100):** "feasibility ... not a precedence guess, determines
which rule can be narrowed". The cluster statement is a near-verbatim distillation of
this memo. This is the owner decision itself (kind `decided`), correctly attributed
to Şenol and dated 2026-07-11. The pre-decision steelman lives separately in
`plan-5.5/d33-analysis.md` ("Şenol decides; the memo is written after his decision"),
so the memo is genuinely the decision artifact, not the analysis. Distinct from
occurrence 1 (different date, different artifact, decision vs deferral).

## Occurrence 3 - reinforced, 2026-07-11, "task-18 verdict" -> SURVIVES

**Artifact:** `.superpowers/sdd/plan-5.5/task-18-verdict.md` (reviewer verdict on the
T18 implementation of D33).

**Verbatim support (lines 26-28), Spec Compliance / named risks:**
> Named risks: ... (b) genuinely symmetric incl. claimant 0; (c) TC-B via filter,
> not early-out ...

Plus the header adjudication (lines 6-13) confirming no precedence guess was baked in
(within-rank tiebreak is illustrative, not a spec miss) and the final assessment
"Spec compliance. Task quality: Approved." The verdict independently re-affirms the
exact load-bearing property of the statement - symmetry across all claimants
*including claimant 0*, feasibility (acceptance filter) not precedence deciding the
rule. Kind `reinforced` correct. Distinct event from occurrence 2: this is the
implementation review, not the decision.

---

## Distinctness check

Three distinct artifacts (journal entry / design-decisions memo / task verdict), three
distinct events on the D6->D33 arc: deferral (Plan 2) -> decision (D33 memo) ->
implementation review (T18 verdict). No two collapse into one; none is a duplicate.

## Metadata nit (non-blocking, does not affect the verdict)

Occurrence 3's cluster date is `2026-07-11`, but `task-18-verdict.md` dates itself
`2026-07-12` in its own header ("model: opus, 2026-07-12"; file mtime Jul 12 01:55).
The T18 review ran later in the same overnight session 9 (2026-07-11 evening ->
2026-07-12 morning) than the D33 decision. The one-day-off date does not weaken the
occurrence - if anything it sharpens the separation from occurrence 2 - but the
cluster's `date` field for this occurrence should read `2026-07-12`.

## Bottom line

verified_count = 3. All cited refs support their claimed (topic, kind). No fabricated,
misattributed, or duplicate recurrence. **CONFIRMED** - promotion stands.
