# Task 3 implementer brief - Plan 9

**Role:** fresh implementer for Plan 9, Task 3 (the worker-panic payload, end
to end: D98, D99, D100, plus D96's amendment-3 rustdoc rider; spec S-1's
WorkerPanicked row and S-2). Model tier: mid (dispatch model: Opus 5). Effort:
xhigh. An independent reviewer grades your work afterwards; the controller
re-runs your claims on disk.

This is the largest task in the plan: it crosses Rust core, the CLI, both
Fluent locales, the TypeScript IPC types, a Vue component and the e2e
fixtures. Eleven steps. Read all of them before starting.

## Preamble (binding, every dispatch)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). You work on `master` in the main worktree
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths in every command; **foreground runs only** - no
  background-run-plus-monitor pattern, no `&`.
- You are the only writer in this tree while you run (the plan's serial
  ruling).
- **Do not pin your reading to a commit hash.** Read the files.

## What to read first (in this order)

1. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` -
   Global Constraints, then **Task 3** in full (Steps 1-11, the Files list, the
   "Must not decide" list). The plan is the contract.
2. `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md` -
   **D98** (field, wire memo, fork 9, fork 12), **D99** (both rejected
   alternatives, the four Fluent fences, the catalog obligations), **D100**
   (render-site fence, scope boundaries), **D96's amendment-3 rider** (the
   replacement `run_batch` doc comment - your Step 2 transcribes it), design
   section 0 notes 4 and 5, section 2's "stay discarded" paragraph, section 5
   (the panic bullets and the amended D96 bullet), and the `## Amendment log`
   at its current state.
3. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - authoritative
   above the design on conflict. Your Step 9 amends two of its passages.
4. The four house-knowledge files as ground truth alongside them; cite entries
   by id, re-verify any `:line`.

**The replacement doc comment is NOT reproduced in this brief and not in the
plan.** It lives once, in D96's amendment-3 rider, and you transcribe it from
there character for character, wrapping included. Two copies of a
character-for-character contract drift; that is why there is one.

## Scope

Exactly the plan's Task 3: its Files list (exhaustive except the one entry the
plan marks EXEMPLARY - the compiler-flagged `JobOutcome` constructors), its
Steps 1-11, its "Must not decide" list. Nothing else in the tree.

## Carried in, verbatim

- **From Task 2's interfaces:** "the four silently-discarded executor failures
  (`job.rs` create_dir_all, `joblog.rs` remove_dir_all, `spawn.rs` kill,
  `spawn.rs` wait) STAY discarded - the recorded steelman of `exec-36`'s ruled
  no-facade position; no task 'improves' them in passing." This task edits
  files where they live. Do not touch them.
- **Into Task 6:** "the RunHistory export stays raw-output-only and the history
  table is unchanged; the ruled render surface is the live job row."

## Anchors, measured by the controller at commit `63fc5b2`, 2026-07-28

The plan's authoring-time line numbers for `queue.rs` are stale - Task 2's
hoist shifted the file. Verify these yourself and **locate by content, never by
line number** (`proc-57-briefs-not-ground-truth`; refuting a premise with
evidence is a valid completion). Pasted from the runs:

```
$ grep -n "fn recover_panicked_worker\|eprintln!\|fn worker_panic_is_reported_as_failed_not_cancelled" crates/muxsmith-core/src/executor/queue.rs
424:fn recover_panicked_worker(
441:    eprintln!("muxsmith: worker thread panicked while running job {index}: {message}");
783:    fn worker_panic_is_reported_as_failed_not_cancelled() {

$ grep -rn "eprintln!\|println!\|print!(" crates/muxsmith-core/src        # Step 10's fire, pre-state
crates/muxsmith-core/src/lib.rs:24:// literal at ~21 `eprintln!` sites across muxsmith-core, muxsmith-cli and
crates/muxsmith-core/src/executor/queue.rs:441: (the one call site)
      -> exactly 2 hits; the post-state expectation is 1 (the comment only)
$ same grep over crates/muxsmith-cli/src                                  # known-present control
      -> 6 files
```

