# Adversarial audit: `proc-01-sdd` (PROMOTION candidate)

- **Cluster id:** `proc-01-sdd`
- **Kind / domain:** pattern / process
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 7
- **Verdict:** **CONFIRMED** (surviving distinct occurrences: **7**, threshold for promotion is 3)
- **Action:** promotion stands. `proc-01-sdd` is a genuine standing convention; keep it in the Tier-2 house-knowledge set.

The statement is factually accurate, the pattern is genuinely practiced, and
every cited ref resolves to a real artifact that supports the claimed
`(topic, approach, kind)`. Crucially, the seven occurrences are **seven distinct
sessions**, not seven views of one event. This is the exact opposite failure
profile of `core-01-rule-uniqueness` (REJECTED: three views of one session-1
decision). Here each occurrence is an independent surfacing, and in four of the
seven cases SDD was not merely re-stated but **actually re-executed** (implementer
+ reviewer subagent dispatches recorded in the journal metrics).

---

## Counting unit (from the doctrine being audited)

`decision-ledger.md` / software-dev-process doctrine §7 defines the recurrence
unit: the controller records/increments when **a subagent report or a session**
surfaces the topic. The unit is a *session* or a *distinct subagent report*, not a
document. Multiple documents from one session collapse to one surfacing (this is
what sank `core-01`). The audit therefore tests two things per occurrence:

1. Does the cited artifact **support** the claimed `(topic, approach)` as the stated `kind`?
2. Is the occurrence a **distinct session/surfacing**, or a duplicate view of another listed occurrence's session?

---

## Per-occurrence verification

### Occurrence #1 — Plan 1 adoption (2026-07-08, `decided`) — session 1

- **Refs:** journal 2026-07-08 "Mechanics" + handoff `2026-07-09-plan-1-close.md` + Plan 1 REQUIRED SUB-SKILL header. Global index E0[3]/E1[14].
- **Artifacts checked:**
  - `docs/process-journal.md` "2026-07-08 | ... | session 1", *Mechanics and metrics*: "~31 subagent dispatches: 13 implementers, 13 task reviews, ~7 fix/re-review waves, 1 docs pass, 1 final whole-branch review + confirmation." Verbatim match to the occurrence's evidence line. Same entry, *Moments*: "Final reviewer graded the plan's own code against the plan's constraints and failed it on three counts; 'the plan does not grade its own work' proved to be the load-bearing process rule." — supports the "plan never grades its own work" clause.
  - `docs/superpowers/plans/2026-07-08-plan-1-core-foundations-validate-cli.md:3`: "REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development ... to implement this plan task-by-task."
  - `docs/process-journal/artifacts/handoffs/2026-07-09-plan-1-close.md:25`: "executed via subagent-driven-development (fresh implementer per task, task review, fix waves, final whole-branch review; model split ...)" and `:26` "Controller verifies independently ... Re-run suites yourself; never trust report arithmetic."
- **Supports the statement / kind?** Yes — the full apparatus (fresh implementer, task review, fix waves, whole-branch review, controller re-runs suites, plan-does-not-grade-itself) is documented as adopted at Plan 1. Correctly `decided`.
- **Distinct?** Yes — session 1, the origin adoption. All three refs are one-session views bundled into one occurrence (correct).
- **Result: KEEP.**

### Occurrence #2 — Plan 2 deviation + correction (2026-07-09, `violated-corrected`) — session 2

