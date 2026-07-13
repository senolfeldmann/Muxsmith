# Adversarial audit: `proc-03-model-assignment` (PROMOTION candidate)

- **Cluster id:** `proc-03-model-assignment`
- **Kind / domain:** pattern / process
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 3
- **Verdict:** **CONFIRMED** (surviving distinct occurrences: **3**, threshold for promotion is 3)
- **Action:** promotion stands; the cluster may become a Tier-2 house convention on this evidence.

**Statement under audit:** "Model strength goes where judgment is, cheaper
capacity where transcription is: strongest model for the final review and the
decision memos, mid-tier for judgment implementers and every task reviewer,
cheapest for tasks whose code the plan already carries."

Unlike the `core-01` reject, all three cited refs resolve to **three distinct
sessions** (1, 2, 3), each is present verbatim in the primary-source journal,
each is correctly attributed as `decided`, and no two collapse to a single
surfacing. This is a real recurrence, not three views of one event.

---

## Counting unit (from the doctrine being audited)

`decision-ledger.md` / software-dev-process doctrine §7 defines the recurrence
unit: the controller "records a fresh consideration, or increments an existing
one, when **a subagent report or a session** surfaces it." The unit is a
*session* or a *distinct subagent report*, not a document and not a date. The
`cluster-process.md` header (line 9) restates the discipline for this backfill:
"same-session/same-document citations collapse to one occurrence." The audit
below is applied at that granularity.

---

## Per-occurrence verification

### Occurrence #1 — `journal 2026-07-08 model-split bullet (E0[6])` (2026-07-08, decided)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-08 | Spec + Plan 1 complete, repo live | **session 1** (Peter,
  Fable 5)", under "Mechanics and metrics":
  "Model split: haiku for transcription implementers (plan contained complete
  code), sonnet for judgment implementers (T9, T12, fix wave, docs) and all
  task reviewers, fable for the final review; controller main loop Fable 5;
  the per-role models above cover every dispatch, none ran on the controller
  model."
- **Quote fidelity:** the cluster's evidence string ("haiku for transcription
  implementers, sonnet for judgment implementers and all task reviewers, fable
  for the final review") matches the journal verbatim with only the
  parentheticals elided. Faithful.
- **Supports the statement?** Yes on every tier: cheapest (haiku) for
  transcription implementers whose code the plan already carried; mid-tier
  (sonnet) for judgment implementers and all task reviewers; strongest (fable)
  for the final review. Genuine, correctly attributed as `decided`.
- **Distinct?** **Yes — session 1.** Corroborated by the E0 reconstruction
  (`find-E0.md` E0-47 "Model split by task type", occ_ref "journal 2026-07-08
  'Mechanics' model-split bullet").
- **Result: KEEP.**

### Occurrence #2 — `journal 2026-07-09 Plan 2 Decisions (E2[19])` (2026-07-09, decided)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-09 | Plan 2 written and implemented | **session 2** (Peter,
  Fable 5 -> Opus 4.8 mid-session)", under "Decisions and why", first bullet:
  "Session opened under a Fable quota crunch; Şenol asked what was worth the
  last Fable tokens. Decided: spend model strength on the DECISIONS (D1-D6) not
  the transcription. Wrote the design memo first, then folded D1-D5 into the
  authoritative spec ... Then quota moved to Opus and the rest proceeded."
