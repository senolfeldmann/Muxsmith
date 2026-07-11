use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{AppliedChange, Batch, RunInputs, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::match_expr::Scalar;
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::report::{DiagCode, Severity};

mod support;
use support::{FakeIdent, lang};

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");
// Task 8: one video track plus three attachments (id1 "a.ttf", id2 "b.otf",
// id3 "cover.jpg" - 1-based, mkvmerge -J wire format), the fixture the
// brief's attachment-rule test cases use.
const WITH_ATTACHMENTS: &str = include_str!("fixtures/identify/with-attachments.json");

fn plan_one(
    profile_yaml: &str,
    file_name: &str,
    ident_json: &str,
) -> (muxsmith_core::planner::Batch, tempfile::TempDir) {
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
    (batch, dir)
}

const P_VIDEO_AUDIO: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
    - match: { exact: { type: audio, language: en } }
"#;

#[test]
fn resolves_each_rule_to_one_track() {
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments.len(), 2);
    assert_eq!(plan.assignments[0].track_id, Some(0));
    assert_eq!(plan.assignments[1].track_id, Some(1));
}

// Task 4: Plan/Assignment resolution-field defaults (Tasks 5-8 fill real
// resolution; this only asserts the wiring is defaulted correctly).
#[test]
fn plan_and_assignment_carry_resolution_field_defaults() {
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments,
        muxsmith_core::planner::AttachmentPlan {
            primary: muxsmith_core::planner::PrimaryAttachments::KeepAll,
            add_files: vec![],
        }
    );
    assert_eq!(plan.chapters, muxsmith_core::planner::ChapterSource::Keep);
    assert_eq!(
        plan.tags,
        muxsmith_core::planner::TagFlags {
            global_keep: true,
            track_keep: true,
        }
    );
    assert_eq!(plan.title, muxsmith_core::planner::TitleAction::Keep);
    assert_eq!(plan.assignments[0].track_kind.as_deref(), Some("video"));
    assert!(plan.assignments[0].changes.is_empty());
}

// Task 5: settable `changes` resolve onto a matched assignment, in
// property-name order (the rule's `changes` map is a BTreeMap).
#[test]
fn changes_resolve_to_applied_changes_in_property_order() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio, language: en } }
      changes:
        language: tr
        track_name: X
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.assignments[0].changes,
        vec![
            AppliedChange {
                property: "language".into(),
                value: Scalar::Str("tr".into()),
            },
            AppliedChange {
                property: "track_name".into(),
                value: Scalar::Str("X".into()),
            },
        ]
    );
}

// Task 5: a settable `language` value validated at plan time (D2), at the
// point of application, distinct from the batch-level `match.exact.language`
// walk (`bad_language_value_is_batch_invalid_property_value`).
#[test]
fn invalid_changes_language_is_plan_time_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio, language: en } }
      changes:
        language: notalanguage
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::InvalidPropertyValue
                && d.config_path == "tracks[0].changes.language"),
        "diags: {:?}",
        fr.diagnostics
    );
}

// Task 5: a non-string `changes.language` value (e.g. a bool) is also
// InvalidPropertyValue at plan time, same as a recognized-but-invalid
// string ("notalanguage" above): `resolve_changes` only accepts `Scalar::Str`.
#[test]
fn changes_language_non_string_value_is_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio, language: en } }
      changes:
        language: true
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::InvalidPropertyValue
                && d.config_path == "tracks[0].changes.language"),
        "diags: {:?}",
        fr.diagnostics
    );
}

// Task 4 (D19): a well-formed BCP-47 regional tag is not an ISO 639 code, but
// `is_valid_value` accepts it too, so `changes.language: pt-BR` no longer
// hard-fails at plan time (mkvmerge, not muxsmith, is the authority on
// whether the tag actually exists).
#[test]
fn changes_language_pt_br_regional_tag_is_not_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio, language: en } }
      changes:
        language: pt-BR
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    assert!(
        !fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::InvalidPropertyValue),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn ambiguous_rule_when_two_tracks_match() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles, codec_kind: srt, language: en } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  rules:
    - match: { exact: { type: audio, language: de } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  rules:
    - match: { exact: { type: video } }
    - match: { exact: { type: audio, language: de } }
      optional: true
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  rules:
    - match: { exact: { type: video } }
    - match: { exact: { codec_id: 'V_MPEG4/ISO/AVC' } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none());
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::OverlappingRules)
    );
}

#[test]
fn overlapping_rules_names_every_claimant_not_just_the_first_two() {
    // Three rules each resolve to the single audio track (track 1): the
    // OverlappingRules diagnostic must name all three, not only the first
    // pair. The `rules` param is the rendered claimant list.
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio } }
    - match: { exact: { codec_id: A_AAC } }
    - match: { exact: { type: audio, language: en } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let overlap = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::OverlappingRules)
        .unwrap_or_else(|| panic!("expected OverlappingRules, got: {:?}", fr.diagnostics));
    let rules = &overlap.params["rules"];
    for expected in ["tracks[0]", "tracks[1]", "tracks[2]"] {
        assert!(
            rules.contains(expected),
            "claimant {expected} missing from rules list {rules:?}"
        );
    }
    assert_eq!(overlap.params["track"], "1");
}

