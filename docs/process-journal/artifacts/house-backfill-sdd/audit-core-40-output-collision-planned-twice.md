# Audit: core-40-output-collision-planned-twice (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 3, promoted: true
**Statement under audit:** Two planned outputs colliding is always a hard `OutputCollision` error (drop both plans) regardless of `on_collision`; the policy (error / warn+drop / info+keep) governs only pre-existing on-disk files. Corrected an inline deviation that collapsed Overwrite into Error and made skip a no-op; spec 4.8 amended.

**Verdict: CONFIRMED** — all 3 occurrences survive; verified_count = 3. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no strict duplicate. The rule is real, deliberately decided by Şenol, implemented with parametrized tests, independently reviewed as correct+complete, and encoded in the spec. One honest caveat on the *nature* of the count is recorded at the end; it does not meet the drop bar.

---

## Per-occurrence verification

### Occurrence 1 — "independent review decision #3(c)" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`

- Line 33, under the header **"Decisions (Şenol, 2026-07-09)"**:
  `#3 -> (c): two-planned-output collision is always an error regardless of on_collision; the policy governs only pre-existing on-disk files; amend spec 4.8.`
  This is the verbatim rule, recorded as an explicit Şenol decision. Kind "decided" is exact.
- Corroborating within the same file:
  - Line 61 (Reviewer finding): `PLAUSIBLE planner.rs:511 - planned-twice collapses Overwrite into Error, undocumented deviation from 4.8. [-> decision #3(c)]` — the finding that motivated the decision.
  - Line 22 / Line 57: bug **E** (`on_collision: skip` maps to Warning but only Error drops a plan -> skip is a no-op) — the second half of the statement ("made skip a no-op").
- This is the origin decision. Rock solid.

### Occurrence 2 — "F6 review" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-2-fixes-sdd/F6-review.md`

- Independent review of commit `b5acada` titled *"output rendered-name invariant + collision semantics"*. Substantive, independent work: ran `cargo test --workspace`, `planner_resolution` (16 passed), and clippy itself, and even found a *new* regression (Finding 1) in the same function — so it is not a copy of Occurrence 1.
- Directly verifies the exact rule under "What was verified as correct":
  `detect_output_collisions: two-planned-outputs is now unconditionally Severity::Error regardless of on_collision (None, Error, Overwrite, Skip all tested in one parametrized test), and both colliding plans are dropped ... Matches spec 4.8's amended decision #3 exactly.`
  and confirms bug E is fixed: `on_disk_collision_under_skip_is_warning_and_drops_plan`.
- Topic unambiguously arose here in a real, distinct artifact.
- **Kind caveat (does not trigger drop):** this artifact is a *verification review*, not a decision. "decided" is imprecise; "reviewed" would be exact. It does not decide anything new — it confirms the already-made decision was implemented. Recorded, not dropped, because the topic genuinely and substantively arose here.

### Occurrence 3 — "commit b5acada" (2026-07-09, kind: decided) — SURVIVES

Ref: `git show b5acada` — `fix(core): planner rendered-name and collision severity per spec 4.8`, dated 2026-07-09 04:53:35 +0200.

- Commit message states the rule and the amendment verbatim:
  `detect_output_collisions: two plans rendering to the same path are now unconditionally OutputCollision at Error severity, independent of on_collision (decision #3, amended spec 4.8) ... skip now explicitly drops the plan (bug E: a Warning-severity diagnostic was previously a no-op since finalize_plans only drops Error severity).`
- Diff touches `planner.rs`, `report.rs`, and adds 171 lines of `planner_resolution.rs` tests. Real implementation artifact, distinct from the review that verifies it.
- **Kind caveat (does not trigger drop):** this is the *implementation* of the decision, not the decision itself. "decided" is loose ("implemented a decision"); "implemented" would be exact. Kept because the topic materially arose here in code + tests.

---

## Corroboration outside the three refs

Spec is amended as the statement claims (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, the §4.8 area):

- Line 227: `Two planned outputs rendering to the same path is ALWAYS an error (OutputCollision, error severity), independent of on_collision ...`
- Line 226: `on_collision ... Governs collisions with the FILESYSTEM only: a rendered output path that already exists as a pre-existing on-disk file ...`
- Line 267 (diagnostic catalog): `OutputCollision | error (two planned) / per policy (on-disk) | ...`

The rule is settled, spec-encoded, coded, and tested — exactly the profile a standing convention should have.

---

## Honest caveat on the *nature* of the count (recorded, below the drop bar)

All three occurrences are the **same single decision (#3 / #3(c)) carried through its own lifecycle on one date (2026-07-09)**: decision (Occ 1) -> commit implementing it (Occ 3) -> independent review of that commit (Occ 2). This is one design decision fully pipelined, not the pattern independently recurring in three different contexts. Two of the three kind labels are imprecise ("decided" for what are really a *review* and an *implementation*).

This is legitimate for a **settled** pattern: the evidence that the rule is real and load-bearing (deliberate decision + tests + independent review + spec amendment) is exactly what a standing convention needs, and none of it is fabricated. But a future consumer of this house-knowledge should read the "count: 3" as "one decision, fully realized and verified," not "this issue kept cropping up." Under the audit's drop criteria (fabricated / misattributed-topic / duplicate-of-another) none of the three qualifies, so the count is not reduced.

**verified_count = 3 -> CONFIRMED.**
