# Amendment 3, plan side - review brief

**Role:** independent reviewer of the plan-side half of Plan-9 amendment 3.
You did not write it. Model tier: top (dispatch model: Fable 5). Effort: xhigh.

**You commit nothing and edit no file under `docs/` or `crates/`.**

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Repo `/home/senol/Git/Muxsmith`, `master`, main worktree.
- Absolute paths, foreground runs only.
- **Read the FILES, not a hash.** The plan amendment is commit `36d8538`; use
  `git show 36d8538` for the diff, but grade the current file. House commits
  land between dispatches, and every line number in the author's report and in
  the earlier design verdict predates this amendment's own line shifts.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a3plan-rev/`
  (create it). Never re-run an instrument the author wrote, never a shared
  default path. Any absence check needs its own fire.

## Context

Owner ruling 2026-07-28: the rustdoc that Task 2 moved verbatim into
`muxsmith_core::executor::queue` became false about the function when the same
commit gave it a second caller; the correction is a DESIGN change and the code
edit rides Task 3. The design half is done, independently reviewed, APPROVED
with no findings (`.superpowers/sdd/plan-9/amendment-3-verdict.md`); it carries
the replacement doc comment as a transcription fence in D96's amendment-3
rider. This half carries that into the execution plan.

Inputs, all evidence rather than ground truth: the author's brief
(`.superpowers/sdd/plan-9/amendment-3-plan-brief.md`), its report
(`.superpowers/sdd/plan-9/amendment-3-plan-report.md`), the design verdict
above.

## Ground truth

The v1 spec, then the amended Plan-9 design (D96 + its amendment-3 rider,
section 5, the amendment log), then the plan itself. The four house-knowledge
YAMLs alongside; cite ids, re-verify any `:line`.

## Dimensions

1. **Coverage.** Does the plan now carry everything the amended design
   requires of it, and does the design's D96 rider have exactly one
   implementing task and step? Walk it; a design obligation with no plan home
   is a finding.
2. **Single home for the fence.** The plan must POINT at the rider, never
   duplicate it. Verify no copy exists anywhere in the plan.
3. **Latitude, both forms**, in the new Step 2 and in every line the amendment
   touched: no explicit permission, and no omission - nothing a Task-3
   implementer must invent. Specifically check the wrapping instruction (the
   author says this fence is transcribed with its wrapping, unlike Task 1's
   fences, which permit re-wrapping): is that unambiguous as written?
4. **Renumbering integrity.** Task 3's steps went from ten to eleven. Verify
   the sequence has no gap or duplicate, that every internal cross-reference
   moved with it (the author reports exactly one, the staging note's
   "Step-2 compiler sweep"), and that no reference to a Task-3 step survives
   anywhere else in the plan, in the design, or in the SDD scratch. Run that
   sweep yourself with a fire.
5. **The three ripple edits, which the author flagged for your ruling.**
   Beyond its enumerated mandate it also updated Task 3's header parenthetical,
   the coverage map's D96 row, and the sequencing 2->3 edge, arguing they are
   enumerations the reviewer's own walk consumes and that the design amendment
   swept its analogous consumers. Rule: in scope, or over-reach against the
   brief's "nothing else changes"? Say which, per edit.
6. **The two decisions** (historical qualifier on Task 2 Step 1's executed
   "rustdoc moved with it"; extending the queue.rs Files-entry parenthetical).
   Both were required to be decided rather than left open. Grade the decisions
   and their stated reasons - including whether the Files-entry extension is
   consistent with the owner's file-vs-within-file ruling
   (`proc-latitude-clause-boundary`, `docs/process-conventions.yaml`).
7. **Accuracy.** Every factual claim the amendment adds to the plan - the
   fence's location and width, the anchor, the "no src-tauri file" statement,
   the amendment-log entry's account of what moved - verified at the source.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/amendment-3-plan-verdict.md`
and make your final message the same content: verdict
(APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered severity-tagged
findings with file:line, evidence and the exact required change; your rulings
on dimensions 5 and 6; an evidence appendix naming your instrument paths; and a
HARVEST section including anything Task 3's dispatch must carry.
