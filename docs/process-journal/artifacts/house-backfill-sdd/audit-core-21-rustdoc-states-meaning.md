# Audit: core-21-rustdoc-states-meaning (PROMOTION candidate)

**Cluster:** `core-21-rustdoc-states-meaning` (kind: pattern, domain: core, status: settled)
**Claimed count:** 3 | **promoted:** true
**Statement:** deny(missing_docs) enforces presence only; the quality bar is that rustdoc states meaning, contract and edge cases, not a name echo. Enforcement: five restatement-style docs from the 151-item backfill were caught by review and sharpened.

**Verdict: REJECTED** — verified_count = 2 (< 3). Demote to Tier 1; the promotion to a standing convention does not stand.

---

## Per-occurrence verification

### Occ 1 — `decided`, 2026-07-08, ref: "BUILDING.md Documentation standard (agreed Plan 1)" — SURVIVES

`BUILDING.md` §"Documentation standard" (lines 104-109) reads verbatim:

> - Rustdoc states MEANING, not a name echo: what the item is for, its contract, its edge cases. `#![deny(missing_docs)]` enforces presence only; this line carries the quality bar (agreed Plan 1, previously chat-only).

The `##Deliberately not used` header above dates the file to the "Plan-1 tooling stock-take, 2026-07-08". §"The Rust gate" (lines 80-82) reinforces the same distinction (presence vs correctness). The artifact records the standard **as a decision** ("agreed Plan 1"), exactly matching `occ.kind = decided`. Supports the topic and kind. **Confirmed.**

### Occ 2 — `violated-corrected`, 2026-07-08, ref: "journal Rustdoc (5 restatement docs)" — SURVIVES (as the single enforcement event)

`docs/process-journal.md` lines 70-71 (Plan-1 session summary):

> - Rustdoc: 151 public items backfilled in one pass + per-variant DiagCode docs via macro meta-forwarding; 5 restatement-style docs caught by review.

Records the 151-item backfill and the 5 restatement docs caught by review. Supports the statement's enforcement clause and `occ.kind = violated-corrected`. **Confirmed as an artifact — but see the duplicate finding below.**

### Occ 3 — `violated-corrected`, 2026-07-08, ref: "commit 9a7f49f" — DROP (duplicate of Occ 2)

`git show 9a7f49f`: *"docs: sharpen the five restatement rustdocs from review; correct Scalar ordering claim"* (Wed Jul 8 2026). The diff sharpens exactly five name-echo docs (cli.rs `command`; model.rs `description`, `Overwrite`, `Keep`, `Drop`) from restatement to meaning/contract, plus the separate Scalar ordering-claim correction. Considered on its own, the commit fully supports the statement and the kind.

**However, it is the same event as Occ 2, not a second one.** Both refs describe:
- the identical set — **5** restatement-style docs,
- caught by the **same** review,
- from the **same** 151-item backfill pass,
- on the **same** date (2026-07-08).

The journal line is the retrospective record of the work that commit 9a7f49f *is*. One enforcement event, recorded in two artifacts (a session-summary line and the commit that performed it). Per the audit rule, a ref that is "a duplicate of another listed occurrence" is dropped. Occ 3 is dropped as the duplicate; the single enforcement event is retained once via Occ 2. **Dropped (duplicate).**

---

## Count reconciliation

The claimed count of 3 is inflated. The pattern has, in reality, **two distinct occurrences**, both inside the same Plan-1 episode:

1. one **decision** (BUILDING.md, agreed Plan 1), and
2. one **enforcement** (5 restatement docs caught by review and sharpened) — double-listed as journal + commit 9a7f49f.

There is no *recurrence* here: the standard was agreed once and first-enforced once, in the same 2026-07-08 window. Listing the journal-record and its commit as separate occurrences manufactures a third that did not independently happen — precisely the fabricated-recurrence failure this audit exists to catch.

**verified_count = 2.** 2 < 3 → **REJECTED.** Demote `core-21-rustdoc-states-meaning` to Tier 1; do not admit it as a standing convention on a count of 3.
