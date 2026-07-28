# Amendment 3, plan side - author report

Author: plan-side amendment author (Fable 5, fresh dispatch). One file
edited: `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`.
Committed pathspec-scoped as `36d8538`; not pushed. This report file is
deliberately not committed (the commit grant was scoped to the plan file).

## 1. Status: DONE

All four must-carry items are in the plan, both open questions are decided
and recorded (in the plan's Amendment-3 log entry, not only here), the
fence is pointed at and not duplicated, no Files-list change, no design
latitude reaches Task 3's implementer in either form. Typography scan over
the whole plan post-edit: zero banned glyphs (pattern fire-verified
against an em-dash sample, count 1).

## 2. The exact diff (git show -U1 36d8538)

```diff
diff --git a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
index bd28b42..454ae56 100644
--- a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
+++ b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
@@ -86,3 +86,3 @@ Every design section -> the task or actor that implements it. This is the walk t
 | D95 one `"."` default in the seam | Task 1 |
-| D96 `run_batch` hoist + two tests move | Task 2 |
+| D96 `run_batch` hoist + two tests move | Task 2; the amendment-3 rustdoc rider -> Task 3 Step 2 |
 | D97 runs-root deletion, three call sites | Task 2 |
@@ -109,3 +109,3 @@ Every design section -> the task or actor that implements it. This is the walk t
 - 1 -> 2: same files (CLI `run.rs`, src-tauri `run.rs`). Task 1 migrates CLI `run`'s planning stretch onto the seam while **retaining the inline mpsc queue block byte-unchanged**; Task 2 replaces that block with the hoisted core `run_batch`. This staged cut is deliberate: both intermediate states compile and pass the suite, and the design's end state for copy 2 (D91 mapping 2: "the queue via core `run_batch` (D96)") is reached at Task 2.
-- 2 -> 3: same file (core `executor/queue.rs`): Task 2 moves `run_batch` and the two tests in; Task 3 then edits `recover_panicked_worker` and runs its compiler-driven `JobOutcome` sweep over the tests in their final home.
+- 2 -> 3: same file (core `executor/queue.rs`): Task 2 moves `run_batch` and the two tests in; Task 3 then edits `recover_panicked_worker`, replaces `run_batch`'s rustdoc with D96's amendment-3 rider fence (amendment 3), and runs its compiler-driven `JobOutcome` sweep over the tests in their final home.
 - 3 -> 6: the e2e panic-render test needs `JobOutcome.panic` on the wire (D98) and the `JobRow.vue` render site (D100).
@@ -192,3 +192,3 @@ Read first: design D96 (including the caller-side checklist) and D97 (the three-
 
-- [ ] **Step 1: the move (D96).** Move `run_batch` (src-tauri `run.rs:782-804` at authoring; locate by `fn run_batch`) as-is into `crates/muxsmith-core/src/executor/queue.rs`, `pub`, exact signature per D96's fence, rustdoc moved with it. The boundary is exactly where today's function ends: it does NOT absorb `run_document`, `finalize_joblog`, or the CLI's joblog messages.
+- [ ] **Step 1: the move (D96).** Move `run_batch` (src-tauri `run.rs:782-804` at authoring; locate by `fn run_batch`) as-is into `crates/muxsmith-core/src/executor/queue.rs`, `pub`, exact signature per D96's fence, rustdoc moved with it (amendment 3: this clause stays as the record of Task 2's order; the moved rustdoc is restated for its core home by Task 3 Step 2, transcribing D96's amendment-3 rider fence, so this clause does not describe the plan's end state). The boundary is exactly where today's function ends: it does NOT absorb `run_document`, `finalize_joblog`, or the CLI's joblog messages.
 - [ ] **Step 2: GUI caller.** The runner thread calls the core function; `TeardownGuard` stays wrapped AROUND the call on the runner thread; `fail_fast` stays caller-built (`false` hardcoded in the GUI's `QueueOpts`). Nothing else in the shell's composition (reservation, cancel, teardown tests) moves.
@@ -215,5 +215,5 @@ git -c commit.gpgsign=false commit -m "executor: hoist run_batch into core, dele
 
-## Task 3: The worker-panic payload, end to end (D98, D99, D100; spec S-1 WorkerPanicked row, S-2)
+## Task 3: The worker-panic payload, end to end (D98, D99, D100, D96's amendment-3 rider; spec S-1 WorkerPanicked row, S-2)
 
-Read first: design D98 (field, wire memo, fork 9, fork 12), D99 (both rejected alternatives, the four Fluent fences, catalog obligations), D100 (render site fence, scope boundaries), section 0 notes 4 and 5; design section 5 (panic bullets). Model tier: mid.
+Read first: design D98 (field, wire memo, fork 9, fork 12), D99 (both rejected alternatives, the four Fluent fences, catalog obligations), D100 (render site fence, scope boundaries), D96's amendment-3 rider (the replacement `run_batch` doc fence, its per-passage account, and the no-src-tauri-sentence decision), section 0 notes 4 and 5; design section 5 (panic bullets). Model tier: mid.
 
@@ -221,3 +221,3 @@ Read first: design D98 (field, wire memo, fork 9, fork 12), D99 (both rejected a
 - Modify: `crates/muxsmith-core/src/executor/job.rs` (`JobOutcome.panic` field, D98's doc verbatim)
-- Modify: `crates/muxsmith-core/src/executor/queue.rs` (`recover_panicked_worker` sets `Some`; the `eprintln!` deleted; the licence comment rewritten; the panic-recovery test extended; every flagged constructor in this file gains `panic: None`)
+- Modify: `crates/muxsmith-core/src/executor/queue.rs` (`recover_panicked_worker` sets `Some`; the `eprintln!` deleted; the licence comment rewritten; the panic-recovery test extended; every flagged constructor in this file gains `panic: None`; the `run_batch` rustdoc replaced with D96's amendment-3 rider fence)
 - Modify: `crates/muxsmith-core/src/executor/joblog.rs` (`JobRecord` gains `panic: Option<&'a str>` via `outcome.panic.as_deref()`)
@@ -239,10 +239,11 @@ Read first: design D98 (field, wire memo, fork 9, fork 12), D99 (both rejected a
 - [ ] **Step 1: the field (D98).** Add `panic: Option<String>` to `JobOutcome` with D98's doc comment verbatim; plainly serialized (no `skip_serializing_if`). `recover_panicked_worker` sets `panic: Some(message)` from the existing downcast chain, keeps `errors: vec![format!("{}: job {index}", DiagCode::WorkerPanicked.key())]` byte-identical, and deletes the `eprintln!` (locate by its literal text; `queue.rs:396` at authoring). Rewrite the function's licence doc block (the "core's one deliberate prose-free exception" passage) to state D98 fork 12's replacement rationale: the payload is now carried as data and rendered through the catalog at presentation time - the spec's normal path, not an exception; core still authors no user-facing prose (`core-37-prose-free-core`).
-- [ ] **Step 2: the compiler sweep.** Build; every flagged existing `JobOutcome` constructor gains `panic: None`. `recover_panicked_worker` remains the only `Some`-setter among non-test code; the two new deliberately-`Some` fixtures are this task's CLI unit test and Task 6's e2e event (design round-2 note 1).
-- [ ] **Step 3: the wire mirrors (D98's memo, complete):** `JobRecord` in `joblog.rs` gains `panic: Option<&'a str>` (`outcome.panic.as_deref()`); `src/ipc.ts` `JobOutcome` gains `panic: string | null` required (so `RunJobEntry extends JobOutcome` inherits it) and `JobLogRecord` gains the same; the smoke live-run fixture literals add `panic: null` (the type-check enumerates them - do not hunt by eye).
-- [ ] **Step 4: CLI rendering (D99).** `render_finished`'s `JobState::Failed` arm branches on `outcome.panic` (typed, no string sniffing): `None` -> unchanged `run-job-failed`; `Some(detail)` -> line 1 `run-job-panicked`, line 2 the `worker-panicked` catalog message with `$detail`. Copy the four Fluent texts character for character from D99's four fences (en/de `diagnostics.ftl` replacement line; en/de `cli.ftl` new key next to the other `run-job-*` lines).
-- [ ] **Step 5: catalog obligations (D99's list, complete):** `run-job-panicked` joins `ALLOWLISTED_CLI_KEYS` with fixture args `[("index", "1"), ("total", "3"), ("output", "/out/movie.mkv")]` in `allowlisted_cli_key_args`; the `DiagCode::WorkerPanicked` fixture row changes from `vec![]` to `vec![("detail", "queue worker thread panicked")]`.
-- [ ] **Step 6: the pinned tests.** (a) Extend `worker_panic_is_reported_as_failed_not_cancelled` (queue.rs inline tests) to additionally assert the recovered outcome carries `panic: Some(..)` (the payload string) AND the unchanged `worker-panicked: job N` prefix token in `errors` - the acceptance-4 core assertion. (b) CLI: `outcome()` helper gains `panic: None`; new unit test `finished_panicked_renders_two_lines_without_na` in `commands/run.rs`'s test module, mirroring `finished_failed_renders_exit_code` with `panic: Some(..)`, asserting both lines render and neither contains `"n/a"`.
-- [ ] **Step 7: GUI render (D100).** In `JobRow.vue`'s state cell: computed `panicDetail` (`state.kind === "finished" ? state.outcome.panic : null`) and the span exactly as D100's fence (`data-testid="job-panic"`, `$t("worker-panicked", { detail: panicDetail })`). Placement/styling inside the cell is implementer-owned (`latitude-carveout-presentation-tokens`); key, param, gating condition and testid are fixed. No `JobRowData` change; no history-table or RunHistory-export change.
-- [ ] **Step 8: spec amendments** S-1 (the WorkerPanicked-row replacement text only) and S-2, exactly as the design's section 3 fences write them.
-- [ ] **Step 9: verification.**
+- [ ] **Step 2: the `run_batch` rustdoc restatement (D96's amendment-3 rider).** Replace the `///` block immediately above `pub fn run_batch` in `crates/muxsmith-core/src/executor/queue.rs` (`:327-347` at amendment time; locate by the `pub fn run_batch` anchor, never by line number) with the replacement doc comment in D96's amendment-3 rider, transcribed character for character, line wrapping included: the rider's fence arrives pre-wrapped (max width 75, measured at amendment time), so Task 1's rustdoc-line-wrapping allowance does NOT apply here. No other change to the function - body, signature and everything else in the file outside this `///` block stay byte-identical under this step - and no src-tauri file is touched on this amendment's account (the rider adds no src-tauri sentence; the teardown rationale already lives caller-side at the rider's three named sites). The fence itself lives in the design only: this plan deliberately points at the rider instead of copying it, so the character-for-character contract has exactly one home.
+- [ ] **Step 3: the compiler sweep.** Build; every flagged existing `JobOutcome` constructor gains `panic: None`. `recover_panicked_worker` remains the only `Some`-setter among non-test code; the two new deliberately-`Some` fixtures are this task's CLI unit test and Task 6's e2e event (design round-2 note 1).
+- [ ] **Step 4: the wire mirrors (D98's memo, complete):** `JobRecord` in `joblog.rs` gains `panic: Option<&'a str>` (`outcome.panic.as_deref()`); `src/ipc.ts` `JobOutcome` gains `panic: string | null` required (so `RunJobEntry extends JobOutcome` inherits it) and `JobLogRecord` gains the same; the smoke live-run fixture literals add `panic: null` (the type-check enumerates them - do not hunt by eye).
+- [ ] **Step 5: CLI rendering (D99).** `render_finished`'s `JobState::Failed` arm branches on `outcome.panic` (typed, no string sniffing): `None` -> unchanged `run-job-failed`; `Some(detail)` -> line 1 `run-job-panicked`, line 2 the `worker-panicked` catalog message with `$detail`. Copy the four Fluent texts character for character from D99's four fences (en/de `diagnostics.ftl` replacement line; en/de `cli.ftl` new key next to the other `run-job-*` lines).
+- [ ] **Step 6: catalog obligations (D99's list, complete):** `run-job-panicked` joins `ALLOWLISTED_CLI_KEYS` with fixture args `[("index", "1"), ("total", "3"), ("output", "/out/movie.mkv")]` in `allowlisted_cli_key_args`; the `DiagCode::WorkerPanicked` fixture row changes from `vec![]` to `vec![("detail", "queue worker thread panicked")]`.
+- [ ] **Step 7: the pinned tests.** (a) Extend `worker_panic_is_reported_as_failed_not_cancelled` (queue.rs inline tests) to additionally assert the recovered outcome carries `panic: Some(..)` (the payload string) AND the unchanged `worker-panicked: job N` prefix token in `errors` - the acceptance-4 core assertion. (b) CLI: `outcome()` helper gains `panic: None`; new unit test `finished_panicked_renders_two_lines_without_na` in `commands/run.rs`'s test module, mirroring `finished_failed_renders_exit_code` with `panic: Some(..)`, asserting both lines render and neither contains `"n/a"`.
+- [ ] **Step 8: GUI render (D100).** In `JobRow.vue`'s state cell: computed `panicDetail` (`state.kind === "finished" ? state.outcome.panic : null`) and the span exactly as D100's fence (`data-testid="job-panic"`, `$t("worker-panicked", { detail: panicDetail })`). Placement/styling inside the cell is implementer-owned (`latitude-carveout-presentation-tokens`); key, param, gating condition and testid are fixed. No `JobRowData` change; no history-table or RunHistory-export change.
+- [ ] **Step 9: spec amendments** S-1 (the WorkerPanicked-row replacement text only) and S-2, exactly as the design's section 3 fences write them.
+- [ ] **Step 10: verification.**
   - **Acceptance observable 4's task-3 emitters, as stated in the design's section 7 item 4** (the e2e emitter rides Task 6). The core-stdio absence check: `grep -rn "eprintln!\|println!\|print!(" crates/muxsmith-core/src` -> expected post-state EXACTLY one hit, the comment line in `lib.rs` reading "... `eprintln!` sites ..." - zero call sites. Fire: the same grep pre-edit returns exactly 2 hits (the comment + the `queue.rs` call; pasted in the authoring section). Control that pattern and pathspec produce output on a known-present case: the same grep over `crates/muxsmith-cli/src` hits six files (authoring run pasted).
@@ -250,3 +251,3 @@ Read first: design D98 (field, wire memo, fork 9, fork 12), D99 (both rejected a
   - `pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e` - green, foreground (the fixture sweep and the ipc.ts mirror are what these enforce).
-- [ ] **Step 10: commit.**
+- [ ] **Step 11: commit.**
 
@@ -256,3 +257,3 @@ git add crates/muxsmith-core/src/executor/job.rs crates/muxsmith-core/src/execut
 
-plus any additional file the Step-2 compiler sweep touched (stage each by name; never `git add -A`), then:
+plus any additional file the Step-3 compiler sweep touched (stage each by name; never `git add -A`), then:
 
@@ -455 +456,10 @@ The registered ROADMAP trigger "A FOURTH e2e spec file needs the local `name()`
 - Sequencing ownership map updated (four smoke-writers, serial), the close actions note the trigger record already written controller-side, and the amendment-2 anchors joined the authoring-verification section with the pasted fire and count measurements.
+
+## Amendment 3 (2026-07-28, owner-ruled design change mid-execution, after Task 2)
+
+Routing: `.superpowers/sdd/plan-9/amendment-3-brief.md` (design half) and `.superpowers/sdd/plan-9/amendment-3-plan-brief.md` (this half); amended design at commit `08621cb`, its Round-4 amendment-log entry the authoritative delta record, delta review APPROVED with no findings (`.superpowers/sdd/plan-9/amendment-3-verdict.md`). The two logs are deliberately not numbered in lockstep: amendment 2 was plan-only, so this plan's amendment 3 is the design log's Round 4 - each log follows its own file's shape. The ruling: Task 2 moved `run_batch`'s rustdoc verbatim, exactly as D96's "as-is" and this plan's "rustdoc moved with it" ordered, while the same commit gave the function its second caller, falsifying three caller-specific passages of that doc (Task-2 review MEDIUM-1); the owner ruled the correction a design change - the restated doc entered D96 as the amendment-3 rider, and the code edit rides Task 3, which already owns `queue.rs`. What moved in this plan:
+
+- **Task 3 gained Step 2, the `run_batch` rustdoc restatement** (steps renumbered to eleven, recounted from the headers; the one internal step cross-reference, the staging note's "Step-2 compiler sweep", renumbered with them - no other step reference to Task 3 exists in this plan or in `progress.md`, both grepped). The step: replace the `///` block immediately above `pub fn run_batch` in `crates/muxsmith-core/src/executor/queue.rs` with D96's amendment-3 rider fence, character for character including the pre-wrapped lines, located by the `pub fn run_batch` anchor, no other change to the function. Its own step rather than a Step-1 rider, decided here: the work answers a different design entry (the D96 rider, not D98), a dedicated checkbox keeps the reviewer's design-coverage walk one-to-one, and amendments 1 and 2 both added amendment work as dedicated steps. The fence is deliberately NOT duplicated into this plan: the plan points at the rider, so the character-for-character contract has exactly one home and cannot drift.
+- **Task 3's "Read first" gained the rider**, its header parenthetical now names it, the coverage map's D96 row carries "the amendment-3 rustdoc rider -> Task 3 Step 2", and the sequencing 2->3 edge names the rustdoc replacement among Task 3's `queue.rs` edits - the reference sweep over the enumerations the new step falsified, each of them a surface the plan reviewer's walk consumes.
+- **No Files-list change:** `crates/muxsmith-core/src/executor/queue.rs` already sits on Task 3's exhaustive list and in its Step-11 `git add` line. `src-tauri/src/run.rs` is NOT on that list and does not join it - the rider deliberately adds no src-tauri sentence, and the earlier controller-brief claim that the file was listed is refuted (design author's refutation, reviewer's re-measurement, and this amendment's own grep agree). Decided, not left open: the queue.rs Files-entry parenthetical DID gain the rustdoc clause - optional under the file-vs-within-file ruling (`proc-latitude-clause-boundary`: the entry carries no 'only', span or region qualifier, so it never constrained within-file work), added because a work description enumerating five items and omitting the sixth invites the exhaustive misreading the Task-2 review measured implementers resolving by feel.
+- **Task 2 Step 1's "rustdoc moved with it" gained a historical qualifier** - decided, not left open: the clause stays verbatim as the record of Task 2's order, with an amendment-3 parenthetical pointing at Task 3 Step 2, because the design side closed the identical misreading by qualifying its own executed "moves as-is" sentence in place, and a silent leave-as-is would re-open for every plan reader exactly the reading amendment 3 exists to close.
```

## 3. The two decisions (brief item 5)

**Decision 1 - Task 2 Step 1's "rustdoc moved with it" (plan `:193` at
review time): historical qualifier ADDED, wording left verbatim.** The
clause now carries a parenthetical: "(amendment 3: this clause stays as
the record of Task 2's order; the moved rustdoc is restated for its core
home by Task 3 Step 2, transcribing D96's amendment-3 rider fence, so
this clause does not describe the plan's end state)". Reasons: (a) the
design side closed the identical misreading by qualifying its own
executed mandate in place - D96's opening sentence now reads "amendment 3
restates the moved rustdoc for its new home ... body and signature stay
as-is" - and the Round-4 log states the purpose: so "moves as-is" cannot
be read as covering the rustdoc going forward. The plan's Task-2 wording
is the same executed mandate one document down; leaving it silent
re-opens exactly that reading for every plan reader, the whole-branch
reviewer at the close first among them. (b) The record is not rewritten:
the original clause stands verbatim; the parenthetical is additive and
marked as amendment-3 provenance. Steelman for leave-as-is (purity of the
executed record) considered and outweighed: the plan already annotates
executed history where reality moved (corrections table row 2), and the
brief itself warns that a silent leave-as-is invites the misreading.

**Decision 2 - the queue.rs Files-entry parenthetical: EXTENDED** with
"; the `run_batch` rustdoc replaced with D96's amendment-3 rider fence".
Reasons: (a) I read the ruling itself, not the brief's paraphrase - it
lives in `docs/process-conventions.yaml:353` (`proc-latitude-clause-boundary`,
adopted in `d7fd277`): "an entry constrains work WITHIN its file only
where it carries an explicit within-file qualifier - the word 'only', a
named line span, a named region". The queue.rs entry carries none of the
three forms (five named work items are not a span/region qualifier), so
the extension is genuinely optional, exactly as the brief said - the
entry never constrained within-file work. (b) Extended anyway: the
parenthetical is a work description a reviewer walks the diff against,
and the same ruling's own evidence record (the T2 review) measured two
implementers resolving the file-vs-within-file question in opposite
directions "by feel". A five-item enumeration omitting the sixth work
item is cheap to complete and expensive to leave as bait. The normative
source stays the new Step 2; the parenthetical stays descriptive.

**Placement call inside item 1 (also required: "make it, and say why"):
its own step - Task 3 Step 2**, before the compiler sweep; old Steps 2-10
renumbered to 3-11. Why a dedicated step: the work answers a different
design entry (the D96 rider, not D98), so folding it into Step 1 would
muddy the step-to-design citation discipline the plan maintains; a
dedicated checkbox keeps the reviewer's design-coverage walk one-to-one
and cannot be half-skipped mid-task; and both prior amendments added
their work as dedicated steps (amendment 1: Task 1 renumbered to seven;
amendment 2: Task 6 Step 3, renumbered to five), so renumbering is the
established house shape. Why position 2: it groups the two queue.rs doc
edits (Step 1 rewrites `recover_panicked_worker`'s licence block, Step 2
replaces `run_batch`'s block) ahead of the build-driven sweep, whose
"Build" then compiles the file with both doc edits in place. Renumber
blast radius measured before deciding: exactly one internal step
cross-reference in Task 3 (the staging note's "Step-2 compiler sweep",
now "Step-3"), zero "Task 3 Step N" references elsewhere in the plan,
zero in `progress.md` (which contains only the controller's
post-authoring ground-truth note - see section 5).

Two deliberate non-changes worth recording: (a) Task 3's per-task
verification does NOT gain a `cargo doc` run for the transcribed links -
the house pattern in this plan is a per-task subset without `cargo doc`
(Task 1, the largest fence-transcriber, runs none either), the rider
states the link set is unchanged and core-resolvable, and the ten-part
gate binds pre-push. (b) The new step explicitly closes the wrapping
fork the Task-1 precedent opens: Task 1's fences permit re-wrapping, this
fence does not ("character for character" includes the wrapping; the
fence arrives pre-wrapped, max width 75 - measured myself, see section 4).

## 4. Premise checks on brief items 1-3 (all reproduced; no refutations)

Every load-bearing claim re-verified at the files with my own runs;
pasted evidence:

- **"Read first" line, no rider yet (item 2):** `grep -n "Read first:
  design D98"` -> `218:Read first: design D98 (field, wire memo, fork 9,
  fork 12), D99 (both rejected alternatives, the four Fluent fences,
  catalog obligations), D100 (render site fence, scope boundaries),
  section 0 notes 4 and 5; design section 5 (panic bullets). Model tier:
  mid.` - names D98/D99/D100 only. Reproduced.
- **queue.rs on Task 3's Files list and in its `git add` (item 3):** the
  `grep -n "crates/muxsmith-core/src/executor/queue.rs"` run over the
  plan hit `:222` (the Task-3 Files entry) and `:254` (Task 3's commit
  block `git add ... crates/muxsmith-core/src/executor/queue.rs ...`).
  Reproduced.
- **`src-tauri/src/run.rs` NOT on that list (item 3, the refuted
  controller-brief claim):** verified as a positive enumeration, not a
  bare negative - `grep -n "src-tauri/src/run.rs"` over the plan returns
  hits at `:54`, `:57`, `:148`, `:169`, `:185`, `:199`, `:206` (authoring
  anchors, Task 1 and Task 2 material), none inside Task 3's Files region
  (`:220-232` at review time). The design author's refutation stands; the
  error was not inherited.
- **Global Constraints already bind the amendment log at execution time:**
  pasted from line 17: "its D-entries plus EVERY entry in its
  `## Amendment log` bind this plan, at the log's state at EXECUTION
  time, not at plan-authoring." Reproduced - which is why the new entry
  binds Task 3 with no Global-Constraints edit.
- **"rustdoc moved with it" at `:193` (item 5a):** `grep -n "rustdoc
  moved with it"` -> exactly `:193`, Task 2 Step 1. Reproduced.
- **The code target exists as the rider describes it:** `grep -n "pub fn
  run_batch" crates/muxsmith-core/src/executor/queue.rs` -> `:348`; sed
  over `:320-352` shows the contiguous `///` block at `:327-347`
  (first `///` line 327, function line 348), matching the rider's
  "`:327-347` at amendment time" exactly. The block on disk still
  carries the three falsified passages (window-emit/collector,
  `#[tauri::command]` wrapper, `finish_teardown`/D31), i.e. Task 3's
  Step 2 has a real pre-state.
- **The fence is pre-wrapped (my Step-2 text leans on this):** `awk` over
  design `:576-600` -> `fence lines: 25 max width: 75`; control: a known
  long plan line measured 963. The borrowed verdict claim reproduced with
  my own instrument.

## 5. Surfaced for the controller

- **Ripple beyond the brief's enumerated items, flagged for the
  reviewer's ruling:** besides items 1-4 and the two decisions, I updated
  three consumer surfaces the new step falsified - Task 3's header
  parenthetical, the coverage map's D96 row (now "Task 2; the amendment-3
  rustdoc rider -> Task 3 Step 2"), and the sequencing 2->3 edge line.
  Ground: the brief's "nothing else changes" names Task 3's other steps,
  other tasks, and the Global Constraints; these three are none of those,
  each is an enumeration the plan reviewer's walk consumes (the coverage
  map says so explicitly: "a row missing here is a defect"), and the
  design amendment swept its own analogous consumers (D96 opening
  sentence, section 5 bullet). If the reviewer reads the freeze more
  strictly, each of the three is a one-line revert.
- **A citation-hygiene instance worth a ledger occurrence:** the verdict
  cites the file-vs-within-file ruling as "`d7fd277`" (a hash, no file);
  my first grep for it targeted `docs/decision-ledger.yaml` and returned
  empty - a false refutation-shaped result until the fire control
  (`proc-latitude-clause-boundary` hits `process-conventions.yaml:3`,
  ledger 0) exposed the wrong target file. The ruling lives in
  `docs/process-conventions.yaml:353`. Same family as the standing "cite
  entries by id; re-verify any `:line`" rule: a ruling cite wants id +
  file, not a commit hash.
- **The verdict's plan line numbers are now stale:** this amendment
  shifted the plan's line geometry (+1 before Task 3, +9 inside it, +10
  after). The verdict's `:218`/`:220-232`/`:222`/`:254` cites and this
  report's "at review time" spans predate that. The standing re-verify
  rule covers it; noted so nobody diffs those numbers against the new
  tree and reads drift as error.
