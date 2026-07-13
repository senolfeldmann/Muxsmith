# Audit: core-24-diagcode-key-integrity (PROMOTION candidate)

- **Cluster id:** `core-24-diagcode-key-integrity`
- **Kind / domain:** pattern / core
- **Claimed count:** 3 (all `violated-corrected`, all 2026-07-08)
- **Claimed status:** settled, promoted (`promoted_at: 3`)
- **Statement under audit:** "The hand-authored `.key()` literals and serde's kebab rename were two independent encodings linked only by hand; T2 flagged the divergence risk, corrected with `DiagCode::ALL` plus exhaustive consistency and uniqueness tests."

## Verdict: REJECTED

**verified_count = 1** distinct occurrence. The claimed count of 3 is inflated: the three listed refs are three *artifacts of a single corrective episode* (flag -> fix -> narration), not three independent times the pattern arose. Under the audit's duplicate rule, two of the three collapse into the third. A single fix demoted below the promotion threshold -> **demote to Tier 1.**

## Method

Opened every cited ref in `/home/senol/Git/Muxsmith` and confirmed against the artifact whether the topic genuinely arose there as a `violated-corrected` event, then tested each pair for being the *same* episode (duplicate) rather than an independent recurrence.

Note on locations: the DiagCode work is Plan 1 (Jul 8), not the current plan. The live `.superpowers/sdd/task-2-*.md` files are a *different* task-2 (hoisting report JSON into core) and are irrelevant here. The Plan-1 task-2 review verdict survives only in the salvaged archive `docs/process-journal/artifacts/plan-1-sdd/verdicts/`.

## Per-occurrence findings

### Occurrence 1 - `task-2 review verdict Important #1` -> SUPPORTS (kept as the one distinct occurrence)

File: `docs/process-journal/artifacts/plan-1-sdd/verdicts/task-2-review-verdict.md`, Issues > Important #1 (round 1, `final_message_ts 2026-07-07T23:02:39Z`).

Text matches the statement precisely: "The 'single kebab-case catalog contract' is actually two independently-produced encodings, not one. `.key()` returns a hand-authored `$key` literal... the JSON `code` field is produced separately by serde's automatic `rename_all`... Nothing in the type system ties these together... no test checks that `.key()` and the JSON output ever agree on the same variant." This is the exact locus where the divergence risk was recognized and articulated. Genuine, on-topic, correctly attributed to T2. This is the primary (and only) distinct occurrence.

### Occurrence 2 - `journal` -> DUPLICATE of Occurrence 1 (dropped)

File: `docs/process-journal.md`, line 46 (session-1 entry, 2026-07-08): "key()/serde kebab encodings unlinked (task review T2) -> DiagCode::ALL + exhaustive consistency tests."

On-topic and not fabricated - but it is a one-line *narration of Occurrence 1*, not an independent event. It explicitly back-references "(task review T2)", i.e. it points at the very same review finding rather than recording a second, separate time the pattern was violated and corrected. It is the journal's summary of the same episode. Dropped as a duplicate of Occurrence 1.

### Occurrence 3 - `commit a7c0d89` -> DUPLICATE of Occurrence 1 (dropped)

`git show a7c0d89` ("test(core): enforce DiagCode key/serde consistency and uniqueness", Jul 8 01:04): adds `DiagCode::ALL` to the `diag_codes!` macro and the two tests `all_keys_match_serde_encoding` / `all_keys_are_unique`.

On-topic and real - but this commit *is the "corrected" half* of the single `violated-corrected` episode that Occurrence 1 already represents in full. The task-2 round-2 verdict (`task-2-review-verdict-round-2.md`) confirms the linkage: "The previously flagged Important finding is resolved" - same finding, same fix. A commit that resolves a review finding is the corrective action of that finding, not a separate occurrence of the pattern arising. Dropped as a duplicate of Occurrence 1.

## Why the count is inflated (the core defect)

A single `violated-corrected` episode *always* leaves this exact trail: a review verdict that flags it, a journal line that narrates it, and a commit that fixes it. Counting those three artifacts as three occurrences would let literally any one-off fix clear a "recurred 3 times" bar, which is precisely the fabricated-recurrence failure this audit exists to catch. The pattern here arose exactly once - in the Plan-1 DiagCode-catalog task (T2) - and was corrected once. The related Plan-1 delta (T13 completeness guard later switching to `DiagCode::ALL` iteration, journal lines 95-96) is the same fix propagating within the same session, is not one of the listed occurrences, and does not add a distinct occurrence either.

## Result

| Occurrence | Ref | On-topic? | Distinct occurrence? | Disposition |
|---|---|---|---|---|
| 1 | task-2 verdict, Important #1 | yes | yes (primary) | KEEP |
| 2 | journal line 46 | yes | no - narrates #1 | DROP (duplicate) |
| 3 | commit a7c0d89 | yes | no - fix of #1 | DROP (duplicate) |

- **verified_count:** 1
- **Threshold for promotion:** >= 3 distinct occurrences
- **Verdict:** REJECTED - demote `core-24-diagcode-key-integrity` from promoted house-knowledge to Tier 1. The observation itself is true and worth keeping as a single-incident Tier-1 note; it has simply not recurred enough to become a standing convention.
