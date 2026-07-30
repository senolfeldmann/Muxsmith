# Task 1 implementer brief - Plan 11.5 (one task)

**Role:** fresh implementer. Model tier: mid (dispatch model: Opus 5). Effort:
xhigh. An independent reviewer grades your work afterwards; the controller
re-runs your claims.

**The change:** add one key to `deny.toml`'s `[advisories]` table so that an
ignore entry which no longer matches any crate in the tree becomes a hard
failure instead of a silent leftover.

```
unused-ignored-advisory = "deny"
```

**Why it exists, because the comment you write has to convey it.** Plan 11 turned
on evaluation of the `unsound` advisory class (`unsound = "all"`) and ignored one
advisory, `RUSTSEC-2024-0429` (glib), with its reason and its drop condition.
Two failure modes follow from that, and this key closes both:

1. If someone later deletes the `unsound = "all"` line, the ignored advisory
   stops being detected, the ignore entry silently refers to nothing, and
   `cargo deny` reports `advisories ok` while the class is no longer checked.
2. When `glib` is eventually fixed or leaves the dependency tree, the ignore
   entry becomes pointless in exactly the same way - and nobody finds out,
   because a suppression that suppresses nothing is invisible.

With this key, both states fail the check by name and line. **That second case is
the one the owner cares about most: it turns "somebody has to remember to look"
into a message the tool sends by itself.**

**Owner ruling, 2026-07-30, and the reason in his words:** we want to hear about
security-relevant findings in transitive dependencies too, not only in our direct
ones. Set it.

## Preamble (binding)

- **Work in `/home/senol/Git/Muxsmith` on `master`.** No branch, no worktree; do
  not run `git worktree`. You are the only writer in this tree while you run.
- Never call session-relocation tools. Absolute paths, **foreground runs only**.
- **A second writer may share this working tree's git index.** Commit with an
  explicit pathspec: `git commit -- deny.toml`, never a bare `git commit`.
- Variant configs for your fire tests go to a scratch path OUTSIDE the
  repository, driven with `cargo deny check advisories -c <path>`. The
  repository's own `deny.toml` is never mutated to produce one; prove it with
  `git diff --exit-code -- deny.toml` afterwards.
- Shell hazards this session has already paid for: a bare `cp` is aliased
  interactive here and blocks - use `command cp -f`. And do not pipe text
  through `echo` if it contains backslash escapes; `printf '%s\n'` or a quoted
  heredoc. A single NUL byte written into a file makes every later `grep` over it
  return nothing at exit 1, which is indistinguishable from an honest no-match.
- **Typography:** ASCII hyphens, straight quotes, no em-dash, no Unicode
  ellipsis.

## What to read first

1. `deny.toml` in full - especially the `[advisories]` table, the comment block
   above `unsound = "all"` (whose register and width your own comment should
   match), and the ignore list's header comment.
2. `cargo-deny`'s own source for this key, under
   `~/.cargo/registry/src/index.crates.io-*/cargo-deny-0.19.9/src/advisories/cfg.rs` -
   **verify what the key actually does at its definition rather than from this
   brief.** If your reading contradicts anything above, that is NEEDS_CONTEXT
   with both readings pasted, not a silent adjustment.
3. `docs/ROADMAP.md`, the Triggers entry that begins "`cargo deny check` fails
   naming `RUSTSEC-2024-0429`" - it is the controller-owned record of what this
   key is for, and your comment must not contradict it. **Do not edit it.**
4. `BUILDING.md`'s Rust gate block, to confirm `cargo deny check` is invoked
   there unchanged - **the invocation is not touched by this task.**

## What you decide, and what you do not

- **The key and its value are decided** (`unused-ignored-advisory = "deny"`) and
  are not yours to change.
- **The comment beside it is YOURS to write.** It must convey: what the key does;
  that it exists so a dropped `unsound` scope and an obsolete ignore entry both
  become loud instead of silent; and that the expected day-to-day consequence is
  a failing check telling you to delete an ignore entry that has served its
  purpose. Match the file's existing comment register and line width. Do not
  restate the ROADMAP entry; point at nothing that will go stale.
- **Placement is yours**, inside the `[advisories]` table. State the postcondition
  you assert afterwards rather than a position - this project spent a fix round
  on an instruction that named a place and became unperformable when the file
  changed underneath it.

## Verification, all outputs pasted

Four runs, each with its own scratch config where a variant is needed, each with
its real exit code (**not through a pipeline - `$?` after a pipe is the pipe's
status, an error this session already made**):

1. **Shipped state after your edit:** `cargo deny check advisories` exits 0,
   `advisories ok`.
2. **The regression this key exists to catch:** a copy with the `unsound = "all"`
   line deleted must exit **1** and name the ignore line with
   `no crate matched advisory criteria`.
3. **The obsolescence case:** a copy whose ignored id is replaced by an advisory
   id that applies to no crate in this tree must exit **1** the same way. This is
   the case the owner asked about - it demonstrates that the eventual glib fix
   announces itself.
4. **The control that the key is what does the work:** a copy WITHOUT the new key
   and with the `unsound` line deleted must exit **0**. Without this, runs 2 and 3
   are equally consistent with a config that would have failed anyway.

Then: `python3 scripts/ledger-lint.py` green, `git diff --stat` naming exactly one
file, and `git diff -U0 -- deny.toml` pasted in full showing only your addition -
no existing ignore id reworded, reordered or removed.

**You do not run the full gate**; the controller runs it before the push.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner for this repo; you do not ask.
Agent commits are deliberately UNSIGNED:
`git -c commit.gpgsign=false commit -- deny.toml`, with exactly one trailer,
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line. Stage explicitly; never `git add -A`. **Do not push.**

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11.5/task-1-report.md`: every
command with its pasted output and real exit code, your comment text with the
reasoning for its wording, your commit SHA, and anything you found that
contradicts this brief.

Return to the controller only: status, the commit SHA, a one-line verification
summary, and concerns.