- **Refs:** journal 2026-07-09 Plan 2 Decisions + fix-pass; `plan-2-review/independent-review-2026-07-09.md`. Global index E2[21]/[22].
- **Artifacts checked:**
  - `docs/process-journal.md` "2026-07-09 | Plan 2 written and implemented | session 2": "Execution deviated from Plan 1's subagent-driven-development: the controller (Opus) executed the tasks inline ... Tradeoff: no independent per-task reviewer; a whole-branch adversarial review is still owed." — the violation.
  - `docs/process-journal.md` "2026-07-09 | Plan 2 fix pass (SDD, corrective) | session 2 (cont.)", *Contrast with inline Plan 2*: "the independent reviewer/controller separation turned '125 tests green, shipped ~11 bugs' into caught-before-merge." — verbatim match to the occurrence evidence.
  - `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md` (file exists): header "Retrofit of the SDD independent-review stage that Plan 2 skipped ... the controller's '125 tests green' self-verification missed every item below." Bug list A–K = 11 confirmed defects ("~11 bugs"), all caught pre-merge by the retrofit review.
- **Supports the statement / kind?** Yes — the one documented deviation, its cost (~11 shipped bugs behind a green suite), and the retrofit SDD review that caught them all before merge. Correctly `violated-corrected`.
- **Distinct?** Yes — session 2. The inline execution and the same-session corrective fix-pass form one violation→correction arc, correctly bundled as one occurrence (not double-counted).
- **Result: KEEP.**

### Occurrence #3 — Plan 3 first full Muxsmith SDD (2026-07-09, `decided`) — session 3

- **Refs:** journal 2026-07-09 Plan 3 complete (Scope) + progress.md. Global index E3[27].
- **Artifacts checked:**
  - `docs/process-journal.md` "2026-07-09 | Plan 3 complete | session 3", *Scope*: "First Muxsmith plan executed fully via superpowers subagent-driven-development (SI-1), in contrast to Plan 2's inline execution." — verbatim match. *Mechanics*: "12 implementers + 1 fix + 1 final-minor-fix = 14 build; 12 task reviews + 1 T9 re-review + 1 whole-branch = 14 reviews ... Controller re-ran the gate after every task (SI-1)." — SDD actually executed, not just stated.
- **Supports the statement / kind?** Yes — Plan 3 is the first Muxsmith plan run end-to-end under SDD after the Plan 2 deviation. Correctly `decided`.
- **Distinct?** Yes — session 3, distinct from session 2 (Plan 2). Note: the `+ progress.md` secondary ref does not resolve at HEAD — `.superpowers/sdd/progress.md` currently holds **Plan 5's** ledger (the stale-report overwrite trap the journal itself documents for Plan 3). The primary journal ref is verbatim-solid, so the occurrence stands on it; the stale path is a data-quality note, not a drop.
- **Result: KEEP.**

### Occurrence #4 — Plan 3.5 reinforced (dated 2026-07-10 in cluster; actual session 4, journal-dated 2026-07-09, `reinforced`)

- **Refs:** Plan 3.5 Global Constraints + progress ledger. Global index E4[35].
- **Artifacts checked:**
  - `docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md:20`: "Execute via SDD per HANDOFF SI-1 (fresh implementer + independent reviewer per task); the controller re-runs suites itself." — verbatim match to the occurrence evidence. Line 635 mandates the whole-branch review on the most capable model per SI-1.
  - `docs/process-journal.md` "2026-07-09 | Plan 3.5 complete | session 4": "7 SDD tasks + 1 review-fix ... 7 implementer + 1 fixer dispatches; 7 task reviews + 1 whole-branch ... controller re-ran full gate ... after every task." — SDD actually executed.
- **Supports the statement / kind?** Yes — the pattern is re-instructed in the plan's binding constraints and actually practiced. Correctly `reinforced`. (This is also the session that produced the SI-1 rewrite — "Superpowers-throughout + parallelize-independent" — after Şenol's serial-execution criticism, which occurrence #7 later folds into doctrine.)
- **Distinct?** Yes — session 4, distinct from sessions 2/3.
- **Data-quality note:** cluster `date: 2026-07-10` is off by one; Plan 3.5 is session 4, journal-dated 2026-07-09 (`2026-07-09-plan-3.5-close.md`). Misdating, not misattribution — does not affect the occurrence.
- **Result: KEEP.**

### Occurrence #5 — Plan 5 reinforced (2026-07-10, `reinforced`) — session 6

