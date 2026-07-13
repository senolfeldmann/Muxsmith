# Audit: ci-06-per-commit-gate (PROMOTION candidate)

- **Cluster:** `ci-06-per-commit-gate` (pattern, ci, settled)
- **Statement:** Standing house rule - `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo deny check` must all pass before every commit, never skipped. The CI test job caught fmt-dirty commits when tasks 3 and 5 were pushed after clippy but not fmt --check (run at 0e64c1e failed, fixed 72c59d2); a controller-discipline gap, not a plan gap.
- **Claimed count:** 3. Promotion asserts >=3 genuine, distinct occurrences.
- **Verdict: CONFIRMED** - 3 of 3 occurrences survive. Promotion stands.

The three refs point to three distinct work products: one concrete *violated-corrected* event (a real CI failure in the Plan 2 session that caught a discipline gap and was fixed by a named rustfmt commit) and two *reinforced* events (the gate carried forward as a binding Global Constraint into the Plan 3 and Plan 4 plan docs, each mirrored in its own progress ledger). No fabrication, no misattribution, no duplicate. The count is not padded by re-citing one artifact.

---

## Occ 1 - `journal 2026-07-09 Plan 2 'What the process caught' + commit 72c59d2` - kind: violated-corrected - SURVIVES

- **Artifacts (both cited, both present):**
  - `docs/process-journal.md`, entry **## 2026-07-09 | Plan 2 written and implemented** (l.115), **What the process caught** section (l.146), bullet l.147-150: "CI (test job) caught fmt-dirty commits: tasks 3 and 5 were pushed after running clippy but not `cargo fmt --check`; the intermediate run at 0e64c1e failed only on `cargo fmt --all --check`. Fixed by rustfmt commit 72c59d2. Origin: controller discipline gap, not the plan."
  - Commit **72c59d2** ("style: apply rustfmt to Plan 2 core modules"), message body: "earlier task commits ran clippy but not fmt --check. No behavior change." Touches 7 core/test files - a pure formatting catch-up, exactly the described fix.
- **Supports statement?** Yes, verbatim on every load-bearing element: the *test* CI job caught it, the violation was clippy-run-but-fmt-skipped on tasks 3 and 5, the failing point was 0e64c1e, the fix was 72c59d2, and the origin is classified as a controller-discipline gap ("not the plan"). This is the statement's core narrative, sourced directly.
- **Independent corroboration (not required, strengthens):** `gh-log.md` records concrete CI run IDs - l.91: "run 28982463732 at e1bfba7 -> both jobs success. The prior run 28981826696 at 0e64c1e failed only on `cargo fmt --all --check`"; l.84 repeats "tasks 3 and 5 were pushed fmt-dirty". Two independent contemporaneous records agree, and the failing run has a real GitHub run ID. Chronology holds: 0e64c1e committed 01:04, 72c59d2 committed 01:19 the same night.
- **Kind correct?** Yes. `violated-corrected` is exact: the gate was violated (fmt not run before push), the gate machinery (CI test job) caught it, and it was corrected by a named commit. This is the one occurrence that demonstrates the rule catching a real defect, which is what earns a per-commit gate its standing.
- **Distinct?** Yes. A dated, concrete Plan 2 incident with a run ID and a fix commit - not a restatement of the rule text. Shares nothing with Occ 2/3 beyond the subject rule.

## Occ 2 - `plan Global Constraints + progress.md (Plan 3)` - kind: reinforced - SURVIVES

- **Artifacts (both cited, both present):**
  - `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md`, **## Global Constraints** l.15 (verbatim): "**Per-commit gate, never skipped:** `cargo test --workspace` AND `cargo fmt --all --check` AND `cargo clippy --workspace --all-targets -- -D warnings` AND `cargo deny check` all pass before each commit." (Reiterated as a final-HEAD full gate at l.614.)
  - Plan 3 progress ledger, l.6: "Per-task gate: cargo test --workspace && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check." Per-task green recorded throughout (e.g. l.12, l.41).
