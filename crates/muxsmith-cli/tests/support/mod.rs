//! Shared test helpers for muxsmith-cli's integration test suites.
//!
//! A `tests/` SUBDIRECTORY module (`tests/support/mod.rs`), not a
//! `tests/*.rs` file: Cargo's test-target autodiscovery only globs
//! `tests/*.rs`, so this file is never compiled as its own test binary. Each
//! consumer test file pulls it in with `mod support;` (mirrors
//! `muxsmith-core/tests/support/mod.rs`'s existing convention).

use std::path::Path;

use assert_cmd::Command;
use insta::Settings;

/// Baseline `insta` filters shared by every CLI snapshot test (spec 10):
/// two kinds of machine-dependent text a human-rendered CLI line can carry
/// regardless of which test produced it.
///
/// - `mkvmerge v...` version banners: no current call site echoes
///   `Mkvmerge::version()`'s raw `--version` output into CLI text (grepped
///   the whole crate for this task); nothing stops a future `RuntimeError`
///   `Display` from doing so through spec 10's accepted third-party-text
///   exception, and CI genuinely runs divergent mkvmerge builds per leg
///   (Plan 5.5 Task 2: apt ships 97.0-1build1 on Linux, choco/brew ship
///   100.0.0 on Windows/macOS) -- a future leak would silently diverge the
///   two without this filter already in place.
/// - job duration text (`run-job-ok`/`-warning`/`-failed`'s `{ $seconds }`,
///   always `format!("{:.1}", ms as f64 / 1000.0)`, see `commands/run.rs`):
///   genuinely nondeterministic wall-clock time, machine- and load-dependent.
///
/// Absolute filesystem paths are deliberately NOT handled here: each test
/// builds its own `TempDir` at a fresh, unpredictable location, so the
/// exact path is only known to that one call site. Register it there via
/// [`insta_settings_with_tmp`] -- an exact, escaped literal match, not a
/// generic "looks like an absolute path" regex that could also swallow
/// unrelated content (e.g. a `config_path` value) and mask a real bug.
#[allow(dead_code)]
pub fn insta_settings() -> Settings {
    let mut settings = Settings::clone_current();
    settings.add_filter(r"mkvmerge v\d+(?:\.\d+){1,3}[^\n]*", "mkvmerge v[VERSION]");
    settings.add_filter(r"\d+\.\d+s\b", "[N.Ns]");
    settings
}

/// [`insta_settings`]'s baseline plus an exact-literal filter for `path`
/// (escaped via [`regex::escape`], not a pattern -- `path` is a real
/// `TempDir` root, e.g. containing a literal `.` on Unix, which would
/// otherwise be interpreted as "any character").
#[allow(dead_code)]
pub fn insta_settings_with_tmp(path: &Path) -> Settings {
    let mut settings = insta_settings();
    settings.add_filter(&regex::escape(&path.display().to_string()), "[TMPDIR]");
    settings
}

/// Points a child process's PATH at a fake `mkvmerge` script that succeeds
/// on `--version` (so `Mkvmerge::locate()` succeeds) but fails on every
/// other invocation, including `--list-languages`: a cheap, deterministic
/// stand-in for an installed but broken mkvmerge, no real MKVToolNix
/// needed. Unix-only: a `#!/bin/sh` script has no direct Windows
/// equivalent, and `Command::new("mkvmerge")` would look for
/// `mkvmerge.exe`/`.cmd`/`.bat` there instead. Shared by `run_cli.rs` and
/// `dry_run_cli.rs`.
#[allow(dead_code)]
#[cfg(unix)]
pub fn fake_mkvmerge_that_fails_queries() -> tempfile::TempDir {
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

/// The one CLI-invocation funnel (D64, `cli-multilang-rendering`'s
/// companion constraint): every integration test that runs the muxsmith
/// binary builds its `Command` here. The funnel appends `--locale en`
/// AFTER the caller's args: `--locale` is a per-subcommand argument
/// (`cli.rs`), so it must follow the subcommand, which appending
/// guarantees. Pinning rides the CLI's own contractual surface, never
/// environment variables: `sys_locale` reads OS APIs, not env vars, on
/// Windows and macOS (D64's rejected alternative). Post-sweep invariant:
/// `cargo_bin("muxsmith")` appears nowhere outside this function.
pub fn muxsmith(args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("muxsmith").unwrap();
    cmd.args(args);
    cmd.args(["--locale", "en"]);
    cmd
}

/// The closed TWO-caller funnel exception (round-5-amended D64):
/// (1) `no_args_shows_usage_and_fails` verifies that a BARE `muxsmith`
/// invocation - a real user scenario - prints usage and fails; through
/// the funnel it would verify a different behavior (unexpected
/// top-level argument). (2) `cli_schema.rs::schema_json`: `Schema` is
/// an argument-less unit variant, so `muxsmith schema --locale en`
/// exits 2 on clap's unexpected-argument error, and the `Schema` arm
/// constructs no `Renderer` - schema output is locale-independent
/// permanently (English-only `description` fields, spec 8.4/D47). Both
/// callers are locale-moot by construction: no `Renderer` exists on
/// either path. The exception set is closed: exactly these two
/// callers; a third reopens D64 rather than riding the helper.
#[allow(dead_code)]
pub fn muxsmith_bare() -> Command {
    Command::cargo_bin("muxsmith").unwrap()
}
