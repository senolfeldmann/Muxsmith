# Audit: cli-02-dryrun-config-diags (PROMOTION candidate)

**Cluster kind:** pattern (domain: cli), status: settled, claimed count: 4, promoted: true, promoted_at: 3
**Statement under audit:** Dry-run must run `profile::validate` + `lint::provable_overlaps` and fold config-time diagnostics in on every code path (happy, mkvmerge-not-found, mkvmerge-query-failed) and both renderers (human + JSON), per spec 5.5. Caught and enforced repeatedly across eras as the same defect class — bug A (dry-run never ran validate/lint), the not-found branch dropping them, the query-failed path deferred, then the query-failed human path fixed. Resolved by E7.

**Verdict: CONFIRMED** — all 4 occurrences survive; verified_count = 4. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no strict duplicate. Each ref points at a distinct physical artifact, each artifact substantively supports its occurrence at the claimed kind, and the four are genuinely distinct events across three code paths and two eras — not one decision pipelined. This is a stronger promotion case than the lifecycle-style clusters (cf. `audit-core-85`): the defect class demonstrably recurred, was caught each time, and the last surfacing was still being closed at E7 (task 9), which is exactly what "settled convention, real count" should look like.

Drop criteria applied (per sibling precedent `audit-core-40` / `audit-core-85`): the only triggers are fabricated / misattributed-topic / duplicate-of-another. None fires here.

---

## Base check (is the pattern real at all?)

Yes. The rule is code-grounded and spec-anchored:

- Commit `b507f6e` message: "Per spec 5.5, dry-run must be a strict superset of validate: run the config-time pass first, fold its diagnostics into the exit-code computation and the human/JSON output, then plan as before (no fail-fast)."
- `crates/muxsmith-cli/src/commands/dry_run.rs` carries `validate` + `provable_overlaps` calls threaded into `exit_code()` and both renderers; `config_only_json(...)` (added by `09d7244`) handles the not-found branch; the E7 fix adds the config-diags-first print inside the query-failed human else-branch.
- Spec 5.5 ("dry-run must stay a strict superset of validate") is cited unconditionally by every fix commit and the T9 verdict.

The rule exists, is specified, and is enforced in code. Not a fabrication at the base.

---

## Per-occurrence verification

### Occurrence 1 — "2026-07-09, bug A" (kind: violated-corrected) — SURVIVES

Refs: `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md` + commit `b507f6e`.

