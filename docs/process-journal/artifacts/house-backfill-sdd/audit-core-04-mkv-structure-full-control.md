# Audit: core-04-mkv-structure-full-control (PROMOTION candidate)

**Verdict: REJECTED** — demote to Tier 1.
**verified_count: 2** (distinct surviving occurrences, below the >=3 promotion threshold).

## Candidate

- **id:** `core-04-mkv-structure-full-control`
- **kind/domain:** pattern / core, `status: settled`, `promoted: true`, `count: 3`
- **statement:** "v1 configures tracks, attachments, chapters, tags and title, accepting the larger surface deliberately over agent-recommended global toggles."

A promotion to standing house-knowledge, so the count must reflect genuine, distinct recurrence, not one decision triple-booked.

## Per-occurrence verification

All three occurrences are dated 2026-07-08, kind `decided`, and originate from the **same session 1**. Each ref was opened in `/home/senol/Git/Muxsmith`.

### Occ 1 — `spec §2 MKV-structure row` — SURVIVES
`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, §2 Decision log, line 22:

> `| MKV structure scope | Full control in v1 | Tracks, attachments, chapters, tags, title all configurable. More surface than tracks-only, accepted deliberately. |`

Supports "topic (MKV structure scope) + approach (full control over tracks/attachments/chapters/tags/title, larger surface accepted deliberately) arose here as **decided**." It is a row in the design-log table. Distinct artifact. **Kept.**

Nuance: the spec row justifies the surface as "more surface than tracks-only" and does **not** mention "agent-recommended global toggles." That specific clause of the statement is attested only by occ 2 (journal). The row still supports the core topic/approach as decided, so it survives.

### Occ 2 — `journal Plan 1` — SURVIVES
`docs/process-journal.md`, session-1 entry (2026-07-08), "Decisions and why", lines 25-26:

> - Full MKV-structure control in v1 (attachments/chapters/tags/title) against agent recommendation of global toggles; his scope call.

Supports the topic + approach as **decided** ("his scope call"), and is the sole source of the "against agent recommendation of global toggles" clause in the statement. Distinct artifact (a separate document, independently authored, adding detail not present in the spec row). **Kept.**

### Occ 3 — `commit 61249f9` — DROP (duplicate of occ 1)
`git show --name-status 61249f9`:

- Status `A` (added), **1 file changed, 376 insertions** — the sole change is adding `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`.
- Its diff contains the exact §2 row that IS occurrence 1 (`+| MKV structure scope | Full control in v1 | ... accepted deliberately. |`).

Commit 61249f9 is the version-control act that **creates occurrence 1's artifact**. It carries no independent decision content; the decision-relevant payload of the commit is identically occurrence 1's spec row. Counting both the spec row and the commit that introduces it is counting one artifact twice. **Dropped as a duplicate of occ 1.**

## Result and reasoning

- Distinct surviving occurrences: occ 1 (spec) + occ 2 (journal) = **2**.
- 2 < 3 -> **REJECTED**. Promotion does not stand; demote to Tier 1.

### Structural note (why this matters beyond the single drop)

The count of 3 was inflated by recording **one** session-1 scope decision across the three artifacts a single session inherently produces: the design spec (occ 1), the process journal (occ 2), and the git commit that adds the spec (occ 3). There is no independent later reaffirmation in this cluster — no "applied" or re-`decided` occurrence from a subsequent session (the actual implementation of the scope landed in Plan 3, which is not listed here). This is exactly the fabricated-recurrence shape a promotion audit exists to catch: a one-time decision presented as a thrice-recurring pattern. Even the two survivors are co-records of the same decision event; the strict drop mechanics already put the count below threshold, so the leniency toward occ 2 does not change the verdict.

**Action:** set `promoted: false`, revert `promoted_at`, and return the cluster to Tier 1. The knowledge itself (full MKV-structure control in v1) is a real, settled design decision — it is simply not backed by three genuine distinct occurrences and so must not become a standing convention on this evidence.