- **Refs:** Plan 5 waves + journal Plan 5. Global index E6[50].
- **Artifacts checked:**
  - `docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md:24-37`: "## Dependency graph and execution waves (SI-1)" — Wave 0 through Wave 7; parallel worktree waves at Wave 1/3/5; ":37 After each worktree passes its task review, merge to master sequentially, re-running the full gate per merge." — supports "7 waves, 3 parallel worktree waves, sequential merges re-gating each time."
  - `docs/process-journal.md` "2026-07-10 | Plan 5 complete | session 6", *Mechanics*: "14 tasks, 7 waves; 3 parallel worktree waves (4+2+2 streams) ... 12 implementers + 12 task reviewers ... 1 whole-branch reviewer ... controller re-ran the 4-command gate itself." — SDD actually executed, matching the occurrence evidence.
- **Supports the statement / kind?** Yes. Correctly `reinforced`.
- **Distinct?** Yes — session 6, distinct from all prior.
- **Result: KEEP.**

### Occurrence #6 — Plan 5.5 (2026-07-12, `decided`) — session 9

- **Refs:** Plan 5.5 Global Constraints + progress.md. Global index E7[61].
- **Artifacts checked:**
  - `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md:17`: "Execution via SDD: fresh implementer + independent reviewer per task; controller re-runs gates itself (HANDOFF SI-1)." `:24`: "Reviewer verdicts are FILES in that dir at creation (doctrine §2)." `:25`: "full gate re-run after every merge." — verbatim match to all three clauses of the occurrence evidence.
  - `.superpowers/sdd/plan-5.5/progress.md`: per-task verdict files (`task-2-verdict.md` etc.), whole-branch review (fable), controller gate re-runs — SDD actually executed. (The `plan-5.5/` verdict files exist on disk: `task-1-verdict.md` … `task-14-verdict.md`.)
  - Corroborated by `docs/process-journal.md` "2026-07-12 | Plan 5.5 complete | session 9": "~30 implementer dispatches ... ~20 task reviews ... whole-branch review + final verification on fable."
- **Supports the statement / kind?** Yes — every clause (fresh implementer + independent reviewer, controller re-runs gates, verdicts-are-files-at-creation) verified. Correctly `decided`/reinforced.
- **Distinct?** Yes — session 9, distinct from all prior.
- **Result: KEEP.**

### Occurrence #7 — doctrine folding (dated 2026-07-12 in cluster; actual session 7, journal-dated 2026-07-11, `reinforced`)

- **Refs:** journal session 7 pt2. Global index E8[82].
- **Artifacts checked:**
  - `docs/process-journal.md` "2026-07-11 | Session 7 ... process doctrine, Plan 5.5 authored | session 7", point 2 (*Decisions and why*): "Packaged as software-dev-process skill + doctrine file ... behavior-as-package ... feedback_superpowers_throughout memory deleted, content integrated as doctrine section 0." — "journal session 7 pt2" is a precise, real reference and it supports the claim verbatim.
  - `find-E8.md` item 19: "Execution method :: Superpowers-throughout + parallelize-independent (SI-1) folded in as doctrine section 0 — pattern/reinforced/process ... occ_ref: journal session 7 pt2." — the evidence registry's own record matches.
  - Corroboration (framework-side): `~/dotfiles-private/.../skills/software-dev-process/SKILL.md` exists and carries the execution-method binding ("§3 Pre-execution gate (method is binding)"). The literal label "section 0" from 2026-07-11 has since been renumbered into §3 as the doctrine matured — expected doctrine evolution, not a broken reference. The occurrence's cited artifact (the journal) supports the claim regardless of the current section number.
