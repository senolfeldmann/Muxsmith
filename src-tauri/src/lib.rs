//! Muxsmith GUI shell (Tauri 2). This crate wires the Tauri application
//! (window, plugins, IPC commands) around the same `muxsmith-core`
//! planning/execution engine the CLI uses; no muxing or planning logic
//! lives here (D23: "commands + job event stream, no logic").
//!
//! Two tasks contribute to this shell, sharing one [`AppState`]: T7
//! (D23/D27/D28) adds the read-only IPC surface -- `validate_profile`,
//! `dry_run`, `identify`, `detect_mkvmerge` -- plus app-settings
//! persistence (`get_settings`/`set_settings`); T8 (D23/D31) adds the run
//! lifecycle (`start_run`/`cancel_run`/`cancel_job`/`list_runs`/
//! `get_job_log`, the `muxsmith://job-event`/`muxsmith://run-finished`
//! window events, and the close-with-active-run confirmation dialog).
//!
//! Not `#![deny(missing_docs)]`: `src-tauri` is a bin-shaped crate (the
//! `[lib]` target exists only so Tauri's mobile entry point can call into
//! it), unlike `muxsmith-core`/`muxsmith-cli`. Public items are still
//! documented.

mod error;
mod run;
mod settings;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

use muxsmith_core::capability::runtime::{MIN_SUPPORTED, Mkvmerge};
use muxsmith_core::identify::{Identification, IdentifyCache, LiveIdentifier};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report;
use serde::Serialize;
use tauri::State;

use error::IpcError;
use run::RunSlot;
use settings::AppSettings;

/// Satisfies [`report::json::config_only_document`]/[`report::json::batch_document`]'s
/// mandatory [`report::json::DiagnosticRenderer`] parameter without the
/// shell asserting any prose of its own (spec 8.4; this task's "no
/// user-facing prose in the shell" constraint). The frontend re-renders
/// every diagnostic from its own `code`/`params` through its own Fluent
/// bundle -- `locales/` is shared by the CLI (fluent-rs) and the frontend
/// (`@fluent/bundle`) alike (spec section 7's directory layout) -- and
/// never reads this field, so this renderer echoes the diagnostic's own
/// code key rather than composing a translated message the shell has no
/// business producing.
///
/// `pub(crate)`: [`run`]'s `start_run` builds the same document shapes for
/// the run-lifecycle path and used to carry an identical duplicate
/// definition of this same zero-field unit struct; this is the one
/// surviving copy.
pub(crate) struct ShellRenderer;

impl report::json::DiagnosticRenderer for ShellRenderer {
    fn diagnostic(&self, diagnostic: &report::Diagnostic) -> String {
        diagnostic.code.key().to_string()
    }
}

/// Tauri-managed application state (D23), unified across both tasks that
/// contribute IPC commands to this shell: T7's settings persistence and
/// T8's run lifecycle. There is exactly one `AppState` value, `.manage`d
/// once in [`run`] -- Tauri resolves `State<AppState>` for every command
/// regardless of which task's module declares it, so the fields cannot be
/// split across two managed structs.
pub struct AppState {
    /// The resolved settings file path (T7, D27); `None` if the platform
    /// config directory itself could not be resolved (e.g. no `HOME`), in
    /// which case every settings-touching command fails with
    /// `settings-dir-unavailable`.
    settings_path: Option<PathBuf>,
    /// The run-lifecycle single-run slot (T8, D23): at most one run may be
    /// active at a time; this is the source of truth `start_run`/
    /// `cancel_run`/`cancel_job`/[`run::on_close_requested`] all read or
    /// write through.
    active: Mutex<Option<RunSlot>>,
    /// D31 quit-after-finished (T8): set when the user confirms the
    /// close-with-active-run dialog; consumed (exactly once, via `swap`)
    /// by whichever completion path finishes last -- the runner's
    /// teardown, a soft outcome's synchronous finish, or the dialog
    /// callback itself when the run already tore down while the dialog
    /// was open.
    quit_after_finished: AtomicBool,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            settings_path: settings::settings_path(),
            active: Mutex::new(None),
            quit_after_finished: AtomicBool::new(false),
        }
    }
}

