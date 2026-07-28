# Amendment 3 brief - Plan 9 design (owner ruling 2026-07-28)

**Role:** author of amendment 3 to the Plan-9 design document. You write the
amendment; an independent reviewer grades it against this brief, the tree and
the design's own rules; the controller routes. Model tier: top (dispatch
model: Fable 5). Effort: xhigh.

**Why you and not the amendment-1/2 author:** those agents lived in the
previous session and cannot be resumed across sessions. You are fresh, so
read the design and its amendment log before writing anything - the log binds
at its current state.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Repo `/home/senol/Git/Muxsmith`, `master`, main worktree,
  HEAD `e592f55`, clean tree.
- Absolute paths, foreground runs only.
- **You edit exactly one file:** the design document
  `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`.
  Not the plan (a separate amendment dispatch follows yours), not the spec,
  not any source file, and never a house-knowledge YAML (the controller is
  the single writer there - surface ledger-worthy observations in your report).
- You commit your amendment. Commits on this repo are **standing-authorized
  by the owner**: `git -c commit.gpgsign=false`, stage the design file
  explicitly (never `git add -A`), exactly one trailer
  `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, no
  `Claude-Session` line. Do not push.

## The owner ruling you are implementing

Plan 9 Task 2 hoisted `run_batch` from `src-tauri/src/run.rs` into
`crates/muxsmith-core/src/executor/queue.rs`. D96 said the function "moves
as-is" and the plan's Task-2 Step 1 said "rustdoc moved with it", so the
implementer moved the doc comment verbatim - correctly; rewriting it would
have been latitude it must not take.

In the same commit the function gained its second caller (the CLI). Three
passages of that doc thereby became false about the function they document,
which the Task-2 reviewer raised as MEDIUM-1 (verdict at
`.superpowers/sdd/plan-9/task-2-verdict.md`, section 2).

**The owner ruled on 2026-07-28: this is a DESIGN change, not a defect fix.**
The correction therefore enters the design as amendment 3, and the code edit
rides Task 3 (which already owns `queue.rs`). The alternatives - treating it
as an ordinary truthfulness fix in a separate vehicle, or deferring it to a
1.x doc pass - were put to him and rejected. Do not re-open that choice.

## The defect, measured

The current doc is `crates/muxsmith-core/src/executor/queue.rs:327-347`
(the `pub fn run_batch` at `:348`). Verify it at the file; pasted here as it
stood at `e592f55`:

```rust
/// The run lifecycle's core body (D23), from the moment its [`JobSpec`]s
/// are known to the moment they are all terminal: runs `specs` to
/// completion via [`run_queue`] on its own scoped worker thread while this
/// function's own call stack drains the event channel, tee-ing every
/// [`JobEvent`] through `logger` (when persistence is available) and
/// `on_event` (the shell's window-emit in production, a plain collector in
/// tests). Synchronous by design so it is directly unit-testable with a
/// scripted [`Spawn`]; the `#[tauri::command]` wrapper is what moves the
/// whole call onto a detached `std::thread` so `start_run` itself returns
/// immediately.
///
/// Deliberately does NOT clear the active-run slot: that is
/// `finish_teardown`'s job, and it must run only after the joblog is
/// finalized and the terminal event emitted (D31: "slot empty" has to
/// mean "teardown fully complete", or a confirmed quit could exit the
/// process before `summary.json` is written).
///
/// Returns the outcomes (index-aligned to `specs`, exactly like
/// `run_queue`) and `logger` back, still open, so the caller can build the
/// terminal `run_document` and only then call [`RunLogger::finish`] on it
/// (`finish` needs the very document it is about to persist).
```

The three passages the reviewer named, with its measurements:

1. `` `on_event` (the shell's window-emit in production, a plain collector in tests) ``
   - the CLI's `on_event` (`crates/muxsmith-cli/src/commands/run.rs`, the
     closure passed to `run_batch`) renders milestone lines to stdout. Neither
     a window-emit nor a test collector.
2. `` the `#[tauri::command]` wrapper is what moves the whole call onto a detached `std::thread` so `start_run` itself returns immediately ``
   - false for the CLI, which calls `run_batch` synchronously on its own
     thread; no such wrapper exists on that path.
