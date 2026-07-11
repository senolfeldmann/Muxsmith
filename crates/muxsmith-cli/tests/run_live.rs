//! Task 11 (D15): a gated end-to-end acceptance test that drives the actual
//! `muxsmith` binary against real mkvmerge, self-skipping when mkvmerge is
//! not on `PATH` -- the same locate-or-skip idiom `run_cli.rs`'s
//! `have_mkvmerge` and `command_integration.rs`'s `mkvmerge()` helper both
//! use. Two cases:
//! - a clean two-file `run` mux: exit 0, both outputs a real, mkvmerge
//!   `-J`-identifiable Matroska file (confirmed empirically: `container.
//!   recognized: true`, `container.type: "Matroska"`), and the human
//!   `run-summary` line on stdout.
//! - the rerun workflow guard (D14/D17): rerunning the same `run` with
//!   `--on-collision skip` must exit 1 and leave both outputs untouched.
//!   "Untouched" is asserted so it cannot pass vacuously on a filesystem
//!   with coarse mtime granularity: each output's mtime is deliberately
//!   backdated by an hour before the rerun, so any write at all -- even one
//!   that reproduces the exact same bytes -- would leave a fresh mtime
//!   unmistakably different from the stale one, independent of clock
//!   resolution; full byte-content equality is checked on top, which alone
//!   already catches any truncate/rewrite regardless of mtime at all.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

use assert_cmd::cargo::CommandCargoExt;

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

/// A minimal profile matching `Show.SxxExx.mkv`-style basenames and keeping
/// each source's one subtitle track (the SRT fixture pattern below needs no
/// media codec libs to mux, unlike audio/video).
const PROFILE: &str = "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  rules:\n    - match: { exact: { type: subtitles } }\n";

/// Builds one tiny single-subtitle-track source MKV named `name` inside
/// `dir` via real mkvmerge.
fn build_source_mkv(dir: &Path, name: &str) -> PathBuf {
    let srt = dir.join(format!("{name}.srt"));
    fs::write(&srt, "1\n00:00:00,000 --> 00:00:01,000\nHello\n").unwrap();
    let source = dir.join(name);
    let status = Command::new("mkvmerge")
        .args(["-q", "-o"])
        .arg(&source)
        .arg(&srt)
        .status()
        .expect("spawn mkvmerge to build a fixture source");
    assert!(status.success(), "mkvmerge failed to build {name}");
    source
}

/// `mkvmerge -J path` recognizes `path` as a valid Matroska container:
/// `container.recognized == true` and `container.type == "Matroska"`,
/// confirmed empirically against the installed mkvmerge v100 (module doc).
/// The strongest available proof that `run` produced a real, playable mux
/// and not just a same-named stub file.
fn assert_identifies_as_matroska(path: &Path) {
    let out = Command::new("mkvmerge")
        .arg("-J")
        .arg(path)
        .output()
        .expect("spawn mkvmerge -J");
    assert!(out.status.success(), "mkvmerge -J failed on {path:?}");
    let json: serde_json::Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|e| panic!("mkvmerge -J did not print JSON for {path:?}: {e}"));
    assert_eq!(
        json["container"]["recognized"], true,
        "expected {path:?} to be a recognized container, got: {json}"
    );
    assert_eq!(
        json["container"]["type"], "Matroska",
        "expected {path:?} to identify as Matroska, got: {json}"
    );
}

/// Task 11: two tiny sources through `muxsmith run` over real mkvmerge must
/// exit 0, produce two real Matroska outputs, and print the human
/// `run-summary` line.
#[test]
fn live_run_muxes_two_sources_and_reports_exit_zero() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let source_dir = dir.path().join("source");
    fs::create_dir_all(&source_dir).unwrap();
    build_source_mkv(&source_dir, "Show.S01E01.mkv");
    build_source_mkv(&source_dir, "Show.S01E02.mkv");

    let profile = dir.path().join("p.yaml");
    fs::write(&profile, PROFILE).unwrap();
    let output_dir = dir.path().join("out");

    let out = muxsmith()
        .args(["run"])
        .arg(&profile)
        .args(["--source"])
        .arg(&source_dir)
        .args(["--output"])
        .arg(&output_dir)
        // Task 6 (D26): a real mux reaches the queue and would otherwise
        // persist job logs into the real platform data dir; point it at a
        // tempdir instead.
        .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))
        .output()
        .unwrap();

    assert!(
        out.status.success(),
        "exit: {:?}, stdout: {}, stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // `output.filename` defaults to `keep` (spec 4.8): file_stem + ".mkv",
    // i.e. the full source stem (see the identical comment in
    // `run_cli.rs`'s `run_json_on_a_real_mux_...` test, which pins the same
    // mapping down).
    let out1 = output_dir.join("Show.S01E01.mkv");
    let out2 = output_dir.join("Show.S01E02.mkv");
    assert!(out1.exists(), "missing {out1:?}");
    assert!(out2.exists(), "missing {out2:?}");
    assert_identifies_as_matroska(&out1);
    assert_identifies_as_matroska(&out2);

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("2 ok, 0 warning, 0 failed, 0 cancelled"),
        "expected the run-summary line in stdout, got: {stdout}"
    );
}

