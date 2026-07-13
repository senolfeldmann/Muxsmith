# Adversarial audit: `core-01-rule-uniqueness` (PROMOTION candidate)

- **Cluster id:** `core-01-rule-uniqueness`
- **Kind / domain:** pattern / core
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 3
- **Verdict:** **REJECTED** (surviving distinct occurrences: **1**, threshold for promotion is 3)
- **Action:** demote to Tier 1 (`decision-ledger.md`); do **not** stand up as a Tier-2 house convention on this evidence.

The statement itself is factually accurate and the rule is genuinely implemented
(spec §5.1 step 2, non-goals row on wildcard rules "breaks strict uniqueness",
the suggestion engine as the declared compensator). This audit does **not**
dispute the rule's correctness. It disputes the **recurrence count** that
authorizes promotion. The threshold-3 mechanism exists, by the ledger's own
words, "to avoid overfitting one-offs into the rulebook." This candidate is a
one-off dressed as a three-count.

---

## Counting unit (from the doctrine being audited)

`decision-ledger.md` / software-dev-process doctrine §7 defines the recurrence
unit explicitly: the controller "records a fresh consideration, or increments an
existing one, when **a subagent report or a session** surfaces it." The unit of
recurrence is therefore a *session* or a *distinct subagent report*, not a
document. Three documents produced within one session are one surfacing.

---

## Per-occurrence verification

### Occurrence #1 — `spec §2 rule-semantics row` (2026-07-08, decided)

- **Artifact:** `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, §2
  Decision log, row `Rule semantics | Strict independent uniqueness`:
  "Every rule must resolve to exactly one track regardless of rule order; all
  overlaps are errors. Most explicit; configs must spell out exclusions (e.g.
  `forced_track: false`); error quality is a first-class feature to compensate."
- **Supports the statement?** Yes, verbatim on every clause (independence, no
  order effects, overlaps-are-errors, explicit `forced_track: false` exclusions,
  error-quality compensation). Genuine, correctly attributed as `decided`.
- **Distinct?** **No — duplicate of #3.** This row was *introduced by* commit
  61249f9. `git show 61249f9` is a single-file, 376-insertion diff whose lines
  add exactly this §2 row (diff line 39) plus the §5.1 resolution semantics
  (diff line 250: "the candidate set is computed independently of all other
  rules (strict uniqueness, no consumption, no order effects)"). The spec-row
  and the commit are the same content-introduction event viewed two ways, not
  two independent decisions.
- **Result: DROP** (duplicate of #3).

### Occurrence #2 — `journal Plan 1` (2026-07-08, decided)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-08 | Spec + Plan 1 complete, repo live | session 1", under
  "Decisions and why": "Strict independent uniqueness over ordered consumption
  or a global solver: Şenol chose maximal explicitness knowing configs need
  explicit exclusions (forced_track: false); compensated by making error quality
  and the suggestion engine first-class. Ordered consumption would have made his
  own example work unmodified; he rejected the implicitness."
- **Supports the statement?** Yes, and it adds the rationale (rejected ordered
  consumption / global solver). Genuine, correctly attributed as `decided`.
- **Distinct?** **No.** This is the journal of **session 1** narrating the same
  decision made in that same session. Its own Scope line reads "Entire first
  session: requirements interview, spec, Plan 1 authoring ... Commits
  61249f9..97ae031" — i.e. it is the meta-record of the very session whose spec
  and root commit are occurrences #1 and #3. By the doctrine's counting unit,
  session 1 is a single surfacing; its journal entry is not a second, later
  session re-surfacing the topic.
- **Result: DROP** (same session-1 surfacing as #1/#3, not an independent
  recurrence).

### Occurrence #3 — `commit 61249f9` (2026-07-08, decided)

- **Artifact:** `git show 61249f9` — the repository's **root commit**
  ("docs: add Muxsmith v1 design spec"), a single-file diff adding
  `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (376 insertions).
  Introduces both the §2 rule-semantics row and the §5.1 resolution semantics.
- **Supports the statement?** Yes. Genuine, correctly attributed as `decided`.
- **Distinct?** It is the canonical artifact of the session-1 design decision.
- **Result: KEEP** as the single surviving representative of the decision event.

---

## Distinctness analysis

All three refs resolve to **one session (session 1, 2026-07-08)** and **one
decision**:

| # | ref | file | relation to the event |
|---|-----|------|-----------------------|
| 1 | spec §2 row | v1-design.md | content **created by** commit 61249f9 |
| 3 | commit 61249f9 | git (root commit) | the git event that wrote the spec |
| 2 | journal Plan 1 | process-journal.md | session 1's narrative of the same decision |

- #1 and #3 are the same content-introduction (root commit is spec-only; it *is*
  the §2 row's birth). Unambiguous duplicate.
- #2 is the same session's own record of that decision. Under §7's session/report
  counting unit, session 1 is one surfacing; the journal does not add a second.

There is **no** later-session or independent-subagent-report recurrence among the
cited occurrences (e.g. a Plan 3 resolution implementer re-affirming strict
uniqueness, a review re-validating it, an idiomacy-review touch). Every cited ref
is session 1.

**Surviving distinct occurrences: 1** (the session-1 design decision).

---

## Verdict

**REJECTED.** One genuine surfacing, not three. The count of 3 was manufactured
by counting the spec row, the root commit that wrote that spec row, and the
session journal narrating that session as three separate occurrences of what is a
single day-one decision. That is exactly the one-off overfit the threshold-3 rule
is meant to block.

**Recommended action:**

1. Demote `core-01-rule-uniqueness` from the promoted/Tier-2 set back to Tier 1
   (`decision-ledger.md`) with an honest `count: 1`, `last: 2026-07-08`,
   `outcome: accepted`.
2. Do not add it to `docs/CONVENTIONS.md` on this evidence.
3. It may legitimately re-accrue count toward promotion later from **independent
   surfacings** (a resolution/planner subagent report, a review, an idiomacy
   pass) — the rule is real and settled, so genuine recurrences are plausible;
   they simply have not been recorded yet. Promotion waits for real recurrence,
   not for three views of one commit.

Note for the backfill process generally: when scoring promotion counts, collapse
occurrences that share a session/commit lineage before counting. A spec section,
the commit that introduced it, and the journal entry for that session are one
event, not three.
