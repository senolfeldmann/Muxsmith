# Audit: cross-01-stack (PROMOTION candidate)

**Verdict: REDUCED** — one occurrence dropped, 3 distinct occurrences survive, promotion stands at the threshold (but on structurally weak evidence; see caveat).
**verified_count: 3** (surviving distinct occurrences; >=3 promotion threshold met).

## Candidate

- **id:** `cross-01-stack`
- **kind/domain:** pattern / cross, `status: settled`, `promoted: true`, `promoted_at: 3`, `count: 4`
- **statement:** "Build on Tauri 2 with a Rust core crate, a web frontend and a clap CLI; Rust accepted despite being only recently picked up. React was the initial frontend, later swapped to Vue (D27 / 2026-07-10 ...) - a within-frontend refinement, not a stack reversal, so the stack itself stays settled."

A promotion to standing house-knowledge, so the count must reflect genuine, distinct recurrence, not one decision multi-booked across the artifacts a single session inherently emits.

## Per-occurrence verification

All four refs were opened in `/home/senol/Git/Muxsmith`. Three are dated 2026-07-08 and one 2026-07-09; all originate from **session 1** (the 2026-07-09 handoff was, per its own provenance comment, written at session-1 close, `2026-07-08T22:01:36Z`, local 2026-07-09 00:01).

### Occ 1 — `spec §2 (Stack row)` — SURVIVES
`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, §2 Decision log. Original row as of commit 61249f9:

> `| Stack | Tauri 2 + Rust core crate + React/TS frontend + clap CLI | Mature, small bundles, best packaging, largest OSS contributor pool. Tradeoff: Rust learning curve for the core; webkitgtk quirks on Linux. |`

Supports "topic (stack) + approach (Tauri 2 + Rust core crate + web frontend + clap CLI) arose here as **decided**." A row in the design-log table, the authoritative decision record. Distinct artifact. **Kept.**

Nuance (evidence-blurb inaccuracy, not disqualifying): the occurrence's evidence string attributes the comparison "over Wails v3 (alpha risk) and Avalonia (delivery certainty, smaller OSS pull)" to this spec row. That comparison is **not** in the spec §2 row — the row's rationale is "Mature, small bundles, best packaging, largest OSS contributor pool." The Wails/Avalonia weighing is attested only by occ 2 (journal) and occ 4 (handoff). The row still records the stack decision as decided, so the occurrence survives; the borrowed rationale is a blurb defect, not a fabricated occurrence. (HEAD's version of this row now reads Vue, per the D27 2026-07-10 swap, consistent with the cluster's React→Vue framing.)

### Occ 2 — `journal 2026-07-08 (Plan 1, bullet 4)` — SURVIVES
`docs/process-journal.md`, session-1 entry (2026-07-08), "Decisions and why (not obvious from artifacts)", bullet 4 (lines 27-29):

> - Stack: Tauri 2 + Rust core + React/TS over Wails v3 (alpha risk) and Avalonia (delivery certainty, smaller OSS pull). Rust accepted although only recently picked up. MIT over Apache-2.0.

Verbatim match to the occurrence's evidence ("Rust accepted although only recently picked up") and the sole source of the Wails/Avalonia comparison and the Rust-recency clause that the cluster statement leans on. Distinct artifact (process journal, independently authored, carrying content absent from the spec row). **Kept.**

### Occ 3 — `commit 61249f9` — DROP (duplicate of occ 1 + misattributed)
`git show --stat 61249f9`:

- `docs: add Muxsmith v1 design spec`. Status `A`, **1 file changed, 376 insertions** — the sole change is adding `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, whose diff contains the exact §2 Stack row that IS occurrence 1.

Two independent grounds to drop:

