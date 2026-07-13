# Adversarial audit: `core-67-no-mkvtoolnix-magic-guesses` (PROMOTION candidate)

- **Cluster id:** `core-67-no-mkvtoolnix-magic-guesses`
- **Kind / domain:** restraint / core
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 4
- **Verdict:** **REJECTED** (surviving distinct occurrences: **1**, threshold for promotion is 3)
- **Action:** demote to Tier 1 (`decision-ledger.md`); do **not** stand up as a Tier-2 house convention on this evidence.

The restraint itself is correct, well-argued, and genuinely settled: `docs/IDEAS.md`
#1-4 carry Şenol's explicit "hard no" rulings on filename-derived language/flags,
`unique`-suffixing, and sequence auto-append, with the declarative-batch rationale
and the mkvtoolnix-GUI steelman intact. This audit does **not** dispute the rule's
correctness. It disputes the **recurrence count** that authorizes promotion. Per the
ledger's own words the threshold-3 mechanism exists "to avoid overfitting one-offs
into the rulebook"; per doctrine §7 the counter must reflect real recurrences, not
projections of one decision. This candidate is a single Plan-3.5 decision counted
four times.

---

## Counting unit (from the doctrine being audited)

software-dev-process doctrine §7: an occurrence is a distinct time a
consideration-and-outcome "surfaces" (a *session* or a *distinct subagent report*),
and the three artifact families are explicitly framed as projections of one event:
**"D-memos are the append-only event log; CONVENTIONS.yaml is the materialized
current-state view; the ledger is an aggregate view (recurrence). Same decision
events, three projections, no redundancy."** The counter grows *past* promotion only
via `violated-corrected` occurrences (agents fighting a settled rule), never via the
promotion write itself.

This matches the precedent already set by the audited sibling
`audit-core-01-rule-uniqueness.md`: "The unit of recurrence is a session or a distinct
subagent report, not a document. Three documents produced within one session are one
surfacing."

---

## Per-occurrence verification

### Occurrence #1 — `IDEAS.md #1-4` (2026-07-09, decided)

