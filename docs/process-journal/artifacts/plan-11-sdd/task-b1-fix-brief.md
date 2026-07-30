# Task B1 fix-round brief - Plan 11, stream B

**Role:** fresh implementer for Task B1's fix round. Model tier: mid (dispatch
model: Opus 5). Effort: xhigh. You did not write the original commit; the
findings below are written out to the line, which is why this goes to fresh eyes
rather than to a resumed agent.

**What happened.** Task B1 shipped `c422999`, which was reviewed
APPROVED_WITH_MINORS. Its three Important findings were defects in the PLAN's
fenced content, not in the implementation - the implementer applied the fence
verbatim as required and reported the contradictions. **Amendment 5 has since
replaced that fence**, went through its own independent delta review, took a fix
round of its own, and is what you now apply.

**The shipped `deny.toml` comment asserts two independent falsehoods** in its
first two sentences, and both are user-visible in the sense that matters here -
they explain, wrongly, why a security gate behaves as it does:

- it attributes the ignore-ENTRY count (18) to the `unmaintained` CLASS, which
  reported 16, with 2 more coming from the vulnerability class;
- it says the `workspace` scope "excludes every external crate", which is not
  what the filter tests - it keys on the advisory crate's DIRECT DEPENDENTS and
  asks whether any of them is a workspace member, so an external crate a
  workspace member depends on directly IS in scope and fires.

## Preamble (binding)

- **Work in `/home/senol/Git/muxsmith-plan11-b`** (branch `plan-11-stream-b`,
  head `c422999`). Never on `master`, never in the main worktree, never in
  `/home/senol/Git/muxsmith-plan11-a`. Absolute paths throughout.
- **Never call session-relocation tools.** Do not run `git worktree`.
- **Foreground runs only.**
- **Do not rewrite history.** Your repair is a NEW commit on top of `c422999`,
  not an amend.
- Variant configs for the fire go to a scratch path OUTSIDE the repository and
  are driven with `cargo deny check advisories -c <path>`; the repository's own
  `deny.toml` is never mutated to produce one.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash.

## What to read first

1. **The plan's Task B1 Step 4(a) AS AMENDED**, in
   `/home/senol/Git/muxsmith-plan11-b/docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
   - **read it from `master`, not from your worktree**, because Amendment 5 lives
   on `master` and your branch predates it. Read the blob:
   `git -C /home/senol/Git/Muxsmith show master:docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`.
   Also read that file's `## Amendment 5` section, which states what changed and
   why.
2. `.superpowers/sdd/plan-11/task-b1-verdict.md` - findings 1 and 2 and
   adjudication 1(b), whose wording the replacement fence is.
3. `.superpowers/sdd/plan-11/amendment-5-verdict.md` - in particular its
   finding 1, which is the reason the instruction is a REPLACEMENT.
4. `deny.toml` in your worktree, as `c422999` left it.

## The operation, and the trap that was found before you hit it

**This is a REPLACEMENT of the eight lines between the two anchors, not an
insertion.** The plan's original instruction said "insert between these anchors",
which was correct for a pristine file; your file already contains the OLD fence -
including its own `unsound = "all"` line - sitting between exactly those anchors.
The amendment's reviewer fired both readings against your file's state: a literal
insert makes `cargo deny` fail with `failed to parse config: duplicate key:
unsound`, while replacing the region exits 0.

**Postcondition to assert after the edit: exactly one `^unsound = ` line in
`deny.toml`.** Paste the check.

## Also in scope: your predecessor's report

`.superpowers/sdd/plan-11/task-b1-report.md` still asserts, at two places, the
claim the review refuted (that dropping the scope key would be a SILENT
regression - measured false: cargo-deny emits `warning[advisory-not-detected]`
naming the exact ignore line), and it carries two numbered findings where its
verdict required a third.

**Controller ruling on how to fix it, because the two options are not
equivalent:** that report is a DATED RECORD of what an implementer measured and
claimed, and this project does not falsify dated records. **Annotate, do not
rewrite.** Add a clearly marked, dated erratum block at each of the two sites and
at the findings list, stating what was claimed, what was measured, and by whom -
the same form the controller used on Task A1's brief when its own instruction
turned out wrong. The original sentences stay legible.

## Exit bar before you commit

- The three-way `cargo deny` fire re-run against the NEW fence, all three runs
  pasted with their exit codes: shipped state green; the scope live (ignore entry
  removed) failing with `ID: RUSTSEC-2024-0429`; the control with BOTH the scope
  key and the ignore entry removed exiting 0. **The fence changed, so the fire is
  re-run rather than cited from the first round.**
- `git diff -U0 -- deny.toml` over your new commit alone, pasted: comment text
  only, no existing ignore id reworded, reordered or removed, and the
  `unsound = "all"` line itself unchanged in value.
- `git diff --stat` over your new commit names exactly one file.
- The whole-file check that the two falsehoods are gone and nothing else moved:
  compare `deny.toml` at `c422999` against your end state and account for every
  changed line.

You do **not** run the full eleven-part gate - the controller runs it on the
merged state.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner; you do not ask. Agent commits are
deliberately UNSIGNED:
`git -c commit.gpgsign=false commit -- deny.toml`, exactly one trailer,
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line, no context-window suffix. Stage explicitly; never `git add -A`. Do not
push. The report erratum is a separate matter - that file is git-ignored scratch.

## Report contract

Append your fix report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-b1-report.md` under a
clearly marked fix-round heading (this is the same file you are annotating; keep
the erratum blocks and the fix report distinguishable).

Return to the controller only: status, the commit SHA, a one-line verification
summary, and concerns.
