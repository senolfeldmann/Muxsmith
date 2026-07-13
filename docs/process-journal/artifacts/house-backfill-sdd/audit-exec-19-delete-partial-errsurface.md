# Audit: exec-19-delete-partial-errsurface (PROMOTION candidate)

**Cluster kind:** pattern (domain: executor), status: settled, claimed count: 3, promoted: true (promoted_at 3)
**Statement under audit:** Surfacing a failed partial-output delete via `outcome.errors` was flagged and deferred twice at Plan-4 review (no JobOutcome error channel existed, and a failed remove leaves a broken partial that `on_collision:skip` would accept), then implemented in T5 as a `delete_partial_failed: <io error>` passthrough into `outcome.errors`; `NotFound` stays silently ignored.

**Verdict: CONFIRMED** — all 3 occurrences survive; verified_count = 3. Promotion to standing house-knowledge stands.

Every ref opens, names the exact topic, and carries the claimed `occ.kind`. Nothing is fabricated, nothing is misattributed to the wrong topic, and the two Plan-4 deferrals are genuinely distinct review artifacts (task-level review vs whole-branch review), not one restated. The final `decided` implementation is confirmed both in the review verdict and in the live source. Two honest caveats (a date-provenance mismatch, and the "deferred twice" being two granularities of the same issue in one plan) are recorded below; neither meets the drop bar.

---

## Per-occurrence verification

### Occurrence 1 — "plan-4 task-2-review-verdict.md" (kind: deferred) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-2-review-verdict.md`

- Under **Issues → Minor (Nice to Have)**, verbatim:
  `delete_partial ignores *all* IO errors, not just NotFound (job.rs:164-166). Brief said "ignoring NotFound"; this swallows permission/other errors too. On a failed remove (e.g. EACCES), a broken partial stays in the tree and a later on_collision=skip rerun accepts it as valid — the exact hazard delete-partial exists to prevent. Genuinely unreportable given JobOutcome has no channel, so defensible, but broader than the brief's letter.`
- This carries both halves of the cluster's "deferred" evidence exactly: (a) "a failed remove leaves a broken partial that on_collision:skip would accept" and (b) "no JobOutcome channel existed to report it" ("Genuinely unreportable given JobOutcome has no channel").
- Kind `deferred` is exact: it is filed as a Minor and explicitly left as "defensible, no action"; the round-2 verdict re-confirms it stands as "ledgered, no action" (`Minors — stand as previously assessed`).
- Topic genuinely arose here. Rock solid.

### Occurrence 2 — "plan-4 whole-branch-review-verdict.md (ledger #2)" (kind: deferred) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-4-sdd/verdicts/whole-branch-review-verdict.md`

- Under **Triage of the 13 ledgered findings**, item 2 verbatim:
  `2. v1.x backlog - failure mode needs remove_file to fail on a just-closed file (permissions/AV); fix direction exists (push a line into outcome.errors, which already carries the spawn-error string).`
- The cluster's quoted evidence (`'fix direction exists (push a line into outcome.errors, which already carries the spawn-error string).'`) is byte-faithful to this line.
- Kind `deferred` is exact: it is triaged **v1.x backlog** in a distinct whole-branch review pass (reviewer subagent `a44acd0735a4896d9`), separate from the task-2 review (subagent `a37edc817e9001305`). It re-decides the deferral at branch granularity and records the concrete fix direction that T5 later took.
- Not a strict duplicate of Occ 1: different reviewer, different review scope, substantive independent triage that adds the fix direction (the exact approach eventually implemented). The statement's "flagged and deferred twice at Plan-4 review" is a faithful description of these two artifacts.

### Occurrence 3 — "plan-5 T5 step1.4 / task-5-review-verdict.md" (kind: decided) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-5-sdd/verdicts/task-5-review-verdict.md`

- Under **Spec Compliance**, verbatim:
  `✅ delete_partial error surfacing matches the specified "delete_partial_failed: <io error>" format exactly (job.rs:204-211), NotFound still silently ignored.`
- The cluster's evidence quotes this line verbatim, including the `job.rs:204-211` anchor and "NotFound still silently ignored".
- Kind `decided` is accurate here (implementation landed): the deferred backlog item became a concrete T5 deliverable, format-specified and verified.
- **Independently corroborated in live source** (`crates/muxsmith-core/src/executor/job.rs:204-210`):
  ```rust
  fn delete_partial(output: &Path, errors: &mut Vec<String>) {
      if let Err(e) = std::fs::remove_file(output)
          && e.kind() != std::io::ErrorKind::NotFound
      {
          errors.push(format!("delete_partial_failed: {e}"));
      }
  }
  ```
  The `delete_partial_failed: <io error>` passthrough into `errors` (which becomes `outcome.errors`) and the silent `NotFound` skip are both present exactly as claimed. The rustdoc at job.rs:196-203 documents it as "the deliberate exception to core staying prose-free ... since core otherwise has no channel back to the caller" — i.e. the channel the two Plan-4 deferrals said did not yet exist was opened here. A dedicated test `delete_partial_failure_surfaces_into_errors` (job.rs:440) pins it.

---

## Distinctness / duplicate check

- Occ 1 vs Occ 2: **distinct.** Two different review artifacts, two different reviewer subagents, two granularities (task-2 vs whole-branch), each independently recording a deferral of the same underlying issue. Occ 2 adds the fix direction, so it is not a verbatim restatement of Occ 1. This is exactly the "deferred twice" the statement claims, not a citation of one file twice.
- Occ 3 vs Occ 1/2: **distinct.** Different session (`62503ddd...` vs `f6ee0efc...`), different plan (5 vs 4), and it is the implementation/decision endpoint rather than a deferral. Confirmed additionally by live code.

No occurrence is fabricated, misattributed to the wrong topic, or a strict duplicate. None triggers a drop.

---

## Honest caveats (recorded, below the drop bar)

1. **Date-provenance mismatch.** The cluster stamps all three occurrences `2026-07-11`. The salvaged verdict files carry `final_message_ts` of `2026-07-09` (both Plan-4 refs) and `2026-07-10` (Plan-5), and were salvaged `2026-07-10`. The topic unambiguously arose in each cited artifact; only the recorded dates are off by ~1-2 days. Under the audit's drop criteria (fabricated / misattributed-topic / duplicate) a date slip is not a drop trigger, so the count is unaffected. A future consumer should not treat the `2026-07-11` dates as precise.

2. **Nature of the count.** This is one issue tracked through its lifecycle within a single plan family: flagged+deferred at task-2 review (Occ 1), re-deferred with a fix direction at the Plan-4 whole-branch review (Occ 2), then implemented in Plan-5 T5 (Occ 3). It is a real recurrence across three genuinely distinct artifacts/reviewers (not one decision echoed), which is a legitimate profile for a **settled** pattern — but it is "one deferred concern, twice-deferred then resolved," not "this hazard kept independently re-surfacing in unrelated contexts." The evidence a standing convention needs (two independent deferrals with a stated channel-absence rationale, a format-specified implementation, a pinning test, and rustdoc documenting the deliberate exception) is all present and none fabricated.

**verified_count = 3 -> CONFIRMED.** Promotion stands.
