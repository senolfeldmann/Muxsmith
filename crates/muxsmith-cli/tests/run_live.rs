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

mod support;

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
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
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

    let out = support::muxsmith(&[
        "run",
        profile.to_str().unwrap(),
        "--source",
        source_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ])
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

    // The run-summary line ("{ok} ok, {warning} warning, ...", `run.rs`'s
    // `render_summary`) is a genuine wording pin, snapshotted rather than
    // hardcoded here (spec 10). Only that one line, not the full stdout:
    // the per-job milestone lines above it carry real elapsed seconds and
    // (for a near-instant fixture mux) a nondeterministic subset of the 25/
    // 50/75% progress thresholds, neither safe to pin byte-for-byte even
    // with duration redaction. `run-joblog-written` may follow it as the
    // last line instead (a logger is created whenever a real runs-root
    // resolves), so the summary line is found by its distinctive shape
    // rather than assumed to be `stdout`'s last line.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let summary_line = stdout
        .lines()
        .find(|l| l.contains(" ok, ") && l.contains(" cancelled"))
        .unwrap_or_else(|| panic!("expected the run-summary line in stdout, got: {stdout}"));
    insta::assert_snapshot!(summary_line);
}

/// D38 acceptance: a zero-rule `unmatched: keep` profile is a legal pure
/// passthrough - dry-run reports `passthrough-profile` (info) and no
/// `no-track-rules`, and `run` produces an identifiable Matroska output
/// carrying both source tracks unchanged.
#[test]
fn zero_rule_keep_profile_is_a_pure_passthrough() {
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
    assert!(
        Command::new("mkvmerge")
            .args(["-q", "-o"])
            .arg(&media)
            .arg(wav)
            .arg(srt)
            .status()
            .unwrap()
            .success()
    );

    let profile = dir.path().join("p.yaml");
    fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  unmatched: keep\n  rules: []\n",
    )
    .unwrap();
    let output_dir = dir.path().join("out");

    // Dry run: exit 0, passthrough announced, no NoTrackRules.
    let dry = support::muxsmith(&[
        "dry-run",
        profile.to_str().unwrap(),
        "--source",
        dir.path().to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
        "--json",
    ])
    .output()
    .unwrap();
    assert!(
        dry.status.success(),
        "dry-run must accept the passthrough profile, stdout: {}, stderr: {}",
        String::from_utf8_lossy(&dry.stdout),
        String::from_utf8_lossy(&dry.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&dry.stdout).unwrap_or_else(|e| {
        panic!(
            "json report: {e}, stderr: {}",
            String::from_utf8_lossy(&dry.stderr)
        )
    });
    // PassthroughProfile/NoTrackRules are config-time (validate) diagnostics,
    // reported under `config_diagnostics`, not per-file `diagnostics`
    // (dry_run_cli.rs's `dry_run_surfaces_config_time_invalid_regex` and
    // siblings pin the same top-level field for the same reason).
    let codes: Vec<&str> = report["config_diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["code"].as_str().unwrap())
        .collect();
    assert!(
        codes.contains(&"passthrough-profile"),
        "expected passthrough-profile in config_diagnostics, got: {report}"
    );
    assert!(
        !codes.contains(&"no-track-rules"),
        "no-track-rules must not fire for a keep passthrough, got: {report}"
    );

    // Run: output exists and identifies with both tracks intact.
    let run = support::muxsmith(&[
        "run",
        profile.to_str().unwrap(),
        "--source",
        dir.path().to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ])
    // Task 6 (D26): a real mux reaches the queue and would otherwise
    // persist job logs into the real platform data dir; point it at a
    // tempdir instead (same idiom as the two live-run tests above).
    .env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))
    .output()
    .unwrap();
    assert!(
        run.status.success(),
        "run must succeed on the passthrough profile, stdout: {}, stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    // `output.filename` defaults to `keep` (spec 4.8): file_stem + ".mkv",
    // same mapping the two live-run tests above pin.
    let out_file = output_dir.join("Show.S01E01.mkv");
    assert!(out_file.exists(), "missing {out_file:?}");
    let ident = Command::new("mkvmerge")
        .arg("-J")
        .arg(&out_file)
        .output()
        .unwrap();
    let j: serde_json::Value = serde_json::from_slice(&ident.stdout).unwrap();
    assert_eq!(j["container"]["recognized"], true);
    assert_eq!(
        j["tracks"].as_array().unwrap().len(),
        2,
        "both tracks pass through"
    );
}

