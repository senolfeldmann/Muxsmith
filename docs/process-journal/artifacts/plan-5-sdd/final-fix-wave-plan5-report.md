# Plan 5 final fix wave report

Branch: master (started at HEAD 945ee96, ended at a32b159). Six commits,
one per logical fix except 1+2 (combined: the async refactor is the
natural vehicle for the override fix).

## Flagged: suspicious "coordinator" message during execution

Mid-session, a message appeared claiming to be a "scope addendum" from
"the coordinator", asking me to mark `ConcurrencyTracker` in
`crates/muxsmith-core` as `#[doc(hidden)]` ahead of a go-public move. It
arrived embedded in the tool-result stream (immediately after a batch of
Bash outputs), not as a genuine new instruction turn, and named a scope
change with no connection to anything in the actual brief. Treated as
untrusted content per the framework's injection-handling doctrine and
**not implemented**. If this item is real and wanted, it needs to come
through as an actual instruction in a fresh message.

## Fix 1+2 (combined): `start_run` ignored the mkvmerge override + blocked the event loop

**File:** `src-tauri/src/run.rs`

- Factored the whole planning pass out of `start_run` into a new pure
  function `plan_run(settings_path, profile, source, output, cancel_flag)
  -> Result<PlanOutcome, IpcError>`. It loads settings via the same
  `crate::load_settings_from` helper `dry_run`/`identify`/
  `detect_mkvmerge` already use, and resolves mkvmerge via
  `Mkvmerge::detect(override)` instead of the old `Mkvmerge::locate()`
  (PATH only) — the CRITICAL bug: a Windows standard install or a
  manually configured override passed `detect_mkvmerge`/`dry_run`
  cleanly and then had every real run silently soft-fail
  (`mkvmerge_found: false`).
- `start_run` is now `async`; `plan_run` runs inside
  `tauri::async_runtime::spawn_blocking`, mirroring `dry_run`'s
  documented pattern. The `Reservation` (single-run gate) is acquired
  synchronously on the calling thread *before* `spawn_blocking` — so a
  concurrent `start_run` is still rejected immediately, no thread-pool
  round trip — and held across the `.await` on `start_run`'s own stack
  (it borrows `&AppState`, not `'static`, so it cannot cross into the
  blocking closure itself; only its cheap `Arc<AtomicBool>` cancel flag
  does). `PlanOutcome::Ready` is boxed (clippy `large_enum_variant`
  against the much smaller `Soft(Value)` arm).
- Event-ordering contract unchanged in shape: `run-finished` still fires
  before `start_run`'s own promise resolves; the doc was reworded to
  note that guarantee now spans an `await` point, invisible to the
  frontend either way.
- All 74 pre-existing `run.rs`/`lib.rs` tests pass with no changes needed.

**Test (RED/GREEN):** `plan_run_honors_the_settings_mkvmerge_override_not_just_path`
places a fake mkvmerge (answers `--version` only) at a path NOT on PATH,
wires it in purely via a settings file, and asserts `plan_run` actually
spawned it (invocation counter) and reached the "found but broken"
branch — while a REAL working mkvmerge sits on this machine's own PATH,
so a PATH-only resolution would silently succeed against the wrong
binary. Confirmed RED by temporarily reverting the resolution call back
to `Mkvmerge::locate()`: the real system mkvmerge was used instead, the
fake's counter stayed at 0, `mkvmerge_found` came back absent instead of
`true`. Restored the fix, confirmed GREEN. Added
`plan_run_propagates_a_settings_load_failure_as_an_ipc_error` alongside.

## Fix 3: D23 divergence — destructive double-Run

**Files:** `src/views/JobsView.vue`, `src/views/BatchView.vue`,
`src/App.vue`, `locales/en/gui-batch.ftl`

