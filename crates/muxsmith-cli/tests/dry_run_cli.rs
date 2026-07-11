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

/// Task 8 (#8): human mode must speak even when nothing matched, not stay
/// silent on a clean exit. A clean profile over an empty source directory
/// plans zero primaries; asserts the batch summary line names the zero
/// count, the searched root, and the configured extensions (compare
/// `run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent`
/// in run_cli.rs).
#[test]
fn dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
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
        .output()
        .unwrap();

    assert!(
        out.status.success(),
        "exit: {:?}, stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("0 files matched"),
        "expected the zero-count batch summary line, got stdout: {stdout}"
    );
    assert!(
        stdout.contains(&dir.path().display().to_string()),
        "expected the searched root in the summary line, got stdout: {stdout}"
    );
    assert!(
        stdout.contains("mkv"),
        "expected the configured extensions in the summary line, got stdout: {stdout}"
    );
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

/// D15 (spec 8.1): `--on-collision` overrides the profile's output-collision
/// policy for a single dry-run invocation. Default policy is `error`, so a
/// pre-existing file at the planned output path exits 2; passing
/// `--on-collision skip` downgrades the same collision to a warning and
/// exits 1, with the JSON report carrying an `output-collision` diagnostic
/// at `warning` severity.
#[test]
fn dry_run_on_collision_flag_overrides_default_error_policy() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    // Separate source and output tempdirs: nesting the output dir inside
    // the source dir would let discovery (which scans recursively) pick up
    // the pre-existing collision file as a second primary, muddying the
    // fixture with an unrelated `duplicate-identifier`/`source-overwrite`.
    let src_dir = tempfile::tempdir().unwrap();
    let out_dir = tempfile::tempdir().unwrap();
    let wav = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/seeds/tone.wav"
    );
    let srt = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/seeds/sub.srt"
    );
    let media = src_dir.path().join("Show.S01E01.mkv");
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

    let profile = src_dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    // Pre-existing file at the planned output path ("keep" filename: source
    // basename, ".mkv" enforced) triggers OutputCollision (spec 4.8).
    std::fs::write(out_dir.path().join("Show.S01E01.mkv"), b"pre-existing").unwrap();

    // Default policy (error): exits 2.
    let default_out = Command::cargo_bin("muxsmith")
        .unwrap()
        .args(["dry-run"])
        .arg(&profile)
        .args(["--source"])
        .arg(src_dir.path())
        .args(["--output"])
        .arg(out_dir.path())
        .arg("--json")
        .output()
        .unwrap();
    assert_eq!(
        default_out.status.code(),
        Some(2),
        "stdout: {}, stderr: {}",
        String::from_utf8_lossy(&default_out.stdout),
        String::from_utf8_lossy(&default_out.stderr)
    );

    // --on-collision skip: downgrades to a warning, exits 1.
    let skip_out = Command::cargo_bin("muxsmith")
        .unwrap()
        .args(["dry-run"])
        .arg(&profile)
        .args(["--source"])
        .arg(src_dir.path())
        .args(["--output"])
        .arg(out_dir.path())
        .args(["--on-collision", "skip"])
        .arg("--json")
        .output()
        .unwrap();
    assert_eq!(
        skip_out.status.code(),
        Some(1),
        "stdout: {}, stderr: {}",
        String::from_utf8_lossy(&skip_out.stdout),
        String::from_utf8_lossy(&skip_out.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&skip_out.stdout).unwrap_or_else(|e| {
        panic!(
            "json report: {e}, stderr: {}",
            String::from_utf8_lossy(&skip_out.stderr)
        )
    });
    let diag = report["files"][0]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["code"] == "output-collision")
        .unwrap_or_else(|| panic!("expected an output-collision diagnostic, got: {report}"));
    assert_eq!(diag["severity"], "warning");
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