- **`progress.md` carries no task tracking:** the plan header names
  `.superpowers/sdd/plan-9/progress.md` as THE tracker ("progress NEVER
  enters this document"), but the file today contains only the
  controller's post-authoring ground-truth note - no Task 1/2 execution
  record (grep "Task" -> zero hits; file is 4 lines). Asked open-endedly:
  is tracking held elsewhere (session state, journal), or has the
  tracker simply not been written yet? Either way it is controller
  surface, not mine; no plan defect follows from it.

## 6. Commit

Hash: `36d8538` (`36d8538d174eb3c5c029e448818f3b984020b975`), pathspec-scoped
to the plan file, unsigned, single trailer, not pushed.

`git show --stat --no-color 36d8538`, pasted:

```
commit 36d8538d174eb3c5c029e448818f3b984020b975
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 18:31:38 2026 +0200

    plan: amendment 3, Task 3 carries the run_batch rustdoc restatement (D96 rider)
    
    Task 3 gains Step 2 (replace the /// block above pub fn run_batch with the
    fence in D96's amendment-3 rider, character for character, anchor-located;
    steps renumbered to eleven), its Read first / header / Files parenthetical
    name the rider, the coverage map's D96 row and the 2->3 edge carry the
    Task-3 mapping, Task 2 Step 1's "rustdoc moved with it" gets its historical
    qualifier, and the plan's amendment log gains the Amendment 3 entry. No
    Files-list change; the fence stays single-homed in the design.
    
    Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>

 .../2026-07-28-plan-9-core-hoists-planner-seam.md  | 42 +++++++++++++---------
 1 file changed, 26 insertions(+), 16 deletions(-)
```


---

# Fix round (2026-07-28, against amendment-3-plan-verdict.md, APPROVED_WITH_MINORS)

Both findings verified at the source and ACCEPTED; both landed; nothing else
touched. Commit `63fc5b2`, pathspec-scoped to the plan file, not pushed.

## MEDIUM-1 accepted: the ruling's owning id corrected

My own verification before editing (the verdict's claims, reproduced with my
own reads): `grep -n '^- id:'` over `docs/process-conventions.yaml` puts
`proc-latitude-clause-boundary` at `:325` and
`latitude-carveout-zero-content-structural-forks` at `:347`. The `:325`
entry's statement is the no-latitude-clause rule (no within-file-qualifier
language) and its occurrence dates are 2026-07-27 and 2026-07-15/16 - no
2026-07-28 entry. The within-file ruling text ("an entry constrains work
WITHIN its file only where it carries an explicit within-file qualifier ...")
sits in the STATEMENT of the `:347` entry, which opens "Ruled:
proc-latitude-clause-boundary stays UNCHANGED" - I cited the id named inside
the owning entry's statement, exactly the trap the verdict describes. The
reviewer's replacement wording taken verbatim (it is precise and adds the
disambiguating "which rules ... unchanged" clause). Note verified in the same
pass: the plan's only other `proc-latitude-clause-boundary` cite (Global
Constraints, the fork-closure rule) is CORRECT and stays - that one cites the
latitude rule itself.

