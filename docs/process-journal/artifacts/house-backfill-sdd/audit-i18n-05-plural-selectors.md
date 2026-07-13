# Audit: i18n-05-plural-selectors (PROMOTION candidate)

- **Cluster id:** `i18n-05-plural-selectors`
- **Kind / domain:** pattern / i18n
- **Claimed count:** 3 (all occurrences), `promoted: true`, `promoted_at: 3`
- **Statement audited:** Count-dependent user-facing messages pluralize via Fluent CLDR plural selectors (`{ $n -> [one] ... *[other] ... }`), not an `error(s)`/`(s)` provisional. First applied to run-job-warning, later extended to five keys replacing the `error(s)` provisional, with singular+plural renderer assertions.
- **Verdict:** CONFIRMED (3/3 occurrences survive)
- **Audited:** 2026-07-13

## Method

Each cited ref opened in `/home/senol/Git/Muxsmith` and checked for three failure modes: fabricated (ref/quote does not exist), misattributed (ref is about a different topic), duplicate (same event as another listed occurrence). Pattern reality cross-checked against the live locale files and commit history.

## Occurrence-by-occurrence

### Occ 1 - 2026-07-10, kind `deferred`, ref `task-8-review-verdict.md`

- **Resolved ref:** `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-8-review-verdict.md` (the cluster ref omits the plan; only plan-4's task-8 verdict carries the quoted text - the plan-1/3/5 copies do not, so the ref resolves unambiguously and is not fabricated).
- **Quote match:** line 55, Important finding: *"the defect is in the brief's mandated text, not the code; cosmetic, v1 CLI; the human decides whether to amend the spec."* Byte-matches the evidence string.
- **Topic match:** The finding is exactly the plural-selector question. The reviewer notes `run-job-warning` renders "1 warnings", that the implementer *"correctly did not introduce Fluent pluralization"* (would deviate from locked brief text), and kicks the amend-the-spec decision to the human.
- **Kind match (`deferred`):** Correct. The reviewer explicitly declines to fix and defers to the human - a genuine deferral of the plural-selector decision, not a fix.
- **Verdict:** SURVIVES. Real, correctly attributed, correct kind.

### Occ 2 - 2026-07-10, kind `decided`, ref `memo D15 amendment 2026-07-10 + commit 79f0447`

- **Memo:** `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`, amendment appended to the end of the D15 section (immediately before `## D16`): *"Amendment 2026-07-10 (Şenol): run-job-warning pluralizes the warning count via a Fluent plural selector (plan's locked text rendered '1 warnings')."* Byte-matches the evidence string; the "D15 amendment" attribution is accurate.
- **Commit:** `79f0447` exists - `fix(cli): pluralize run-job-warning count via Fluent plural selector`, dated 2026-07-10 00:17 +0200, touching `run.rs`, `i18n.rs`, `cli.ftl`, and the plan-4 design-decisions memo. The commit both records the amendment and implements it.
- **Kind match (`decided`):** Correct. Şenol amended the locked spec (owner decision) and the change was committed - the resolution of Occ 1's deferral.
- **Duplicate check vs Occ 1:** Not a duplicate. Different artifacts (memo + commit vs a review verdict), different actor (owner decision vs reviewer flag), different kind (decided vs deferred). This is the far end of a legitimate deferred->decided transition, not the same event recorded twice.
- **Verdict:** SURVIVES.

### Occ 3 - 2026-07-12, kind `decided`, ref `task-19-verdict.md / plan T19`

- **Resolved ref:** `.superpowers/sdd/plan-5.5/task-19-verdict.md`.
- **Quote match:** line 16, Spec Compliance: *"five keys on CLDR [one]/\*[other]; zero (s) patterns remain; singular+ plural renderer assertions per key"*. Matches the evidence string, and the statement's "singular+plural renderer assertions" claim is corroborated in the same line.
- **Topic / provisional-replacement check:** The verdict's scope-growth section confirms the replaced provisional was real ("the brief's own (s)-grep covers them", "leaving them bare-count would be under-scoping", "zero (s) patterns remain"). Independently verified against the live tree: `grep '(s)'` over `locales/en/*.ftl` returns nothing, and the CLDR `[one]/*[other]` selectors are present across `cli.ftl`, `gui-batch.ftl`, `gui-jobs.ftl`, `diagnostics.ftl` in both `en` and `de`.
- **Kind match (`decided`):** Correct. Task 19 was approved (fix wave controller-verified, 12/12 i18n tests incl. a new pin test, lint+fmt clean) - implemented and accepted.
- **Duplicate check:** Distinct subject (five diagnostics/summary keys) from Occ 1/2's single `run-job-warning` key; distinct plan (5.5) and date.
- **Verdict:** SURVIVES.

## Skeptical note (does not change the verdict)

Under the audit's stated drop criteria (fabricated / misattributed / duplicate), all three survive: every ref exists, quotes match byte-for-byte, kinds are correct, and no two are the same event.

One honesty caveat on what the count represents: Occ 1 and Occ 2 are the deferral and the resolution of the **same** subject (`run-job-warning`). So the pattern has been *applied* to two distinct message-subjects - `run-job-warning` (Occ 1->Occ 2) and the five task-19 keys (Occ 3) - while the count of 3 comes from scoring the deferred and decided lifecycle stages of the first subject separately. That is legitimate under an occurrence-ledger model that treats `deferred` and `decided` as distinct events (and the pattern's own narrative frames run-job-warning as "first applied" and the five keys as the "extension"). If the promotion bar were instead "N independent application sites," this sits at 2, not 3. The recurrence is genuine either way - the pattern is live in the shipped locale files and settled - so the standing-convention promotion is sound; the count is real, not fabricated.

## Result

- verified_count = 3
- Verdict = **CONFIRMED** (>=3 survive; promotion stands)
