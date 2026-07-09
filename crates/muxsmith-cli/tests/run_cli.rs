//! CLI-level tests for `muxsmith run`'s planning-failure paths (Task 8): any
//! bad-profile or missing-mkvmerge condition must exit with the usual
//! diagnostic-fold code and never touch the job queue. The full execute
//! path (a real mux, exit 0/1/2 driven by actual job outcomes) is Task 11's
//! gated end-to-end test, not this file's job.

use std::process::Command;

use assert_cmd::cargo::CommandCargoExt;

fn muxsmith() -> Command {
    Command::cargo_bin("muxsmith").unwrap()
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
/// uncompilable, so discovery finds zero primaries and `run` folds straight
/// to exit 2 without ever building a `JobSpec` or starting the queue.
/// Deliberately not gated on a real mkvmerge being installed: whether
/// mkvmerge is present (plans an empty batch) or missing (stops even
/// earlier, see the sibling test below), both paths print the same
/// config-time diagnostic and exit 2 without a single job line, so these
/// assertions hold regardless of the test machine's mkvmerge situation.
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
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Invalid regular expression"),
        "expected the config-time diagnostic in stdout, got: {stdout}"
    );
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
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stdout.contains("Invalid regular expression"),
        "expected the config-time diagnostic in stdout, got: {stdout}"
    );
    assert!(stderr.contains("mkvmerge"), "got stderr: {stderr}");
    asserts_no_job_ran(&stdout);
}