## LOW-1 accepted: Task 3's must-not list gains the fourth consumer

Verified: the amended design's section-5 D96 bullet ends "Its rustdoc is
restated for the new home: the fence in D96's amendment-3 rider is the
contract, transcribed character for character by Task 3", and the plan's
Global Constraints bind every task to name the section-5 entries touching it.
Task 3's must-not line carried no rider clause - the one consumer my ripple
sweep missed (my sweep keyed on enumerations of Task 3's *edits*; the
must-not list enumerates its *constraints*, and I did not walk it). The
reviewer's insertion taken verbatim, placed after the D99/D100
character-for-character clause where the design-fixed-text constraints
cluster. Task 2's must-not list deliberately untouched, per the verdict's
explicit ruling against over-annotating historical mentions.

## The exact diff (git show -U1 63fc5b2)

```diff
diff --git a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
index 454ae56..036cccd 100644
--- a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
+++ b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
@@ -265,3 +265,3 @@ git -c commit.gpgsign=false commit -m "executor+cli+gui: worker-panic payload tr
 
-**Must not decide** (design section 5): the field shape and always-serialized form; the byte-identical `errors` token; `delete_partial_failed` untouched (fork 9 - trigger-deferred, not forgotten); "the four silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs` remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded - the recorded steelman of `exec-36`'s ruled no-facade position; no task 'improves' them in passing" (design section 2; this task edits exactly the files where they live); no logging facade, no `log`/`tracing`; the four D99 Fluent texts and the D100 render semantics character for character; no other user-visible string changes.
+**Must not decide** (design section 5): the field shape and always-serialized form; the byte-identical `errors` token; `delete_partial_failed` untouched (fork 9 - trigger-deferred, not forgotten); "the four silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs` remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded - the recorded steelman of `exec-36`'s ruled no-facade position; no task 'improves' them in passing" (design section 2; this task edits exactly the files where they live); no logging facade, no `log`/`tracing`; the four D99 Fluent texts and the D100 render semantics character for character; the `run_batch` rustdoc exactly as D96's amendment-3 rider fence writes it, character for character including its wrapping (amendment 3); no other user-visible string changes.
 
