# Adversarial audit: `core-58-attach-add-cardinality` (PROMOTION candidate)

- **Cluster id:** `core-58-attach-add-cardinality`
- **Kind / domain:** pattern / core
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 4
- **Verdict:** **REJECTED** (surviving distinct occurrences: **2**, threshold for promotion is 3)
- **Action:** demote to Tier 1 (`decision-ledger.md`); do **not** stand up as a Tier-2 house convention on this evidence.

The statement is factually accurate and the rule is genuinely implemented (D12 in
the Plan 3 design-decisions spec, `resolve_attachments` in `planner.rs`, 7 passing
tests, reviewer-confirmed). This audit does **not** dispute the pattern's
correctness or its being settled. It disputes the **recurrence count** that
authorizes promotion. Three of the four cited occurrences are artifacts of one
session (session 3, 2026-07-09) surfacing the same decision; only one is an
independent surfacing. The count of 4 is one decision plus one subagent report,
dressed as four.

---

## Counting unit (from the doctrine being audited)

`decision-ledger.md` (lines 8-17) defines the recurrence unit explicitly: the
controller "records a fresh consideration, or increments an existing one, when
**a subagent report or a session** surfaces it." The unit of recurrence is
therefore a *session* or a *distinct subagent report*, **not a document**. Three
documents produced within one session are one surfacing. This is the same
doctrine the `core-01` audit applied: "A spec section, the commit that introduced
it, and the journal entry for that session are one event, not three."

**Session boundary (load-bearing).** The process journal entry itself fixes the
boundary: `docs/process-journal.md` heads the relevant entry
`## 2026-07-09 | Plan 3 complete (pure layer: resolution + command) | session 3
(Peter, Opus 4.8)`. Its Scope line lists design docs `d039e24` (plan), `62d4956`
(D12) **and** the impl range `62d4956..7d46547` — i.e. the D12 debate, the memo,
the finalizing commit, and the Plan 3 SDD execution (including task-8) all fall
inside **session 3**. Its Moments line confirms the debate was in-session:
"The D12 slot-vs-collection reframe (chat, pre-impl)."

---

## Per-occurrence verification

### Occurrence #1 - `memo D12` (2026-07-09, decided)

- **Artifact:** `docs/superpowers/specs/2026-07-09-plan-3-design-decisions.md`,
  section `## D12: attachment add cardinality and zero-match severity` (lines
  172-197).
