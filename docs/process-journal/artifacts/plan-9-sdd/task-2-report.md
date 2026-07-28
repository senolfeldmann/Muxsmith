# Task 2 implementer report - Plan 9

Commit `9b2843f`, master, main worktree. Date checked against the OS clock:
`date -I` -> `2026-07-28`.

## 1. Status

**DONE_WITH_CONCERNS** - the task is complete and every exit-bar check is
green, but the move forced three doc/comment repairs the Files list did not
predict (all zero-outward-effect, all named in section 4, each answerable
yes/no in section 5). No design fork was resolved at the keyboard: in each
case the plan's own instruction has exactly one non-failing execution, so
there was nothing to route.

## 2. What changed, per file

Exactly the plan's three files; nothing else in the tree touched.

### `crates/muxsmith-core/src/executor/queue.rs` (+104 / -1)

- Imports: `use std::sync::mpsc::Sender;` -> `use std::sync::mpsc::{self, Sender};`
  (`run_batch`'s own `mpsc::channel()`); new `use super::joblog::RunLogger;`
  (the signature's `Option<RunLogger>`). `joblog` already imports
  `super::queue::JobEvent`, so this is a module-level mutual reference, which
  Rust permits; the crate graph is unchanged.
- **Step 1 (D96):** `pub fn run_batch` added directly after `run_queue`
  (public API before the private helpers `worker_count`/`lock_outcomes`/
  `recover_panicked_worker`, the file's existing order). Signature character
  for character as D96's fence; body byte-identical to the src-tauri original.
  The boundary held: it does not absorb `run_document`, `finalize_joblog` or
  the CLI's joblog messages - those stayed in their callers. Byte-identity of
  the move is proved in section 3.5, not asserted.
- Rustdoc moved with it, one line changed - the two link brackets around
  `finish_teardown` dropped (section 4, divergence 1; proof in section 3.5).
- **Step 4:** the two named tests moved into `queue.rs`'s inline test module,
  at its end, under the two `// -- run_batch: ...` section comments they
  carried in src-tauri. Adaptations, all path/import-level:
  - `spec(dir.path(), "a.mkv")` -> the destination module's own
    `spec(0, dir.path().join("a.mkv"))` helper (section 4, divergence 3).
  - `ctl(specs.len())` / `opts()` (src-tauri-local helpers) -> inline
    `QueueControl::new(specs.len(), Arc::new(AtomicBool::new(false)))` and
    `QueueOpts { jobs: 1, fail_fast: false }`, which is exactly how every
    existing `queue.rs` test builds them.
  - `muxsmith_core::executor::job::JobState::Ok` -> `JobState::Ok` (already
    imported in `queue.rs`).
  - No assertion added, removed, weakened or reworded.

### `src-tauri/src/run.rs` (+8 / -122)

- **Step 5 (D97):** `resolve_runs_root` and its doc comment deleted; the
  THREE call sites converted exactly per D97's table - `plan_run` to
  `default_runs_root().and_then(...)` with an unchanged tail, `list_runs` and
  `get_job_log` to `default_runs_root().as_deref()`.
- **Step 1:** `fn run_batch` and its rustdoc deleted from this file.
- **Step 4:** the two moved tests deleted (pure deletions - see the hunk
  headers in section 3).
- **Step 2 (GUI caller):** unchanged in composition. `TeardownGuard` is still
  wrapped around the call on the runner thread, `QueueOpts { jobs, fail_fast:
  false }` is still caller-built, the call site's text is byte-identical - it
  now resolves to the core function through the import. Reservation, cancel
  and teardown tests untouched.
- Imports the compiler flagged as dead after the deletion:
  `use std::sync::mpsc;` removed, `JobOutcome` dropped from the
  `executor::job` import, `Spawn` dropped from the `executor::spawn` import,
  `JobEvent`/`run_queue` dropped from the `executor::queue` import and
  `run_batch` added there. Net import line:
  `use muxsmith_core::executor::queue::{QueueControl, QueueOpts, run_batch};`
- Module doc line 4: `[`run_queue`]` re-pointed to
  `[`muxsmith_core::executor::queue::run_queue`]` (section 4, divergence 2).
  **Not predicted by the Files list.**

### `crates/muxsmith-cli/src/commands/run.rs` (+18 / -37, net -19 lines)

- **Step 3:** the inline queue block (`mpsc::channel` through
  `handle.join().expect("queue worker thread panicked")`, located by content)
  replaced by one `run_batch` call. The `on_event` closure is the design's
  classed-(a) per-event work verbatim:

  ```rust
  let (outcomes, logger) = run_batch(&specs, &spawner, opts, &ctl, logger, |event| {
      // --json suppresses human progress lines; the final document is
      // built from the returned outcomes instead.
      if json {
          return;
      }
      for line in milestones.render(event, total, renderer) {
          println!("{line}");
      }
  });
  ```

  `MilestoneState::new(outputs)` moved out of the deleted scope to just above
  the call (its constructor is a pure two-field build, no side effect);
  `let mut logger` became `let logger` (`run_batch` takes and returns it);
  `Arc::clone(&ctl)` for the queue thread became `&ctl`. The logger tee is now
  the moved function's, which tees before invoking `on_event` - the CLI's own
  previous order - so persistence stays unconditional under `--json`.
- Imports: `use std::sync::mpsc;` removed; `run_queue` -> `run_batch` in the
  `executor::queue` import (`JobEvent` retained - `MilestoneState::render`
  still names it).
- Two adjacent comments rewritten because the deletion invalidated their
  referents (section 4, divergence 2). **Not predicted by the Files list.**
- The `MUXSMITH_RUNS_ROOT` gate in `create_logger` is byte-identical (proved
  by hash and by `diff` in section 3).

## 3. Evidence

All runs foreground, absolute working directory `/home/senol/Git/Muxsmith`.

### 3.0 Brief anchors, re-verified before touching anything

All five reproduced exactly as the brief pasted them (`proc-57`: verified, not
trusted). Pasted from the run:

```
=== A: fn run_batch in src-tauri ===
758:fn run_batch(
1204:    fn run_batch_emits_started_output_finished_in_order() {
1310:    fn run_batch_writes_job_log_files() {
=== B: resolve_runs_root ===
src-tauri/src/run.rs:301:        resolve_runs_root().and_then(|root| RunLogger::create(&root, &run_id, &specs).ok());
src-tauri/src/run.rs:505:    Ok(list_runs_in(resolve_runs_root().as_deref()))
src-tauri/src/run.rs:511:    get_job_log_in(resolve_runs_root().as_deref(), &run_id, index)
src-tauri/src/run.rs:803:fn resolve_runs_root() -> Option<PathBuf> {
=== C: CLI queue block ===
205:    let (tx, rx) = mpsc::channel();
235:        handle.join().expect("queue worker thread panicked")
=== D: moved test names ===
1204:    fn run_batch_emits_started_output_finished_in_order() {
1310:    fn run_batch_writes_job_log_files() {
=== E: MUXSMITH_RUNS_ROOT tree set ===
crates/muxsmith-cli/tests/run_live.rs:110:    .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))
crates/muxsmith-cli/tests/run_live.rs:245:    .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))
crates/muxsmith-cli/tests/run_live.rs:370:    .env("MUXSMITH_RUNS_ROOT", &runs_root)
crates/muxsmith-cli/tests/run_live.rs:468:        cmd.env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"));
crates/muxsmith-cli/tests/run_cli.rs:172:    .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))
crates/muxsmith-cli/src/commands/run.rs:281:/// [`default_runs_root`] (debug builds only: a `MUXSMITH_RUNS_ROOT` env
crates/muxsmith-cli/src/commands/run.rs:295:    let runs_root = std::env::var_os("MUXSMITH_RUNS_ROOT")
src-tauri/src/run.rs:801:/// `MUXSMITH_RUNS_ROOT` override (a test seam, deliberately absent from
src-tauri/src/run.rs:806:        std::env::var_os("MUXSMITH_RUNS_ROOT")
```

Set E is 9 lines, recomputed from that enumeration (5 CLI test sites + 2 CLI
source lines + 2 src-tauri source lines = 9), matching the brief's figure.

Everything was located by content, never by line number; Task 1 had shifted
this file (the plan's authoring numbers `:782`/`:1228`/`:1334`/`:827` are the
pre-Task-1 positions and do not reproduce, exactly as the plan and brief say).

### 3.1 Acceptance observable 2 (design section 7 item 2): `run_batch` hoist

**Fire (pre-edit, the state where the check must hit):**

```
### FIRE-2: grep -n "fn run_batch" src-tauri/src/run.rs
758:fn run_batch(
1204:    fn run_batch_emits_started_output_finished_in_order() {
1310:    fn run_batch_writes_job_log_files() {
exit=0
```

**Green (post-edit):**

```
### GREEN-2: grep -n "fn run_batch" src-tauri/src/run.rs
exit=1
```

**Member-by-member green-state argument.** The green state is not vacuous:
`run_batch` still occurs six times in src-tauri, and none of them is a
definition -

```
### GREEN-2b: all run_batch occurrences left in src-tauri
src-tauri/src/run.rs:44:use muxsmith_core::executor::queue::{QueueControl, QueueOpts, run_batch};
src-tauri/src/run.rs:447:        // Armed for the whole runner-thread body (fix): if `run_batch`'s
src-tauri/src/run.rs:462:        let (outcomes, logger) = run_batch(&specs, &spawner, opts, &ctl, logger, |event| {
src-tauri/src/run.rs:657:/// worker panic used to propagate straight out through [`run_batch`]'s
src-tauri/src/run.rs:1104:        run_batch(&specs, &fake, opts(), &control, None, |_| {});
src-tauri/src/run.rs:1158:    /// real trigger is `run_batch`'s `handle.join().expect(...)` when the
```

Recomputed from that enumeration: 6 survivors - one import, two call
expressions (`:462` production, `:1104` the retained
`commit_promotes_the_reservation_and_finish_teardown_clears_it` test), three
prose/doc mentions. The `fn run_batch` pattern matches none of them, so the
grep's pass is a real absence of the definition, not an absence of the name.

**Both surfaces call `muxsmith_core::executor::queue::run_batch`:**

```
src-tauri/src/run.rs:44:use muxsmith_core::executor::queue::{QueueControl, QueueOpts, run_batch};
crates/muxsmith-cli/src/commands/run.rs:13:use muxsmith_core::executor::queue::{JobEvent, QueueControl, QueueOpts, run_batch};
```

**The two moved tests run under `cargo test -p muxsmith-core`, by name:**

```
### cargo test -p muxsmith-core
running 121 tests
test executor::queue::tests::run_batch_writes_job_log_files ... ok
test executor::queue::tests::run_batch_emits_started_output_finished_in_order ... ok
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

### 3.2 Acceptance observable 3 (design section 7 item 3): runs-root deletion

**Fires (pre-edit):**

```
### FIRE-3a: grep -rn "MUXSMITH_RUNS_ROOT" src-tauri
src-tauri/src/run.rs:801:/// `MUXSMITH_RUNS_ROOT` override (a test seam, deliberately absent from
src-tauri/src/run.rs:806:        std::env::var_os("MUXSMITH_RUNS_ROOT")
exit=0

### FIRE-3b: grep -rn "resolve_runs_root" src-tauri
src-tauri/src/run.rs:301:        resolve_runs_root().and_then(|root| RunLogger::create(&root, &run_id, &specs).ok());
src-tauri/src/run.rs:505:    Ok(list_runs_in(resolve_runs_root().as_deref()))
src-tauri/src/run.rs:511:    get_job_log_in(resolve_runs_root().as_deref(), &run_id, index)
src-tauri/src/run.rs:803:fn resolve_runs_root() -> Option<PathBuf> {
exit=1
```

(3b's fire is 4 hits, recomputed from the enumeration: one definition + three
call sites - the exact set D97's table converts.)

**Greens (post-edit):**

```
### GREEN-3a: grep -rn "MUXSMITH_RUNS_ROOT" src-tauri
exit=1

### GREEN-3b: grep -rn "resolve_runs_root" src-tauri
exit=1
```

**Green-state argument.** The deletion removed all four `resolve_runs_root`
sites and both `MUXSMITH_RUNS_ROOT` lines; anchor set E above shows no other
src-tauri occurrence of either existed to survive. A whole-tree sweep confirms
nothing outside `docs/` still names the deleted function, with a control
proving pattern and pathspec produce output:

```
### full-tree resolve_runs_root (excluding .git, target, node_modules, .worktrees)
docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md:57, :185, :197, :200
docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md:79, :537, :543, :544, :545, :576, :1459
docs/process-journal/artifacts/... (historical review diffs and idiomacy findings)
### CONTROL same invocation, known-present pattern default_runs_root
95
```

(hit list abbreviated by path; every hit is prose in the plan, the design or a
historical journal artifact - no code, no test, no script. The control returns
95 lines, so pathspec and invocation demonstrably produce output.)

**The CLI gate is byte-identical.** The gate region moved by 18 lines
(deletions above it), so it was located by content (`fn create_logger`) and
compared by hash and by `diff`:

```
### pre-state gate region (HEAD, lines 280-309)
ce5284107659477b739a1184fae20ea82651e4f0c6f4a658892cb7814af9759b  -
### post-state gate region (lines 262-291)
ce5284107659477b739a1184fae20ea82651e4f0c6f4a658892cb7814af9759b  -
### and the byte-diff of the gate region
IDENTICAL
```

Fire for that `diff`-based check (the same invocation on a region that DID
change, so the "IDENTICAL" result is not a broken comparison):

```
### CONTROL: the same diff invocation on a region that DID change (CLI queue block)
1d0
<     // D26: persisted job logs. Created before the queue runs so `on_event`
4c3
<     let mut logger = create_logger(renderer, &specs);
---
>     let logger = create_logger(renderer, &specs);
6,33c5,17
<     let (tx, rx) = mpsc::channel();
...
(diff exit=0)
```

**`cargo test -p muxsmith-gui` passes untouched:**

```
### cargo test -p muxsmith-gui
test result: ok. 80 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

"Untouched" is proved structurally rather than by a count: the two hunks
inside `mod tests` are pure deletions (`-35,+0` and `-18,+0`), i.e. the two
moved tests and nothing else - no surviving test was adapted for the runs-root
deletion:

```
### every change in src-tauri/src/run.rs, hunk headers
@@ -4 +4,2 @@
@@ -32 +32,0 @@ use std::sync::atomic::{AtomicBool, Ordering};
@@ -40 +40 @@ use muxsmith_core::capability::runtime::Mkvmerge;
@@ -44,2 +44,2 @@ use muxsmith_core::executor::joblog::{
@@ -301 +301 @@ fn plan_run(
@@ -505 +505 @@ pub fn list_runs() -> Result<Vec<RunMeta>, IpcError> {
@@ -511 +511 @@ pub fn get_job_log(run_id: String, index: usize) -> Result<serde_json::Value, Ip
@@ -737,44 +736,0 @@ fn emit_run_finished(...)
@@ -799,17 +754,0 @@ fn finalize_joblog(...)
@@ -1201,35 +1139,0 @@ mod tests {
@@ -1307,18 +1210,0 @@ mod tests {
```

The design's stated reason for that - "the src-tauri tests ... call
`list_runs_in`/`get_job_log_in` with an explicit `Option<&Path>`" - re-verified
independently:

```
1454:        let metas = list_runs_in(Some(root.path()));
1463:        assert!(list_runs_in(None).is_empty());
1466:        assert!(list_runs_in(Some(&missing)).is_empty());
1478:        let v = get_job_log_in(Some(root.path()), "20260710-000000Z", 0).unwrap();
1484:        let err = get_job_log_in(Some(Path::new("/tmp")), "../etc", 0).unwrap_err();
1490:        let err = get_job_log_in(None, "20260710-000000Z", 0).unwrap_err();
1497:        let err = get_job_log_in(Some(root.path()), "20260710-000000Z", 0).unwrap_err();
```

### 3.3 The gate subset (Task-2 Step 6), all foreground

```
### cargo fmt --all --check
(exit=0)

### cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
```

`cargo fmt --all --check`'s pass is an absence of output, so the mechanism was
fired on a known-bad input first:

```
### FIRE for the fmt absence-check: rustfmt --check on a deliberately misformatted file
Diff in /tmp/.../scratchpad/fire.rs:1:
-fn  main( ) {
-let x=1;
+fn main() {
+    let x = 1;
 }
(rustfmt --check exit=1)
```

`cargo test --workspace`:

```
### any FAILED / error / panicked lines
(none)
### aggregate over every "test result:" line
total passed: 494  total failed: 0
```

**Typography** (ASCII hyphens, straight quotes, no Unicode ellipsis), swept
over the three committed source files and over this report. The first sweep I
wrote returned empty against a control file that DID contain every glyph, so
the empty result meant nothing (`grep` here is `ugrep 7.5.0`, and the
`$'a\|b'` alternation silently matched nothing). Re-armed with `-P` until the
control fired, then run:

```
$ grep -nP '[\x{2014}\x{2013}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}]' <scratch file carrying all eight glyphs>
1:<the file's single line, echoed back by the match>
(exit=0)
$ grep -nP '[\x{2014}\x{2013}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2012}\x{2015}\x{2212}]' \
    crates/muxsmith-core/src/executor/queue.rs src-tauri/src/run.rs crates/muxsmith-cli/src/commands/run.rs
(exit=1)
$ ... same pattern, this report
(exit=1)
```

(The control's matched line is described rather than pasted: reproducing it
here would put the very glyphs into this report that the sweep exists to keep
out. Its content was one line carrying em-dash, en-dash, both smart-quote
pairs, the Unicode ellipsis and a non-breaking space.)

Byte-behavior preservation for the rewritten CLI call path - the subprocess
suites and their insta snapshots pass unchanged:

```
     Running tests/run_cli.rs (target/debug/deps/run_cli-3e587b0db07411e9)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
     Running tests/run_live.rs (target/debug/deps/run_live-9bdc34d53bbb876f)
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s
```

### 3.4 Rustdoc (not in the exit bar; run because the move ripples into it)

Baseline on HEAD `bd7a322` before any edit: green.

The verbatim doc move then broke the gate. Pasted from the run - this is the
fire for divergence 1:

```
error: unresolved link to `finish_teardown`
   --> crates/muxsmith-core/src/executor/queue.rs:339:7
    |
339 | /// [`finish_teardown`]'s job, and it must run only after the joblog is
    |       ^^^^^^^^^^^^^^^ no item named `finish_teardown` in scope
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: could not document `muxsmith-core`
```

After the repair:

```
### rustdoc gate
 Documenting muxsmith-gui v0.1.0 (/home/senol/Git/Muxsmith/src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.50s
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files
```

The second break was invisible to the default gate because `mod run;` is
private in `src-tauri/src/lib.rs`, and rustdoc does not link-check
undocumented private modules:

```
$ grep -n "^mod " src-tauri/src/lib.rs
22:mod error;
23:mod run;
24:mod settings;
567:mod tests {
```

Fired with `--document-private-items`:

```
error: unresolved link to `run_queue`
 --> src-tauri/src/run.rs:4:43
  |
4 | //! (re-plan, build [`JobSpec`]s, drive [`run_queue`]) so planning stays
```

After the repair, workspace-wide with private items included:

```
$ RUSTDOCFLAGS="" cargo doc --workspace --no-deps --document-private-items | grep -c "unresolved link"
0
```

That grep answers a narrower question than "would the flag be free", so it was
re-run with `-D warnings` and no pattern filter, which is what the gate would
actually do (see section 6, item 1):

```
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items 2>&1 | grep -E "^(error|warning):" | sort | uniq -c
      1 error: could not document `muxsmith-gui`
      2 error: `run` is both a function and a module
$ RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-gui --no-deps --document-private-items 2>&1 | grep -E "^error|-->"
error: `run` is both a function and a module
  --> src-tauri/src/lib.rs:54:21
error: `run` is both a function and a module
  --> src-tauri/src/lib.rs:87:15
error: could not document `muxsmith-gui`
```

### 3.5 The move is verbatim (D96 "moves as-is"), byte-compared

Old region taken from `git show HEAD~1:src-tauri/src/run.rs`, new region from
the committed `queue.rs`; both extracted by the line spans the greps report.

```
### diff: old function (src-tauri HEAD~1 758-779) vs new (core 348-369)
1c1
< fn run_batch(
---
> pub fn run_batch(
(exit=1)
```

Only the visibility keyword differs - signature and all 22 body lines are
byte-identical. Control that this comparison can fail (same old region against
a one-line-shifted new window):

```
### CONTROL: same diff against a shifted window
1d0
< fn run_batch(
22a22
>
(exit=0)
```

The rustdoc, same method:

```
### diff old rustdoc (HEAD~1 737-757) vs moved rustdoc (core 327-347)
13c13
< /// [`finish_teardown`]'s job, and it must run only after the joblog is
---
> /// `finish_teardown`'s job, and it must run only after the joblog is
(exit=1)
```

One line of 21, and only its two link brackets - the divergence 1 repair, and
nothing else.

## 4. Divergences and judgment calls

Three, all under the structural-conformance grant (no API/symbol surface
change, no data-format change, no verification weakened, nothing user-visible),
all surfaced rather than silently absorbed. None resolved a design fork: in
each case the plan's own instruction has exactly one non-failing execution.

1. **`[`finish_teardown`]` de-linked to `` `finish_teardown` `` in the moved
   rustdoc.** Plan Step 1 says "rustdoc moved with it"; executed verbatim, the
   doc names a src-tauri-private function, which no path can address from
   core, so the `-D warnings` rustdoc gate fails (fired above). The prose is
   otherwise byte-identical - only the two link brackets are gone. No
   alternative preserves both the words and the gate.
2. **Two dangling references the deletion created, repaired in place.**
   (a) `src-tauri/src/run.rs:4`'s `[`run_queue`]` resolved through the import
   this task removes; re-pointed to
   `[`muxsmith_core::executor::queue::run_queue`]`, the dominant house pattern
   for cross-crate doc links: 8 occurrences before this change, recomputed
   from their enumeration (`src-tauri/src/error.rs` 4, `src-tauri/src/run.rs`
   1, `src-tauri/src/settings.rs` 1, CLI `commands/run.rs` 1, CLI
   `commands/dry_run.rs` 1) - including
   `[`muxsmith_core::planner::plan_batch`]` at `:360` of this same file. The
   display-text-preserving alternative
   `` [`x`](path) `` exists once repo-wide (`crates/muxsmith-cli/src/i18n.rs:126`),
   so it is the minority form. Prose unchanged. (b) The CLI comment above `create_logger` said "see the
   drain loop below"; there is no drain loop below any more, so it now reads
   "so `run_batch` can tee every event into it as it arrives", and the
   neighbouring persistence comment states the tee ORDER (`run_batch` tees
   before invoking the closure) rather than restating the tee itself. Both are
   comment text only. **Neither file's entry in the Files list predicted a doc
   edit** - flagged as such per the report structure.
3. **The moved tests use the destination module's `spec(index, output)`
   helper**, not src-tauri's `spec(dir, name)`. Forced by
   `testing-support-helpers` (Tier 2): "duplicating a helper within a crate is
   a defect", so adding a second `spec` to `muxsmith-core` was not available.
   Consequence: the fixture argv changes from `["x"]` to `["0"]`. Verified
   unobserved - `FakeSpawner::spawn` only records argv into `self.spawned`, and
   neither moved test calls `FakeSpawner::spawned()`; both assert on events,
   outcomes and the joblog file. `opts()`/`ctl()` were inlined the way every
   existing `queue.rs` test builds them. No assertion touched.

Explicitly NOT done, per the carried-verbatim constraint: the four
silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs`
remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) are untouched and stay
discarded; this task did not enter those files at all.

## 5. Concerns (each answerable yes/no by the reviewer)

1. Is de-linking `[`finish_teardown`]` to a plain code span (divergence 1) the
   right reading of "rustdoc moved with it", given the alternative is a red
   `-D warnings` rustdoc gate at the plan close?
2. Should the moved rustdoc keep its two shell-specific sentences at all now
   that it lives in core - "`on_event` (the shell's window-emit in
   production...)", "the `#[tauri::command]` wrapper is what moves the whole
   call onto a detached `std::thread` so `start_run` itself returns
   immediately", and the whole `finish_teardown`/D31 paragraph? They are
   accurate about the *caller*, and the plan said move the rustdoc, so I moved
   them; a core reader has no `start_run`, `finish_teardown` or D31 to look up.
   Rewriting them is design latitude I did not take.
3. Is re-pointing `src-tauri/src/run.rs:4`'s doc link (divergence 2a)
   in-scope, or should the dangling link have been returned as a finding
   instead of repaired?
4. Is the CLI comment rewrite (divergence 2b) in-scope, or should the stale
   "see the drain loop below" have been left as-is?
5. Is `spec(0, ...)` with its `argv: ["0"]` (divergence 3) acceptable as a
   "paths/imports" adaptation, given `testing-support-helpers` forbids the
   alternative?

## 6. Surfaced for the controller

**Ledger-worthy.**

1. **The private-module rustdoc blind spot recurred one task later, in the
   same file - and the ROADMAP's cost figure for its fix is now stale by one.**
   Nothing new about the blind spot itself: `docs/ROADMAP.md:1017` already
   carries the section "Gate: rustdoc does not link-check private modules",
   measured at the Plan 9 Task 1 review, and it names the identical failure
   shape ("The task's own LOW-2 was exactly that: an import removal left
   `[`plan_batch`]` unresolvable at `src-tauri/src/run.rs:359` and nothing
   could see it", `:1023-1025`). Task 2's `run_queue` break is that class's
   next instance: same file, same cause (an import removal), same invisibility
   to the gate, one task later. That recurrence is the datum - the deferral to
   the Plan-9 close (`:1043-1048`, "Deliberately NOT done mid-plan") is now
   carrying a defect class that has fired in two consecutive tasks, and Task 3
   edits both of the files it fired in (`queue.rs`, src-tauri `run.rs`) under a
   compiler-driven sweep, which is the same shape of change that produced both
   instances. Whether that changes the deferral is the controller's call; the
   ruling's reason (do not fork the gate contract mid-plan) is untouched by
   this datum.
   **Correction to that section, measured:** `:1031-1041` says "the flag lands
   with three one-line fixes" (one unresolved link plus the two ambiguous
   `[`run`]` links at `src-tauri/src/lib.rs:54`/`:87`). The unresolved one was
   Task 1's LOW-2, and commit `fed55be` fixed it, so the figure did not get
   re-recomputed after its own fix round. Today the cost is **two**, both the
   ambiguity errors, both still exactly where that line says
   (`src-tauri/src/lib.rs:54:21` and `:87:15`; pasted in section 3.4). Evidence
   that the third is gone: with `--document-private-items` the only unresolved
   link in the workspace was the one this task introduced, and the count is `0`
   after the repair. I did not verify at `bd7a322` directly - the inference is
   from my own pre-repair run, where `run_queue` was the sole hit.
   Note for whoever lands the flag: `grep -c "unresolved link"` is NOT the
   check to measure its cost with; ambiguity is a different diagnostic under
   the same lint, and it is what still fails. Not my call, not my file.
2. **A verbatim cross-crate code move carries its doc links with it, and those
   are a hidden ripple surface.** D96/D97's "moves as-is" was exact about the
   signature and the body; the rustdoc that travels with a function can point
   at symbols that exist only in the origin crate, and the compiler is silent
   about it (only rustdoc complains). Worth a convention line for the two
   remaining hoist-shaped items: a move's file list should name the doc-link
   sweep, the same way it names the import sweep.

**Brief premises that did not reproduce:** none. All five anchor
measurements reproduced exactly as pasted (section 3.0), including the
9-line `MUXSMITH_RUNS_ROOT` set and the post-Task-1 line numbers.

**For Task 3 (carried verbatim, as instructed):** "the four
silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs`
remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded - the
recorded steelman of `exec-36`'s ruled no-facade position; no task 'improves'
them in passing."

**Also for Task 3, factual** (pasted from
`grep -n "fn recover_panicked_worker\|eprintln!\|fn worker_panic_is_reported_as_failed_not_cancelled" crates/muxsmith-core/src/executor/queue.rs`
after this commit):

```
424:fn recover_panicked_worker(
441:    eprintln!("muxsmith: worker thread panicked while running job {index}: {message}");
783:    fn worker_panic_is_reported_as_failed_not_cancelled() {
```

All three shifted by the `run_batch` insertion above them (the plan's
authoring anchors `queue.rs:396` for the `eprintln!` and `:738` for the test
no longer hold). Locate by content, as always. Task 3's `JobOutcome` compiler sweep now also has to cover
the two tests this task moved into `queue.rs` - which is exactly the 2 -> 3
edge the plan's sequencing predicted.

## 7. Commit

`9b2843f287165b5a2ad4585d2b6db135b0b6bcda`, unsigned (`git log -1
--format='%G?'` -> `N`), one trailer, no `Claude-Session` line, staged by
explicit pathspec (never `git add -A`). Not pushed. Working tree clean after
the commit.

```
commit 9b2843f287165b5a2ad4585d2b6db135b0b6bcda
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 15:06:54 2026 +0200

    executor: hoist run_batch into core, delete the src-tauri runs-root seam (D96, D97)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/src/commands/run.rs    |  55 ++++--------
 crates/muxsmith-core/src/executor/queue.rs | 105 ++++++++++++++++++++++-
 src-tauri/src/run.rs                       | 130 ++---------------------------
 3 files changed, 130 insertions(+), 160 deletions(-)
```
