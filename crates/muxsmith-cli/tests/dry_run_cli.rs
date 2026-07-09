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

/// Bug A (spec 5.5): dry-run must run the config-time validate pass before
/// planning. A profile with an unbalanced-paren `input.pattern` regex must
/// surface `invalid-regex` (not silently plan an empty, "clean" batch) and
/// exit 2, not 0.
#[test]
fn dry_run_surfaces_config_time_invalid_regex() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .args(["dry-run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .arg("--json")
        .output()
        .unwrap();

    assert_eq!(
        out.status.code(),
        Some(2),
        "stdout: {}, stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).expect("json report");
    let config_diags = report["config_diagnostics"]
        .as_array()
        .expect("config_diagnostics array");
    assert!(
        config_diags.iter().any(|d| d["code"] == "invalid-regex"),
        "expected an invalid-regex diagnostic, got: {report}"
    );
}

/// Bug F (spec 5.2): `--json` must attach a rendered message to every
/// diagnostic it emits, config-time, batch-level, and per-file alike.
#[test]
fn dry_run_json_diagnostics_all_carry_rendered_text() {
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
    // Not matched by input.pattern -> IgnoredFile (batch-level, info).
    std::fs::write(dir.path().join("extra.mkv"), b"not a real mkv").unwrap();

    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        // rule 0 matches the audio track; rule 1 matches nothing (no video
        // track in the fixture) -> per-file MissingTrack; rule 2 references
        // an unknown property -> config-time UnknownProperty.
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  - match: { exact: { type: audio } }\n  - match: { exact: { type: video } }\n  - match: { exact: { bogus_property: 1 } }\n",
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

    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "json report: {e}, stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    });

    let mut all_diags: Vec<&serde_json::Value> = Vec::new();
    all_diags.extend(report["config_diagnostics"].as_array().unwrap());
    all_diags.extend(report["batch_diagnostics"].as_array().unwrap());
    for f in report["files"].as_array().unwrap() {
        all_diags.extend(f["diagnostics"].as_array().unwrap());
    }
    assert!(
        all_diags.len() >= 3,
        "expected config, batch, and per-file diagnostics, got: {report}"
    );
    for d in &all_diags {
        let rendered = d["rendered"]
            .as_str()
            .unwrap_or_else(|| panic!("diagnostic missing rendered field: {d}"));
        assert!(!rendered.is_empty(), "empty rendered text for {d}");
    }
}
