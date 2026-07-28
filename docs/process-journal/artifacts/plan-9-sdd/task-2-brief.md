# Task 2 implementer brief - Plan 9

**Role:** fresh implementer for Plan 9, Task 2 (`run_batch` hoists into
`muxsmith_core::executor::queue`; the src-tauri runs-root seam is deleted).
Model tier: mid (dispatch model parameter: Opus 5). Effort: xhigh.
An independent reviewer grades your work afterwards against the plan, the
design, the spec and the Tier-2 files; every empirical claim in your report
is re-run by the controller on disk.

## Preamble (binding, every dispatch)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). You work on `master` in the main worktree
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths in every command; **foreground runs only** - no
  background-run-plus-monitor pattern, no `&`.
- You are the only writer in this tree while you run (the plan's serial
  ruling). Nobody else commits here during your task.

## What to read first (in this order)

1. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` -
   the "Global Constraints", "Sequencing" and **"Task 2"** sections. The plan
   is the contract; its Task-2 steps 1-7 are what you execute, exactly.
2. `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md` -
   **D96** in full (including the caller-side checklist and the boundary
   paragraph), **D97** in full (the three-row table, the per-site debug-build
   loss, the rejected wrapper alternative), section 2's "stay discarded"
   paragraph, section 5's `run_batch` and runs-root bullets, and the
   `## Amendment log` at its current state (the log binds; the pointer is the
   contract, not an enumeration of it).
3. `docs/ROADMAP.md`, the "Plan 9" anchor: the owner's IN rulings for the
   `run_batch` hoist and for **deleting** (not hoisting) the runs-root seam.
4. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - the v1 spec is
   authoritative above the design on conflict. A conflict is reported, never
   improvised around.
5. The four house-knowledge files as ground truth alongside them
   (`docs/product-boundaries.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`). Cite entries
   by id; **re-verify any `:line` before relying on it**.

## Scope

Exactly the plan's Task 2: Files (EXHAUSTIVE) list, Steps 1-7, the
"Must not decide" list. Nothing else in the tree is touched - not a
neighbouring cleanup, not a rename, not a doc fix outside the moved items.

Carried verbatim from Task 1 (its Interfaces section): *"the inline CLI queue
block (`mpsc::channel` -> scope -> `expect(\"queue worker thread panicked\")`)
is retained byte-unchanged by Task 1 and replaced only by Task 2."* That block
is your Step 3.