/// Backdates `path`'s mtime by an hour and returns the value actually
/// stored (re-read from the filesystem rather than trusted from memory, so
/// any precision loss the filesystem applies is already baked into the
/// reference the later assertion compares against).
fn backdate_mtime(path: &Path) -> SystemTime {
    let target = SystemTime::now() - Duration::from_secs(3600);
    // Open with write access: Windows requires FILE_WRITE_ATTRIBUTES to set_modified.
    let file = fs::OpenOptions::new().write(true).open(path).unwrap();
    file.set_modified(target).unwrap();
    fs::metadata(path).unwrap().modified().unwrap()
}

/// Task 11, the rerun workflow guard (D14/D17): running `run` again over
/// outputs that already exist on disk, with `--on-collision skip`, must
/// exit 1 (a collision is a warning-severity diagnostic under `skip`) and
/// must never touch either pre-existing output.
#[test]
fn live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched() {
    if !have_mkvmerge() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let source_dir = dir.path().join("source");
    fs::create_dir_all(&source_dir).unwrap();
    build_source_mkv(&source_dir, "Show.S01E01.mkv");
    build_source_mkv(&source_dir, "Show.S01E02.mkv");

    let profile = dir.path().join("p.yaml");
    fs::write(&profile, PROFILE).unwrap();
    let output_dir = dir.path().join("out");

    // Task 6 (D26): the setup run below reaches the queue and would
    // otherwise persist job logs into the real platform data dir; point
    // both invocations at a tempdir instead.
    let run_args = |on_collision: Option<&str>| {
        let mut cmd = muxsmith();
        cmd.args(["run"])
            .arg(&profile)
            .args(["--source"])
            .arg(&source_dir)
            .args(["--output"])
            .arg(&output_dir)
            .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"));
        if let Some(policy) = on_collision {
            cmd.args(["--on-collision", policy]);
        }
        cmd
    };

    let first = run_args(None).output().unwrap();
    assert!(
        first.status.success(),
        "setup run failed, stdout: {}, stderr: {}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );

    let out1 = output_dir.join("Show.S01E01.mkv");
    let out2 = output_dir.join("Show.S01E02.mkv");
    assert!(
        out1.exists() && out2.exists(),
        "setup run produced no outputs"
    );

    let content1 = fs::read(&out1).unwrap();
    let content2 = fs::read(&out2).unwrap();
    let stale1 = backdate_mtime(&out1);
    let stale2 = backdate_mtime(&out2);

    let second = run_args(Some("skip")).output().unwrap();

    assert_eq!(
        second.status.code(),
        Some(1),
        "stdout: {}, stderr: {}",
        String::from_utf8_lossy(&second.stdout),
        String::from_utf8_lossy(&second.stderr)
    );
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(
        !stdout.contains(" ok, "),
        "a skipped collision must never start or summarize a job, got stdout: {stdout}"
    );

    // Non-vacuous "untouched": exact byte content is unchanged, and the
    // mtime is still the deliberately stale value backdated above (any real
    // write, even a same-bytes rewrite, would bump it to "now").
    assert_eq!(fs::read(&out1).unwrap(), content1, "out1 content changed");
    assert_eq!(fs::read(&out2).unwrap(), content2, "out2 content changed");
    assert_eq!(
        fs::metadata(&out1).unwrap().modified().unwrap(),
        stale1,
        "out1 mtime changed: rerun touched an existing output"
    );
    assert_eq!(
        fs::metadata(&out2).unwrap().modified().unwrap(),
        stale2,
        "out2 mtime changed: rerun touched an existing output"
    );
}