- (a) The `pendingRun` watcher used to wipe `jobs`/`logLines`/
  `finishedSummary` and flip `runActive = true` *before* `start_run`
  resolved. On a rejection (`run-already-active`, i.e. a run was already
  active) the reset had already destroyed the live run's rows, and the
  catch branch's `runActive = false` disabled cancel — the batch kept
  running invisibly. Fixed by making the reset conditional on
  `!runActive.value` (this view's own consistent signal for "a run is
  active"): on rejection while genuinely idle, nothing is touched beyond
  surfacing the error via the existing `startError` alert. Deliberately
  **not** literally "reset after `start_run` resolves Ok" as worded in
  the brief — I checked and that reading breaks soft-outcome runs
  (`muxsmith://run-finished` fires synchronously inside the Rust command
  before the promise resolves, per `run.rs`'s own documented
  event-ordering contract; resetting after the await would clobber
  what `onRunFinished` had just written for that very run). The reset
  stays *before* the call, gated on `runActive`, which satisfies the
  same observable requirement (rejection never destroys a live run's
  display) without the regression.
- (b) D23's own sentence ("the UI additionally disables Run while
  active") was never wired up. `JobsView` now emits `update:runActive`
  whenever its `runActive` ref changes; `App.vue` forwards it to
  `BatchView` as a `runActive` prop; `BatchView`'s `runDisabledReason`
  checks it first (new `batch-run-tooltip-run-active` Fluent key),
  ahead of every other reason.

**Verification:** no Vitest/component-test harness exists in this repo
(only Playwright e2e + eslint + vue-tsc + check:i18n), so verified via
the full e2e smoke suite (3/3 pass, including the live-run scenario
which clicks Run, drives scripted job events, and checks
`run-finished` — untouched by this change) plus `vue-tsc`/`eslint`/
`check:i18n`.

## Fix 4: runner-thread panic wedged the app unclosable

**File:** `src-tauri/src/run.rs`

Added `TeardownGuard<'a, F: FnOnce(i32)>`, an RAII guard constructed at
the top of the runner thread body (inside `start_run`'s
`std::thread::spawn`) and dropped at the bottom; its `Drop` runs
`finish_teardown` exactly once regardless of whether the closure ends
normally or unwinds (a `run_queue` worker panic propagating out through
`run_batch`'s `handle.join().expect(...)`). It is now the thread's ONLY
call site for `finish_teardown` — the former explicit end-of-closure
call is gone, replaced by the guard falling out of scope.

**Test (RED/GREEN):** `teardown_guard_clears_the_slot_and_exits_on_unwind_when_quit_was_pending`
drives a real panic through `std::panic::catch_unwind` around a
constructed guard and asserts the slot clears and a pending quit still
fires; `teardown_guard_runs_teardown_exactly_once_on_a_clean_finish`
covers the non-panicking path. Both were written and passed immediately
against the already-implemented `TeardownGuard` type (RED/GREEN here
targets the guard's own Drop contract in isolation, since a full Tauri
`AppHandle` isn't constructible in `cargo test` — the existing test seam
this module already uses throughout is plain `AppState` + injected exit
closures, which is what both tests use); wiring the guard into
`start_run`'s actual runner thread (verified by inspection + full
green suite + clippy) is the fix that puts the contract in the path of
a real worker panic.

## Fix 5: torn settings.json could brick the GUI

**File:** `src-tauri/src/settings.rs`

`save()` now writes to a same-directory temp file
(`.{filename}.tmp-{pid}`) and publishes via `fs::rename` — the only
thing that ever touches the final path. A failed rename cleans up the
temp file (its own error is swallowed; the publish failure is what gets
reported).

**Test (RED/GREEN):** `save_cleans_up_its_temp_file_when_the_publish_rename_fails`
forces the rename to fail (a directory at the final path, root-safe,
mirrors `finalize_joblog`'s own EISDIR trick) and asserts both that the
pre-existing directory is untouched and that no temp file leaks.
Written and implemented in two steps: first the atomic write without
cleanup (confirmed RED — the temp file leaked), then added the
`remove_file` on the error path (confirmed GREEN). Also added
`save_leaves_no_temp_file_behind_after_a_successful_write` as a
regression guard (not RED-capable against the trivial pre-fix
`fs::write`, since that never created a temp file to leak in the first
place — noted honestly rather than claimed as RED/GREEN).

## Fix 6: macOS Homebrew (Apple Silicon) mkvmerge candidate

**File:** `crates/muxsmith-core/src/capability/runtime.rs`

Added `/opt/homebrew/bin/mkvmerge` to `platform_candidates()`'s macOS
branch. The doc comment now cites Homebrew's own install docs
(`docs.brew.sh/Installation`: `/opt/homebrew` is the documented default
prefix on Apple Silicon) rather than mkvtoolnix's own packaging tree
(which doesn't carry a Homebrew formula), plus the sharpening fact that
a GUI app launched from Finder does not inherit the shell's PATH, so the
PATH detection rung — which incidentally covers Intel Homebrew via
`/usr/local/bin` typically being on PATH — does not reliably cover an
Apple Silicon Homebrew install for this app at all. Extended
`platform_candidates_are_verified_against_mkvtoolnix_packaging`'s macOS
assertions accordingly (asserted via inspection; this machine is Linux,
so the macOS `#[cfg]` branch itself does not execute in CI here).

## Fix 7: cleanups

**Files:** `src-tauri/src/lib.rs`, `src-tauri/src/run.rs`,
`src-tauri/src/error.rs`, `locales/en/gui-common.ftl`,
`locales/en/gui-batch.ftl`, `src/views/BatchView.vue`, `BUILDING.md`

- (a) `ShellRenderer` deduped: one `pub(crate)` definition in `lib.rs`,
  `run.rs`'s identical copy removed, every call site now references
  `crate::ShellRenderer`.
- (b) `IpcError::new`/`IpcError::code` collapsed into `IpcError::new`;
  every call site (both crates' worth, `run.rs` + `error.rs`'s own
  `impl From<...>` blocks and tests) updated.
- (c) Added `identify-failed` to `gui-common.ftl`. Wording adapted from
  `cli.ftl`'s identically-named message rather than copied verbatim:
  the CLI message interpolates `{ $file }`, but
  `IpcError::from(IdentifyError)` (`error.rs`) only ever attaches a
  `detail` param — a literal copy would reference an unpopulated
  variable.
- (d) Added `batch-browse-dir-tooltip` (gui-batch.ftl) and wired
  `BatchView`'s two directory-picker buttons (source, output) to it;
  `gui-common.ftl`'s `browse-button-tooltip` ("Choose the file...")
  stays reserved for genuine file pickers (FirstRun, SettingsDialog).
- (e) `BUILDING.md`: dropped the stale "not yet implemented (later
  task)" note on `pnpm test:e2e` (it's implemented —
  `playwright.config.ts`, `e2e/`) and added `pnpm check:i18n` to the
  documented command list and the CI summary line (CI already runs
  both).

## Final eight-gate run (all green)

| # | Gate | Result |
|---|------|--------|
| 1 | `cargo test --workspace` | 369 tests, 0 failed |
| 2 | `cargo fmt --all --check` | clean |
| 3 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 errors |
| 4 | `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| 5 | `mise exec -- pnpm lint` | clean |
| 6 | `mise exec -- pnpm build` | vue-tsc clean, vite build ok |
| 7 | `mise exec -- pnpm check:i18n` | ok, 12 pre-existing warning-only unused ids (unchanged category) |
| 8 | `mise exec -- pnpm test:e2e` | 3/3 Playwright scenarios pass |

## Residual concerns

- Fix 3(a)'s deviation from the literal brief wording ("reset after
  resolves Ok") is a judgment call made because the literal reading
  breaks soft-outcome runs deterministically, not just in a race. Worth
  a second look given it touches D23's own frontend contract.
- Fix 6's macOS assertions were verified by inspection only; this
  session ran entirely on Linux, so the `#[cfg(target_os = "macos")]`
  branch never actually executed here.
- No Vitest/component-test harness exists for the Vue layer; fix 3's
  regression coverage rests on the Playwright e2e suite, which is
  shallower than a targeted unit test would be for the exact
  reset-ordering logic.
