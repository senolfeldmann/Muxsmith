# Audit: cli-09-collision-parity (PROMOTION candidate)

- **Cluster id:** `cli-09-collision-parity`
- **Kind / domain:** pattern / cli
- **Claimed count:** 3 (promoted, `promoted_at: 3`)
- **Statement:** `--on-collision <error|skip|overwrite>` exposed on both `run` and `dry-run` for
  parity, via a CLI-local `CollisionArg` (core stays clap-free) mapping 1:1 to `CollisionPolicy`
  and threading into the pre-existing `RunInputs.on_collision`. `run` reuses `CollisionArg`
  verbatim; the rerun workflow (D14/D17) is where the flag is needed.
- **Verdict:** **CONFIRMED** - 3 distinct occurrences survive; promotion stands.
- **verified_count:** 3

Anchoring: E5 = Plan 4 (~2026-07-10). "memo D15" = design-decision D15 in
`docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`. Verdicts live under
`docs/process-journal/artifacts/plan-4-sdd/verdicts/`. The `find-E5.md` / `cluster-cli.md`
files are synthesized reconstructions; every occurrence below was verified against the
**primary** artifact it cites, not against those summaries.

---

## Occurrence 1 - memo D15, `decided` - SURVIVES

- **Ref:** `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`, section `## D15`, lines 98-102.
- **Cited evidence:** "Spec 4.2 names the override as a run input; `RunInputs.on_collision`
  existed since Plan 2 but no flag exposed it, and run is where the rerun workflow needs it."
- **What the artifact actually says** (under the `**Decision.**` heading of D15, verbatim):
  > `--on-collision <error|skip|overwrite>` is added to both `run` and `dry-run` (parity: both
  > plan). Spec 4.2 names the collision-policy override as a run input with CLI flags overriding
  > profile defaults; `RunInputs.on_collision` has existed since Plan 2 but no flag exposes it,
  > and `run` is where the rerun workflow (D14, D17) needs it.
- **Assessment:** Exact match. It is a genuine `decided` occurrence (formal `**Decision.**`
  block), and every clause of the evidence line is present verbatim: parity on both surfaces,
  Spec 4.2 as a run input, RunInputs.on_collision since Plan 2, run/D14/D17 rerun rationale.
  Distinct artifact (the design-decision memo). **Not** fabricated, misattributed, or a duplicate.

## Occurrence 2 - task-4-review-verdict.md, `reinforced` - SURVIVES

- **Ref:** `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-4-review-verdict.md`.
- **Cited evidence:** "`CollisionArg` CLI-local, 1:1 to `CollisionPolicy`, threads into
  `RunInputs.on_collision` (planner.rs:249 fallback)."
- **What the artifact actually says:**
  - Line 16: "`RunInputs.on_collision` was already `Option<CollisionPolicy>` with the fallback
    semantics documented and implemented in core (`planner.rs:249:
    run.on_collision.unwrap_or(profile.output.on_collision)`), predating this task."
  - Line 19: "`CollisionArg` is CLI-local (`crates/muxsmith-cli/src/cli.rs:56-74`), core
    untouched, 1:1 mapping matches core's `CollisionPolicy` exactly (`error|skip|overwrite`, no
    default arm ...). `dry_run::run` gains `on_collision: Option<CollisionPolicy>` ... replacing
    the hardcoded `None` ..."
  - Line 26 re-attests the `planner.rs:249` fallback as real, not aspirational.
- **Assessment:** Full, explicit, independent re-attestation of the pattern on the **dry-run**
  surface, including the exact `planner.rs:249` fallback detail. A genuine `reinforced` (a review
  verdict separate from the decision memo). Line 24 even calls it "a task explicitly setting up
  reuse for Task 8", tying it into the parity/reuse story. Distinct artifact. **Not** fabricated,
  misattributed, or a duplicate of occurrence 1 (memo) or occurrence 3 (task-8 verdict).

## Occurrence 3 - task-8-review-verdict.md, `reinforced` - SURVIVES (with evidence caveat)

- **Ref:** `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-8-review-verdict.md`.
- **Cited evidence:** "Run's clap variant reuses `CollisionArg` verbatim; the rerun workflow
  (D14/D17) needs the flag."
- **What the artifact actually says:**
  - Line 21: "Step 1 clap `Run` variant matches the brief field-for-field (`cli.rs:57-83`), doc
    comments added ..." The brief it matches (`task-8-brief.md`, Step 1) is where the run variant
    reuses `CollisionArg`: `#[arg(long, value_enum)] on_collision: Option<CollisionArg>`, and
    "Consumes: ... T4 `CollisionArg`".
  - Line 22 / 23 / 35: explicit run<->dry-run **parity** of flow ("identical to dry-run through
    `plan_batch`", "byte-identical to dry-run's order", shared helpers "single source, no drift").
- **Caveat (adversarial):** the verdict text itself contains **none** of the tokens
  `CollisionArg`, `collision`, `on_collision`, `D14`, `D17`, or `rerun` (grep-confirmed empty).
  The reuse is attested only **transitively** - via "matches the brief field-for-field", and the
  brief is what carries the `CollisionArg` field. The evidence line's "rerun workflow (D14/D17)"
  rationale is **imported from the D15 memo**, not stated in this verdict; the phrasing
  overstates the verdict's explicitness.
- **Assessment: keep.** The audit test is whether the ref supports that the topic (run surface
  reusing `CollisionArg` for the flag) arose here as a reinforcement. It does: task-8 is
  literally the task adding the run flag by reusing `CollisionArg`, and this verdict independently
  reviewed and approved that run variant field-for-field, plus explicitly certified run/dry-run
  parity. The occurrence is real, on a distinct surface, in a distinct artifact (different task,
  different reviewer subagent/session `tool_use_id`) - **not** fabricated, misattributed, or a
  duplicate. The defect is evidence-summary embellishment (transitive support dressed as an
  explicit "reuses CollisionArg verbatim" statement, plus grafted memo rationale), which lowers
  evidence quality but does not sink the occurrence.

---

## Distinctness / count integrity

Three genuinely distinct artifacts, three distinct epistemic roles:

| # | Ref | Kind | Surface | Distinct? |
|---|-----|------|---------|-----------|
| 1 | memo D15 (plan-4 design decisions) | decided | both (the decision) | yes |
| 2 | task-4-review-verdict.md | reinforced | dry-run | yes |
| 3 | task-8-review-verdict.md | reinforced | run | yes |

No same-document double-count; no decided/reinforced pair collapses onto one file. This matches
`cluster-cli.md`'s own count-integrity note (cli-09 "additionally re-attests on both the run and
dry-run surfaces"). Occurrences 2 and 3 attest **different** surfaces of the same reuse pattern,
so they are not duplicates of each other.

## Bottom line

All 3 occurrences survive as distinct, genuine attestations of the same pattern. No fabricated
recurrence. `verified_count = 3` >= 3 -> **CONFIRMED**, promotion to standing house-knowledge
holds. One quality note for the record: occurrence 3's evidence line embellishes what the task-8
verdict literally says (transitive, not explicit, and imports the D14/D17 rationale from the
memo) - worth tightening if the cluster's evidence text is ever surfaced as the rule's citation,
but it does not change the verdict.