**A correction to what Task 2's implementer wrote about your task:** its report
claimed your `JobOutcome` compiler sweep must also cover the two tests it moved
into `queue.rs`. Its reviewer measured that and refuted it - neither moved test
mentions `JobOutcome`; they read `outcomes[0].state` by field access, which a
new field does not break. Let the compiler enumerate the real set (plan Step 3),
do not hunt by eye, and do not assume that claim.

## Standing rules that bind this dispatch

- **No design latitude, in either form** (`proc-latitude-clause-boundary`):
  neither an explicit permission nor an omission - an unenumerated set in a
  normative position, a list left open, a name/string/file you would have to
  invent. A fork discovered on code contact returns as **NEEDS_CONTEXT with a
  decision memo** (options, costs against the named invariants, a
  recommendation). You do not resolve it at the keyboard and report afterwards.
- **Structural-conformance grant, as the owner amended it on 2026-07-28**
  (`latitude-carveout-zero-content-structural-forks` in
  `docs/process-conventions.yaml` - read the entry, do not work from this
  summary): following the touched file's existing structural patterns is in
  scope where the extension has zero outward effect (no API/symbol surface, no
  data-format change across a serialization boundary, verification never
  weakened, nothing user-visible). The boundary of a Files list runs over
  FILES: an entry constrains work inside its file only where it carries an
  explicit within-file qualifier. Repairing a reference that **your own
  enumerated edit invalidated** - a doc link, a comment referent, an import -
  inside a LISTED file is named in scope. The same repair in an unlisted file
  returns as a finding. Weakening, deleting, skipping or rewording existing
  assertions, mutating existing fixture values, new test files and new test
  infrastructure all stop and return.
- **The import-removal doc-link sweep**
  (`an-import-removal-sweeps-the-doc-links-that-named-the-symbol`): if your
  diff deletes a `use` line, grep the files you touch for intra-doc links
  naming the removed symbols and re-point or de-link each hit in the same
  change. This class has fired in both previous tasks of this plan, in the
  files you are about to edit, and the plain rustdoc gate cannot see it where
  the item or its module is private.
- **No task edits any house-knowledge YAML.** Surface ledger-worthy
  observations in your report; the controller is the single writer.
- **Counts recomputed from their enumerations**; **absence checks fire-verified
  with a reachable green state**; **every observed value pasted from the run
  that produced it**.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis - in
  code, comments and report. The Fluent fences contain correct German
  orthography (`ä`, `ü`, `ß`); those are letters, not glyph tells, and are
  copied exactly.

## Verification bar before the commit (plan Step 10, foreground, no subsets)

All green and pasted, including the fires:

- Acceptance observable 4's Task-3 emitters as the design's section 7 item 4
  states them, with the core-stdio absence check and both its fire and its
  known-present control (measurements above).
- `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`. Existing insta snapshots must NOT churn: the
  `panic: None` rendering path is byte-unchanged, so any snapshot diff is a
  defect, not something to accept.
- `pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e`.
- For Step 2 specifically: after transcribing, diff the new `///` block against
  the rider's fence and show the result is empty, and show the rest of the
  function is byte-identical.

## Commit (SI-4, restated because you cannot see the grant)

Commits and pushes on this repo are **standing-authorized by the owner**; your
global never-commit default does not apply. You commit, you do not push.

- Stage exactly the plan's pathspec plus any file the Step-3 compiler sweep
  touched, each by name - **never `git add -A`**.
- `git -c commit.gpgsign=false commit ...`, pathspec-scoped.
- Message: the plan's Task-3 message.
- Exactly one trailer, no `Claude-Session` line:
  `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-3-report.md` and
end your final message with the same content (read as data, not chat):
status (DONE / DONE_WITH_CONCERNS / NEEDS_CONTEXT); what changed per file
against the Files list, including anything the list did not predict; the
evidence with pasted commands, output and fires; divergences and judgment calls
under the grant, each named; numbered concerns a reviewer can rule on yes/no;
what you surface for the controller; the commit hash and `git show --stat`.
