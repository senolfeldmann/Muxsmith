# Task 1 review brief - Plan 11.5

**Role:** independent reviewer of a single small change: one key added to
`deny.toml`'s `[advisories]` table, `unused-ignored-advisory = "deny"`, so that an
ignore entry matching nothing becomes a hard failure instead of a silent
leftover. You did not write it. Model tier: mid (dispatch model: Opus 5).
Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content in short form as your final message.

The change is small; the reason it still gets a full review is that it changes
what a security gate part reports, and the owner ruled it explicitly today.

## Preamble (binding)

- Never call session-relocation tools. Absolute paths, **foreground runs only**.
- Grade in `/home/senol/Git/Muxsmith` on `master`, head `937ae42`, parent
  `4b01cb6`. Do not enter the six legacy worktrees under `.worktrees/`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/p115rev-independent/`
  (create it). Never re-run a config variant the implementer wrote; build your
  own from the committed `deny.toml`.
- Never mutate the repository's own `deny.toml` to produce a variant; drive
  variants with `cargo deny check advisories -c <path>`. Prove the repo file
  untouched by hash, not by `git diff --exit-code` - that command exits 1 by
  construction here, which is one of the defects the implementer already found in
  the controller's brief.
- **Exit codes are read without a pipeline.** `$?` after a pipe is the pipe's
  status. This session has already made that error once.
- The tree must be byte-identical to `937ae42` when you finish. Prove it.

## Ground truth

1. `cargo-deny` 0.19.9's own source under
   `~/.cargo/registry/src/index.crates.io-*/cargo-deny-0.19.9/` - the key's
   definition, its default, and the path that turns it into a non-zero exit.
   **This is the authority, not the brief and not the report.**
2. `deny.toml` as committed.
3. `docs/ROADMAP.md`'s Triggers entry beginning "`cargo deny check` fails naming
   `RUSTSEC-2024-0429`" - the controller-owned record of what this key is for.
   The shipped comment must not contradict it.
4. The house-knowledge YAML files, cited by id.

`.superpowers/sdd/plan-11.5/task-1-brief.md` (the controller's brief) and
`task-1-report.md` are **evidence, not ground truth** - and the brief is known to
contain at least three defects the implementer found and worked around. Judge
whether its workarounds were the right ones.

## Dimensions

1. **Does the key do what the comment says it does?** Verify at the source, then
   demonstrate. Build your own variants: shipped state; the `unsound` scope line
   deleted; an ignore entry that has stopped matching while the rest of the list
   is untouched; and for each failing case a no-key control proving the key is
   what converts it. Report every exit code.
2. **The confounding the implementer flagged, adjudicated by you.** The
   controller's brief prescribed simulating obsolescence by REPLACING the glib
   id, which also unignores glib and therefore produces an `error[unsound]` of
   its own - the run would exit 1 with or without the key. The implementer ran a
   second, unconfounded form instead (glib entry kept, a non-matching advisory
   added alongside). **Was that the right call, and is its evidence sufficient to
   support the claim the comment makes?**
3. **The stated limitation, checked rather than accepted.** The report says an
   ignored id absent from every advisory database (a typo, a withdrawn advisory)
   takes a different path and stays a warning at exit 0, outside this key's
   reach. Verify it, and say whether the shipped comment over-claims relative to
   that limit.
4. **The comment itself.** It is the implementer's own wording against a
   requirement rather than a fenced string. Is it accurate, is it the right
   register and width for this file, and does it tell a future reader the one
   thing that matters - that a failure here usually means "delete an ignore entry
   that has done its job", not "revert this key"?
5. **Scope.** One file, additive only, no existing ignore id reworded, reordered
   or removed, the `cargo deny` invocation in `BUILDING.md` and `ci.yml`
   untouched, and the gate's command count unchanged (a config key is not a
   command).
6. **The no-work-needed check**: wherever the report concludes something was
   unnecessary or impossible, run the premise.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **Is this change complete as a guard, or does it advertise more than it
   delivers?** Given the limitation in dimension 3, someone could read the
   ignore list as fully self-policing when it is not. Does that need anything -
   a sentence in the comment, a tracker line - or is the comment's current scope
   already honest?
2. **The controller's brief was wrong in three places** (a confounded test, a
   verification command that cannot hold, and a run-3 design that proves nothing
   on its own). The implementer worked around all three and reported them rather
   than silently complying. **Was each workaround correct, and did any of them
   quietly change what the task asserts?**
3. **Timing risk.** The report notes Renovate's first dependency PRs are expected
   within days, and that one moving the gtk-rs generation would make the next
   gate run fail with this new error. **Is the project ready for that failure to
   be read correctly by whoever meets it first, or does something need to say so
   more loudly than a comment in the file?**

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11.5/task-1-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES), numbered severity-tagged
findings with the evidence you ran, the three adjudications, an evidence appendix
with your instrument paths, and a HARVEST section.

Your final message carries the short form only.
