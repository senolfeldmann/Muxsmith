# Adversarial audit: `proc-05-commit-signing` (PROMOTION candidate)

- **Cluster id:** `proc-05-commit-signing`
- **Kind / domain:** pattern / process
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 3
- **Verdict:** **CONFIRMED** (surviving distinct occurrences: **3**, threshold for promotion is 3)
- **Action:** promotion stands. The candidate may be stood up as a Tier-2 house convention; the three-count is real.

The statement narrates a genuine three-stage evolution across three distinct
sessions on three distinct dates: mechanical origin (2026-07-08, GPG blocks
agent commits) -> elevation to policy (2026-07-09, signature = authorship claim,
applies even when signing succeeds, on merges too) -> violation and reinforcement
(2026-07-10, five wave-1 merges accidentally signed). Each stage is documented in
a distinct primary artifact. Unlike a one-off dressed as a three-count, this is a
real recurrence pattern that accrued across the project timeline.

---

## Counting unit (from the doctrine being audited)

`decision-ledger.md` / software-dev-process doctrine §7: the controller records a
fresh consideration, or increments an existing one, when **a subagent report or a
session** surfaces it. The recurrence unit is a *session* or a *distinct subagent
report*, not a document. This is the same lens that sank `core-01` (three views of
one session-1 event). Here the three occurrences fall in **three separate
sessions** (session 1, session 4, session 5) and carry **three distinct
propositions** (mechanical workaround / policy elevation / violation-correction),
so the collapse that rejected core-01 does not apply.

---

## Per-occurrence verification

### Occurrence #1 — `journal 2026-07-08 "Friction" (E0[9])` (2026-07-08, decided)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-08 | Spec + Plan 1 complete, repo live | session 1", under
  "**Friction and failure.**" (lines 77-78): "GPG signing blocks agent commits;
  standing workaround `-c commit.gpgsign=false`."
- **Supports the statement?** Yes, verbatim on the evidence text. This is the
  **mechanical origin** the statement describes ("Origin was mechanical (GPG
  blocks agent commits)"). Correctly attributed as `decided` — a standing
  workaround was adopted.
- **Cross-index:** find-E0.md item E0-50 carries the same occ_ref and evidence,
  and explicitly flags that the "authorship-claim / policy" framing is *not* E0
  but Şenol's later 2026-07-09/10 decision — consistent with keeping this
  occurrence scoped to the mechanical origin only.
- **Distinct?** Yes. Session 1, mechanical-only. Distinct proposition from #2/#3.
- **Result: KEEP.**

### Occurrence #2 — `journal session-4-close + progress-ledger SI-4 (E5[43])` (2026-07-09, decided)

- **Artifact (authoritative half):** `docs/process-journal/artifacts/plan-4-sdd/progress.md`,
  line 53, "For HANDOFF refresh at close-out": "SI-4 update (Şenol 2026-07-09):
  agent commits deliberately UNSIGNED as policy (signature = his authorship
  claim), not merely because GPG blocks; gpgsign=false on every agent commit AND
  merge even when signing would succeed."
