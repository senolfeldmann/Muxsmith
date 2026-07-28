# Task 2 review verdict - Plan 9

Independent reviewer, commit `9b2843f`, master, main worktree, tree clean at
review start and unchanged by this review (no product file edited, nothing
committed). Reviewer scratch root, used for every instrument below:
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t2rev-independent/`.
Nothing here re-runs an instrument the implementer wrote; every empirical
claim of the report that carries a conclusion was reproduced with my own
extraction, my own scratch copy of the workspace, or my own grep, each with
its own fire.

## 1. Verdict

**APPROVED_WITH_MINORS.**

The contract is met. The move is verbatim (my own structural extraction:
one line differs, `fn` -> `pub fn`), the not-absorbed pair stayed with both
callers, `TeardownGuard` and `fail_fast` are untouched caller-side, exactly
D97's function plus three call sites are gone, the CLI gate is byte-identical,
both named tests moved with their assertion sets intact, and the CLI's event
path preserves tee order, unconditional persistence and milestone rendering.
Gate subset re-run green by me; all three report aggregates recompute exactly
(494 / 121 / 80).

No finding requires re-doing Task 2. MEDIUM-1 is a residue the implementer
correctly surfaced rather than absorbed and which needs a controller ruling
before anyone may fix it; MEDIUM-2 is a process route, not a code defect
(I measured that nothing was weakened); the three LOWs are report arithmetic
and one unsurfaced divergence.

## 2. Findings

### MEDIUM-1: the moved rustdoc asserts things that are false about the function it now documents

`crates/muxsmith-core/src/executor/queue.rs:332-342` (public rustdoc:
`pub fn run_batch` at `:348`, in `pub mod queue`
(`crates/muxsmith-core/src/executor/mod.rs:7`) of `pub mod executor`
(`crates/muxsmith-core/src/lib.rs:11`)).

Three passages describe one caller and became false for the other in the same
commit that gave the function its second caller:

- `:332-333` `` /// `on_event` (the shell's window-emit in production, a plain collector in ``
  `/// tests).` The CLI's `on_event`
  (`crates/muxsmith-cli/src/commands/run.rs:208-217`) renders milestone lines
  to stdout. It is neither a window-emit nor a test collector.
- `:334-336` `` /// scripted [`Spawn`]; the `#[tauri::command]` wrapper is what moves the ``
  `` /// whole call onto a detached `std::thread` so `start_run` itself returns ``
  `/// immediately.` False for the CLI, which calls `run_batch` synchronously
  on its own thread; no wrapper exists on that path.
- `:338-342`, the `finish_teardown`/D31 paragraph, names a symbol a core
  reader cannot reach. Measured: `fn finish_teardown` at
  `src-tauri/src/run.rs:651` carries no `pub`; it lives behind `mod run;`
  (`src-tauri/src/lib.rs:23`, private); and `crates/muxsmith-core/Cargo.toml`
  has no dependency on the gui crate.

**Not the implementer's defect.** Plan Step 1 says "rustdoc moved with it" and
D96 says "moves as-is"; design section 5 bars re-opening D96. Moving it
verbatim was the only compliant execution, and rewriting it would have been
exactly the latitude the plan forbids. The implementer surfaced it as concern
2 rather than absorbing it, which is the correct route.

**What must change.** A controller-authored (or owner-ruled) replacement for
those three passages, prose only, because it edits D96's "as-is" instruction.
Shape that restores accuracy without adding content:

- `on_event` described as the caller's per-event work, naming both surfaces
  (the GUI's window-emit, the CLI's milestone rendering).
