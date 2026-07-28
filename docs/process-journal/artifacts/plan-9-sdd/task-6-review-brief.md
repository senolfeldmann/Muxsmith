# Task 6 review brief - Plan 9

**Role:** independent reviewer of Plan 9, Task 6 (JobsView made mountable in the
e2e mount harness, a reactive-props hook, the four ruled D23 reset/gating tests
including the worker-panic render, and the hoist of three local `name()` helper
copies into one shared export. D104; amendment 2; **and amendment 5**, which
re-cut item 2's second assertion mid-task). You did not write this code. Model
tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.** The task is one commit, `a2c1028`, and it is
  HEAD. Three commits landed before it while the task was mid-flight - the
  amendment (`1e0dbd8`) and two house commits - so grade the current tree.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t6rev-independent/`
  (create it). Never re-run an instrument another agent wrote, never a shared
  default path, never a path the reports name. Any absence check needs its own
  fire; the local `grep` is **ugrep 7.5.0**, where `\b` plus bounded repetition
  under `-E` returns zero silently - use `-P` or a script.
- **The e2e suite runs against BUILT bundles.** `pnpm test:e2e` regenerates
  `e2e/.generated/*` (two `vite build` steps) before Playwright runs; a bare
  `pnpm exec playwright test` does not, and the harness change under review
  lives in exactly those bundles. Any mutation you fire needs the rebuild
  between edit and run, and your pasted evidence must show it
  (ledger `frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`).
- If you mutate anything, restore non-interactively (`git checkout --`, never a
  bare `cp` - it is aliased interactive here) with a baseline taken first, and
  prove the restore.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (the v1 spec).
2. The Plan-9 design: **D104 in full, including its amendment-5 rider**, section
   0 note 3, and the `## Amendment log`. The rider is the single home for item
   2's second assertion; anything that restates it elsewhere is drift.
3. The plan's **Task 6** as amended (Files list, Steps 1-5, "Must not decide",
   amendment 2's hoist clauses, amendment 5's Step-2 qualifier).
4. The ROADMAP anchor's harness-scope correction paragraph and the `name()`
   trigger's NOT-FIRED / consumed-early record.
5. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

The implementer's brief and its report (`task-6-report.md`, whose sections 1-9
are the pre-ruling record and whose `## Fix round` is the post-ruling one) are
evidence, not ground truth. So are `amendment-5-report.md` and
`amendment-5-verdict.md`.

## Dimensions

1. **Contract compliance where D104 fences text**: the `mount-entry.ts` glob and
   `resolvePath` fence, the hook's name and merge semantics, the four test names
   (the plan's, character for character), and item 2's replacement pair as the
   rider writes it. Build your own comparison; the implementer's own diff of the
   fence is evidence, not proof.
2. **The four tests against D104's numbered items 1-4.** Each asserts what its
   item requires, and item 2 asserts the ruled observable rather than a weaker
   neighbour. Ask of each: which real code path makes this test red? A test that
   only a mutation can fail is a shape guard, not a behavioural assertion
   (ledger `a-fired-check-still-needs-a-reachable-failing-input`) - and if you
   find one here, say so plainly, because these four are the D23 item's entire
   coverage.
3. **The hoist is claimed to be a PURE move.** Verify both halves yourself: the
   helper body and the travelled rationale comment byte-identical to the deleted
   smoke copy (from the git object, not the working tree), and the three
   migrated files behaviourally unchanged. The rationale comment is the
   load-bearing half - the plan says losing the `exact: true` reason in the move
   is the one cost this step must not pay - so check that the comment that
   travelled is smoke's and not one of the two shorter mirrors.
4. **The absence check and its green state.** `grep -rn "^function name(" e2e/*.spec.ts`
   -> 0 with the survivor findable one glob over. Re-derive it with your own
   instrument and your own fire; a zero from a malformed pattern is
   indistinguishable from a real absence.
5. **Latitude, both forms, including the inverse.** The implementer named seven
   divergences (D-1 to D-7) and six concerns. Three concerns survive the
   amendment ruling and are your adjudication questions below; the rest are
   still yours to check as divergences. Look hardest at what was composed rather
   than fenced: the spec-local helper names, the fixture values, the describe
   title, and the post-resolve barrier.
6. **House dimension**: Tier-2 conformance;
   `latitude-carveout-zero-content-structural-forks` (the Files-list boundary
   over files, the import add/drop cases, and the stop list);
   `a-returning-task-may-commit-the-subset-that-survives-every-option` (the
   implementer declined to commit a subset - was its reading right?);
   `proc-57-briefs-not-ground-truth` (it re-derived every anchor - re-measure a
   sample); `proc-normative-count-recomputed`.
7. **The no-work-needed check**, standing: run every premise the report uses to
   conclude something needs no work, no test or no further measurement. Named
   ones: that the four-condition test-coverage rule cannot fire because the diff
   creates no user-visible behaviour; that the `name()` trigger's NOT-FIRED
   record still stands because the new spec consumes no `name()`; that the
   existing mount specs' passing is sufficient evidence the harness hook
   regressed nothing; that `e2e/mount.ts` needed no change.
8. **Verification quality**: re-run the full bar yourself (`pnpm lint`,
   `pnpm test:e2e`) and recompute every aggregate. The stated ones: **68 passed**
   e2e against a pre-task baseline of 64; **39** `test result:` lines on the
   Rust side, unchanged because this task touches no Rust; **10** spec files
   after the new one; three deleted helper copies; two `FluentVariable` hits per
   migrated file before, zero after.
9. **Amendment-5 hygiene, since the ruling landed mid-task**: the header doc
   comment at `:36-40` was required to POINT at the rider, not restate it. Check
   that it points, that its wording is accurate (it claims test 2's transition is
   asserted at the same gating condition from its other side), and that no
   fourth restatement of the falsified vehicle survives anywhere in the tree.

## Adjudication questions (one explicit verdict each, not pre-rated)

Three come from the implementer and survive the ruling; three are mine.

1. **D-1, the post-resolve barrier in test 1.** After the first summary
   assertion the test does one round trip through the page
   (`page.evaluate(() => document.readyState)`) and re-asserts, on the argument
   that a retrying assertion alone cannot distinguish "displayed and kept" from
   "displayed in flight". Warranted, or noise to cut?
2. **D-3, the joblog note asserted alongside the summary line in test 1.** D104
   fixes `joblog_status: "unavailable"` in the fixture but enumerates no
   assertion on the note it produces. Inside the enumeration as an additive
   consequence of a fenced fixture value, or beyond it?
3. **D-2, the panic fixture's non-fenced fields** copied from
   `recover_panicked_worker`'s real construction rather than invented. Right
   call, or should unfenced fields be minimal placeholders so the fixture cannot
   silently drift from the emitter?
4. **D-5, the props ref's lifetime**: it lives inside `mount()`, so
   `__muxsmithSetProps__` is re-bound per mount, mirroring `__muxsmithModel__`.
   D104 fences the hook's name and merge semantics, not its lifetime. Sound, or
   does per-mount rebinding create a hazard the enumerated tests do not expose?
5. **The commit decision.** The implementer left all seven files uncommitted
   while returning NEEDS_CONTEXT, on the ground that the verification bar could
   not be green with the failing spec in the tree and that the plan's fenced
   commit message names exactly the artifact that was missing. The controller
   endorsed it. Was that the right reading of
   `a-returning-task-may-commit-the-subset-that-survives-every-option`, or
   should the six complete files have been committed?
6. **Test 2's replacement pair in situ.** The rider argues the pairing is
   load-bearing because a bare `toHaveCount(0)` would pass against a view that
   never mounted. The amendment reviewer noted the first assertion already
   proves the view mounted, so in situ the bearer buys something else. Does the
   pair as committed assert what item 2 needs, and is there any state that would
   satisfy it while `runActive` is still true?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-6-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with file:line, the evidence you ran and the exact
required change; the six adjudications; an evidence appendix naming your
instrument paths; and a HARVEST including what Task 7 and the plan close must
carry and any observation worth a ledger entry.
