# Audit: core-47-with-severity-builder (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 3, promoted: true
**Statement under audit:** Add a rustdoc'd `Diagnostic::with_severity` builder and use it instead of mutating the public `severity` field directly (`Diagnostic::info` then `.severity = ...`).

**Verdict: CONFIRMED** — all 3 occurrences survive; verified_count = 3. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no duplicate under the framework's own occurrence-counting rule. The pattern is real and shipped: `Diagnostic::with_severity` exists in `crates/muxsmith-core/src/report/mod.rs:255`, is rustdoc'd (satisfies `#![deny(missing_docs)]`), and has a dedicated unit test. Three distinct cited artifacts of three distinct enumerated types (independent-review event, plan doc, per-task review verdict) each genuinely attest the topic. One honest caveat on the *nature* of the count is recorded at the end; it does not meet the drop bar.

---

## Per-occurrence verification

### Occurrence 1 — "fix plan F6" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `docs/superpowers/plans/2026-07-09-plan-2-fixes.md`, under the `### F6: planner output + collision` heading (line 40).

- Verbatim directive: `Prefer a `Diagnostic::with_severity` builder over mutating the public field.` Sits inside the `detect_output_collisions` bullet of the F6 task, so attribution to "fix plan F6" is exact.
- Artifact type = **plan doc**, an explicitly enumerated distinct attestation point in the cluster's occurrence-counting rule. Not fabricated (text is present), not misattributed (correct F6 section).
- **Weakest of the three** and the one an adversarial read must scrutinize: it is a purely forward-looking directive that transcribes Reviewer 2's nit #10 (Occurrence 2) into the fix plan — the controller read the review, then wrote the builder directive into F6. So Occurrences 1 and 2 are causally linked (find -> prescribed-fix), not two independent discoveries. Under the framework's rule this still counts (a plan doc is its own touchpoint, and the rule keeps distinct artifacts as genuine recurrence); it is not a strict "duplicate of another listed occurrence" — different document, different artifact type. Survives, with the caveat recorded below.

### Occurrence 2 — "independent review Reviewer 2 #10" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`, `## Reviewer 2` section, finding 10 (line 64).

- Verbatim: `10. CONFIRMED planner.rs:523 - Diagnostic::info then .severity mutated directly. [nit]`. This is the anti-pattern being flagged — the "violated" origin of the arc.
- Artifact type = **independent review event**, explicitly enumerated. Genuinely independent work: four fresh reviewer subagents against a real mkvmerge; Reviewer 2 found this at the exact source line (`planner.rs:523`). Correctly attributed to Reviewer 2, item #10.
- This is the single concrete site where the anti-pattern existed. Rock solid as the find-event.

### Occurrence 3 — "F6 review" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-2-fixes-sdd/F6-review.md`, "What was verified as correct" (lines 137-142).

- Verbatim: `Diagnostic::with_severity: correctly implemented, rustdoc'd (satisfies #![deny(missing_docs)]), unit-tested, and used to replace the prior direct mutation of the public severity field.` This confirms the correction landed.
- Artifact type = **per-task review verdict**, explicitly enumerated. This is a *different* review event from Occurrence 2: it is the independent review of the F6 fix commit `b5acada` (F6-review.md line 3), whereas Occurrence 2 reviewed the original Plan-2 implementation. The cluster rule explicitly keeps find-vs-fix as two distinct events, so this is not a double-citation of one review.
- Substantive and independent (ran `cargo test --workspace`, `planner_resolution`, clippy itself; separately found an unrelated regression, Finding 1). Topic unambiguously arose here.

---

## Duplicate / misattribution analysis (the drop bar)

The three refs are three separate files of three distinct enumerated artifact types:

| Occ | File | Type |
|---|---|---|
| 1 | `plans/2026-07-09-plan-2-fixes.md` | plan doc |
| 2 | `plan-2-review/independent-review-2026-07-09.md` | independent review event |
| 3 | `plan-2-fixes-sdd/F6-review.md` | per-task review verdict |

Tested against the cluster's "collapsed to one occurrence" clauses (`cluster-core.md` header):

- *"Multiple sections/bullets of the same document"* — no, three different documents.
- *"A co-cited commit-set implementing one work item"* — n/a, no commit is cited.
- *"A single review event cited via both its verdict file and a journal/progress mention"* — no; Occurrences 2 and 3 are two genuinely different review events (original-impl review vs. fix-commit review), which the rule's find-vs-fix clause keeps distinct.

None of the three qualifies as fabricated, misattributed-topic, or duplicate-of-another under the framework's own definition. Drop count = 0.

---

## Corroboration outside the three refs

- Shipped source `crates/muxsmith-core/src/report/mod.rs:255`: `pub fn with_severity(mut self, severity: Severity) -> Self`, rustdoc (lines 249-254) explaining exactly the "set through the builder chain rather than mutating the public `severity` field directly" rationale.
- `F6-report.md` section (c): documents replacing `let mut d = Diagnostic::info(...); ...; d.severity = severity;` with `.with_severity(severity)`, plus the unit test `with_severity_overrides_constructor_severity` and the workspace test-count bump (`muxsmith-core lib: 64 passed (was 63, +1 with_severity test)`).
- The fix commit `b5acada` genuinely implements this. It could have been a fourth occurrence; the list is, if anything, conservative in not citing it.

The pattern is settled, coded, rustdoc'd, and unit-tested — the profile a standing convention should have.

---

## Honest caveat on the *nature* of the count (recorded, below the drop bar)

All three occurrences trace to the **single anti-pattern instance at `planner.rs:523`** (the `OutputCollision` severity mutation), carried through one work item's lifecycle on one date (2026-07-09): review finds it (Occ 2) -> fix plan prescribes the builder (Occ 1) -> fix review confirms it (Occ 3). This is one issue fully pipelined, not the pattern independently recurring in three unrelated contexts. Occurrence 1 in particular is a plan directive derived from Occurrence 2's finding, so the "count: 3" is better read as "one anti-pattern instance: found -> prescribed -> verified" than "this kept cropping up."

This is legitimate for a **settled** pattern and matches the precedent set by the sibling audit `audit-core-40` (the same F6 work item, CONFIRMED at 3 on the same lifecycle logic). The evidence that the rule is real and load-bearing — an independent reviewer flagging the direct field mutation, a plan directive, an independent fix-review, plus a shipped rustdoc'd builder with a unit test — is exactly what a standing convention needs, and none of it is fabricated. Under the audit's drop criteria (fabricated / misattributed-topic / duplicate-of-another) none of the three qualifies, so the count is not reduced.

**verified_count = 3 -> CONFIRMED.**