#[test]
fn keep_filename_renders_mkv_output() {
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
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
fn keep_filename_on_mp4_source_replaces_extension_with_mkv() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mp4] }
tracks:
  rules:
    - match: { exact: { type: video } }
    - match: { exact: { type: audio, language: en } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mp4", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.output.file_name().unwrap(), "Show.S01E01.mkv");
}

// keep is file_stem + ".mkv" unconditionally (spec 4.8): it must never check
// whether the stem already looks like it ends in ".mkv" before appending --
// that conditional belongs to the template arm only. A source whose stem
// already ends in ".mkv" (a double extension, e.g. re-fed prior output) is
// the case that tells the two apart: the keep arm keeps the stem intact and
// still appends, the (wrong) shared-conditional version instead truncates
// one ".mkv" off.
#[test]
fn keep_filename_does_not_apply_the_templates_conditional_append() {
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.output.file_name().unwrap(), "Show.S01E01.mkv.mkv");
}

#[test]
fn template_filename_appends_mkv_when_missing() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: 'Custom' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.output.file_name().unwrap(), "Custom.mkv");
}

#[test]
fn template_filename_already_ending_in_mkv_is_not_doubled() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: 'Custom.mkv' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.output.file_name().unwrap(), "Custom.mkv");
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
  rules:
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
fn unrecognized_container_yields_unsupported_source_not_missing_track() {
    let json = r#"{ "container": { "recognized": false, "supported": true },
                    "file_name": "Show.S01E01.mkv", "identification_format_version": 20,
                    "tracks": [] }"#;
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", json);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let count = fr
        .diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnsupportedSource)
        .count();
    assert_eq!(count, 1, "diags: {:?}", fr.diagnostics);
    assert!(
        !fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::MissingTrack),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn unsupported_container_yields_unsupported_source_not_missing_track() {
    let json = r#"{ "container": { "recognized": true, "supported": false },
                    "file_name": "Show.S01E01.mkv", "identification_format_version": 20,
                    "tracks": [] }"#;
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", json);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let count = fr
        .diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnsupportedSource)
        .count();
    assert_eq!(count, 1, "diags: {:?}", fr.diagnostics);
    assert!(
        !fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::MissingTrack),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn recognized_supported_zero_tracks_stays_missing_track_not_unsupported_source() {
    // D21 decision #5: a recognized+supported container with zero tracks is
    // NOT UnsupportedSource; it stays a per-rule MissingTrack case.
    let json = r#"{ "container": { "recognized": true, "supported": true },
                    "file_name": "Show.S01E01.mkv", "identification_format_version": 20,
                    "tracks": [] }"#;
    let (batch, _dir) = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", json);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        !fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::UnsupportedSource),
        "diags: {:?}",
        fr.diagnostics
    );
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::MissingTrack),
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
  rules:
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
  rules:
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

// Plan-2 FINAL M2 / Plan-5.5 Task 7: a donor resolved only by a file whose
// own output render fails must still be protected. Three-way constellation:
// primary A's external rule resolves donor D (real, on disk), but A's own
// filename template renders empty (EmptyRenderedName), so A.plan is None and
// its assignments never survive into a `Plan`. Primary B's own external rule
// finds nothing (its sibling "donors" directory does not exist for B), yet
// B's rendered output happens to land on D's exact path. D is referenced
// SOLELY through A's (render-failed) assignments - the pre-fix
// `detect_source_overwrites` only gathered protected sources from
// `plan.is_some()` files, so this collision went undetected and B would
// have silently overwritten D.
#[test]
fn source_overwrite_protects_donor_of_render_failed_file() {
    let root = tempfile::tempdir().unwrap();
    let a_dir = root.path().join("a_dir");
    let b_dir = root.path().join("b_dir");
    let donors_dir = a_dir.join("donors");
    std::fs::create_dir_all(&donors_dir).unwrap();
    std::fs::create_dir_all(&b_dir).unwrap();
    std::fs::write(a_dir.join("Prime.mkv"), b"a").unwrap(); // primary A
    std::fs::write(b_dir.join("PrimeZ.mkv"), b"b").unwrap(); // primary B
    std::fs::write(donors_dir.join("Z.mkv"), b"d").unwrap(); // donor D, resolved only by A

    // `tag` is optional and only present in B's own filename: A's template
    // renders empty (EmptyRenderedName); B's renders "Z.mkv", exactly D's
    // basename. The external locator path is relative ("donors"), resolved
    // against each primary's own directory, so it only finds D for A -
    // `b_dir/donors` does not exist, and B's rule (optional) finds nothing.
    let profile_yaml = r#"
profile_version: 1
input: { pattern: 'Prime(?<tag>Z)?', extensions: [mkv] }
output:
  filename: { template: '{tag}' }
tracks:
  rules:
    - match: { exact: { type: video } }
    - source:
        external: { path: 'donors', extensions: [mkv] }
      match: { exact: { type: audio } }
      optional: true
"#;
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: root.path().to_path_buf(),
        // Every primary's output lands in donors_dir, so B's rendered
        // "Z.mkv" collides byte-for-byte with the donor A alone resolved.
        output: Some(donors_dir.clone()),
        // Overwrite, not the Error default: proves SourceOverwrite is what
        // stops this, not the ordinary on-disk-collision path. Under
        // Overwrite, an on-disk collision that is NOT a batch input is only
        // Info-severity and does not null the plan (spec 5.2) - exactly the
        // "silent data loss" M2 described: without the fix, B's plan
        // survives and a real run overwrites donor D.
        on_collision: Some(CollisionPolicy::Overwrite),
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Prime.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "PrimeZ.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "Z.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let a = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("Prime.mkv"))
        .unwrap();
    let b = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("PrimeZ.mkv"))
        .unwrap();

    // A's own render fails on its own terms, independent of this fix.
    assert!(a.plan.is_none(), "diags: {:?}", a.diagnostics);
    assert!(
        a.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        a.diagnostics
    );

    // B's rendered output collides with the donor A resolved; must be
    // caught even though A's own plan never rendered.
    assert!(b.plan.is_none(), "diags: {:?}", b.diagnostics);
    assert!(
        b.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::SourceOverwrite),
        "diags: {:?}",
        b.diagnostics
    );
}