/// Fastfollow bug 1: `load::from_file` failure (e.g. a nonexistent profile
/// path) is the very first branch in `dry_run::run`. Under `--json` it must
/// still emit exactly one parseable document (the `config_only_json` shape,
/// with this single load diagnostic folded in, spec 5.5's superset-of-
/// validate guarantee held even here), not the bare human-formatted line
/// the bug printed unconditionally.
#[test]
fn dry_run_json_emits_a_document_on_profile_load_failure() {
    let dir = tempfile::tempdir().unwrap();
    let missing_profile = dir.path().join("nonexistent.yaml");

    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .args(["dry-run"])
        .arg(&missing_profile)
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
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Exactly one line: the json document's own `println!`, not that plus a
    // separate human-formatted diagnostic line (the diagnostic's rendered
    // text legitimately appears *inside* that one json line, via the
    // `rendered` field `config_only_json` attaches).
    assert_eq!(
        stdout.lines().count(),
        1,
        "expected exactly one stdout line (the json document) in --json mode, got: {stdout}"
    );
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "--json must still emit a single parseable document on \
             profile-load failure, got parse error: {e}, stdout: {stdout}, \
             stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    let config_diags = report["config_diagnostics"]
        .as_array()
        .expect("config_diagnostics array");
    assert!(
        config_diags.iter().any(|d| d["code"] == "parse-error"),
        "expected a parse-error diagnostic for the missing profile, got: {report}"
    );
    assert_eq!(report["files"].as_array().unwrap().len(), 0);
    assert_eq!(report["batch_diagnostics"].as_array().unwrap().len(), 0);
    assert_eq!(report["suggestions"].as_array().unwrap().len(), 0);
    assert!(
        report.get("mkvmerge_found").is_none(),
        "the mkvmerge lookup never ran on a profile-load failure, so \
         `mkvmerge_found` must be absent rather than asserting a fact never \
         established, got: {report}"
    );
}

/// Points a child process's PATH at a fake `mkvmerge` script that succeeds
/// on `--version` (so `Mkvmerge::locate()` succeeds, unlike
/// `empty_path_dir` above) but fails on every other invocation, including
/// `--list-languages`: a cheap, deterministic stand-in for an installed but
/// broken mkvmerge, with no real MKVToolNix needed. Unix-only: a `#!/bin/sh`
/// script has no direct Windows equivalent, and `Command::new("mkvmerge")`
/// would look for `mkvmerge.exe`/`.cmd`/`.bat` there instead.
#[cfg(unix)]
fn fake_mkvmerge_that_fails_queries() -> tempfile::TempDir {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let script = dir.path().join("mkvmerge");
    std::fs::write(
        &script,
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo 'mkvmerge v99.0.0 (fake, for tests)'\n  exit 0\nfi\nexit 1\n",
    )
    .unwrap();
    let mut perms = std::fs::metadata(&script).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script, perms).unwrap();
    dir
}

/// Fastfollow bug 2: `mkv.list_languages()` failure (mkvmerge located but
/// broken) prints nothing to stdout at all under `--json` today. Must emit
/// the same config-only document shape as the `locate()`-failure branch
/// just above it in the source.
#[test]
#[cfg(unix)]
fn dry_run_json_emits_a_document_when_the_language_query_fails() {
    let fake_path = fake_mkvmerge_that_fails_queries();
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .env("PATH", fake_path.path())
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
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("Querying mkvmerge"),
        "expected no human line on stdout in --json mode, got: {stdout}"
    );
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "--json must still emit a single parseable document when the \
             language query fails, got parse error: {e}, stdout: {stdout}, \
             stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    assert_eq!(report["files"].as_array().unwrap().len(), 0);
    assert_eq!(report["batch_diagnostics"].as_array().unwrap().len(), 0);
    assert_eq!(report["suggestions"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["mkvmerge_found"], true,
        "locate() succeeded on this path (only the language query failed), \
         so mkvmerge_found must be true, got: {report}"
    );
}

/// Same forced-broken-mkvmerge condition, human mode: pins down that
/// today's behavior (bare stderr message, empty stdout, exit 2) is
/// untouched by the `--json` fix; no earlier test in this file covered the
/// human-mode side of this branch.
#[test]
#[cfg(unix)]
fn dry_run_human_mode_still_just_reports_the_language_query_failure_on_stderr() {
    let fake_path = fake_mkvmerge_that_fails_queries();
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .env("PATH", fake_path.path())
        .args(["dry-run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(2));
    assert!(
        out.stdout.is_empty(),
        "stdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Querying mkvmerge"), "stderr: {stderr}");
}