- **Supports the statement?** Yes, verbatim on the evidence text. This is the
  **elevation to policy** ("elevated to policy (SI-4) because a signature is
  Şenol's authorship claim, applied even when signing would succeed"). Correctly
  attributed as `decided` — a Şenol policy decision dated 2026-07-09.
- **Citation nit (not a drop):** the compound ref's *journal session-4-close* half
  does **not** carry the signing policy. That entry
  ("## 2026-07-09 | Session close: Plan 4 designed and planned ...", lines 267-275)
  mentions SI-4 only as standing commit/**push authorization** and the permission-
  classifier friction — nothing about GPG signing. The signing policy's journal
  home is actually the 2026-07-10 Plan-4-complete Decisions block (lines 289-292),
  attributed there as a Şenol 2026-07-09 decision. The ref survives on the
  strength of its **progress-ledger** half, which is exact and correctly dated;
  the journal sub-citation is misplaced but the occurrence is genuine, dated, and
  distinct — not fabricated, not misattributed to the wrong topic.
- **Distinct?** Yes. Session 4, policy elevation. Genuinely different content and
  date from #1 (mechanical origin) and #3 (violation). Not a duplicate: it changes
  the rule's scope (applies even when signing succeeds; extends to merges) rather
  than restating #1.
- **Result: KEEP.**

### Occurrence #3 — `journal Plan-4-complete (E5[44])` (2026-07-10, violated-corrected)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-10 | Plan 4 complete (executor + run + queue) | session 5":
  - "**Friction and failure.**" (lines 333-334): "Controller ran merges without
    gpgsign=false -> the signed/unsigned mix Şenol spotted on GitHub. Policy above."
  - "**Decisions and why.**" (lines 289-292): "Şenol (2026-07-09): agent commits
    stay UNSIGNED as policy (a GPG signature is his authorship claim), after the
    controller accidentally signed the **five wave-1 merge commits (unlocked
    gpg-agent)**. Left in history; rule now in HANDOFF SI-4 and Peter memory."
- **Supports the statement?** Yes, on every clause of the evidence: "five wave-1
  merges signed (unlocked gpg-agent)" (line 290-291), "signed/unsigned mix spotted
  on GitHub" (line 333-334), "left in history, rule reinforced in HANDOFF SI-4 +
  Peter memory" (line 291-292). Correctly attributed as `violated-corrected` — the
  accidental signing is the violation, the reinforcement (left in history + rule
  restated in HANDOFF + memory) is the correction.
- **Reinforcement cross-check:** the HANDOFF SI-4 restatement is real
  (`handoffs/2026-07-10-plan-4-close.md` §SI-4: "... agent commits stay UNSIGNED")
  and the ledger note (`plan-4-sdd/progress.md` line 53) records the memory update.
- **Distinct?** Yes. Session 5, violation-correction event. Distinct from #1/#2.
- **Result: KEEP.**

---

## Distinctness analysis

| # | ref | session / date | proposition | kind |
|---|-----|----------------|-------------|------|
| 1 | journal 2026-07-08 Friction | session 1 / 2026-07-08 | GPG blocks commits -> `gpgsign=false` workaround | decided (mechanical origin) |
| 2 | progress-ledger SI-4 (+journal, misplaced) | session 4 / 2026-07-09 | signature = authorship claim; unsigned even when signing succeeds; on merges too | decided (policy elevation) |
| 3 | journal Plan-4-complete | session 5 / 2026-07-10 | five wave-1 merges accidentally signed; mix spotted; rule reinforced | violated-corrected |

Three separate sessions, three separate dates, three separate propositions. No
shared session/commit lineage collapses any pair (contrast core-01, where spec
row + root commit + session journal were one session-1 event). The pattern
genuinely recurred and evolved: adopted as a workaround, elevated to policy,
tested by a violation and reinforced. This is precisely the multi-surfacing
history that the threshold-3 promotion gate is designed to reward.

**Surviving distinct occurrences: 3.**

---

## Verdict

**CONFIRMED.** All three occurrences are backed by primary artifacts (journal
entries and the plan-4 progress-ledger), all three are genuine, and all three are
distinct surfacings across separate sessions. The count of 3 is real, not
manufactured. Promotion stands.

**Recommended actions:**

1. Keep `proc-05-commit-signing` in the promoted/Tier-2 set; it is eligible for
   `docs/CONVENTIONS.md` as a standing house convention on this evidence.
2. **Fix the occurrence-2 journal sub-citation** when the cluster is next touched:
   replace "journal session-4-close" with the 2026-07-10 Plan-4-complete
   *Decisions* block (lines 289-292), which is where the journal actually records
   the signing policy. The progress-ledger citation is correct and should stay.
   This is a provenance-hygiene fix, not a count change.
3. Minor: the `E0[9]` / `E5[43]` / `E5[44]` bracket indices do not line up with
   the find-file item numbers (`E0-50`, find-E5 items 62/63). The *content* of
   every cited find entry matches the occurrence evidence verbatim, so this is a
   cosmetic index-label drift, not a substantive misattribution; worth normalizing
   if the E-index scheme is meant to be dereferenceable.