// Plan-5.5 Task 7.5: #7 completion. T7 above covers a track-rule donor; this
// is the same class through an attachment donor
// (`attachments.rules[i].add`, resolved by `resolve_attachments`). Same
// three-way constellation: primary A's `add` locator resolves donor D (real,
// on disk), but A's own filename template renders empty
// (EmptyRenderedName), so A.plan is None; primary B's own `add` locator
// finds nothing (its sibling "donors" directory does not exist for B), yet
// B's rendered output lands on D's exact path. D is referenced SOLELY
// through A's (render-failed) `attachments.add_files` - before this fix,
// `resolved_sources` only gathered `Assignment.source` (track donors), never
// `AttachmentPlan.add_files`, so this collision went undetected and B would
// have silently overwritten D.
#[test]
fn source_overwrite_protects_attachment_donor_of_render_failed_file() {
    let root = tempfile::tempdir().unwrap();
    let a_dir = root.path().join("a_dir");
    let b_dir = root.path().join("b_dir");
    let donors_dir = a_dir.join("donors");
    std::fs::create_dir_all(&donors_dir).unwrap();
    std::fs::create_dir_all(&b_dir).unwrap();
    std::fs::write(a_dir.join("Prime.mkv"), b"a").unwrap(); // primary A
    std::fs::write(b_dir.join("PrimeZ.mkv"), b"b").unwrap(); // primary B
    std::fs::write(donors_dir.join("Z.mkv"), b"d").unwrap(); // attachment donor D, resolved only by A

    // `tag` is optional and only present in B's own filename: A's template
    // renders empty (EmptyRenderedName); B's renders "Z.mkv", exactly D's
    // basename. The `add` locator path is relative ("donors"), resolved
    // against each primary's own directory, so it only finds D for A -
    // `b_dir/donors` does not exist, and B's own `add` locator finds
    // nothing (a MissingExternal warning, not an error - spec 4.9).
    let profile_yaml = r#"
profile_version: 1
input: { pattern: 'Prime(?<tag>Z)?', extensions: [mkv] }
output:
  filename: { template: '{tag}' }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - add: { path: 'donors', extensions: [mkv] }
"#;
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: root.path().to_path_buf(),
        // Every primary's output lands in donors_dir, so B's rendered
        // "Z.mkv" collides byte-for-byte with the attachment donor A alone
        // resolved.
        output: Some(donors_dir.clone()),
        // Overwrite, not the Error default: proves SourceOverwrite is what
        // stops this, not the ordinary on-disk-collision path (same
        // reasoning as T7's test above).
        on_collision: Some(CollisionPolicy::Overwrite),
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Prime.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "PrimeZ.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "Z.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let a = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("Prime.mkv"))
        .unwrap();
    let b = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("PrimeZ.mkv"))
        .unwrap();

    // A's own render fails on its own terms, independent of this fix.
    assert!(a.plan.is_none(), "diags: {:?}", a.diagnostics);
    assert!(
        a.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        a.diagnostics
    );

    // B's rendered output collides with the attachment donor A resolved;
    // must be caught even though A's own plan never rendered.
    assert!(b.plan.is_none(), "diags: {:?}", b.diagnostics);
    assert!(
        b.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::SourceOverwrite),
        "diags: {:?}",
        b.diagnostics
    );
}

