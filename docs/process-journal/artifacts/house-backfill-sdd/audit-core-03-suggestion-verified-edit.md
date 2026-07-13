# Audit: core-03-suggestion-verified-edit (PROMOTION candidate)

**Cluster:** `core-03-suggestion-verified-edit` - batch-wide simulated suggestions
**Kind/status:** pattern / settled · claimed count 4 · promoted (at 3)
**Statement audited:** Suggestions are structured edits simulated against the cached identification of the whole batch before being shown; only refinements that resolve every instance and add no new diagnostic are emitted; an applied suggestion survives the next dry run (re-run through the real planning pass, no parallel matcher).

**Verdict: REDUCED** - 1 occurrence dropped as a duplicate, 3 distinct attestation points survive. Promotion stands (>=3). Count should be corrected 4 -> 3.

---

## Per-occurrence verification

### 1. 2026-07-08 decided - spec §2/§5.3 (Şenol's amendment) - SURVIVES

- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`.
- §2 decision-log row "Disambiguation help | Batch-wide validated suggestions": "Suggestions are simulated against the whole batch before being shown; an applied suggestion survives the next dry run." -> supports sub-claims (a) batch-simulated-before-shown and (c) survives-dry-run.
- §5.3 Suggestion engine contract: "generate candidate refinements ... then simulate each candidate against the cached identification data of the entire batch. Emit only refinements that (a) resolve every instance of the conflict batch-wide and (b) introduce no new diagnostic for any file." + "Suggestions are structured edits (`config_path` + proposed change), not prose." -> supports all three sub-claims fully.
- Attribution "Şenol's amendment": corroborated by the journal (occ 2) which records it as "his amendment"; spec authored by Şenol Feldmann. Genuine `decided` at design time.
- Caveat (not disqualifying): the §5.3 "Algorithm ... via the real planner ... D6" pointer line was added 2026-07-09 (commit d4390d7, Plan-2 fold-in), but the load-bearing contract (§2 row + §5.3 contract paragraph) was present from creation on 2026-07-08. Date and attribution hold for the contract itself.

**Verdict: CONFIRMED.** Canonical attestation point for the Plan-1 design decision.

### 2. 2026-07-08 decided - journal Plan 1 - SURVIVES

- `docs/process-journal.md`, entry "2026-07-08 | Spec + Plan 1 complete", "Decisions and why" bullet (lines 22-24): "Suggestion engine contract (his amendment): suggestions are simulated against the whole batch before being shown; an applied suggestion must survive the next dry run. Turned a hint feature into a verified-edit feature."
- Real, distinct document (the process journal, a separate discipline from the spec). Directly attests topic + approach + `decided` kind, and confirms the "his amendment" / verified-edit framing.
- Under the cluster's own counting rule (cluster-core.md line 5), "a dated decision-journal entry" is an admitted distinct attestation point, separate from "a design spec." Not fabricated, not misattributed.

**Verdict: CONFIRMED.** Distinct attestation point.

### 3. 2026-07-08 decided - commit 61249f9 - DROPPED (duplicate of occ 1)

- `git show 61249f9`: "docs: add Muxsmith v1 design spec", author Şenol Feldmann, **1 file changed, 376 insertions, no code**. It is the commit that *created* the spec file cited in occurrence 1; its diff literally contains the §2 row and §5.3 text verified there.
- This is not an "implementation commit-set" (the counting rule's basis for a distinct commit occurrence). The suggestion engine was actually implemented later in Plan 2 (`e9fcaaa` "feat(core): batch-validated suggestion engine for AmbiguousRule (D6)"), which the cluster does **not** cite. Citing the spec-authoring docs-commit is the same attestation point as occ 1, counted twice.
- The methodology's own collapse rule (cluster-core.md line 5) folds "a single review event cited via both its verdict file and a journal/progress mention" into one; by direct analogy a single spec cited via both the spec file and the commit that authored it collapses. Sibling cluster core-01 repeats the same inflation (spec §2 row + journal + commit 61249f9 for one 2026-07-08 decision).

**Verdict: DROP.** Duplicate of occurrence 1 (same artifact-creation act), not an independent implementation.

### 4. 2026-07-09 decided - memo D6 steps 3-4 - SURVIVES

- `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md`, D6 "Algorithm":
  - Step 3 (Simulate): "apply the edit to a cloned profile and re-run the REAL planning pass (same code path as dry-run, **no parallel implementation**) against the cached identification of the entire batch." -> supports "simulated against the cached identification of the whole batch" and "re-run through the real planning pass, no parallel matcher".
  - Step 4 (Accept): "Keep a candidate iff (a) every instance of the group's conflict is gone, and (b) the diagnostic set introduces nothing new anywhere ... This IS the 'an applied suggestion survives the next dry run' invariant." -> supports "resolve every instance and add no new diagnostic" and the survives-dry-run sub-claim.
- The statement's exact phrasing "no parallel matcher" is synthesized from D6 step 3's "no parallel implementation"; the statement was substantially built from these steps.
- D6 line 105 labels the *contract* a "recap (spec 5.3, unchanged)", so the contract portion overlaps occ 1 - but D6 makes a genuine *additional* design decision (the algorithm: closed edit grammar, discriminator generation, the acceptance-via-real-planner mechanization). A distinct design memo (`Dnn`), a day later, in a different plan. Distinct attestation point, not a duplicate.

**Verdict: CONFIRMED.** Distinct attestation point.

---

## Result

| # | Ref | Outcome |
|---|-----|---------|
| 1 | spec §2/§5.3 (Şenol's amendment) | survives |
| 2 | journal Plan 1 | survives |
| 3 | commit 61249f9 | **dropped - duplicate of occ 1** |
| 4 | memo D6 steps 3-4 | survives |

**verified_count = 3** distinct attestation points (spec §2/§5.3, journal Plan 1, memo D6 steps 3-4).

**Verdict: REDUCED.** One occurrence (commit 61249f9) dropped as a duplicate of the spec-authoring artifact; 3 genuine, independently-verified attestation points remain. Promotion threshold (>=3) still met - the standing convention is real, but its count is 3, not 4. The dropped commit is a systematic artifact of the backfill counting a docs-only spec-authoring commit as if it were an independent implementation touchpoint (same pattern in sibling core-01).