- **Artifact:** `docs/IDEAS.md`, items 1-4: language-from-filename (#1),
  flags-from-filename/track-name (#2), `unique` auto-suffix collision policy (#3),
  CD1/CD2 sequence auto-append (#4). Each states the mkvtoolnix default, the
  declarative-batch objection ("the same guess would fire unseen across hundreds of
  files with no review step"), and Şenol's ruling ("hard no" / "same hard no" /
  "Fail loud instead"; #4's design note names auto-append "the same magic-guess
  pattern Muxsmith rejects (ideas 1-2)").
- **Supports the statement?** Yes, on the core clauses. Genuine, correctly attributed
  as `decided`. (Fidelity nit, not a drop: the statement also lists **auto-title** as
  covered by "IDEAS.md #1-4" — auto-title is *not* among #1-4; it lives in IDEAS #6,
  "Output naming from title ... genuine naming divergence, never tiered." The
  auto-title claim is over-attributed but does not affect the count.)
- **Distinct?** This is the canonical record of the Plan-3.5 shelving decision.
- **Result: KEEP** as the single surviving representative of the decision event.

### Occurrence #2 — `commit b04c4a2` (2026-07-09, decided)

- **Artifact:** `git show b04c4a2` — "docs: Plan 3.5 (mkvtoolnix parity) design memo,
  plan, and ideas list." The diff **creates `docs/IDEAS.md` new** (`new file mode`,
  +134 lines) containing exactly items 1-4. Its entire contribution to this topic is
  writing occurrence #1's file.
- **Supports the statement?** Yes. Genuine, correctly attributed as `decided`.
- **Distinct?** **No — duplicate of #1.** The commit *is* the birth of IDEAS.md #1-4;
  the file-state and the commit that wrote it are the same content-introduction event
  viewed two ways, same session, same day.
- **Result: DROP** (duplicate of #1).

### Occurrence #3 — `journal Plan 3.5 (four hard-nos)` (2026-07-09, decided)

- **Artifact:** `docs/process-journal.md`, entry "## 2026-07-09 | Plan 3.5 complete
  (mkvtoolnix parity fixes) | session 4", decision line 238: "input-time convenience
  guesses (filename-derived lang/flags, auto-title, unique-name suffix, sequence
  append) are not [parity targets], **shelved to docs/IDEAS.md**."
- **Supports the statement?** Yes; it also records the SI-3 framing rule. Genuine,
  correctly attributed as `decided`. (The journal does not literally enumerate "four
  hard-nos"; it lists the four guess classes and points to IDEAS.md — same substance.)
- **Distinct?** **No.** This is session 4's own journal narrating the very decision
  made in that session — it explicitly points at occurrence #1's file as the shelving
  destination. It is the history projection of the same event, not a later independent
  re-surfacing. Under §7's session/report counting unit, session 4 is one surfacing.
- **Result: DROP** (same Plan-3.5 / session-4 surfacing as #1/#2).

### Occurrence #4 — `CONVENTIONS.md Restraints` (2026-07-12, reinforced)

- **Artifact:** `docs/CONVENTIONS.md` Restraints, "No mkvtoolnix input-convenience
  guesses ... (docs/IDEAS.md 1-4)." Introduced by `git show b38a46f` (2026-07-12),
  "docs: house-knowledge instance - CONVENTIONS.md + decision-ledger (Tier 1)" — the
  commit that **created CONVENTIONS.md and decision-ledger.md together**, i.e. the
  promotion itself.
- **Supports the statement?** The text matches the restraint, but the ref is the
  **promotion write**, not an independent reinforcement. `git log -S` confirms the
  restraint text enters git only at this promotion commit (plus journal-recovery
  commits); the 2026-07-12 idiomacy review (session 10, journal) shows **no**
  re-litigation, violation-correction, or fresh finding on this restraint.
- **Distinct?** **No.** CONVENTIONS.md is the doctrine's "materialized current-state
  view" — a projection of the decision, not a decision event. A `reinforced`
  occurrence citing the very Tier-2 entry being promoted is circular: the promotion
  cannot be its own evidence. Per §7 the counter grows past promotion only via
  `violated-corrected` occurrences, which this is not.
- **Result: DROP** (the promotion projection, not a recurrence).

---

## Distinctness analysis

| # | ref | file | relation to the event |
|---|-----|------|-----------------------|
| 1 | IDEAS.md #1-4 | docs/IDEAS.md | canonical record of the Plan-3.5 decision |
| 2 | commit b04c4a2 | git | the commit that **created** IDEAS.md #1-4 |
| 3 | journal Plan 3.5 | process-journal.md | session 4's narrative of the same decision |
| 4 | CONVENTIONS.md restraint | docs/CONVENTIONS.md | the promotion write (materialized view) |

- #1, #2, #3 collapse to **one surfacing**: session 4 / Plan 3.5 / 2026-07-09. #2 is
  the birth commit of #1's file; #3 is the same session's journal of the same
  decision (it names IDEAS.md as the shelving target). "Same decision events, three
  projections, no redundancy" (§7).
- #4 is the 2026-07-12 promotion commit's materialized-view entry, not an independent
  reinforcement (git and the idiomacy-review journal confirm no fresh surfacing).

There is **no** later-session or independent-subagent-report recurrence among the
cited occurrences (no reviewer flagging a filename-guess creeping in, no implementer
re-proposing derivation and being rejected, no idiomacy-review `house` finding).

**Surviving distinct occurrences: 1** (the Plan-3.5 design decision).

Even under the most lenient reading that credits #4 as a genuine second occasion, the
total is **2** — still below the promotion threshold of 3. The verdict does not turn
on the treatment of #4.

---

## Verdict

**REJECTED.** One genuine surfacing, not four. The count of 4 was manufactured by
counting the shelving document (IDEAS.md #1-4), the commit that wrote that document,
and the session journal narrating that session as three separate occurrences of one
day-one Plan-3.5 decision, then adding the promotion write itself as a fourth. That is
exactly the one-off overfit the threshold-3 rule is meant to block, and the same
collapse pattern already found in `audit-core-01-rule-uniqueness.md`.

**Recommended action:**

1. Demote `core-67-no-mkvtoolnix-magic-guesses` from the promoted/Tier-2 set back to
   Tier 1 (`decision-ledger.md`) with an honest `count: 1`, `last: 2026-07-09`,
   `outcome: accepted` (restraint kind).
2. Remove the "No mkvtoolnix input-convenience guesses" entry from
   `docs/CONVENTIONS.md` Restraints on this evidence; keep the full argument where it
   already lives (`docs/IDEAS.md` #1-4).
3. Fix the statement's fidelity nit before any future re-promotion: **auto-title** is
   IDEAS #6, not #1-4.
4. The restraint is real and settled, so it may legitimately re-accrue toward
   promotion later from **independent surfacings** (a reviewer `house`-dimension
   catch, an implementer re-proposing a filename derivation and being rejected, an
   idiomacy pass re-affirming it). Promotion waits for real recurrence, not for three
   projections of one commit plus the promotion write.

Note for the backfill process generally: collapse occurrences that share a
session/commit lineage before counting, and never count the promotion write
(CONVENTIONS.md entry) as one of the occurrences that justify the promotion.
