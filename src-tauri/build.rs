//! Tauri build-time codegen: embeds `tauri.conf.json` and the capability
//! set into the `muxsmith-gui` binary (spec: Task 4 scaffold).

fn main() {
    tauri_build::build()
}
