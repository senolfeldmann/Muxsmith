# Task A2 implementer brief - Plan 11, stream A

**Role:** fresh implementer for Plan 11, Task A2 (W2: the two line-number
citations that survive outside Plan 10's source selector). Model tier: mid
(dispatch model: Opus 5). Effort: xhigh. An independent reviewer grades your
work afterwards; the controller re-runs your claims.

You are the SECOND task of stream A. Task A1 is committed and reviewed
(`a0d5d3e`); A3 and A4 follow you serially in this same worktree. A separate
stream B runs concurrently in a DIFFERENT worktree and never touches your files.

## Preamble (binding)

- **Work in `/home/senol/Git/muxsmith-plan11-a`** (branch `plan-11-stream-a`,
  head `a0d5d3e`). Never on `master`, never in the main worktree, never in
  `/home/senol/Git/muxsmith-plan11-b`. Absolute paths throughout.
- **Never call session-relocation tools** (EnterWorktree/ExitWorktree or any
  equivalent). Do not run `git worktree` at all.
- **Foreground runs only.** No background-run-plus-monitor pattern.
- You are the only writer in your worktree while you run.
- **Read the files, not a commit hash.** Grade and edit the current tree.
- If any step of yours ever needs to mutate a tracked file and restore it, note
  the trap that hit Task A1: `git checkout -- <file>` restores from HEAD, which
  silently discards uncommitted work. Capture the file's CURRENT content as the
  baseline (`sha256sum` plus a copy at a scratch path) and restore to that.
  A bare `cp` is aliased interactive here; use `command cp -f`.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash - in the files you edit and in your report.

## What to read first

1. The plan,
   `/home/senol/Git/muxsmith-plan11-a/docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`:
   the **Global Constraints** section in full, **Task A2** in full (Read-first
   list, EXHAUSTIVE Files list, Steps 1 through 6, "Must not decide"), acceptance
   rows **W2-a through W2-f**, and the **Authoring-time verification** section's
   block "Item 2's corpus: TWO surviving members, not one".
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 2.
3. **Tier-2 `comments-locate-by-symbol-never-by-line-number` in full**, including
   its handle, its SCOPE BOUNDARY sentence and its "WIDENED BY OWNER RULING
   2026-07-29 (session 28)" clause - that is the governing text for this task and
   it reaches CI and configuration comments. Plus Tier-2
   `code-comment-line-citations-drift`.
4. `docs/ROADMAP.md`'s "Docs accuracy" first entry in full, **including its "OPEN
   OWNER QUESTION" paragraph, which the Tier-2 statement supersedes and which the
   CONTROLLER repairs, not this task**.
5. `.github/workflows/ci.yml`'s `cargo doc` step and its leading comment block;
   `crates/muxsmith-core/src/executor/queue.rs` (Step 2 names a symbol out of it);
   `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` and
   `crates/muxsmith-core/tests/profile_save.rs`, whose committed `(design D48)`
   form is the precedent this task follows.

## BINDING CROSS-TASK CONSTRAINT - read this before you touch `ci.yml`

`docs/ROADMAP.md` carries a rider under **"Remove mise from CI (post-1.0)"**
that is gated on "the next `ci.yml`-touching change whichever it is - the edit is
the trigger", and it prescribes an exact replacement text for a comment inside
**the very block your Files list authorizes you to edit**. Its fenced text
contains `BUILDING.md, Rust gate part 6`.

**You do NOT consume that rider, and you do not apply its replacement text.**
Task A1 of this same plan removed `BUILDING.md`'s positional gate ordinals hours
ago, so applying the rider as written would write that ordinal back into
`ci.yml` - re-creating in another file exactly the construction this plan just
deleted, and doing it invisibly, because none of your checks reads `ci.yml` for
ordinals. The controller has already recorded the rider as FIRED AND
DELIBERATELY RE-DEFERRED with a new observable, and the ROADMAP entry says so.

Your edit to `ci.yml` is **only** the two fenced lines in the plan's Task A2
Step 2(a) and nothing else in that comment block.

## The one thing that decides this task

Both fenced replacements are **verbatim**. Your Step-1 re-measurement decides
WHICH sites are in scope, and if it returns a set different from the authoring
run's one-line-each, that set is the ground truth and your report says so. It
never licenses composing a replacement wording that is not written down in the
plan. Three cases return as **NEEDS_CONTEXT** rather than being edited: a hit in
a file the Files list does not name, a hit outside a comment, and a hit the plan
does not fence even when it sits inside one of the two named files.

The symbol in replacement (a) is derived from the **citing commit's parent**,
not from what the cited line holds today. Open both and verify before writing.

The count `17` in the fixture comment is **not** re-measured and **not** changed.

## Carried from Task A1's review, because it binds you too

- **Your free `python3 scripts/ledger-lint.py` run means less than it looks.**
  That gate part reads four markers, the fenced command lines and one sentence.
  It is blind to every prose and comment region you touch. A green run after your
  edit is evidence about YAML integrity and gate arithmetic, not about your
  deliverable. Do not cite it as coverage for your own work.
- **A classification section is an enumeration, and an enumeration is a claim.**
  Where you report a residual set - Step 4's prose-locator blind-spot sweep is
  exactly this - paste the command and its FULL output, then classify line by
  line. Do not describe the remainder in prose. A1's report listed 9 of 13 lines
  its sweep returned and the completeness claim rested on a list that did not
  list what it counted.
- **Keep every instrument you build in the scratch, never in the tree.**

## Exit bar before you commit

Step 4 in full, six checks, every output pasted: absence check A with its fire
and soundness control; absence check B likewise; the prose-locator blind-spot
sweep with every hit classified; the YAML-parse check; the comment-only `-U0`
diff over both files plus `cargo test --workspace` green; `git diff --stat`
naming exactly two files. Plus the weighed test duty as Step 4 states it.

You do **not** run the full gate - the stream runs it once before merge, and that
is the controller's dispatch, not yours.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner for this repo; you do not ask.
Agent commits are deliberately UNSIGNED. Use exactly the fenced commands in
Step 6, and the trailer

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

exactly one trailer, no `Claude-Session` line, no context-window suffix. Stage
explicitly; never `git add -A`. Do not push.

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a2-report.md`
(the MAIN repo path, not your worktree - the scratch is shared and git-ignored).
It carries every command with its pasted output, the Step-5 surfacing list, your
commit SHA, and anything you noticed but did not touch.

Return to the controller only: status
(`DONE` / `DONE_WITH_CONCERNS` / `NEEDS_CONTEXT` / `BLOCKED`), the commit SHA, a
one-line verification summary, and concerns. Not the report body.
