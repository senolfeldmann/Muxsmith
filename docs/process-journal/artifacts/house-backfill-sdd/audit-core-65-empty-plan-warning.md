# Audit: core-65-empty-plan-warning (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 4, promoted: true (promoted_at: 3)
**Statement under audit:** A plan resolving to zero track assignments emits `EmptyPlan` (warning, per-file, batch-visible); keep-mode plans are naturally exempt. A deliberate divergence from mkvtoolnix-gui: an interactive GUI shows selection state by construction, a declarative batch tool must state it. Deferred twice (Plan 3, Plan 4/D18) before being built at T6, where the check was relocated post-finalize (gated on `plan.is_some()`), closing an unenumerated `CollisionPolicy::Skip` false-warning case.

**Verdict: CONFIRMED** — all 4 occurrences survive; verified_count = 4. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no duplicate. Unlike the single-day pipelined clusters, this is a genuine multi-plan recurrence: the same topic (a zero-track plan producing no diagnostic) surfaced as a review finding in Plan 3, was re-triaged and re-deferred in the Plan 4 design memo, then built and immediately corrected in Plan 5.5. Four distinct artifacts, four distinct decision points. Two minor date/label caveats are recorded at the end; neither meets the drop bar.

---

## Per-occurrence verification

### Occurrence 1 — "whole-branch verdict Minor 3 (Plan 3)" (2026-07-09, kind: deferred) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-3-sdd/verdicts/whole-branch-review-verdict.md` (salvaged from the Plan 3 whole-branch review session; `final_message_ts: 2026-07-09T13:27:30Z`).

- Under **"#### Minor (Nice to Have)"**, item **3** verbatim:
  *"A zero-track plan renders a valid-but-empty MKV with no diagnostic. If every rule is `optional` and matches nothing, the plan survives (no error), and `command` emits an all-`--no-*` argv; mkvmerge produces an empty MKV (exit 0, verified). Nothing warns that a plan resolved to zero output tracks. Narrow (needs an all-optional profile), and arguably Plan 4 / a planner 'empty plan' diagnostic rather than Plan 3. Deferrable, worth recording."*
- Reinforced in the same file's **Recommendations**: *"Consider a planner 'plan resolves to zero output tracks' warning (Minor 3), or defer explicitly to Plan 4."*
- Topic, approach (a planner-level warning), and kind (`deferred` — explicitly "Deferrable", punted toward Plan 4) all match exactly. The ref label "Minor 3" is literally correct. Rock solid; this is the origin.

### Occurrence 2 — "memo D18 (Plan-4 cleanup)" (2026-07-10, kind: deferred) — SURVIVES

Ref: `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`, section **"## D18: Plan 3 follow-up triage"** (lines 138-155).

- Under **"Deferred to a later cleanup pass:"** verbatim:
  *"zero-track plan renders an empty MKV with no diagnostic (planner 'empty plan' warning); ..."*
- D18 is exactly the triage decision that took the Plan 3 review's Minor items and split them into "Into Plan 4" vs "Deferred to a later cleanup pass". The empty-plan warning was placed in the deferred bucket — a *second, distinct* deferral, decided in a different artifact (the Plan 4 design memo) than Occurrence 1 (the Plan 3 review verdict). Not a duplicate: one is a reviewer's finding recommending deferral, the other is the controller/owner triage that ratifies it and names the vehicle ("a later cleanup pass").
- Corroborated independently in `.../plan-4-sdd/task-11-brief.md` line 13: *"Deferred by decision: ... zero-track empty-plan warning (cleanup pass)."*
- **Date caveat (does not trigger drop):** the memo file is dated 2026-07-09 in its filename; the occurrence is dated 2026-07-10. The journal (line 265) records the memo as "written, parked, UNREVIEWED by Şenol — review first", with review at Plan 4 start (07-10). Dating the deferral to its ratification (07-10) rather than its drafting (07-09) is defensible; the substance is unambiguous.

### Occurrence 3 — "task-6 (EmptyPlan built)" (2026-07-11, kind: decided) — SURVIVES

Ref: Plan 5.5 Task 6 — `.superpowers/sdd/plan-5.5/task-6-brief.md`, `.../task-6-verdict.md`, commit `d9db161..c09875e`.

