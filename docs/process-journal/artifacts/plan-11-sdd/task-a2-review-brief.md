# Task A2 review brief - Plan 11

**Role:** independent reviewer of Plan 11, Task A2 (W2: the two line-number
citations that survive outside Plan 10's source selector). You did not write this
change. Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content in short form as your final message.

## Preamble (binding)

- Never call session-relocation tools. Absolute paths, **foreground runs only**.
- **The work sits in a worktree, not on `master`:**
  `/home/senol/Git/muxsmith-plan11-a`, branch `plan-11-stream-a`, head `5d305a2`
  over the previous task's `a0d5d3e`. The stream's base is `5378264`. Do not read
  or touch `/home/senol/Git/muxsmith-plan11-b`.
- **The worktree's `docs/ROADMAP.md` is deliberately older than `master`'s.** The
  controller has since written a "FIRED ... RE-DEFERRED" paragraph into the
  ROADMAP on `master`; the branch picks it up at merge. If you need that text,
  read it in `/home/senol/Git/Muxsmith` - and note that reading the main worktree
  is fine, writing to it is not.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a2rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never use a
  shared default path. Any absence-shaped check you rely on needs its own fire,
  built by you.
- The tree must be byte-identical to `5d305a2` when you finish. Prove it.

## Ground truth, in precedence order

1. The plan, in the worktree:
   `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md` -
   Global Constraints, **Task A2** in full, acceptance rows **W2-a through W2-f**,
   and the authoring section's block "Item 2's corpus: TWO surviving members, not
   one". **The plan is being amended concurrently on `master` (Amendment 5) for
   defects in OTHER tasks; grade against the worktree's copy and raise anything
   that looks inconsistent rather than chasing the amendment.**
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 2.
3. Tier-2 `comments-locate-by-symbol-never-by-line-number` in full - its handle,
   its SCOPE BOUNDARY sentence and its "WIDENED BY OWNER RULING 2026-07-29"
   clause - plus `code-comment-line-citations-drift`.
4. The four house-knowledge YAML files; cite entries by id.

The implementer's brief (`task-a2-brief.md`) and report (`task-a2-report.md`) are
**evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/review-a0d5d3e..5d305a2.diff`
carries the commit list, the stat summary and the full diff with context.

## Dimensions

1. **Contract compliance, character for character.** Both fenced replacements
   against your own extraction from the plan. The A1 reviewer's method is
   recommended and cheap: reconstruct the end state from `a0d5d3e`'s files plus
   the plan's fenced substitutions in your own process, and compare byte for byte
   with the committed files. That single equality answers contract compliance and
   scope together.
2. **The symbol is the right one.** Replacement (a) names `QueueOpts::jobs` and a
   private `worker_count` helper, derived from the CITING COMMIT'S PARENT rather
   than from what the cited line holds today. Verify both reads yourself:
   what line 73 held at that parent, and what the symbol's doc comment says now.
   A wrong symbol here is a silent regression of the very class the task closes.
3. **Re-measure both corpus expressions** on the pre-state (`a0d5d3e`) and the end
   state, with the extension alternation DERIVED from the tree rather than typed.
   Expected 1 -> 0 each. Build your own known-present control for each before
   trusting a zero.
4. **The blind spot both expressions share.** Both require a colon, so neither
   sees a prose locator. Re-run that sweep yourself and classify every hit; the
   implementer reports two, both test data.
5. **Comment-only, in both files, and the fixture's DATA unmoved.** The fixture is
   consumed by a round-trip test, which would pass over a changed value that still
   round-trips - so the diff, not the test run, is what proves the data did not
   move. Check that the report attributes it that way.
6. **The workflow still parses.** No gate part reads `.github/workflows/ci.yml`,
   so the local gate stays green over a broken workflow and the break would first
   appear on the plan close's single push.
7. **The count `17` in the fixture comment is untouched**, deliberately - a
   different fact with its own consumers in `profile_save.rs`.
8. **Latitude, both forms**, including the inverse.
9. **House dimension**, by id. **The no-work-needed check**: run any premise the
   report uses to conclude something was unnecessary.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **The rider the task was forbidden to consume.** A ROADMAP rider gated on "the
   next `ci.yml`-touching change" prescribes replacement text citing a
   `BUILDING.md` gate ordinal that Task A1 removed; the controller re-deferred it
   and forbade this task from applying it, and the implementer verified the
   premise itself rather than borrowing it. **Was the prohibition correctly
   observed - are the rider's target lines byte-identical to their pre-state - and
   does leaving them untouched conflict with anything the plan requires of this
   task?**
2. **A positional locator that drifted.** The plan's Step 5 and Read-first list
   name the ROADMAP's "Docs accuracy" **first** entry; the implementer reports,
   and the controller reproduced, that the target is now the **second** bullet
   because another entry was filed above it while the plan was being written.
   **Did the implementer surface the right entry despite the stale locator, and
   is the surfacing complete enough for the controller to act on it without
   re-deriving it?**
3. **The scope boundary.** The task sweeps tracked files outside `docs/` and
   deliberately leaves process artifacts citing `<file>:<line>` at a named commit.
   **Is the boundary applied as the convention states it - by the artifact DOING
   the citing rather than the artifact cited - and does the class-closure claim
   the plan close may then make carry its surface honestly?**
4. **What was NOT edited.** No file under `docs/`, no line of code or data, no
   `runs-on`, no pin, no `run:` or `name:` value. Prove it at the tree rather than
   from the report.
5. **The free `ledger-lint` run this task deliberately did not make.** A1's review
   established that a green `ledger-lint` says nothing about a prose or comment
   deliverable. The implementer therefore did not run it and does not cite it.
   **Was that right, or does a task editing a tracked file owe the cheapest
   available integrity check regardless of what it proves?**

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a2-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings, each with `file:line`, the evidence you ran,
  and the exact required change.
- The five adjudications, one explicit verdict each.
- An evidence appendix naming your instrument paths and commands.
- A **HARVEST** section, including anything Tasks A3 and A4 must carry.

Your final message carries the short form only.
