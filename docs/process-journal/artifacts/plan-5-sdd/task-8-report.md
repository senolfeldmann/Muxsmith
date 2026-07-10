# Task 8 report: Shell IPC - run lifecycle (D23)

Branch `plan5-t8`, worktree `.worktrees/plan5-t8`, base `f54cbab`.

## What was implemented

**`src-tauri/src/error.rs`** (new): `IpcError { code: String, params:
HashMap<String, String> }`, `Serialize`, with `IpcError::code(..)` and a
builder-style `.with(key, value)`. Mirrors `muxsmith_core::report::Diagnostic`'s
own `code`+`params` shape (never prose) so every shell-level failure renders
through the same Fluent-catalog mechanism the frontend already needs for core
diagnostics.

**`src-tauri/src/run.rs`** (new), commands exactly per the brief's signatures:

- `start_run(app, state, profile, source, output, jobs) -> Result<StartedRun, IpcError>`
- `cancel_run(state) -> Result<(), IpcError>`
- `cancel_job(state, index) -> Result<(), IpcError>`
- `list_runs(state) -> Result<Vec<RunMeta>, IpcError>`
- `get_job_log(state, run_id, index) -> Result<Value, IpcError>`
- `on_close_requested(window, event)` - wired via `.on_window_event(...)`, calls
  `QueueControl::cancel_all()` on the active run without preventing the close.

`start_run` re-plans through the same core calls the CLI's `run` command uses
(`load::from_file` -> `validate`/`lint` -> `Mkvmerge::locate` ->
`list_languages` -> `plan_batch` -> `filter_map(|f| f.plan.as_ref())` into
`JobSpec`s via `command()`), reserves `AppState`'s single-run slot, and hands
the real work to a detached `std::thread`. The factored, directly-testable
core is `run_batch(state, specs, spawner: &(dyn Spawn + Sync), opts, ctl,
logger, on_event)`: runs `run_queue` on its own scoped worker thread while
draining the event channel on the calling stack, tee-ing every `JobEvent`
into `logger.on_event` and the caller's sink, then clears `state.active` and
hands the outcomes + still-open logger back so the caller can build
`run_document` and only then call `RunLogger::finish` on it.

`AppState { active: Mutex<Option<ActiveRun>> }` (managed via
`.manage(run::AppState::default())`); `ActiveRun` carries only the
`Arc<QueueControl>` the cancel paths and window-close handler need.

## Design decisions (flagging for controller/T7 reconciliation)

1. **Shared-shape surfaces with T7.** `IpcError`, `AppState`, and the general
   "managed state + thin command wrapper" pattern are surfaces both T7's brief
   and T8's independently require with the *same* contract. T7's own worktree
   (`plan5-t7`) had no files beyond the untouched scaffold at the time T8 was
   implemented, so there was nothing to align against. I built `error.rs` and
   `AppState` to the letter of both briefs' descriptions; when T7 lands, its
   own `error.rs`/`AppState` additions (settings fields) will need a manual
   merge (progress.md already anticipates "T7 then T8" order for exactly
   `lib.rs`'s `invoke_handler`; the same applies to these two files, which is
   new information the merge step should know about).
2. **Planning "soft" outcomes stay `Ok`, not `IpcError`.** The brief only
   names one IPC-level error for `start_run` (`"run-already-active"`) and
   requires the terminal event to carry "the full `run_document`". So every
   branch where planning itself succeeds but produces nothing to run --
   profile-load failure, mkvmerge missing/unqueryable, or a batch that plans
   zero jobs -- returns `Ok(StartedRun { total_jobs: 0, run_dir: None, .. })`
   and synchronously emits `muxsmith://run-finished` with the same
   `batch_document`/`config_only_document`-based `run_document` shape the CLI
   already builds for its own equivalent branches (D15's "json callers always
   get a complete document" contract, ported to the GUI's event instead of
   stdout). `IpcError` is reserved for genuine IPC-protocol failures: the
   single-run conflict, an inactive `cancel_run`/`cancel_job` (`"no-active-run"`,
   not in the brief's literal list but needed for the state machine to be
   testable/symmetric), an unsafe `run_id` (`"invalid-run-id"`), and an
   unreadable/missing job log (`"job-log-unavailable"` /
   `"job-log-not-found"`). In practice `start_run` is only reachable after
   T10's Batch view has shown a clean `dry_run` and gated the Run button, so
   these branches are defensive, not the expected path -- worth a controller
   sanity check that this reading matches intent for T11's Jobs view.
