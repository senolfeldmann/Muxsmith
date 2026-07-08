//! Integration tests that spawn the real mkvmerge. Skipped (pass trivially)
//! when mkvmerge is not on PATH, so the suite stays green on machines without
//! it; CI installs mkvtoolnix so these run there.

use muxsmith_core::capability::runtime::Mkvmerge;

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
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
