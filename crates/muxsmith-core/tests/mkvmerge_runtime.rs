//! Integration tests that spawn the real mkvmerge. Skipped (pass trivially)
//! when mkvmerge is not on PATH, so the suite stays green on machines without
//! it; CI installs mkvtoolnix so these run there.

use muxsmith_core::capability::runtime::{MIN_SUPPORTED, Mkvmerge, RuntimeError};

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
}

/// Writes a fake `mkvmerge` shell script to `dir` that answers `--version`
/// with `version_line` and fails every other invocation. Unix-only, mirrors
/// the identical helper pattern in `muxsmith-cli/tests/run_cli.rs`
/// (`fake_mkvmerge_that_fails_queries`).
#[cfg(unix)]
fn fake_mkvmerge(dir: &std::path::Path, version_line: &str) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let script = dir.join("mkvmerge");
    std::fs::write(
        &script,
        format!(
            "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '{version_line}'\n  exit 0\nfi\nexit 1\n"
        ),
    )
    .unwrap();
    let mut perms = std::fs::metadata(&script).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script, perms).unwrap();

    // A freshly written+chmod'd file can transiently answer `ExecutableFileBusy`
    // ("Text file busy") on `execve` under heavy parallel `cargo test` load:
    // the kernel's writecount release on close() races the shell's own
    // execve() against other test threads doing the same write-then-exec
    // sequence at once. Confirmed by hand: `cargo test -p muxsmith-core
    // --test mkvmerge_runtime` flaked on this roughly 1 run in 3 without the
    // warm-up below, 0 in 10+ with it. Retrying a throwaway invocation here
    // (rather than in every call site) makes the script's own readiness a
    // property of `fake_mkvmerge` itself.
    for attempt in 0.. {
        match std::process::Command::new(&script)
            .arg("--version")
            .output()
        {
            Ok(_) => break,
            Err(e) if e.kind() == std::io::ErrorKind::ExecutableFileBusy && attempt < 50 => {
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
            Err(e) => panic!("fake mkvmerge script at {script:?} never became runnable: {e}"),
        }
    }

    script
}

#[test]
fn version_reports_mkvmerge() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };
    let v = m.version().expect("version query");
    assert!(v.to_lowercase().contains("mkvmerge"), "got: {v}");
}

#[test]
fn list_types_includes_matroska() {
    let Some(m) = mkvmerge() else { return };
    let types = m.list_types().expect("list-types");
    assert!(types.contains(&"mkv".to_string()));
}

#[test]
fn list_languages_normalizes_english_and_german() {
    let Some(m) = mkvmerge() else { return };
    let idx = m.list_languages().expect("list-languages");
    assert_eq!(idx.normalize("en"), idx.normalize("eng"));
    assert_eq!(idx.normalize("de"), idx.normalize("ger"));
    assert_ne!(idx.normalize("en"), idx.normalize("de"));
}

/// Ladder step 1 (D28): an explicit override is used without ever
/// consulting PATH. The fake script lives outside PATH entirely, so a
/// PATH-first implementation would fail to locate it and this test would
/// fail regardless of what real mkvmerge (if any) is installed.
#[test]
#[cfg(unix)]
fn detect_prefers_override_over_path() {
    let dir = tempfile::tempdir().unwrap();
    let override_path = fake_mkvmerge(dir.path(), "mkvmerge v123.4.5 ('Override') 64-bit");

    let detected = Mkvmerge::detect(Some(&override_path)).expect("override script should be used");
    assert_eq!(detected.path(), override_path);
    assert_eq!(detected.version_pair().unwrap(), (123, 4));
}

/// The version floor (D28): a located mkvmerge below `MIN_SUPPORTED` is
/// reported as `TooOld` with both the found and minimum versions, not
/// silently accepted or masked as `NotFound`.
#[test]
#[cfg(unix)]
fn detect_reports_too_old_with_found_and_minimum() {
    let dir = tempfile::tempdir().unwrap();
    let override_path = fake_mkvmerge(dir.path(), "mkvmerge v50.0.0 ('Old') 64-bit");

    let err = Mkvmerge::detect(Some(&override_path)).expect_err("v50 is below the floor");
    match err {
        RuntimeError::TooOld { found, minimum } => {
            assert!(found.contains("v50.0.0"), "found: {found}");
            assert_eq!(minimum, "86.0");
        }
        other => panic!("expected TooOld, got {other:?}"),
    }
}

/// Gated: `detect(None)` falls through to PATH (like `locate()`) and finds
/// the real local mkvmerge, whose version clears `MIN_SUPPORTED`.
#[test]
fn detect_none_finds_real_mkvmerge_meeting_the_version_floor() {
    if mkvmerge().is_none() {
        eprintln!("mkvmerge not found; skipping");
        return;
    }
    let detected = Mkvmerge::detect(None).expect("detect(None) should find PATH mkvmerge");
    let pair = detected.version_pair().expect("version_pair query");
    assert!(
        pair >= MIN_SUPPORTED,
        "found {pair:?}, expected at least {MIN_SUPPORTED:?}"
    );
}