/// Reads current settings from the resolved settings file path, or
/// [`AppSettings::default`] on a missing file (D27, first run). `path` is
/// `AppState::settings_path` (or a clone of it, inside a blocking task);
/// `None` -- the platform config directory itself was unresolvable -- fails
/// with `settings-dir-unavailable`, any other I/O or parse failure with the
/// mapped [`settings::SettingsError`]. A free function (not an `AppState`
/// method) so the async commands can move a cloned path into their
/// `spawn_blocking` closure and read settings THERE, off the webview
/// thread, where `State`'s borrow cannot follow.
fn load_settings_from(path: Option<&Path>) -> Result<AppSettings, IpcError> {
    let path = path.ok_or_else(|| IpcError::new("settings-dir-unavailable"))?;
    settings::load(path).map_err(IpcError::from)
}

impl AppState {
    /// [`load_settings_from`] on this state's resolved settings path.
    fn load_settings(&self) -> Result<AppSettings, IpcError> {
        load_settings_from(self.settings_path.as_deref())
    }

    /// Writes `settings` to disk (D27; `recent_profiles` capped at write
    /// time, `settings::save`'s own invariant). Same `settings-dir-unavailable`
    /// failure mode as [`Self::load_settings`].
    fn save_settings(&self, settings: &AppSettings) -> Result<(), IpcError> {
        let path = self
            .settings_path
            .as_deref()
            .ok_or_else(|| IpcError::new("settings-dir-unavailable"))?;
        settings::save(path, settings).map_err(IpcError::from)
    }
}

/// Detected mkvmerge info for the frontend (D28): the resolved path, the
/// version found (`"{major}.{minor}"`), and whether it clears
/// [`MIN_SUPPORTED`]. `Ok` is the command's only success shape; a missing
/// or too-old mkvmerge is an [`IpcError`] (via the `RuntimeError` mapping
/// in `crate::error`, distinguishing `mkvmerge-not-found` from
/// `mkvmerge-too-old`), never an `Ok` with `meets_minimum: false` --
/// `Mkvmerge::detect` already refuses a too-old candidate outright (D28),
/// so `meets_minimum` here is a defensive re-check of that same fact, not
/// the primary signal the frontend branches on.
#[derive(Debug, Clone, PartialEq, Serialize)]
struct MkvmergeInfo {
    /// The resolved executable path (override, PATH, or a platform
    /// candidate).
    path: String,
    /// The found version, formatted `"{major}.{minor}"`.
    version: String,
    /// Whether the found version clears [`MIN_SUPPORTED`] (D28).
    meets_minimum: bool,
}

/// `validate_profile`'s command body (testable without a Tauri runtime):
/// static, mkvmerge-free validation (spec 5.4) of the profile at `path`,
/// mirroring `muxsmith-cli`'s `validate::collect` (load, then
/// `validate::validate` + `lint::provable_overlaps`), but rendered through
/// [`report::json::config_only_document`] per this task's brief rather
/// than the CLI `validate` subcommand's flatter `{diagnostics: [...]}`
/// shape -- every GUI report command returns the SAME document shape, so
/// the frontend has one rendering path, not one per command. `mkvmerge_found`
/// is always absent from the result: this command never touches mkvmerge.
fn validate_profile_body(path: &Path) -> serde_json::Value {
    let diags = match load::from_file(path) {
        Err(d) => vec![d],
        Ok(profile) => {
            let mut diags = validate::validate(&profile);
            diags.extend(lint::provable_overlaps(&profile));
            diags
        }
    };
    report::json::config_only_document(&diags, None, &ShellRenderer)
}