- The concurrency sentence restated as a property of the function ("synchronous
  by design; the caller decides whether to run it on its own thread - the GUI's
  `#[tauri::command]` wrapper does, the CLI does not").
- The slot paragraph restated as "performs no caller-side teardown", with the
  `finish_teardown`/D31 rationale left in `src-tauri/src/run.rs` where
  `finish_teardown` lives.

**Timing.** Task 3 edits `queue.rs`. If the ruling lands before Task 3
dispatches, the fix rides its commit at no cost. It must not be picked up
silently by Task 3 without the ruling.

### MEDIUM-2: the fixture mutation in the moved tests was resolved at the keyboard instead of returned (no code change required)

`crates/muxsmith-core/src/executor/queue.rs:1367` and `:1404` call
`spec(0, ...)` (helper at `:510-515`, `argv: vec![index.to_string()]`), so the
fixture argv is `["0"]`. The src-tauri originals built `argv: vec!["x"]`
(`HEAD~1:src-tauri/src/run.rs:938-942`).

`latitude-carveout-zero-content-structural-forks`
(`docs/process-conventions.yaml:353`) names "mutating existing fixture values"
in its explicit stop list, and plan Step 4 is narrower still ("adapted only in
paths/imports"). Under either, this was a stop-and-return, not a grant case:
the grant is "controller-authored and pre-decided, never implementer-judged at
the keyboard", and `proc-latitude-clause-boundary` says a fork "is routed
BEFORE it is resolved". The implementer decided, then reported.

The stated justification is also incomplete. It rules out adding a second
`spec` helper - correctly, `testing-support-helpers` bars it and Rust would
reject the name collision in the same module - and jumps from there to the
destination helper. The option that preserves the fixture exactly, an inline
`JobSpec { argv: vec!["x".to_string()], output: ... }` literal, adds no helper
at all and therefore never touches `testing-support-helpers`. It is never
weighed.

**Outcome verified harmless; I ran the premise rather than weighing it.** In my
own scratch copy of the workspace I replaced the fixture in both moved tests
with `argv: vec!["ZZZ-reviewer".to_string()]`: both stay green. Control on the
same instrument, mutating a value that IS observed (scripted exit `Some(0)` ->
`Some(7)`): `run_batch_emits_started_output_finished_in_order` goes red. Full
output in the appendix. Assertion sets are identical pre/post (4/4 and 2/2,
diffed structurally).

**Recommendation: keep `spec(0, ...)`.** It is the destination module's
unbroken pattern - 35 `spec(` call sites in `queue.rs`, and exactly one
`JobSpec {` literal in the whole file, inside the helper itself - so the
inline-literal alternative would be the lone deviation `conventions.md` calls a
defect even when individually idiomatic. No code change. The finding is the
route, and it is worth an occurrence on
`latitude-carveout-zero-content-structural-forks`.

### LOW-1: the report undercounts its own comment rewrites (two claimed, three made)

Report section 4, divergence 2(b): "Two adjacent comments rewritten". Measured
(`git show 9b2843f -- crates/muxsmith-cli/src/commands/run.rs | grep -E "^[-+]\s*//"`):
three comment blocks were rewritten and one deleted.

The unsurfaced third is the in-closure `--json` comment
(`crates/muxsmith-cli/src/commands/run.rs:209-210`): "Task 9 builds the final
document from the returned outcomes instead" became "the final document is
built from the returned outcomes instead". Unlike the other two, this rewrite
is not forced by the deletion - the "Task 9" reference is stale, not falsified
by this change - so it is an unforced comment edit inside a listed file that
the report's divergence list does not name.

Zero outward effect; nothing to change in the code. The controller should carry
the third rewrite when routing divergence 2(b).

### LOW-2: the minority-doc-link-form count is off by one as stated

Report section 4, divergence 2(a): the display-text-preserving form
`` [`x`](path) `` "exists once repo-wide (`crates/muxsmith-cli/src/i18n.rs:126`)".

Measured on the pre-state, it exists twice in Rust sources:
`crates/muxsmith-cli/src/i18n.rs:126`
(`` [`Diagnostic`](muxsmith_core::report::Diagnostic) ``) and
`crates/muxsmith-core/src/capability/runtime.rs:393`
(`` [`normalize`](Self::normalize) ``). The claim is true only under a
cross-crate restriction the sentence does not state.

The conclusion is unaffected (minority form either way), and the load-bearing
half of that paragraph reproduced exactly: 8 fully-qualified cross-crate links
pre-state, in precisely the five files the report enumerates.

### LOW-3: "22 body lines" double-counts the signature

Report section 3.5: "signature and all 22 body lines are byte-identical".
Measured with my own structural extraction: the moved region is 22 lines in
total (8 signature lines, `:348-355`, plus 14 body lines), of which exactly one
differs. There are not 22 body lines in addition to the signature.

## 3. The five adjudication verdicts

### Q1 - the de-linked rustdoc: CORRECT READING, IN SCOPE, NO DEFECT

Of the three options the question names, two are unavailable, so "rustdoc moved
with it" had exactly one non-failing execution and there was no fork with two
branches to route.

- *Keep the link and return the gate failure.* Not a live option in the sense
  the question implies. Unlike the src-tauri instances, this break is visible to
  the plain gate: `queue` is a public module of a public crate, so
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` documents and
  link-checks it without `--document-private-items`. Returning it would have
  meant leaving the tree red on gate part 4 with no repair available.
- *Point the link at something that resolves.* Impossible, measured above under
  MEDIUM-1: private fn, private module, crate not in core's dependency graph.
  No path expresses it.
- *De-link to a code span.* Chosen. Preserves every word of the prose; the only
  thing lost is a hyperlink that could never have resolved from core.

### Q2 - shell-specific prose surviving into core: RIGHT EXECUTION, DEFECTIVE RESULT (= MEDIUM-1)

Keeping them was right: the plan said move the rustdoc, section 5 bars
re-opening D96, and a rewrite is precisely the design latitude
`proc-latitude-clause-boundary` forbids resolving at the keyboard. Surfacing it
as a concern is the correct route for the residue.

But the concern's own framing understates the problem. "They are accurate about
the *caller*" is true of the `finish_teardown`/D31 paragraph and false of the
other two: once the CLI became a caller in this same commit, "`on_event` (the
shell's window-emit ...)" and "the `#[tauri::command]` wrapper is what moves the
whole call onto a detached `std::thread`" stopped being caller-flavoured and
became simply wrong about the function they document. That crosses from style
into correctness on a public API's rustdoc. Routed as MEDIUM-1: controller
ruling, then a prose-only fix.

### Q3 - the repaired doc link at `src-tauri/src/run.rs:4`: IN SCOPE, CORRECT

`latitude-carveout-zero-content-structural-forks` applies and all four
zero-outward-effect conditions hold: a doc link introduces no item and changes
no symbol surface; nothing crosses a serialization boundary; no assertion is
touched; nothing user-visible (rustdoc is developer-facing, no string, no
catalog key).

The grant "fills SILENCE only", and Task 2's Files list is silent *within* file.
That is measurable rather than a reading: Task 1's Files list carries 5 "only"
qualifiers scoping the edit inside each listed file ("call-site mapping 4 only",
"the `config_only_document` doc comment at `:66-73` only", ...); Task 2's three
entries carry 0. The exhaustive marker binds the file set, which the repair
never leaves.

The repair also matches the file's dominant pattern (8 fully-qualified
cross-crate links pre-state, one of them at `:360` of this very file, itself
written by Task 1's fix round). The alternative - leaving a link the task's own
import removal broke - would have shipped a knowingly-dangling link that the
scheduled `--document-private-items` change turns red at the plan close.

**Over-restriction: yes.** See HARVEST (c).

### Q4 - the rewritten CLI comments: IN SCOPE, CORRECT, WITH DIRECT PRECEDENT

Same grant, and this class is already ruled. `docs/process-conventions.yaml:363`,
occurrence dated 2026-07-21 on that same entry: "first correctly-permitted
comment-edit application - ... the en edit repaired a sentence the change itself
falsified (sweep duty)". "see the drain loop below" was falsified by the
deletion Step 3 mandates; there is no drain loop below any more.

Both rewritten texts are accurate. I checked the claim the new comment makes:
the tee does happen inside `run_batch` before `on_event`
(`crates/muxsmith-core/src/executor/queue.rs:360-363`: `l.on_event(&event)` at
`:361`, `on_event(&event)` at `:363`).

Caveat: the third, unsurfaced rewrite is LOW-1.

**Over-restriction: yes.** See HARVEST (c).

### Q5 - the test-helper adaptation: LEGITIMATE AS AN OUTCOME, OUT OF BOUNDS AS A ROUTE

Ruled in full under MEDIUM-2. Short form: the argv change is a fixture-value
mutation, which the grant names in its explicit stop list and which plan Step
4's "adapted only in paths/imports" excludes independently, so it was a
stop-and-return. It was instead decided and then disclosed, and the disclosure's
reasoning rules out only the option `testing-support-helpers` bars, never the
fixture-preserving inline literal.

Nothing was weakened in fact - I mutated argv myself and both tests stayed
green, with a control mutation proving the tests can go red - and
`spec(0, ...)` is the destination module's unbroken pattern, so my
recommendation is to keep the code as committed and record the route.

## 4. Evidence appendix

All runs foreground, absolute paths, in `/home/senol/Git/Muxsmith` unless the
scratch root is named. Instruments written by me at
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t2rev-independent/`:
`extract_move.py`, `cli_gate.py`, `test_moves.py`, `mutate.py`, `repo-copy/`
(scratch workspace copy), `target/` (its own cargo target dir).

### 4.1 The move is verbatim - my own structural extraction

`extract_move.py` reads `HEAD~1:src-tauri/src/run.rs` and
`HEAD:crates/muxsmith-core/src/executor/queue.rs` from the git objects and
locates the function by walking back over the contiguous `///` block and
forward to the closing brace at column 0. It never uses a line span the
implementer reported.

```
$ python3 .../t2rev-independent/extract_move.py
old doc lines=21 fn lines=22
new doc lines=21 fn lines=22
=== BODY DIFF (old src-tauri vs new core) ===
--- old/src-tauri
+++ new/core
@@ -1 +1 @@
-fn run_batch(
+pub fn run_batch(
=== RUSTDOC DIFF (old src-tauri vs new core) ===
--- old/src-tauri
+++ new/core
@@ -13 +13 @@
-/// [`finish_teardown`]'s job, and it must run only after the joblog is
+/// `finish_teardown`'s job, and it must run only after the joblog is
=== CONTROL: same old body vs itself with one char changed ===
--- old
+++ mutated
@@ -6 +6 @@
-    mut logger: Option<RunLogger>,
+    mut logger: Option<RunLogger>, // mutation
```

The control is the fire: the differ demonstrably reports a one-line change, so
the single-line body diff is a real result, not a broken comparison.

### 4.2 The fixture premise, run rather than weighed (MEDIUM-2, brief dimension 6)

Scratch copy of the workspace at `.../t2rev-independent/repo-copy` with its own
`CARGO_TARGET_DIR`. `mutate.py` scopes its edits to the two moved test
functions only.

```
$ python3 .../mutate.py .../repo-copy/crates/muxsmith-core/src/executor/queue.rs argv
mode=argv lines mutated=2
1367:        let specs = vec![JobSpec { argv: vec!["ZZZ-reviewer".to_string()], output: dir.path().join("a.mkv") }];
1404:        let specs = vec![JobSpec { argv: vec!["ZZZ-reviewer".to_string()], output: out_dir.path().join("a.mkv") }];

$ cargo test -p muxsmith-core run_batch          # in repo-copy
running 2 tests
test executor::queue::tests::run_batch_writes_job_log_files ... ok
test executor::queue::tests::run_batch_emits_started_output_finished_in_order ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.05s
```

CONTROL (the fire), same instrument, mutating a value that IS observed:

```
$ python3 .../mutate.py <restored queue.rs> exit          # Some(0) -> Some(7)
mode=exit lines mutated=2
$ cargo test -p muxsmith-core run_batch
running 2 tests
test executor::queue::tests::run_batch_writes_job_log_files ... ok
test executor::queue::tests::run_batch_emits_started_output_finished_in_order ... FAILED
thread '...run_batch_emits_started_output_finished_in_order' panicked at crates/muxsmith-core/src/executor/queue.rs:1381:9:
assertion `left == right` failed
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.05s
```

Reading by source, in the same direction: `FakeSpawner::spawn`
(`crates/muxsmith-core/src/executor/spawn.rs:199-201`) records argv into
`self.spawned` and otherwise ignores it; `.spawned()` is read at
`queue.rs:669` and `:1090`, neither inside a moved test. argv does reach disk
via `joblog.rs:227`, but the joblog test asserts existence only.

### 4.3 Test integrity - assertion sets diffed structurally

`test_moves.py` extracts each test from `HEAD~1:src-tauri/src/run.rs` and from
`HEAD:crates/muxsmith-core/src/executor/queue.rs` and diffs both the whole
function and the assertion lines alone.

```
run_batch_emits_started_output_finished_in_order  (pre 31 lines / post 32 lines)
--- assertion lines: pre=4 post=4
-assert_eq!(
+assert_eq!(outcomes[0].state, JobState::Ok);      # the only assertion delta:
                                                  # rewrapped to one line, path
                                                  # dropped because JobState is
                                                  # in scope in core
run_batch_writes_job_log_files  (pre 14 lines / post 18 lines)
--- assertion lines: pre=2 post=2
(assertion set identical)
```

The non-assertion adaptations are semantics-preserving by construction:
`opts()` at `HEAD~1:src-tauri/src/run.rs:945-950` is
`QueueOpts { jobs: 1, fail_fast: false }` and `ctl(n)` at `:952-954` is
`QueueControl::new(spec_count, Arc::new(AtomicBool::new(false)))`, which is
character for character what was inlined.

Nothing else left the shell's test module:

```
$ git show HEAD~1:src-tauri/src/run.rs | grep -c "#\[test\]"   -> 40
$ grep -c "#\[test\]" src-tauri/src/run.rs                     -> 38
$ diff <(pre-state test fn names, sorted) <(post-state, sorted)
< run_batch_emits_started_output_finished_in_order
< run_batch_writes_job_log_files
```

### 4.4 Acceptance observable 2 (design section 7 item 2), my own fires

```
=== FIRE (pre-state, read straight from the git object) ===
$ git show HEAD~1:src-tauri/src/run.rs | grep -n "fn run_batch"
758:fn run_batch(
1204:    fn run_batch_emits_started_output_finished_in_order() {
1310:    fn run_batch_writes_job_log_files() {
exit=0

=== GREEN ===
$ grep -n "fn run_batch" src-tauri/src/run.rs
exit=1

=== GREEN member-by-member: every surviving occurrence of the NAME ===
src-tauri/src/run.rs:44:use muxsmith_core::executor::queue::{QueueControl, QueueOpts, run_batch};
src-tauri/src/run.rs:447:        // Armed for the whole runner-thread body (fix): if `run_batch`'s
src-tauri/src/run.rs:462:        let (outcomes, logger) = run_batch(&specs, &spawner, opts, &ctl, logger, |event| {
src-tauri/src/run.rs:657:/// worker panic used to propagate straight out through [`run_batch`]'s
src-tauri/src/run.rs:1104:        run_batch(&specs, &fake, opts(), &control, None, |_| {});
src-tauri/src/run.rs:1158:    /// real trigger is `run_batch`'s `handle.join().expect(...)` when the
```

Six survivors, recomputed from that enumeration: one import, two call
expressions, three prose mentions. `fn run_batch` matches none, so the pass is
an absence of the definition and not of the name.

Both surfaces on the core path:

```
crates/muxsmith-cli/src/commands/run.rs:13:use muxsmith_core::executor::queue::{JobEvent, QueueControl, QueueOpts, run_batch};
src-tauri/src/run.rs:44:use muxsmith_core::executor::queue::{QueueControl, QueueOpts, run_batch};
```

Both moved tests run under `cargo test -p muxsmith-core`, by name:

```
test executor::queue::tests::run_batch_emits_started_output_finished_in_order ... ok
test executor::queue::tests::run_batch_writes_job_log_files ... ok
```

Deduplication actually landed: the `expect` string existed twice pre-state and
once now, byte-identical.

```
$ git grep -n "queue worker thread panicked" HEAD~1 -- '*.rs'
crates/muxsmith-cli/src/commands/run.rs:235
crates/muxsmith-core/src/report/mod.rs:185   (prose in a DiagCode doc)
src-tauri/src/run.rs:775
$ git grep -n "queue worker thread panicked" -- '*.rs'
crates/muxsmith-core/src/executor/queue.rs:365
crates/muxsmith-core/src/report/mod.rs:185   (same prose)
```

### 4.5 Acceptance observable 3 (design section 7 item 3), my own fires

```
=== FIRE 3a ===                            === GREEN 3a ===
$ git grep -n "MUXSMITH_RUNS_ROOT" HEAD~1 -- src-tauri
src-tauri/src/run.rs:801                   $ grep -rn "MUXSMITH_RUNS_ROOT" src-tauri/
src-tauri/src/run.rs:806                   exit=1
exit=0

=== FIRE 3b ===                            === GREEN 3b ===
$ git grep -n "resolve_runs_root" HEAD~1 -- src-tauri
src-tauri/src/run.rs:301                   $ grep -rn "resolve_runs_root" src-tauri/
src-tauri/src/run.rs:505                   exit=1
src-tauri/src/run.rs:511
src-tauri/src/run.rs:803
exit=0
```

Whole-tree sweep for survivors outside `docs/`, with its control:

```
$ grep -rn "resolve_runs_root" --exclude-dir={.git,target,node_modules,docs,.worktrees} .
(no output)
$ grep -rn "default_runs_root" --exclude-dir={.git,target,node_modules,docs,.worktrees} . | wc -l
15
```

`MUXSMITH_RUNS_ROOT` pre-state tree set, my own recount: 9 lines (5 CLI test
sites, 2 CLI source lines, 2 src-tauri source lines), matching the brief and the
report.

CLI gate byte-identity, by my own structural extraction of `create_logger`
(`cli_gate.py`; the region shifted from `:280-309` to `:261-290`, so it is
located by content):

```
pre-state region lines 280-309 (30 lines)
post-state region lines 261-290 (30 lines)
=== GATE REGION DIFF === (byte-identical)
=== CONTROL: instrument must be able to show a difference ===
-    logger
+    logger // reviewer mutation
=== MUXSMITH_RUNS_ROOT lines inside the region, both states ===
HEAD~1: ['/// [`default_runs_root`] (debug builds only: a `MUXSMITH_RUNS_ROOT` env', '    let runs_root = std::env::var_os("MUXSMITH_RUNS_ROOT")']
HEAD  : (identical)
```

`cargo test -p muxsmith-gui` green, 80 passed. The two shell-side hunks in
`mod tests` are pure deletions, so the count moved by exactly the two moved
tests and no surviving test was adapted (4.3).

### 4.6 Gate subset, re-run by me

```
$ cargo fmt --all --check                                   exit=0
  FIRE (my own misformatted file, .../fmtfire/fire.rs):
  $ rustfmt --edition 2024 --check .../fmtfire/fire.rs
  Diff in .../fire.rs:1:
  -fn  main( ) {   -let z=42;   +fn main() {   +    let z = 42;
  exit=1
$ cargo clippy --workspace --all-targets -- -D warnings      Finished, no diagnostics
$ cargo test --workspace                                     exit=0
  39 "test result:" lines; no FAILED / failures / panicked
  aggregate over every result line: passed=494 failed=0 ignored=0
$ cargo test -p muxsmith-core     lib target: 121 passed
$ cargo test -p muxsmith-gui      lib target:  80 passed
```

All three report aggregates reproduce exactly. Per-file line counts also
reproduce (`git show --numstat 9b2843f`): CLI 18/37 (net -19), queue 104/1,
src-tauri 8/122.

Behavior preservation on the CLI path has a real producer here, not a skipped
one: `mkvmerge` is present on this machine (`mkvmerge v100.0`), so
`crates/muxsmith-cli/tests/run_live.rs` actually muxes rather than early-returning
on its `have_mkvmerge()` guard.

```
$ cargo test -p muxsmith-cli --test run_live -- --nocapture
test live_run_muxes_two_sources_and_reports_exit_zero ... ok
test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok
test zero_rule_keep_profile_is_a_pure_passthrough ... ok
test live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched ... ok
test result: ok. 4 passed
```

That suite drives the rewritten call path end to end with the insta snapshot
`run_live__live_run_muxes_two_sources_and_reports_exit_zero.snap` unchanged, and
with `MUXSMITH_RUNS_ROOT` pointed at a tempdir, so the CLI gate that D97
protects is exercised too.

Typography sweep over the three committed files, with its fire:

```
$ grep -nP '[\x{2014}\x{2013}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2012}\x{2015}\x{2212}]' \
    crates/muxsmith-core/src/executor/queue.rs src-tauri/src/run.rs crates/muxsmith-cli/src/commands/run.rs
exit=1
FIRE: same pattern against my own control file carrying all eleven glyphs -> 1 match, exit=0
```

Latitude sweep over the same three files, with its fire (the first form of this
check returned empty because no `TODO` exists anywhere in the Rust tree, so the
empty result proved nothing until the control token was added):

```
$ grep -niE "TODO|FIXME|XXX|if desired|at your discretion|feel free|may choose|optional(ly)?" <the three files>
exit=1
FIRE: same invocation plus the token "deliberately", known present:
crates/muxsmith-cli/src/commands/run.rs:272 / queue.rs:338 / queue.rs:838 / src-tauri/src/run.rs:523 / :1009
exit=0
```

### 4.7 House-dimension counts, measured

Fully-qualified cross-crate doc-link form, pre-state, Rust sources:

```
$ git grep -n -E '\[`muxsmith_core::' HEAD~1 -- 'crates/*.rs' 'src-tauri/*.rs'
crates/muxsmith-cli/src/commands/dry_run.rs:18
crates/muxsmith-cli/src/commands/run.rs:41
src-tauri/src/error.rs:9, :94, :169, :188
src-tauri/src/run.rs:360
src-tauri/src/settings.rs:53
by file: dry_run 1, cli run 1, error 4, tauri run 1, settings 1  -> 8, recomputed
```

Explicit-path form, same scope: 2, not 1 (LOW-2). Control for the regex
machinery: the relaxed pattern over the same pathspec returns 396 doc-comment
link lines, so the two-hit result is a real measurement.

`testing-support-helpers` (`docs/conventions.yaml`, entry read in full): "Cross-file
test helpers ... duplicating a helper within a crate is a defect. Scope: same-crate
consolidation". Destination-module pattern, measured: 35 `spec(` call sites in
`crates/muxsmith-core/src/executor/queue.rs`, exactly one `JobSpec {` literal in
the file (at `:511`, inside the helper).

Placement of the hoisted function follows the file's order (public API before
private helpers): `pub fn run_queue` `:181`, `pub fn run_batch` `:348`,
`fn worker_count` `:379`, `fn lock_outcomes` `:394`,
`fn recover_panicked_worker` `:424`.

Within-file scope qualifiers in the two Files lists (the measurement behind Q3):

```
Task 1 Files list, occurrences of "only": 5
Task 2 Files list, occurrences of "only": 0
```

### 4.8 Rustdoc, private items - measured, not relayed (HARVEST a)

At HEAD, the real tree:

```
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
error: `run` is both a function and a module
  --> src-tauri/src/lib.rs:54:21
error: `run` is both a function and a module
  --> src-tauri/src/lib.rs:87:15
error: could not document `muxsmith-gui`
```

Exactly two repairable diagnostics. No unresolved link anywhere in the
workspace. That absence has its own fire, run on my scratch copy so no product
file was touched: a broken intra-doc link injected on the PRIVATE fn
`worker_count` inside the PUBLIC module `queue`.

```
$ # injected: /// Reviewer fire: [`no_such_symbol_xyz`]   above fn worker_count
$ RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-core --no-deps
   Generated .../target/doc/muxsmith_core/index.html            <- passes, blind
$ RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-core --no-deps --document-private-items
error: unresolved link to `no_such_symbol_xyz`
   --> crates/muxsmith-core/src/executor/queue.rs:379:22
error: could not document `muxsmith-core`                        <- fires
```

## 5. HARVEST

### Dominant patterns observed

1. **Locate-by-content held under a moving file.** Task 1 shifted
   `src-tauri/src/run.rs`, so every line number in the plan's Task-2 steps is
   stale by design. The implementer re-derived all five anchor sets before
   touching anything and said so. Reproduced; the anchors it pasted match the
   git objects exactly. This is the discipline `proc-57-briefs-not-ground-truth`
   asks for, executed without prompting.
2. **A self-caught broken instrument.** The report records that its first
   typography sweep returned empty against a control file that contained every
   glyph, because `ugrep`'s `$'a\|b'` alternation matched nothing, and that it
   re-armed with `-P` until the control fired. That is the make-the-check-fire
   rule applied to the checker's own tooling
   (`docs/process-conventions.yaml:576`), and it is worth naming as a positive
   instance rather than only recording violations of that entry.
3. **Repeated rejection: none.** Nothing in the design was re-opened. The
   "stay discarded" constraint held trivially and verifiably: the commit touches
   three files and none of them is `job.rs`, `joblog.rs` or `spawn.rs`.

### (a) The implementer's surfaced item 1, measured

**The recurrence claim: confirmed in kind, with one correction to its framing.**
Both Plan-9 instances are the same mechanical cause - an import removal leaving
an intra-doc link unresolvable in `src-tauri/src/run.rs` - one task apart. But
the ROADMAP section is titled "rustdoc does not link-check private **modules**",
and measured, the blind spot is wider than that: my fire (4.8) put a broken link
on a **private item inside a public module** of `muxsmith-core` and the plain
gate passed it. `queue.rs` alone holds three such private fns. Whoever lands the
flag should widen that heading, because the current wording implies `src-tauri`
is the only exposed crate and it is not.

**The cost figure: confirmed stale, and the corrected figure is two.** My own run
at HEAD produces exactly the two `run`-ambiguity errors at
`src-tauri/src/lib.rs:54:21` and `:87:15`. The third item the ROADMAP counts was
the unresolved `` [`plan_batch`] `` link, fixed in commit `fed55be` ("requalify
the plan_batch link"); at HEAD~1 that site already reads
`` [`muxsmith_core::planner::plan_batch`] `` (`src-tauri/src/run.rs:360`,
measured), so the figure was two before Task 2 as well - the implementer's
inference is sound and I verified its premise directly rather than by inference.

The rider is right and worth keeping: `grep -c "unresolved link"` is the wrong
instrument for that cost, because the two survivors are ambiguity diagnostics
under the same `broken_intra_doc_links` lint. The single invocation in 4.8
produces both classes, which is what makes it the correct instrument.

### (b) The implementer's surfaced item 2: endorse, with a sharper trigger

The proposal is that a verbatim cross-crate move should carry a doc-link sweep
in its file list the way it carries an import sweep. Endorsed, but **a move is
not the trigger; an import removal is.** Both Plan-9 instances were produced by
the same step, and the handle is greppable rather than something to notice: for
every `use` line a task deletes, grep the listed files for `` [`<symbol>`] ``
and re-point or de-link.

Stated that way it also covers Task 1's case (a spec-amendment ripple is the
same class - a reference invalidated by an edit), and it satisfies the
"trigger you can read, handgrip you execute" bar. Concretely, for the two
remaining hoist-shaped items: a task whose Files list says a file "loses" an
import carries a step "grep every listed file for intra-doc links naming the
removed symbols", and the design puts the resulting edits INTO the file list, so
the grant boundary never has to be crossed at the keyboard.

### (c) Over-restriction (asked for Q3 and Q4): yes, with new calibration data

Both repairs would have forced a NEEDS_CONTEXT round-trip whose fork has one
branch. "Should the doc link you just broke point at the thing it names?" and
"should the comment that says 'see the drain loop below' keep saying that after
you deleted the drain loop?" have no decision content: the alternative in each
case is shipping something the task itself made false.

**The new datum is the contrast with Task 1, and it resolves that entry's
framing rather than reinforcing it.** Task 1's implementer surfaced instead of
fixing and it cost a licensed fix round (`docs/decision-ledger.yaml:4649`);
Task 2's fixed and surfaced; both reviewers ruled the fix side correct. The
discriminator is clean and measurable: Task 1's invalidated reference sat in
`crates/muxsmith-core/src/identify.rs`, a file **not on Task 1's Files list**, so
stopping was right; Task 2's sat in two listed files, so repairing was right.
The boundary is coherent. What is missing is that nobody wrote the rule down, so
two implementers reached opposite - and individually correct - behaviour by feel,
one of them at the cost of a fix round.

**Second measured datum on the same axis: within-file scope is controlled by the
word "only", and that has never been said out loud.** Task 1's Files list carries
5 "only" qualifiers; Task 2's carries 0 (4.7). So Task 1's list genuinely
enumerated within-file and Task 2's genuinely is silent within-file, which is
exactly the condition the grant fills.

Recommended amendment to
`latitude-carveout-zero-content-structural-forks`, which already carries the
companion brief-authoring rule "a Files/Interfaces list reads as an enumeration
boundary": add that the boundary is over **files** unless an entry carries a
within-file qualifier, and that repairing a reference (doc link, comment
referent, import) which the task's own enumerated edit invalidated, inside a
listed file, with no meaning changed beyond the referent, is in scope. That
loosens nothing - all four zero-outward-effect conditions still have to hold -
and it removes the coin flip.

Counter-datum, so this is not read as a general loosening: Q5 is the same
boundary working exactly as designed. There the fork had real content (house
pattern versus fixture preservation, with a fixture-preserving option available),
and the implementer crossed it. Over-restriction evidence in one direction is not
evidence for the other.

### (d) For Task 3

1. **Task 3 edits both files this defect class fired in** (`queue.rs`,
   `src-tauri/src/run.rs`) with a compiler-driven `JobOutcome` sweep, which is
   the same shape of change that produced both instances. Give it (b)'s
   import-removal grep step explicitly in its brief.
2. **MEDIUM-1 lands in `queue.rs`, which Task 3 edits.** If the controller rules
   on the rustdoc text before Task 3 dispatches, the fix rides that commit for
   free. Without a ruling it must not be picked up silently.
3. **Correction to the implementer's Task-3 note.** Its report says "Task 3's
   `JobOutcome` compiler sweep now also has to cover the two tests this task moved
   into `queue.rs`". Measured, it does not: neither moved test mentions
   `JobOutcome` at all (`grep -n "JobOutcome"` over `queue.rs:1360-1420` returns
   nothing; control: the file holds 11 `JobOutcome` mentions, and the two
   construction sites a new field would break are `:316` and `:449`, both
   predating this task). The moved tests read `outcomes[0].state` by field access,
   which a new field does not break.
4. **Anchors shifted by this commit, my own measurement** (the plan's `:396` and
   `:738` are stale, as is every `src-tauri/src/run.rs` number in the plan):
   `fn recover_panicked_worker` at `crates/muxsmith-core/src/executor/queue.rs:424`,
   the `eprintln!` at `:441`, `worker_panic_is_reported_as_failed_not_cancelled`
   at `:783`.
5. **Coverage note, pre-existing but newly relevant.** The "persistence stays
   unconditional under `--json`" invariant has no joint end-to-end producer on the
   CLI path: no subprocess test asserts a populated joblog directory while
   `--json` is set. The two halves are covered separately - the tee by
   `run_batch_writes_job_log_files` in core, the suppression by
   `crates/muxsmith-cli/tests/run_cli.rs:185`. This task did not weaken it, but it
   is the task that moved the tee across a crate boundary, so the gap is worth
   recording where the next executor-touching task can see it.
