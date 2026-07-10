//! `muxsmith-gui` binary entry point: hands off to
//! [`muxsmith_gui_lib::run`] for all window/plugin setup.

// Suppresses the extra console window Windows would otherwise open
// alongside the webview in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    muxsmith_gui_lib::run();
}
