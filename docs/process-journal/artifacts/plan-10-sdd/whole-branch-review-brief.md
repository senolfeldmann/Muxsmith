# Whole-branch review brief - Plan 10 (plan close)

**Role:** independent whole-branch reviewer for Plan 10, the last planned
product package before Muxsmith's 1.0 tag. You did not write any of it. **Model
tier: top** - this is the one role the top tier serves in this project
(`proc-03-model-assignment`). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## What this review is for

Five task reviews already happened, each scoped to its own task and each with
its own independent instruments. Your job is the thing none of them could do:
judge the branch AS A WHOLE against the contract, and find what falls between
tasks. Concretely -

- cross-task consequences no single task's diff shows;
- claims the package makes about ITSELF that are now false (a count, a scope
  statement, a "this class is closed");
- coverage the plan promised and the sum of the five commits does not deliver;
- anything in the five task verdicts that was disposed too easily.

Do not re-litigate settled non-findings from the task reviews. Their verdicts
are inputs, not targets.

## Preamble (binding)

- Never call session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.**
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/wbrev-independent/`
  (create it). Never re-run an instrument another agent wrote; never a shared
  default path.
- If you mutate a tracked file, baseline first (`sha256sum`), restore
  non-interactively (`git checkout --`; a bare `cp` is aliased interactive
  here), prove it.
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty. Capture exit codes
  directly.
- **Prove tree identity per FILE against blobs** at the end, not by a clean
  `git status`.

## Ground truth, in precedence order

1. The v1 spec, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` -
   authoritative over plan and design on conflict.
2. The plan,
   `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`, in full:
   Global Constraints, the work-item coverage map, the **acceptance map** (22
   halves), the five tasks, the plan-close section, and Amendments 1 and 2.
3. `.superpowers/sdd/plan-10/plan-brief.md` - the coverage ground truth for this
   package, which has **no design document** by owner approval.
4. `docs/ROADMAP.md`, the entries the brief cites: the Plan-9 anchor's D102
   paragraph, "Gate-count derivation has no check", "Docs accuracy", and the
   README, Renovate and OWNER QA PASS entries in the Pre-1.0 release gates
   section.
5. The four house-knowledge YAML files; cite entries by id.

The five task reports and verdicts, and the progress tracker, are **evidence,
not ground truth**.

## The branch

Package: `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/review-whole-branch.diff`
(the full branch diff with context). The branch is `master` itself - the plan
ruled serial execution in the main worktree, no branches, no worktrees - so the
range is the session's base commit through HEAD.

Two commit KINDS are in that range and you should judge them differently:

- **Five task commits** (the product changes): `ddb8f42` gate-count invariant,
  `35bc363` D102 producers, `630d418` `renovate.jsonc`, `e657263` + `845cf89`
  the documentation pass, `1a23283` the comment sweep.
- **Six controller commits** touching only `docs/ROADMAP.md` and the four house
  YAML files: tracker dispositions and the verdict harvest. These are process
  artifacts, written by the controller as the single writer. Judge them for
  ACCURACY (does a recorded measurement match the tree? does a routed item name
  a real vehicle? does a promotion satisfy its matrix rule?), not for style.

## The deferred and disposed findings you are asked to triage

`.superpowers/sdd/plan-10/progress.md` carries every finding from the five task
reviews with its disposition. Two are explicitly deferred TO YOU:

1. **Task 4 fix round, new NIT:** the rewritten exit-code sentence opens
   "Interrupt any of them", and *them* has no antecedent inside its paragraph -
   the nearest bindable plural is "your scripts", which is a coherent but wrong
   reading. Two-word repair proposed: "any subcommand". Rule whether it must
   land before this package closes.
2. **Task 5, MINOR:** two ragged mid-sentence re-wraps
   (`e2e/smoke.spec.ts`, `src/views/EditorView.vue`) against a report claim that
   no line was left ragged. Cosmetic; rule whether it lands now.

Also re-examine, briefly, the findings the controller disposed WITHOUT a tree
change (they are listed per task in the tracker with their rulings). If any was
disposed too easily, say so.

## Dimensions

1. **Acceptance map, walked end to end.** Twenty-two halves, each with a named
   producer. For each: does the producer exist in the branch, and does it
   actually produce the observable the row claims? The row that is NOT
   machine-verifiable (W4-c, Renovate activation) must be OPEN at the close, not
   claimed. **This is the dimension the whole review exists for** - the plan's
   own history is that a producer named for a whole observable covered one side
   of it.
2. **Cross-task consequences.** Five commits, one tree. Task 2 mutated a file
   Task 5 later edited; Task 4 changed prose that Task 5's comments cite; Task 1
   changed a gate part every later task ran. Look for the semantic conflict that
   no single task's review could see.
3. **Self-referential claims.** The package states things about itself - counts,
   scopes, "the class is closed", "the corpus is 24 lines across 16 files". At
   least one such claim was already found false at task-review time (the sweep's
   class is not closed tree-wide; a member survives outside the corpus's file
   selector). **Hunt the rest**, in the commits, in the ROADMAP dispositions,
   and in the house-knowledge occurrences the controller wrote.
4. **The gate and its new check.** `scripts/ledger-lint.py` gained a check over
   `BUILDING.md`. Verify the whole gate as `BUILDING.md` enumerates it, green,
   with your own run, and satisfy yourself the new check cannot mis-fire on
   states this repository will legitimately reach.
5. **House-knowledge accuracy.** The controller appended occurrences and two
   new Tier-1 entries per verdict, and promoted one entry to Tier 2 on its third
   occurrence. Check the promotion against the `source x nature` matrix
   (agent-emergent + process promotes at count 3), check `count ==
   len(occurrences)` holds where the controller wrote, and check that each
   occurrence's `ref` cites a real artifact and that its statement carries the
   event STRICTLY rather than by a stretched reading.
6. **What the close is allowed to claim.** Tier-2
   `owner-manual-qa-gates-the-1-0-release` binds: no completeness claim about
   1.0 may be made before the owner's manual QA and bug-hunting pass has run,
   however short the remaining list looks. Check that nothing in the branch -
   commit messages, README, ROADMAP, journal - claims or implies otherwise.
7. **The no-work-needed check**, over the whole branch: wherever any artifact
   concludes a guard, a test, an enumeration or a repair is unnecessary, run the
   premise.
8. **Spec conformance.** The package changed no product behaviour by design.
   Verify that: no shipped behaviour differs, and the two new tests assert the
   contract the spec states rather than the implementation's current shape.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/whole-branch-verdict.md`:

- Verdict: READY / READY_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings with `file:line`, the evidence you ran, and
  the exact required change.
- An explicit ruling on each of the two deferred items above.
- Your walk of the acceptance map: one line per half, with its verdict.
- An evidence appendix naming your instrument paths and commands.
- A **HARVEST** for the close: what this package teaches that outlives it.

Your final message carries the same in short form.