- The brief's own header: **"Task 6: Zero-track plan warning (#6 - warning only, decided)"** — the "cleanup pass" deferred in D18 finally scheduled as a built task. Steps specify the `EmptyPlan` DiagCode (warning), per-file diagnostic, batch-report visibility, and the keep-mode exemption ("a keep-mode plan always carries the primary's tracks, so the warning naturally cannot fire there").
- The verdict confirms it was built and is correct: *"Core predicate, D20 handling, batch-visibility plumbing correct and source-verified."*
- Kind `decided` is accurate: this is where the long-deferred item transitioned from deferred to built. Distinct event from Occurrence 4 (the initial build, commit `c09875e`, vs. the fix wave `a60e9a0`).

### Occurrence 4 — "task-6 fix a60e9a0 (relocation)" (2026-07-11, kind: violated-corrected) — SURVIVES

Ref: `git show a60e9a0` — *"fix(planner): EmptyPlan decided post-finalize, batch-report test (T6 review)"*, 2026-07-11 22:27:13 +0200; plus `.../task-6-verdict.md`.

- The initial T6 build placed the check inside `resolve_file` (pre-finalize). The verdict Round 1 flags it: *"Error-severity guard judged CORRECT in intent ... but structurally unreachable for cross-file passes (push in resolve_file predates them). ... IMPORTANT 2: relocate the check post-finalize_plans gated on f.plan.is_some()."* — a real implementation defect caught in review, hence `violated-corrected` is the right kind.
- The fix wave is verified in code, not just prose. The `a60e9a0` diff on `planner.rs` removes the old pre-finalize `if plan.is_some() && !has_tracks && ...` push and adds a relocated `detect_empty_plans(&mut files)` run after both finalize passes, gating on the surviving plan: `let Some(plan) = &f.plan else { continue };` and recomputing `has_tracks` from `plan.assignments` / `plan.keep_unmatched` / `plan.primary_track_ids`.
- The verdict confirms the load-bearing claim in the statement: the relocation *"closes a THIRD unenumerated case (CollisionPolicy::Skip nulls the plan with only a warning — the old guard could never catch the false EmptyPlan there)."* This is exactly the statement's "closing an unenumerated `CollisionPolicy::Skip` false-warning case." Verified both in the reviewer artifact and in the diff logic (a Skip-nulled `plan: None` is now `continue`-skipped, so no false EmptyPlan is stacked on it).

---

## Corroboration outside the four refs

- `docs/ROADMAP.md` line 159: *"Zero-track plan warning: DONE 2026-07-12 (Plan 5.5 T6; EmptyPlan decided post-finalize, batch-report test; deliberate divergence recorded in the plan-5.5 memo)."*
- The SI-3 divergence claim in the statement ("interactive GUI shows selection state by construction, a declarative batch tool must state it") is recorded in the T6 verdict: *"mkvtoolnix-gui has no general zero-selected-tracks warning; deliberate divergence recorded in the plan-5.5 memo."*
- The deferral chain is independently narrated in `docs/process-journal.md` (line 231, Plan 3 -> Plan 4 open-threads handoff naming the "planner empty-plan warning?"; line 444, "zero-track -> warning-only, one sane default, options parked in IDEAS #5"; line 645, the T6 "EmptyPlan false-warning window on cross-file drops").

The rule is settled: deferred through two plans, built to spec, review-corrected, batch-report-tested, and recorded as a deliberate mkvtoolnix divergence — exactly the profile a standing convention should have.

---

## Caveats recorded (below the drop bar)

1. **Occurrence 2 date** is the ratification date (07-10), not the memo's draft date (07-09). Defensible; noted for precision.
2. **ROADMAP DONE date (07-12)** differs from the T6 occurrence dates (07-11). The occurrences track the actual commits (`c09875e` build, `a60e9a0` fix, both 2026-07-11); the ROADMAP roll-up is dated to the next day. A downstream-date drift in the ROADMAP line, not a defect in any occurrence.

Neither caveat is a fabrication, misattribution, or duplicate, so neither reduces the count. Unlike single-day pipelined clusters, all four occurrences are substantively distinct events in distinct artifacts across three plans (Plan 3 review -> Plan 4 memo -> Plan 5.5 build -> Plan 5.5 fix). The count is a real recurrence, not one decision counted four times.

**verified_count = 4 -> CONFIRMED.**
