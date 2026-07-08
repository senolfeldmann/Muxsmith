//! End-to-end CLI test, gated on a real mkvmerge (self-skips otherwise).
//! Builds a fixture MKV via mkvmerge, writes a profile, runs `dry-run --json`,
//! and checks the batch report shape and exit code.

use std::process::Command;

use assert_cmd::cargo::CommandCargoExt;

fn have_mkvmerge() -> bool {
    Command::new("mkvmerge")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
fn dry_run_plans_a_single_file() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let wav = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/seeds/tone.wav"
    );
    let srt = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/seeds/sub.srt"
    );
    let media = dir.path().join("Show.S01E01.mkv");
    let ok = Command::new("mkvmerge")
        .args(["-q", "-o"])
        .arg(&media)
        .args(["--language", "0:eng"])
        .arg(wav)
        .args(["--language", "0:eng"])
        .arg(srt)
        .status()
        .unwrap()
        .success();
    assert!(ok);

    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .args(["dry-run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .args(["--output"])
        .arg(dir.path().join("out"))
        .arg("--json")
        .output()
        .unwrap();

    assert!(
        out.status.success(),
        "exit: {:?}, stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).expect("json report");
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    assert_eq!(report["files"][0]["identifier"], "S01E01");
    assert!(report["files"][0]["plan"].is_object());
}
