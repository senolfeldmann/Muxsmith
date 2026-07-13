# Audit - proc-06-mkvtoolnix-parity (PROMOTION candidate)

**Cluster:** `proc-06-mkvtoolnix-parity` (kind: pattern, domain: process, status: settled)
**Claimed count:** 4 | **promoted:** true
**Statement:** mkvtoolnix is interactive-GUI, Muxsmith declarative-batch -> only muxing semantics/output are parity targets, not input-time convenience guesses; comparable behavior verified against the real mkvtoolnix binary/source (not memory), match-or-divergence recorded in the memo; licensing boundary GPL->MIT (behavior/facts/interfaces yes, literal expression no; modeled wordings recorded as explicit memo decisions).

**Audit question per occurrence:** does the cited artifact actually support "this (topic, approach) arose here as {occ.kind}"? Drop if fabricated, misattributed, or a duplicate of another listed occurrence.

---

## Occurrence 1 - 2026-07-09 / decided - SURVIVES

**Ref:** Plan 3.5 memo Origin/Grounding + journal (standing method, HANDOFF SI-3) (E4[32]/[33])

**Primary artifacts read:**
- `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md` lines 10-13: *"Origin: the Plan 1-4 mkvtoolnix parity audit (2026-07-09). That audit is now a standing method (HANDOFF SI-3 ...): compare against mkvtoolnix-gui / mkvmerge wherever meaningful, weighing the interactive-GUI vs declarative-batch distinction."*
- `docs/process-journal.md` line 238 (session 4, 2026-07-09): *"Framing rule, now SI-3: mkvtoolnix is INTERACTIVE ... Muxsmith DECLARATIVE BATCH ... -> only muxing semantics/output are parity targets; input-time convenience guesses ... are not, shelved to docs/IDEAS.md."*
- `HANDOFF.md` SI-3 (lines 32-45): full method incl. the interactive/declarative distinction and "Muxing semantics and output are parity targets; input-time convenience guesses are NOT."

**Verdict:** CONFIRMED. Genesis of the method (parity-target criterion + SI-3 formalization) is present verbatim in the dated memo and journal. Matches evidence and occ_kind `decided`.

---

## Occurrence 2 - 2026-07-10 / reinforced - SURVIVES

**Ref:** memo D30/D31 (E6[47]) - cites `MainWindow::beforeCloseCheckRunningJobs ... main_window.cpp:492-548`

**Primary artifact read:** `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md`
- D30 (line 220): *"Parity classification for the job-queue UX (SI-3)"* - explicit Match / Justified-divergence / Genuine-gap classification against mkvtoolnix-gui, each with source cites (mux_job.cpp, model.cpp).
- D31 (lines 242-257): *"Window close with an active run - full mkvtoolnix parity (amendment 2026-07-10)"*, "Parity evidence (SI-3): ... MainWindow::beforeCloseCheckRunningJobs ... (main_window.cpp:492-548)."

**Verdict:** CONFIRMED. A genuine, separate application of the method on a distinct day to a distinct subsystem (GUI), citing the real GPL C++ source at file:line exactly as claimed. Not a duplicate of occ 1 (different plan, different date, different artifact). occ_kind `reinforced` fits.

---

## Occurrence 3 - 2026-07-10 / decided - SURVIVES

**Ref:** journal session-6 close + HANDOFF SI-3 (E6[56]) - licensing boundary

**Primary artifact read:** `docs/process-journal/artifacts/handoffs/2026-07-10-session-6-close.md` SI-3 section (lines 53-65): *"Licensing boundary (Şenol 2026-07-10, mkvtoolnix is GPL / Muxsmith is MIT): BEHAVIOR and FACTS ... are fair game (semantics, exit codes, protocols, file locations, UX flow - ideas and interfaces ...); LITERAL code or text passages are NOT taken. Where a wording is deliberately [modeled, recorded as an explicit memo decision]."* Same boundary carried into current `HANDOFF.md` SI-3.

**Verdict:** CONFIRMED. The licensing-boundary component of the promoted statement originates here as an explicit Şenol decision dated 2026-07-10, matching the evidence text. **Distinctness check vs occ 2 (also 2026-07-10):** not a duplicate - occ 2 is the parity *application* to GUI decisions (D30/D31), occ 3 is the *licensing rule* about what may be extracted from GPL source; different facet, different document. The cluster's own collapse rule ("same-session/same-document") does not fire: same session, different documents.

---

## Occurrence 4 - 2026-07-12 / decided - SURVIVES

**Ref:** Plan 5.5 Global Constraints + d32-analysis.md SI-3 + task-6-verdict.md (E7[66])

**Primary artifacts read:**
- `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` line 22 (Global Constraints): *"mkvtoolnix parity method (SI-3) wherever a behavior is comparable: verify against the real binary / source at ~/Downloads/mkvtoolnix, record match/divergence in the memo."*
- `docs/process-journal/artifacts/plan-5.5-sdd/d32-analysis.md` lines 246-286: parity finding read from the GPL source with file:line cites (track.h:24, file_identifier.cpp:310, track.cpp:277, id_result.h:37), concluding *"match mkvtoolnix's permissiveness at the identify/data layer (already done), diverge deliberately at the rule layer where Muxsmith has a hazard mkvtoolnix does not."* The occurrence's quote is this line, near-verbatim.
- `docs/process-journal/artifacts/plan-5.5-sdd/task-6-verdict.md` "SI-3 (carried to memo)": *"mkvtoolnix-gui has no general zero-selected-tracks warning; deliberate divergence recorded in the plan-5.5 memo."*
- Cross-check: `docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md` D32 (lines 38-43) records the SI-3 parity finding in the memo; git log shows D32 sub-decisions resolved by Şenol 2026-07-12; journal 2026-07-12 (session 9) ties D32/T6 to the session.

**Date note:** the SDD analysis/verdict files carry 2026-07-11 headers; the occurrence dates it 2026-07-12 because the D32 decision was finalized by Şenol on 07-12 (git + journal confirm). One-day attribution slack, not a misattribution - the topic/approach unambiguously arose here as decided.

**Verdict:** CONFIRMED. The quote is real and drawn from the cited artifact; the "match at data layer / diverge at rule layer" refinement is a genuine distinct application (D32 unknown-property + T6 EmptyPlan). Not a duplicate of occ 1-3.

---

## Result

| # | date | kind | ref supports it? | duplicate? |
|---|------|------|------------------|------------|
| 1 | 2026-07-09 | decided | yes (memo + journal + HANDOFF SI-3) | no |
| 2 | 2026-07-10 | reinforced | yes (D30/D31, real C++ file:line cite) | no |
| 3 | 2026-07-10 | decided | yes (session-6 close licensing boundary) | no (distinct facet from #2) |
| 4 | 2026-07-12 | decided | yes (Plan 5.5 GC + d32-analysis + task-6-verdict, verbatim quote) | no |

**verified_count = 4** (all distinct occurrences survive).

**VERDICT: CONFIRMED.** Four independent, dated, source-backed occurrences. No fabricated recurrence, no misattribution, no collapsed duplicate. Each covers a real facet of the promoted statement: genesis parity-target criterion (1), applied parity classification with source cite (2), the GPL->MIT licensing boundary (3), the data-layer/rule-layer divergence refinement (4). Promotion to standing house-knowledge (Tier 2 / SI-3) stands.