- **Supports the statement / kind?** Yes — the SDD/SI-1 execution method being folded into the packaged, PROJECT-binding doctrine is a genuine reinforcement (the "folded into the packaged doctrine" clause of the cluster statement). Correctly `reinforced`.
- **Distinct?** Yes — session 7, a meta/process session distinct from every plan-execution session above.
- **Data-quality note:** cluster `date: 2026-07-12` is off; the doctrine folding is session 7, journal-dated 2026-07-11. Misdating, not misattribution.
- **Result: KEEP.**

---

## Distinctness analysis

| # | session | trail point | kind | SDD re-executed here? | verdict |
|---|---------|-------------|------|-----------------------|---------|
| 1 | session 1 (07-08) | Plan 1 | decided (adoption) | yes (~31 dispatches) | KEEP |
| 2 | session 2 (07-09) | Plan 2 inline + fix-pass | violated-corrected | yes (fix-pass SDD) | KEEP |
| 3 | session 3 (07-09) | Plan 3 | decided (first full) | yes (14 build / 14 review) | KEEP |
| 4 | session 4 (07-09) | Plan 3.5 | reinforced | yes (7 tasks + reviews) | KEEP |
| 5 | session 6 (07-10) | Plan 5 | reinforced | yes (12+12 + whole-branch) | KEEP |
| 6 | session 9 (07-12) | Plan 5.5 | decided/reinforced | yes (~30 + ~20 + WB) | KEEP |
| 7 | session 7 (07-11) | doctrine folding | reinforced | n/a (packaging event) | KEEP |

- **No two occurrences share a session or commit lineage.** Each maps to a distinct
  session with its own commit range (61249f9.., 3b71a71.., 62d4956.., 91b19eb..,
  735c723.., e8e85d9.., fe7119d..). This is the decisive contrast with `core-01`,
  where all three refs were the single session-1 spec/commit/journal triple.
- Occurrences #2 and #3 share the calendar date 2026-07-09 but are **sessions 2 and
  3** with disjoint commit ranges and different plans — genuinely distinct
  surfacings, not one day double-counted.
- Within occurrences #1, #2, #6 the cited refs bundle several documents (journal +
  handoff + plan header; deviation + fix-pass + review file; plan constraints +
  progress). Each bundle is **one session's** views, correctly collapsed into **one**
  occurrence. The count of 7 counts sessions, not documents.
- Independent-surfacing quality is unusually strong for a process pattern: four
  occurrences (#3–#6) record SDD being **actually re-executed** (implementer/reviewer
  dispatch counts in the journal), not merely re-asserted in prose.

**Surviving distinct occurrences: 7.**

---

## Verdict

**CONFIRMED.** All seven occurrences trace to real artifacts that support the
claimed `(topic, approach, kind)`, and all seven are distinct sessions. The
recurrence is real: SDD was adopted (Plan 1), deviated-from-and-corrected once
(Plan 2), then held and re-executed across every subsequent plan (Plan 3, 3.5, 5,
5.5) and folded into the packaged doctrine (session 7). This is a settled standing
convention, not a manufactured count.

**Recommended action:**

1. Keep `proc-01-sdd` in the promoted Tier-2 house-knowledge set (`docs/CONVENTIONS.md`); the `count: 7` is honest.
2. Fix two cosmetic data-quality slips in the cluster record (they do not affect the verdict):
   - Occurrence #4 `date` should be `2026-07-09` (Plan 3.5 / session 4), not `2026-07-10`.
   - Occurrence #7 `date` should be `2026-07-11` (session 7 doctrine folding), not `2026-07-12`.
3. Note that occurrence #3's `+ progress.md` secondary ref no longer resolves at
   `.superpowers/sdd/progress.md` (overwritten by Plan 5 under the stale-report
   trap); the salvaged Plan-3 ledger under `docs/process-journal/artifacts/` is the
   durable copy. The primary journal ref carries the occurrence regardless.

Note for the backfill process generally: `proc-01-sdd` is the model case for what a
real promotion looks like — one occurrence per distinct session, each an
independent surfacing, several of them actual re-executions rather than restated
intent. Contrast `core-01`, where three refs collapsed to one session.