3. **`ShellRenderer`** (private, in `run.rs`): a minimal `DiagnosticRenderer`
   whose `diagnostic()` returns the diagnostic's own `code.key()` instead of
   localized prose, since `batch_document`/`config_only_document` require a
   renderer and the shell's real rendering happens client-side from
   `code`+`params` (T9/T10's Fluent catalog never reads the `"rendered"`
   field for GUI documents). This is the one piece of "orchestration-adjacent"
   code invented rather than reused verbatim from the CLI; flagging per the
   brief's own escape clause since it's minimal and necessary, not a shortcut.
4. **`RunMeta::started_at`** is hand-parsed from `run_id`'s own
   `"YYYYMMDD-HHMMSSZ"` prefix (first 16 bytes, still present on a
   collision-suffixed dir name) rather than read out of `summary.json`,
   because `report::json::run_document` carries no run-level timestamp field
   and extending it is outside this task's `Files` list (and would ripple
   into the CLI's own golden JSON contract). Manual byte-slice parsing, not
   the `time` crate, since `muxsmith-core`'s `time` dependency only pins the
   `formatting` feature (not `parsing`) and this task has no reason to widen
   that.
5. **`list_runs` sorts newest-first** (`run_id` descending, which is also
   chronological) even though the given signature doesn't mandate it, since
   T11's brief already says "`list_runs` (newest first)" and it costs nothing
   to guarantee server-side rather than push onto every frontend caller.
