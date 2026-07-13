# Audit: i18n-02-prose-free-core (PROMOTION candidate)

**Cluster:** `i18n-02-prose-free-core` - "core emits diagnostic codes + params only, never user-facing prose; all text lives in Fluent catalogs / per-locale markdown, rendered at presentation time. A new `DiagCode` without a matching `.ftl` message fails catalog-completeness."
**Claimed count:** 5 | **promoted:** true | **status:** settled | **residual breach:** `i18n-17`
**Audit date:** 2026-07-13 | **Method:** each occurrence's cited ref opened in the repo (spec/plan section, journal entry, verdict file, or `git show <hash>`) and checked against the claim "this (topic, approach) arose here as {kind}".

**Verdict: REDUCED** - 4 of 5 occurrences survive as genuine, distinct, on-topic attestation points. One "decided" occurrence is a same-day, same-kind double-cite of another and is collapsed. 4 >= 3, so **promotion stands**. The surviving four span Plan 1, Plan 3.5, and Plan 4 across three lifecycle actions (decided / violated-corrected / reinforced x2), which is the substantive test for a standing convention; the promotion is robust under every defensible counting rule (never below 4).

---

## Statement under audit

> `muxsmith-core` emits diagnostic codes + structured params only, never user-facing prose; all labels, messages and hints live in shared Fluent catalogs (and per-locale markdown) rendered at presentation time. A new `DiagCode` without a matching `.ftl` message fails the catalog-completeness test. (Open residual breach tracked as `i18n-17`.)

## Counting model (cluster-i18n.md header + sibling cluster-core.md line 5)

The cluster keeps "distinct `date+ref` as genuine recurrence" and dedups "identical `date+ref`". The sibling operational definition of a duplicate is "the same attestation point / same artifact double-booked." The cluster header proactively discloses its two judgment calls (record 5 double-cited across clusters; records 4+7 deduped to one). This audit accepts that model and applies it uniformly, and departs from the cluster only on the one call below where the disclosed split re-documents a single same-day decision.

---

## Per-occurrence findings