// Plan-5.5 Task 7.6: #7 class closure. T7 and T7.5 above cover a track-rule
// donor and an attachment donor; this is the same class through the
// chapters donor (`profile.chapters.external`, resolved by
// `resolve_chapters`). Same three-way constellation, adapted for chapters'
// stricter uniqueness rule (spec 4.9: unlike a track rule's external
// source, there is no `optional` escape - zero matches is always
// `MissingExternal`) - so, unlike T7/T7.5 where B's own locator finds
// nothing because its sibling "donors" directory does not exist at all,
// here B needs its OWN successful chapters resolution (a distinct,
// harmless donor under its own "donors" directory) so its plan survives
// long enough to reach `detect_source_overwrites`. Primary A's chapters
// locator resolves donor D (real, on disk), but A's own filename template
// renders empty (EmptyRenderedName), so A.plan is None; primary B's own
// chapters locator resolves its own distinct donor (`b_dir/donors/Z.mkv`),
// yet B's rendered output lands on D's exact path (`a_dir/donors/Z.mkv`).
// D is referenced SOLELY through A's (render-failed)
// `ChapterSource::External` - before this fix, `resolved_sources` never
// gathered chapters at all (only `Assignment.source` and, since Task 7.5,
// `AttachmentPlan.add_files`), so this collision went undetected and B
// would have silently overwritten D.
#[test]
fn source_overwrite_protects_chapters_donor_of_render_failed_file() {
    let root = tempfile::tempdir().unwrap();
    let a_dir = root.path().join("a_dir");
    let b_dir = root.path().join("b_dir");
    let a_donors = a_dir.join("donors");
    let b_donors = b_dir.join("donors");
    std::fs::create_dir_all(&a_donors).unwrap();
    std::fs::create_dir_all(&b_donors).unwrap();
    std::fs::write(a_dir.join("Prime.mkv"), b"a").unwrap(); // primary A
    std::fs::write(b_dir.join("PrimeZ.mkv"), b"b").unwrap(); // primary B
    std::fs::write(a_donors.join("Z.mkv"), b"d").unwrap(); // chapters donor D, resolved only by A
    std::fs::write(b_donors.join("Z.mkv"), b"e").unwrap(); // B's own distinct chapters donor

    // `tag` is optional and only present in B's own filename: A's template
    // renders empty (EmptyRenderedName); B's renders "Z.mkv", exactly D's
    // basename. The chapters locator (no `match_pattern`/`match_to_source`,
    // same as T7's track-rule locator) matches every file in its target
    // directory; each primary's "donors" subdirectory holds exactly one
    // `.mkv` file of its own, so each resolves to exactly one hit and
    // neither triggers chapters' `MissingExternal`.
    let profile_yaml = r#"
profile_version: 1
input: { pattern: 'Prime(?<tag>Z)?', extensions: [mkv] }
output:
  filename: { template: '{tag}' }
tracks:
  rules:
    - match: { exact: { type: video } }
chapters:
  external: { path: 'donors', extensions: [mkv] }
"#;
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: root.path().to_path_buf(),
        // Every primary's output lands in a_donors, so B's rendered
        // "Z.mkv" collides byte-for-byte with the chapters donor A alone
        // resolved.
        output: Some(a_donors.clone()),
        // Overwrite, not the Error default: proves SourceOverwrite is what
        // stops this, not the ordinary on-disk-collision path (same
        // reasoning as T7's and T7.5's tests above).
        on_collision: Some(CollisionPolicy::Overwrite),
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Prime.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    by_name.insert(
        "PrimeZ.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let a = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("Prime.mkv"))
        .unwrap();
    let b = batch
        .files
        .iter()
        .find(|f| f.source.ends_with("PrimeZ.mkv"))
        .unwrap();

    // A's own render fails on its own terms, independent of this fix.
    assert!(a.plan.is_none(), "diags: {:?}", a.diagnostics);
    assert!(
        a.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        a.diagnostics
    );

    // B's rendered output collides with the chapters donor A resolved;
    // must be caught even though A's own plan never rendered.
    assert!(b.plan.is_none(), "diags: {:?}", b.diagnostics);
    assert!(
        b.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::SourceOverwrite),
        "diags: {:?}",
        b.diagnostics
    );
}

#[test]
fn empty_rendered_name_when_template_renders_to_dot() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: '.' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        fr.diagnostics
    );
}

#[test]
fn empty_rendered_name_when_template_renders_to_empty_string() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: '' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        fr.diagnostics
    );
}

// I1 regression: a template that renders to exactly ".mkv" is non-empty and
// not "."/".." BEFORE ".mkv" is appended, so a check scoped to the
// pre-append value never fires; the template arm then sees the rendered
// value already ends in ".mkv" and appends nothing, producing the hidden,
// empty-stem output file ".mkv" with exit 0. The invariant must be checked
// on the FINAL name's stem (after ".mkv" is stripped back off), not on the
// pre-append rendered value.
#[test]
fn empty_rendered_name_when_template_renders_to_literal_mkv() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: '.mkv' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        fr.diagnostics
    );
}

// Same failure mode as above, reached a different way: the template field
// itself renders empty (an optional capture group, "x" here, that exists in
// input.pattern but does not participate in this file's match, so `Ctx`
// never binds it and it interpolates as "") followed by a literal ".mkv"
// segment. Confirms the fix catches "empty field + literal .mkv", not just
// the bare ".mkv" literal.
#[test]
fn empty_rendered_name_when_template_field_renders_empty_before_literal_mkv() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})(?<x>Q)?', extensions: [mkv] }
output:
  filename: { template: '{x}.mkv' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    assert!(
        fr.diagnostics
            .iter()
            .any(|d| d.code == DiagCode::EmptyRenderedName),
        "diags: {:?}",
        fr.diagnostics
    );
}

