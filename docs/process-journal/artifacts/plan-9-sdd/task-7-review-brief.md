# Task 7 review brief - Plan 9

**Role:** independent reviewer of Plan 9, Task 7 - the D49 G1/G2 removal
experiment (D105). You did not run it. Model tier: mid (dispatch model: Opus 5).
Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

**What is under review is a MEASUREMENT, not a diff.** The task produced no
commit and no file in the repo; the tree is claimed byte-identical to its start.
So the deliverable is: was the fenced mutation applied and only it, is each
recorded observation reproducible, is the selected branch of D105's decision
rule the right one, and is the restore complete. A measurement review is
worthless if it only re-reads the report - reproduce it.

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.** HEAD is `a8fe11f`, tree clean.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t7rev-independent/`
  (create it). Never re-run an instrument another agent wrote, never a shared
  default path, never a path the report names. Any absence check needs its own
  fire; the local `grep` is **ugrep 7.5.0**, where `\b` plus bounded repetition
  under `-E` returns zero silently - use `-P` or a script.
- **You may mutate to reproduce, and you restore the same way the task did**:
  byte baseline taken BEFORE the edit, `git checkout --` to restore (never a
  bare `cp`, it is aliased interactive here), then `sha256sum -c` plus an empty
  `git status --porcelain` and `git diff --stat` as the proof. Leave the tree at
  `a8fe11f`, clean.
- **A deeper probe is permitted and bounded.** The task's report names one
  hypothesis it could not test inside its own prohibitions: which suggestions G1
  actually iterated under the mutation. Its prohibitions were the task's, not
  yours - but you still do not edit the repo's test files. If you want that
  measurement, build it in an isolated copy of the crate under your own
  instrument directory and say so. It is optional; the branch selection does not
  depend on it.

## Ground truth, in precedence order

1. The Plan-9 design: **D105 in full** - the protocol, the exact mutation, the
   decision rule and the recording are design-fixed.
2. The plan's **Task 7** (Steps 1-6, "Must not decide") and Global Constraints.
3. The tree: `crates/muxsmith-core/src/planner.rs` and
   `crates/muxsmith-core/tests/suggestions.rs`.
4. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

The task's report (`task-7-report.md`) is evidence, not ground truth.

## Dimensions

1. **The mutation is the fenced one, and only it.** Re-derive the `AddExact`
   arm yourself and check that the edit D105 prescribes is exactly what the
   report's diff shows - one line, one arm, `AddNotExact` untouched.
2. **Reproduce all three runs yourself**: the green control, the suite under the
   mutation, and the restored green. Record per guard, not per suite. If your
   per-guard result differs from the report's in any cell, that is a BLOCKING
   finding - the branch selection rests entirely on that table.
3. **Rule on the branch selection.** Observed: G1 green, G2 red, G3 green. The
   report selects the anomaly branch. Check it against D105's rule as written,
   in both directions: is there any reading under which this is the all-fail or
   the only-G3 branch, and is the anomaly branch's own condition met.
4. **The failure MODE inside G2 is load-bearing and deserves its own check.**
   The report says G2 went red through its anti-vacuity assertion, not through
   its type-equality assertion - i.e. the guard never reached a comparison
   because the engine's output no longer contained the candidate it inspects.
   Verify that at the source and at the run. If true, the experiment measured
   set composition rather than type degradation, which is the report's central
   claim about why the premise failed.
5. **The mechanism section is context, not measurement, and the report says so.**
   Check that separation held: no conclusion about G1/G2 being load-bearing may
   rest on the unverified hypothesis. Also run its four verified claims
   yourself - the identity-on-strings property of the mutation, the
   `apply_suggestion` routing through `delta_for`, the `AddExact`-only scope, and
   the disappearance of the `id` suggestion from the fixture's output.
6. **The restore is complete.** Reproduce the byte proof independently: your own
   pre-existing baseline is `git show a8fe11f:crates/muxsmith-core/src/planner.rs`,
   which is a reference the task could not have written. Confirm the working
   file matches it.
7. **The prohibitions held**: no commit, no repo file created or modified, no
   house-knowledge YAML touched, no guard removed, no ledger text composed. The
   last one matters - D105 fixes statement text for the two clean branches only,
   and the anomaly branch has none, so an implementer-composed wording would be
   a latitude breach. Check the report for one.
8. **The no-work-needed check**, standing: run every premise the report uses to
   conclude something needs no further measurement - in particular that the
   isolated per-guard runs agree with the in-suite ones, and that the design's
   cited line span is "not stale in any load-bearing way".

## Adjudication questions (one explicit verdict each, not pre-rated)

1. **Is the anomaly branch the correct selection**, or is there a defensible
   reading of D105's rule under which the observed pattern belongs to a clean
   branch?
2. **Does the experiment, as designed, test what D105 says it tests?** The
   report argues the mutation is a no-op for string-valued properties and a
   candidate-remover for the rest, so it never presents the guards with a clean
   type-degradation. Sound, or does the design's premise survive?
3. **Is the unverified G1 hypothesis material?** The report leaves open which
   suggestions G1 iterated under the mutation. Rule on whether the branch
   selection stands without it, and whether the plan close needs that
   measurement before recording anything.
4. **Was returning NEEDS_CONTEXT right**, or could this task have completed by
   recording the anomaly itself? Note what the task may and may not write.
5. **Is the restore genuinely complete**, on your own reference rather than the
   report's baseline?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-7-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with file:line, the evidence you ran and the exact
required change; the five adjudications; an evidence appendix naming your
instrument paths; and a HARVEST including what the plan close must carry - in
particular anything bearing on how the controller should record an outcome the
design fixed no text for - and any observation worth a ledger entry.
