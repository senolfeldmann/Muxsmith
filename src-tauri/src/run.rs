//! Run lifecycle IPC (D23): `start_run`/`cancel_run`/`cancel_job`, the
//! `muxsmith://job-event`/`muxsmith://run-finished` window events, and run
//! history (`list_runs`/`get_job_log`). Mirrors the CLI's `run` command
//! (re-plan, build [`JobSpec`]s, drive [`run_queue`]) so planning stays
//! shared through core (spec 7) -- this module adds only the shell-specific
//! parts: the single-active-run gate, window-event emission, and
//! persisted-history reads.
//!
//! **D23 single-run.** At most one run may be active; [`AppState::active`]
//! is the source of truth. [`start_run`] *reserves* the slot up front
//! ([`Reservation`]) and then plans **without holding the lock**: planning
//! shells out to mkvmerge per file and can run long, and
//! [`on_close_requested`] locks this same mutex synchronously on Tauri's
//! event-loop thread, so a lock held across planning would freeze the
//! whole UI for that duration. The reservation keeps the single-run
//! invariant airtight end-to-end (a second `start_run` during planning is
//! still rejected with `"run-already-active"`) while every lock
//! acquisition stays O(1).
//!
//! **D31 close-with-active-run.** A window close while a run is active
//! (planning or running) never exits immediately: [`on_close_requested`]
//! prevents the close and asks, via a native non-blocking confirmation
//! dialog, whether to abort the running batch. Yes cancels everything and
//! exits only once the runner's teardown has fully completed (kills
//! landed, joblog `summary.json` written); No leaves the window open and
//! the run untouched. With no active run the window closes normally.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::SystemTime;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State, Window, WindowEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::command::command;
use muxsmith_core::executor::job::{JobOutcome, JobSpec};
use muxsmith_core::executor::joblog::{RunLogger, default_runs_root, make_run_id};
use muxsmith_core::executor::queue::{JobEvent, QueueControl, QueueOpts, run_queue};
use muxsmith_core::executor::spawn::{LiveSpawner, Spawn};
use muxsmith_core::identify::{IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report::Diagnostic;
use muxsmith_core::report::json::{
    DiagnosticRenderer, batch_document, config_only_document, run_document,
};

use crate::AppState;
use crate::error::IpcError;

/// What currently occupies [`AppState`]'s single run slot (the struct
/// itself, plus its `quit_after_finished` sibling field, live in `lib.rs`
/// -- unified there across this task's run-tracking fields and T7's
/// settings field). Both variants count as "a run is active" for the D23
/// single-run gate; they differ only in what a cancel can reach (see each
/// variant). `pub(crate)`: named from `AppState`'s field type in the
/// parent module.
pub(crate) enum RunSlot {
    /// [`start_run`] is mid-planning: the slot is held so a concurrent
    /// `start_run` is rejected, but no queue exists yet. Carries the
    /// batch-cancel flag that will become the queue's own (via
    /// [`QueueControl::new`]) if the run materializes, so a
    /// `cancel_run`/window-close landing in this window is not lost: the
    /// queue is born already-cancelled and every job finishes `Cancelled`
    /// without spawning (D16 semantics, before the first dequeue).
    Reserved(Arc<AtomicBool>),
    /// The queue is running; the [`ActiveRun`]'s control reaches it.
    Running(ActiveRun),
}

/// The running queue occupying [`AppState::active`]'s slot: the
/// [`QueueControl`] that reaches it for `cancel_run`/`cancel_job`/
/// window-close teardown. The run's id is returned to the caller directly
/// by [`start_run`] (as [`StartedRun::run_id`]) and is not otherwise
/// consumed while the run is in flight, so it is not duplicated here.
/// `pub(crate)`, matching [`RunSlot`]: it is reachable through
/// `RunSlot::Running`'s field, which is itself `pub(crate)`-visible from
/// `AppState` in the parent module.
pub(crate) struct ActiveRun {
    ctl: Arc<QueueControl>,
}

/// An RAII hold on [`AppState::active`]'s slot for the span of
/// [`start_run`]'s lock-free planning pass (D23). [`Reservation::acquire`]
/// installs [`RunSlot::Reserved`] and releases the mutex immediately;
/// dropping the reservation *without* [`Reservation::commit`] clears the
/// slot again -- one mechanism covering every soft-outcome early return
/// (and a mid-planning panic) so no path can leak the reservation and
/// wedge the app in permanent `"run-already-active"`.
struct Reservation<'a> {
    state: &'a AppState,
    cancel: Arc<AtomicBool>,
    committed: bool,
}

impl<'a> Reservation<'a> {
    /// Reserves the single-run slot, or fails with `"run-already-active"`
    /// if either a reservation or a running queue already holds it. The
    /// mutex is held only for this check-and-insert, never across
    /// planning. Also discards any stale quit-after-finished request
    /// (D31): a new run starting supersedes a pending quit -- without
    /// this, a flag orphaned by e.g. a mid-callback panic would silently
    /// exit the app after the *next* unrelated run.
    fn acquire(state: &'a AppState) -> Result<Reservation<'a>, IpcError> {
        let mut slot = state.active.lock().unwrap();
        if slot.is_some() {
            return Err(IpcError::code("run-already-active"));
        }
        state.quit_after_finished.store(false, Ordering::SeqCst);
        let cancel = Arc::new(AtomicBool::new(false));
        *slot = Some(RunSlot::Reserved(Arc::clone(&cancel)));
        Ok(Reservation {
            state,
            cancel,
            committed: false,
        })
    }

    /// The reservation's batch-cancel flag, to be shared into
    /// [`QueueControl::new`] as the queue's own batch flag: a cancel that
    /// landed during planning ([`do_cancel_run`]/[`on_close_requested`] on
    /// [`RunSlot::Reserved`]) is thereby carried into the queue instead of
    /// lost.
    fn cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancel)
    }

    /// Promotes the reservation to a running queue: installs
    /// [`RunSlot::Running`] over the `Reserved` placeholder. Called before
    /// the queue thread spawns, so the thread's own end-of-run clear
    /// ([`finish_teardown`]) can never race ahead of the install.
    fn commit(mut self, ctl: Arc<QueueControl>) {
        *self.state.active.lock().unwrap() = Some(RunSlot::Running(ActiveRun { ctl }));
        self.committed = true;
    }
}

impl Drop for Reservation<'_> {
    fn drop(&mut self) {
        if !self.committed {
            *self.state.active.lock().unwrap() = None;
        }
    }
}

