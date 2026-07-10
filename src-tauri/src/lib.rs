//! Muxsmith GUI shell (Tauri 2). This crate wires the Tauri application
//! (window, plugins) around the same `muxsmith-core` planning/execution
//! engine the CLI uses; no muxing logic lives here.
//!
//! Not `#![deny(missing_docs)]`: `src-tauri` is a bin-shaped crate (the
//! `[lib]` target exists only so Tauri's mobile entry point can call into
//! it), unlike `muxsmith-core`/`muxsmith-cli`. Public items are still
//! documented.

pub mod error;
pub mod run;

/// Builds and runs the Tauri application: registers the `dialog` and
/// `clipboard-manager` plugins (capabilities in `capabilities/default.json`
/// gate what each grants), the run-lifecycle managed state and IPC
/// commands (D23), and launches the main window from `tauri.conf.json`.
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
        .plugin(tauri_plugin_clipboard_manager::init())
        // -- run lifecycle (D23, Task 8) --
        .manage(run::AppState::default())
        .invoke_handler(tauri::generate_handler![
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
