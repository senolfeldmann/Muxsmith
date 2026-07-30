# Task A1 review brief - Plan 11

**Role:** independent reviewer of Plan 11, Task A1 (W5: `BUILDING.md` loses its
three positional gate ordinals and its one over-80 prose line, in one edit). You
did not write this change. Model tier: mid (dispatch model: Opus 5).
Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content in short form as your final message.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Absolute paths, **foreground runs only**.
- **The work sits in a worktree, not on `master`:**
  `/home/senol/Git/muxsmith-plan11-a`, branch `plan-11-stream-a`, head
  `a0d5d3e`, base `5378264`. `master` in the main worktree does NOT contain this
  change and is not where you grade it. A second stream is running concurrently
  in `/home/senol/Git/muxsmith-plan11-b`; do not read or touch it.
- **Read the files, not a commit hash.** The tree is at `a0d5d3e`; the
  controller has made no product edit since.
- **Independent instruments.** Build every harness you use - scripts, mutated
  copies, fixture files - under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a1rev-independent/`
  (create it). **Never re-run an instrument the implementer wrote, and never use
  a shared default path**: agents in one session converge on the same names, so
  a re-run that silently executes the implementer's own script produces
  agreement by construction. Any absence-shaped check you rely on needs its own
  fire, built by you.
- If you mutate a tracked file to fire something, take the baseline FIRST
  (`sha256sum`), restore non-interactively (`command cp -f`; a bare `cp` is
  aliased interactive here and hangs with the tree still mutated), and prove the
  restoration. **Note the trap this task already hit:** `git checkout -- <file>`
  restores from HEAD, which is correct here (the work IS committed) but was not
  correct at the point in the task where the implementer needed it.
- The tree must be byte-identical to `a0d5d3e` when you finish. Prove it.

## Ground truth, in precedence order

1. The plan,
   `/home/senol/Git/muxsmith-plan11-a/docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`:
   its **Global Constraints**, the **Authoring-time verification** section's
   block "Item 5's corpus: three ordinals and one long line, all reproduced",
   **Task A1** in full (Files list, Steps 1 through 5, "Must not decide"), and
   acceptance rows **W5-a through W5-e**, which are the halves this task must
   produce.
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 5.
3. `docs/ROADMAP.md`, the **"Gate-count derivation has no check"** section
   including its MEASURED block and its "A neighbouring class" paragraph.
4. The four house-knowledge YAML files (`docs/decision-ledger.yaml`,
   `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`) as ground truth alongside them; cite entries
   by id.

The implementer's brief (`.superpowers/sdd/plan-11/task-a1-brief.md`, **including
its ERRATUM block**) and its report (`.superpowers/sdd/plan-11/task-a1-report.md`)
are **evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/review-5378264..a0d5d3e.diff`
carries the commit list, the stat summary and the full diff with context. Read
it in one call rather than re-deriving the range with git commands.

## Dimensions

1. **Contract compliance, character for character.** Both fenced replacements of
   Step 2, compared byte for byte against your own extraction from the plan - not
   by eye. The deletion in (a) is a pure deletion; the paragraph in (b) is a
   six-line block replaced by a seven-line block. Check that the four named
   commands are in fact the first four of `BUILDING.md`'s own Rust gate block, in
   its order, read out of the file rather than taken from the plan.
2. **Scope.** `git diff -U0` over the branch: every changed line must lie in one
   of Step 2's two regions. No marker line (the `gate-total` marker and the three
   `gate-block` markers), no fence line, no line inside a fence, and not the
   canonical gate-total sentence may appear as changed. `git diff --stat` names
   exactly one file.
3. **Re-measure both absence checks with your own expressions**, on the end state
   and on the pre-state (`git show 5378264:BUILDING.md`). Check O must go 3 -> 0
   and check L 1 -> 0. If your counts differ from the plan's or the
   implementer's, that is a finding of the first order.
4. **Fire what you rely on.** An empty grep and a broken grep look identical.
   Build your own known-present case for each expression rather than reusing the
   implementer's, and satisfy yourself that `scripts/ledger-lint.py` still reads
   this file - make it fire yourself, on your own copy or with your own
   mutation, and restore under proof.
