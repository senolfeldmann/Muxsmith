# Task 11 report: Jobs view - live queue, cancel, history, log export (spec 8.2 view 3, D30)

## What was implemented

- `src/views/JobsView.vue` (replaces the T9 placeholder): watches the
  Wave-5 `pending-run` prop; on every non-null value, awaits both
  `muxsmith://job-event` and `muxsmith://run-finished` listener
  registration (via `listen()`, itself async, `Promise.all`'d once and
  cached), then invokes `start_run`, then emits `consumed` in a `finally`
  regardless of outcome. Accumulates a `JobRowData[]` from the live event
  stream (`ensureJobsLength` grows it index-aligned as events/`total_jobs`
  arrive), reconciles every row authoritatively from the `run-finished`
  document's `jobs[]` (the only source for a job that emitted nothing live
  at all -- a batch-cancelled job never dequeued, D16), renders the
  ok/warning/failed/cancelled summary in a polite `aria-live` region plus a
  `joblog_status` note when it is `incomplete`/`unavailable` (that nuance
  exists only in this live event, never in `summary.json`, per the task's
  binding contract point 5). Batch header shows finished/total with a
  cancel-batch button; per-row cancel wired through `JobRow`'s emit.
- `src/components/JobRow.vue`: one `<tr data-testid="job-row" data-index>`
  per job -- output filename (or a "Job N" placeholder pre-`started`),
  state chip (queued/running/ok/warning/failed/cancelled), a native
  `<progress>` (indeterminate -- no `value` attribute -- until a percent is
  known, matching D29's "role=progressbar ... or native `<progress>`"),
  warning-count badge (Fluent plural), and a cancel button disabled only
  once terminal (stays enabled while queued, matching D25's "cancel a
  queued job" semantics).
- `src/components/LiveLog.vue`: `role="log"` region fed by `output` events,
  DOM-capped by the caller (`JobsView`) at 5000 entries, per-job `<select>`
  filter, sticky-bottom auto-scroll (tracks whether the pane is scrolled
  near its bottom edge; a user who scrolls up keeps their position even as
  new lines arrive). One `<style scoped>` block (`max-height`+
  `overflow-y: auto`) -- functionally required for the scroll behavior to
  mean anything, the only styling added anywhere in this task.
- `src/components/RunHistory.vue`: `list_runs` (already newest-first
  server-side) -> select a run -> its `summary.json` jobs -> select a job
  -> `get_job_log` -> copy-to-clipboard (`clipboard-manager`) or save-as
  (`dialog` `save()` + `fs` `writeTextFile()`), the D30 gap closure.
- `src/jobRowState.ts`: shared `JobRowData`/`JobRowLiveState` model and the
  `jobStateKey()` terminal-state -> Fluent-key mapping, used by both
  `JobRow.vue` (live rows) and `RunHistory.vue` (persisted `RunJobEntry`s).
- `locales/en/gui-jobs.ftl`: every UI string plus, in its own section,
  `run.rs`'s five `IpcError` codes keyed directly on the literal code
  string (`run-already-active`, `no-active-run`, `invalid-run-id`,
  `job-log-unavailable`, `job-log-not-found`), mirroring
  `gui-common.ftl`'s established convention for shell-level IPC errors.

## New dependency: `@tauri-apps/plugin-fs` / `tauri-plugin-fs` (exact 2.5.1 pin)

The brief names "save-as (dialog plugin save)" for the D30 log-export gap,
but the dialog plugin's `save()` only returns a user-picked path -- it
never writes bytes, and no `fs` plugin was already present in this repo (no
Cargo dep, no npm dep, no capability entry). No pure-browser API is a
portable substitute across WebView2/WebKitGTK/WKWebView for writing to an
arbitrary local path. Per Tauri's own docs, `save()`'s picked path is
automatically added to the `fs` plugin's scope for that session, which is
exactly the officially documented combo for this pattern -- so only
`fs:allow-write-text-file` was added to `capabilities/default.json`, not
the broader `fs:default`. Also added `dialog:allow-save` (previously only
`dialog:allow-open` was granted). `tauri-plugin-fs` (crate) and
`@tauri-apps/plugin-fs` (npm) are pinned to the identical `2.5.1` (crates.io
`cargo search` and npm's published versions both top out there, and it is
the exact version the crate itself already tracks). `.plugin(tauri_plugin_fs::init())`
registered in `src-tauri/src/lib.rs`'s `run()`, doc comment updated to
explain the pairing rationale. Zero other new dependencies.

## ipc.ts param verification (per task instructions)

Read `src-tauri/src/run.rs` (the real `#[tauri::command]` signatures and
`StartedRun`/`RunMeta`/`JoblogStatus` structs), `queue.rs` (`JobEvent`
serde shape), `job.rs` (`JobOutcome`/`JobState`), and `joblog.rs`
(`JobRecord`, i.e. `get_job_log`'s return shape) directly rather than
trusting memory. **Result: `src/ipc.ts` (already committed by T8) is
correct as-is; no changes were needed.** Specifically verified:

- Tauri's command-arg case convention is camelCase by default (no
  `#[tauri::command(rename_all = ...)]` override in this crate); `getJobLog`
  already sends `{ runId, index }` for `get_job_log(run_id, index)`, and
  every other command's params (`profile`/`source`/`output`/`jobs`/`index`)
  contain no underscores, so no rename was ever at risk there.
- `JobEvent`, `JobOutcome`/`JobState`, `RunJobEntry`/`RunDocument`,
  `RunFinishedEvent`, `RunMeta`, `JobLogRecord`, `StartedRun`, `JoblogStatus`
  all mirror their Rust counterparts field-for-field, including the
  `#[serde(tag = "event", rename_all = "snake_case")]` discriminated union
  shape for `JobEvent`.

## Gate (foreground, all green)

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo test --workspace` (72 passed in `muxsmith-gui`'s own
suite alone, all pre-existing -- this task added no Rust tests, it touches
only plugin registration/capabilities/`Cargo.toml`), `cargo deny check`
(advisories/bans/licenses/sources all ok, including `tauri-plugin-fs`'s new
transitive deps), `mise exec -- pnpm lint`, `mise exec -- pnpm build`
(`vue-tsc --noEmit && vite build`) -- all clean on the final tree.

One real fix needed along the way: `vue-tsc` failed type-checking
`App.vue`'s (T10's, not touched here) still-unwired `<JobsView v-else />`
against a *required* `pendingRun` prop. `withDefaults(..., { pendingRun:
null })` did NOT resolve this (a `vue-tsc` 3.3.7 cross-file inference
quirk with a `T | null`-typed prop plus `withDefaults` -- the consumer-side
inferred type still showed `pendingRun` as non-optional even though the
default was present); switching to a plain optional prop
(`pendingRun?: RunRequest | null`, no `withDefaults`) resolved it cleanly.
Also one `vue/return-in-computed-property` ESLint finding in `JobRow.vue`
(an exhaustive `switch` over `state.kind` without a `default` arm trips
this rule even though TS itself proves it exhaustive); rewritten as an
if-chain with a final unconditional `return`, which both tools accept.

## Self-review

- **Event-contract fidelity**: `ensureListeners()` returns the
  `Promise.all([listen(...), listen(...)])` itself (not a fire-and-forget),
  and the run-dispatch watcher `await`s it before calling `startRun` --
  satisfies "register... AND confirm registered before invoking," not just
  "call `listen()` before `invoke()` textually." `start_run` is never
  called from inside the `run-finished` handler (nothing in this view ever
  calls `start_run` except the `pending-run` watcher). `Finished` events
  arriving with no prior `Started` (per-job-cancel-before-dequeue, D25) are
  handled correctly -- `onJobEvent`'s `finished` case unconditionally
  overwrites `row.state`, no assumption of a prior `started` state. A
  batch-cancelled job that emits nothing live at all is covered by
  `onRunFinished`'s unconditional per-index overwrite from the document.
- **A11y**: native `<progress>` (no redundant explicit ARIA role/
  aria-valuenow layered on top -- this codebase's eslint config carries no
  a11y plugin, so this was a design choice, not a lint requirement), `role=
  "log"` only on the genuinely live pane (deliberately NOT reused on
  `RunHistory`'s static, already-complete log dump -- that one gets a
  visible `<h4>` heading instead, since `role="log"`'s ARIA semantics imply
  ongoing updates a finished record does not have), polite `aria-live` for
  the run summary, every `title`/`aria-label` Fluent-bound, `<table>` with
  `<caption>`+`<th scope="col">` throughout, native `<button>`/`<select>`/
  `<label for>` everywhere, `aria-busy` on the two async action buttons in
  `RunHistory` that can be mid-flight (refresh, save).
- **Zero raw strings**: every visible string and every `title`/
  `aria-label`/`placeholder`/`alt` is Fluent-bound; `@intlify/vue-i18n/
  no-raw-text` (configured for exactly those four attributes) passed clean.
  The one script-side translation (`RunHistory.vue`'s save-dialog filter
  `name`) uses fluent-vue's `useFluent().$t(...)` composable (no prior
  precedent in this codebase for script-side translation, since T9's
  components only ever compute *keys* in script and resolve them via `$t`
  in the template -- checked `fluent-vue`'s shipped `.d.mts` directly to
  confirm the composable's existence and signature rather than guessing).
  File extension tokens (`"log"`, `"txt"`) in the save filter are left
  literal, matching the established, universal convention that filter
  *extensions* (not filter *names*) are technical values, not prose --
  consistent with T10's own brief naming a literal `.yaml/.yml` filter.
- **Data races considered, not just assumed away**: the soft-outcome path
  (`start_run` returns `Ok` with `total_jobs: 0` after synchronously
  emitting `run-finished` from inside the Rust command, before its own
  promise resolves) is handled correctly regardless of which of the two
  async deliveries (the event, or the command's own resolution) the JS
  event loop processes first -- verified by tracing both orderings by
  hand, not just by trusting the happy path. Fixed one real gap this
  surfaced: the section that shows the batch header/table/summary was
  originally gated on `jobs.length > 0 || runActive`, which would have hidden
  the summary/joblog-status line entirely for a zero-job soft outcome (both
  go false once `run-finished` lands); added `|| finishedSummary` to the
  guard so that edge case still shows its ok/warning/failed/cancelled line
  and joblog-unavailable note instead of silently reverting to "no run
  active" and discarding a document the app already received.

## Concerns

- **View-switch tab loses live state.** `App.vue` (T10's file, not
  touched here) currently mounts `BatchView`/`JobsView` with `v-if`/
  `v-else`, so navigating away from the Jobs tab mid-run unmounts
  `JobsView` entirely -- its listeners are torn down (`onUnmounted`) and
  all locally accumulated row/log state is discarded. The run itself keeps
  going on the Rust side regardless (D23's single-run queue does not care
  which view is mounted), but events emitted while unmounted are lost, and
  returning to the tab remounts a fresh `JobsView` with no memory of them
  (rows would show whatever the next live event says, or nothing until
  `run-finished` reconciles from the document). Cancel (batch and per-job)
  is unaffected since it hits the IPC layer directly, independent of view
  state. Not fixable from this file per the wave contract ("the PARALLEL
  task owns ALL App.vue edits") -- would need `v-show` instead of `v-if`,
  or lifting the live-run state above the view switch. Flagging for
  whoever integrates T10+T11 (or Task 13's close-out review).
- The `vue-tsc`/`withDefaults` quirk noted above (plain optional prop used
  instead) is worth a second look once App.vue actually passes
  `pending-run` for real -- if it turns out to be a real upstream bug
  rather than my misuse, worth a note in the plan's own tooling-pins
  section so a later task does not rediscover it from scratch.

## Files changed

- `src/views/JobsView.vue` (replaced placeholder)
- `src/components/JobRow.vue`, `src/components/LiveLog.vue`,
  `src/components/RunHistory.vue` (new)
- `src/jobRowState.ts` (new)
- `locales/en/gui-jobs.ftl` (new)
- `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`,
  `src-tauri/capabilities/default.json` (fs plugin wiring)
- `package.json`, `pnpm-lock.yaml`, `Cargo.lock` (dependency additions)

Commit: `6e763a7` -- `feat(gui): jobs view - live queue, per-job/batch
cancel, history + log export`.

---

## Fix wave (coordinator review verdict: approved + 1 Important, 2 Minor)

Commit `66419f5` -- `fix(gui): per-action export busy flags, live-log
filter reset, fs rationale`, on top of `6e763a7`. Three files changed.

1. **Important -- exportBusy half-applied (`RunHistory.vue`).** Split the
   single `exportBusy` into `copyBusy`/`saveBusy`, one per action; each
   button now binds `:disabled` + `:aria-busy` to its own flag only, and
   each handler self-guards on re-entry (`if (... || copyBusy.value)
   return;` / same for save). Save-as no longer disables the unrelated
   copy button while the native dialog sits open, and copy's async gap is
   guarded against overlapping `writeText` calls. While fixing the same
   async gap, also captured `jobLog.value`/`selectedRunId`/
   `selectedJobIndex` into locals at `saveLog` entry: the previous code
   re-read `jobLog.value` after the `await save(...)` gap, so selecting a
   different job while the dialog stayed open would have written the newly
   loaded log's content under the old job's suggested filename (and TS's
   property-narrowing unsoundness across awaits let it compile). Same
   defect class as the busy-flag issue, in the same function, a few lines
   -- folded in rather than left for another round.

2. **Minor -- LiveLog filter survives into the next run.** `selected`
   resets to the all-jobs option via a non-deep `watch(() => props.jobs)`:
   `JobsView` replaces its `jobs` array reference exactly once per run
   dispatch (`jobs.value = []`) and only mutates that array afterwards
   (`ensureJobsLength` pushes), so the reference change is a precise
   new-run signal -- the minimal mechanism, no extra prop or reset event.
   Comment in the file records why the reference-watch is sufficient.

3. **Minor -- fs+dialog rationale comment.** Extended the existing fs
   paragraph in `lib.rs`'s `run()` doc (next to the plugin registration,
   where the other plugin rationale already lives): chosen over a bespoke
   `#[tauri::command]` write because the plugin route inherits Tauri's own
   "this path came from a real save dialog" trust chain end to end, where
   a custom command would accept any frontend-supplied path and have to
   hand-roll that provenance check itself.

Gate (foreground, final tree): `cargo fmt --all --check`, `cargo clippy
--workspace --all-targets -- -D warnings`, `cargo test --workspace` (all
suites ok, 0 failed), `cargo deny check` (advisories/bans/licenses/sources
ok), `mise exec -- pnpm lint`, `mise exec -- pnpm build` -- all green.

Residual concerns: unchanged from the original report (App.vue's
`v-if`/`v-else` view switch unmounting JobsView mid-run -- T10/close-out
territory; the vue-tsc `withDefaults` quirk note).