/// [`start_run`]'s success payload.
#[derive(Debug, Clone, Serialize)]
pub struct StartedRun {
    /// This run's id ([`make_run_id`]'s timestamp stamp), always present
    /// even on a path that never touched the queue (D15: a run that plans
    /// zero jobs still "happened", it just has no [`Self::run_dir`]).
    pub run_id: String,
    /// The number of jobs actually queued; `0` on a path that never
    /// touched the queue.
    pub total_jobs: usize,
    /// The run's persisted log directory, or `None` when nothing was ever
    /// written (no `runs_root` resolvable, [`RunLogger::create`] itself
    /// failed, or the run never touched the queue at all -- D26: no run
    /// directory for a run with nothing to log).
    pub run_dir: Option<String>,
}

/// One entry of [`list_runs`]'s result: enough to render a history list
/// without opening every per-job log.
#[derive(Debug, Clone, Serialize)]
pub struct RunMeta {
    /// The run's directory name under the runs root.
    pub run_id: String,
    /// The run's start time, RFC3339, parsed from `run_id`'s own
    /// timestamp prefix (see [`started_at_from_run_id`]).
    pub started_at: String,
    /// The run's persisted `summary.json` (a `report::json::run_document`),
    /// verbatim.
    pub summary: serde_json::Value,
}

/// D26 joblog persistence status, spliced into the `muxsmith://run-finished`
/// payload as `joblog_status` (never prose, mirroring the CLI's own
/// `run-joblog-written`/`run-joblog-incomplete`/`run-joblog-unavailable`
/// distinction as a stable code instead of a rendered line).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JoblogStatus {
    /// Every job log plus `summary.json` was written.
    Complete,
    /// The run directory exists but at least one write into it failed.
    Incomplete,
    /// No run directory exists at all (unresolvable runs root, a failed
    /// [`RunLogger::create`], or a run that never touched the queue).
    Unavailable,
}

/// Starts a run (D23): re-plans `profile`/`source`/`output` from scratch
/// (never reusing a stale dry-run, mirroring the CLI's `run` command and
/// spec 5.5's re-plan invariant), then drives the resulting jobs to
/// completion on a detached thread. Rejects with `"run-already-active"` if
/// a run is already in flight (D23 single-run).
///
/// Every branch where planning never reaches a real job queue (profile
/// load failure, mkvmerge missing/unqueryable, or a batch that plans zero
/// jobs) still returns `Ok`: it synchronously emits
/// `muxsmith://run-finished` with the same `run_document` shape a real run
/// would (spec 7 -- CLI and GUI share this document shape), `total_jobs:
/// 0`, and no `run_dir`. `IpcError` is reserved for the one failure the
/// caller must react to differently (the single-run conflict); a planning
/// outcome is data, not a shell-level error -- and in practice `start_run`
/// is only reachable once T10's Batch view has already shown a clean
/// `dry_run`, so these branches are defensive, not the expected path.
///
/// **Event-ordering contract (frontend requirement).** Listeners for BOTH
/// `muxsmith://job-event` and `muxsmith://run-finished` MUST be registered
/// *before* invoking `start_run`. On the soft-outcome branches above,
/// `muxsmith://run-finished` is emitted synchronously, before this
/// command's own `Result` ever returns to the caller; a frontend that
/// waits for `start_run` to resolve and only then subscribes misses that
/// terminal event entirely (and can race the first job events of a real
/// run the same way). Subscribe first, then invoke.
#[tauri::command]
pub fn start_run(
    app: AppHandle,
    state: State<AppState>,
    profile: String,
    source: Option<String>,
    output: Option<String>,
    jobs: Option<usize>,
) -> Result<StartedRun, IpcError> {
    // Reserve the single-run slot, then plan WITHOUT holding the lock
    // (module doc: a lock held across planning would freeze the event
    // loop). Dropping `reservation` uncommitted -- every soft-outcome
    // early return below -- releases the slot again.
    let reservation = Reservation::acquire(&state)?;

    let profile_path = PathBuf::from(profile);
    let profile = match load::from_file(&profile_path) {
        Ok(p) => p,
        Err(d) => {
            drop(reservation);
            let doc = run_document(config_only_document(&[d], None, &ShellRenderer), &[], &[]);
            return Ok(finish_without_queue(&app, &state, doc));
        }
    };

    let mut config_diags = validate::validate(&profile);
    config_diags.extend(lint::provable_overlaps(&profile));

    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            drop(reservation);
            let doc = run_document(
                config_only_document(&config_diags, Some(false), &ShellRenderer),
                &[],
                &[],
            );
            return Ok(finish_without_queue(&app, &state, doc));
        }
    };
    let lang = match mkv.list_languages() {
        Ok(l) => l,
        Err(_) => {
            drop(reservation);
            let doc = run_document(
                config_only_document(&config_diags, Some(true), &ShellRenderer),
                &[],
                &[],
            );
            return Ok(finish_without_queue(&app, &state, doc));
        }
    };

    let run_inputs = RunInputs {
        source: source
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".")),
        output: output.map(PathBuf::from),
        on_collision: None,
    };
    let mut ident = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &mkv,
    };
    let batch = plan_batch(&profile, &run_inputs, &mut ident, &lang);

    // Error-severity files already carry `plan: None` (spec 5.1); this
    // filter_map is also the "does this file get muxed" gate, exactly like
    // the CLI's `run` command (no shell-side planning logic).
    let specs: Vec<JobSpec> = batch
        .files
        .iter()
        .filter_map(|f| f.plan.as_ref())
        .map(|p| JobSpec {
            argv: command(p),
            output: p.output.clone(),
        })
        .collect();

    if specs.is_empty() {
        drop(reservation);
        let doc = run_document(
            batch_document(&config_diags, &batch, &ShellRenderer),
            &[],
            &[],
        );
        return Ok(finish_without_queue(&app, &state, doc));
    }

    let run_id = make_run_id(SystemTime::now());
    // The reservation's own flag becomes the queue's batch-cancel flag: a
    // cancel_run/window-close that landed while planning ran is carried
    // into the queue (born already-cancelled) instead of lost.
    let ctl = QueueControl::new(specs.len(), reservation.cancel_flag());
    let logger =
        resolve_runs_root().and_then(|root| RunLogger::create(&root, &run_id, &specs).ok());
    let run_dir = logger.as_ref().map(|l| l.dir().display().to_string());
    let total_jobs = specs.len();

    reservation.commit(Arc::clone(&ctl));

    let base = batch_document(&config_diags, &batch, &ShellRenderer);
    let outputs: Vec<String> = specs
        .iter()
        .map(|s| s.output.display().to_string())
        .collect();
    let mkv_path = mkv.path().to_path_buf();
    let jobs = jobs.unwrap_or(1);
    let app_bg = app.clone();

    std::thread::spawn(move || {
        let spawner = LiveSpawner { mkvmerge: mkv_path };
        let opts = QueueOpts {
            jobs,
            fail_fast: false,
        };
        let (outcomes, logger) = run_batch(&specs, &spawner, opts, &ctl, logger, |event| {
            let _ = app_bg.emit("muxsmith://job-event", event);
        });

        let document = run_document(base, &outcomes, &outputs);
        let status = finalize_joblog(logger, &document);
        emit_run_finished(&app_bg, document, status);
        // Teardown is complete only now (kills landed, joblog finalized,
        // terminal event emitted): clear the slot and honor a pending
        // quit-after-finished (D31) -- never earlier, or a confirmed quit
        // could kill the process before summary.json is written.
        finish_teardown(&app_bg.state::<AppState>(), |code| app_bg.exit(code));
    });

    Ok(StartedRun {
        run_id,
        total_jobs,
        run_dir,
    })
}

