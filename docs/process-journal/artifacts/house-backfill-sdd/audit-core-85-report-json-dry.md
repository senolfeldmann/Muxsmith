# Audit: core-85-report-json-dry (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 3, promoted: true
**Statement under audit:** The batch/config/run JSON report documents (spec 7) are hoisted from the CLI into `core::report::json`, lifted 1:1, so CLI and GUI render byte-identical report structures from a single core module; neither surface owns document logic.

**Verdict: CONFIRMED** — all 3 occurrences survive; verified_count = 3. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no strict duplicate (no two refs point at the same artifact, none is a copy of another). The pattern is real and grounded in code (`crates/muxsmith-core/src/report/json.rs` carries `DiagnosticRenderer`, `batch_document`, `config_only_document`, `run_document`), deliberately specified, planned as a concrete task, and independently reviewed as byte-identical and spec-compliant. Two honest caveats (a wrong occurrence date, and the "same decision's lifecycle" nature of the count) are recorded below; neither meets the drop bar (fabricated / misattributed-topic / duplicate-of-another).

Calibration follows the sibling precedent set in `audit-core-40-output-collision-planned-twice.md`: the only drop triggers are fabricated / misattributed-topic / duplicate; a "decided" label that is really a *review* or a *source-principle* is a recorded kind-caveat, not a drop, as long as the topic genuinely and substantively arose in that distinct artifact.

---

## Base check (is the pattern real at all?)

Yes. `crates/muxsmith-core/src/report/json.rs` exists and contains:
- `pub trait DiagnosticRenderer` (line 23)
- `pub fn batch_document` (line 35)
- `pub fn config_only_document` (line 78)
- `pub fn run_document` (line 112)

`report.rs` was turned into `report/mod.rs` with a `pub mod json;`. CLI call sites (`dry_run.rs`, `run.rs`) delegate to these. The hoist happened; the statement is not a fabrication at the base.

---

## Per-occurrence verification

### Occurrence 1 — "spec §7" (2026-07-10, kind: decided) — SURVIVES

Ref: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, `## 7. Architecture`, "Rules that keep it DRY" (line 349):
> CLI and GUI render the same diagnostic and report structures; neither owns logic.

- This is the source DRY principle the whole pattern instantiates. The statement itself parenthesizes "(spec 7)" as its origin, and Plan 5's architecture paragraph derives the hoist explicitly from the "spec 7 DRY rule." So spec §7 is correctly attributed as the originating design decision for this pattern, not a different topic. Kind "decided" is accurate at the design-rule level (the spec decides the DRY rule).
- **Caveat A (date, does not trigger drop):** the principle lives in the `2026-07-08` v1-design doc, i.e. it was decided ~2026-07-08, not the claimed `2026-07-10`. Minor provenance imprecision; the drop bar is fabricated/misattributed-topic/duplicate, none of which this is.
- **Caveat B (breadth, does not trigger drop):** spec §7's DRY rule is broader than the report-JSON hoist — it also covers zero frontend semantic validation and single-source schema/validation. This occurrence is the *general principle*, not the specific `report::json` decision. It is the weakest of the three on topic-specificity, but it genuinely is the source of *this* pattern (the plan and the statement both cite it as such), so the topic arose here. Kept.

### Occurrence 2 — "Plan T2" (2026-07-10, kind: decided) — SURVIVES

Ref: `docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md`, Task 2 "Hoist report JSON assembly into core (`report::json`)" (line 82+), plus the architecture paragraph (line 7): "a hoist of the batch/run JSON document assembly from the CLI into `core::report` so both surfaces render identical structures (spec 7 DRY rule)."

- This is the concrete decision that *is* the statement: move `batch_document`/`config_only_document`/`run_document` into `core::report::json`, "signatures lifted 1:1... byte-identical documents," consumed by the GUI's IPC (T7/T8). Rock solid.
- Kind "decided" accurate (a plan task is a decision to build X this way). Date `2026-07-10` matches the plan doc. Distinct physical artifact from Occ 1 (it cites spec §7 as rationale but instantiates it with the actual module and function names — restatement-with-specifics, not a copy).

### Occurrence 3 — "task-2 verdict" (2026-07-10, kind: decided) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-5-sdd/verdicts/task-2-review-verdict.md` (final message ts `2026-07-10T11:48:59Z`).

- Independent review verdict, "Task quality: Approved / Spec compliant." Substantive, distinct work — not a copy of the plan or the report: it compared each moved function body line-by-line against the deleted CLI code ("character-identical except for the renamed function/param and the `&Renderer` -> `&dyn DiagnosticRenderer`"), checked `muxsmith-core/Cargo.toml` for absence of `fluent-bundle`/`muxsmith-cli`, grepped the tree for stale `batch_json(`/`config_only_json(`/`run_json_document(` call sites, and adjudicated the `DiagnosticRenderer` design deviation against "spec 7 is trying to kill [document-shape duplication]." The topic (byte-identical hoist, neither surface owns document logic) materially arose and was verified here.
- Date `2026-07-10` matches.
- **Caveat C (kind, does not trigger drop):** this is a *review/verification*, not a *decision*. "decided" is imprecise; "reviewed" would be exact. Per the core-40 precedent, a distinct artifact where the topic substantively arose is kept despite a loose kind label.

---

## Corroboration outside the three refs

The pattern is echoed consistently across the tree, confirming it is settled and load-bearing, not a one-off:
- `task-2-brief.md:10` / `task-2-report.md` — the executed hoist, DRY rationale quoted from spec 7.
- Rustdoc in code cites "spec 7" for the crate boundary (core "emits no user-facing prose... for the CLI and GUI renderers"), reinforced in plan-1/3/4/5 review diffs.
- `task-10-report.md:101`, plan-5 review diffs (`review-c822a17..42ecc34.diff:540-541`, `review-945ee96..7a2bc15.diff`): downstream tasks reuse the same `run_document` shape "so 'CLI and GUI render the same diagnostic and report structures; neither owns logic' (spec 7) holds by construction."

---

## Honest caveat on the *nature* of the count (recorded, below the drop bar)

All three occurrences are one decision carried through its SDD lifecycle: source principle (spec §7) -> concrete plan task (Plan T2) -> independent review verdict (task-2 verdict), clustered on 2026-07-10 (the principle itself predating it, 2026-07-08). This is one design decision fully pipelined and verified, **not** the pattern independently recurring in three unrelated contexts. Two of the three "decided" labels are imprecise (spec §7 is a broader source principle; the verdict is a review).

This is legitimate for a **settled** pattern: deliberate spec rule + concrete plan + line-by-line independent review + grounding code is exactly the evidence a standing convention should carry, and none of it is fabricated. But a future consumer of this house-knowledge should read "count: 3" as "one decision — specified, planned, and independently verified," not "this issue kept cropping up." Under the audit's strict drop criteria (fabricated / misattributed-topic / duplicate-of-another) none of the three qualifies, so the count is not reduced.

**verified_count = 3 -> CONFIRMED.**
