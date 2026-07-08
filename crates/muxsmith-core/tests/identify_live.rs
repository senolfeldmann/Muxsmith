//! Live identification against real mkvmerge, muxing a fixture MKV from a
//! committed wav + srt seed (no ffmpeg dependency). Self-skips when mkvmerge
//! is unavailable.

use std::path::Path;
use std::process::Command;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::IdentifyCache;

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
}

// Mux the committed seeds into a temp .mkv via mkvmerge itself (spec 10).
fn make_sample(dir: &Path) -> std::path::PathBuf {
    let out = dir.join("sample.mkv");
    let wav = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/seeds/tone.wav");
    let srt = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/seeds/sub.srt");
    let status = Command::new("mkvmerge")
        .args(["-q", "-o"])
        .arg(&out)
        .args(["--language", "0:eng", "--track-name", "0:English"])
        .arg(wav)
        .args(["--language", "0:ger"])
        .arg(srt)
        .status()
        .expect("spawn mkvmerge to build fixture");
    assert!(status.success(), "mkvmerge failed to build the sample");
    out
}

#[test]
fn identifies_and_caches_a_real_file() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };
    let dir = tempfile::tempdir().unwrap();
    let sample = make_sample(dir.path());

    let mut cache = IdentifyCache::new();
    let id = cache.get_or_identify(&m, &sample).unwrap().clone();
    assert!(id.is_identifiable());
    assert!(id.tracks.iter().any(|t| t.kind == "audio"));
    assert!(id.tracks.iter().any(|t| t.kind == "subtitles"));

    // Second call is served from cache (same mtime/size); result is identical.
    let again = cache.get_or_identify(&m, &sample).unwrap();
    assert_eq!(&id, again);
}