- Independent-review line 18 (controller triage): *"A. dry-run never runs validate()/lint -> config-time diagnostics unreachable; broken regex returns empty + exit 0; typo'd property -> misleading MissingTrack."* Cluster evidence quotes this verbatim. Two independent reviewers (Reviewer 2 #2 dry_run.rs:55; Reviewer 4 #1 dry_run.rs:23-29) found the same headline bug — the review's own framing: *"Two reviewers independently found the same headline bug (dry-run bypasses validate)."*
- Commit `b507f6e` ("fix(cli): dry-run runs config-time validate and renders JSON diagnostics") is the corresponding fix for the **main/happy path**. violated-corrected is accurate: found (review) then corrected (commit).
- Topic exact, kind exact. SURVIVES.

### Occurrence 2 — "E2 fix-pass F1, not-found branch" (kind: violated-corrected) — SURVIVES

Refs: `docs/process-journal.md` §Open-threads/what-caught (line 187) + `docs/process-journal/artifacts/plan-2-fixes-sdd/progress.md` ledger F1 (line 21) + commit `09d7244`.

- Journal line 187 verbatim: *"F1 (dry-run/validate): reviewer FAILED spec on the mkvmerge-not-found path silently dropping config diagnostics — the implementer had explicitly waved it off as a judgment call. Fixed; the fixer then found the branch WAS testable (PATH override)."* Matches the cluster evidence word for word.
- The "waved it off" claim is corroborated at the source: `F1-report.md` design-decisions section says the implementer kept the narrower change on the not-found path ("Flagging this as the one open dimension rather than silently picking it") — i.e. explicitly deferred it as a judgment call, exactly what the reviewer then failed.
- progress.md ledger line 21: *"Task F1: complete (commits d9422b3..09d7244, independent review found + fixed a real spec-5.5 gap: mkvmerge-not-found path dropped config diagnostics)."*
- Commit `09d7244` ("fix(cli): surface config diagnostics when mkvmerge is missing in dry-run") adds `config_only_json(...)` and the `mkvmerge_found: false` JSON envelope to the not-found early return, plus the PATH-override test.
- **Distinctness from Occ 1 verified:** `b507f6e` is a git ancestor of `09d7244` (checked via `git merge-base --is-ancestor`); `b507f6e` fixed the main path and the F1-report documents it explicitly *not* touching the not-found branch, which `09d7244` then fixed. Two commits, two code paths, two events. Not a duplicate.
- Topic exact, kind exact. SURVIVES.

### Occurrence 3 — "E2 fix-pass, query-failed deferred" (kind: deferred) — SURVIVES

Refs: `docs/process-journal/artifacts/plan-2-fixes-sdd/progress.md` ledger F1 residual (line 22) + `docs/process-journal.md` Open-threads (line 200).

- progress.md line 22 verbatim: *"Residual (Minor, for final whole-branch review): the mkvmerge-query-failed path (list_languages fails) has the same defect — config diags dropped — left out of F1 scope."* Matches cluster evidence word for word.
- Journal line 200 (§Open threads of the Plan-2-fixes entry): *"the mkvmerge-query-failed path still drops config diags (same class as the F1 fix, logged in the ledger)."*
- kind "deferred" is exact: the query-failed path was consciously left out of F1 scope and routed to the final whole-branch review. This is a third surfacing of the same defect class (a **third** code branch, distinct from happy and not-found), honestly labeled as deferred rather than corrected.
- **Distinctness from Occ 4 verified:** this is the E2 *deferral* of the query-failed path (different artifact — the F1 ledger residual + journal open-threads; different era — E2; different kind — deferred). Occ 4 is the E7 *resolution*. A defect surfacing-and-deferred and later resolving are two lifecycle events, not one artifact double-counted.
- Topic exact, kind exact. SURVIVES.

### Occurrence 4 — "E7, query-failed human path fixed" (kind: violated-corrected) — SURVIVES

Ref: `.superpowers/sdd/plan-5.5/task-9-verdict.md` point (vii) (also mirrored at `docs/process-journal/artifacts/plan-5.5-sdd/task-9-verdict.md`).

- T9 verdict (vii) verbatim: *"NOT-deliberate determination well-evidenced (F1 commits were --json-scoped; spec 5.5 unconditional; sibling prints already), fix strictly inside the human else-branch (JSON untouched)."* Cluster evidence — "Query-failed human path found NOT deliberate (F1 commits were --json-scoped, spec 5.5 unconditional); fixed to print config diagnostics first inside the human else-branch, JSON untouched." — matches point (vii) exactly.
- This is the E7 resolution of what Occ 3 deferred: the query-failed **human** path (JSON was already handled by the F1-era `--json` work, which is why the fix is scoped to the human else-branch). violated-corrected is accurate.
- Verdict is a genuine independent review (opus, 2026-07-11/12), source-verified, "Spec compliance ✅, Approved." Substantive distinct artifact.
- Topic exact, kind exact. SURVIVES.

---

## Distinctness matrix (the crux of an adversarial audit)

| Occ | Code path | Event | Kind | Era | Primary artifact |
|-----|-----------|-------|------|-----|------------------|
| 1 | happy / default | original discovery + fix | violated-corrected | 2026-07-09 (review) | independent-review + `b507f6e` |
| 2 | mkvmerge-not-found | fix-pass reviewer catch + fix | violated-corrected | E2 (F1) | journal L187 + ledger L21 + `09d7244` |
| 3 | mkvmerge-query-failed | deferred out of F1 scope | deferred | E2 (F1 residual) | ledger L22 + journal L200 |
| 4 | mkvmerge-query-failed (human) | E7 resolution | violated-corrected | E7 (task 9) | task-9-verdict (vii) |

No two occurrences share both artifact and event. Occ 1↔2 are separate commits on separate branches (ancestry-verified). Occ 3↔4 both concern the query-failed path but are the deferral vs the resolution — different eras, artifacts, and kinds. This is a genuinely recurring defect class across three code paths, which is precisely what a promoted "settled pattern" count should represent.

---

## Note on the promotion mechanics

`promoted_at: 3` with `count: 4`: the fourth occurrence (E7 resolution) post-dates the promotion threshold, which is consistent — the pattern was already promotion-worthy at three surfacings and the E7 fix is the closing event, not padding. The count is real: unlike lifecycle-style clusters where "3" means one decision specified/planned/reviewed, here "4" means the same defect class was genuinely caught on four distinct occasions across three code paths. Read as a standing convention, "dry-run folds config diagnostics on every path and renderer" is well-earned.

**verified_count = 4 -> CONFIRMED.**