/// `dry_run`'s command body (testable without a Tauri runtime): mirrors
/// `muxsmith-cli`'s `dry-run` orchestration (load -> config-time validate
/// -> resolve mkvmerge -> `list_languages` -> `plan_batch` -> assemble
/// document, `muxsmith-cli/src/commands/dry_run.rs`) with one deliberate
/// substitution -- [`Mkvmerge::detect`] (override + PATH + platform
/// candidates, D28) in place of the CLI's PATH-only [`Mkvmerge::locate`] --
/// so a GUI user's configured mkvmerge override (`AppSettings::mkvmerge_path`)
/// is honored here exactly as it is by `detect_mkvmerge`, not only at
/// first-run detection time.
///
/// `mkvmerge_found` in the returned document: absent on a profile-load
/// failure (the lookup never ran); `Some(false)` when [`Mkvmerge::detect`]
/// itself fails for ANY reason, including `TooOld` -- a too-old mkvmerge is
/// exactly as unusable for planning as a missing one, and the detailed
/// distinction (with `found`/`minimum`) is `detect_mkvmerge`'s job, not
/// this document's; `Some(true)` when a usable mkvmerge was resolved but
/// the separate `list_languages` query failed (a broken installation).
fn dry_run_body(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    mkvmerge_override: Option<&Path>,
) -> serde_json::Value {
    let profile = match load::from_file(profile_path) {
        Ok(p) => p,
        Err(d) => return report::json::config_only_document(&[d], None, &ShellRenderer),
    };

    let mut config_diags = validate::validate(&profile);
    config_diags.extend(lint::provable_overlaps(&profile));

    let mkv = match Mkvmerge::detect(mkvmerge_override) {
        Ok(m) => m,
        Err(_) => {
            return report::json::config_only_document(&config_diags, Some(false), &ShellRenderer);
        }
    };
    let lang = match mkv.list_languages() {
        Ok(l) => l,
        Err(_) => {
            return report::json::config_only_document(&config_diags, Some(true), &ShellRenderer);
        }
    };

    // No natural "current directory" for a bundled desktop app, but kept
    // for parity with the CLI's own fallback (dry_run.rs); in practice the
    // batch view (T10) always supplies an explicit source directory via
    // its dir picker before calling this command.
    let source_dir = source.unwrap_or_else(|| PathBuf::from("."));
    let run = RunInputs {
        source: source_dir,
        output,
        on_collision: None,
    };
    let mut ident = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &mkv,
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang);

    report::json::batch_document(&config_diags, &batch, &ShellRenderer)
}

/// The `identify` command's JSON shape: mirrors `muxsmith-cli`'s
/// `print_identify_json` field-for-field (spec 7). Not hoisted into
/// `report::json` alongside `config_only_document`/`batch_document`/
/// `run_document`: T2 scoped that hoist to those three specifically, and
/// this shape is a handful of lines of leaf-level JSON construction, not
/// orchestration logic -- below the threshold that would justify a new
/// core module for one GUI command and one CLI command to share.
fn identify_document(id: &Identification) -> serde_json::Value {
    let tracks: Vec<serde_json::Value> = id
        .tracks
        .iter()
        .map(|t| serde_json::json!({ "id": t.id, "type": t.kind, "codec": t.codec }))
        .collect();
    serde_json::json!({
        "file_name": id.file_name,
        "identification_format_version": id.format_version,
        "identifiable": id.is_identifiable(),
        "tracks": tracks,
    })
}

/// `identify`'s command body (testable without a Tauri runtime): resolves
/// mkvmerge via [`Mkvmerge::detect`] (honoring the settings override, same
/// as [`dry_run_body`]) and identifies `file`. Unlike `validate_profile`/
/// `dry_run`, there is no "always returns a document" contract here (spec
/// 5.5's superset guarantee is dry-run's alone), so a genuine failure is an
/// [`IpcError`] rather than folded into a placeholder document.
fn identify_body(
    file: &Path,
    mkvmerge_override: Option<&Path>,
) -> Result<serde_json::Value, IpcError> {
    let mkv = Mkvmerge::detect(mkvmerge_override)?;
    let mut cache = IdentifyCache::new();
    let id = cache
        .get_or_identify(&mkv, file)
        .map_err(|e| IpcError::from(e).with("file", file.display().to_string()))?;
    Ok(identify_document(id))
}

/// `detect_mkvmerge`'s command body (testable without a Tauri runtime,
/// D28): resolves mkvmerge via the override/PATH/platform-candidate ladder
/// and reports its path, version, and floor status. A too-old or
/// unreachable mkvmerge is an [`IpcError`] (see [`MkvmergeInfo`]'s doc),
/// not a partial `Ok`.
fn detect_mkvmerge_body(mkvmerge_override: Option<&Path>) -> Result<MkvmergeInfo, IpcError> {
    let mkv = Mkvmerge::detect(mkvmerge_override)?;
    let pair = mkv.version_pair()?;
    Ok(MkvmergeInfo {
        path: mkv.path().display().to_string(),
        version: format!("{}.{}", pair.0, pair.1),
        meets_minimum: pair >= MIN_SUPPORTED,
    })
}

