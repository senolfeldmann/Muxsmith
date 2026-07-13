//! Live `--gui-mode` grammar check against real mkvmerge (spec 6, D13, SI-3):
//! confirms `LiveSpawner` yields the observed `#GUI#progress NN%` line
//! grammar and a clean exit for a genuine mux, rather than trusting the
//! `spawn.rs` doc comment to stay accurate. Self-skips when mkvmerge is
//! unavailable, mirroring `identify_live.rs` / `command_integration.rs`.

use std::path::PathBuf;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::executor::spawn::{LiveSpawner, Spawn};

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
}

#[test]
fn live_gui_mode_progress_reaches_100_percent() {
    let Some(m) = mkvmerge() else {
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
        return;
    };

    let dir = tempfile::tempdir().unwrap();
    let srt = dir.path().join("seed.srt");
    std::fs::write(&srt, "1\n00:00:00,000 --> 00:00:01,000\nHello\n").unwrap();
    let out = dir.path().join("out.mkv");

    let spawner = LiveSpawner {
        mkvmerge: PathBuf::from(m.path()),
    };
    let argv = vec![
        "--output".to_string(),
        out.to_string_lossy().into_owned(),
        srt.to_string_lossy().into_owned(),
    ];
    let mut job = spawner.spawn(&argv).unwrap();

    let mut lines = Vec::new();
    while let Some(line) = job.next_line() {
        lines.push(line);
    }
    let exit = job.wait();

    assert_eq!(exit, Some(0), "lines: {lines:?}");
    assert!(
        lines.iter().any(|l| l.starts_with("#GUI#progress ")),
        "lines: {lines:?}"
    );
    assert_eq!(
        lines.iter().rfind(|l| l.starts_with("#GUI#progress ")),
        Some(&"#GUI#progress 100%".to_string()),
        "lines: {lines:?}"
    );
}
