# Audit: exec-40-persist-perjob-logs (PROMOTION candidate)

**Cluster kind:** pattern (domain: executor), status: settled, claimed count: 3, promoted: true
**Statement under audit:** Persisting per-job logs was deferred twice - at D7 (to Plan 5, the GUI job-queue view being the consumer, CLI streaming to stdout + `--json` meanwhile) and again at the Plan-4 self-review - before being adopted in D26 as `runs/<run-id>/{summary.json, job-<index>.json}` under the platform data dir, written unconditionally by core for both CLI and GUI runs; dry-runs persist nothing.

**Verdict: REJECTED** — occurrence 2 dropped as a non-substantive carry-forward restatement of occurrence 1's deferral; only 2 distinct occurrences survive (defer at D7, adopt at D26). Below the promotion threshold of 3. **Demote to Tier 1** (decision ledger); the D26 decision itself stays authoritative in the plan-5 GUI design memo.

The three cited artifacts all *exist* and none is fabricated. The factual claims about D26's shape are exactly correct. The candidate fails not on fabrication but on **count integrity**: two of the three "occurrences" are the *same deferral decision*, one of them a bare bookkeeping restatement.

---

## Per-occurrence verification

### Occurrence 1 — "memo D7 (Deferrals)" (2026-07-09, kind: deferred) — SURVIVES

Ref: `docs/superpowers/specs/2026-07-09-plan-3-design-decisions.md`, §D7, **Deferrals** subsection (lines 37-40). Committed `497502e` 2026-07-09 11:21 ("docs: Plan 3 design decisions (D7-D11, pure layer)").

Verbatim:
> **Deferrals.** Persisted per-job logs in the platform app-data directory (spec 6, "mkvtoolnix-gui-style job log") are deferred to Plan 5: the job-queue view is their consumer, and the CLI `run` in Plan 4 streams progress/results to stdout + `--json` instead.

This is the **originating deferral decision**. Text exists, correctly attributed to D7, kind "deferred" is exact. The evidence quote in the cluster matches. Rock solid. **KEEP.**

### Occurrence 2 — "plan-4 Self-review" (claimed 2026-07-10, kind: deferred) — DROPPED (duplicate)

Ref: `docs/superpowers/plans/2026-07-09-plan-4-executor-run-queue.md`, `## Self-review (controller, after all tasks)` section, line 546 (**Memo coverage** bullet). Committed `c0c0ef7` **2026-07-09 21:21** ("docs: Plan 4 implementation plan").

Verbatim:
> **Memo coverage:** D13 -> T1-T3 ... D18 -> T5, T6, T7. **Deferred by decision:** NDJSON `--json-events` (v1.x); **persisted job logs (Plan 5)**; `--fail-fast=now` (v1.x); zero-track empty-plan warning (cleanup pass).

The text exists and is correctly located in the plan-4 self-review — not fabricated, not misattributed. It is dropped as a **duplicate of Occurrence 1** for three reasons:

1. **Its own wording labels it a restatement.** "Deferred **by decision**: ... persisted job logs (Plan 5)" is, by construction, a pointer to an *already-made* decision (the D7 deferral). It records no new deferral, no re-deliberation, no independent analysis — it is a one-line coverage-checklist entry confirming the D7 deferral is honored (i.e. not built) in Plan 4.

2. **No distinct facet.** The sibling house-audit `audit-core-40` sets the standard for keeping multiple occurrences of one decision: each kept occurrence added a *distinct substantive facet* (the decision, the commit+tests implementing it, an independent review that ran tests and found a new bug). Occurrence 2 adds no facet — it neither implements, verifies, nor re-decides the deferral. It is the same deferral status, re-listed. That is precisely what "duplicate of another listed occurrence" targets, and it is weaker than anything `audit-core-40` chose to keep.

3. **It is one of at least six identical carry-forwards, arbitrarily selected.** The single D7 deferral is echoed in bookkeeping across the whole plan-3→5 span — none of them a second decision:
   - `plan-3-close.md:63` "...are deferred here (the GUI job-queue view is their consumer)."
   - `plan-3.5-close.md:92` "Deferred by decision (do NOT build): ... persisted job logs (Plan 5)"
   - `plan-3.5-close.md:113`, `plan-3-resolution-command.md:622` "-> Plan 5 (GUI)"
   - `plan-4-executor-run-queue.md:546` (this occurrence) **and** its verbatim twin `plan-4-sdd/task-11-brief.md:13`
   - `plan-4-close.md:109` "...were EXPLICITLY deferred to Plan 5 - they are in scope now."
   
   Picking one carry-forward line and promoting it to a distinct "second deferral" is exactly the count inflation this audit exists to catch. Any of the six would have served equally; none is a genuine second deliberation.

**Secondary integrity flag:** the cluster dates this occurrence 2026-07-10, but git shows the self-review line was authored 2026-07-09 21:21 — part of *authoring* the plan-4 plan, not a post-close review on 07-10. So it is not even a temporally-later touchpoint; it sits the same day as D7, ~10h after it. This reinforces that it is plan-authoring bookkeeping, not an independent recurrence.

**DROP.**

### Occurrence 3 — "memo D26 (format+scope Şenol)" (2026-07-10, kind: decided) — SURVIVES

Ref: `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md`, §D26 (lines 106-132). Committed `c121d70` 2026-07-10 12:45 ("docs: Plan 5 GUI design memo FINAL (D22-D30)").

The **adoption decision**, with the full approach the cluster statement describes, verbatim:
> **Decision.** (Format and both-surfaces scope: Şenol 2026-07-10.) A new `executor::joblog` module writes, per run, a directory `<platform-data-dir>/muxsmith/runs/<run-id>/` containing `summary.json` ... and `job-<index>.json` per job ... The queue layer writes incrementally ... so CLI `run` and GUI runs both persist unconditionally. Dry-runs persist nothing.

Rationale line matches the cluster's D26 evidence quote exactly:
> Rationale: spec 6 phrases persistence as a job-engine property ... so GUI-only persistence would diverge from the spec ...

Confirmed by Şenol, format and both-surfaces scope, kind "decided" exact. Text exists, correctly attributed, real distinct artifact and commit. **KEEP.**

---

## Why this fails the threshold

Strip the bookkeeping and the actual history of this feature is the **mundane two-step lifecycle of any deferred feature**: deferred once (D7), adopted once (D26). That is not a *recurring pattern* — it is one feature parked and later built. `decision-ledger.md` warns explicitly against "overfitting one-offs into the rulebook"; a defer→adopt pair is the canonical one-off. The claimed count of 3, and thus the promotion, rests entirely on counting one of six interchangeable carry-forward mentions (the plan-4 self-review) as a distinct third occurrence. It is not.

Two genuine, substantively-distinct occurrences survive (D7 deferral; D26 adoption). **verified_count = 2 < 3 → REJECTED.** The candidate should be demoted to Tier 1: keep it in the decision ledger at count 2 (defer / adopt), do **not** promote it to an always-loaded standing convention. The D26 design decision remains authoritative where it already lives (plan-5 GUI design memo, §D26).

## Contrast with the CONFIRMED sibling (calibration note)

`audit-core-40` also had "three occurrences of one decision on one date" yet was CONFIRMED — the difference is decisive and worth recording so this pair stays consistent: there the three were **decide / implement+test / independently-review-and-find-a-new-bug**, three distinct substantive facets, each adding evidence the rule is real and load-bearing. Here occurrence 2 is a bare carry-forward mention adding no facet. Distinct facets of one decision survive; a restatement of one does not.