1. **Duplicate of occ 1.** Commit 61249f9 is the version-control act that *creates occurrence 1's artifact*; it carries no decision payload beyond the spec row already counted. Counting both the spec row and the commit that introduces it counts one artifact twice. (Identical treatment to the sibling audit `audit-core-04`, which dropped this same commit as a duplicate of its spec row.)
2. **Misattributed / fabricated evidence.** The occurrence describes 61249f9 as "plan-1 implementation of the chosen stack." It is not an implementation of anything — it is the 376-line design-spec markdown, one file. The actual plan-1 implementation begins at `a7d550e` ("chore: scaffold cargo workspace (muxsmith-core, muxsmith-cli)") and continues through the `feat(core)`/`feat(cli)` commits. No stack-implementation code lives in 61249f9.

**Dropped.**

### Occ 4 — `handoff 2026-07-09-plan-1-close.md` — SURVIVES
`docs/process-journal/artifacts/handoffs/2026-07-09-plan-1-close.md`, "Decisions made (and why)" (line 38), with corroboration in the Objective (line 18):

> - Stack Tauri 2 + Rust core + React/TS, MIT license -> settled after alternatives (Wails v3 alpha, Avalonia) were weighed.
> ... (Objective) "Rust core + CLI now, Tauri 2 GUI later ..."

Ref supports "stack **decided**/settled, carried through plan-1 close." Not fabricated, not misattributed (it is the plan-1-close handoff and it does record the stack), and not a strict ref-duplicate of any other listed occurrence (a distinct hand-authored document, adding the "CLI now, Tauri GUI later" sequencing). By the operative drop mechanics of the sibling audits — drop mechanical VCS duplicates, keep distinct hand-authored records even when they co-record the same decision — it **survives**. **Kept.**

## Result and reasoning

- Surviving distinct occurrences: occ 1 (spec) + occ 2 (journal) + occ 4 (handoff) = **3**.
- Dropped: occ 3 (commit, duplicate of occ 1 + misattributed as implementation).
- 3 >= 3 -> **REDUCED** (one dropped, threshold still met). Promotion stands.

### Structural caveat (why this is a marginal pass)

The three survivors are **all session-1 co-records of a single stack decision**: the design spec (occ 1), the process journal (occ 2) and the session-close handoff (occ 4) are the three hand-authored artifacts one session inherently produces, plus occ 3 was the git commit that emits the spec — the same fabricated-recurrence shape `audit-core-04` was rejected for (a one-time decision presented as N-times recurring). Two aggravating points specific to this cluster:

- Occ 4's decision payload substantially **duplicates occ 2** (both record "Wails v3 alpha / Avalonia weighed" and Rust acceptance); it adds only the CLI-now/GUI-later sequencing. It is a distinct document, not a strict ref-duplicate, so the literal drop mechanics keep it — but it is a session-close recap of occ 2's decision, not an independent later reaffirmation.
- There is **no genuinely later re-decision or `applied` occurrence from a subsequent session** in this cluster. The one real later event touching the stack — the React→Vue swap (D27, 2026-07-10) — is (correctly) framed as a within-frontend refinement and is not even listed as an occurrence; it would not reaffirm the *stack* anyway.

So the count survives the threshold only because this cluster happens to list a fourth session-1 artifact (the handoff) where `audit-core-04` listed three. Under the stricter reading that "distinct occurrence" means "distinct decision event" (the philosophy the `audit-core-04` structural note gestures at), this cluster collapses to one decision and would be REJECTED. The verdict here follows the **operative** house mechanic actually applied by the sibling audits (drop only the mechanical VCS duplicate; keep distinct hand-authored documents), which yields 3 and REDUCED — but the promotion rests on co-records of a single decision, not on genuine recurrence, and should be treated as the weakest kind of pass.

**Action:** keep `promoted: true` (threshold met), but reset `count: 4 -> 3` and drop occ 3 from the occurrence list. Note in the ledger that the surviving evidence is three session-1 co-records of one decision, not three independent recurrences; if the promotion bar is later tightened to require an independent post-session-1 reaffirmation, this cluster should be revisited for demotion to Tier 1. The knowledge itself (Tauri 2 + Rust core + web frontend + clap CLI) is a real, settled design decision and remains true regardless of tier.