// Two primaries whose profile renders both to a fixed literal name (no
// {match} field), in a sibling `out/` directory so pre-existing files in it
// are never rediscovered as extra primaries (input.recursive defaults true,
// scoped to `run.source` only). `policy` is threaded through as the run
// override; `None` exercises the profile default (`error`).
fn plan_two_same_output(policy: Option<CollisionPolicy>) -> (Batch, tempfile::TempDir) {
    let root = tempfile::tempdir().unwrap();
    let src_dir = root.path().join("src");
    let out_dir = root.path().join("out");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::write(src_dir.join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(src_dir.join("Show.S01E02.mkv"), b"x").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: 'Same.mkv' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: src_dir,
        output: Some(out_dir),
        on_collision: policy,
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
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    (batch, root)
}

#[test]
fn two_planned_outputs_to_same_path_are_always_output_collision_error() {
    // Decision #3: an internally inconsistent batch is always an error,
    // independent of on_collision -- neither skip nor overwrite can pick a
    // winner between two plans claiming the same path.
    for policy in [
        None,
        Some(CollisionPolicy::Error),
        Some(CollisionPolicy::Overwrite),
        Some(CollisionPolicy::Skip),
    ] {
        let (batch, _dir) = plan_two_same_output(policy);
        assert_eq!(batch.files.len(), 2, "policy {policy:?}");
        for fr in &batch.files {
            assert!(
                fr.plan.is_none(),
                "policy {policy:?}: diags: {:?}",
                fr.diagnostics
            );
            let d = fr
                .diagnostics
                .iter()
                .find(|d| d.code == DiagCode::OutputCollision)
                .unwrap_or_else(|| {
                    panic!(
                        "policy {policy:?}: expected OutputCollision, got: {:?}",
                        fr.diagnostics
                    )
                });
            assert_eq!(d.severity, Severity::Error, "policy {policy:?}: {d:?}");
        }
    }
}

// A single primary whose "keep" output name already exists on disk (a file
// that is not itself a batch input; sibling out/ dir keeps it out of
// discovery's recursive scan of src/).
fn plan_one_with_existing_output(policy: CollisionPolicy) -> (Batch, tempfile::TempDir) {
    let root = tempfile::tempdir().unwrap();
    let src_dir = root.path().join("src");
    let out_dir = root.path().join("out");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::create_dir_all(&out_dir).unwrap();
    std::fs::write(src_dir.join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(out_dir.join("Show.S01E01.mkv"), b"pre-existing").unwrap();

    let profile = from_str(P_VIDEO_AUDIO, Format::Yaml).unwrap();
    let run = RunInputs {
        source: src_dir,
        output: Some(out_dir),
        on_collision: Some(policy),
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    (batch, root)
}

#[test]
fn on_disk_collision_under_skip_is_warning_and_drops_plan() {
    let (batch, _dir) = plan_one_with_existing_output(CollisionPolicy::Skip);
    let fr = &batch.files[0];
    // Bug E: "skip" means the output is not produced, even though the
    // diagnostic itself is only a warning.
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::OutputCollision)
        .unwrap_or_else(|| panic!("expected OutputCollision, got: {:?}", fr.diagnostics));
    assert_eq!(d.severity, Severity::Warning);
}

#[test]
fn on_disk_collision_under_overwrite_is_info_and_keeps_plan() {
    let (batch, _dir) = plan_one_with_existing_output(CollisionPolicy::Overwrite);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::OutputCollision)
        .unwrap_or_else(|| panic!("expected OutputCollision, got: {:?}", fr.diagnostics));
    assert_eq!(d.severity, Severity::Info);
}

#[test]
fn on_disk_collision_under_error_is_error_and_drops_plan() {
    let (batch, _dir) = plan_one_with_existing_output(CollisionPolicy::Error);
    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::OutputCollision)
        .unwrap_or_else(|| panic!("expected OutputCollision, got: {:?}", fr.diagnostics));
    assert_eq!(d.severity, Severity::Error);
}

// Task 6: `title: clear` resolves to `TitleAction::Clear`.
#[test]
fn title_clear_resolves_to_clear() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
title: clear
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.title, muxsmith_core::planner::TitleAction::Clear);
}

// Task 6: `title: keep` resolves to `TitleAction::Keep` via the real
// resolution path (not just the Task-4 hardcoded default).
#[test]
fn title_keep_resolves_to_keep() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
title: keep
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.title, muxsmith_core::planner::TitleAction::Keep);
}

// Task 6: a title template renders via the same literal-mode engine as the
// filename template; `{season}` is a raw capture (no filter applied).
#[test]
fn title_template_renders_raw_capture_into_set() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<e>\d{2})', extensions: [mkv] }
title: { template: 'Show S{season}' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S03E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.title,
        muxsmith_core::planner::TitleAction::Set("Show S03".into())
    );
}

// Task 6: title has no path-separator/empty-name invariants (unlike
// filenames, spec 4.9 vs 4.8): a template rendering to an empty string is a
// legitimate `Set("")`, not an error.
#[test]
fn title_template_rendering_empty_is_a_legitimate_set() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})(?<x>Q)?', extensions: [mkv] }
title: { template: '{x}' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.title,
        muxsmith_core::planner::TitleAction::Set(String::new())
    );
}

// Task 6: `source_stem` is available to a title template, exactly as it is
// to output.filename templates (validate.rs allows it identically for
// both, so the resolve-time Ctx must supply it identically too).
#[test]
fn title_template_supports_source_stem_field() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
title: { template: '{source_stem}' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.title,
        muxsmith_core::planner::TitleAction::Set("Show.S01E01".into())
    );
}

// Task 6: `tags: { global: drop, track: keep }` resolves to the matching
// `TagFlags`.
#[test]
fn tags_global_drop_track_keep_resolves_to_flags() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tags: { global: drop, track: keep }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.tags,
        muxsmith_core::planner::TagFlags {
            global_keep: false,
            track_keep: true,
        }
    );
}

#[test]
fn bad_language_value_is_batch_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio, language: notalanguage } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    assert!(
        batch
            .batch_diagnostics
            .iter()
            .any(|d| d.code == DiagCode::InvalidPropertyValue),
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
}

/// A [`FakeIdent`] wrapper that also answers [`Identify::known_extensions`],
/// standing in for a runtime whose `--list-types` output is known (Task 5,
/// #3): the batch-validation tests below need to control this independently
/// of the per-file identification `FakeIdent` already provides.
struct FakeIdentWithExtensions {
    inner: FakeIdent,
    known_extensions: Option<Vec<String>>,
}

