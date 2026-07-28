# Plan 9 plan review brief (round 1)

Independent reviewer, fresh eyes; you did not author this plan. Artifact
under review:
`/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`
(commit `2155c1d`, 432 lines, seven tasks).

Ground truth, in this order of authority: the v1 spec
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`), the
owner-approved design
(`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`,
D91-D105, plus every entry in its amendment log), the controller brief
`.superpowers/sdd/plan-9/plan-brief.md`, the ROADMAP Plan 9 anchor, the four
house-knowledge YAML files, and the actual tree. Verify claims; never believe
them.

Neither the owner's rulings nor the design's decisions are under review. A
finding that re-opens a settled decision is out of scope; a finding that the
plan MISREPRESENTS one is exactly in scope.

## Dimensions

1. **Coverage - the primary dimension, and the reason this review exists.**
   Walk the design section by section: every `## Dn` from D91 to D105, its
   section 3 spec amendments (S-1 to S-7), its section 5 must-not-decide
   list, its section 6 triggers, and every section 7 acceptance observable.
   For each, name the task that implements it. A design section with no task
   is a finding. Do this walk from the DESIGN, not from the plan's own
   coverage map - then check the map against your walk; a map that claims
   coverage the tasks do not deliver is a worse finding than a missing task,
   because it hides one. The characteristic plan defect is the missing task,
   and no downstream stage can catch it: an implementer sees only its own
   task and cannot notice one that does not exist.
2. **Executability by a fresh implementer.** For each task, ask whether an
   agent with the plan, the design and the named files could execute it
   without inventing anything: no unnamed file, no unwritten string, no
   unspecified test name, no "the appropriate module". Check that every
   Files/Interfaces list is explicitly marked EXHAUSTIVE or EXEMPLARY - an
   unmarked list reads as exhaustive by house rule, which is a finding if the
   task actually needs more.
3. **Latitude scan, both forms.** Explicit permissions and omission latitude
   (an unenumerated set in a normative position, a list ending open, a "one
   per X" without the X list). Applied to every normative sentence in the
   plan, including task steps and verification steps.
4. **Sequencing soundness.** The plan rules strictly serial, no worktrees,
   with a file-graph argument. Test it: is the dependency graph as claimed
   (check the file overlaps yourself), and would any task's own work actually
   exceed a worktree setup plus a merge plus a second full ten-part gate run?
   Check the staged cut between tasks 1 and 2 specifically - the plan claims
   both intermediate states compile and pass the suite, which is a
   falsifiable claim about a state that does not exist yet; judge whether the
   reasoning holds. Check that cross-task constraints travel verbatim into
   the dependent task rather than living only in the earlier task's prose.
5. **Verification quality.** Every command in every verification step is
   executable as written. Every check whose passing result is an ABSENCE
   carries both properties: a fire-verified RED state and a reachable GREEN
   state (`proc-verification-step-must-be-falsifiable` +
   `proc-check-green-state-reachable`). The design shipped a green-unreachable
   check in its own first round, so this is a live failure mode here. The
   ten-part gate must appear at its mandatory sites, foreground, no subsets.
6. **Model tiers.** A tier per task with a stated ground, per
   `proc-03-model-assignment`. The plan claims no task qualifies for the
   cheap tier - test that claim against the tasks whose content the plan
   carries most literally.
7. **House dimension.** Conformance to the four house files, cited by entry
   id. Specifically: no progress markers in the plan document (the tracker is
   the SDD scratch), no task edits any house-knowledge YAML, the SI-4 commit
   grant is restated in every dispatch that expects a commit with the trailer
   DERIVED from the dispatch's model parameter rather than written as a
   literal, explicit staging with no `git add -A`, and typography.
8. **Close actions complete.** The roll-up funnel; the promotion sweep of the
   FIVE owner-ruled entries plus `core-121`'s separate `blocked_on` clearing;
   the `core-d49-g1g2-experiment` entry written by the controller from the
   task's measurement; the whole-branch review on the top tier; the salvage
   with its `diff -r` re-check; the journal entry; the HANDOFF snapshot.
9. **No-work-needed check.** Every passage concluding that something is
   unnecessary, already covered, or safe to skip - run the premise, do not
   weigh it.
10. **Counts and evidence.** Every count recomputed from its enumeration;
    every evidence line's output reproduces when you run the command. Four
    counts failed this in the design cycle, one of them the controller's, so
    re-run the plan's authoring-time verification section rather than reading
    it.
11. **Scope.** No work on the two OUT items beyond the ruled D23 tests, no
    new dependency in either ecosystem, no release or tag action, no README
    placeholder resolution, no product-boundary change.
12. **The two uncovered acceptance observables.** The design records that the
    GUI Run-gate consequence of the new error severity and the BatchView
    branch D103 edits have no producer. Check the plan neither drops them
    silently nor rescues them with an invented producer.

## Output

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/plan-review-round-1.md`:

- Verdict: `APPROVED` or `NEEDS FIXES`.
- A coverage table: design section -> task, with every gap marked.
- Findings by severity (Critical / Important / Minor), each with location,
  what is wrong, and what to change; empirical findings carry the command and
  its output.
- A `## HARVEST` section: dominant patterns, repeated rejections, and any
  place where this brief's own boundary forced a stop you judge it should
  have covered.

Final message: the verdict word, at most three lines, and the file path.

## Constraints

Read-only on the tree except your own verdict file. No git commands. No
session-relocation tools. Absolute paths. Foreground only. Build your own
probes at a scratch path you name in this pass - never re-run an instrument
the plan author or the design reviewer left behind, and never at the shared
path both would default to. Any negative result you report is fire-verified
against a known-present case first, and you say so.