/// Cancels the whole active run (D23, [`QueueControl::cancel_all`]).
/// Errors with `"no-active-run"` if nothing is running.
#[tauri::command]
pub fn cancel_run(state: State<AppState>) -> Result<(), IpcError> {
    do_cancel_run(&state)
}

/// Cancels one job of the active run by its queue index (D25,
/// [`QueueControl::cancel_job`]); an out-of-range `index` is a no-op at
/// the core layer (see `QueueControl::cancel_job`'s own doc), not
/// surfaced as an error here. Errors with `"no-active-run"` if nothing is
/// running.
#[tauri::command]
pub fn cancel_job(state: State<AppState>, index: usize) -> Result<(), IpcError> {
    do_cancel_job(&state, index)
}

/// Lists every readable run under the runs root (D26), newest first
/// (`run_id` descending -- [`make_run_id`]'s fixed-width timestamp format
/// sorts chronologically as a plain string). A directory that cannot be
/// read as a complete run record (no `summary.json`, invalid JSON, or an
/// unparseable `run_id` prefix) is silently skipped rather than failing
/// the whole listing; an unresolvable or not-yet-existing runs root
/// yields an empty list, never an error (spec 6: a caller never dies over
/// the log directory).
///
/// `state` is currently unused (nothing here reads `AppState` yet) but is
/// part of the given interface for parity with the other run-lifecycle
/// commands and any future runs-root override.
#[tauri::command]
pub fn list_runs(_state: State<AppState>) -> Result<Vec<RunMeta>, IpcError> {
    Ok(list_runs_in(resolve_runs_root().as_deref()))
}

/// Reads one job's persisted log record (`job-<index>.json`, D26).
///
/// `state` is currently unused; see [`list_runs`]'s doc.
#[tauri::command]
pub fn get_job_log(
    _state: State<AppState>,
    run_id: String,
    index: usize,
) -> Result<serde_json::Value, IpcError> {
    get_job_log_in(resolve_runs_root().as_deref(), &run_id, index)
}

/// The English GUI catalog, embedded at build time (D31): the shell's few
/// native-dialog strings are looked up here via [`ftl_message`] instead of
/// being written as Rust literals, so `locales/en/gui-common.ftl` stays
/// the single source of truth that T9's frontend Fluent loader will also
/// read.
const GUI_COMMON_FTL: &str = include_str!("../../locales/en/gui-common.ftl");

/// Minimal single-line Fluent-message lookup over [`GUI_COMMON_FTL`]:
/// finds the `key = value` line and returns the trimmed value.
/// Deliberately NOT a Fluent parser: the shell only ever consumes simple
/// one-line messages (the `.ftl` carries a comment pinning that
/// constraint, and `close_abort_strings_resolve` tests each key), and a
/// full Fluent stack in the shell would duplicate the frontend's loader
/// for four strings. A missing key degrades to the key itself -- a stable
/// code, not a panic, matching the shell's prose-free posture.
fn ftl_message(key: &'static str) -> &'static str {
    GUI_COMMON_FTL
        .lines()
        .find_map(|line| {
            line.strip_prefix(key)?
                .trim_start()
                .strip_prefix('=')
                .map(str::trim)
        })
        .unwrap_or(key)
}

/// What a `CloseRequested` event should do, decided from the run slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CloseDecision {
    /// No active run: let the window close normally (D31 unchanged path).
    Close,
    /// A run is active (planning or running): prevent the close and ask
    /// whether to abort the batch.
    ConfirmAbort,
}

/// The close-vs-ask decision (D31), factored off the Tauri types so it is
/// unit-testable: any occupant of the slot (Reserved or Running) means
/// "confirm first".
fn close_decision(state: &AppState) -> CloseDecision {
    if state.active.lock().unwrap().is_some() {
        CloseDecision::ConfirmAbort
    } else {
        CloseDecision::Close
    }
}

/// Window-close handling (D31, supersedes D23's bare cancel_all): with no
/// active run the window closes normally; with an active run the close is
/// prevented and a native, non-blocking confirmation dialog asks whether
/// to abort the running batch (mkvtoolnix-gui parity, SI-3). Yes runs
/// [`abort_and_quit`] (cancel everything, exit after teardown completes);
/// No does nothing -- the window stays open and the run continues.
///
/// Everything on the event-loop thread is O(1): the slot check is a
/// pointer read and `show` only schedules the dialog, its callback firing
/// later. The dialog itself carries the [`ftl_message`] strings, the one
/// place the shell hands prose to the OS (sourced from the `.ftl`, not
/// hardcoded).
pub fn on_close_requested(window: &Window, event: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };
    let app = window.app_handle();
    if close_decision(&app.state::<AppState>()) == CloseDecision::Close {
        return;
    }
    api.prevent_close();

    let app = app.clone();
    app.dialog()
        .message(ftl_message("close-abort-message"))
        .title(ftl_message("close-abort-title"))
        .kind(MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::OkCancelCustom(
            ftl_message("close-abort-confirm").to_string(),
            ftl_message("close-abort-dismiss").to_string(),
        ))
        .show(move |abort| {
            if abort {
                abort_and_quit(&app.state::<AppState>(), |code| app.exit(code));
            }
        });
}

