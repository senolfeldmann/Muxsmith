# Task B1 implementer brief - Plan 11, stream B

**Role:** fresh implementer for Plan 11, Task B1 (W1: the two open dependency
alerts - one lockfile bump, one `cargo deny` configuration repair, one
investigation that produces a finding rather than a fix). Model tier: mid
(dispatch model: Opus 5). Effort: xhigh. An independent reviewer grades your
work afterwards; the controller re-runs your claims.

You are the ONLY task of stream B. A separate stream A runs concurrently in a
DIFFERENT worktree; its files and yours are pairwise disjoint, and stream A
merges first.

## Preamble (binding)

- **Work in `/home/senol/Git/muxsmith-plan11-b`** (branch `plan-11-stream-b`).
  Never on `master`, never in the main worktree, never in stream A's worktree.
  Absolute paths throughout.
- **Never call session-relocation tools** (EnterWorktree/ExitWorktree or any
  equivalent). Do not run `git worktree` at all - the controller owns worktree
  lifecycle.
- **Foreground runs only.** No background-run-plus-monitor pattern. Your gate
  run is long; run it in the foreground anyway.
- A fresh worktree carries neither `target/` nor `node_modules/`, so your first
  runs pay a cold `pnpm install` and a cold cargo build. That cost is the price
  of the stream split and is not re-argued.
- **Read the files, not a commit hash.** Grade and edit the current tree.
- The two variant `deny.toml` configurations of Step 5 are **copies written to a
  scratch path OUTSIDE the repository** and driven through
  `cargo deny check advisories -c <path>`. The repository's own `deny.toml` is
  never mutated to produce them; you prove that with
  `git diff --exit-code -- deny.toml` after the variants.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash - in the files you edit and in your report.

## What to read first

1. The plan,
   `/home/senol/Git/muxsmith-plan11-b/docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`:
   the **Global Constraints** section in full, **Task B1** in full (its Read-first
   list, its EXHAUSTIVE Files list and its "No other file is written" paragraph,
   the boundary-reversal paragraph, Steps 1 through 11, and its "Must not decide"
   line), and acceptance rows **W1-a through W1-m** in the acceptance map, which
   are what your evidence has to satisfy. Also the **Authoring-time verification**
   section's four stream-B blocks in full ("The two alerts, re-verified at the
   source", "`postcss`: the lockfile move is available and its mechanism is
   measured", "`cargo deny`: one advisory, and a default scope that excluded it",
   "`glib`: eleven parents, all one generation").
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 1 in full.
3. `docs/ROADMAP.md`: the **"TWO OPEN VULNERABILITY ALERTS"** entry in the
   Pre-1.0 release gates section in full, including its exposure analysis and its
   RULED block; the **v1.x-candidates entry for `glib` unsoundness
   RUSTSEC-2024-0429**, which carries the owner's 2026-07-30 ruling and which your
   Step-4 `deny.toml` comment must agree with; the **"Dependabot/Renovate
   activation"** entry for the two riders.
4. **`deny.toml` in full** - Step 4 edits two named regions of it, Step 5 fires
   against it.
5. **cargo-deny 0.19.9's own `src/advisories/cfg.rs` and `src/cfg.rs`** under
   `~/.cargo/registry/src/index.crates.io-*/`. A claim about a default is
   verified at the `Default` impl, never at the tool's output. This is the
   defect that cost this plan a blocking review finding once already.
6. **`BUILDING.md`'s Rust gate block** - `cargo deny check` is one of its
   commands and you must not change the invocation. It is also the authoritative
   enumeration of the full gate you run in Step 9.
7. `package.json` and `pnpm-lock.yaml` - parts of Step 2 are assertions about
   them.
8. Tier-2 entries: `ci-04-dependabot-cadence`, `ci-10-pin-everything`,
   `gate-includes-cross-target-lint-for-the-unrun-os`,
   `proc-07-verify-against-source`, `proc-no-work-needed-check`.

## Three things that decide this task

1. **The landing `postcss` version is measured, never fenced.** The requirement
   is `>= 8.5.18`. The authoring probe landed on `8.5.24` while the registry's
   `latest` read `8.5.25`; you paste what you observe. A third package moving in
   the lockfile diff is a **finding**, not a pass; a package that is not
   reachable from postcss returns as **NEEDS_CONTEXT**.
2. **A single green `cargo deny` run proves nothing.** Step 5 is three runs -
   shipped state green, scope-live failing, and the control that separates "the
   scope is on" from "the ignore entry is load-bearing". Any deviation from the
   three prescribed outcomes is a finding with its pasted output, not a config to
   adjust. The scope VALUE `all` is decided in the plan with its reasoning and is
   not yours to change.
3. **Part (c) is an investigation whose finding IS the result.** No upgrade is
   attempted, `Cargo.lock` is not edited, no `[patch]` section is added, no Tauri
   version changes - whatever the measurement shows. "An upgrade project in
   someone else's tree, not a bump" is the acceptable completion.

If a transitive parent constrains `postcss` below the patched version, that is
**NEEDS_CONTEXT with options and costs**. There is deliberately no
pre-authorised fallback - an override would be a sanctioned fork.

## Exit bar before you commit

Step 9 in full: **the full gate as `BUILDING.md` enumerates it, foreground,
green, in your worktree** - this is the one task in the plan whose own change
can move a gate part for reasons outside its diff, so its exit bar is the whole
gate rather than a subset. Plus `git diff --stat` naming exactly two files, and
the weighed test duty as Step 9 states it.

## Commit (SI-4, standing owner grant for this repository)

Commits are standing-authorized by the owner for this repo; you do not ask.
Agent commits are deliberately UNSIGNED. Use exactly the fenced commands in
Step 10, and the trailer

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

exactly one trailer, no `Claude-Session` line, no context-window suffix. Stage
explicitly; never `git add -A`. Do not push - the controller pushes once, at the
plan close.

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-b1-report.md`
(note: the MAIN repo path, not your worktree - the scratch is shared and
git-ignored). It carries: every command you ran with its pasted output, Step 6's
four statements in their own terms, Step 8's full measurement, Step 11's
surfacing list, your commit SHA, and anything you noticed but did not touch.

Return to the controller only: status
(`DONE` / `DONE_WITH_CONCERNS` / `NEEDS_CONTEXT` / `BLOCKED`), the commit SHA,
a one-line verification summary, and concerns. Not the report body.
