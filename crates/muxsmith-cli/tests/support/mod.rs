//! Shared test helpers for muxsmith-cli's integration test suites.
//!
//! A `tests/` SUBDIRECTORY module (`tests/support/mod.rs`), not a
//! `tests/*.rs` file: Cargo's test-target autodiscovery only globs
//! `tests/*.rs`, so this file is never compiled as its own test binary. Each
//! consumer test file pulls it in with `mod support;` (mirrors
//! `muxsmith-core/tests/support/mod.rs`'s existing convention).

use std::path::Path;

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