3. The `finish_teardown`/D31 paragraph names a symbol a core reader cannot
   reach: `fn finish_teardown` at `src-tauri/src/run.rs:651` is not `pub`, it
   sits behind a private `mod run;` (`src-tauri/src/lib.rs:23`), and
   `crates/muxsmith-core/Cargo.toml` has no dependency on the gui crate. Its
   link was already reduced to a plain code span in Task 2 (ruled correct,
   ledger `proc-48-docsurface-delink`); the paragraph itself still points a
   core reader at something they cannot look up.

Re-verify each of these three claims yourself before building on them - a
brief is not ground truth (`proc-57-briefs-not-ground-truth`), and refuting
one of my premises with evidence is a valid completion, not a failure.

## What the amendment must contain

You have design authority over the replacement prose within the ruling; the
shape below is the reviewer's suggestion, not a mandate, and you may improve
on it as long as the result is accurate for both callers.

1. **A rider on D96** stating that the moved rustdoc is restated for its new
   home, with the reason (the function acquired a second caller in the same
   change, so caller-specific prose became false), and making explicit that
   D96's move decision itself - verbatim body, today's signature, the
   not-absorbed pair, `TeardownGuard`/`fail_fast` caller-side - is unchanged.
2. **The exact replacement doc comment, in a fence, character for character**,
   so Task 3's implementer transcribes rather than composes. It must:
   - describe `on_event` as the caller's per-event work, true for both
     surfaces;
   - state the concurrency property of the FUNCTION (synchronous; the caller
     decides whether to run it on its own thread) rather than of one caller's
     wrapper;
   - keep the no-teardown fact while leaving the `finish_teardown`/D31
     rationale where `finish_teardown` lives (it is legitimate prose in
     src-tauri; it is not core's to explain);
   - add no new content beyond restating what is there, and change no factual
     claim that is currently true (the D23 reference, the tee order, the
     index-alignment and still-open-logger contract, the `RunLogger::finish`
     sentence).
   Whether the src-tauri caller gains a sentence pointing at the teardown
   rationale is yours to decide and to write into the fence if you want it -
   but if you do, say so explicitly, because that file is in Task 3's Files
   list and the plan amendment must carry it.
3. **A completeness pass over the rest of that doc comment and its
   neighbours.** The three passages are what one reviewer found; you check
   whether any other sentence in the moved doc, or in the docs of the items
   directly around it in `queue.rs`, carries a claim that stopped being true
   when the function acquired its second caller. Report what you checked and
   what you found, including "nothing else" if that is the answer - with the
   check you ran, since a negative result is only evidence if the check can
   fire.
4. **An `## Amendment log` entry** in the log's existing shape, recording the
   ruling, its date, and what changed.

## Constraints

- **Prose only.** No signature change, no behavior change, no new field, no
  test change. If your replacement text implies a code change beyond the doc
  comment, stop and return NEEDS_CONTEXT with a decision memo.
- **No design latitude reaches the implementer**, in either form: neither an
  explicit permission ("the implementer may reword...") nor an omission (a
  passage you mandate but do not write out, a set left unenumerated, an
  ellipsis). The fence is the contract; anything a Task-3 implementer would
  have to invent is a defect in your amendment.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, in the
  amendment and in every string it prescribes.
- **Counts recomputed from their enumerations**; every observed value pasted
  from the run that produced it (`design-empirical-claims-reproducible`).
- The v1 spec stays authoritative above the design; the ROADMAP Plan-9 anchor
  and the four house-knowledge files are ground truth alongside it.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/amendment-3-report.md`
and make your final message the same content (read as data, not chat):
status; what you changed in the design, quoted; the three premise checks with
their evidence; the completeness pass with its check and result; what the plan
amendment must carry as a consequence (file list, which task, which step);
anything surfaced for the controller; the commit hash and `git show --stat`.
