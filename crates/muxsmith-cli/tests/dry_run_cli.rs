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
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
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
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
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
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n    - match: { exact: { type: video } }\n    - match: { exact: { bogus_property: 1 } }\n",
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
    assert!(
        !report["config_diagnostics"].as_array().unwrap().is_empty(),
        "expected a non-empty config_diagnostics specifically, got: {report}"
    );
    for d in &all_diags {
        let rendered = d["rendered"]
            .as_str()
            .unwrap_or_else(|| panic!("diagnostic missing rendered field: {d}"));
        assert!(!rendered.is_empty(), "empty rendered text for {d}");
    }
}

/// Points a child process's PATH at a directory with no `mkvmerge`, so
/// `Mkvmerge::locate()` fails deterministically regardless of whether the
/// real mkvmerge is installed on the machine running the test. Confirmed by
/// hand (see F1 fix report) that `Command::new("mkvmerge")` under an
/// overridden child `PATH` fails with a spawn error, which `locate()` maps
/// to `RuntimeError::NotFound`.
fn empty_path_dir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

/// Finding 1: dry-run must stay a strict superset of `validate` (spec 5.5)
/// even when mkvmerge cannot be located. A profile with an unbalanced-paren
/// `input.pattern` regex must still surface `config_diagnostics` (containing
/// `invalid-regex`) in the `--json` report, with `files`/`batch_diagnostics`/
/// `suggestions` empty (planning never ran) and a `mkvmerge_found: false`
/// marker; exit code stays 2. Does not depend on `have_mkvmerge()`: the
/// no-mkvmerge condition is forced via PATH, not by the test environment's
/// actual mkvmerge presence.
#[test]
fn dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing() {
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let no_mkvmerge_path = empty_path_dir();
    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .env("PATH", no_mkvmerge_path.path())
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
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "json report: {e}, stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    let config_diags = report["config_diagnostics"]
        .as_array()
        .expect("config_diagnostics array");
    assert!(
        config_diags.iter().any(|d| d["code"] == "invalid-regex"),
        "expected an invalid-regex diagnostic, got: {report}"
    );
    for d in config_diags {
        assert!(
            d["rendered"].as_str().is_some_and(|s| !s.is_empty()),
            "config diagnostic missing rendered text: {d}"
        );
    }
    assert_eq!(report["files"].as_array().unwrap().len(), 0);
    assert_eq!(report["batch_diagnostics"].as_array().unwrap().len(), 0);
    assert_eq!(report["suggestions"].as_array().unwrap().len(), 0);
    assert_eq!(report["mkvmerge_found"], false);
}

/// Same forced-missing-mkvmerge condition as above, human (non-`--json`)
/// mode: the config diagnostic must still be printed (superset of
/// `validate`), and the mkvmerge-not-found message must still appear, exit
/// code 2.
#[test]
fn dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing() {
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let no_mkvmerge_path = empty_path_dir();
    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .env("PATH", no_mkvmerge_path.path())
        .args(["dry-run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stdout.contains("Invalid regular expression") || stdout.to_lowercase().contains("regex"),
        "expected the config-time diagnostic in stdout, got stdout: {stdout}, stderr: {stderr}"
    );
    assert!(
        stderr.contains("mkvmerge"),
        "expected the mkvmerge-not-found message on stderr, got: {stderr}"
    );
}