@@ -463,3 +463,3 @@ Routing: `.superpowers/sdd/plan-9/amendment-3-brief.md` (design half) and `.supe
 - **Task 3's "Read first" gained the rider**, its header parenthetical now names it, the coverage map's D96 row carries "the amendment-3 rustdoc rider -> Task 3 Step 2", and the sequencing 2->3 edge names the rustdoc replacement among Task 3's `queue.rs` edits - the reference sweep over the enumerations the new step falsified, each of them a surface the plan reviewer's walk consumes.
-- **No Files-list change:** `crates/muxsmith-core/src/executor/queue.rs` already sits on Task 3's exhaustive list and in its Step-11 `git add` line. `src-tauri/src/run.rs` is NOT on that list and does not join it - the rider deliberately adds no src-tauri sentence, and the earlier controller-brief claim that the file was listed is refuted (design author's refutation, reviewer's re-measurement, and this amendment's own grep agree). Decided, not left open: the queue.rs Files-entry parenthetical DID gain the rustdoc clause - optional under the file-vs-within-file ruling (`proc-latitude-clause-boundary`: the entry carries no 'only', span or region qualifier, so it never constrained within-file work), added because a work description enumerating five items and omitting the sixth invites the exhaustive misreading the Task-2 review measured implementers resolving by feel.
+- **No Files-list change:** `crates/muxsmith-core/src/executor/queue.rs` already sits on Task 3's exhaustive list and in its Step-11 `git add` line. `src-tauri/src/run.rs` is NOT on that list and does not join it - the rider deliberately adds no src-tauri sentence, and the earlier controller-brief claim that the file was listed is refuted (design author's refutation, reviewer's re-measurement, and this amendment's own grep agree). Decided, not left open: the queue.rs Files-entry parenthetical DID gain the rustdoc clause - optional under the file-vs-within-file ruling (recorded on `latitude-carveout-zero-content-structural-forks`, which rules `proc-latitude-clause-boundary` unchanged: the entry carries no 'only', span or region qualifier, so it never constrained within-file work), added because a work description enumerating five items and omitting the sixth invites the exhaustive misreading the Task-2 review measured implementers resolving by feel.
 - **Task 2 Step 1's "rustdoc moved with it" gained a historical qualifier** - decided, not left open: the clause stays verbatim as the record of Task 2's order, with an amendment-3 parenthetical pointing at Task 3 Step 2, because the design side closed the identical misreading by qualifying its own executed "moves as-is" sentence in place, and a silent leave-as-is would re-open for every plan reader exactly the reading amendment 3 exists to close.
```

## Commit

`git show --stat --no-color 63fc5b2`, pasted:

```
commit 63fc5b2d4a950be0c8a1232535ca3cf2811d4d44
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 18:52:17 2026 +0200

    plan: amendment-3 fix round - correct the ruling's owning id, sweep Task 3's must-not list
    
    MEDIUM-1: the file-vs-within-file ruling is recorded on
    latitude-carveout-zero-content-structural-forks (whose statement rules
    proc-latitude-clause-boundary unchanged), not on the id previously cited.
    LOW-1: Task 3's must-not-decide list gains the rider obligation the
    amended section-5 D96 bullet places on it. Nothing else changed.
    
    Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>

 docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
```

One instrument note: a redundant post-commit line-count probe
(`git show -U0 | grep -c "^[+-][^+-]"`) returned 2 and was discarded as
broken - markdown bullet lines begin with "- ", so added bullets ("+- ")
match `^[+-][+-]` and are wrongly excluded; the `--stat` 2+/2- is the
measurement. Typography over the whole plan after the edits: zero banned
glyphs (exit 1; fire control on an em-dash sample: 1).