/// `validate_profile` IPC command (D23, this task's brief): static
/// validation only, never touches mkvmerge. Runs on a blocking task
/// (`tauri::async_runtime::spawn_blocking`) since [`validate_profile_body`]
/// touches the filesystem; the webview thread never blocks on it. The
/// `Err` case here is the async task itself panicking (a bug, not an
/// expected outcome): every "expected" failure -- a missing or malformed
/// profile -- is folded into the returned document by
/// [`validate_profile_body`] instead.
#[tauri::command]
async fn validate_profile(path: String) -> Result<serde_json::Value, IpcError> {
    tauri::async_runtime::spawn_blocking(move || validate_profile_body(Path::new(&path)))
        .await
        .map_err(|e| IpcError::new("internal-task-failed").with("detail", e.to_string()))
}

/// `dry_run` IPC command (D23, D28): on a blocking task (never on the
/// webview thread), reads the mkvmerge override from settings and runs
/// [`dry_run_body`]. The settings read (filesystem I/O) lives INSIDE the
/// `spawn_blocking` closure with the body itself -- `State`'s borrow
/// cannot move into the closure, so the resolved settings *path* is cloned
/// in and loaded there. Like `validate_profile`, the outer `map_err` case
/// means the blocking task itself panicked; every expected planning
/// outcome is a document, and a settings failure is the closure's own
/// [`IpcError`].
#[tauri::command]
async fn dry_run(
    state: State<'_, AppState>,
    profile: String,
    source: Option<String>,
    output: Option<String>,
) -> Result<serde_json::Value, IpcError> {
    let settings_path = state.settings_path.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mkvmerge_override = load_settings_from(settings_path.as_deref())?.mkvmerge_path;
        Ok(dry_run_body(
            Path::new(&profile),
            source.map(PathBuf::from),
            output.map(PathBuf::from),
            mkvmerge_override.as_deref().map(Path::new),
        ))
    })
    .await
    .map_err(|e| IpcError::new("internal-task-failed").with("detail", e.to_string()))?
}

/// `identify` IPC command (D23, D28): on a blocking task (spawns mkvmerge;
/// never on the webview thread), reads the mkvmerge override from settings
/// and runs [`identify_body`]. Settings read inside the closure for the
/// same reason as [`dry_run`].
#[tauri::command]
async fn identify(state: State<'_, AppState>, file: String) -> Result<serde_json::Value, IpcError> {
    let settings_path = state.settings_path.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mkvmerge_override = load_settings_from(settings_path.as_deref())?.mkvmerge_path;
        identify_body(
            Path::new(&file),
            mkvmerge_override.as_deref().map(Path::new),
        )
    })
    .await
    .map_err(|e| IpcError::new("internal-task-failed").with("detail", e.to_string()))?
}

/// `detect_mkvmerge` IPC command (D28). Declared `async` and run on a
/// blocking task, deviating from this task's brief (whose terse interface
/// sketch omits `async`/`Result` throughout, e.g. `fn get_settings(state)
/// -> AppSettings`, and cannot be taken as an exact signature): unlike
/// `get_settings`/`set_settings` (a local JSON file, effectively
/// instantaneous), [`detect_mkvmerge_body`] spawns up to several `mkvmerge
/// --version` subprocesses in sequence (the override, then PATH, then
/// every platform candidate). Tauri v2 runs a non-`async` command on the
/// application's main thread unless `#[tauri::command(async)]` is added, so
/// a plain `fn` here would stall the webview for however long that probing
/// takes -- exactly the "webview must never block" constraint this task's
/// context calls out for `dry_run`/`identify`. This command is called on
/// every GUI startup (T9's first-run flow), making that stall guaranteed,
/// not just possible.
///
/// Settings read inside the closure for the same reason as [`dry_run`].
#[tauri::command]
async fn detect_mkvmerge(state: State<'_, AppState>) -> Result<MkvmergeInfo, IpcError> {
    let settings_path = state.settings_path.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mkvmerge_override = load_settings_from(settings_path.as_deref())?.mkvmerge_path;
        detect_mkvmerge_body(mkvmerge_override.as_deref().map(Path::new))
    })
    .await
    .map_err(|e| IpcError::new("internal-task-failed").with("detail", e.to_string()))?
}

/// `get_settings` IPC command (D27). Plain local JSON file I/O, no
/// subprocess; kept a non-async, main-thread command like Tauri's own
/// trivial-state-access examples.
#[tauri::command]
fn get_settings(state: State<AppState>) -> Result<AppSettings, IpcError> {
    state.load_settings()
}

