# Task 4 review brief - Plan 9

**Role:** independent reviewer of Plan 9, Task 4 (`EmptyRawProperty` at error
severity with its own DiagCode, the Run-gate e2e scenario, and - after a
mid-task amendment - the locale-parameterized test helper that carries the
German subprocess test). You did not write this code. Model tier: mid
(dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus
the same content as your final message.

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.** Task 4 is a TWO-commit task: `d768657` (the
  feature) and `3412fcc` (the amendment-4 fix round). House commits land
  between dispatches; grade the current tree.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t4rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never a
  shared default path. Any absence check needs its own fire.
- If you mutate anything, restore non-interactively (`git checkout --`) with a
  baseline taken first and prove the restoration. A bare `cp` here is aliased
  interactive and can hang with the tree still mutated.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (this task amended
   four of its passages: S-1's new row, S-3, S-5, S-6).
2. The Plan-9 design: **D101** in full, including the amendment-1 producer
   paragraph and its scenarios-in / infrastructure-out boundary; design
   section 5's raw bullets and two-scenarios bullet; the `## Amendment log`.
3. The plan's **Task 4** (Files list, Steps 1-8, "Must not decide") **as
   amended twice** - amendment 4 pins the helper's signature and body, the
   funnel's delegation, the funnel's rustdoc, the helper's own rustdoc, the
   test's invocation, and a Step-7 file-level invariant check.
4. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

The implementer's brief and its report (`task-4-report.md`, including the
appended fix round) are evidence, not ground truth.

## Dimensions

1. **Contract compliance across both commits**: D101's three-branch funnel
   form, the variant doc, both Fluent lines, the catalog row, the two pinned
   core tests with the existing B-2/B-3 pair as their control, the two pinned
   subprocess tests, both snapshots, the Run-gate scenario as D101 enumerates
   it, and the four spec amendments - each character-for-character where the
   design fences it. Build your own comparison.
2. **The German test's load-bearing assertion.** Its exit code proves nothing
   on its own: a mis-invocation produces the same exit 2 with empty stdout,
   which is the trap that caused amendment 4. Verify the snapshot's CONTENT
   against D101's German fence with your own extraction, and satisfy yourself
   the test would fail if the German rendering broke.
3. **The delegation's blast radius.** `muxsmith` now delegates; every existing
   CLI subprocess test rides it. Verify none changed behaviour, and that
   `muxsmith_bare` plus its closed two-caller exception doc are byte-identical
   to the pre-state (diff it yourself).
4. **The two rustdocs.** The funnel's had to lose the statements the
   delegation falsified and state the invariant at FILE level (its old
   function-level wording was already imprecise); the helper's own doc was
   pinned by amendment 4 to a closed content list that does NOT restate D64's
   rationale or the invariant. Grade both against what the plan pins.
5. **Step 7's invariant check**: real fire, reachable red, green on the end
   state.
6. **Latitude, both forms**, including the inverse - did the implementer
   resolve at the keyboard something that should have returned? Its numbered
   concerns are where to look hardest.
7. **House dimension**: Tier-2 conformance; the amended
   `latitude-carveout-zero-content-structural-forks` (file-vs-within-file);
   `a-null-assertion-over-a-dynamic-map-proves-nothing-without-a-presence-check`;
   `proc-normative-count-recomputed` including its callers'-docs facet; and the
   owner's execution-time precedence ruling in
   `tests-ship-with-the-feature-never-after` - a consequence this package's own
   diff creates, uncovered, additive, on existing infrastructure, is BUILT and
   reported, not routed.
8. **The no-work-needed check**: run every premise a passage uses to conclude
   something is unnecessary, unobserved or already covered.
9. **Verification quality**: re-run the Step-7 bar yourself and recompute every
   aggregate the report states (it claims 39 test binaries, 63 e2e, seven CLI
   suite counts, 212 catalog ids).

## Adjudication questions (one explicit verdict each, not pre-rated)

1. **`get(\"\")` -> `get("")` in the variant doc.** The design gives the doc
   text inside a quoted string with the inner quotes escaped; the implementer
   transcribed the content and resolved the escapes. Correct transcription, or
   a deviation from a character-for-character fence?
2. **One sentence added to `raw_opt_in_diagnostic`'s rustdoc**, because the
   task's own edit made its two-outcome enumeration false. In scope under the
   amended grant (listed file, own edit invalidated it, zero outward effect),
   or should it have been surfaced instead of applied?
3. **The `#[allow(dead_code)]` that was drafted and then removed.** The
   implementer measured that the lint does not fire because the funnel
   delegates, and removed the attribute as dead weight and a false signal.
   Right call?
4. **Committing the verified subset while returning NEEDS_CONTEXT** (the first
   commit). The implementer argues it was the lesser risk and reversible.
   Acceptable, or should a returning task leave the tree uncommitted? Say
   whether this is worth a standing rule, since Tasks 5-7 can hit the shape.
5. **The Run-gate scenario's placement** inside the batch describe, chosen to
   maximize distance from where Task 5's scenario will land. Sound, or does it
   collide with Task 5's region ownership?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-4-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with file:line, the evidence you ran and the exact
required change; the five adjudications; an evidence appendix naming your
instrument paths; and a HARVEST including what Tasks 5-7 must carry.
