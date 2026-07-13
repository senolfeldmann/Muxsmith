# Adversarial audit: `ci-08-gated-tests-run` (PROMOTION candidate)

- **Cluster id:** `ci-08-gated-tests-run`
- **Kind / domain:** pattern / ci
- **Claimed status:** settled, `promoted: true`, `promoted_at: 3`
- **Claimed count:** 6
- **Verdict:** **CONFIRMED** (surviving distinct occurrences: **6**, threshold for promotion is 3)
- **Action:** promotion stands. The candidate is eligible as a Tier-2 house
  convention (`docs/CONVENTIONS.md`); the six-count is real, not manufactured.

The statement narrates a genuine four-stage arc across four sessions and three
dates: gap noted and deferred (2026-07-09, session 2) -> decision to close it
(memo D18, ratified during Plan 4) -> Linux-only implementation reviewed
(task-6 verdict) and empirically verified to run (post-push CI-log check,
session 5) -> go-public follow-up extends the install to all three legs with a
skip-marker-count-zero assertion per leg (Plan 5.5 T2), which immediately caught
18 silently-skipped Windows tests and then a real Windows-only bug (2026-07-11).
Each stage is anchored in a distinct primary artifact. This is a real recurrence
that accrued and evolved across the timeline, not a one-off dressed as a count.

---

## Counting unit (from the doctrine being audited)

Same lens the `proc-05` / `core-01` audits used: the recurrence unit is a
**session or a distinct subagent report**, not a document, and identical
`date+ref` collapses to one occurrence while distinct `date+ref` carrying a
distinct proposition is kept. Applied here, the six occurrences span **four
sessions** (2, 4/5, 5, 7) and carry **six distinct propositions** (gap-deferred /
decide-to-install / implementation-reviewed / empirically-ran / three-leg+assert
decided / assertion-caught-real-skip). No two share a `date+ref`. The collapse
that sank `core-01` (three views of one session-1 event) does not apply.

---

## Per-occurrence verification

### Occurrence #1 - `journal 2026-07-09 Plan 2 Open threads` (2026-07-09, deferred)

- **Artifact:** `docs/process-journal.md`, entry
  "## 2026-07-09 | Plan 2 written and implemented | session 2" (heading line 115),
  under "**Open threads.**" line 177-178: "CI does not install mkvtoolnix so the
  gated tests self-skip there."