/// D40 regression (the `batch_document` panic, whole-branch-verdict.md
/// Finding 1): the README's passthrough recipe (the YAML block under its
/// "Pure passthrough: a profile with zero rules" heading) inlined verbatim --
/// deliberately NOT read from the file at test time, so a change to either
/// side (recipe or test) is a visible diff, not a silent divergence; if the
/// README recipe's YAML ever changes, this literal must be updated to
/// match it byte-for-byte. The recipe's `title: { template: 'S{season}E{episode}' }`
/// resolves to `TitleAction::Set`, one of the three plan enums whose old
/// `#[serde(tag = "kind")]` + newtype-payload shape (`Set(String)`) cannot
/// serialize under serde's internally-tagged representation: pre-fix, the
/// mux succeeds and then `run`/`dry-run --json` panic building the
/// `--json`/persisted report ("cannot serialize tagged newtype variant
/// TitleAction::Set containing a string"), exit 101 after a successful mux.
/// Pins two things: the D40 fix itself, and the recipe's own
/// paste-runnability (a doc recipe is only proven by driving every command
/// its prose tells the reader to run next, not the cheapest one).
#[test]
fn readme_passthrough_recipe_with_title_template_survives_dry_run_and_run() {
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
    assert!(
        Command::new("mkvmerge")
            .args(["-q", "-o"])
            .arg(&media)
            .arg(wav)
            .arg(srt)
            .status()
            .unwrap()
            .success()
    );

    // The YAML block under README.md's "Pure passthrough: a profile with
    // zero rules" heading, verbatim.
    let profile = dir.path().join("p.yaml");
    fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<season>\\d{2})E(?<episode>\\d{2})', extensions: [mkv] }\ntracks:\n  unmatched: keep\n  rules: []\ntitle: { template: 'S{season}E{episode}' }\n",
    )
    .unwrap();
    let output_dir = dir.path().join("out");
    let runs_root = dir.path().join("runs");

    // `dry-run --json`: exit 0, exactly one valid JSON document on stdout
    // (pre-fix: panicked in `batch_document` while building the `Set` plan
    // value; `--json` per README.md's "What you get" section, "Scriptable
    // everything" bullet).
    let dry = support::muxsmith(&[
        "dry-run",
        profile.to_str().unwrap(),
        "--source",
        dir.path().to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
        "--json",
    ])
    .output()
    .unwrap();
    assert!(
        dry.status.success(),
        "dry-run --json must exit 0, stdout: {}, stderr: {}",
        String::from_utf8_lossy(&dry.stdout),
        String::from_utf8_lossy(&dry.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&dry.stdout).unwrap_or_else(|e| {
        panic!(
            "dry-run --json must print one valid JSON document: {e}, stderr: {}",
            String::from_utf8_lossy(&dry.stderr)
        )
    });
    let files = report["files"].as_array().unwrap();
    assert_eq!(files.len(), 1, "report: {report}");
    assert_eq!(
        files[0]["plan"]["title"],
        serde_json::json!({"kind": "set", "title": "S01E01"}),
        "expected the templated Set title in the plan, got: {report}"
    );

    // `run`: exit 0 (pre-fix: exit 101 -- the mux itself already succeeded,
    // but the unconditional `run_document(batch_document(..))` build in
    // `crates/muxsmith-cli/src/commands/run.rs`'s `run` then panicked on the
    // same `Set` value).
    let run = support::muxsmith(&[
        "run",
        profile.to_str().unwrap(),
        "--source",
        dir.path().to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ])
    .env("MUXSMITH_RUNS_ROOT", &runs_root)
    .output()
    .unwrap();
    assert!(
        run.status.success(),
        "run must exit 0 (pre-fix: panicked after a successful mux), stdout: {}, stderr: {}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let out_file = output_dir.join("Show.S01E01.mkv");
    assert!(out_file.exists(), "missing {out_file:?}");
    let ident = Command::new("mkvmerge")
        .arg("-J")
        .arg(&out_file)
        .output()
        .unwrap();
    let j: serde_json::Value = serde_json::from_slice(&ident.stdout).unwrap();
    assert_eq!(j["container"]["recognized"], true, "ident: {j}");
    assert_eq!(
        j["container"]["properties"]["title"], "S01E01",
        "the templated title must land in the muxed output, ident: {j}"
    );

    // Run document/log persisted (D26), same summary.json shape
    // `muxsmith_core::executor::joblog`'s own tests pin: exactly one run
    // directory under the runs root, carrying a `summary.json` that parses
    // and whose plan title matches what dry-run reported.
    let run_dirs: Vec<_> = fs::read_dir(&runs_root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(
        run_dirs.len(),
        1,
        "expected exactly one persisted run directory, got: {run_dirs:?}"
    );
    let summary_path = run_dirs[0].join("summary.json");
    let summary: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&summary_path)
            .unwrap_or_else(|e| panic!("summary.json must be readable at {summary_path:?}: {e}")),
    )
    .unwrap_or_else(|e| panic!("summary.json must parse as JSON: {e}"));
    assert_eq!(
        summary["files"][0]["plan"]["title"],
        serde_json::json!({"kind": "set", "title": "S01E01"}),
        "persisted summary.json: {summary}"
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
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
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
        let mut args = vec![
            "run",
            profile.to_str().unwrap(),
            "--source",
            source_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ];
        if let Some(policy) = on_collision {
            args.extend(["--on-collision", policy]);
        }
        let mut cmd = support::muxsmith(&args);
        cmd.env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"));
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