impl Identify for FakeIdentWithExtensions {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        self.inner.identify(path)
    }

    fn known_extensions(&mut self) -> Option<Vec<String>> {
        self.known_extensions.clone()
    }
}

fn plan_one_with_extensions(
    profile_yaml: &str,
    file_name: &str,
    ident_json: &str,
    known_extensions: Option<Vec<&str>>,
) -> (Batch, tempfile::TempDir) {
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
    let mut ident = FakeIdentWithExtensions {
        inner: FakeIdent { by_name },
        known_extensions: known_extensions.map(|exts| exts.into_iter().map(String::from).collect()),
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    (batch, dir)
}

// Task 5 (#3): `input.extensions` checked once per batch against the
// runtime's `--list-types` output, mirroring
// `bad_language_value_is_batch_invalid_property_value`'s layout.
#[test]
fn unknown_extension_is_batch_warning_naming_the_extension() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv, mp4a] }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one_with_extensions(
        p,
        "Show.S01E01.mkv",
        SERIES,
        Some(vec!["mkv", "mp4", "avi"]),
    );
    let unknown: Vec<_> = batch
        .batch_diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnknownExtension)
        .collect();
    assert_eq!(
        unknown.len(),
        1,
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
    assert_eq!(unknown[0].severity, Severity::Warning);
    assert_eq!(
        unknown[0].params.get("extension").map(String::as_str),
        Some("mp4a")
    );
    // Batch continues: the file still resolves to a plan despite the warning.
    assert!(
        batch.files[0].plan.is_some(),
        "diags: {:?}",
        batch.files[0].diagnostics
    );
}

// Task 5 (#3): the runtime's extension list is unavailable (mkvmerge
// absent/query failed): the check degrades to a no-op rather than blocking
// planning, unlike `lang`'s hard batch-planning precondition.
#[test]
fn unknown_extension_check_degrades_when_runtime_unavailable() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv, mp4a] }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one_with_extensions(p, "Show.S01E01.mkv", SERIES, None);
    assert!(
        !batch
            .batch_diagnostics
            .iter()
            .any(|d| d.code == DiagCode::UnknownExtension),
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
    assert!(
        batch.files[0].plan.is_some(),
        "diags: {:?}",
        batch.files[0].diagnostics
    );
}

// Task 5 (#3): extension matching (and its validation) is case-insensitive
// (model.rs `Input.extensions` doc).
#[test]
fn known_extension_case_insensitive_is_not_flagged() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [MKV] }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one_with_extensions(
        p,
        "Show.S01E01.mkv",
        SERIES,
        Some(vec!["mkv", "mp4", "avi"]),
    );
    assert!(
        !batch
            .batch_diagnostics
            .iter()
            .any(|d| d.code == DiagCode::UnknownExtension),
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
}

// Task 5.9 (spec 4.6): a track rule's external locator's `extensions` is
// batch-checked against the runtime's `--list-types` output too, same as
// `input.extensions` (Task 5). `optional: true` keeps the locator's
// zero-hit resolution from adding its own `MissingExternal` error, so the
// `UnknownExtension` warning is the only diagnostic under test.
#[test]
fn unknown_extension_in_track_rule_locator_is_batch_warning() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
    - source:
        external: { path: '.', extensions: [srt, mp4a] }
      match: { exact: { type: subtitles } }
      optional: true
"#;
    let (batch, _dir) = plan_one_with_extensions(
        p,
        "Show.S01E01.mkv",
        SERIES,
        Some(vec!["mkv", "srt", "avi"]),
    );
    let unknown: Vec<_> = batch
        .batch_diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnknownExtension)
        .collect();
    assert_eq!(
        unknown.len(),
        1,
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
    assert_eq!(unknown[0].severity, Severity::Warning);
    assert_eq!(
        unknown[0].config_path,
        "tracks[1].source.external.extensions[1]"
    );
    assert_eq!(
        unknown[0].params.get("extension").map(String::as_str),
        Some("mp4a")
    );
    // Batch continues: the file still resolves to a plan despite the warning.
    assert!(
        batch.files[0].plan.is_some(),
        "diags: {:?}",
        batch.files[0].diagnostics
    );
}

// Task 5.9 (spec 4.6): a `chapters` external locator's `extensions` is
// checked the same way. A real `.xml` donor keeps chapters resolution from
// raising its own `MissingExternal` error (chapters has no `optional`
// escape, unlike a track rule's external source), isolating the
// `UnknownExtension` warning under test.
#[test]
fn unknown_extension_in_chapters_locator_is_batch_warning() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("Show.S01E01.xml"), b"<Chapters/>").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters:
  external: { path: '.', extensions: [xml, mp4a], match_to_source: true }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdentWithExtensions {
        inner: FakeIdent { by_name },
        known_extensions: Some(vec!["mkv".into(), "xml".into()]),
    };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let unknown: Vec<_> = batch
        .batch_diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnknownExtension)
        .collect();
    assert_eq!(
        unknown.len(),
        1,
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
    assert_eq!(unknown[0].config_path, "chapters.external.extensions[1]");
    assert_eq!(
        unknown[0].params.get("extension").map(String::as_str),
        Some("mp4a")
    );
    assert!(
        batch.files[0].plan.is_some(),
        "diags: {:?}",
        batch.files[0].diagnostics
    );
}

