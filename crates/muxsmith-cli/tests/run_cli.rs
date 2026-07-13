//! CLI-level tests for `muxsmith run`'s planning-failure paths (Task 8) and
//! its `--json` final document (Task 9, D15): every path that can produce a
//! document (a real mux, the specs-empty path, and the mkvmerge-not-found
//! path) must emit exactly one, with `jobs`/`summary` matching the actual
//! outcomes. Beyond the single-fixture case here, a real end-to-end mux
//! under concurrency and failure/warning job states is Task 11's gated
//! end-to-end test, not this file's job.

use std::process::Command;

use assert_cmd::cargo::CommandCargoExt;

mod support;

fn muxsmith() -> Command {
    Command::cargo_bin("muxsmith").unwrap()
}

fn have_mkvmerge() -> bool {
    Command::new("mkvmerge")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// No job ever started: neither a milestone `start` line (`run-job-start`'s
/// fixed `"... start"` fragment) nor the final `run-summary` line (always
/// containing `" ok, "`) appears.
fn asserts_no_job_ran(stdout: &str) {
    assert!(
        !stdout.contains("... start") && !stdout.contains(" ok, "),
        "planning-error path must never start or summarize a job, got stdout: {stdout}"
    );
}

/// `load::from_file` failure (missing/unparsable profile) is the very first
/// branch in `run::run`, ahead of even the config-time validate pass: exits
/// 2 without touching mkvmerge or the queue at all, regardless of whether
/// mkvmerge is installed on the machine running the test.
#[test]
fn missing_profile_file_exits_two_before_any_planning() {
    let out = muxsmith()
        .args(["run", "/nonexistent/profile.yaml"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    asserts_no_job_ran(&String::from_utf8_lossy(&out.stdout));
}

/// A config-time `invalid-regex` (spec 5.2) leaves `input.pattern`
/// uncompilable, so discovery finds zero primaries and `run` reaches
/// `plan_batch` (when mkvmerge is present on PATH) and prints the config-time
/// diagnostic with the batch result, exiting 2 without a single job line.
/// The snapshot captures this mkvmerge-present behavior; for the mkvmerge-missing
/// case, see the sibling test below.
#[test]
fn bad_regex_profile_exits_two_without_executing_a_job() {
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = muxsmith()
        .args(["run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .output()
        .unwrap();

    assert_eq!(
        out.status.code(),
        Some(2),
        "stdout: {}, stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    support::insta_settings_with_tmp(dir.path()).bind(|| {
        insta::assert_snapshot!(stdout);
    });
    asserts_no_job_ran(&stdout);
}

/// Same malformed profile, with mkvmerge forced missing via `PATH` (so the
/// assertion does not depend on whether the test machine happens to have
/// mkvmerge installed): the config-time diagnostic is still surfaced (spec
/// 5.5's superset-of-validate guarantee), the mkvmerge-not-found message
/// lands on stderr, exit code stays 2, and again no job ever runs.
#[test]
fn bad_regex_profile_with_missing_mkvmerge_exits_two_without_executing_a_job() {
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();
    let no_mkvmerge_path = tempfile::tempdir().unwrap();

    let out = muxsmith()
        .env("PATH", no_mkvmerge_path.path())
        .args(["run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(2));
    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    insta::assert_snapshot!(stdout);
    assert!(stderr.contains("mkvmerge"), "got stderr: {stderr}");
    asserts_no_job_ran(&stdout);
}

/// Task 9 (D15): a real single-file mux under `--json` produces exactly one
/// stdout document (all human lines suppressed, per the module doc), whose
/// `jobs` array has one index-0 entry with the real output path, `state:
/// "ok"`, `exit_code: 0`, no captured warnings/errors, and a numeric
/// `duration_ms`; `summary` counts that same job as `ok`; and the
/// dry-run-shaped base fields (`files`, `batch_diagnostics`, `suggestions`)
/// still carry the plan, exactly like `dry-run --json` would.
#[test]
fn run_json_on_a_real_mux_reports_a_populated_jobs_array_and_summary() {
    if !have_mkvmerge() {
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
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
    let output_dir = dir.path().join("out");

    let out = muxsmith()
        .args(["run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .args(["--output"])
        .arg(&output_dir)
        .arg("--json")
        // Task 6 (D26): a real mux reaches the queue and would otherwise
        // persist job logs into the real platform data dir; point it at a
        // tempdir instead.
        .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))
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
        !stdout.contains("... start") && !stdout.contains(" ok, "),
        "--json must suppress every human line, got stdout: {stdout}"
    );
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "json report: {e}, stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    let jobs = report["jobs"].as_array().expect("jobs array");
    assert_eq!(jobs.len(), 1);
    let job = &jobs[0];
    assert_eq!(job["index"], 0);
    assert_eq!(job["state"], "ok");
    assert_eq!(job["exit_code"], 0);
    assert_eq!(job["warnings"].as_array().unwrap().len(), 0);
    assert_eq!(job["errors"].as_array().unwrap().len(), 0);
    assert!(
        job["duration_ms"].as_u64().is_some(),
        "duration_ms must be a number, got: {job}"
    );
    // `output.filename` defaults to `keep` (spec 4.8): file_stem + ".mkv",
    // i.e. the full source stem, not the identifier.
    let expected_output = output_dir.join("Show.S01E01.mkv");
    assert_eq!(job["output"], expected_output.display().to_string());
    assert!(expected_output.exists());
    assert_eq!(
        report["summary"],
        serde_json::json!({ "ok": 1, "warning": 0, "failed": 0, "cancelled": 0 })
    );
}

/// Task 9 (D15): the specs-empty path (nothing plans cleanly enough to mux,
/// here because `input.pattern` fails to compile, an error-severity
/// config-time diagnostic) must still emit a valid `--json` document: the
/// queue never ran, so `jobs` stays empty and `summary` stays zeroed, but
/// the offending `config_diagnostics` entry is still there and the exit
/// code is still the diagnostic-fold 2, exactly like plain `run` (see
/// `bad_regex_profile_exits_two_without_executing_a_job` above).
#[test]
fn run_json_on_specs_empty_from_a_bad_regex_still_emits_a_document_with_empty_jobs() {
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = muxsmith()
        .args(["run"])
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
    asserts_no_job_ran(&stdout);
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
    assert_eq!(report["jobs"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["summary"],
        serde_json::json!({ "ok": 0, "warning": 0, "failed": 0, "cancelled": 0 })
    );
}

/// Same specs-empty path as above, reached via a clean profile over a
/// source directory with no matching files instead of a bad regex: nothing
/// errors, so the exit code is 0 (not 2) with an empty `jobs` array and a
/// zeroed `summary`, distinguishing "queue never ran because there was
/// nothing to run" from "queue never ran because planning failed".
#[test]
fn run_json_on_specs_empty_from_an_empty_source_dir_exits_clean_with_a_zeroed_summary() {
    if !have_mkvmerge() {
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = muxsmith()
        .args(["run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .arg("--json")
        .output()
        .unwrap();

    assert!(
        out.status.success(),
        "exit: {:?}, stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "json report: {e}, stderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    assert_eq!(report["files"].as_array().unwrap().len(), 0);
    assert_eq!(report["jobs"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["summary"],
        serde_json::json!({ "ok": 0, "warning": 0, "failed": 0, "cancelled": 0 })
    );
}

/// Task 8 (#8): human mode must speak even when nothing matched, not stay
/// silent on a clean exit. Same empty-source-dir fixture as the `--json`
/// sibling above, without `--json`: asserts the batch summary line names
/// the zero count, the searched root, and the configured extensions.
#[test]
fn run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent() {
    if !have_mkvmerge() {
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = muxsmith()
        .args(["run"])
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
    let stdout = String::from_utf8(out.stdout).unwrap();
    // The zero-count, searched-root and configured-extensions facts (Task
    // 8) all live in the one rendered `batch-summary` line; a single
    // redacted snapshot covers all three instead of three separate
    // substring checks. The searched root is `dir.path()` itself, real and
    // machine-specific, so it is filtered to a stable placeholder.
    support::insta_settings_with_tmp(dir.path()).bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

/// Task 9 (D15): the mkvmerge-not-found path must surface the same document
/// dry-run's `config_only_json` builds for the same condition (spec 5.5
/// superset-of-validate guarantee), extended with an empty `jobs` array and
/// a zeroed `summary`. Forces the condition via an empty `PATH`, so this
/// does not depend on whether the test machine actually has mkvmerge
/// installed (compare `dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing`
/// in dry_run_cli.rs).
#[test]
fn run_json_surfaces_the_mkvmerge_not_found_document() {
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("bad.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2}E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();
    let no_mkvmerge_path = tempfile::tempdir().unwrap();

    let out = muxsmith()
        .env("PATH", no_mkvmerge_path.path())
        .args(["run"])
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
    asserts_no_job_ran(&stdout);
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
    assert_eq!(report["mkvmerge_found"], false);
    assert_eq!(report["jobs"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["summary"],
        serde_json::json!({ "ok": 0, "warning": 0, "failed": 0, "cancelled": 0 })
    );
}

/// Fastfollow bug 1: `load::from_file` failure (e.g. a nonexistent profile
/// path) is the very first branch in `run::run`, ahead of even the
/// config-time validate pass. Under `--json` it must still emit exactly one
/// parseable document (dry-run's `config_only_json` shape extended with an
/// empty `jobs`/zeroed `summary`, exactly like the mkvmerge-not-found and
/// specs-empty paths above), not the bare human-formatted line the bug
/// printed unconditionally.
#[test]
fn run_json_emits_a_document_on_profile_load_failure() {
    let dir = tempfile::tempdir().unwrap();
    let missing_profile = dir.path().join("nonexistent.yaml");

    let out = muxsmith()
        .args(["run"])
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
    asserts_no_job_ran(&stdout);
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
    assert_eq!(report["jobs"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["summary"],
        serde_json::json!({ "ok": 0, "warning": 0, "failed": 0, "cancelled": 0 })
    );
    assert!(
        report.get("mkvmerge_found").is_none(),
        "the mkvmerge lookup never ran on a profile-load failure, so \
         `mkvmerge_found` must be absent rather than asserting a fact never \
         established, got: {report}"
    );
}

/// Fastfollow bug 2: `mkv.list_languages()` failure (mkvmerge located but
/// broken) prints nothing to stdout at all under `--json` today. Must emit
/// the same config-only document shape as the `locate()`-failure branch
/// just above it in the source, wrapped with an empty `jobs`/zeroed
/// `summary` exactly like `run`'s own mkvmerge-not-found path.
#[test]
#[cfg(unix)]
fn run_json_emits_a_document_when_the_language_query_fails() {
    let fake_path = support::fake_mkvmerge_that_fails_queries();
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("p.yaml");
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: audio } }\n",
    )
    .unwrap();

    let out = muxsmith()
        .env("PATH", fake_path.path())
        .args(["run"])
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
    asserts_no_job_ran(&stdout);
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
    assert_eq!(report["jobs"].as_array().unwrap().len(), 0);
    assert_eq!(
        report["summary"],
        serde_json::json!({ "ok": 0, "warning": 0, "failed": 0, "cancelled": 0 })
    );
    assert_eq!(
        report["mkvmerge_found"], true,
        "locate() succeeded on this path (only the language query failed), \
         so mkvmerge_found must be true, got: {report}"
    );
}

/// Same forced-broken-mkvmerge condition, human mode. Like dry-run, `run`
/// runs the config-time validate pass before the mkvmerge query, and spec
/// 5.5's superset-of-validate guarantee is unconditional: human mode must
/// surface those config diagnostics on stdout, before the stderr failure
/// line and without ever touching the queue. (This branch used to drop them,
/// stderr only; item vii.)
#[test]
#[cfg(unix)]
fn run_human_mode_surfaces_config_diagnostics_on_a_language_query_failure() {
    let fake_path = support::fake_mkvmerge_that_fails_queries();
    let dir = tempfile::tempdir().unwrap();
    let profile = dir.path().join("p.yaml");
    // An empty match expression is a config-time warning; it must reach stdout.
    std::fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: {}\n",
    )
    .unwrap();

    let out = muxsmith()
        .env("PATH", fake_path.path())
        .args(["run"])
        .arg(&profile)
        .args(["--source"])
        .arg(dir.path())
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&out.stdout);
    asserts_no_job_ran(&stdout);
    // `tracks[0].match` is the schema-relative `config_path` the
    // `diagnostic-line` template echoes verbatim; a structural identifier
    // from the profile, not translatable wording, so it stays a plain
    // substring check (superset-of-validate) rather than becoming a
    // snapshot.
    assert!(
        stdout.contains("tracks[0].match"),
        "config diagnostics must be surfaced on stdout (superset-of-validate); stdout: {stdout}"
    );
    // `mkvmerge-query-failed`'s fixed, param-free wording ("Querying
    // mkvmerge failed."): a genuine wording pin, converted to a snapshot.
    let stderr = String::from_utf8(out.stderr).unwrap();
    insta::assert_snapshot!(stderr);
}