/// `set_settings` IPC command (D27). Writes the full settings object the
/// frontend hands over; [`settings::save`] caps `recent_profiles` at write
/// time regardless of what the caller sent.
#[tauri::command]
fn set_settings(state: State<AppState>, settings: AppSettings) -> Result<(), IpcError> {
    state.save_settings(&settings)
}

/// Builds and runs the Tauri application: registers the `dialog`,
/// `clipboard-manager`, `os`, and `fs` plugins (capabilities in
/// `capabilities/default.json` gate what each grants to the *frontend*; the
/// shell's own Rust-side dialog use in the private `run::on_close_requested`
/// bypasses the IPC permission layer and needs no capability entry), manages the
/// single unified [`AppState`], registers both tasks' IPC commands under
/// ONE `invoke_handler` (Tauri resolves `State<AppState>` once per managed
/// type, so the read-only/settings commands and the run-lifecycle commands
/// must share the same registration), wires the D31 close-with-active-run
/// confirmation, and launches the main window from `tauri.conf.json`.
///
/// The `os` plugin (T9, D28) is here rather than in `@tauri-apps/api`
/// (which this task's brief names): Tauri 2 moved OS/platform detection
/// out of the core API into this separate plugin (`@tauri-apps/plugin-os`
/// on the frontend side), so `FirstRun.vue`'s per-OS guidance needs it
/// registered like `dialog`/`clipboard-manager` are.
///
/// The `fs` plugin (T11, D30 gap closure: export a finished job's log as
/// text) is the write half of the history view's save-as flow -- the
/// `dialog` plugin's own `save()` only returns a user-picked path, it never
/// writes bytes. Paired deliberately with `dialog`, not `fs:default`: a
/// `save()` call adds its picked path to the fs plugin's own scope for the
/// rest of that session (Tauri's documented combo for this exact pattern),
/// so `fs:allow-write-text-file` alone (`capabilities/default.json`) is
/// sufficient -- no static path allowlist needed, and the frontend can
/// never write anywhere the user did not just pick via the OS dialog.
/// Chosen over a bespoke `#[tauri::command]` write for the same reason:
/// the plugin route inherits Tauri's own "this path came from a real save
/// dialog" trust chain end to end, where a custom command would accept any
/// frontend-supplied path and have to hand-roll that provenance check
/// itself.
///
/// # Panics
///
/// Panics if the Tauri runtime fails to launch. This mirrors the Tauri
/// scaffold default: a launch failure here means the webview/window
/// backend is unusable, which has no meaningful recovery.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            validate_profile,
            dry_run,
            identify,
            detect_mkvmerge,
            get_settings,
            set_settings,
            run::start_run,
            run::cancel_run,
            run::cancel_job,
            run::list_runs,
            run::get_job_log,
        ])
        .on_window_event(run::on_close_requested)
        .run(tauri::generate_context!())
        .expect("error while running muxsmith-gui");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    /// Mirrors `muxsmith-core`'s identical `fake_mkvmerge` test helper
    /// (`crates/muxsmith-core/tests/mkvmerge_runtime.rs`): a fake
    /// `mkvmerge` that answers `--version` with a fixed line and fails
    /// every other invocation, so the mkvmerge-resolution branches
    /// (`Mkvmerge::detect`) are tested deterministically without a real
    /// mkvmerge on the test machine.
    #[cfg(unix)]
    fn fake_mkvmerge(dir: &Path, version_line: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let script = dir.join("mkvmerge");
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '{version_line}'\n  exit 0\nfi\nexit 1\n"
            ),
        )
        .unwrap();
        let mut perms = std::fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script, perms).unwrap();
        // See the identical warm-up in muxsmith-core's fake_mkvmerge: a
        // freshly written+chmod'd script can transiently answer
        // ExecutableFileBusy under parallel test load.
        for attempt in 0.. {
            match Command::new(&script).arg("--version").output() {
                Ok(_) => break,
                Err(e) if e.kind() == std::io::ErrorKind::ExecutableFileBusy && attempt < 50 => {
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
                Err(e) => panic!("fake mkvmerge script at {script:?} never became runnable: {e}"),
            }
        }
        script
    }

    /// Like [`fake_mkvmerge`], but the script also appends one line to a
    /// counter file on EVERY invocation (returned alongside the script
    /// path), so a test can assert exactly how many times the executable
    /// was spawned. Warm-up invocations are discarded by resetting the
    /// counter before returning. Mirrors the identical helper in
    /// `muxsmith-core/tests/mkvmerge_runtime.rs`.
    #[cfg(unix)]
    fn counting_fake_mkvmerge(dir: &Path, version_line: &str) -> (PathBuf, PathBuf) {
        use std::os::unix::fs::PermissionsExt;
        let script = dir.join("mkvmerge");
        let counter = dir.join("spawn-count");
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\necho run >> '{}'\nif [ \"$1\" = \"--version\" ]; then\n  echo '{version_line}'\n  exit 0\nfi\nexit 1\n",
                counter.display()
            ),
        )
        .unwrap();
        let mut perms = std::fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script, perms).unwrap();
        // Same ExecutableFileBusy warm-up as fake_mkvmerge (see its comment).
        for attempt in 0.. {
            match Command::new(&script).arg("--version").output() {
                Ok(_) => break,
                Err(e) if e.kind() == std::io::ErrorKind::ExecutableFileBusy && attempt < 50 => {
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
                Err(e) => panic!("fake mkvmerge script at {script:?} never became runnable: {e}"),
            }
        }
        std::fs::write(&counter, "").unwrap();
        (script, counter)
    }

    #[cfg(unix)]
    fn spawn_count(counter: &Path) -> usize {
        std::fs::read_to_string(counter)
            .unwrap_or_default()
            .lines()
            .count()
    }

    fn real_mkvmerge_available() -> bool {
        Command::new("mkvmerge")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn minimal_profile_yaml() -> &'static str {
        "profile_version: 1\ninput: { pattern: '.*', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n"
    }

    // --- validate_profile_body ---

    #[test]
    fn validate_profile_body_reports_load_failure_with_no_mkvmerge_key() {
        let dir = tempfile::tempdir().unwrap();
        let doc = validate_profile_body(&dir.path().join("missing.yaml"));
        assert!(doc["files"].as_array().unwrap().is_empty());
        assert!(!doc["config_diagnostics"].as_array().unwrap().is_empty());
        assert!(doc.get("mkvmerge_found").is_none());
    }

    #[test]
    fn validate_profile_body_reports_validate_diagnostics_for_a_loadable_invalid_profile() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("p.yaml");
        std::fs::write(
            &path,
            "profile_version: 1\ninput: { pattern: '.*', extensions: [mkv] }\ntracks:\n  rules: []\n",
        )
        .unwrap();

        let doc = validate_profile_body(&path);
        let codes: Vec<&str> = doc["config_diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["code"].as_str().unwrap())
            .collect();
        assert!(codes.contains(&"no-track-rules"), "codes: {codes:?}");
        assert!(doc.get("mkvmerge_found").is_none());
    }

    // --- dry_run_body ---

    #[test]
    fn dry_run_body_load_failure_mirrors_config_only_document_with_no_mkvmerge_key() {
        let dir = tempfile::tempdir().unwrap();
        let doc = dry_run_body(&dir.path().join("missing.yaml"), None, None, None);
        assert!(doc.get("mkvmerge_found").is_none());
        assert!(doc["files"].as_array().unwrap().is_empty());
    }

    #[test]
    #[cfg(unix)]
    fn dry_run_body_broken_mkvmerge_override_sets_mkvmerge_found_false() {
        let dir = tempfile::tempdir().unwrap();
        let profile_path = dir.path().join("p.yaml");
        std::fs::write(&profile_path, minimal_profile_yaml()).unwrap();
        let missing_override = dir.path().join("no-such-binary");

        let doc = dry_run_body(&profile_path, None, None, Some(&missing_override));
        assert_eq!(doc["mkvmerge_found"], serde_json::json!(false));
        assert!(doc["files"].as_array().unwrap().is_empty());
    }

    #[test]
    #[cfg(unix)]
    fn dry_run_body_too_old_mkvmerge_also_sets_mkvmerge_found_false() {
        let dir = tempfile::tempdir().unwrap();
        let profile_path = dir.path().join("p.yaml");
        std::fs::write(&profile_path, minimal_profile_yaml()).unwrap();
        let override_path = fake_mkvmerge(dir.path(), "mkvmerge v50.0.0 ('Old') 64-bit");

        let doc = dry_run_body(&profile_path, None, None, Some(&override_path));
        // TooOld folds into the same "not usable" bucket as NotFound (this
        // module's documented design decision, see dry_run_body's doc).
        assert_eq!(doc["mkvmerge_found"], serde_json::json!(false));
    }

    /// The "mkvmerge found but querying it failed" branch (a broken
    /// installation): the fake answers `--version` (so `detect` succeeds)
    /// but fails `--list-languages`, so planning never runs and the
    /// config-only document must say `mkvmerge_found: true` -- the binary
    /// WAS found, distinguishing this from the not-found/too-old branch.
    #[test]
    #[cfg(unix)]
    fn dry_run_body_query_failure_after_successful_detect_sets_mkvmerge_found_true() {
        let dir = tempfile::tempdir().unwrap();
        let profile_path = dir.path().join("p.yaml");
        std::fs::write(&profile_path, minimal_profile_yaml()).unwrap();
        let override_path = fake_mkvmerge(dir.path(), "mkvmerge v123.4.5 ('Broken') 64-bit");

        let doc = dry_run_body(&profile_path, None, None, Some(&override_path));
        assert_eq!(doc["mkvmerge_found"], serde_json::json!(true));
        assert!(doc["files"].as_array().unwrap().is_empty());
        assert!(doc["batch_diagnostics"].as_array().unwrap().is_empty());
        assert!(doc["suggestions"].as_array().unwrap().is_empty());
    }

    #[test]
    fn dry_run_body_full_success_path_with_real_mkvmerge() {
        if !real_mkvmerge_available() {
            eprintln!("mkvmerge not found; skipping");
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        let wav = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../crates/muxsmith-core/tests/fixtures/seeds/tone.wav"
        );
        let media = dir.path().join("sample.mkv");
        let ok = Command::new("mkvmerge")
            .args(["-q", "-o"])
            .arg(&media)
            .args(["--language", "0:eng"])
            .arg(wav)
            .status()
            .unwrap()
            .success();
        assert!(ok, "mkvmerge failed to build the fixture");

        let profile_path = dir.path().join("p.yaml");
        std::fs::write(&profile_path, minimal_profile_yaml()).unwrap();

        let doc = dry_run_body(
            &profile_path,
            Some(dir.path().to_path_buf()),
            Some(dir.path().join("out")),
            None,
        );
        assert!(doc.get("mkvmerge_found").is_none(), "doc: {doc}");
        let files = doc["files"].as_array().unwrap();
        assert_eq!(files.len(), 1, "doc: {doc}");
    }

    // --- identify_body ---

    /// A nonexistent OVERRIDE path is `RuntimeError::Spawn`, not `NotFound`:
    /// `Mkvmerge::detect`'s override branch calls `enforce_floor` directly
    /// (not through `locate()`, which is the only place that remaps a
    /// `Spawn`-from-ENOENT into `NotFound`, and only for its own PATH
    /// probe) -- "a broken override hard-fails by design" (this task's
    /// context notes). `NotFound` itself is exercised directly against the
    /// `RuntimeError` mapping in `crate::error`'s own tests; reproducing a
    /// genuine ladder-exhausted `NotFound` here would mean hiding the real
    /// `mkvmerge` from PATH process-globally, which is unsafe to do inside
    /// a parallel test binary.
    #[test]
    #[cfg(unix)]
    fn identify_body_broken_override_maps_to_mkvmerge_spawn_failed() {
        let dir = tempfile::tempdir().unwrap();
        let missing_override = dir.path().join("no-such-binary");
        let target = dir.path().join("video.mkv");

        let err = identify_body(&target, Some(&missing_override)).unwrap_err();
        assert_eq!(err.code, "mkvmerge-spawn-failed");
    }

    #[test]
    fn identify_body_full_success_path_with_real_mkvmerge() {
        if !real_mkvmerge_available() {
            eprintln!("mkvmerge not found; skipping");
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        let wav = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../crates/muxsmith-core/tests/fixtures/seeds/tone.wav"
        );
        let media = dir.path().join("sample.mkv");
        let ok = Command::new("mkvmerge")
            .args(["-q", "-o"])
            .arg(&media)
            .args(["--language", "0:eng"])
            .arg(wav)
            .status()
            .unwrap()
            .success();
        assert!(ok, "mkvmerge failed to build the fixture");

        let doc = identify_body(&media, None).expect("identify");
        assert_eq!(doc["identifiable"], serde_json::json!(true));
        assert_eq!(doc["tracks"].as_array().unwrap().len(), 1);
        assert_eq!(doc["tracks"][0]["type"], serde_json::json!("audio"));
    }

    // --- detect_mkvmerge_body ---

    /// See `identify_body_broken_override_maps_to_mkvmerge_spawn_failed`'s
    /// doc for why a nonexistent override maps to `mkvmerge-spawn-failed`,
    /// not `mkvmerge-not-found`.
    #[test]
    #[cfg(unix)]
    fn detect_mkvmerge_body_broken_override_maps_to_mkvmerge_spawn_failed() {
        let dir = tempfile::tempdir().unwrap();
        let err = detect_mkvmerge_body(Some(&dir.path().join("no-such-binary"))).unwrap_err();
        assert_eq!(err.code, "mkvmerge-spawn-failed");
    }

    #[test]
    #[cfg(unix)]
    fn detect_mkvmerge_body_too_old_carries_found_and_minimum() {
        let dir = tempfile::tempdir().unwrap();
        let override_path = fake_mkvmerge(dir.path(), "mkvmerge v50.0.0 ('Old') 64-bit");
        let err = detect_mkvmerge_body(Some(&override_path)).unwrap_err();
        assert_eq!(err.code, "mkvmerge-too-old");
        assert_eq!(err.params["minimum"], "86.0");
        assert!(err.params["found"].contains("v50.0.0"));
    }

    #[test]
    #[cfg(unix)]
    fn detect_mkvmerge_body_success_reports_version_and_meets_minimum() {
        let dir = tempfile::tempdir().unwrap();
        let override_path = fake_mkvmerge(dir.path(), "mkvmerge v123.4.5 ('New') 64-bit");
        let info = detect_mkvmerge_body(Some(&override_path)).expect("detect");
        assert_eq!(info.version, "123.4");
        assert!(info.meets_minimum);
        assert_eq!(info.path, override_path.display().to_string());
    }

    /// The whole `detect_mkvmerge` command body -- `Mkvmerge::detect` plus
    /// the `version_pair()` for `MkvmergeInfo` -- must spawn `mkvmerge
    /// --version` exactly ONCE: `detect`'s floor check already parsed the
    /// version, and the returned handle caches the pair (core contract,
    /// `mkvmerge_runtime.rs`), so this every-GUI-startup path never pays a
    /// second subprocess.
    #[test]
    #[cfg(unix)]
    fn detect_mkvmerge_body_spawns_version_exactly_once() {
        let dir = tempfile::tempdir().unwrap();
        let (script, counter) =
            counting_fake_mkvmerge(dir.path(), "mkvmerge v123.4.5 ('Counting') 64-bit");

        let info = detect_mkvmerge_body(Some(&script)).expect("detect");
        assert_eq!(info.version, "123.4");
        assert!(info.meets_minimum);
        assert_eq!(
            spawn_count(&counter),
            1,
            "detect_mkvmerge_body must spawn --version exactly once"
        );
    }

    #[test]
    fn detect_mkvmerge_body_finds_the_real_mkvmerge_when_available() {
        if !real_mkvmerge_available() {
            eprintln!("mkvmerge not found; skipping");
            return;
        }
        let info = detect_mkvmerge_body(None).expect("detect");
        assert!(info.meets_minimum, "info: {info:?}");
    }

    // --- AppState settings I/O ---

    #[test]
    fn get_settings_returns_defaults_on_first_run() {
        let dir = tempfile::tempdir().unwrap();
        let state = AppState {
            settings_path: Some(dir.path().join("settings.json")),
            ..AppState::default()
        };
        let settings = state.load_settings().expect("load");
        assert_eq!(settings, AppSettings::default());
    }

    #[test]
    fn set_settings_then_get_settings_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let state = AppState {
            settings_path: Some(dir.path().join("settings.json")),
            ..AppState::default()
        };
        let updated = AppSettings {
            default_jobs: 3,
            recent_profiles: vec!["/a.yaml".into()],
            ..AppSettings::default()
        };

        state.save_settings(&updated).expect("save");
        let loaded = state.load_settings().expect("load");
        assert_eq!(loaded, updated);
    }

    #[test]
    fn settings_commands_fail_distinctly_when_config_dir_is_unavailable() {
        let state = AppState {
            settings_path: None,
            ..AppState::default()
        };
        let err = state.load_settings().unwrap_err();
        assert_eq!(err.code, "settings-dir-unavailable");
        let err = state.save_settings(&AppSettings::default()).unwrap_err();
        assert_eq!(err.code, "settings-dir-unavailable");
    }
}