5. **The 80-column norm has no linter**, so the end-state claim rests on a
   measurement. Re-run a fence-aware length pass with your own script and report
   the longest non-fenced line and its length.
6. **Latitude, both forms**, including the inverse: did the implementer decide at
   the keyboard something that should have returned as NEEDS_CONTEXT? Conversely,
   did it return or omit something the plan had already settled? Its three
   numbered concerns are where to look hardest.
7. **House dimension.** Tier-2 conformance, by id - in particular
   `ledger-lint-runs-before-every-push`, `proc-wrapped-prose-quote-grep`,
   `proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`,
   `proc-normative-count-recomputed`, and the positional-reference entry in
   `docs/decision-ledger.yaml` whose statement uses "gate part 6" as one of its
   own examples. Flag deviations by id.
8. **The no-work-needed check.** Wherever the report concludes that something is
   unnecessary, already covered, or cannot happen - run the premise that makes it
   so. Do not weigh it.
9. **Blast radius.** Three further tasks (A2, A3, A4) execute serially in this
   same worktree after you. Satisfy yourself that nothing in this diff can turn
   a gate part red on a tree those tasks will legitimately produce.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **The unfireable soundness control.** The plan's Step 3 names
   `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` as the
   known-present control for absence check O, and the implementer reports that
   the file carries only SPELLED ordinals, so the digit-form expression returns
   nothing there. The controller reproduced that independently: the digit form
   returns 0 in that file and the spelled form returns 3, and the plan's own
   authoring section names the same file as the control for the SPELLED sweep.
   The implementer did not adjust the plan; it used the authoring section's own
   control for this expression plus a live digit-form file. **Your verdict, in
   both directions: is acceptance row W5-a satisfied by the substituted control,
   or does the plan need an amendment before this task can be called done?** If
   you judge an amendment necessary, say precisely what it must say; the
   controller routes it to the plan's author and original reviewer, and does not
   patch it in.
2. **The restore mechanism.** The brief prescribed `git checkout -- BUILDING.md`
   as the restore after a fire, which the implementer refuted as
   deliverable-destroying at that point in the task; the controller has accepted
   that and recorded an erratum on the brief. The implementer substituted a
   sha256 baseline of the edited file plus exact-inverse string mutations. **Was
   that substitution the correct handling of a wrong instruction, or was the
   fork one that should have returned as NEEDS_CONTEXT before proceeding?**
3. **The third live consumer.** The implementer reports `docs/ROADMAP.md` around
   line 2725 carrying a fenced forward-looking replacement text that cites
   "BUILDING.md, Rust gate part 6", where the plan's authoring section claims
   exactly ONE live consumer of that wording. The controller reproduced it and
   will dispose of it as a close action, since the ROADMAP is controller-owned.
   **Your question is narrower: was surfacing it the right handling, and is the
   plan's "one live consumer" claim a defect the acceptance map inherits?**
4. **The ledger neighbour.** `docs/decision-ledger.yaml`'s positional-reference
   entry uses "gate part 6" inside its own statement as an ILLUSTRATIVE EXAMPLE
   of the class it describes, not as a citation of `BUILDING.md`'s current text.
   The implementer flagged it as a lower-confidence controller judgment call.
   **Does this task's edit make that statement stale, or is an example of a
   pattern immune to the pattern's removal at one site?**
5. **What the task did NOT do.** The plan forbids touching house-knowledge YAML,
   the ROADMAP and `ci.yml`'s spelled ordinal, and requires all three to be
   surfaced instead. Verify each was surfaced with its quoted clause and none was
   edited, and say whether the surfacing is complete enough for the controller to
   act on without re-deriving it.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a1-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings (Critical / Important / Minor), each with
  `file:line`, the evidence you ran, and the exact required change.
- The five adjudications, one explicit verdict each.
- An evidence appendix naming your instrument paths and the commands you ran.
- A **HARVEST** section: observed dominant patterns, repeated rejections, and
  anything the remaining three stream-A tasks must carry. The controller is the
  single writer into the house-knowledge files; you surface, you do not write.

Your final message carries the same content in short form: verdict, the findings
as one-liners with severity, the five adjudications, and the verdict file path.