// Task 5.9 (spec 4.6): an `attachments.rules[i].add` locator's `extensions`
// is checked the same way. An `add` locator's zero-hit case is a warning,
// not an error (spec 4.9), so no donor file is needed to keep the plan
// resolving.
#[test]
fn unknown_extension_in_attachments_add_locator_is_batch_warning() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - add: { path: '.', extensions: [ttf, mp4a] }
"#;
    let (batch, _dir) = plan_one_with_extensions(
        p,
        "Show.S01E01.mkv",
        SERIES,
        Some(vec!["mkv", "ttf", "otf"]),
    );
    let unknown: Vec<_> = batch
        .batch_diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnknownExtension)
        .collect();
    assert_eq!(
        unknown.len(),
        1,
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
    assert_eq!(
        unknown[0].config_path,
        "attachments.rules[0].add.extensions[1]"
    );
    assert_eq!(
        unknown[0].params.get("extension").map(String::as_str),
        Some("mp4a")
    );
    assert!(
        batch.files[0].plan.is_some(),
        "diags: {:?}",
        batch.files[0].diagnostics
    );
}

// Task 5.9: T5's walk never deduped `input.extensions` occurrences by
// value (two entries with the same unknown string each get their own
// diagnostic, keyed by their own index/path); the locator walk keeps that
// behavior rather than introducing batch-wide dedup by extension value, so
// the same unknown extension repeated across `input.extensions` and a
// locator yields two independent warnings.
#[test]
fn unknown_extension_repeated_across_input_and_locator_is_not_deduped() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv, mp4a] }
tracks:
  rules:
    - match: { exact: { type: video } }
    - source:
        external: { path: '.', extensions: [mp4a] }
      match: { exact: { type: subtitles } }
      optional: true
"#;
    let (batch, _dir) =
        plan_one_with_extensions(p, "Show.S01E01.mkv", SERIES, Some(vec!["mkv", "srt"]));
    let unknown: Vec<_> = batch
        .batch_diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::UnknownExtension)
        .collect();
    assert_eq!(
        unknown.len(),
        2,
        "batch diags: {:?}",
        batch.batch_diagnostics
    );
    let paths: Vec<&str> = unknown.iter().map(|d| d.config_path.as_str()).collect();
    assert!(paths.contains(&"input.extensions[1]"), "paths: {paths:?}");
    assert!(
        paths.contains(&"tracks[1].source.external.extensions[0]"),
        "paths: {paths:?}"
    );
}

// Task 7: `chapters: drop` resolves to `ChapterSource::Drop`.
#[test]
fn chapters_drop_keyword_resolves_to_drop() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters: drop
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.chapters, muxsmith_core::planner::ChapterSource::Drop);
}

// Task 7: `chapters: keep` resolves to `ChapterSource::Keep` (also the
// default already covered by Task 4's tests, but exercised explicitly here
// since Task 7 is what makes the keyword branch real).
#[test]
fn chapters_keep_keyword_resolves_to_keep() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters: keep
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.chapters, muxsmith_core::planner::ChapterSource::Keep);
}

// Task 7: an external chapters locator with exactly one matching donor
// resolves to `ChapterSource::External(<that path>)`. A chapters file is
// never run through Identify (it is XML/simple, not an mkvmerge source), so
// the fixture map only needs the primary.
#[test]
fn chapters_external_one_match_resolves_to_external_path() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("Show.S01E01.xml"), b"<Chapters/>").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters:
  external: { path: '.', extensions: [xml], match_to_source: true }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    let expected = dir.path().join("Show.S01E01.xml");

    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.chapters,
        muxsmith_core::planner::ChapterSource::External(expected)
    );
}

// Task 7: zero matches for an external chapters locator is a hard error
// (`MissingExternal` at `chapters.external`); the file gets no plan. Unlike
// a track rule's external source, chapters has no `optional` escape.
#[test]
fn chapters_external_zero_matches_yields_missing_external_and_no_plan() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters:
  external: { path: '.', extensions: [xml], match_to_source: true }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::MissingExternal)
        .unwrap_or_else(|| panic!("expected MissingExternal, got: {:?}", fr.diagnostics));
    assert_eq!(d.config_path, "chapters.external");
}

// Task 7: two matches for an external chapters locator is `AmbiguousExternal`
// at `chapters.external`, with `count` = 2, and no plan.
#[test]
fn chapters_external_two_matches_yields_ambiguous_external() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("Show.S01E01.xml"), b"<Chapters/>").unwrap();
    std::fs::write(dir.path().join("Show.S01E01.alt.xml"), b"<Chapters/>").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters:
  external: { path: '.', extensions: [xml], match_to_source: true }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let fr = &batch.files[0];
    assert!(fr.plan.is_none(), "diags: {:?}", fr.diagnostics);
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::AmbiguousExternal)
        .unwrap_or_else(|| panic!("expected AmbiguousExternal, got: {:?}", fr.diagnostics));
    assert_eq!(d.config_path, "chapters.external");
    assert_eq!(d.params.get("count").map(String::as_str), Some("2"));
}

// Task 8: a `select` rule keeps only the attachments it matches; `unmatched:
// drop` removes everything else, reducing to `Subset` of just the matched id
// (1-based: WITH_ATTACHMENTS' "a.ttf" is id 1, mkvmerge -J wire format).
#[test]
fn attachment_select_rule_keeps_matched_and_unmatched_drop_removes_rest() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  unmatched: drop
  rules:
    - select: { substring: { file_name: .ttf } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::Subset(vec![1])
    );
}

