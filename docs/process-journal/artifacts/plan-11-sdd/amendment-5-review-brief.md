# Amendment 5 delta review brief - Plan 11

**Role:** independent reviewer of Amendment 5 to Plan 11. You did not write it.
Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no file.** Your output is a verdict file plus the
same content in short form as your final message.

## What this amendment is, and why its standard is unusual

Plan 11 is **mid-execution**. Two of its five tasks are committed and reviewed,
a third is running as you read this, and **two of the defects this amendment
repairs already shipped into a commit**. So the replacement fence it writes is
not a first implementer's input: it is what a **B1 fix round will apply to
`deny.toml`** as soon as you approve. A false sentence you let through lands in
the repository.

The amendment repairs six defects plus two minors, all found at execution by
implementers and reviewers, all independently reproduced by the controller before
routing. It is scoped ONE-PAIR: no task added, removed or re-cut.

## Preamble (binding)

- Never call session-relocation tools. Absolute paths, foreground runs only.
- **Read and grade in the main worktree, `/home/senol/Git/Muxsmith`, on `master`.**
  Do NOT enter `/home/senol/Git/muxsmith-plan11-a` - Task A3 is writing there
  right now - and do not write anything anywhere in the repository.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a5rev-independent/`
  (create it). Never re-run an instrument the amendment's author wrote; never use
  a shared default path. Any absence-shaped check you rely on needs its own fire.
- The tree must be byte-identical when you finish. Prove it.

## The delta

`git diff 83af0d5..44bc6f7` over
`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md` -
one file, 70 insertions, 20 deletions, plus the new `## Amendment 5` section.

## Ground truth

1. `.superpowers/sdd/plan-11/amendment-5-brief.md` - the controller's brief,
   which enumerates the six defects and the two minors. **It is the requirement
   set you grade coverage against, and it is itself fallible**: its author reports
   that the brief's own multi-site list was short, naming three defects as
   multi-site where a fourth also was. Grade what the DEFECTS required, not only
   what the brief listed.
2. `.superpowers/sdd/plan-11/task-a1-verdict.md` (findings 4, 5, adjudication 1)
   and `task-b1-verdict.md` (findings 1 to 5, adjudications 1 and 2) - the
   verdicts the repairs come from, including the replacement wording adjudication
   1(b) supplies.
3. `.superpowers/sdd/plan-11/amendment-5-report.md` - the author's own account.
   Evidence, not ground truth.
4. The house-knowledge YAML files; cite entries by id.

## Dimensions

1. **Per-defect coverage, at EVERY site.** For each of the six defects and two
   minors, establish the site set **by searching the pre-amendment document
   yourself** and check every member. The author reports the retired
   "external crate" shorthand at six sites and the guard premise at three lines
   plus four restatements, one of them invisible to a line-based grep because a
   table row carries it in two columns. **Verify those set sizes rather than
   accepting them** - a repair that fixes the site a finding named and misses its
   restatements is this plan's single most repeated defect.
2. **No new wrong figure.** Three previous fix rounds on this document introduced
   fresh wrong figures while repairing old ones. Re-run every figure the
   amendment states or moves, executing the document's own text rather than a
   retyped equivalent.
3. **The new `deny.toml` fence is TRUE, sentence by sentence, and lands.** It
   asserts specific cargo-deny semantics. Verify each assertion at the tool's own
   source (`~/.cargo/registry/src/index.crates.io-*/cargo-deny-0.19.9/`), not at
   its output, and re-measure the three note-class counts yourself. Then check it
   against the artifact it will land in: ASCII only, straight quotes, comment
   syntax, and line width against `deny.toml`'s own longest line.
4. **The count was DROPPED rather than corrected, deliberately.** The reviewer's
   ground was that a count over a growing ignore list goes stale on the next
   addition. **Is dropping it right, or does the sentence now under-explain the
   asymmetry it exists to explain?**
5. **The guard deferral's replacement reasoning.** The old ground ("that would be
   new gate infrastructure") was refuted. Check that what replaced it does not
   quietly assert the opposite either - the B1 reviewer explicitly did NOT
   recommend adding the key, because it also reddens the gate when an ignored
   advisory legitimately disappears upstream, and the controller has parked it as
   an owner decision. The row must record a routed decision, not a resolved one.
6. **Scope.** Nothing in Task A2, A3 or A4, and nothing in ADR D111's territory,
   may have moved - Task A3 is executing against a worktree copy that predates
   this amendment, so a change there would mean an implementer working from a
   superseded contract. Verify that independently.
7. **The amendment does not compress.** The plan's reviewer ruled that its
   meta-text is condensed at the plan close, not mid-execution.
8. **The no-work-needed check.** Where the amendment or its report concludes
   something needed no repair, run the premise.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **Is the amendment's scale call right?** It declares itself ONE-PAIR on the
   ground that no task is added, removed or re-cut. But defect 5 changes the
   reasoning behind something Task B1 DEFERS, and defects 3 and 4 change a fence
   B1 already shipped. **Does re-grounding a deferral and replacing a shipped
   fence re-cut the task, or is the task's work genuinely unchanged?**
2. **The surfacing list stayed at seven.** The author deliberately did not add
   the mise rider as an eighth item, reasoning that it is execution-time
   provenance already disposed of by the controller, and flags that the count has
   a consumer in the self-review. **Right call, or is a controller-disposed item
   still owed a line in the list the plan close walks?**
3. **Two figures in the SOURCE verdicts were wrong** and the author set them from
   its own runs rather than transcribing them (a control described as "2 matches"
   is 2 lines carrying 5 occurrences; a stated maximum line width of 78 measures
   77). **Was correcting them silently in the amendment the right handling, or
   does a verdict figure that turns out wrong owe a note back to the verdict?**
4. **The fence's placement instruction.** B1's Step 4 fences both the comment and
   its insertion point. The replacement changes the comment's length. **Does the
   placement instruction still resolve unambiguously against the file as it
   stands after `c422999`, given that the fix round applies this to a file that
   already contains the OLD fence rather than to a pristine one?** This is the
   question most likely to bite the fix round.
5. **Completeness against the verdicts.** Walk both verdicts' findings and
   adjudications end to end. **Is there anything either reviewer required that
   this amendment does not carry** - including things the controller's brief did
   not name?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/amendment-5-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings, each with its site, the evidence you ran,
  and the exact required change.
- The five adjudications, one explicit verdict each.
- An evidence appendix naming your instrument paths and commands.
- A **HARVEST** section.

Your final message carries the short form only.
