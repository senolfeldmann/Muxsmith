# Audit: core-34-suggestion-cap-non-silent (PROMOTION candidate)

**Cluster:** `core-34-suggestion-cap-non-silent` (kind: pattern, domain: core, status: settled)
**Claimed count:** 4 | **promoted:** true (`promoted_at: 3`)
**Verdict:** REDUCED | **verified_count:** 3 | **Promotion stands** (>=3 survive).

**Statement under audit:** Deterministic preference order among accepted candidates; emit at most 3 per conflict group; if more were accepted, log the cap (`SuggestionsCapped` info) - bounded output, no silent truncation. Enforcement: the inline code truncated silently; F7 added the diag code threaded through `report.rs` + `diagnostics.ftl` + spec 5.2.

---

## Occurrence-by-occurrence verification

### occ 1 - memo D6 step 5 (decided) - SURVIVES

- **Ref resolved:** `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md`, section `## D6: Suggestion engine design`, algorithm **step 5 "Rank and cap"** (lines 165-170).
- **Supports topic + kind:** Yes, verbatim. Step 5 reads: "Deterministic preference order among accepted candidates: typed flags ... > language > codec-derived > track_name substring; ... Emit at most 3 per conflict group, log the cap in the report if more were accepted (no silent truncation)." This is the design decision that established the pattern. Kind `decided` is correct.
- **Distinct:** Yes - the originating design memo, no other occurrence points here.

### occ 2 - independent review nit "cap not logged" (violated-corrected) - SURVIVES

- **Ref resolved:** `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`.
  - Line 35 (Nits): "... D6 cap-3 truncation not logged ...".
  - Line 71 (Reviewer 2 spec gaps): "cap-3 truncation (planner.rs:595) not logged despite D6 requiring it".
- **Supports topic + kind:** Yes. This is the independent discovery that the shipped inline code violated the D6 "no silent truncation" contract. Legitimate surfacing of the violation as a review finding. Kind `violated-corrected` is acceptable (the violation was flagged here; corrected downstream).
- **Distinct:** Yes - this is the Plan 2 *review* (found the bug, pre-fix), a separate work product from both the design memo and the later fix commit. Not the same file as occ 3's "F7 review".

### occ 3 - F7 review (c) (violated-corrected) - **DROPPED**

- **Ref resolved:** `docs/process-journal/artifacts/plan-2-fixes-sdd/F7-review.md`, section `## (c) SuggestionsCapped diagnostic`.
- **Why dropped - duplicate of occ 4 + kind misattributed:**
  - F7-review.md opens: **"Independent review of commit `68ec6aa`"** - it is the QA verification document *for the exact commit occ 4 points to*. The two refs describe one and the same corrective event (the F7 fix), not two independent surfacings of the topic.
  - Provenance chain confirms the 1:1 binding: the independent-review nit (occ 2) spawned Plan 2 fix task **F7** (`plan-2-fixes.md:43` "F7 ... (bugs C, D, **D6 nit**)"; line 47 "Log the cap-3 truncation (D6) ... do not truncate silently"). F7's implementation is commit `68ec6aa` (occ 4); F7-review.md (occ 3) reviews that commit. occ 3 is the review-step *of* occ 4's correction.
  - Kind mismatch: section (c) renders **"Verdict: correct."** - the F7 review found *no* violation and corrected nothing. It confirms a prior correction. Labeling it a distinct `violated-corrected` occurrence is a misattribution; it is a review-confirms-fix, already fully represented by occ 4.
- Dropped the review (derivative) rather than the commit (primary corrective event), since occ 4 matches the statement's "Enforcement" clause verbatim.

### occ 4 - commit 68ec6aa (violated-corrected) - SURVIVES

- **Ref resolved:** `git show 68ec6aa` - `fix(core): suggestion engine no-clobber, valid YAML fragments, cap logging`.
- **Supports topic + kind:** Yes, verbatim to the statement's enforcement clause. The diff adds `DiagCode::SuggestionsCapped` (info, `dropped` param) and threads it through exactly the three named surfaces:
  - `crates/muxsmith-core/src/report.rs` (+1 `diag_codes!` variant with rustdoc citing spec 5.3/D6),
  - `locales/en/diagnostics.ftl` (+1 Fluent message `suggestions-capped`),
  - `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` §5.2 catalog (+1 `SuggestionsCapped | info` row).
  Commit body: "The suggestion engine's cap-3 truncation was silent. Log it via a new DiagCode::SuggestionsCapped ...". This is the correction of the violation flagged in occ 2. Kind `violated-corrected` correct.
- **Distinct:** Yes - the actual fix commit (the correction), distinct event from the design memo (occ 1) and the review discovery (occ 2).

---

## Result

| occ | ref | kind | verdict |
|-----|-----|------|---------|
| 1 | memo D6 step 5 | decided | SURVIVES |
| 2 | independent review nit (cap not logged) | violated-corrected | SURVIVES |
| 3 | F7 review (c) | violated-corrected | **DROPPED** (duplicate of occ 4; verdict "correct", kind misattributed) |
| 4 | commit 68ec6aa | violated-corrected | SURVIVES |

**verified_count = 3** distinct surviving occurrences (design decision + independent review finding + fix commit form a genuine decided -> violated -> corrected chain across three separate work products).

**Verdict: REDUCED.** One occurrence dropped as a duplicate, but >=3 survive, so the promotion to house knowledge stands. The recurrence is real, not fabricated; the `count` field should be corrected from 4 to 3.
