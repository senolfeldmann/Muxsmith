# Amendment 3 review brief - Plan-9 design

**Role:** independent reviewer of amendment 3 to the Plan-9 design document.
You did not write it. Model tier: top (dispatch model: Fable 5). Effort: xhigh.

**You commit nothing and edit no file under `docs/` or `crates/`.** Your output
is a verdict file plus your final message.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Repo `/home/senol/Git/Muxsmith`, `master`, main worktree.
- Absolute paths, foreground runs only.
- **Do not pin your reading to a commit hash I give you.** The amendment is
  commit `08621cb`; house-knowledge commits have landed after it and more may
  land while you work. Read the FILES, and use `git show 08621cb` for the diff.
- **Independent instruments.** Any claim of the author's that you reproduce,
  you reproduce with your own extraction, your own grep, your own scratch
  file - at a path the author could not have written. Your scratch root:
  `/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/a3rev-independent/`.
  A re-run of someone else's instrument agrees by construction.
- Any check whose pass is an absence needs its fire. Any number or quotation
  in your verdict is measured or copied, never recalled.

## What was ruled, and by whom

Plan 9's Task 2 hoisted `run_batch` into `muxsmith_core::executor::queue`,
carrying its rustdoc verbatim as D96 and the plan required. In the same commit
the function gained a second caller (the CLI), and three passages of that doc
became false about the function they document - raised as MEDIUM-1 by the
Task-2 reviewer (`.superpowers/sdd/plan-9/task-2-verdict.md`).

**The owner ruled 2026-07-28** that this is a DESIGN change, not an ordinary
truthfulness fix: it enters the design as amendment 3 and the code edit rides
Task 3. The rejected alternatives (separate fix vehicle; defer to 1.x) are
settled - do not re-open them, and do not grade the ruling.

The controller brief the author worked from is
`.superpowers/sdd/plan-9/amendment-3-brief.md`; its report is
`.superpowers/sdd/plan-9/amendment-3-report.md`. **Both are evidence, not
ground truth.**

## Ground truth

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (v1 spec,
   authoritative on conflict).
2. The Plan-9 design as amended, especially D96 and its new amendment-3 rider,
   section 5's D96 bullet, and the `## Amendment log`.
3. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`
   (Task 3's Files list and steps are what the follow-on plan amendment will
   touch).
4. The code the doc describes: `crates/muxsmith-core/src/executor/queue.rs`,
   `crates/muxsmith-cli/src/commands/run.rs`, `src-tauri/src/run.rs`.
5. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

## Dimensions

1. **Does the replacement fence tell the truth, for BOTH callers?** Check each
   sentence against the code, not against the old text. The claims to test at
   the source include: the tee order (logger before `on_event`), the
   synchronous-return property, the GUI's detached runner thread and the CLI's
   blocking call, "renders its human milestone lines" under `--json`, the
   index-alignment and still-open-logger contract, and the D23 reference.
2. **Is anything true lost?** The restatement may not drop a fact that still
   holds. Two specific ones to rule on: the removed "a plain collector in
   tests" clause, and the dropped `finish_teardown`/D31 rationale - the author
   argues it survives caller-side in three places
   (`src-tauri/src/run.rs` around `:641-650`, `:656-667`, `:447-454`); verify
   that, and rule whether a core reader is left with enough.
3. **Latitude, in both forms.** The fence must be transcribable: no ellipsis,
   no mandated-but-unwritten passage, no set left open, nothing a Task-3
   implementer would have to invent. Also check the rider's own instructions
   for an explicit permission.
4. **Coverage.** Does the amendment do everything the ruling requires, and
   does the design still say one thing? Specifically: is there any remaining
   design text that still mandates the OLD doc (the author swept for
   "moves as-is" and reports exactly one hit, now qualified - re-run that
   sweep yourself), and does the amendment-log entry match the log's existing
   shape?
5. **The completeness pass.** The author claims nothing else in `queue.rs`
   went caller-stale, from a keyword grep plus a read of every top-level doc
   block. Re-run it your own way - a negative is only evidence if your own
   instrument can fire - and say whether you agree, naming anything it missed.
6. **House conformance.** Doc style and line width against the file's
   neighbours; the intra-doc link set unchanged (the author claims no new link
   surface); typography (ASCII hyphens, straight quotes, no Unicode ellipsis);
   `proc-48-docsurface-delink` respected (no link resurrected to a private or
   unreachable item).
7. **Consequences for the plan amendment.** The author says Task 3 needs one
   added instruction in `queue.rs` (already on its Files list) plus a
   "Read first" addition, and NO new file. It also reports that my brief was
   wrong to imply `src-tauri/src/run.rs` sits on Task 3's exhaustive Files
   list. Verify both, because the next dispatch builds on them.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/amendment-3-verdict.md`
and make your final message the same content (read as data):

1. **Verdict**: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
2. **Findings**, numbered, severity-tagged (BLOCKING / MEDIUM / LOW), each with
   file:line, the evidence you ran, and precisely what must change.
3. **Rulings** on dimension 2's two named drops.
4. **Evidence appendix** with your instrument paths named.
5. **HARVEST**: observed patterns, repeated rejections, and anything the plan
   amendment or Task 3 must carry.
