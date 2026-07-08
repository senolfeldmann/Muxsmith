use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
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

fn plan_one(profile_yaml: &str, file_name: &str, ident_json: &str) -> muxsmith_core::planner::Batch {
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
    assert!(fr.diagnostics.iter().any(|d| d.code == DiagCode::AmbiguousRule));
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
    assert!(fr.diagnostics.iter().any(|d| d.code == DiagCode::MissingTrack));
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
    assert!(fr.diagnostics.iter().any(|d| d.code == DiagCode::OverlappingRules));
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