- **Supports the statement?** Yes, on every clause. Verbatim: "An `add` locator
  attaches **all** files it matches (not exactly one)"; the slot-vs-collection
  rationale ("everything that fills a unique output slot is uniqueness-constrained
  ... everything that populates the attachment collection is multi (`select`,
  `drop`, and therefore `add`)"); zero-match "emits a **warning**
  (`MissingExternal` ...), not an error: it does not suppress the plan"; dedup by
  path; and the steelman ("exactly-one would instead make `add` the lone unique
  attachment rule kind"). Genuine, correctly attributed as `decided`.
- **Distinct?** It is the canonical design-decision artifact of the session-3
  decision. Kept as the single representative of that decision surfacing.
- **Result: KEEP** (representative of the session-3 decision surfacing).

### Occurrence #2 - `commit 62d4956` (2026-07-09, decided)

- **Artifact:** `git show 62d4956` - "docs: finalize D12 (add-dedup,
  slot-vs-collection invariant)". A 2-file, 16-insertion diff that rewrites the
  D12 memo's rationale from the earlier "fonts come in sets" framing to the
  slot-vs-collection invariant, adds the dedup clause, and folds dedup-by-path
  into the Plan 3 Task 8 plan step.
- **Supports the statement?** Yes - it is literally the diff that produced the
  final text of memo D12 (occurrence #1). Genuine, correctly attributed as
  `decided`.
- **Distinct?** **No - duplicate of #1.** The commit is the git event that wrote
  memo D12's final content; the memo and the commit are the same
  content-introduction viewed two ways, both in session 3. This is exactly the
  `core-01` #1/#3 collapse (spec row vs the commit that introduced it). (History:
  D12 was first introduced by `d039e24` and finalized by `62d4956`, both session
  3, same decision refined - not two decisions.)
- **Result: DROP** (duplicate of #1; same session-3 content-introduction).

### Occurrence #3 - `journal Plan 3 (two-round debate ratified)` (2026-07-09, reinforced)

- **Artifact:** `docs/process-journal.md`, entry "## 2026-07-09 | Plan 3 complete
  ... | session 3", "Decisions and why" bullet on `add` cardinality (D12):
  "decided via a two-round debate. Şenol pushed back on the font special-case ...
  Resolved by reframing the real invariant as slot-vs-collection ... Şenol
  ratified collection-populator (all matched + zero-match warning + dedup)."
- **Supports the statement?** Yes, and it adds the debate provenance. Genuine,
  but the "reinforced" label is misleading: this is the session's own close
  narrative of the decision it made, not a later independent re-surfacing.
- **Distinct?** **No.** This is **session 3's own journal** narrating the D12
  decision made in session 3. By the doctrine's counting unit a session is one
  surfacing; a session's journal of its own decision is not a second, independent
  recurrence. Identical to the `core-01` occurrence #2 collapse (the session-1
  journal of a session-1 decision was dropped). The "two-round debate" it records
  is the *decision process for memo D12*, i.e. the provenance of #1/#2 - not a
  distinct event.
- **Result: DROP** (same session-3 surfacing as #1/#2; a session's own journal is
  not an independent recurrence).

### Occurrence #4 - `task-8 verdict (zero findings)` (2026-07-09, reinforced)

- **Artifact:**
  `docs/process-journal/artifacts/plan-3-sdd/verdicts/task-8-review-verdict.md` -
  the Plan 3 SDD task-8 (attachment resolution) reviewer verdict
  (agent `a201598fdbc77bf9e`, session `2b4312c5...`, final message
  `2026-07-09T12:04:21Z`).
- **Supports the statement?** Yes, strongly and independently. The reviewer
  re-validated the D12 semantics against the actual code (not the report): "all
  hits from every `add` locator are appended ... emptiness is checked on that
  rule's own `hits` ... so the warning is scoped per-rule"; "Confirmed
  `Diagnostic::warning(...)`, not `.error(`"; "Dedup ... preserves original
  (first-seen) order while removing later duplicates"; "adds have no 'ambiguous'
  concept since multiple hits are valid by design (D12)". Its "Issues: None found
  at Critical, Important, or Minor severity" is the "zero findings" the ref
  claims. Genuine, correctly attributed as `reinforced`.
- **Distinct?** **Yes.** A review verdict is a *distinct subagent report*, which
  the doctrine names as its own counting unit ("a subagent report **or** a
  session"). Even though it runs inside session 3, it counts separately: an
  independent reviewer re-surfacing the pattern by re-validating it in the
  implementation. Not a duplicate of the design decision (#1/#2/#3), not a
  duplicate of the whole-branch review (which addressed mkvmerge attachment-id
  identity and flag spellings, not add-cardinality).
- **Result: KEEP** (distinct subagent report; genuine independent reinforcement).

---

## Distinctness analysis

| # | ref | artifact | relation to the event |
|---|-----|----------|-----------------------|
| 1 | memo D12 | plan-3 design-decisions spec §D12 | the session-3 design decision (kept as representative) |
| 2 | commit 62d4956 | git event | wrote memo D12's final text - same content-introduction, session 3 |
| 3 | journal Plan 3 | process-journal, session 3 entry | session 3's own narrative of the same decision |
| 4 | task-8 verdict | SDD reviewer subagent report | independent in-code re-validation |

- #1, #2, #3 all resolve to **one session (session 3, 2026-07-09)** surfacing
  **one decision**: #2 is the commit that wrote #1; #3 is that session's journal
  of it. By the doctrine ("three documents produced within one session are one
  surfacing") they collapse to a single surfacing, represented by #1.
- #4 is a **distinct subagent report** - the doctrine's other valid counting unit
  - and is a genuine second surfacing.

**Surviving distinct occurrences: 2** (the session-3 decision + the task-8
reviewer report).

---

## Verdict

**REJECTED.** Two genuine surfacings, not four. The count of 4 was manufactured by
counting the memo, the commit that wrote that memo, and the session's own journal
of that decision as three separate occurrences of what is a single session-3
decision - the exact "one-off dressed as a multi-count" the threshold-3 rule
exists to block - plus one legitimately independent reviewer report. Two is short
of the promotion threshold.

**Recommended action:**

1. Demote `core-58-attach-add-cardinality` from the promoted/Tier-2 set back to
   Tier 1 (`decision-ledger.md`) with an honest `count: 2`, `last: 2026-07-09`,
   `outcome: accepted` (occurrences: the session-3 D12 decision + the task-8
   reviewer report).
2. Do **not** add it to `docs/CONVENTIONS.md` on this evidence.
3. The pattern is real, settled, and correctly implemented; it may re-accrue count
   toward promotion later from an **independent** surfacing (a Plan 4/5 executor or
   command subagent re-touching attachment `add`, an idiomacy pass, a later review
   re-validating the collection-populator invariant). Promotion waits for a real
   third recurrence, not for three views of one session-3 decision.

Note for the backfill process generally (reaffirming the `core-01` note): collapse
occurrences that share a session/commit lineage before counting. A design-decision
memo, the commit that finalized it, and the session journal narrating that
decision are one surfacing. A distinct subagent report (a review verdict) is a
separate unit - but one such report on top of a single session's decision is two,
not the promotion-authorizing three.
