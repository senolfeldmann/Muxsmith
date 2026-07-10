# Plan 5: Tauri 2 GUI run path Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** The GUI run path per D22: a Tauri 2 shell + Vue 3 frontend with a batch view (profile pick, dry-run preview, suggestions show+copy) and a jobs view (live progress, per-job/batch cancel, history), persisted JSON job logs for CLI and GUI (D26), platform mkvmerge detection with first-run guidance (D28), and the core extensions these need (raw-output events D24, per-job cancel D25, report-JSON hoist).

**Architecture:** Core gains three contained extensions (raw `Output` event variant; `QueueControl` with per-job cancel keyed by job index; `executor::joblog` RunLogger) plus a hoist of the batch/run JSON document assembly from the CLI into `core::report` so both surfaces render identical structures (spec 7 DRY rule). `src-tauri` is a thin shell: commands wrap core calls, `start_run` spawns `run_queue` on a std thread and re-emits each `JobEvent` as a Tauri event (the CLI's drain pattern, run.rs:210-235). The Vue frontend performs zero semantic validation and renders core's code+params diagnostics through Fluent.

**Tech Stack:** Rust (edition 2024), Tauri 2 (+ plugin-dialog, plugin-clipboard-manager), Vue 3 + TypeScript + Vite, pnpm via corepack, fluent-vue over @fluent/bundle, eslint 9 (vue + intlify no-raw-text), Playwright (+ @axe-core/playwright), new core deps `dirs` + `time`.

## Global Constraints

- Per-commit gate, run all four, do NOT skip fmt: `cargo test --workspace` AND `cargo fmt --all --check` AND `cargo clippy --workspace --all-targets -- -D warnings` AND `cargo deny check`. From Task 4 on, additionally: `pnpm lint` AND `pnpm build`.
- ALL cargo/pnpm commands run in the FOREGROUND (a Plan-4 implementer stalled twice on background-run + Monitor waits). Subagent briefs must repeat this.
- Core emits no user-facing prose: codes + params only; human text in `locales/en/*.ftl`. Accepted exception: third-party error text passed through as a `detail`-style param (I/O, serde).
- `#![deny(missing_docs)]` on lib crates: every new public item needs a doc comment.
- Frontend: NO hardcoded user-facing strings, including `aria-label`s - every string comes from the Fluent catalogs (D29 a11y block: semantic HTML first, accessible names everywhere, `role="log"`/`role="progressbar"` live regions, `getByRole` as primary Playwright locator, `data-testid` fallback). Spec 8.3 baseline applies to every view: non-obvious controls carry a Fluent-sourced tooltip (`title` + `aria-describedby` where richer); the help-mode sidebar itself is Plan 6.
- Commits authorized and UNSIGNED: `git -c commit.gpgsign=false commit ...`, trailer final line `Co-Authored-By: <session model> <noreply@anthropic.com>`. Pushes logged in `gh-log.md`.
- Typography: ASCII punctuation only; umlauts and `Ş` intact.
- Confirm mkvmerge behavior by RUNNING the binary (v100 installed), never from memory (SI-3). Gated tests use the self-skip idiom (`mkvmerge_runtime.rs:7-16`).
- Design memo: `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md` (D22-D30, FINAL). Spec wins on conflict.
- Deferred, do NOT build: NDJSON `--json-events`, `--fail-fast=now`, log pruning, help-mode sidebar, profile editor, one-click apply-suggestion, packaging CI (docs/IDEAS.md items stay shelved).

## Dependency graph and execution waves (SI-1)

```
Wave 0 (GATE, Şenol):        T0 environment prep (local dnf + corepack; NOT a subagent task)
Wave 1 (PARALLEL worktrees): T1 core Output events    T2 core report hoist    T3 core detect+floor    T4 scaffold+CI
Wave 2 (serial, both touch queue/job/run.rs):  T5 per-job cancel [T1]  then  T6 joblog+CLI wiring [T1,T2,T5]
Wave 3 (PARALLEL worktrees): T7 shell read-only IPC [T2,T3,T4]    T8 shell run lifecycle [T4,T5,T6]
Wave 4:                      T9 frontend app shell + first-run + settings [T7]
Wave 5 (PARALLEL worktrees): T10 batch view [T7,T9]    T11 jobs view [T8,T9]
Wave 6:                      T12 Playwright smoke + i18n gate + CI finish [T9,T10,T11]
Wave 7:                      T13 whole-branch review + journal + HANDOFF (controller)
```

Conflict notes: T5 and T6 both edit `queue.rs`/`run.rs` - strictly serial. T7 and T8 both append to the `invoke_handler` list in `src-tauri/src/lib.rs` - parallel OK, trivial merge. T10/T11 own separate view files and separate `.ftl` files (`gui-batch.ftl` vs `gui-jobs.ftl`) - conflict-free. After each worktree passes its task review, merge to master sequentially, re-running the full gate per merge.

---

### Task 0: Environment prep (GATE - requires Şenol's explicit go; system changes)

**Files:** none in repo.

- [ ] **Step 1: Şenol confirms/runs local installs (Fedora):**

```bash
sudo dnf install webkit2gtk4.1-devel librsvg2-devel libappindicator-gtk3-devel nodejs
corepack enable
```

- [ ] **Step 2: Verify:** `pkg-config --modversion webkit2gtk-4.1` prints a version; `node --version` >= 20; `corepack pnpm --version` resolves. Do not proceed to Task 4 before this passes locally.

---

### Task 1: Raw output events (D24) + JobEvent serde golden test

**Files:**
- Modify: `crates/muxsmith-core/src/executor/job.rs` (line loop + `JobProgress`), `crates/muxsmith-core/src/executor/queue.rs` (`JobEvent` + worker `on_progress` mapping)
- Test: `crates/muxsmith-core/tests/executor_events.rs` (new)

**Interfaces:**
- Produces: `JobProgress::OutputLine(String)`; `JobEvent::Output { index: usize, line: String }` serializing as `{"event":"output","index":0,"line":"..."}`. Semantics: every line mkvmerge writes that is NOT a `#GUI#progress` tick is emitted verbatim (tags included); tagged warning/error lines ADDITIONALLY keep their existing tag-stripped `WarningLine`/`ErrorLine` emission. Consumed by T6 (persistence) and T8/T11 (live log).

- [ ] **Step 1: Failing golden test.** In `executor_events.rs`, one test asserting exact `serde_json::to_string` output for ALL `JobEvent` variants including the new one (the GUI now consumes the stream; the wire shape is contract):

```rust
let ev = JobEvent::Output { index, line: "#GUI#warning hello".into() };
assert_eq!(serde_json::to_string(&ev).unwrap(),
    r#"{"event":"output","index":0,"line":"#GUI#warning hello"}"#);
```

Plus `started`/`progress`/`warning`/`error`/`finished` (finished embeds the full `JobOutcome` object: `state`, `exit_code`, `warnings`, `errors`, `duration_ms`). Run: `cargo test -p muxsmith-core --test executor_events` - FAILS (variant missing).

- [ ] **Step 2: Failing behavior test** (same file, `FakeSpawner::script`): a script with a progress tick, a plain line, and a tagged warning yields `OutputLine` for the plain AND the tagged line (verbatim), `WarningLine` for the tagged one, NO `OutputLine` for the tick.
- [ ] **Step 3: Implement**: add the variants; in `run_job`'s line loop emit `OutputLine(line.clone())` for every non-tick line before the existing tag handling; map in the worker's `on_progress` closure (queue.rs:146-155) to `JobEvent::Output`.
- [ ] **Step 4: Full gate green.** Existing CLI milestone rendering must ignore `Output` events (check `MilestoneState::render` handles the new variant via a wildcard or explicit no-op arm).
- [ ] **Step 5: Commit** `feat(core): raw output-line job events (D24) + JobEvent wire golden test`

---

### Task 2: Hoist report JSON assembly into core (`report::json`)

**Files:**
- Create: `crates/muxsmith-core/src/report/json.rs` (turn `report.rs` into `report/mod.rs` if needed)
- Modify: `crates/muxsmith-cli/src/commands/dry_run.rs` (`batch_json`, `config_only_json` move out), `crates/muxsmith-cli/src/commands/run.rs` (`run_json_document` moves out)
- Test: existing CLI snapshot/JSON tests are the harness; add `crates/muxsmith-core/tests/report_json.rs` for direct shape assertions

**Interfaces:**
- Produces: `report::json::{config_only_document(..), batch_document(..), run_document(..)}` with signatures lifted 1:1 from the current CLI functions (same inputs, same `serde_json::Value` output, byte-identical documents). `run_document` keeps injecting per-job `"index"` and `"output"` into each serialized `JobOutcome`. Consumed by T6 (summary.json), T7/T8 (IPC returns).
- Rationale (spec 7): "CLI and GUI render the same diagnostic and report structures; neither owns logic."

- [ ] **Step 1:** Move the three functions to core verbatim (public, documented); CLI call sites delegate. Add a rustdoc note on `run_document`: `jobs[].index` indexes the QUEUE (specs slice), not the source-file list (HANDOFF backlog item).
- [ ] **Step 2:** `cargo test --workspace` - the existing dry-run/run JSON tests prove byte-identical output. Add one direct core test asserting `run_document` field presence (`jobs[].{index,output,state}`, `summary.{ok,warning,failed,cancelled}`).
- [ ] **Step 3: Commit** `refactor(core): hoist batch/config/run JSON documents into report::json`

---

### Task 3: mkvmerge detection ladder + version floor (D28)

**Files:**
- Modify: `crates/muxsmith-core/src/capability/runtime.rs`
- Test: `crates/muxsmith-core/tests/mkvmerge_runtime.rs` (append)

**Interfaces:**
- Produces: `Mkvmerge::detect(override_path: Option<&Path>) -> Result<Mkvmerge, RuntimeError>` (ladder: override -> PATH via `locate()` -> platform candidates, each probed with `--version`); `fn platform_candidates() -> Vec<PathBuf>` (private, but its list unit-tested via a seam); `Mkvmerge::version_pair(&self) -> Result<(u64, u64), RuntimeError>` parsing `"mkvmerge vNN.N.N ..."`; `pub const MIN_SUPPORTED: (u64, u64)`; `RuntimeError::TooOld { found: String, minimum: String }` (new variant). Consumed by T7's `detect_mkvmerge` command.

- [ ] **Step 1: Empirically fix the floor (SI-3).** The capability table is generated from identification schema v20; find which mkvtoolnix release introduced format version 20: `grep -rn "identification_format_version" ~/Downloads/mkvtoolnix/NEWS.md | head` (and the src if NEWS is ambiguous). `MIN_SUPPORTED` = that release. Record the evidence in the const's doc comment.
- [ ] **Step 2: Fix the candidate list from authority, not memory.** Windows: `%ProgramFiles%\MkvToolNix\mkvmerge.exe` (+ `(x86)`); macOS: `/Applications/MKVToolNix-*.app/Contents/MacOS/mkvmerge` (glob), `/opt/homebrew/bin/mkvmerge`, `/usr/local/bin/mkvmerge`; Linux: `/usr/bin/mkvmerge`, `/usr/local/bin/mkvmerge`, `/var/lib/flatpak/exports/bin/org.bunkus.mkvtoolnix-gui`? - VERIFY each against mkvtoolnix's own packaging (windows installer NSIS script + macOS bundle layout in ~/Downloads/mkvtoolnix/packaging/) and drop any location the packaging does not actually use. Cite file paths in the doc comment.
- [ ] **Step 3: TDD.** Failing tests: `version_pair` parses `"mkvmerge v100.0.0 ('Message') 64-bit"` -> `(100, 0)`; ladder prefers override over PATH (probe a tempdir fake script on Unix; gate Windows-only aspects); `TooOld` surfaces found+minimum. Implement minimal. Gated live test: `detect(None)` finds the real v100 and `version_pair() >= MIN_SUPPORTED`.
- [ ] **Step 4: Full gate green. Commit** `feat(core): mkvmerge detection ladder + minimum version floor (D28)`

---

### Task 4: Scaffold src-tauri + Vue frontend + toolchain + CI

**Files:**
- Create: `src-tauri/{Cargo.toml,build.rs,tauri.conf.json,capabilities/default.json,src/main.rs,src/lib.rs,icons/*}`, `package.json`, `pnpm-lock.yaml`, `vite.config.ts`, `tsconfig.json`, `index.html`, `src/main.ts`, `src/App.vue` (placeholder), `eslint.config.js`, `locales/en/gui-common.ftl`, `.gitignore` additions (root: `node_modules/`, `dist/`, `src-tauri/gen/`)
- Modify: root `Cargo.toml` (members += `"src-tauri"`), `.github/workflows/ci.yml`
- Test: the gate itself (workspace now compiles tauri) + `pnpm lint` + `pnpm build`

**Interfaces:**
- Produces: crate `muxsmith-gui` (bin, `src-tauri/`); `pnpm dev|build|lint` scripts; identifier `io.github.senolfeldmann.muxsmith`; Vite devUrl `http://localhost:5173`, frontendDist `../dist`. Deps: tauri 2 + `tauri-plugin-dialog` + `tauri-plugin-clipboard-manager`; frontend `vue@3`, `fluent-vue`, `@fluent/bundle`, `@tauri-apps/api`, plugins' JS bindings; dev `vite`, `@vitejs/plugin-vue`, `typescript`, `vue-tsc`, `eslint@9`, `eslint-plugin-vue`, `@intlify/eslint-plugin-vue-i18n`, `typescript-eslint`. `packageManager` field pins pnpm (corepack).
- App.vue placeholder renders one Fluent-sourced heading from `gui-common.ftl` (proves the i18n pipe end to end; no literal strings from day one).

- [ ] **Step 1:** Scaffold via `pnpm create tauri-app` (vue-ts template) or manual - either way normalize to the paths above; wire fluent-vue with the `.ftl` loaded as raw string via Vite `?raw` import.
- [ ] **Step 2: VERIFY the lint gate fires (D27).** Add a literal string to a template; `pnpm lint` MUST fail via `@intlify/eslint-plugin-vue-i18n` `no-raw-text` configured standalone (we use Fluent, not vue-i18n - confirm the rule works without the vue-i18n runtime; current docs via context7). If it does not, replace with a minimal custom eslint rule in `eslint.config.js` scanning Vue template text nodes. Remove the probe string.
- [ ] **Step 3: CI.** In `ci.yml` `test` job add, before the cargo steps: Linux-only apt install of Tauri build deps (`libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev` - verify list against current Tauri v2 Linux prerequisites docs); `actions/setup-node@v4` (node 22) + `corepack enable`; `pnpm install --frozen-lockfile`; after cargo steps: `pnpm lint`, `pnpm build`. Windows/macOS legs (PR/tag matrix) need no webkit deps; guard the apt step with `runner.os == 'Linux'`. Keep checkout@v4 (the v5 bump is a separate pending one-liner).
- [ ] **Step 4:** `cargo deny check` - the tauri tree adds many crates; extend `deny.toml` only with license allowances that actually appear (MIT/Apache-2.0/BSD/Zlib/ISC family expected), never blanket-allow.
- [ ] **Step 5: Full gate + pnpm lint + pnpm build green locally; push branch, verify the CI Linux job passes with the new steps. Commit** `feat(gui): scaffold Tauri 2 shell + Vue 3 frontend + CI toolchain`

---

### Task 5: Per-job cancel in core (D25): QueueControl, index-keyed killers, skip-set

**Files:**
- Modify: `crates/muxsmith-core/src/executor/queue.rs` (QueueControl, RegisteringSpawner re-key, worker loop, watcher), `crates/muxsmith-core/src/executor/job.rs` (cancel closure + pre-spawn check + delete_partial error surfacing), `crates/muxsmith-cli/src/commands/run.rs` (construct QueueControl)
- Test: `crates/muxsmith-core/tests/executor_events.rs` / queue unit tests (append)

**Interfaces:**
- Produces:

```rust
pub struct QueueControl { /* batch: Arc<AtomicBool>, jobs: Vec<AtomicBool>, killers: Mutex<HashMap<usize, Killer>> */ }
impl QueueControl {
    pub fn new(spec_count: usize, batch: Arc<AtomicBool>) -> Arc<QueueControl>;
    pub fn cancel_all(&self);                       // sets batch flag (watcher kills in-flight, as today)
    pub fn cancel_job(&self, index: usize);         // sets jobs[index]; if a killer is registered for index, invokes it NOW
    pub fn job_cancelled(&self, index: usize) -> bool; // batch || jobs[index]
}
pub fn run_queue(specs, spawner, opts, ctl: &Arc<QueueControl>, events) -> Vec<JobOutcome>  // signature change
```

- `run_job` signature change: `cancel: &AtomicBool` becomes `cancelled: &dyn Fn() -> bool` (queue passes `|| ctl.job_cancelled(index)`); job.rs:129 becomes `None if cancelled() => JobState::Cancelled`.
- Consumed by T8 (`cancel_run` -> `cancel_all`, `cancel_job(index)`), CLI (wraps its ctrlc flag: `QueueControl::new(specs.len(), Arc::clone(&cancel))`).

- [ ] **Step 1: TDD, one behavior per failing test:**
  1. *Skip queued:* cancel_job(2) before job 2 dequeues -> outcome[2] Cancelled, `Finished{index:2, Cancelled}` event emitted, NO `Started{index:2}` (deviation from never-dequeued silence - the GUI needs the confirmation; document in run_queue's rustdoc).
  2. *Kill in-flight:* with a blocking FakeSpawner job, cancel_job(index) kills exactly that job -> Cancelled, partial deleted (D17); other jobs unaffected, batch continues.
  3. *Pre-spawn check* (HANDOFF backlog): cancelled flag set before spawn -> Cancelled, spawner never called, nothing deleted.
  4. *delete_partial surfacing* (HANDOFF backlog): a failing partial delete pushes `"delete_partial_failed: <io error>"` into `outcome.errors` (third-party detail passthrough exception).
  5. *Batch semantics unchanged:* existing cancel tests pass with the CLI's flag wrapped in QueueControl.
- [ ] **Step 2:** Implement: registry re-key `Mutex<HashMap<usize, Killer>>` (RegisteringSpawner gains `index`, registers under it, worker removes after run_job); watcher unchanged except it iterates the map's values on batch cancel.
- [ ] **Step 3:** Update CLI run.rs construction; full gate green. **Commit** `feat(core): per-job cancellation via QueueControl (D25) + pre-spawn cancel check`

---

### Task 6: Persisted job logs (`executor::joblog`, D26) + CLI wiring

**Files:**
- Create: `crates/muxsmith-core/src/executor/joblog.rs`
- Modify: `crates/muxsmith-core/src/executor/mod.rs`, `crates/muxsmith-core/Cargo.toml` (`dirs = "6"`, `time = { version = "0.3", features = ["formatting"] }`), `crates/muxsmith-cli/src/commands/run.rs` (tee + finish), `locales/en/cli.ftl` (`run-joblog-unavailable`, `run-joblog-written`)
- Test: `crates/muxsmith-core/tests/joblog.rs` (new, tempdir-based)

**Interfaces:**
- Produces:

```rust
pub struct RunLogger { /* dir, per-job accumulators (argv, output path, lines, started_at) */ }
impl RunLogger {
    pub fn create(runs_root: &Path, run_id: &str, specs: &[JobSpec]) -> std::io::Result<RunLogger>; // mkdir -p runs_root/<run_id>, collision suffix "-2", "-3", ...
    pub fn on_event(&mut self, ev: &JobEvent);      // accumulates Output lines; on Finished writes job-<index>.json
    pub fn finish(self, run_document: &serde_json::Value) -> std::io::Result<PathBuf>; // writes summary.json, returns dir
    pub fn dir(&self) -> &Path;
}
pub fn default_runs_root() -> Option<PathBuf>;      // dirs::data_dir()?/muxsmith/runs  (D26 location)
pub fn make_run_id(now: std::time::SystemTime) -> String; // UTC "YYYYMMDD-HHMMSSZ" via `time`
```

- `job-<index>.json` fields: `index`, `output`, `argv`, `state`, `exit_code`, `warnings`, `errors`, `duration_ms`, `lines` (raw Output lines; NO progress ticks per D24), `started_at`/`finished_at` (RFC3339 UTC). `summary.json` = the T2 `run_document`. Consumed by T8 (GUI runs + `list_runs`/`get_job_log` read these files).

- [ ] **Step 1: TDD** (tempdir as runs_root): create->on_event(Started/Output/Finished)->finish yields exactly `job-0.json` + `summary.json` with the fields above; collision suffix test (pre-create the dir); skipped-job case (Finished{Cancelled} without Started still writes a record with empty `lines`).
- [ ] **Step 2:** Implement; single-threaded writer (the caller's drain loop is single-threaded - no Mutex; say so in the rustdoc, with the invariant that BOTH surfaces must tee their drain loop through it so persistence stays unconditional, spec 6).
- [ ] **Step 3: CLI wiring** in run.rs: before the queue thread, `default_runs_root()` + `RunLogger::create` (failure -> render `run-joblog-unavailable` warning to stderr, continue without logs - a mux run never dies for a log dir); tee `logger.on_event(&event)` in the drain loop (also when `--json`); after join, `finish(&run_document)`; human mode prints `run-joblog-written` with the dir. Update `run_cli.rs`/`run_live.rs` expectations.
- [ ] **Step 4:** `cargo deny check` with the two new deps; full gate green. **Commit** `feat(core+cli): persisted per-job JSON logs under platform data dir (D26)`

---

### Task 7: Shell IPC - read-only commands + settings (D23, D27, D28)

**Files:**
- Create: `src-tauri/src/settings.rs`, `src-tauri/src/error.rs`
- Modify: `src-tauri/src/lib.rs` (commands + state + invoke_handler)
- Test: `src-tauri/src/settings.rs` unit tests (tempdir config dir); command fns factored so the core-calling body is testable without a Tauri runtime

**Interfaces:**
- Produces (all returns `Result<_, IpcError>`; `IpcError { code: String, params: HashMap<String, String> }` - codes only, frontend renders via Fluent):

```rust
#[tauri::command] async fn validate_profile(path: String) -> ...serde_json::Value   // report::json::config_only_document
#[tauri::command] async fn dry_run(profile: String, source: Option<String>, output: Option<String>) -> ...Value  // batch_document; runs on spawn_blocking
#[tauri::command] async fn identify(file: String) -> ...Value
#[tauri::command] fn detect_mkvmerge(state: State<AppState>) -> ...MkvmergeInfo     // { path: String, version: String, meets_minimum: bool } via Mkvmerge::detect(settings override) + version_pair >= MIN_SUPPORTED
#[tauri::command] fn get_settings(state) -> AppSettings
#[tauri::command] fn set_settings(state, s: AppSettings) -> ...()
```

- `AppSettings` (serde, JSON at `app_config_dir()/settings.json`): `mkvmerge_path: Option<String>`, `default_jobs: usize` (1), `locale: Option<String>`, `recent_profiles: Vec<String>` (cap 10, MRU), `dir_memory: HashMap<String, DirMemory { source: Option<String>, output: Option<String> }>` keyed by profile path (D27: never written into the user's YAML).
- Consumed by: T9 (first-run + settings dialog), T10 (validate/dry_run/recents/dir_memory).

- [ ] **Step 1: TDD settings round-trip** (write/read/missing-file defaults/MRU cap) against a tempdir.
- [ ] **Step 2:** Implement commands as thin wrappers; no prose, no logic beyond argument plumbing and settings I/O. Register in `invoke_handler`; grant `dialog` + `clipboard-manager` + `event` permissions in `capabilities/default.json`.
- [ ] **Step 3:** Full gate + pnpm build green. **Commit** `feat(gui): read-only IPC commands + app settings persistence`

---

### Task 8: Shell IPC - run lifecycle (D23): start_run, events, cancel, history

**Files:**
- Create: `src-tauri/src/run.rs`
- Modify: `src-tauri/src/lib.rs` (state + handler registration)
- Test: `src-tauri/src/run.rs` unit tests with `FakeSpawner` (factor the thread body so it takes a `&dyn Spawn` and an event sink closure; assert single-run rejection, event forwarding order, joblog files written)

**Interfaces:**
- Produces:

```rust
#[tauri::command] fn start_run(app: AppHandle, state, profile: String, source: Option<String>, output: Option<String>, jobs: Option<usize>) -> Result<StartedRun, IpcError>
    // StartedRun { run_id: String, total_jobs: usize, run_dir: Option<String> }
    // rejects with code "run-already-active" if a run is active (D23 single-run)
#[tauri::command] fn cancel_run(state) -> Result<(), IpcError>      // QueueControl::cancel_all
#[tauri::command] fn cancel_job(state, index: usize) -> Result<(), IpcError>
#[tauri::command] fn list_runs(state) -> Result<Vec<RunMeta>, IpcError>  // read runs_root: RunMeta { run_id, started_at, summary: Value } from summary.json (skip unreadable dirs)
#[tauri::command] fn get_job_log(state, run_id: String, index: usize) -> Result<Value, IpcError>  // job-<index>.json
```

- Events to the window: each `JobEvent` re-emitted as `muxsmith://job-event` (payload = the D24-golden serde shape verbatim); terminal `muxsmith://run-finished` with the full `run_document`. Thread: re-plan, build specs, `QueueControl::new`, `RunLogger`, `run_queue` on a std thread, drain loop tees `logger.on_event` + `app.emit`. On `WindowEvent::CloseRequested`: `cancel_all()` (cooperative teardown, D23).
- Consumed by: T11 (jobs view + history).

- [ ] **Step 1: TDD** the factored runner with FakeSpawner: events arrive in order Started->Output->Finished; second start rejected while active; active flag clears after finish; joblog dir populated.
- [ ] **Step 2:** Implement; managed state `AppState { active: Mutex<Option<ActiveRun>>, ... }`. Any error-severity diagnostic file skips exactly as the CLI (re-plan path shared through core - no shell-side planning logic).
- [ ] **Step 3:** Full gate + pnpm build green. **Commit** `feat(gui): run lifecycle IPC - start/cancel/events/history`

---

### Task 9: Frontend app shell, Fluent, first-run, settings dialog

**Files:**
- Create: `src/app/` (nav + view switch), `src/i18n/` (fluent-vue setup, catalog loader), `src/views/FirstRun.vue`, `src/components/SettingsDialog.vue`, `locales/en/gui-settings.ftl`; extend `locales/en/gui-common.ftl`
- Modify: `src/App.vue`, `src/main.ts`
- Test: `vue-tsc` (in `pnpm build`) + eslint; behavior covered by T12 smoke

**Interfaces:**
- Produces: single-window layout - `<nav>` (Batch | Jobs, `aria-current` on active) + `<main>`; view switch is a `ref<'batch'|'jobs'>` (no router at two views); `t()` from fluent-vue everywhere; first-run flow: on mount call `detect_mkvmerge` -> found: proceed; missing/too old: `FirstRun.vue` full-screen guidance per OS (`platform()` from @tauri-apps/api) with manual path picker (dialog plugin) writing `set_settings` then re-detect; SettingsDialog: native `<dialog>`, labeled form fields (`label for`/`id`) for mkvmerge path, default jobs, locale.
- Consumed by: T10/T11 mount their views into the switch; conventions (semantic HTML, Fluent-only strings, `data-testid` on structural nodes) are the template the view tasks copy.

- [ ] **Step 1:** Implement shell + Fluent loader (catalogs via `?raw` imports, one FluentBundle, diagnostics.ftl included - diagnostic rendering reuses the SAME message templates as the CLI, spec 8.4).
- [ ] **Step 2:** First-run + settings against T7 commands; every control labeled; `pnpm lint` (no-raw-text) + `pnpm build` green.
- [ ] **Step 3: Commit** `feat(gui): app shell, Fluent wiring, first-run detection, settings dialog`

---

### Task 10: Batch view (spec 8.2 view 2, minus apply-suggestion per D22)

**Files:**
- Create: `src/views/BatchView.vue`, `src/components/{ResolutionTable,DiagnosticsPanel,SuggestionCard}.vue`, `locales/en/gui-batch.ftl`
- Test: T12 smoke covers behavior; `pnpm lint`/`build` per commit

**Interfaces:**
- Consumes: T7 (`validate_profile`, `dry_run`, `get_settings`/`set_settings` for recents + dir_memory), T9 shell conventions.
- Produces: `startRun()` emit consumed by App to switch to Jobs view with run parameters (profile, source, output, jobs).

- [ ] **Step 1:** Profile picker (dialog plugin, `.yaml/.yml` filter) + recents list (MRU from settings); on pick -> `validate_profile` -> diagnostics render (code+params -> Fluent, severity icon + text, `role="status"` for the summary line).
- [ ] **Step 2:** Source/output dir pickers, prefilled from `dir_memory[profile]`, persisted back on change. Dry-run button (disabled while running, `aria-busy` during); render `batch_document`: per-file `<table>` (caption, `<th scope>`) rule -> resolved track; config + per-file diagnostics; suggestions as cards with the YAML fragment in `<pre><code>` and a copy button (clipboard-manager plugin) - copy is the ONLY suggestion action (D22).
- [ ] **Step 3:** Run button -> emits startRun with the current selection; disabled with a Fluent tooltip when errors exist or mkvmerge missing.
- [ ] **Step 4:** lint + build green. **Commit** `feat(gui): batch view - profile/dirs, dry-run report, suggestions show+copy`

---

### Task 11: Jobs view - live queue, cancel, history, log export (spec 8.2 view 3, D30)

**Files:**
- Create: `src/views/JobsView.vue`, `src/components/{JobRow,LiveLog,RunHistory}.vue`, `locales/en/gui-jobs.ftl`
- Test: T12 smoke; `pnpm lint`/`build` per commit

**Interfaces:**
- Consumes: T8 (`start_run`, `cancel_run`, `cancel_job`, `list_runs`, `get_job_log`, the `muxsmith://job-event` + `muxsmith://run-finished` events), T9 conventions.

- [ ] **Step 1:** On startRun: call `start_run`, render one `JobRow` per job (`data-testid="job-row"` + `data-index`): output filename, state chip, `role="progressbar"` with `aria-valuenow` (or native `<progress>`), per-row cancel button; batch header: overall progress (finished/total), cancel-batch button. Event subscription drives all state; warning count badges from `warning` events.
- [ ] **Step 2:** `LiveLog`: `role="log"` region fed by `output` events (auto-scroll unless the user scrolled up; DOM-capped at 5000 lines - the full log is in the file); per-job filter select.
- [ ] **Step 3:** `run-finished`: summary line in a polite `aria-live` region (ok/warning/failed/cancelled counts); rows finalize from the document.
- [ ] **Step 4:** `RunHistory`: `list_runs` (newest first) -> select run -> jobs from its `summary.json`, per-job log via `get_job_log`; copy-log button + save-as (dialog plugin) - the D30 gap closure (mkvtoolnix-gui parity: open finished job log as text).
- [ ] **Step 5:** lint + build green. **Commit** `feat(gui): jobs view - live queue, per-job/batch cancel, history + log export`

---

### Task 12: Playwright smoke + i18n completeness gate + CI finish

**Files:**
- Create: `playwright.config.ts`, `e2e/{smoke.spec.ts,mocks.ts}`, `scripts/check-i18n.mjs`
- Modify: `package.json` (scripts `test:e2e`, `check:i18n`), `.github/workflows/ci.yml`
- Test: this task IS the test layer

**Interfaces:**
- Consumes: everything T9-T11 rendered; `mockIPC`/`mockWindows` from `@tauri-apps/api/mocks` (the smoke runs the Vite build in a plain browser; no tauri-driver - spec 10 keeps GUI tests thin).

- [ ] **Step 1: Smoke scenarios** (all locators `getByRole` first, `data-testid` fallback; locale pinned `en`): (a) detect fails -> first-run guidance visible, manual path via mocked dialog fixes it; (b) mocked `dry_run` document -> resolution table + diagnostics + suggestion copy calls the mocked clipboard; (c) mocked run: emit scripted job-events -> rows progress, log fills, cancel_job invoked by row button, run-finished summary announced.
- [ ] **Step 2: a11y assertion:** `@axe-core/playwright` scan on each view in the smoke; fail on serious/critical violations.
- [ ] **Step 3: `check-i18n.mjs`:** parse `locales/en/gui-*.ftl` + `diagnostics.ftl` message ids; scan `src/**/*.{vue,ts}` for `t('...')`/`$t('...')` ids; unknown id -> exit 1 (missing key); report unused gui-* keys as warnings. Wire as `pnpm check:i18n`.
- [ ] **Step 4: CI:** append `pnpm check:i18n`, `pnpm exec playwright install --with-deps chromium` + `pnpm test:e2e` to the Linux leg. Full gate + all pnpm gates green, CI verified on a push. **Commit** `test(gui): playwright smoke + axe a11y + i18n completeness gate`

---

### Task 13: Close-out (controller, not a subagent)

- [ ] Whole-branch review on the most capable model (superpowers:requesting-code-review), triage + fix waves as needed.
- [ ] Verify gated tests RAN in CI (no skip markers), 3-OS via a PR if warranted.
- [ ] Process journal entry per `docs/process-journal/PROMPT.md` (SI-2); salvage `.superpowers/sdd/` artifacts to `docs/process-journal/artifacts/plan-5-sdd/` and verify the file count IN the commit.
- [ ] New HANDOFF.md (reproduce SI section verbatim); push; `gh-log.md` entry.
