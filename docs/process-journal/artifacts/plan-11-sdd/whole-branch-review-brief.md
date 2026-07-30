# Whole-branch review brief - Plan 11

**Role:** independent whole-branch reviewer at Plan 11's close. You did not write
any of it. Model tier: **top** (dispatch model: Fable 5) - the only role this
project's model convention gives the top tier. Effort: xhigh.

**You commit nothing and edit no file.** Your output is a verdict file plus the
same content in short form as your final message.

**What you are for.** Five task reviews already ran and each graded one task
against its own contract. You grade what none of them could see: the merged
state, the interactions between tasks, and whether the package as a whole is
what the plan promised. **Nothing goes out until you have run** - no push, no
tracker disposition, no journal entry.

## The package, in one paragraph

Plan 11 landed two independent bodies of pre-1.0 residue in two streams, merged
A then B, with the full gate green on each merged state. Stream A: four serial
documentation-accuracy tasks, one of which turned into the plan's only BEHAVIOUR
change - `raw:` now compares without type conversion, per ADR D111. Stream B: the
project's two open dependency-vulnerability alerts, in three parts. Along the way
the plan itself was amended once (Amendment 5, six defects found at execution),
that amendment was independently reviewed and took its own fix round, and Task B1
took a fix round applying the corrected text.

## Preamble (binding)

- Never call session-relocation tools. Absolute paths, **foreground runs only**.
- Grade in the main worktree, `/home/senol/Git/Muxsmith`, on `master` at head
  `245a51a`. Both stream worktrees still exist and are **not** yours to enter or
  modify.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/wbrev-independent/`
  (create it). Never re-run an instrument any earlier agent wrote; never use a
  shared default path. Mutation experiments go on a COPY of the crate.
- The tree must be byte-identical when you finish. Prove it.
- **Do not re-run the full eleven-part gate** - the controller ran it on both
  merged states, per part, exit codes captured separately: 507 Rust tests over 39
  suites, 68 e2e cases, zero failures, all parts 0. Re-run what your own findings
  require.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/review-whole-branch-5378264..HEAD.diff`
- the full branch range from the plan's base, with commit list, stat and context.

## Ground truth

1. The v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`), which
   outranks the plan on conflict, and **ADR D111**
   (`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`), which is
   ground truth for the behaviour change.
2. The plan,
   `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`,
   **including Amendment 5** - its Global Constraints, its work-item coverage map,
   its acceptance map (40 halves), its deferred-by-decision table and its
   plan-close section.
3. `.superpowers/sdd/plan-11/plan-brief.md` - the brief the plan implements.
4. The four house-knowledge YAML files, as review ground truth alongside the
   spec; cite entries by id.
5. `.superpowers/sdd/plan-11/progress.md` - the controller's tracker, carrying
   every disposition, every parked item and every controller-surfaced finding.

The five task verdicts, the amendment verdict and the implementer reports are
**evidence, not ground truth**. Several reports carry dated ERRATUM blocks where
a review refuted a claim; the originals are left legible on purpose.

## Dimensions

1. **Coverage against the work-item map and the acceptance map.** Walk all five
   work items and all 40 acceptance halves, and for each say whether its named
   producer exists and produced. **A row whose producer is missing is a finding
   even if every task passed its own review** - that is the gap a per-task review
   structurally cannot see.
2. **Cross-task interaction, which is the other thing only you can see.** A1 and
   A4 both touch documents A2 and A3 also touch; A3 and A4 amend the same spec
   file in different sections; B1's `deny.toml` change and A1's `BUILDING.md`
   change both concern the same gate part. Look for a statement one task made
   true and another made false.
3. **SPOT-RE-RUN THE LOAD-BEARING PASTED EVIDENCE, and this is not optional.**
   Task A4's review found two pasted grep outputs in one report section that are
   not what their commands return, with the label "the full enumeration" over a
   list short by one. The claims were independently true, so nothing failed. Two
   in one section is a pattern, and the A4 reviewer's own harvest names where to
   start: **A3's twelve-repair / seven-retain split IS a pasted grep result**, and
   which twelve sites got changed rests on it. Re-derive that split yourself.
   Note the measured environmental trap while you do: a recursive grep's emitted
   ORDER is not stable here (nine runs, seven distinct hashes), so compare the
   SET of lines, never the text.
4. **The behaviour change, end to end.** `raw:` no longer cross-compares int and
   float; the typed `exact` path still does. Verify both halves against the
   shipped binary, and satisfy yourself that the safeguard test genuinely guards -
   the A3 reviewer measured that stripping the cross arms fails exactly one test
   workspace-wide, which means that one test is the sole net.
5. **The security-gate change.** `cargo deny` now evaluates the unsound class and
   ignores one advisory with a recorded reason. Verify the shipped comment's
   claims at cargo-deny's own source, and confirm the three-way fire still holds
   on the MERGED state rather than on stream B's branch.
6. **Claim language at the close** (`milestone gate`, claim-language rule): the
   package's completion is NOT a 1.0 completeness statement, the `glib` alert is
   IGNORED and not fixed and stays open on GitHub, and the line-citation class is
   closed over **tracked files outside `docs/`** and not tree-wide. Check that no
   artifact in this branch - commit message, spec text, comment, ROADMAP line -
   over-claims any of the three.
7. **The no-work-needed check** across the whole package: wherever a report, the
   plan or a comment concludes something is unnecessary, already covered or
   impossible, run the premise. This shape produced three refuted claims in this
   plan already, one of them in a deferral's cost argument.
8. **House dimension** across the merged diff, by id. **Harvest** as usual.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **Was the amendment the right instrument?** Six plan defects surfaced at
   execution, two of them after a false sentence had already shipped in a commit.
   The controller routed them into ONE amendment rather than per-finding fixes,
   scaled it one-pair, and had a fix round apply the corrected text afterwards.
   **Was that the right shape, and did anything get lost between the finding and
   the applied text?**
2. **Do the errata work?** Several implementer reports carry dated erratum blocks
   instead of corrections, on a controller ruling that a dated record is not
   falsified retroactively. **Read two of them cold: can a later reader tell what
   was claimed, what was measured, and which is true now - or do the originals
   mislead a reader who stops at the first paragraph?**
3. **The parked owner decision.** B1's review refuted the premise of a deferral
   (a permanent guard was said to need new gate infrastructure; it needs one key
   in the same table) and simultaneously recommended AGAINST adding the key
   blind, because it also reddens the gate when an ignored advisory legitimately
   disappears upstream. The controller parked it as a one-key owner decision with
   the measurement attached. **Is parking right, or does the branch ship a gap
   that should not wait?**
4. **What this package did to the gate's coverage.** `cargo deny check` now
   covers a class it did not cover before. **Is that stated anywhere a future
   reader would look, and is the claim accurate about what is covered and what is
   merely ignored?**
5. **Is anything in this branch unfinished in a way the per-task reviews could
   not see** - a half-applied convention, a document that now contradicts a
   sibling, a test that passes for the wrong reason, a deferral whose vehicle
   does not exist?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/whole-branch-verdict.md`:

- Verdict: READY / READY_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings with `file:line`, the evidence you ran, and
  the exact required change. **Triage each against merge: must-fix before the
  push, or deferrable with a named vehicle.**
- The five adjudications, one explicit verdict each.
- A triage of the tracker's deferred minors and parked items (in
  `progress.md`): which must be fixed before the push, which are correctly
  deferred.
- An evidence appendix naming your instrument paths and commands.
- A **HARVEST** section.

Your final message carries the short form only.