/// The dialog's Yes path (D31): requests quit-after-finished, then cancels
/// whatever currently occupies the slot -- a running queue via
/// `cancel_all`, a still-planning reservation via its cancel flag (the
/// future queue's own batch flag, so the cancel is carried, not lost). The
/// pending quit is then honored by whichever completion path runs last:
/// [`finish_teardown`] after a real run, [`finish_without_queue`] after a
/// soft outcome, or right here when the run already tore down while the
/// dialog was open (slot empty = teardown fully complete, including the
/// joblog, since clearing is teardown's final step).
///
/// `exit` is injected (production: `app.exit`) so the exactly-once
/// semantics are unit-testable; the `swap` inside [`quit_if_requested`]
/// is what guarantees a single exit no matter which paths race.
fn abort_and_quit(state: &AppState, exit: impl FnOnce(i32)) {
    state.quit_after_finished.store(true, Ordering::SeqCst);
    let already_torn_down = match state.active.lock().unwrap().as_ref() {
        Some(RunSlot::Reserved(cancel)) => {
            cancel.store(true, Ordering::SeqCst);
            false
        }
        Some(RunSlot::Running(run)) => {
            run.ctl.cancel_all();
            false
        }
        None => true,
    };
    if already_torn_down {
        quit_if_requested(state, exit);
    }
}

/// Consumes a pending quit-after-finished request (D31): calls `exit(0)`
/// iff the flag was set, clearing it atomically (`swap`), so concurrent
/// completion paths ([`abort_and_quit`] vs [`finish_teardown`] vs
/// [`finish_without_queue`]) can all call this and exactly one ever exits.
fn quit_if_requested(state: &AppState, exit: impl FnOnce(i32)) {
    if state.quit_after_finished.swap(false, Ordering::SeqCst) {
        exit(0);
    }
}

/// The runner thread's final step (D23/D31): clears the active-run slot,
/// then honors a pending quit-after-finished. Runs strictly after
/// [`finalize_joblog`] and the terminal emit, so an empty slot always
/// means "teardown fully complete" -- the invariant [`abort_and_quit`]'s
/// direct-exit arm relies on. The tradeoff is documented: a frontend that
/// reacts to `muxsmith://run-finished` by instantly invoking `start_run`
/// can race this clear by microseconds and get a spurious
/// `"run-already-active"`; that is recoverable (retry), whereas exiting
/// before `summary.json` is written would lose the run's history (the
/// exact loss D26/D31 exist to prevent).
fn finish_teardown(state: &AppState, exit: impl FnOnce(i32)) {
    *state.active.lock().unwrap() = None;
    quit_if_requested(state, exit);
}

/// Synchronously finishes a [`start_run`] call that never touched the
/// queue (profile load failure, mkvmerge unavailable, or a batch that
/// planned zero jobs): emits `muxsmith://run-finished` with `document`
/// (already the full `run_document` shape) and a `joblog_status` of
/// [`JoblogStatus::Unavailable`] (D26: nothing ran, so no run directory
/// exists), and returns the [`StartedRun`] `start_run` itself returns --
/// `total_jobs: 0`, no `run_dir`. The caller drops its [`Reservation`]
/// before calling this, releasing the single-run slot; the
/// [`quit_if_requested`] at the end is this path's teardown-completion
/// hook (D31): a quit confirmed while planning ran still exits even
/// though no queue ever existed.
///
/// The emit happens synchronously inside the `start_run` invocation, i.e.
/// before the command's `Result` reaches the frontend: this is the emit
/// site behind [`start_run`]'s event-ordering contract (listeners must be
/// registered before invoking the command, or this event is lost).
fn finish_without_queue(
    app: &AppHandle,
    state: &AppState,
    document: serde_json::Value,
) -> StartedRun {
    let run_id = make_run_id(SystemTime::now());
    emit_run_finished(app, document, JoblogStatus::Unavailable);
    quit_if_requested(state, |code| app.exit(code));
    StartedRun {
        run_id,
        total_jobs: 0,
        run_dir: None,
    }
}

/// Emits `muxsmith://run-finished`: `document` (a
/// `report::json::run_document`) with a `joblog_status` field spliced in
/// (D26). `document` is always a JSON object here (every `run_document`
/// base is built from an object literal), so the splice never panics.
///
/// Called from two places with different timing: the queue thread's
/// natural end (after the last job), and [`finish_without_queue`]'s
/// synchronous soft-outcome path -- the latter fires before `start_run`
/// even returns, which is why [`start_run`]'s doc requires listeners to be
/// registered before the command is invoked.
fn emit_run_finished(app: &AppHandle, mut document: serde_json::Value, status: JoblogStatus) {
    document["joblog_status"] =
        serde_json::to_value(status).expect("JoblogStatus always serializes");
    let _ = app.emit("muxsmith://run-finished", document);
}

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
/// [`finish_teardown`]'s job, and it must run only after the joblog is
/// finalized and the terminal event emitted (D31: "slot empty" has to
/// mean "teardown fully complete", or a confirmed quit could exit the
/// process before `summary.json` is written).
///
/// Returns the outcomes (index-aligned to `specs`, exactly like
/// `run_queue`) and `logger` back, still open, so the caller can build the
/// terminal `run_document` and only then call [`RunLogger::finish`] on it
/// (`finish` needs the very document it is about to persist).
fn run_batch(
    specs: &[JobSpec],
    spawner: &(dyn Spawn + Sync),
    opts: QueueOpts,
    ctl: &Arc<QueueControl>,
    mut logger: Option<RunLogger>,
    mut on_event: impl FnMut(&JobEvent),
) -> (Vec<JobOutcome>, Option<RunLogger>) {
    let (tx, rx) = mpsc::channel();
    let outcomes = std::thread::scope(|scope| {
        let handle = scope.spawn(move || run_queue(specs, spawner, opts, ctl, &tx));
        for event in rx {
            if let Some(l) = logger.as_mut() {
                l.on_event(&event);
            }
            on_event(&event);
        }
        handle.join().expect("queue worker thread panicked")
    });

    (outcomes, logger)
}