6. `jobs: Option<usize>` defaults to `1` (matches the system-wide
   `default_jobs` convention T7's `AppSettings` establishes); `fail_fast` is
   hardcoded `false` since the given `start_run` signature has no such
   parameter.
7. `tauri.conf.json`'s `csp: null` was left untouched -- progress.md's
   controller note ("set a real CSP when IPC surface lands") names it for
   "T7/T8" collectively, not as a mandated action inside either task's own
   `Files` list; leaving it for whichever of the two lands last, or a
   dedicated follow-up.
8. `AppState.active` is a plain private field (not `pub`), since nothing
   outside `run.rs` needs direct access (`lib.rs` only touches
   `AppState::default()` and `run::on_close_requested`). The brief's sketch
   (`AppState { active: Mutex<Option<ActiveRun>>, ... }`) reads as
   descriptive, not a literal visibility mandate; happy to widen it if T7's
   settings need to reach it directly.
9. `capabilities/default.json` was left unchanged: confirmed via Tauri 2 docs
   (context7, `/websites/v2_tauri_app`, "Capabilities" page) that commands
   registered through `invoke_handler`/`generate_handler!` are allowed to
   every window by default -- capability grants are for *plugin* commands
   (dialog, clipboard-manager), which this task adds none of.

## TDD

Test list was derived from the brief's Step 1 wording before writing any
implementation: (a) events arrive `Started -> Output -> Finished`, (b) second
start rejected while active, (c) active flag clears after finish, (d) joblog
dir populated. Because these four behaviors and their surrounding pure
helpers (`lock_for_start`, `run_batch`, `finalize_joblog`, `list_runs_in`,
`get_job_log_in`, `started_at_from_run_id`, `valid_run_id`,
`do_cancel_run`/`do_cancel_job`) are tightly coupled, tests and
implementation were written together as one unit rather than as a strict
per-test RED-commit cycle -- I did not observe an intermediate state where a
test compiled and failed on a real assertion before the implementation
existed. What I *do* have as evidence: the first `cargo test` run failed to
compile (`ActiveRun`/`MutexGuard` missing `Debug` for one test's
`unwrap_err()`), fixed by restructuring that one assertion to a `match`
instead of deriving `Debug` through `Arc<QueueControl>` (which cannot derive
`Debug` since `QueueControl` doesn't); the next run was 27/27 green. All four
brief-named behaviors, plus the additional helpers listed above, have
dedicated tests (37 total across `error.rs` + `run.rs`, see gate output
below). `finalize_joblog`'s write-failure case uses the same root-safe
"directory at the target path" trick `executor::job`'s own
`delete_partial_failure_surfaces_into_errors` test uses (a chmod-based test
would silently pass-for-the-wrong-reason if ever run as root).

## Gate (foreground, all green)

- `cargo test --workspace`: every suite `ok`, including the new
  `error::tests::*` (3) and `run::tests::*` (27); grepped full output for
  `FAILED|error\[` - zero hits beyond expected `test result: ok` lines.
- `cargo fmt --all --check`: clean (one auto-format pass applied first, then
  verified clean).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`
  (pre-existing multiple-version warnings from Tauri's own transitive tree,
  unrelated to this change - no new external crate versions introduced,
  confirmed via the `Cargo.lock` diff: only two new dependency *edges*,
  `muxsmith-core` and `tempfile`, both already present elsewhere in the
  workspace at the same versions).
- `mise exec -- pnpm lint`: clean (`eslint .`, no findings).
- `mise exec -- pnpm build`: `vue-tsc --noEmit && vite build` succeeded.

## Self-review / residual concerns

- `start_run`'s own body (profile load -> validate/lint -> mkvmerge lookup ->
  plan_batch -> spec-building -> thread spawn) is not independently unit
  tested beyond the pure helpers it calls into (`lock_for_start`, `run_batch`,
  `finalize_joblog`) and core's own already-tested functions
  (`load::from_file`, `plan_batch`, etc.). This matches the brief's explicit
  "the `#[tauri::command]` wrappers stay thin and untested," but `start_run`'s
  wrapper is not *trivially* thin (it mirrors the CLI's multi-branch
  orchestration inline, same as the CLI's own `run()` has no direct unit
  test for its top-level body either -- only its extracted pieces
  (`MilestoneState`, `job_exit_code`, `render_summary`) are unit-tested).
  Flagging this boundary explicitly rather than silently deciding it's fine.
- No integration-level test drives an actual Tauri app/webview for this
  surface (no `tauri::test` mock builder is set up in this repo yet); T12's
  Playwright smoke (mocked IPC) is the plan's designated integration layer
  for the frontend side, and this task's `Files`/`Test` list didn't call for
  a Rust-side Tauri integration harness.
- `on_close_requested` is untestable in isolation without a real Tauri
  `Window`/`WindowEvent::CloseRequested` (its `CloseRequestApi` field has no
  public constructor); its logic is a 3-line dispatch into already-tested
  `QueueControl::cancel_all`, so this is an accepted, unavoidable gap rather
  than a deferred one.
- `git status` in the worktree: `Cargo.lock` (deps only), `src-tauri/Cargo.toml`,
  `src-tauri/src/lib.rs` modified; `src-tauri/src/error.rs`,
  `src-tauri/src/run.rs` new. No other files touched.

## Commit

`plan5-t8`, one commit: `feat(gui): run lifecycle IPC - start/cancel/events/history`

---

# Fix-round report (review follow-up on b820370)

Commit: `20abae1` `fix(gui): reserve run slot without locking across planning; harden run_id; document event ordering`

## Fix 1 (Important): lock held across planning blocked the event loop

- `AppState.active` is now `Mutex<Option<RunSlot>>` with
  `RunSlot::{Reserved(Arc<AtomicBool>), Running(ActiveRun)}`. Both variants
  count as "active" for the D23 single-run gate.
- New RAII `Reservation`: `acquire` check-and-inserts `Reserved` and releases
  the mutex immediately (the lock is held only for the pointer swap, never
  across planning); `Drop` without `commit` clears the slot -- ONE mechanism
  covering every soft-outcome early return (all four: profile-load failure,
  mkvmerge locate failure, list_languages failure, empty specs) plus a
  mid-planning panic, so no path can leak the reservation into a permanent
  `run-already-active`. `commit` promotes to `Running` and runs before the
  queue thread spawns, so the thread's end-of-run clear can never race ahead
  of the install.
- Beyond the letter of the assignment: a `cancel_run` or window close landing
  in the planning window is now *honored*, not just tolerated. The
  reservation's flag is the very flag `start_run` hands to
  `QueueControl::new` as the queue's batch flag, so a queue that
  materializes after such a cancel is born already-cancelled (every job
  finishes `Cancelled` without spawning, D16 pre-dequeue semantics).
  `do_cancel_job` during `Reserved` is an accepted no-op (documented): job
  indices only become known to the frontend once `start_run` returns, and by
  then the slot is `Running`; core's own out-of-range `cancel_job` no-op is
  the mirrored precedent.
- `on_close_requested` handles both variants (`Reserved` -> set flag,
  `Running` -> `cancel_all`), each O(1) under the lock.
- Tests (new): `second_start_is_rejected_while_the_first_is_mid_planning`
  (the reservation stands in for the planning pass -- `start_run`'s body
  between acquire and commit/drop is a straight-line sequence with no other
  slot access, so a held reservation IS the mid-planning state; no fake
  planning fixture exists since planning runs against the live mkvmerge and
  adding a seam for it is outside this task's files),
  `reservation_clears_on_a_soft_outcome`,
  `second_start_is_rejected_while_a_queue_is_running_too`,
  `commit_promotes_the_reservation_and_run_batch_clears_it` (full lifecycle
  in slot terms), `cancel_run_during_planning_reaches_the_later_queue`
  (asserts the later-built `QueueControl` already reports every job
  cancelled), `cancel_job_during_planning_is_an_accepted_no_op`. The two
  `lock_for_start` tests were removed with the function they tested.

## Fix 2 (Important): event-ordering contract documented

- `start_run`'s rustdoc gained an explicit "Event-ordering contract
  (frontend requirement)" section: listeners for BOTH
  `muxsmith://job-event` and `muxsmith://run-finished` MUST be registered
  before invoking `start_run`, because soft-outcome branches emit
  `run-finished` synchronously before the command's `Result` returns (and a
  real run's first job events can race a late subscription the same way).
- `finish_without_queue` and `emit_run_finished` each carry a matching note
  identifying themselves as the emit sites behind that contract. No behavior
  change.

## Fix 3: test-count reconciliation

The original report's "37 total across error.rs + run.rs" was wrong; the
correct counts at b820370 were **27** (error.rs 3 + run.rs 24). After this
fix round the counts are **31** (error.rs 3 + run.rs 28): removed 2
(`lock_for_start_*`), added 6 (the reservation/cancel-window tests listed
under Fix 1). The `valid_run_id` colon cases extended an existing test
rather than adding one. These numbers are from the actual
`cargo test -p muxsmith-gui --lib` run output ("running 31 tests"), not
estimated.

## Fix 4 (Minor): valid_run_id rejects ':'

- One character added to the check chain; doc updated with the Windows
  drive-prefix rationale (`PathBuf::join("C:...")` replaces the root on
  Windows; `':'` never occurs in a `make_run_id` name).
- Test extended: `valid_run_id_rejects_traversal_separators_and_drive_prefixes`
  now also covers `"C:"`, `"C:x"`, `"a:b"`.

## Gate (foreground, all green)

- Covering tests: `cargo test -p muxsmith-gui --lib` = 31/31 ok.
- `cargo test --workspace`: 32 test binaries, all `test result: ok`, zero
  `FAILED`/`error[` hits.
- `cargo fmt --all --check`: clean. `cargo clippy --workspace --all-targets
  -- -D warnings`: clean. `cargo deny check`: advisories/bans/licenses/
  sources ok.
- `mise exec -- pnpm lint`: clean. `mise exec -- pnpm build`: vue-tsc +
  vite build succeeded.

## Residual concerns

- The mid-planning rejection test exercises the reservation primitive, not
  `start_run` end-to-end (no way to park `start_run` inside `plan_batch`
  without adding a planning seam this task's file list does not include;
  the primitive plus the straight-line body between acquire and
  commit/drop is the full invariant surface). If the controller wants an
  end-to-end version, `start_run` would need an injectable planner or
  mkvmerge handle -- a deliberate interface change to raise at merge time,
  not something to smuggle in during a fix round.
- The fourth review item (close-without-wait product risk: the window
  closes while kills are still in flight) remains with the project owner
  per the coordinator's message; nothing in this round changes that
  behavior.

---

# D31 scope-extension report (close-with-active-run, on top of 20abae1)

Commit: `e7cb673` `feat(gui): D31 close-with-active-run confirmation - abort dialog + quit-after-finished`

## What was implemented

**`on_close_requested` rewritten per D31** (supersedes D23's bare
`cancel_all`):

- No active run: returns without touching `api` -- the window closes
  normally (unchanged path).
- Active run (`Reserved` OR `Running`): `api.prevent_close()` (API shape
  verified via context7 against tauri 2.9.2 docs: `WindowEvent::
  CloseRequested { api, .. }`, `CloseRequestApi::prevent_close`), then a
  native confirmation dialog via tauri-plugin-dialog's Rust API in its
  NON-BLOCKING callback form (`app.dialog().message(..).title(..)
  .kind(Warning).buttons(OkCancelCustom(..)).show(callback)` -- verified
  via context7 `/websites/v2_tauri_app` dialog-plugin page; `show` only
  schedules, the event loop is never blocked).
- Dialog Yes -> `abort_and_quit`: sets `AppState.quit_after_finished`,
  cancels the slot's occupant (`Running` -> `cancel_all`; `Reserved` ->
  the reservation's cancel flag, which is the future queue's own batch
  flag, so the abort is carried into a queue that materializes later);
  if the slot is already empty (run tore down while the dialog was open),
  exits immediately. Dialog No -> nothing.

**Quit-after-finished plumbing (exactly-once):**

- `quit_if_requested(state, exit)` consumes the flag via atomic `swap`, so
  no interleaving of the three completion paths can exit twice.
- `finish_teardown(state, exit)`: the runner thread's new final step --
  clears the slot, then `quit_if_requested`. Ordering change that matters:
  the slot clear MOVED out of `run_batch` to strictly after
  `finalize_joblog` + `emit_run_finished`, so "slot empty" now means
  "teardown fully complete (kills landed, summary.json written)" -- the
  invariant `abort_and_quit`'s direct-exit arm relies on, and exactly
  D31's rationale (never lose the joblog to an early exit). Documented
  tradeoff: a frontend invoking `start_run` synchronously from its
  run-finished handler can now race the clear by microseconds and get a
  spurious `run-already-active` (recoverable by retry; the reverse
  ordering risked unrecoverable joblog loss).
- `finish_without_queue` (all four soft-outcome paths) runs
  `quit_if_requested` after its emit: the coordinator-mandated
  Reserved-but-never-Running case (Yes during planning, then planning
  produces nothing) still exits.
- `Reservation::acquire` discards a stale quit request: a new run
  supersedes a pending quit, so an orphaned flag (e.g. mid-callback
  panic) can never silently exit the app after the next unrelated run.

## Dialog-strings route (per the assignment's either/or)

Route taken: **`.ftl` as single source of truth, embedded at build time.**
Four new entries in `locales/en/gui-common.ftl`
(`close-abort-title/-message/-confirm/-dismiss`), wording from D31's named
reference (mkvtoolnix-gui `main_window.cpp` `beforeCloseCheckRunningJobs`,
read from the local source tree: title "Abort running jobs", text "There
is currently a job running. Do you really want to abort all currently
running jobs?" extended with "and quit" since our dialog also quits,
buttons "Abort jobs and quit" / "Cancel" -- Qt's "&" accelerator dropped
as meaningless in native dialogs). Rust side: `include_str!` +
`ftl_message`, a deliberate line-parser (`key = value`), NOT a Fluent
parser -- the shell consumes four single-line messages and a full Fluent
stack would duplicate T9's frontend loader. The constraint is pinned three
ways: a comment block in the `.ftl`, `ftl_message`'s rustdoc, and the
`close_abort_strings_resolve_from_the_ftl_catalog` test (each key must
resolve to a non-key value; the title's exact wording is asserted so a
multiline/attribute edit fails the build instead of shipping a raw key as
the dialog title). A missing key degrades to the key itself -- a stable
code, no panic, still prose-free. Why this route over the documented
exception: it satisfies the prose-free rule outright, costs ~15 lines,
and leaves T9 nothing to reconcile (the entries are already in the file
its loader reads).

## Plugin registration / capabilities check (as assigned)

- `tauri_plugin_dialog::init()` was already registered in `lib.rs`'s
  builder (T4 scaffold) -- nothing to add.
- `capabilities/default.json` unchanged: capability permissions gate the
  *frontend's* IPC access to plugin commands; the shell's Rust-side
  `app.dialog()` call never crosses the IPC permission layer (confirmed
  via context7: capabilities control what windows/webviews may invoke).
  `lib.rs`'s doc comment now states this explicitly so the next reader
  does not "fix" a nonexistent gap.

## Tests

`muxsmith-gui` lib: **39/39** (error.rs 3 + run.rs 36; was 31 after the
fix round). New/changed:

- `close_decision_lets_an_idle_window_close_normally` (also asserts the
  quit machinery stays disengaged -- coordinator's "no-active-run close
  paths never engage the machinery"),
  `close_decision_confirms_while_planning_and_while_running`.
- `quit_flag_plus_teardown_completion_exits_exactly_once` (injected exit
  closure counts invocations: none before teardown, exactly one on
  completion, none on a second consume -- the coordinator's minimum test),
  `abort_and_quit_exits_immediately_when_the_run_already_tore_down`,
  `abort_and_quit_during_planning_exits_after_a_soft_outcome` (the
  mandated Reserved case), `a_new_reservation_discards_a_stale_quit_request`.
- `finish_teardown_clears_the_slot_without_exiting_when_no_quit_is_pending`
  (replaces `run_batch_clears_the_active_slot_when_done`; the clear moved),
  `commit_promotes_the_reservation_and_finish_teardown_clears_it`
  (renamed/rewired lifecycle test).
- `close_abort_strings_resolve_from_the_ftl_catalog`,
  `ftl_message_falls_back_to_the_key_and_never_prefix_matches`.
- The dialog itself and `on_close_requested`'s Tauri wiring stay untested
  (UI shell, per the assignment; `CloseRequestApi` has no public
  constructor).

## Gate (foreground, all green)

`cargo test --workspace` 32 suites ok (gui lib 39/39); `cargo fmt --all
--check` clean; `cargo clippy --workspace --all-targets -- -D warnings`
clean; `cargo deny check` advisories/bans/licenses/sources ok;
`mise exec -- pnpm lint` clean; `mise exec -- pnpm build` ok.

## Residual concerns

- A second `CloseRequested` while the dialog is already open (user hits
  the close button again) spawns a second dialog; both Yes handlers are
  idempotent (flag store + idempotent cancels, exit still exactly-once via
  swap), so this is cosmetic stacking, not a correctness issue. Guarding
  it would need dialog-open bookkeeping for zero behavioral gain; noted
  rather than built.
- `quit_after_finished` deliberately does NOT survive a new
  `Reservation::acquire`; if the product ever wants "quit even if the user
  starts something new mid-quit", that is a different (and stranger)
  semantic than mkvtoolnix's, which also quits-after-abort on the aborted
  jobs only.
- The documented microsecond `run-already-active` window on
  instant-restart-from-run-finished (see finish_teardown above) is the
  accepted cost of the joblog-safety ordering; T11's Jobs view should
  simply not fire `start_run` synchronously from the run-finished handler
  (it has no reason to).