Carried verbatim INTO Task 3, so leave it intact: *"the four
silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs`
remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded - the
recorded steelman of `exec-36`'s ruled no-facade position; no task 'improves'
them in passing."*

## Anchors: measured by the controller at HEAD `bd7a322`, 2026-07-28

A brief is not ground truth (`proc-57-briefs-not-ground-truth`): verify each
of these yourself and **locate by content, never by the line number** - Task 1
shifted `src-tauri/src/run.rs` (the plan's authoring-time numbers are stale by
design, and the plan says so). Pasted from the runs:

```
$ grep -n "fn run_batch" src-tauri/src/run.rs
758:fn run_batch(
$ grep -rn "resolve_runs_root" src-tauri/src crates
src-tauri/src/run.rs:301:        resolve_runs_root().and_then(|root| RunLogger::create(&root, &run_id, &specs).ok());
src-tauri/src/run.rs:505:    Ok(list_runs_in(resolve_runs_root().as_deref()))
src-tauri/src/run.rs:511:    get_job_log_in(resolve_runs_root().as_deref(), &run_id, index)
src-tauri/src/run.rs:803:fn resolve_runs_root() -> Option<PathBuf> {
$ grep -n "mpsc::channel\|queue worker thread panicked" crates/muxsmith-cli/src/commands/run.rs
205:    let (tx, rx) = mpsc::channel();
235:        handle.join().expect("queue worker thread panicked")
$ grep -n "fn run_batch_emits_started_output_finished_in_order\|fn run_batch_writes_job_log_files" src-tauri/src/run.rs
1204:    fn run_batch_emits_started_output_finished_in_order() {
1310:    fn run_batch_writes_job_log_files() {
$ grep -rn "MUXSMITH_RUNS_ROOT" crates src-tauri e2e src scripts
crates/muxsmith-cli/tests/run_cli.rs:172, crates/muxsmith-cli/src/commands/run.rs:281 (doc), :295 (read),
crates/muxsmith-cli/tests/run_live.rs:110, :245, :370, :468,
src-tauri/src/run.rs:801 (doc), :806 (read)          [9 lines, recomputed from this enumeration]
```

If any of these does not reproduce, that is a finding: report it, do not
work around it.

## Standing rules that bind this dispatch

- **No design latitude, in either form** (`proc-latitude-clause-boundary`):
  neither an explicit permission nor an omission (an unenumerated set in a
  normative position, a list left open, a name/string/file you would have to
  invent). If you hit a fork - a ripple, a hidden consumer, a colliding
  invariant, a design statement the code refutes - you **stop and return
  NEEDS_CONTEXT with a decision memo**: the options, their costs against the
  named invariants, and your recommendation. You do not resolve it at the
  keyboard, and you do not decide-then-report. Returning a refuted premise is
  a valid completion, not a failure.
- **Structural-conformance grant** (fills silence only, never overrides an
  explicit enumeration): following the touched file's existing structural
  patterns is in scope where the extension has zero outward effect - no
  API/symbol surface change, no data-format change across a serialization
  boundary, verification never weakened, nothing user-visible. Weakening,
  deleting, skipping or rewording existing assertions, mutating existing
  fixture values, new test files and new test infrastructure all stop and
  return.
- **No task edits any house-knowledge YAML.** The controller is the single
  writer. Something ledger-worthy (a pattern you established, a deliberate
  deviation, a recurring house question) is **surfaced in your report**.
- **Conform to the Tier-2 files** and surface, do not silently resolve, any
  new pattern you establish or deviation you take.
- **Counts are recomputed from their enumerations**
  (`proc-normative-count-recomputed`).
- **Absence checks are fire-verified and have a reachable green state**
  (`proc-verification-step-must-be-falsifiable`,
  `proc-check-green-state-reachable`): run the check on the pre-state where it
  must hit, paste that firing output, then reach the pass on the end state.
  Both halves are in your report or the check does not count.
- **Evidence lines carry pasted output** (`design-empirical-claims-reproducible`):
  every observed value in your report is pasted from the run that produced it,
  never recalled, never attributed to a command that was not the one run.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis - in
  code, comments and report.

## Verification bar before the commit (foreground, no subsets of these)

Per the plan's Task-2 Step 6, all green and pasted:

- Acceptance observable 2 as stated in the design's section 7 item 2, with its
  fire (pre-edit `grep -n "fn run_batch" src-tauri/src/run.rs` hits, per the
  measurement above) and its member-by-member green-state argument.
- Acceptance observable 3 as stated in the design's section 7 item 3, with its
  fires (`MUXSMITH_RUNS_ROOT` and `resolve_runs_root` in src-tauri pre-edit,
  per the measurement above).
- The two moved tests appear by name in `cargo test -p muxsmith-core` output.
- `cargo test -p muxsmith-gui` passes untouched.
- `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`.

The ten-part gate is the controller's pre-push run at the plan close; this
subset is your exit bar, not a gate substitute.

## Commit (SI-4, restated because you cannot see the grant)

Commits and pushes on this repo are **standing-authorized by the owner**; your
global never-commit default does not apply here. You commit, you do not push.

- Stage exactly the plan's pathspec, explicitly - **never `git add -A`**.
- `git -c commit.gpgsign=false commit ...` (agent commits are deliberately
  unsigned as policy).
- Message: the plan's Task-2 message.
- Exactly one trailer, no `Claude-Session` line:
  `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`
- One commit for the task.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-2-report.md`
(this path is git-ignored scratch; the controller salvages it) and end your
final message with the same status verdict. Structure:

1. **Status**: DONE / DONE_WITH_CONCERNS / NEEDS_CONTEXT.
2. **What changed, per file**, against the Files list - including anything the
   list did not predict, called out as such.
3. **Evidence**: every verification above with its pasted command and output,
   fires included.
4. **Divergences and judgment calls** you made under the structural-conformance
   grant, each named.
5. **Concerns** (if DONE_WITH_CONCERNS): numbered, each answerable by a
   reviewer with a yes/no verdict.
6. **Surfaced for the controller**: ledger-worthy observations, brief premises
   that did not reproduce, anything the next task should carry.
7. **Commit hash** and the `git show --stat` output for it.