/// Persists `document` to `logger`'s `summary.json` (D26) and folds the
/// result into a [`JoblogStatus`]: `None` (no logger -- runs_root
/// unresolvable, or [`RunLogger::create`] itself failed) is
/// [`JoblogStatus::Unavailable`]; `Some` folds [`RunLogger::finish`]'s
/// `Ok`/`Err` into [`JoblogStatus::Complete`]/[`JoblogStatus::Incomplete`]
/// (`Err` covers both a failed `summary.json` write and an earlier lost
/// `job-<index>.json` write, exactly like the CLI's own
/// `run-joblog-incomplete` path).
fn finalize_joblog(logger: Option<RunLogger>, document: &serde_json::Value) -> JoblogStatus {
    match logger {
        None => JoblogStatus::Unavailable,
        Some(logger) => match logger.finish(document) {
            Ok(_) => JoblogStatus::Complete,
            Err(_) => JoblogStatus::Incomplete,
        },
    }
}

/// Resolves the run-log directory (D26), mirroring the CLI's own
/// `create_logger`: [`default_runs_root`], with a debug-build-only
/// `MUXSMITH_RUNS_ROOT` override (a test seam, deliberately absent from
/// release builds -- see the CLI's identical gate for the rationale).
fn resolve_runs_root() -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    {
        std::env::var_os("MUXSMITH_RUNS_ROOT")
            .map(PathBuf::from)
            .or_else(default_runs_root)
    }
    #[cfg(not(debug_assertions))]
    {
        default_runs_root()
    }
}

/// [`cancel_run`]'s testable body. A run mid-planning
/// ([`RunSlot::Reserved`]) is cancelled by setting the reservation's flag,
/// which [`start_run`] shares into [`QueueControl::new`] as the queue's
/// batch flag: the queue is born already-cancelled and every job finishes
/// `Cancelled` without spawning, so a cancel in the planning window is
/// honored, not lost.
fn do_cancel_run(state: &AppState) -> Result<(), IpcError> {
    match state.active.lock().unwrap().as_ref() {
        Some(RunSlot::Reserved(cancel)) => {
            cancel.store(true, Ordering::SeqCst);
            Ok(())
        }
        Some(RunSlot::Running(run)) => {
            run.ctl.cancel_all();
            Ok(())
        }
        None => Err(IpcError::code("no-active-run")),
    }
}

/// [`cancel_job`]'s testable body. During [`RunSlot::Reserved`] the job
/// set does not exist yet, so a per-job cancel has no target: it is
/// accepted as a no-op, mirroring [`QueueControl::cancel_job`]'s own
/// out-of-range no-op semantics. The window is unreachable from a
/// well-behaved frontend anyway -- job indices only become known once
/// `start_run` returns, and by then the slot is `Running` (commit happens
/// before `start_run` returns).
fn do_cancel_job(state: &AppState, index: usize) -> Result<(), IpcError> {
    match state.active.lock().unwrap().as_ref() {
        Some(RunSlot::Reserved(_)) => Ok(()),
        Some(RunSlot::Running(run)) => {
            run.ctl.cancel_job(index);
            Ok(())
        }
        None => Err(IpcError::code("no-active-run")),
    }
}

fn list_runs_in(runs_root: Option<&Path>) -> Vec<RunMeta> {
    let Some(root) = runs_root else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut metas: Vec<RunMeta> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter_map(|e| run_meta_from_dir(&e.path()))
        .collect();
    metas.sort_by(|a, b| b.run_id.cmp(&a.run_id));
    metas
}

fn run_meta_from_dir(dir: &Path) -> Option<RunMeta> {
    let run_id = dir.file_name()?.to_str()?.to_string();
    let started_at = started_at_from_run_id(&run_id)?;
    let text = fs::read_to_string(dir.join("summary.json")).ok()?;
    let summary: serde_json::Value = serde_json::from_str(&text).ok()?;
    Some(RunMeta {
        run_id,
        started_at,
        summary,
    })
}

/// Parses [`make_run_id`]'s fixed `"YYYYMMDD-HHMMSSZ"` prefix (the first
/// 16 bytes -- present even on a collision-suffixed directory name like
/// `"...Z-2"`) into an RFC3339 timestamp, e.g. `"20260710-153612Z"` ->
/// `"2026-07-10T15:36:12Z"`. `None` for anything that does not match: a
/// directory this crate did not itself create.
fn started_at_from_run_id(run_id: &str) -> Option<String> {
    let prefix = run_id.get(0..16)?;
    let b = prefix.as_bytes();
    let all_digits = |r: std::ops::Range<usize>| b[r].iter().all(u8::is_ascii_digit);
    if b[8] != b'-' || b[15] != b'Z' || !all_digits(0..8) || !all_digits(9..15) {
        return None;
    }
    Some(format!(
        "{}-{}-{}T{}:{}:{}Z",
        &prefix[0..4],
        &prefix[4..6],
        &prefix[6..8],
        &prefix[9..11],
        &prefix[11..13],
        &prefix[13..15],
    ))
}

fn get_job_log_in(
    runs_root: Option<&Path>,
    run_id: &str,
    index: usize,
) -> Result<serde_json::Value, IpcError> {
    if !valid_run_id(run_id) {
        return Err(IpcError::code("invalid-run-id").with("run_id", run_id));
    }
    let root = runs_root.ok_or_else(|| IpcError::code("job-log-unavailable"))?;
    let path = root.join(run_id).join(format!("job-{index}.json"));
    let not_found = || {
        IpcError::code("job-log-not-found")
            .with("run_id", run_id)
            .with("index", index.to_string())
    };
    let text = fs::read_to_string(&path).map_err(|_| not_found())?;
    serde_json::from_str(&text).map_err(|_| not_found())
}

/// Guards path traversal (D26): a valid `run_id` is a single plain path
/// component -- no separators, no `':'` (a Windows drive prefix like `"C:"`
/// is a root-replacing component under `PathBuf::join` there, and `':'`
/// never occurs in a [`make_run_id`] name anyway), and not `.`/`..` --
/// checked before it is ever joined onto the runs root.
fn valid_run_id(run_id: &str) -> bool {
    !run_id.is_empty()
        && run_id != "."
        && run_id != ".."
        && !run_id.contains('/')
        && !run_id.contains('\\')
        && !run_id.contains(':')
}

/// Minimal [`DiagnosticRenderer`] the shell hands to `batch_document`/
/// `config_only_document` (both require one): every diagnostic's
/// `"rendered"` field carries its own stable `code` key rather than
/// localized prose. The frontend never reads this field -- it renders each
/// diagnostic from its own `code`+`params` through its own Fluent catalog
/// (spec 7/8.4's GUI-side split, T9/T10's territory) -- so this passthrough
/// is correct data, simply not prose.
struct ShellRenderer;

