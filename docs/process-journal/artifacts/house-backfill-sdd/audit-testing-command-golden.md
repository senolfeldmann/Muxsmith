# Audit - testing-command-golden (PROMOTION candidate)

**Cluster:** `testing-command-golden` (kind: pattern, domain: testing, status: settled)
**Claimed count:** 3 | **promoted:** true | **promoted_at:** 3
**Statement:** Command generation (Plan -> mkvmerge argv) is verified by golden tests mapping fixture identification JSON to expected argv; built incrementally with full-vector `assert_eq` goldens where each command task extends and re-locks the prior task's golden as an intended contract change (Task 11 added donor `--no-attachments` to Task 10's golden), culminating in the full spec 4.1 reference example locked as a pure golden.

**Audit question per occurrence:** does the cited artifact actually support "this (topic, approach) arose here as {occ.kind}"? Drop if fabricated, misattributed, or a duplicate of another listed occurrence. Since promotion makes this a standing convention, the count must be real: three genuinely distinct occurrences.

**Plan mapping:** command generation is Plan 3 (`docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md`, Tasks 9-12); the golden-decision origin is the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`). Verdict files are the salvaged Plan-3 reviews under `docs/process-journal/artifacts/plan-3-sdd/verdicts/`.

---

## Occurrence 1 - 2026-07-08 / decided - SURVIVES

**Ref:** spec 2026-07-08 §10
**Evidence claimed:** "command: golden tests, fixture identification JSON -> expected argv."

**Primary artifact read:** `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
- Line 406: `## 10. Testing`
- Line 409 (verbatim): `- ` `command` `: golden tests, fixture identification JSON -> expected argv.`

**Verdict:** CONFIRMED. Exact, byte-faithful match under the §10 Testing heading. This is the genesis decision to verify command argv by fixture-JSON->argv golden tests. occ_kind `decided` fits (spec-level design decision). Distinct artifact (the v1 spec), distinct date, no overlap with occ 2/3.

---

## Occurrence 2 - 2026-07-09 / decided - SURVIVES

**Ref:** plan Tasks 9-12 + journal Deltas + task-11-review-verdict.md
**Evidence claimed:** "each command task extended the argv and updated the prior task's golden (Task 11 added donor `--no-attachments` to Task 10's golden), verified not a regression."

**Primary artifacts read:**
- `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md` - Tasks 9-12 are the command-generation tasks, each opening with a failing golden test and the canonical argv "locked by Task 9-11 golden tests" (line 128). Task 9 (line 483, "global section + track-order skeleton", Step 1 "Write the failing golden test", line 494); Task 10 (line 534, multi-input grouping + per-track property options, golden asserting full argv incl. `--track-order 0:0,0:1,1:0`, line 544); Task 11 (line 565, "attachments, chapters, tags argv", Step 1 cases incl. `Subset -> --attachments`, donor group `--no-attachments`, lines 575-585). Confirms the incremental extend-and-relock structure is planned, not incidental.
- `docs/process-journal.md` line 229 (session 3 Deltas) - near-verbatim source of the evidence quote: *"The plan's incremental-golden design worked: each command task extended the argv and updated the prior task's golden (Task 11 added donor `--no-attachments` to Task 10's golden), verified not a regression."*
- `docs/process-journal/artifacts/plan-3-sdd/verdicts/task-11-review-verdict.md` - NAMED CHECK on `per_track_properties_and_multi_group` (Task 10's golden, tests/command.rs:162-200): *"The only change is a single inserted line, `--no-attachments`, at line 180, positioned immediately after the primary group's closing `)` ... i.e. slot (c) of the donor group ... This is a legitimate incremental golden-test lock, not a masked regression."*
- Cross-check (git log): Task 11 is commit `afa8074 feat(command): attachments, chapters, and tags argv`, following Task 10's `61ab07c feat(command): per-track property options and multi-input grouping` - two distinct commits, the extend-and-relock is real.

**Verdict:** CONFIRMED. All three cited sub-artifacts (plan Tasks 9-12, journal Deltas, task-11 verdict) exist and each supports the claim; the specific "`--no-attachments` added to Task 10's golden, not a regression" fact is confirmed at diff-line granularity by the task-11 verdict's NAMED CHECK. occ_kind `decided` fits (the plan decides the incremental-golden discipline; the task applies it). Not fabricated, not misattributed.

---

## Occurrence 3 - 2026-07-09 / decided - SURVIVES

**Ref:** plan Task 12 + task-12-review-verdict.md
**Evidence claimed:** "Pure golden = FULL spec 4.1 reference example ... locks the reference example end-to-end as a pure golden (no binary)."

**Primary artifacts read:**
- `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md` Task 12 (line 591, "integration - reference-example golden + real-mkvmerge acceptance"): line 599 *"a full golden for the spec 4.1 reference example (pure, no binary needed)"*; line 602 *"**Pure golden:** construct ... a `Plan` and assert the full argv string. This locks the reference example end to end without a binary."* Live real-mkvmerge acceptance is a separate, gated case (line 604, "live test skips if no binary").
- `docs/process-journal/artifacts/plan-3-sdd/verdicts/task-12-review-verdict.md` - reviewer read spec §4.1 (`...v1-design.md:49-116`) and confirmed `tests/fixtures/reference.yaml` is *"byte-for-byte the reference example ... The 'full example, not a subset' claim is accurate, not just asserted"*; the pure golden is full-vector: *"`assert_eq!(command(plan), expected)` (`command_integration.rs:214`) is full-vector equality, not substring/contains."* Live case is separately gated (`command_integration.rs:233-237`), so the reference-example lock is genuinely pure/no-binary.
- Cross-check: spec §4.1 "Reference example" exists at `...v1-design.md:49`; Task 12 is a distinct commit `0dcb116 test(command): reference-example golden and live mkvmerge acceptance`.

**Verdict:** CONFIRMED. Plan Task 12 and the task-12 verdict both support the pure full-reference golden as claimed. occ_kind `decided` fits.

**Distinctness check vs occ 2 (both 2026-07-09, both cite "plan" = Plan 3):** not a duplicate. Occ 2 is the *extend-and-relock discipline* anchored to Task 11 (commit afa8074, task-11 verdict, journal Deltas) - modifying a prior task's golden as an intended contract change. Occ 3 is the *culminating pure reference-example golden* anchored to Task 12 (commit 0dcb116, task-12 verdict) - locking spec §4.1 end-to-end without a binary. Different tasks, different commits, different verdict files, different specific mechanic. Non-identical (date, ref) pairs; genuine distinct facets of the same pattern, exactly the recurrence the cluster claims.

---

## Result

| # | date | occ_kind | ref | status |
|---|---|---|---|---|
| 1 | 2026-07-08 | decided | spec 2026-07-08 §10 | SURVIVES |
| 2 | 2026-07-09 | decided | plan Tasks 9-12 + journal Deltas + task-11-review-verdict.md | SURVIVES |
| 3 | 2026-07-09 | decided | plan Task 12 + task-12-review-verdict.md | SURVIVES |

**verified_count = 3** (three distinct surviving occurrences).

**Verdict: CONFIRMED.** All three occurrences are backed by real artifacts whose text supports the claimed (topic, approach, occ_kind); none is fabricated, misattributed, or a duplicate. The recurrence is genuine: the golden-test approach is *decided* in the v1 spec (occ 1), applied as an incremental extend-and-relock discipline confirmed in Task 11's review + journal (occ 2), and culminates in the pure spec-4.1 reference-example golden confirmed in Task 12's review (occ 3). Count 3 holds; promotion to standing house convention stands.
