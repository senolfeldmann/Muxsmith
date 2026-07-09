use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::report::DiagCode;

// A fake identifier backed by fixture JSON keyed on file name.
struct FakeIdent {
    by_name: HashMap<String, Identification>,
}
impl Identify for FakeIdent {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        let name = path.file_name().unwrap().to_str().unwrap();
        self.by_name
            .get(name)
            .cloned()
            .ok_or_else(|| IdentifyError::Json(format!("no fixture for {name}")))
    }
}

fn lang() -> LanguageIndex {
    LanguageIndex::from_rows(&[
        ["English", "eng", "eng", "en"],
        ["German", "ger", "ger", "de"],
    ])
}

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");

fn plan_one(
    profile_yaml: &str,
    file_name: &str,
    ident_json: &str,
) -> muxsmith_core::planner::Batch {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(file_name), b"x").unwrap();
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        file_name.to_string(),
        Identification::from_json(ident_json).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(dir);
    batch
}

const P_VIDEO_AUDIO: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { type: audio, language: en } }
"#;

#[test]
fn resolves_each_rule_to_one_track() {
    let batch = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments.len(), 2);
    assert_eq!(plan.assignments[0].track_id, Some(0));
    assert_eq!(plan.assignments[1].track_id, Some(1));
}

#[test]
fn ambiguous_rule_when_two_tracks_match() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles, codec_kind: srt, language: en } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule)
    );
}

#[test]
fn missing_track_when_no_match_and_not_optional() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: de } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::MissingTrack)
    );
}

#[test]
fn optional_absent_track_is_no_error() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { type: audio, language: de } }
    optional: true
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments[1].track_id, None);
}

#[test]
fn overlapping_rules_when_two_rules_claim_one_track() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { codec_id: 'V_MPEG4/ISO/AVC' } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::OverlappingRules)
    );
}

#[test]
fn keep_filename_renders_mkv_output() {
    let batch = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
    let plan = batch.files[0].plan.as_ref().unwrap();
    assert!(
        plan.output
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with(".mkv")
    );
    assert_eq!(plan.output.file_name().unwrap(), "Show.S01E01.mkv");
}

#[test]
fn unidentifiable_primary_yields_unidentifiable_source_not_missing_track() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let profile = from_str(P_VIDEO_AUDIO, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    // Empty fixture map: FakeIdent::identify errors for every path.
    let mut ident = FakeIdent {
        by_name: HashMap::new(),
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(dir);

    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::UnidentifiableSource)
        .unwrap_or_else(|| panic!("expected UnidentifiableSource, got: {:?}", fr.diagnostics));
    assert!(
        !d.params.get("detail").unwrap_or(&String::new()).is_empty(),
        "expected a non-empty detail param, got: {d:?}"
    );
    assert!(
        !fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::MissingTrack),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn unidentifiable_donor_yields_unidentifiable_source_not_missing_external() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("Donor.S01E01.srt"), b"y").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
  - source:
      external: { path: '.', extensions: [srt], match_to_source: true }
    match: { exact: { type: subtitles } }
    optional: true
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    // Only the primary identifies; the donor has no fixture, so it fails.
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(dir);

    let fr = &batch.files[0];
    // A present-but-unidentifiable donor is a hard error even though the
    // rule is optional: optional only covers zero matches, not a broken file.
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::UnidentifiableSource),
        "diags: {:?}",
        fr.diagnostics
    );
    assert!(
        !fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::MissingExternal),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn source_overwrite_when_output_equals_donor_path() {
    let root = tempfile::tempdir().unwrap();
    let src_dir = root.path().join("src");
    let donors_dir = root.path().join("donors");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::create_dir_all(&donors_dir).unwrap();
    std::fs::write(src_dir.join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(donors_dir.join("Donor.S01E01.mkv"), b"y").unwrap();

    // The rendered output ("Donor.S01E01.mkv" in the donor directory) is made
    // to collide exactly with the external rule's resolved donor path.
    let donors_dir_str = donors_dir.to_str().unwrap();
    let profile_yaml = format!(
        r#"
profile_version: 1
input: {{ pattern: 'S(?<s>\d{{2}})E(?<e>\d{{2}})', extensions: [mkv] }}
output:
  filename: {{ template: 'Donor.{{match}}.mkv' }}
tracks:
  - match: {{ exact: {{ type: video }} }}
  - source:
      external: {{ path: '{donors_dir}', extensions: [mkv], match_to_source: true }}
    match: {{ exact: {{ type: audio }} }}
"#,
        donors_dir = donors_dir_str
    );
    let profile = from_str(&profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: src_dir.clone(),
        output: Some(donors_dir.clone()),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "Donor.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(root);

    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::SourceOverwrite),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn source_overwrite_is_batch_wide_not_per_primary() {
    let root = tempfile::tempdir().unwrap();
    let src_dir = root.path().join("src");
    let donors_dir = root.path().join("donors");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::create_dir_all(&donors_dir).unwrap();
    std::fs::write(src_dir.join("Show.S01E01.mkv"), b"x").unwrap(); // primary A
    std::fs::write(src_dir.join("Show.S01E02.mkv"), b"x").unwrap(); // primary B
    // B's external rule resolves this real donor via match_to_source (its
    // basename contains B's own identifier, "S01E02"). A's own identifier
    // ("S01E01") never matches it, so A's own rule evaluation never touches
    // this donor and A's per-primary donor set stays empty.
    std::fs::write(donors_dir.join("Donor.S01E02.mkv"), b"y").unwrap();

    // output.filename is a fixed literal (no {match} field), so *every*
    // primary in the batch renders to the identical name; with run.output
    // pointed at donors_dir, that name is byte-for-byte B's donor path - for
    // A as much as for B, even though A never resolved that donor itself.
    let donors_dir_str = donors_dir.to_str().unwrap();
    let profile_yaml = format!(
        r#"
profile_version: 1
input: {{ pattern: 'S(?<s>\d{{2}})E(?<e>\d{{2}})', extensions: [mkv] }}
output:
  filename: {{ template: 'Donor.S01E02.mkv' }}
tracks:
  - match: {{ exact: {{ type: video }} }}
  - source:
      external: {{ path: '{donors_dir}', extensions: [mkv], match_to_source: true }}
    match: {{ exact: {{ type: audio }} }}
    optional: true
"#,
        donors_dir = donors_dir_str
    );
    let profile = from_str(&profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: src_dir.clone(),
        output: Some(donors_dir.clone()),
        on_collision: Some(CollisionPolicy::Overwrite),
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "Show.S01E02.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "Donor.S01E02.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    std::mem::forget(root);

    // A never resolves B's donor itself, but its rendered output equals it -
    // a batch-wide SourceOverwrite, invisible to a check scoped to A's own
    // resolved donors.
    let a = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("Show.S01E01.mkv"))
        .unwrap();
    assert!(a.plan.is_none(), "diags: {:?}", a.diagnostics);
    assert!(
        a.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::SourceOverwrite),
        "diags: {:?}",
        a.diagnostics
    );
}

#[test]
fn bad_language_value_is_batch_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: zz } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
    assert!(
        batch
            .batch_diagnostics
            .iter()
            .any(|d| d.code == DiagCode::InvalidPropertyValue),
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
}
