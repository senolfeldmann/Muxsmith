# Task A4 review brief - Plan 11

**Role:** independent reviewer of Plan 11, Task A4 (W4: the v1 spec's section 8.1
states the shipped CLI surface, and its exit-code bullet gains the cancellation
code). You did not write this change. Model tier: mid (dispatch model: Opus 5).
Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content in short form as your final message.

**This is the last task of the plan.** After you, the stream merges and a
whole-branch review runs. Anything you leave unraised has one net left.

## Preamble (binding)

- Never call session-relocation tools. Absolute paths, **foreground runs only**.
- **The work sits in a worktree:** `/home/senol/Git/muxsmith-plan11-a`, branch
  `plan-11-stream-a`, head `06e896e` over `164e571`. The stream's base is
  `5378264`. Do not touch `/home/senol/Git/muxsmith-plan11-b`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a4rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never use a
  shared default path. Any absence-shaped check you rely on needs its own fire.
- The tree must be byte-identical to `06e896e` when you finish. Prove it.
- **The plan document on `master` has since gained Amendment 5**, touching Tasks
  A1 and B1 only. Your ground truth is the worktree copy.

## Ground truth, in precedence order

1. The v1 spec itself is the artifact under change and outranks the plan on
   conflict (`proc-04-spec-wins`).
2. The plan in the worktree: Global Constraints, **Task A4** in full, acceptance
   rows **W4-a through W4-c**, and the authoring section's block "Item 4's
   corpus: the spec's 8.1 block is stale in four of its five lines".
3. `.superpowers/sdd/plan-11/plan-brief.md` item 4; `docs/ROADMAP.md`'s "Docs
   accuracy" entry beginning "The v1 spec's section 8.1 synopsis"; the four
   house-knowledge YAML files, cited by id.

The implementer's brief and report are **evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/review-164e571..06e896e.diff`

## Dimensions

1. **The surface is what the BINARY says, not what the source or the plan says.**
   Re-derive it yourself: prove `target/debug/muxsmith` is current (the task
   before this one changed `matcher.rs`), then capture `--help` for the top level
   and every subcommand, and build your own divergence table at FLAG granularity
   including value names and possible-value sets. **If your derivation disagrees
   with the shipped block anywhere, that is a finding of the first order** - this
   task exists because the authoritative document had drifted from the binary.
2. **Both fenced replacements, character for character**, and the reconstruction
   check: rebuild the end-state file from its `164e571` blob plus the two fences
   and compare byte for byte. Check the precondition per site - each fenced OLD
   block occurring exactly once - and note that the task before this one amended
   four OTHER sections of the same file, one of whose lines carries both a
   repaired and a retained clause.
3. **The exit-code claim, at its sources.** "Only `run` returns 130; no other
   subcommand installs a SIGINT handler" is a measurable claim about the code.
   Verify it at `commands/run.rs`, `commands/mod.rs` and `cli.rs`, and satisfy
   yourself that no other subcommand can reach a 130 by any path.
4. **The self-contradiction sweep as an enumeration.** The plan pre-classifies six
   hits for the cancellation sweep. Re-run both sweeps with your own instruments,
   confirm the six and their verdicts, and check the flag sweep's prescribed
   control actually returns the amended block. **A hit outside the six is a
   finding the implementer owed.**
5. **What the task deliberately did NOT do.** It edits one file. `README.md` and
   `cli.rs` are surfaced rather than edited, and the explanatory shell-convention
   clause was CUT rather than rescoped, on the ground that an unverifiable Windows
   claim must not enter the authoritative document. **Judge that cut**: is the
   surviving bullet complete enough to be the authority `cli.rs` cites?
6. **The no-permanent-checker decision** rests on a cited house record rather than
   on an argument invented here. Run the premise (`proc-no-work-needed-check`):
   read the ROADMAP's "Reach-claim checker" section and say whether it supports
   the decision as the task claims.
7. **Latitude, both forms**, including the inverse. **House dimension** by id.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **The README keeps what the spec cut.** The implementer reports that
   `README.md` still carries the unscoped shell claim ("Interrupt any subcommand
   with Ctrl-C and you get `130`, the shell's own convention"), which the plan
   deliberately kept out of the spec because its Windows half cannot be measured
   here. So the unverifiable claim now lives only in the user-facing document.
   **Is that a contradiction between two live documents, or two true statements
   about different subjects - and does it need a vehicle now or at the close?**
2. **The plan's prose mis-describes its own fence.** Step 2 says the continuation
   indent "aligns under `<profile>`"; the implementer measured it aligning under
   the option list instead. The fence governs and was applied byte for byte.
   **Is the artifact right, and is the plan's descriptive sentence a defect worth
   an amendment or a close-note?**
3. **The divergence table omits `-h/--help`.** The plan's authoring table does not
   list it in its "binary lists" column for any subcommand, which is correct
   (clap-generated help is an accepted exception in spec 8.4) but reads as an
   exhaustive capture. **Does the omission weaken the table as evidence, and did
   the implementer's own table handle it correctly?**
4. **Half a repair, by design.** The `cli.rs` comment's citation now lands, but
   its "every command shares the exit-code contract ... 130 cancelled" stays
   over-broad. **Is leaving that half to a deferred vehicle right, or does a task
   that makes a citation land owe the sentence it lands in?**
5. **Completeness against the acceptance map.** Walk W4-a, W4-b and W4-c and their
   producers. **Is every half produced, and does anything in this task's
   observable surface lack a producing check?**

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a4-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings with `file:line`, the evidence you ran, and
  the exact required change.
- The five adjudications, one explicit verdict each.
- An evidence appendix naming your instrument paths and commands.
- A **HARVEST** section, including what the merge and the whole-branch review
  must carry.

Your final message carries the short form only.
