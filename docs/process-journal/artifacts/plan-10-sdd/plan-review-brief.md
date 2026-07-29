# Plan 10 plan review brief (round 1)

Independent reviewer, fresh eyes; you did not author this plan. Artifact under
review:
`/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`
(commit `da60634`, 557 lines, five tasks).

Ground truth, in this order of authority: the v1 spec
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`), the controller
brief `.superpowers/sdd/plan-10/plan-brief.md`, the ROADMAP entries that brief
cites, the four house-knowledge YAML files, and the actual tree. Verify claims;
never believe them.

**There is no design document for this package**, by owner-approved decision.
The brief stands in for it as coverage ground truth. That raises the stakes on
your coverage walk rather than lowering them: a design document would have been
reviewed on its own, and this one was not.

Neither the owner's rulings nor the settled decisions in the brief are under
review. A finding that re-opens a settled decision is out of scope; a finding
that the plan MISREPRESENTS one is exactly in scope.

## Dimensions

1. **Coverage - the primary dimension, and the reason this review exists.**
   Walk the brief's five work items W1-W5 section by section, plus the ROADMAP
   entries each cites, plus each work item's explicit in-scope and out-of-scope
   lists. For each obligation, name the task that implements it. Do this walk
   from the BRIEF, not from the plan's own coverage map - then check the map
   against your walk; a map claiming coverage the tasks do not deliver is a
   worse finding than a missing task, because it hides one. The characteristic
   plan defect is the missing task and no downstream stage catches it: an
   implementer sees only its own task and cannot notice one that does not exist.
   **Walk an observable's HALVES, not the observable.** W1 in particular: the
   author reports that D102's contract has exactly FOUR halves rather than six,
   on the ground that `config_only_document` emits two arrays as literal `[]`
   and therefore has no preserved-order half. That is a falsifiable claim about
   code you can read. Check it, then check that a producer is named per
   surviving half.
2. **The author's four refutations are now load-bearing - re-verify them.**
   The plan is built on four corrections to the controller brief: that
   `BUILDING.md`'s frontend section states no count and the Rust heading is the
   file's only gate count; that the citation corpus is 20 comment lines across
   13 files rather than 17; that the cargo `rangeStrategy` default is the global
   `auto` which resolves to `update-lockfile` for that manager; and that the
   current documented validator invocation is neither form the brief started
   from. Re-measure each with your OWN instrument. A refutation that is itself
   wrong would now be carried by the plan unchallenged, which is the one place
   where the refute-the-brief mechanism can invert.
3. **Executability by a fresh implementer.** Per task: could an agent with the
   plan and the named files execute it without inventing anything - no unnamed
   file, no unwritten string, no unspecified test name, no "the appropriate
   module"? Every Files/Interfaces list explicitly marked EXHAUSTIVE or
   EXEMPLARY; an unmarked list reads as exhaustive by house rule, which is a
   finding if the task actually needs more.
4. **Latitude scan, both forms, and three named suspects.** Explicit permissions
   AND omission latitude (an unenumerated set in a normative position, a list
   ending open, a "one per X" with no X list). The author self-reports three
   places where the boundary was drawn deliberately and invites re-opening:
   - Task 1's fence carries the gate count `11 / 6 / 4 / 1` while Global
     Constraints carries none. The author's argument is that the count is that
     task's DELIVERABLE rather than a constraint the plan executes against, and
     that a placeholder would itself be latitude. Judge whether the brief's ban
     is textual or scoped, and say which reading you apply.
   - W1's producer set is measurement-gated across four fully written-out
     candidate tests. Judge whether that is a closed enumeration selected by
     measurement (acceptable) or a fork handed to the implementer (banned).
   - W5's sentence wording is the implementer's while its fact set is closed.
     Same judgment.
5. **Sequencing soundness.** The plan rules strictly serial, one tree, no
   worktrees, and claims **the five tasks' Files lists are pairwise disjoint** -
   so the ordering rests on two arguments rather than on file conflict: a hard
   4 -> 5 edge (three sweep sites cite `README.md` spans that Task 4 edits) and
   Task 1 first because it changes the gate's own tooling. Check the disjointness
   claim yourself against the tasks' Files lists. Check the 4 -> 5 edge is real
   and that no OTHER pair carries an unstated edge. The author measured the warm
   gate at 17.186 s and could not measure a cold worktree gate because creating
   one is a git command the dispatch forbade; judge whether the comparison still
   resolves, and whether "unmeasured but strictly larger, by construction" is
   argued or asserted.
6. **Verification quality.** Every command in every verification step executable
   as written. **Every check whose passing result is an ABSENCE carries a
   fire-verified RED state and a reachable GREEN state.** The plan claims three
   fires in Task 1 and a fired control in Task 5 - re-run them, do not read
   them. This is the live failure mode for W3 specifically: a gate-count check
   that silently matches nothing would pass forever and prove nothing, which is
   precisely the defect it exists to prevent, one level up. The gate must appear
   at its mandatory sites, foreground, no subsets, and named by FILE rather than
   by a count, since this package changes what that file says.
7. **Model tiers.** A tier per task with a stated ground, per
   `proc-03-model-assignment`, top tier reserved for the plan-close whole-branch
   review and its delta re-reviews. Test any claim that a task does or does not
   qualify for the cheap tier against the tasks whose content the plan carries
   most literally - Task 3's config is fenced verbatim, which is the shape that
   usually earns the cheap tier, and the plan should say why it does or does not
   here.
8. **House dimension.** Conformance to the four house files, cited by entry id.
   Specifically: no progress markers in the plan document; no task edits any
   house-knowledge YAML; the SI-4 commit grant restated in every dispatch that
   expects a commit, with the trailer string DERIVED from the dispatch's model
   parameter rather than written as a literal; explicit staging with no
   `git add -A`; typography; and the new
   `comments-locate-by-symbol-never-by-line-number` convention applied to any
   comment the plan itself instructs an implementer to write.
9. **Scope.** No version bump, no tag or release action, no README placeholder
   resolution, no banner removal, no new permanent gate part or CI job or
   runtime dependency for the Renovate validation, no house-YAML edit, no
   `scripts/ledger-lint.py` rename. And the negative half: the plan must not
   claim this package completes 1.0 scope - the owner's manual QA pass is a
   precondition on the tag per `owner-manual-qa-gates-the-1-0-release`.
10. **The out-of-scope findings the author surfaced.** Three bare line-span
    citations with no filename token; two measurably false counts in the
    README's "How this got built" paragraph; `BUILDING.md`'s positional
    "part 6" / "parts 1-4" ordinals; and a sorted-half guard that is
    `have_mkvmerge()`-gated. Check the plan neither silently absorbs these into
    a task nor silently drops them - surfacing them for controller routing is
    the correct treatment, and doing either other thing is a finding.
11. **No-work-needed check.** Every passage concluding that something is
    unnecessary, already covered, or safe to skip - RUN the premise, do not
    weigh it.
12. **Counts and evidence.** Every count recomputed from its enumeration; every
    evidence line's output reproduces when you run the command. The controller's
    own count was wrong once already in this package's short history, so treat
    controller-origin figures with the same suspicion as any other.

## Output

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/plan-review-round-1.md`:

- Verdict: `APPROVED` or `NEEDS FIXES`.
- A coverage table: brief obligation -> task, with every gap marked.
- Findings by severity (Critical / Important / Minor), each with location, what
  is wrong, and what to change; empirical findings carry the command and its
  output.
- A `## HARVEST` section: dominant patterns, repeated rejections, and any place
  where this brief's own boundary forced a stop you judge it should have covered.

Final message: the verdict word, at most three lines, and the file path.

## Constraints

Read-only on the tree except your own verdict file. No git commands beyond
read-only inspection. No session-relocation tools. Absolute paths. Foreground
only. **Build your own probes at a scratch path you name in this pass** - never
re-run an instrument the plan author left behind, and never at the shared path
both of you would default to; agents in one session converge on the same names,
and a re-run that silently executes the author's own instrument produces
agreement by construction. Any negative result you report is fire-verified
against a known-present case first, and you say so.
