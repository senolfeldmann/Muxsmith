# Audit: core-22-edition-2024-codegen-rename (PROMOTION candidate)

**Auditor verdict: REJECTED.** verified_count = 1 distinct occurrence (claimed 3). Demote to Tier 1.

The pattern itself is true and the engineering call was correct. It fails promotion because its recurrence count is not real: all three cited "occurrences" are three documentation facets of one single event, not three independent arisings of the pattern.

---

## Claim under audit

- **Statement:** `gen` is reserved in Rust edition 2024; the implementer's edition-2021 downgrade workaround was rejected by the controller and the module renamed to `codegen` (root cause over symptom).
- **Cluster (cluster-core.md:57):** three occurrences, all `2026-07-08 violated-corrected` — (journal T5), (task-5 review verdict), (commit e78847d).
- **Promotion basis:** count = 3 → standing convention.

## Occurrence-by-occurrence verification

| # | ref | artifact exists? | on-topic? | supports `violated-corrected`? | verdict |
|---|-----|------------------|-----------|-------------------------------|---------|
| 1 | journal T5 | yes — `docs/process-journal.md:44-45`, `:88-89` | yes | yes (records the episode) | **DROP — duplicate of #3** |
| 2 | task-5 review verdict | yes — `docs/process-journal/artifacts/plan-1-sdd/verdicts/task-5-review-verdict.md` | yes | yes (reviews the rename fix) | **DROP — duplicate of #3** |
| 3 | commit e78847d | yes — `git show e78847d` | yes | yes (is the correction) | **KEEP — canonical occurrence** |

None of the three is fabricated and none is misattributed. All three genuinely concern the `gen`/edition-2024 collision and the rename. The problem is not authenticity, it is **independence**.

## Why 1, not 3: it is one event with three paper trails

The full event chain, all on 2026-07-08, all in plan-1 task 5 (the `xtask` crate):

- `830dc47` (01:30) — original `feat(xtask)` commit. Introduces `crates/xtask/src/gen.rs` with an explicit `edition = "2021"` override on `crates/xtask/Cargo.toml`. **This is the violation** (task-5-report.md "Edition Override Note": *"Workaround: Set xtask to edition 2021 explicitly ... overriding workspace default"*).
- `e78847d` (01:32, two minutes later) — `fix(xtask): rename gen module to codegen`. **This is the correction** (report.md:86: *"Root-cause fix per coordinator review"*; restores `edition.workspace = true`).
- **task-5 review verdict** — reviews `e78847d`: *"the authorized rename fix (`gen` -> `codegen`), which is in scope per the controller's amendment"*, *"Root-cause fix, not a patch"*. It is the review OF occurrence #3, not a separate arising.
- **journal T5** — retrospective one-line summary of the same episode: *"`gen` reserved in edition 2024 (implementer T5); implementer's edition-2021 downgrade workaround rejected by controller -> module renamed."* It is a record OF the same episode.

The `gen`-reserved-keyword collision arose **exactly once** in the entire project. Verified negatives:

- `e78847d` is the **only** rename commit (`git log --all --grep` on codegen/edition-2024/gen module returns it alone).
- The collision appears in **only one** task's material (plan-1 task 5 / xtask). The other journal "T5" lines are Plan 4's unrelated task 5 (tests/support consolidation, lost-cancellation race). The `match` reserved-word handling in `planner.rs` / plan-2 F7 is a different keyword and a different mechanism (serde `rename`), not this pattern.

## The accounting error, stated plainly

A promotion threshold of ">=3 occurrences" is a **recurrence-trust mechanism**: a pattern earns standing-convention status by recurring across independent contexts, proving it is not a one-off. Counting one event's SDD paper trail (commit + its review verdict + its journal line) as three occurrences voids that mechanism — by the same logic every SDD task would auto-promote (brief + report + verdict + commit + journal = five "occurrences"). The only coherent reading of "occurrence" is "a distinct time the pattern arose," and the audit's explicit DROP reason *"a duplicate of another listed occurrence"* exists precisely to collapse this. Under that reading, two of the three collapse into the third.

## Disposition

- **Verdict:** REJECTED — fewer than 3 distinct occurrences survive.
- **verified_count:** 1.
- **Action:** do not promote to a standing convention. Demote to Tier 1 (a true, well-documented one-time lesson).
- **Not in dispute:** the pattern's correctness (`gen` is genuinely reserved in edition 2024) or the engineering judgment (root-cause rename over an edition downgrade that forks the workspace edition to preserve a module name nothing depends on). Truth is not the failing test; recurrence is. The lesson can stand at Tier 1 on its merits; it has simply not recurred.
