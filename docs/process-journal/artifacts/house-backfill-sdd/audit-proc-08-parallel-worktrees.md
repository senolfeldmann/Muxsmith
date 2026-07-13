# Audit - proc-08-parallel-worktrees (PROMOTION candidate)

**Cluster:** proc-08-parallel-worktrees | kind pattern | status settled | claimed count 4 | promoted at 3
**Statement audited:** Genuinely independent task streams run as parallel git worktrees with a full gate per merge, not strictly serial. Origin was a failure (strictly-serial D19/D20/D21 in Plan 3.5); SI-1 rewritten to parallelize-independent. Only truly independent streams parallelize; a real dependency chain stays serial.

**Verdict: CONFIRMED** - all 4 occurrences survive verification against primary sources. verified_count = 4 (>= 3, promotion stands).

Each occurrence was checked against the authoritative artifact (the actual journal entry / progress ledger / plan / git object), not the E4/E5/E7 reconstruction files. Bracket indices in the refs (E4[36] etc.) are the global occurrence index; the load-bearing locator is the ref text itself.

---

## Occurrence 1 - 2026-07-09, violated-corrected, "journal Plan 3.5 (Friction/failure)" (E4[36])

**CONFIRMED.** Primary source: `docs/process-journal.md` lines 253-254 (session 4, Plan 3.5 complete), section **Friction/failure**:

> STRICTLY SERIAL execution. After Task 1 the D19/D20/D21 streams were independent (disjoint planner.rs regions), parallelizable in worktrees. Şenol: "I am waiting for something that could have been faster." SI-1 rewritten (Superpowers-throughout + parallelize-independent) ... The clearest miss of the session.

Supports the topic (parallel worktrees for independent streams) and the occ_kind (violated-corrected: serial was the miss -> SI-1 rewritten). Verbatim quote and "clearest miss of the session" both present. Distinct decision-event: the Plan-3.5 failure origin.

## Occurrence 2 - 2026-07-09, decided, "journal session-4-close (plan c0c0ef7)" (E5[39])

**CONFIRMED.** Primary source: `docs/process-journal.md` lines 267-271 (session 4 close), **Decisions and why**:

> Plan 4 plan is WAVED: wave 1 = five independent streams ... to run as parallel worktrees - direct consequence of the Plan 3.5 serial-execution criticism, first parallel run for this repo.

Git corroboration: `c0c0ef7` = 2026-07-09, "docs: Plan 4 implementation plan (executor + run + queue, 11 tasks, waved)". Supports topic + occ_kind (decided: the Plan-4 wave-1 authoring call). Distinct from occ 1: different date-event (Plan-4 authoring vs Plan-3.5 failure), different ref.

## Occurrence 3 - 2026-07-10, reinforced, "journal Plan-4-complete" (E5[40]/[41])

**CONFIRMED.** Primary source: `docs/process-journal.md` lines 284-288 (Decisions) + 336-337 (Friction and failure):

> Wave 1 (T1/T4/T5/T6/T7) ran as five parallel worktree streams (SI-1, first parallel run in this repo); merges T5->T7->T4->T6->T1 with the full gate per merge. Zero real conflicts ...
> Şenol mid-session challenged whether parallelism was underused; the answer held (genuine dependency chain) ...

Git corroboration: the five merge commits `merge: plan4-t{5,7,4,6,1}` exist and are ordered chronologically t5 -> t7 -> t4 -> t6 -> t1, matching the cited sequence exactly; `chore: ignore .worktrees for parallel SDD worktree streams` confirms the mechanism was real. Supports topic + occ_kind (reinforced: execution + boundary-testing challenge held). The [40] execution and [41] challenge share this one date+ref and are correctly folded into ONE occurrence (not double-counted), per the cluster's own defensibility note. Distinct from occ 2: different date, different ref, new empirical content (actual merges + the challenge).

## Occurrence 4 - 2026-07-12, decided, "progress.md + plan dependency graph" (E7[62])

**CONFIRMED.** Primary sources:
- `.superpowers/sdd/plan-5.5/progress.md` line 7 ("Wave 1 as parallel worktree streams under .worktrees/, merged sequentially, full gate re-run per merge"), lines 91-95 ("six worktrees .worktrees/stream-a..f ... Planned merge order: D LAST among DiagCode/key-adders (C1+C4)"), C1 constraint lines 12-18 (T10's exhaustive-match param-fixture table over ALL DiagCodes; a DiagCode-adding stream merging after T10 MUST add its fixture or the build fails - "the guard working").
- Plan `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` lines 27-44 (Dependency graph / execution waves; "Wave-1 streams are disjoint code regions (verified: ...)").

Supports the full evidence string: "six disjoint worktree streams merged sequentially with full gate re-run; DiagCode stream merges last so its exhaustive guard forces the others' fixtures." Stream D = T10 (exhaustive DiagCode guard) merges last so all other DiagCode-adding streams have merged first and are forced through the guard. occ_kind decided; distinct Plan-5.5 execution event. (Dated 07-12 per the session-9 completion journal though the plan was authored 07-11; immaterial to the topic/kind, both supported.)

---

## Skepticism notes

- **No fabrication, no misattribution.** Every ref resolves to a real artifact whose text supports its stated (topic, occ_kind).
- **No duplication.** The four occurrences are four distinct (date, decision-event) pairs across four distinct contexts: Plan-3.5 failure origin (07-09), Plan-4 authoring (07-09), Plan-4 execution (07-10), Plan-5.5 six-stream run (07-12). No two share a date+ref.
- **[40]/[41] correctly collapsed.** The Plan-4 "was parallelism underused?" challenge shares Plan-4-complete's date+ref with the execution reinforcement and is folded into occ 3, not counted separately. No inflation there.
- **Borderline examined (occ 2 vs occ 3):** authoring the Plan-4 parallel structure (occ 2) and executing it (occ 3) are the two halves of Plan-4 parallelization split across a session boundary. They clear the cluster's stated counting unit (distinct date + distinct ref + distinct event-content: a decision vs a first-ever execution plus a live boundary-testing challenge), so counting them as two is defensible. Even under the most hostile reading that collapses them into one Plan-4 occurrence, **three distinct occurrences still survive** (Plan-3.5 origin, Plan-4, Plan-5.5), so the >= 3 promotion threshold holds either way. This is the only soft spot and it does not change the verdict.

**Result: verified_count = 4, CONFIRMED. The recurrence is real; promotion to standing house convention stands.**