impl DiagnosticRenderer for ShellRenderer {
    fn diagnostic(&self, diagnostic: &Diagnostic) -> String {
        diagnostic.code.key().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use muxsmith_core::executor::spawn::FakeSpawner;
    use muxsmith_core::report::DiagCode;

    fn spec(dir: &Path, name: &str) -> JobSpec {
        JobSpec {
            argv: vec!["x".to_string()],
            output: dir.join(name),
        }
    }

    fn opts() -> QueueOpts {
        QueueOpts {
            jobs: 1,
            fail_fast: false,
        }
    }

    fn ctl(spec_count: usize) -> Arc<QueueControl> {
        QueueControl::new(spec_count, Arc::new(AtomicBool::new(false)))
    }

    fn running(control: &Arc<QueueControl>) -> RunSlot {
        RunSlot::Running(ActiveRun {
            ctl: Arc::clone(control),
        })
    }

    // -- Reservation: single-run invariant across the planning window ---------

    /// The reviewer-mandated mid-planning rejection: a second start_run
    /// arriving while the first is still planning (slot Reserved, no
    /// queue yet) must get run-already-active. The reservation stands in
    /// for the whole planning pass here: start_run's body between acquire
    /// and commit/drop is a straight-line sequence with no other slot
    /// access, so holding the reservation IS the mid-planning state.
    #[test]
    fn second_start_is_rejected_while_the_first_is_mid_planning() {
        let state = AppState::default();
        let first = Reservation::acquire(&state).unwrap();

        match Reservation::acquire(&state) {
            Err(e) => assert_eq!(e.code, "run-already-active"),
            Ok(_) => panic!("expected run-already-active, got Ok"),
        }

        drop(first);
    }

    /// A soft outcome (any early return in start_run) drops the
    /// reservation uncommitted; the slot must be free again -- no path may
    /// leak the reservation into a permanent run-already-active.
    #[test]
    fn reservation_clears_on_a_soft_outcome() {
        let state = AppState::default();
        let reservation = Reservation::acquire(&state).unwrap();
        drop(reservation);

        assert!(state.active.lock().unwrap().is_none());
        // And a fresh start_run can reserve again.
        Reservation::acquire(&state).unwrap();
    }

    #[test]
    fn second_start_is_rejected_while_a_queue_is_running_too() {
        let state = AppState::default();
        let control = ctl(1);
        Reservation::acquire(&state).unwrap().commit(control);

        match Reservation::acquire(&state) {
            Err(e) => assert_eq!(e.code, "run-already-active"),
            Ok(_) => panic!("expected run-already-active, got Ok"),
        }
    }

    /// commit installs Running (not Reserved) and survives the
    /// reservation's own drop; the runner thread's final finish_teardown
    /// then frees the slot -- the full lifecycle in slot terms.
    #[test]
    fn commit_promotes_the_reservation_and_finish_teardown_clears_it() {
        let state = AppState::default();
        let control = ctl(1);
        Reservation::acquire(&state)
            .unwrap()
            .commit(Arc::clone(&control));

        assert!(matches!(
            state.active.lock().unwrap().as_ref(),
            Some(RunSlot::Running(_))
        ));

        let dir = tempfile::tempdir().unwrap();
        let specs = vec![spec(dir.path(), "a.mkv")];
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        run_batch(&specs, &fake, opts(), &control, None, |_| {});
        finish_teardown(&state, |_| {});

        assert!(state.active.lock().unwrap().is_none());
        Reservation::acquire(&state).unwrap();
    }

    /// A cancel_run landing in the planning window sets the reservation's
    /// flag, and because start_run shares that same flag into
    /// QueueControl::new, the queue is born already-cancelled: every job
    /// finishes Cancelled without spawning. The cancel is honored, not
    /// lost.
    #[test]
    fn cancel_run_during_planning_reaches_the_later_queue() {
        let state = AppState::default();
        let reservation = Reservation::acquire(&state).unwrap();

        do_cancel_run(&state).unwrap();

        // What start_run does after planning: QueueControl::new with the
        // reservation's own flag.
        let control = QueueControl::new(1, reservation.cancel_flag());
        assert!(
            control.job_cancelled(0),
            "the planning-window cancel must already cover every job"
        );
    }

    #[test]
    fn cancel_job_during_planning_is_an_accepted_no_op() {
        let state = AppState::default();
        let _reservation = Reservation::acquire(&state).unwrap();

        do_cancel_job(&state, 0).unwrap();
    }

    // -- run_batch: event ordering ------------------------------------------

    #[test]
    fn run_batch_emits_started_output_finished_in_order() {
        let dir = tempfile::tempdir().unwrap();
        let specs = vec![spec(dir.path(), "a.mkv")];
        let fake = FakeSpawner::script(vec!["hello".to_string()], Some(0));
        let control = ctl(specs.len());

        let mut collected: Vec<JobEvent> = Vec::new();
        let (outcomes, logger) = run_batch(&specs, &fake, opts(), &control, None, |e| {
            collected.push(e.clone())
        });

        assert_eq!(outcomes.len(), 1);
        assert_eq!(
            outcomes[0].state,
            muxsmith_core::executor::job::JobState::Ok
        );
        assert!(logger.is_none());

        let kinds: Vec<&str> = collected
            .iter()
            .map(|e| match e {
                JobEvent::Started { .. } => "started",
                JobEvent::Output { .. } => "output",
                JobEvent::Finished { .. } => "finished",
                JobEvent::Progress { .. } => "progress",
                JobEvent::Warning { .. } => "warning",
                JobEvent::Error { .. } => "error",
            })
            .collect();
        assert_eq!(kinds, vec!["started", "output", "finished"]);
    }

    // -- finish_teardown: active flag clears after finish ---------------------

    #[test]
    fn finish_teardown_clears_the_slot_without_exiting_when_no_quit_is_pending() {
        let state = AppState::default();
        let control = ctl(1);
        *state.active.lock().unwrap() = Some(running(&control));

        let mut exits = 0;
        finish_teardown(&state, |_| exits += 1);

        assert!(state.active.lock().unwrap().is_none());
        assert_eq!(exits, 0, "a normal run end never exits the app");
    }

    // -- run_batch: joblog dir populated -------------------------------------

    #[test]
    fn run_batch_writes_job_log_files() {
        let runs_root = tempfile::tempdir().unwrap();
        let out_dir = tempfile::tempdir().unwrap();
        let specs = vec![spec(out_dir.path(), "a.mkv")];
        let fake = FakeSpawner::script(vec!["#GUI#progress 100%".to_string()], Some(0));
        let control = ctl(specs.len());
        let logger = RunLogger::create(runs_root.path(), "20260710-000000Z", &specs).unwrap();
        let dir = logger.dir().to_path_buf();

        let (_outcomes, logger) = run_batch(&specs, &fake, opts(), &control, Some(logger), |_| {});

        assert!(dir.join("job-0.json").exists());
        assert!(logger.is_some());
    }

    // -- D31: close decision + quit-after-finished ----------------------------

    #[test]
    fn close_decision_lets_an_idle_window_close_normally() {
        let state = AppState::default();
        assert_eq!(close_decision(&state), CloseDecision::Close);

        // The quit machinery is never engaged on this path: no flag was
        // set, so the consume fires nothing.
        let mut exits = 0;
        quit_if_requested(&state, |_| exits += 1);
        assert_eq!(exits, 0);
    }

    #[test]
    fn close_decision_confirms_while_planning_and_while_running() {
        let state = AppState::default();
        let reservation = Reservation::acquire(&state).unwrap();
        assert_eq!(close_decision(&state), CloseDecision::ConfirmAbort);

        let control = ctl(1);
        reservation.commit(Arc::clone(&control));
        assert_eq!(close_decision(&state), CloseDecision::ConfirmAbort);
    }

    /// The D31 happy path: Yes on the dialog while the queue runs cancels
    /// the batch without exiting; the runner's teardown completion then
    /// exits exactly once, and the consumed request can never fire again.
    #[test]
    fn quit_flag_plus_teardown_completion_exits_exactly_once() {
        let state = AppState::default();
        let control = ctl(1);
        *state.active.lock().unwrap() = Some(running(&control));

        let mut exits: Vec<i32> = Vec::new();
        abort_and_quit(&state, |code| exits.push(code));
        assert!(exits.is_empty(), "exit must wait for teardown");
        assert!(control.job_cancelled(0), "Yes must cancel the batch");

        finish_teardown(&state, |code| exits.push(code));
        assert_eq!(exits, vec![0], "teardown completion exits exactly once");

        quit_if_requested(&state, |code| exits.push(code));
        assert_eq!(
            exits,
            vec![0],
            "the request is consumed; nothing fires twice"
        );
    }

    /// Yes clicked after the run already tore down (dialog was open while
    /// the last job finished): the slot is empty, teardown is complete
    /// (clearing is teardown's final step), so the exit fires immediately,
    /// still exactly once.
    #[test]
    fn abort_and_quit_exits_immediately_when_the_run_already_tore_down() {
        let state = AppState::default();

        let mut exits: Vec<i32> = Vec::new();
        abort_and_quit(&state, |code| exits.push(code));
        assert_eq!(exits, vec![0]);

        quit_if_requested(&state, |code| exits.push(code));
        assert_eq!(exits, vec![0], "consumed; no second exit");
    }

    /// The Reserved-but-not-yet-Running case (coordinator-mandated): Yes
    /// during planning cancels via the reservation's flag; when planning
    /// ends in a soft outcome (reservation dropped, no queue ever built),
    /// the soft path's own completion hook (quit_if_requested inside
    /// finish_without_queue) still exits.
    #[test]
    fn abort_and_quit_during_planning_exits_after_a_soft_outcome() {
        let state = AppState::default();
        let reservation = Reservation::acquire(&state).unwrap();
        let cancel = reservation.cancel_flag();

        let mut exits: Vec<i32> = Vec::new();
        abort_and_quit(&state, |code| exits.push(code));
        assert!(exits.is_empty(), "planning still in flight; no exit yet");
        assert!(
            cancel.load(Ordering::SeqCst),
            "the cancel must reach the reservation's flag (the future queue's batch flag)"
        );

        // Soft outcome: reservation dropped, then start_run's soft path
        // runs its completion hook (this is what finish_without_queue does
        // after emitting run-finished).
        drop(reservation);
        quit_if_requested(&state, |code| exits.push(code));
        assert_eq!(exits, vec![0]);
    }

    #[test]
    fn a_new_reservation_discards_a_stale_quit_request() {
        let state = AppState::default();
        state.quit_after_finished.store(true, Ordering::SeqCst);

        let _reservation = Reservation::acquire(&state).unwrap();

        let mut exits = 0;
        quit_if_requested(&state, |_| exits += 1);
        assert_eq!(
            exits, 0,
            "a stale quit request must not survive into a new run"
        );
    }

    // -- D31: dialog strings from the .ftl -------------------------------------

    #[test]
    fn close_abort_strings_resolve_from_the_ftl_catalog() {
        for key in [
            "close-abort-title",
            "close-abort-message",
            "close-abort-confirm",
            "close-abort-dismiss",
        ] {
            let value = ftl_message(key);
            assert_ne!(
                value, key,
                "{key} must resolve to a real message, not fall back to the key"
            );
            assert!(!value.is_empty());
        }
        // The reference wording (D31: mkvtoolnix-gui parity) is pinned so
        // an accidental .ftl edit that breaks the line-parser contract
        // (multiline, attributes) fails here instead of shipping a key as
        // the dialog title.
        assert_eq!(ftl_message("close-abort-title"), "Abort running jobs");
    }

    #[test]
    fn ftl_message_falls_back_to_the_key_and_never_prefix_matches() {
        assert_eq!(ftl_message("no-such-key"), "no-such-key");
        // A key that is a strict prefix of a real entry must not match it.
        assert_eq!(ftl_message("close-abort"), "close-abort");
    }

    // -- finalize_joblog ------------------------------------------------------

    #[test]
    fn finalize_joblog_none_is_unavailable() {
        let doc = serde_json::json!({});
        assert_eq!(finalize_joblog(None, &doc), JoblogStatus::Unavailable);
    }

    #[test]
    fn finalize_joblog_ok_is_complete() {
        let root = tempfile::tempdir().unwrap();
        let logger = RunLogger::create(root.path(), "20260710-000000Z", &[]).unwrap();
        let doc = serde_json::json!({"ok": true});
        assert_eq!(finalize_joblog(Some(logger), &doc), JoblogStatus::Complete);
    }

    #[test]
    fn finalize_joblog_write_failure_is_incomplete() {
        // A directory at summary.json's own path forces fs::write to fail
        // (EISDIR) regardless of uid -- root-safe, unlike a permission-bit
        // trick (mirrors executor::job's own delete_partial test pattern).
        let root = tempfile::tempdir().unwrap();
        let logger = RunLogger::create(root.path(), "20260710-000000Z", &[]).unwrap();
        std::fs::create_dir(logger.dir().join("summary.json")).unwrap();

        let doc = serde_json::json!({"ok": true});
        assert_eq!(
            finalize_joblog(Some(logger), &doc),
            JoblogStatus::Incomplete
        );
    }

    // -- started_at_from_run_id / valid_run_id ---------------------------------

    #[test]
    fn started_at_from_run_id_parses_the_fixed_prefix() {
        assert_eq!(
            started_at_from_run_id("20260710-153612Z"),
            Some("2026-07-10T15:36:12Z".to_string())
        );
    }

    #[test]
    fn started_at_from_run_id_parses_a_collision_suffixed_dir_name() {
        assert_eq!(
            started_at_from_run_id("20260710-153612Z-2"),
            Some("2026-07-10T15:36:12Z".to_string())
        );
    }

    #[test]
    fn started_at_from_run_id_rejects_garbage() {
        assert_eq!(started_at_from_run_id("not-a-run-id"), None);
        assert_eq!(started_at_from_run_id(""), None);
        assert_eq!(started_at_from_run_id("short"), None);
    }

    #[test]
    fn valid_run_id_accepts_a_plain_component() {
        assert!(valid_run_id("20260710-153612Z"));
        assert!(valid_run_id("20260710-153612Z-2"));
    }

    #[test]
    fn valid_run_id_rejects_traversal_separators_and_drive_prefixes() {
        // "C:"/"C:x" cover the Windows drive-prefix hazard: PathBuf::join
        // with a drive-prefixed component replaces the root there, so ':'
        // must be rejected on every platform (the check runs on Linux CI
        // but guards the Windows build).
        for bad in [
            "../etc/passwd",
            "a/b",
            "a\\b",
            "..",
            ".",
            "",
            "C:",
            "C:x",
            "a:b",
        ] {
            assert!(!valid_run_id(bad), "expected {bad:?} to be rejected");
        }
    }

    // -- list_runs_in -----------------------------------------------------------

    #[test]
    fn list_runs_in_skips_unreadable_dirs_and_sorts_newest_first() {
        let root = tempfile::tempdir().unwrap();

        let older = root.path().join("20260101-000000Z");
        std::fs::create_dir(&older).unwrap();
        std::fs::write(older.join("summary.json"), r#"{"summary":{"ok":1}}"#).unwrap();

        let newer = root.path().join("20260710-120000Z");
        std::fs::create_dir(&newer).unwrap();
        std::fs::write(newer.join("summary.json"), r#"{"summary":{"ok":2}}"#).unwrap();

        // No summary.json at all -- must be skipped, not fail the listing.
        std::fs::create_dir(root.path().join("20260705-000000Z")).unwrap();

        // A plain file, not a directory -- must be skipped.
        std::fs::write(root.path().join("not-a-run"), b"x").unwrap();

        let metas = list_runs_in(Some(root.path()));

        assert_eq!(metas.len(), 2);
        assert_eq!(metas[0].run_id, "20260710-120000Z", "newest first");
        assert_eq!(metas[1].run_id, "20260101-000000Z");
    }

    #[test]
    fn list_runs_in_returns_empty_when_runs_root_is_none_or_missing() {
        assert!(list_runs_in(None).is_empty());
        let root = tempfile::tempdir().unwrap();
        let missing = root.path().join("does-not-exist");
        assert!(list_runs_in(Some(&missing)).is_empty());
    }

    // -- get_job_log_in ---------------------------------------------------------

    #[test]
    fn get_job_log_in_reads_the_record() {
        let root = tempfile::tempdir().unwrap();
        let run_dir = root.path().join("20260710-000000Z");
        std::fs::create_dir(&run_dir).unwrap();
        std::fs::write(run_dir.join("job-0.json"), r#"{"index":0,"state":"ok"}"#).unwrap();

        let v = get_job_log_in(Some(root.path()), "20260710-000000Z", 0).unwrap();
        assert_eq!(v["state"], "ok");
    }

    #[test]
    fn get_job_log_in_rejects_traversal_run_ids() {
        let err = get_job_log_in(Some(Path::new("/tmp")), "../etc", 0).unwrap_err();
        assert_eq!(err.code, "invalid-run-id");
    }

    #[test]
    fn get_job_log_in_reports_unavailable_without_a_runs_root() {
        let err = get_job_log_in(None, "20260710-000000Z", 0).unwrap_err();
        assert_eq!(err.code, "job-log-unavailable");
    }

    #[test]
    fn get_job_log_in_reports_not_found_for_a_missing_record() {
        let root = tempfile::tempdir().unwrap();
        let err = get_job_log_in(Some(root.path()), "20260710-000000Z", 0).unwrap_err();
        assert_eq!(err.code, "job-log-not-found");
    }

    // -- do_cancel_run / do_cancel_job -------------------------------------------

    #[test]
    fn do_cancel_run_errors_when_idle() {
        let state = AppState::default();
        let err = do_cancel_run(&state).unwrap_err();
        assert_eq!(err.code, "no-active-run");
    }

    #[test]
    fn do_cancel_run_cancels_the_active_batch() {
        let state = AppState::default();
        let control = ctl(1);
        *state.active.lock().unwrap() = Some(running(&control));

        do_cancel_run(&state).unwrap();

        assert!(control.job_cancelled(0));
    }

    #[test]
    fn do_cancel_job_errors_when_idle() {
        let state = AppState::default();
        let err = do_cancel_job(&state, 0).unwrap_err();
        assert_eq!(err.code, "no-active-run");
    }

    #[test]
    fn do_cancel_job_cancels_only_the_targeted_job() {
        let state = AppState::default();
        let control = ctl(2);
        *state.active.lock().unwrap() = Some(running(&control));

        do_cancel_job(&state, 0).unwrap();

        assert!(control.job_cancelled(0));
        assert!(!control.job_cancelled(1));
    }

    // -- ShellRenderer ------------------------------------------------------------

    #[test]
    fn shell_renderer_passes_through_the_diag_code_key() {
        let d = Diagnostic::error(DiagCode::ParseError, "x");
        assert_eq!(ShellRenderer.diagnostic(&d), "parse-error");
    }
}
