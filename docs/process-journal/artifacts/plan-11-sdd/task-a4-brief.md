# Task A4 implementer brief - Plan 11, stream A

**Role:** fresh implementer for Plan 11, Task A4 (W4: the v1 spec's section 8.1
states the shipped CLI surface). Model tier: mid (dispatch model: Opus 5).
Effort: xhigh. An independent reviewer grades your work afterwards; the
controller re-runs your claims.

You are the LAST task of stream A. A1 (`a0d5d3e`), A2 (`5d305a2`) and A3
(`164e571`) are committed and reviewed. A separate stream B is in a DIFFERENT
worktree and never touches your file.

**You edit exactly one file** - the v1 spec - in exactly two regions: section
8.1's fenced synopsis block and its exit-code bullet. **Task A3 has just amended
four OTHER sections of that same file** (4.3, 4.4, 7 and 9.2), which is why A4
runs after it: your self-contradiction sweep runs over the FINAL text rather than
over a text about to change.

## Preamble (binding)

- **Work in `/home/senol/Git/muxsmith-plan11-a`** (branch `plan-11-stream-a`,
  head `164e571`). Never on `master`, never in the main worktree, never in
  `/home/senol/Git/muxsmith-plan11-b`. Absolute paths throughout.
- **Never call session-relocation tools.** Do not run `git worktree`.
- **Foreground runs only.**
- You are the only writer in your worktree while you run.
- If a step of yours mutates a tracked file to fire a check, capture the file's
  CURRENT content as the baseline and restore to that - `git checkout -- <file>`
  restores from HEAD and would discard uncommitted work. `command cp -f`, never
  a bare `cp` (aliased interactive here).
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash.
- **The plan document on `master` has since gained Amendment 5**, which touches
  Tasks A1 and B1 only - nothing in A4's territory. Your contract is the plan
  copy in your worktree.

## What to read first

1. The plan, in your worktree: the **Global Constraints** section, **Task A4** in
   full (Steps 1 through 7, its Read-first list, "Must not decide"), acceptance
   rows **W4-a through W4-c**, and the authoring section's block "Item 4's
   corpus: the spec's 8.1 block is stale in four of its five lines".
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 4.
3. `docs/ROADMAP.md`'s "Docs accuracy" entry beginning "The v1 spec's section 8.1
   synopsis".
4. **The v1 spec's section 8 in full**; `README.md`'s "Using the CLI" section
   (the consistency target, which you do NOT edit); `crates/muxsmith-cli/src/cli.rs`;
   `crates/muxsmith-cli/src/commands/run.rs`; `crates/muxsmith-cli/src/commands/mod.rs`;
   D16 in `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`.
5. Tier-2 `proc-04-spec-wins`, `code-comment-line-citations-drift`,
   `proc-wrapped-prose-quote-grep`, `proc-check-green-state-reachable`.

## The one thing that decides this task

**The surface is re-derived from the SHIPPED BINARY, not from the source and not
from this plan.** Build or locate `target/debug/muxsmith`, prove it is not stale
(`find crates src-tauri -name '*.rs' -newer target/debug/muxsmith` must print
nothing - and note that A3 just changed `matcher.rs`, so a rebuild is likely
required), then capture `--help` for the top level and for every subcommand it
lists. **Every correction the plan states must be REPRODUCED by your own
derivation before you write it; a correction your run does not reproduce is a
finding, not a silent drop.**

Exit codes are not in the help text: derive them from the named symbols in
`cli.rs`, `commands/mod.rs` and `commands/run.rs`, and name each symbol with its
file in your report.

## Carried from the three preceding reviews, because it binds you

- **Reconstruct, do not inspect.** Rebuild the end-state file from its `164e571`
  blob plus the plan's two fenced substitutions and compare byte for byte. Its
  precondition is that each fenced OLD block occurs exactly once - check it, and
  note that A3's reviewer measured the spec line carrying BOTH a repaired and a
  retained clause, so this file is one where a file-level identity assumption
  would be wrong.
- **Stay off the sections A3 amended.** The A3 reviewer's harvest names the spec
  sites A3 touched; your two regions are in section 8.1 and nowhere else.
- **The sweep is an enumeration, not a reading.** Step 4 pre-classifies its six
  expected hits so you reconcile rather than report. Paste every command and its
  FULL output, and give every hit a stated verdict. A hit outside the six is a
  finding.
- **A hard-wrapped quotation does not grep as one string.** Step 6's surfacing
  target is wrapped across two `///` lines; locate it by a fragment that sits on
  one line. This exact trap has now caught two agents in this plan, one of them a
  reviewer who had the counter-instrument available.
- **Your exit bar DOES include `ledger-lint`**, unlike A2's. The A2 adjudication
  that its green means little is not licence to drop it here.

## Exit bar before you commit

The four Step-4 checks with every command and full output pasted; the
no-permanent-checker decision recorded with its cited ground (and NEEDS_CONTEXT
if your reading of the ROADMAP's "Reach-claim checker" section does not support
it - never a checker built at the keyboard); `python3 scripts/ledger-lint.py`
green; `git diff --stat` naming exactly one file; the weighed test duty.

You do **not** run the full eleven-part gate - the controller runs it on the
merged state.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner; you do not ask. Agent commits are
deliberately UNSIGNED. Use exactly the fenced commands in Step 7, and the trailer

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

exactly one trailer, no `Claude-Session` line, no context-window suffix. Stage
explicitly; never `git add -A`. Do not push.

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-a4-report.md`
(the MAIN repo path, not your worktree). Every command with its pasted output,
the divergence table at flag granularity, the Step-6 surfacing, your commit SHA,
and anything you noticed but did not touch.

Return to the controller only: status, the commit SHA, a one-line verification
summary, and concerns.