// Task 8: no rules at all, `unmatched: keep` -> every attachment falls
// through to `unmatched` and is kept, reducing to `KeepAll`.
#[test]
fn attachment_no_rules_and_unmatched_keep_resolves_to_keep_all() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  unmatched: keep
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::KeepAll
    );
}

// Task 8: no rules at all, `unmatched: drop` -> every attachment falls
// through to `unmatched` and is dropped, reducing to `DropAll`.
#[test]
fn attachment_no_rules_and_unmatched_drop_resolves_to_drop_all() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  unmatched: drop
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::DropAll
    );
}

// Task 8: a `drop` rule covers exactly one attachment (`cover.jpg`, id 3);
// `unmatched: keep` keeps the other two, reducing to `Subset([1, 2])`
// (1-based: WITH_ATTACHMENTS' "a.ttf"/"b.otf" are ids 1/2, mkvmerge -J wire
// format).
#[test]
fn attachment_drop_rule_covers_one_and_unmatched_keep_keeps_the_rest() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  unmatched: keep
  rules:
    - drop: { substring: { file_name: cover } }
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::Subset(vec![1, 2])
    );
}

// Task 8 (D12): an `add` locator attaches ALL files it matches, not just one;
// both fonts land in `add_files`, in the locator's sorted order.
#[test]
fn attachment_add_locator_attaches_all_matching_files() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("b.ttf"), b"font-b").unwrap();
    std::fs::write(dir.path().join("a.ttf"), b"font-a").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - add: { path: '.', extensions: [ttf] }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    let expected = vec![dir.path().join("a.ttf"), dir.path().join("b.ttf")];

    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.attachments.add_files, expected);
}

// Task 8 (D12): two `add` rules that both match the same file attach it
// exactly once (dedup by path, first-seen order).
#[test]
fn attachment_add_two_rules_matching_same_file_is_deduped() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("font.ttf"), b"font").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - add: { path: '.', extensions: [ttf] }
    - add: { path: '.', extensions: [ttf] }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    let expected = vec![dir.path().join("font.ttf")];

    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.attachments.add_files, expected);
}

// Task 8 (D12): an `add` locator matching zero files is a WARNING
// `MissingExternal` at `attachments.rules[i].add`, an auxiliary payload that
// must not suppress the plan (unlike a track rule's or chapters' external
// zero-match, both of which are errors).
#[test]
fn attachment_add_locator_zero_matches_yields_missing_external_warning_and_plan_survives() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - add: { path: '.', extensions: [ttf] }
"#;
    let profile = from_str(p, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(SERIES).unwrap(),
    );
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());

    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert!(plan.attachments.add_files.is_empty());
    let d = fr
        .diagnostics
        .iter()
        .find(|d| d.code == DiagCode::MissingExternal)
        .unwrap_or_else(|| panic!("expected MissingExternal, got: {:?}", fr.diagnostics));
    assert_eq!(d.config_path, "attachments.rules[0].add");
    assert_eq!(d.severity, Severity::Warning);
}

// Task 6 (#6, ROADMAP "Zero-track plan warning"): a plan resolving to zero
// output tracks used to mux a valid-but-empty MKV silently (exit 0, no
// diagnostic - verified live against mkvmerge in the Plan-3 whole-branch
// review). Decided (Şenol 2026-07-11, sweep walkthrough #6): a per-file
// WARNING, one sane default, no error/skip alternative (that variance is
// parked in IDEAS.md #5, deliberately not built).
#[test]
fn empty_plan_warns_when_all_optional_rules_match_nothing() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio, language: de } }
      optional: true
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    // The plan still renders, unchanged: a satisfied `optional` rule is not
    // an error (spec 5.1), just an unmatched assignment.
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments.len(), 1);
    assert_eq!(plan.assignments[0].track_id, None);
    let empty_plan_warnings: Vec<_> = fr
        .diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::EmptyPlan)
        .collect();
    assert_eq!(empty_plan_warnings.len(), 1, "diags: {:?}", fr.diagnostics);
    assert_eq!(empty_plan_warnings[0].severity, Severity::Warning);
    assert_eq!(
        empty_plan_warnings[0].file.as_deref(),
        Some(fr.source.as_path())
    );
}

// Task 6 (D20 semantics): under `tracks.unmatched: keep`, the primary's own
// tracks always pass through untouched, even when the only rule is
// optional and matches nothing itself - D20's "keep = match to what is
// already there" means that passthrough already counts as matched, so
// `EmptyPlan` naturally cannot fire on a keep-mode plan (as long as the
// primary itself has at least one track, always true here via SERIES).
// Same zero-rule-match profile as the warning test above, plus `unmatched:
// keep`, to isolate the one variable that changes the outcome.
#[test]
fn empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  unmatched: keep
  rules:
    - match: { exact: { type: audio, language: de } }
      optional: true
"#;
    let (batch, _dir) = plan_one(p, "Show.S01E01.mkv", SERIES);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert!(plan.keep_unmatched);
    assert!(!plan.primary_track_ids.is_empty());
    assert!(
        !fr.diagnostics.iter().any(|d| d.code == DiagCode::EmptyPlan),
        "diags: {:?}",
        fr.diagnostics
    );
}