- **Provenance nuance (documented, not a defect):** the live `.superpowers/sdd/progress.md` is a rolling, replace-in-place file and now holds Plan 5 content (its own l.8 notes "Plan-4 ledger archived as progress-plan4.md"). The Plan 3 ledger the ref names is the archived snapshot at `docs/process-journal/artifacts/plan-3-sdd/progress.md`. The content the ref asserts exists there verbatim - the rolling-file overwrite is expected behavior, not a broken citation.
- **Supports statement?** Yes. Both artifacts state the four-command per-commit gate as a binding, never-skipped constraint for the whole plan. Matches the statement's rule text exactly.
- **Kind correct?** Yes. `reinforced` - the standing gate carried forward and re-bound at plan level, not a fresh derivation and not a violation event.
- **Distinct?** Yes. Separate plan (Plan 3), separate date frame from the Plan 2 incident, separate artifacts and ledger. Not a duplicate of Occ 1 (incident) or Occ 3 (different plan/ledger).

## Occ 3 - `plan Global Constraints + progress ledger (Plan 4)` - kind: reinforced - SURVIVES

- **Artifacts (both cited, both present):**
  - `docs/superpowers/plans/2026-07-09-plan-4-executor-run-queue.md`, **## Global Constraints** l.13 (verbatim): "Per-commit gate, run all four, do NOT skip fmt: `cargo test --workspace` AND `cargo fmt --all --check` AND `cargo clippy --workspace --all-targets -- -D warnings` AND `cargo deny check`." The explicit "do NOT skip fmt" is a direct response to the Occ 1 incident.
  - Plan 4 progress ledger `.superpowers/sdd/progress-plan4.md`, l.6 "full gate re-run per merge" plus "controller gate re-run green" recorded per task/merge throughout (l.16, l.18-25).
- **Supports statement?** Yes. Same four-command gate, restated as a binding Plan 4 constraint and evidenced as actually run per merge in the ledger.
- **Kind correct?** Yes. `reinforced` - standing gate re-bound in a later plan; the "do NOT skip fmt" wording shows it is a deliberate carry-forward hardened by the earlier catch.
- **Distinct?** Yes. Plan 4 (dated 2026-07-10 by the cluster), distinct plan doc and distinct progress ledger from Plan 3. Same *kind* of evidence as Occ 2 (a standing-constraint restatement), but a genuinely separate plan context - not the same event cited twice.

---

## Skeptical cross-checks performed

- **Fabrication:** none. Every cited artifact exists and contains the asserted text; commit 72c59d2 and the failing/passing CI run IDs are real.
- **Misattribution:** none. Occ 1's "tasks 3 and 5", run 0e64c1e, and fix 72c59d2 match across journal, commit, and gh-log. Occ 2/3's Global Constraints text matches the named plan docs verbatim.
- **Duplication:** none among the three. Occ 1 is a concrete incident; Occ 2 and Occ 3 are separate-plan reinforcements. Occ 2 and Occ 3 are the same *type* of evidence (plan-doc constraint + ledger), which makes them weaker as independent "recurrence" than a fresh re-derivation would be - but they are distinct plans on distinct dates, not a self-citation, and the promotion is anchored by a real violated-corrected event (Occ 1). Distinctness holds.

## Summary

| # | Ref | Kind | Artifact(s) | Result |
|---|-----|------|-------------|--------|
| 1 | journal Plan 2 'What the process caught' + 72c59d2 | violated-corrected | process-journal.md l.146-150 + commit 72c59d2 (corrob. gh-log l.84, l.91) | SURVIVES |
| 2 | plan Global Constraints + progress.md (Plan 3) | reinforced | plan-3 plan doc §Global Constraints l.15 + plan-3-sdd/progress.md l.6 | SURVIVES |
| 3 | plan Global Constraints + progress ledger (Plan 4) | reinforced | plan-4 plan doc §Global Constraints l.13 + progress-plan4.md l.6 | SURVIVES |

**verified_count = 3** distinct surviving occurrences (1 violated-corrected + 2 reinforced).

**Verdict: CONFIRMED.** All three occurrences are real, correctly attributed, correctly typed, and mutually distinct. The count of 3 is genuine and includes one demonstrated real-world catch. Promotion to standing house convention stands.
