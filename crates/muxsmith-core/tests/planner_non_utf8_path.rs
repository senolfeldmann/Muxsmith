//! D37 (Plan 5.7): every argv-bound path is validated with `to_str()` at
//! plan finalize; a non-UTF-8 path emits an error-severity `NonUtf8Path`
//! (params: lossy rendering + role) and the file's plan is dropped, so no
//! job is ever built from a path that `command`'s
//! `Path::display().to_string()` rendering would corrupt with U+FFFD.
//!
//! Unix-only for the same reason the paths themselves are: non-UTF-8 paths
//! are arbitrary byte sequences, constructible via
//! `std::os::unix::ffi::OsStrExt::from_bytes` and occurring in practice as
//! legacy-encoded (Latin-1/CP1251) directory names on Linux; Windows paths
//! are UTF-16-based and std offers no stable way to build an ill-formed
//! one. File-level cfg gate per `executor_no_hang_live.rs`.

#![cfg(unix)]

use std::collections::HashMap;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use muxsmith_core::discovery::{Identifier, PrimaryFile};
use muxsmith_core::identify::Identification;
use muxsmith_core::planner::{RunInputs, plan_batch, plan_core};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::report::{DiagCode, Severity};

mod support;
use support::{FakeIdent, lang};

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");

const PROFILE: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
    - match: { exact: { type: audio, language: en } }
"#;

fn ident_for(names: &[&str]) -> FakeIdent {
    let mut by_name = HashMap::new();
    for name in names {
        by_name.insert(name.to_string(), Identification::from_json(SERIES).unwrap());
    }
    FakeIdent { by_name }
}

/// A non-UTF-8 `--output-dir` override (verdict item 2's residual case 2):
/// the rendered output path fails `to_str()`, so the finalize pass emits
/// one error `NonUtf8Path` with the lossy rendering and `role: output`,
/// and the file's plan is dropped (no job is built for it).
#[test]
fn non_utf8_output_dir_drops_plan_with_diagnostic() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let profile = from_str(PROFILE, Format::Yaml).unwrap();
    let out_dir = dir.path().join(OsStr::from_bytes(b"out\xff"));
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(out_dir),
        on_collision: None,
    };
    let mut ident = ident_for(&["Show.S01E01.mkv"]);
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "plan must be dropped, not built lossily");

    let diags: Vec<_> = fr
        .diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::NonUtf8Path)
        .collect();
    assert_eq!(diags.len(), 1, "once per offending file: {diags:?}");
    let d = diags[0];
    assert_eq!(d.severity, Severity::Error);
    assert_eq!(d.config_path, "output");
    assert_eq!(d.params["role"], "output");
    // The `path` param carries the lossy `display()` rendering: the 0xFF
    // byte becomes U+FFFD, and the rendered filename is appended below the
    // non-UTF-8 output dir.
    assert_eq!(
        d.params["path"],
        format!("{}/out\u{fffd}/Show.S01E01.mkv", dir.path().display())
    );
    assert_eq!(
        d.file.as_deref(),
        Some(dir.path().join("Show.S01E01.mkv").as_path())
    );
}

/// The drop is per-file (spec 5.1): a primary under a non-UTF-8-named
/// directory loses its plan with `role: primary`, while a clean sibling in
/// the same batch plans normally. Driven through `plan_core` with
/// handcrafted primaries because `plan_batch`'s discovery walk cannot
/// produce the offending path here (the source root itself is clean), and
/// the injected `Identify` bypasses the runtime's own non-UTF-8 rejection
/// (`Mkvmerge::identify_json`), exactly like a GUI simulation would: the
/// finalize pass must hold without upstream help (core-31: guard the
/// invariant, not the induction proof over current callers).
#[test]
fn non_utf8_primary_is_dropped_while_clean_sibling_plans() {
    let dir = tempfile::tempdir().unwrap();
    let bad_dir: PathBuf = dir.path().join(OsStr::from_bytes(b"s\xe4son")); // Latin-1 "säson"
    let bad_primary = bad_dir.join("Show.S01E01.mkv");
    let clean_primary = dir.path().join("Show.S01E02.mkv");
    let primaries =
        [("S01E01", &bad_primary), ("S01E02", &clean_primary)].map(|(id, p)| PrimaryFile {
            path: p.clone(),
            identifier: Identifier {
                whole: id.to_string(),
                groups: Default::default(),
            },
        });

    let profile = from_str(PROFILE, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut ident = ident_for(&["Show.S01E01.mkv", "Show.S01E02.mkv"]);
    let batch = plan_core(&profile, &run, &primaries, &mut ident, &lang());

    assert_eq!(batch.files.len(), 2);
    let bad = &batch.files[0];
    assert!(bad.plan.is_none(), "non-UTF-8 primary must not plan");
    let d = bad
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::NonUtf8Path)
        .expect("NonUtf8Path diagnostic for the offending primary");
    assert_eq!(d.severity, Severity::Error);
    assert_eq!(d.config_path, "input");
    assert_eq!(d.params["role"], "primary");
    assert_eq!(
        d.params["path"],
        format!("{}/s\u{fffd}son/Show.S01E01.mkv", dir.path().display())
    );

    let clean = &batch.files[1];
    assert!(
        clean
            .diagnostics
            .iter()
            .all(|d| d.code != DiagCode::NonUtf8Path),
        "clean sibling must not be contaminated: {:?}",
        clean.diagnostics
    );
    assert!(clean.plan.is_some(), "diags: {:?}", clean.diagnostics);
}