### 1. 2026-07-08 decided - "spec §5.2 + §8.4 + journal 2026-07-08 bullet 6 tail" - SURVIVES (canonical decided event)
- **Artifacts:** `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` §5.2 (line 249-255) + §8.4 (line 392-399); `docs/process-journal.md` 2026-07-08 entry, "Decisions and why" bullet 6 (the i18n bullet, lines 33-35).
- **Evidence:** §5.2: "Diagnostics are data, produced only by `muxsmith-core` ... Core emits no user-facing prose: `code` plus structured `params` select and fill a message and hint template from the shared catalog at presentation time (8.4)." §8.4: "No hardcoded user-facing strings in any layer ... Core emits diagnostic codes and params only (5.2); labels, tooltips, messages and hints live in Fluent catalogs; long-form help lives in per-locale markdown." Journal bullet 6 tail: "Core emits code+params only; this rule later forced a real fix (see below)." - a verbatim match to the occurrence's evidence string.
- **Judgment:** Real, exactly on-topic, the founding "decided" attestation. This is the canonical decision record (the journal's "Decisions and why" entry plus the two authoritative spec sections).

### 2. 2026-07-08 decided - "spec §2 + §8.4; Plan 1 Global Constraints; commits 61249f9 / a671949" - DROPPED (same-day, same-kind double-cite of #1)
- **Artifacts:** spec §2 Decision log rows "DRY strategy" / "Localization" (lines 28-29) + §8.4; `docs/superpowers/plans/2026-07-08-plan-1-core-foundations-validate-cli.md` Global Constraints (line 14); commits `61249f9` + `a671949`.
- **Evidence (all real and on-topic):** spec §2 "diagnostics as data" and "No hardcoded user-facing strings anywhere. One Fluent catalog set ..."; Plan 1 Global Constraints line 14: "`muxsmith-core` emits NO user-facing prose. Diagnostics carry `code` + `params` only (spec 5.2). All human text lives in `locales/*/*.ftl`." Commit `61249f9` = "docs: add Muxsmith v1 design spec"; `a671949` = "docs(spec): i18n-ready architecture ... Diagnostics become code+params (no prose in core)".
- **Why dropped:** Same date (2026-07-08), same kind (`decided`), same session-1 design act as #1, sharing spec §8.4 as the same corroborating section. Both cited commits are the **spec-authoring docs commits themselves** (`61249f9` is the initial spec; `a671949` adds §8.4) - i.e. the very artifact underlying #1's spec citation, not an independent code implementation (the sibling `i18n-01` labels `a671949` "first implementation", which is inaccurate: it is a `docs(spec)` commit). What remains genuinely distinct in #2 is the Plan 1 Global Constraints restatement - but that is the same decision re-documented in the plan the same day, one lifecycle action, not a second event. By the cluster's own dedup precedent (records 4+7: "same event -> one occurrence, not two"), the single 2026-07-08 `decided` event is counted once. Unlike the sibling's accepted find-vs-fix split (two distinct lifecycle actions in time), #1 and #2 are the same action (decided) on the same day.
- **Note:** The cluster's disclosed reading keeps this as a distinct touchpoint (distinct artifact = distinct occurrence, mirroring the identical two-row split in `i18n-01`). Under that reading the count is 5 / CONFIRMED. The verdict and the promotion are identical either way; only the number moves (5 vs 4).

### 3. 2026-07-08 violated-corrected - "journal final review + whole-branch-review-verdict.md (Important #1), round-2 confirmation; fix 3c24845" - SURVIVES
- **Artifacts:** `docs/process-journal.md` 2026-07-08 entry ("What the process caught", line 49-50; "Moments", line 90-92); `docs/process-journal/artifacts/plan-1-sdd/verdicts/whole-branch-review-verdict.md` Important #1 (line 28) + `-round-2.md`; fix commit `3c24845`.
- **Evidence:** whole-branch verdict Important #1: "Core emits English prose in template-error params - fails the plan's own exit criterion and spec 8.4. `validate.rs:360,364,367`: `format!("unknown filter: {name}")` ... per that constraint the spec wins. Fix: ... `kind` (`unclosed-brace`/`empty-field`, a code-like token, not prose) ... catalog uses a Fluent selector on `$kind`." Journal: "Template-error params carried English prose out of core, violating the plan's own exit criterion (final review; spec-wins rule applied)." Commit `3c24845` = "fix: final-review findings - prose-free template errors ...". Every clause of the occurrence's evidence (the `format!("unknown filter: ...")` breach, spec 8.4, spec-wins, restructure to code-like kind/name tokens via a Fluent selector) matches verbatim.
- **Judgment:** Real, exactly on-topic, distinct lifecycle action (a real violation caught at final review and enforced against the plan's own code). The strongest occurrence in the cluster; the pattern earned enforcement teeth here.

### 4. 2026-07-10 reinforced - "plan Global Constraints + Task 6 (UnsupportedSource)" - SURVIVES (with date caveat)
- **Artifacts:** `docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md` Global Constraints (line 14) + **Task 6: `UnsupportedSource` diagnostic (D21)** (line 536ff); commit `2b08de4`.
- **Evidence:** Plan 3.5 Global Constraints line 14: "Core emits no user-facing prose: diagnostics are `DiagCode` + `params` only ... A new `DiagCode` without a matching `.ftl` message fails `crates/muxsmith-cli/tests/catalog_completeness.rs`." - verbatim match to the occurrence's evidence. Task 6 adds `DiagCode::UnsupportedSource => "unsupported-source"` and runs `catalog_completeness`; commit `2b08de4` ("feat(planner): clean UnsupportedSource diagnostic (D21)") added both the DiagCode and the ftl line `unsupported-source = mkvmerge identified this file but its container is not a supported muxing source.` in the same commit - i.e. the catalog-completeness reinforcement of the prose-free rule fired exactly as the occurrence describes.
- **Caveat (non-fatal):** The occurrence is dated **2026-07-10 but the artifact is 2026-07-09** (commit `2b08de4`; Plan 3.5 = session 4, 2026-07-09). This is a one-day slip from the cluster's disclosed plan-cycle dating heuristic ("plan-3/4 = 07-10"), which mis-binds the Plan 3.5 cycle. The referenced event ("Task 6 (UnsupportedSource)") is real, unambiguous, and fully on-topic - it points at exactly one task in the repo (no Plan 4/5/5.5 "Task 6" concerns UnsupportedSource: those are mkvtoolnix-CI / job-logs / zero-track-warning respectively). A misdate is not a misattribution; the occurrence survives.

### 5. 2026-07-10 reinforced - "plan Global Constraints + task-3-review-verdict.md" - SURVIVES
- **Artifacts:** `docs/superpowers/plans/2026-07-09-plan-4-executor-run-queue.md` Global Constraints (line 14); `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-3-review-verdict.md` (Plan 4 Task 3 = "FIFO queue, `JobEvent` stream ...").
- **Evidence:** Plan 4 Global Constraints line 14: "Core emits no user-facing prose: diagnostics/events are codes + params ... A new `DiagCode` without a Fluent message fails `catalog_completeness`." Plan 4 task-3 verdict line 26: "Core prose-free (events carry pass-through mkvmerge text, not core-authored UI prose)." - a verbatim match to the occurrence's evidence. The reviewer actively checked and affirmed the prose-free boundary on the new `JobEvent` surface.
- **Judgment:** Real, on-topic, distinct plan/era (Plan 4) and a distinct facet (the rule extended to and enforced on the executor's `JobEvent` stream: events, like diagnostics, carry only pass-through third-party text, never core-authored prose). Date matches (Plan 4 close = 2026-07-10). Disambiguation: several `task-3-review-verdict.md` files exist, but the JobEvents content and the Plan 4 Global-Constraints wording pin this to Plan 4's.

---

## Duplication / fabrication check

- **Fabricated:** none. All five refs resolve to real artifacts in the repo (three commits - `61249f9`, `a671949`, `3c24845`, `2b08de4` - verified in history; four plan/spec sections; two verdict files; one journal entry).
- **Misattributed:** #4 carries a one-day date error (07-10 vs actual 07-09) from the cluster's plan-cycle dating heuristic, but points unambiguously at the correct, on-topic event (Plan 3.5 Task 6 / commit `2b08de4`). Not fatal.
- **Duplicate (same event double-booked):** #2 is a same-day (2026-07-08), same-kind (`decided`) re-documentation of #1's single design decision, resting on the same spec-authoring commits. Collapsed to one `decided` occurrence. This is the one departure from the cluster's disclosed "distinct artifact = distinct occurrence" rule, made because the two entries are the same lifecycle action on the same day (unlike the sibling's legitimately-kept find-vs-fix, which are two actions in time).

## Robustness

The pattern recurs across **Plan 1 (decided + violated-corrected), Plan 3.5 (reinforced, new DiagCode -> catalog gate), and Plan 4 (reinforced, JobEvent surface)** - three plans, three eras, three lifecycle actions. It is separately restated in the standing `docs/CONVENTIONS.md` ("Diagnostics through the catalog") and has a live tracked residual breach (`i18n-17`), i.e. the rule is load-bearing enough that its one open violation is itself a tracked non-decision. Promotion is robust under every counting rule: 5 under the cluster's disclosed reading, 4 under the strictest same-day-decision collapse, never below the threshold of 3.

## Result

- **verified_count = 4** (dropped #2 as a same-day same-kind double-cite of #1; #1, #3, #4, #5 survive).
- **Verdict: REDUCED** (one occurrence dropped; 4 >= 3 survive; promotion stands). Equivalent to CONFIRMED under the cluster's own disclosed counting rule (count 5); the promotion decision is invariant to the call.