- **Quote fidelity:** the primary evidence string ("spend model strength on the
  DECISIONS (D1-D6) not the transcription") is verbatim; the paraphrase tail
  ("memo + spec fold-in under Fable, then quota to Opus") accurately compresses
  lines 125-128. Faithful.
- **Supports the statement?** Yes — this is the model-assignment principle
  applied to a *scarce-token* budget rather than a per-role split: the scarce
  strongest-model capacity (Fable, under quota crunch) is spent on the highest-
  judgment artifacts (the D1-D6 decisions and the design memo/spec fold-in),
  and cheaper capacity (Opus, "the rest") handles the mechanical transcription.
  This is the sole occurrence backing the statement's "**and the decision
  memos**" clause. Genuine, correctly attributed as `decided`.
- **Distinct?** **Yes — session 2, and not a same-session duplicate.** Session 2
  also carries a Plan-2 fix-pass model-assignment line (line 194: "sonnet for
  implementers and per-task reviewers, opus for the final whole-branch review;
  controller Opus 4.8 (post-switch)"). The cluster did **not** cite that line —
  had it done so alongside this one, it would have padded session 2 with two
  occurrences. It took exactly one surfacing from session 2 (the Fable-quota
  decision). Clean.
- **Result: KEEP.**

### Occurrence #3 — `journal 2026-07-09 Plan 3 complete metrics (E3[28])` (2026-07-09, decided)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-09 | Plan 3 complete (pure layer: resolution + command) |
  **session 3** (Peter, Opus 4.8)", under "Mechanics/metrics":
  "Models: sonnet for all implementers and task reviewers, opus for the
  whole-branch review; controller Opus 4.8; the two roles above cover every
  dispatch, none ran on the controller model."
- **Quote fidelity:** the cluster's evidence string matches the journal
  verbatim (only "the two roles above cover every dispatch," elided). Faithful.
- **Supports the statement?** Yes — mid-tier (sonnet) for all implementers and
  task reviewers, strongest (opus, the top model after the mid-session Fable ->
  Opus switch) for the whole-branch final review. The cheapest/haiku tier is
  absent here because Plan 3 had no pure-transcription implementer tasks; this
  is a faithful application of the principle, not a contradiction of it.
  Genuine, correctly attributed as `decided`.
- **Distinct?** **Yes — session 3**, a separate session from #2 despite the
  shared 2026-07-09 date (session 2 = Plan 2 + fix pass; session 3 = Plan 3).
  Corroborated by `find-E3.md`.
- **Result: KEEP.**

---

## Distinctness analysis

| # | ref | session | surfacing |
|---|-----|---------|-----------|
| 1 | journal Plan 1 model-split bullet | session 1 (2026-07-08) | Plan 1 per-role split (haiku/sonnet/fable) |
| 2 | journal Plan 2 Decisions | session 2 (2026-07-09) | Fable-quota: strong model on the D1-D6 decisions/memo |
| 3 | journal Plan 3 metrics | session 3 (2026-07-09) | Plan 3 per-role split (sonnet/opus) |

- Three distinct sessions, three distinct decision events. This is precisely the
  property `core-01` lacked (there all three refs were session 1).
- No document/commit/session lineage is shared between the three. The one
  same-session trap available (the session-2 fix-pass model line) was correctly
  left uncounted.
- The tier that shifts across occurrences (haiku present in #1, absent in #3
  because Plan 3 had no plan-carries-the-code transcription tasks; the strongest
  slot moving Fable -> Opus with the mid-session model switch) is a faithful
  *application* of the same principle to different task mixes, not evidence of a
  different rule. It does not undermine any occurrence.

**Surviving distinct occurrences: 3.**

---

## Verdict

**CONFIRMED.** Three genuine, verbatim-sourced, correctly-attributed `decided`
surfacings across three distinct sessions. The count of 3 is real, not
manufactured: no spec-plus-commit-plus-journal triple of one event, no
same-session double-count (the available session-2 duplicate was deliberately
excluded). The recurrence is exactly the kind the threshold-3 rule is meant to
admit.

**Recommended action:**

1. Promotion stands. `proc-03-model-assignment` is eligible as a Tier-2 house
   convention.
2. When writing it into `docs/CONVENTIONS.md`, carry the nuance the occurrences
   show: the *cheapest* tier applies only when a task is pure transcription
   whose code the plan already carries (as in Plan 1); once implementer work
   carries judgment, it moves up to the mid tier (Plans 2 fix-pass / 3). The
   principle is stable; the concrete model per tier tracks both the task mix and
   whatever the current strongest available model is (Fable, then Opus 4.8 after
   the mid-session switch).