- **Supports the statement?** Yes, verbatim on the evidence text. This is the
  **coverage gap accepted/deferred** the statement describes ("E2 accepted the
  self-skip ... as a coverage gap"). Correctly attributed as `deferred` - it sits
  in the Open-threads deferral list, not a decision to fix.
- **Distinct?** Yes. Session 2, gap-noted-and-parked. (An identical-sentiment
  restatement exists in the Plan 3 entry Open threads at line 231 - "mkvtoolnix
  still not installed in CI (gated integration tests self-skip there)" - but the
  cluster does **not** cite it as a separate occurrence, so no padding there.)
- **Result: KEEP.**

### Occurrence #2 - `memo D18` (2026-07-10, decided)

- **Artifact:** `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`,
  "## D18: Plan 3 follow-up triage" (line 138), bullet lines 145-146: "Install
  mkvtoolnix in CI: the gated integration tier (including the new executor/run
  tests) actually runs there instead of self-skipping."
- **Supports the statement?** Yes, verbatim on the evidence text. The
  (topic, approach) = *install mkvtoolnix in CI so the gated tier runs instead of
  self-skipping* is exactly D18. Correctly attributed as `decided` - it is a
  design-memo decision item.
- **Provenance nit (not a drop):** the memo file is dated **2026-07-09** and the
  occurrence dates the decision **2026-07-10**. Reconcilable, not a
  misattribution: D18 was written 2026-07-09 but parked **UNREVIEWED by Şenol**
  (journal line 265), and consumed/ratified during Plan 4 execution in session 5
  on 2026-07-10 (implementing commit `5aefde1` "ci: install mkvtoolnix on Linux so
  gated tests run (D18)", journal entry "2026-07-10 | Plan 4 complete | session
  5", line 277). Dating the *decision-as-consumed* at 2026-07-10 is defensible;
  the artifact carries the topic and kind exactly.
- **Distinct?** Yes. The design decision, distinct from #3 (independent review of
  the resulting diff) and #1 (the prior deferral).
- **Result: KEEP.**

### Occurrence #3 - `task-6-review-verdict.md` (2026-07-10, reinforced)

- **Artifact:**
  `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-6-review-verdict.md`
  (salvaged reviewer-subagent verdict, session 5). Line 29: "`if: runner.os ==
  'Linux'` is the correct guard given the job's conditional OS matrix"; line 20:
  "matches the deferred-macOS/Windows / minute-economy rationale from the brief";
  line 24: "Report correctly scopes 'gated tests actually ran in CI' as
  controller's post-push verification, not claimed as done here."
- **Supports the statement?** Yes. Evidence text ("Step guarded by if:
  runner.os == 'Linux'; gated tests self-skip elsewhere, macOS/Windows installs a
  go-public follow-up") is on-point: the Linux-only guard, the deferral of
  macOS/Windows to a go-public follow-up, and the still-self-skipping-elsewhere
  state are all in the verdict. Correctly attributed as `reinforced` - an
  independent reviewer report re-affirming the Linux-only implementation.
- **Distinct?** Yes. Distinct subagent report (task-6 reviewer) reviewing the diff
  **pre-push**; a different artifact and different event from #4 (the controller's
  **post-push** empirical CI-log check). See distinctness note below.
- **Result: KEEP.**

### Occurrence #4 - `journal 2026-07-10 session-5-close` (2026-07-10, reinforced)

- **Artifact:** `docs/process-journal.md`, entry "## 2026-07-10 | Session 5 close
  (post-push addendum) | session 5" (heading line 366), line 371: "T6 deferred
  verification DONE: CI run 29059480785 green ..., mkvtoolnix install step
  executed, 0 'mkvmerge not found' skip markers in the full job log, gated tests
  (live_run_*, executor_live, attachment round trip) ran and passed."
- **Supports the statement?** Yes, verbatim on the evidence text (run number, 0
  skip markers, gated tests ran and passed). This is the **empirical** proof the
  pattern held in CI. Correctly attributed as `reinforced`.
- **Distinct?** Yes. Same session as #3 but a **different artifact** (controller
  journal close vs reviewer verdict) attesting a **different fact**: #3 approved
  the diff at review time and explicitly deferred the "actually ran" claim to
  post-push (verdict line 24); #4 is that post-push confirmation. The
  decision->verification arc, not a restatement.
- **Result: KEEP.**

### Occurrence #5 - `task-2-verdict.md / plan T2` (2026-07-11, decided)

- **Artifacts (compound ref, both halves support):**
  - `.superpowers/sdd/plan-5.5/task-2-verdict.md` (opus verdict, 2026-07-11):
    grep marker "mkvmerge not found; skipping" verified against all 19 call sites;
    per-leg pins for apt/choco/brew (Minor 3/4); post-push checklist "Three legs
    green WITH 'Skip-marker occurrences: 0' and tests actually executed" (line 47).
  - `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md`, "### Task 2:
    mkvmerge on all three CI legs" (line 71): Step 1 (lines 78-89) three per-OS
    install steps (apt/Linux, choco/Windows, brew/macOS); Step 3 (line 93)
    "Acceptance: the marker count on Windows and macOS legs is ZERO (walkthrough
    #14: **'otherwise silent skipping is traded for silent skipping'**)."
- **Supports the statement?** Yes. Both clauses of the evidence ("install on all
  three legs, assert skip-marker count zero; otherwise silent skipping is traded
  for silent skipping") are present - the rationale phrase appears **verbatim** in
  plan T2 line 93. Correctly attributed as `decided` (walkthrough #14 decision).
- **Distinct?** Yes. The go-public three-leg extension + assertion decision,
  distinct from the Linux-only stage (#2/#3/#4) and from #6 (its execution
  outcome).
- **Result: KEEP.**

### Occurrence #6 - `progress.md T2 fix waves / journal` (2026-07-11, reinforced)

- **Artifact:** `.superpowers/sdd/plan-5.5/progress.md`, lines 84-87: "T2 fix wave
  1: Windows leg failed run 29165166725 with marker count 18 - choco writes
  machine PATH, running job never re-reads it; the assertion caught exactly its
  target case. Fix 19deec3 (existence check + GITHUB_PATH append, pwsh)." The
  wave-2 tail ("writable handle for set_modified, Windows PermissionDenied ...
  first-ever live Windows run surfaced it", lines 97-99) corroborates the
  statement's closing clause (the real Windows-only `set_modified`-on-read-only
  bug).
- **Supports the statement?** Yes, verbatim on the evidence text (marker count 18,
  choco machine-PATH not re-read, assertion caught its target case). This is the
  **assertion earning its keep** - silent skipping was not merely traded for
  silent skipping. Correctly attributed as `reinforced`.
- **Distinct?** Yes. The execution outcome, distinct from #5 (the decision/review)
  and from #4 (the earlier Linux-only run). Different artifact, different event.
- **Result: KEEP.**

---

## Distinctness analysis

| # | ref | session / date | proposition | kind |
|---|-----|----------------|-------------|------|
| 1 | journal Plan-2 Open threads | session 2 / 2026-07-09 | gap: no mkvtoolnix in CI, gated tests self-skip | deferred |
| 2 | memo D18 | session 4->5 / 2026-07-10 | decide to install so the gated tier runs | decided |
| 3 | task-6-review-verdict | session 5 / 2026-07-10 | Linux-only impl reviewed; mac/win = go-public follow-up | reinforced |
| 4 | journal session-5-close | session 5 / 2026-07-10 | empirically ran: run 29059480785, 0 skip markers | reinforced |
| 5 | task-2-verdict / plan T2 | session 7 / 2026-07-11 | extend to all three legs + assert count zero | decided |
| 6 | progress.md T2 fix waves | session 7 / 2026-07-11 | assertion caught 18 silent Windows skips (choco PATH) | reinforced |

Four sessions, three dates, six distinct propositions. No two occurrences share a
`date+ref`, and none is a paraphrase-collapse of another: each attests a
different act (defer / decide / review / verify / extend-and-assert /
caught-a-real-failure). The pattern genuinely recurred and evolved from a noted
gap into a standing per-leg assertion - exactly the multi-surfacing history the
threshold-3 gate rewards.

**Adversarial probe - the two same-session pairs.** The only place a stricter
reviewer could shave is the two 2026-07-10-session-5 occurrences (#3, #4) and the
two 2026-07-11 T2 occurrences (#5, #6). Both pairs survive scrutiny:

- **#3 vs #4:** distinct artifacts (reviewer subagent verdict vs controller
  journal close) attesting distinct facts (diff-approved-at-review-time, with the
  "actually ran" claim explicitly deferred per verdict line 24, vs the post-push
  CI-log confirmation that it ran). Decision->verification, not restatement.
- **#5 vs #6:** distinct artifacts (verdict/plan vs progress ledger) attesting
  distinct events (the decide-and-review vs the live CI failure the assertion
  caught). Not a restatement.

Even under the most aggressive reading that collapsed one occurrence from each
pair, **four** distinct occurrences would remain - still above the promotion
threshold of 3. On the correct reading, all six stand.

**Surviving distinct occurrences: 6.**

---

## Verdict

**CONFIRMED.** All six occurrences are backed by primary artifacts (two journal
entries, the D18 design memo, the task-6 reviewer verdict, the plan-5.5 T2
plan+verdict, and the plan-5.5 progress ledger), all six are genuine, correctly
kinded, and distinct across four sessions. The count of 6 is real, not
manufactured. Promotion stands; the cluster is eligible for `docs/CONVENTIONS.md`
as a standing house convention.

**Provenance notes (hygiene, not count changes):**

1. **Occurrence #2 date.** The D18 memo file is dated 2026-07-09; the occurrence
   dates the decision 2026-07-10. This is the parked-then-ratified gap (written
   2026-07-09 unreviewed, consumed in Plan 4 execution 2026-07-10). Defensible,
   but if the cluster's dates are meant to be the artifact's own date, note the
   memo authoring date explicitly to avoid a future reader flagging it as drift.
2. **Statement wording.** "go-public extended the install to all three legs"
   compresses two events: go-public (2026-07-10, Plan 5) enacted the static 3-OS
   *matrix* (ci-01), which then exposed Windows/macOS gated tests silently
   self-skipping because the *install* was still Linux-only; the three-leg install
   + assertion was the Plan 5.5 T2 pre-1.0 hardening task (2026-07-11), a
   go-public *follow-up*, not the go-public act itself. The causal narrative is
   coherent; the phrasing merely telescopes it. Worth tightening if the statement
   is promoted verbatim into CONVENTIONS.
