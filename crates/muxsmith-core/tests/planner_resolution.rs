use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{AppliedChange, Batch, RunInputs, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::match_expr::Scalar;
use muxsmith_core::profile::model::CollisionPolicy;
use muxsmith_core::report::{DiagCode, Severity};

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
        ["Turkish", "tur", "tur", "tr"],
    ])
}

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");
// Task 8: one video track plus three attachments (id0 "a.ttf", id1 "b.otf",
// id2 "cover.jpg"), the fixture the brief's attachment-rule test cases use.
const WITH_ATTACHMENTS: &str = include_str!("fixtures/identify/with-attachments.json");

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

// Task 4: Plan/Assignment resolution-field defaults (Tasks 5-8 fill real
// resolution; this only asserts the wiring is defaulted correctly).
#[test]
fn plan_and_assignment_carry_resolution_field_defaults() {
    let batch = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: audio, language: en } }
    changes:
      language: tr
      track_name: X
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: audio, language: en } }
    changes:
      language: zzz
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
// string ("zzz" above): `resolve_changes` only accepts `Scalar::Str`.
#[test]
fn changes_language_non_string_value_is_invalid_property_value() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: en } }
    changes:
      language: true
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
fn keep_filename_on_mp4_source_replaces_extension_with_mkv() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mp4] }
tracks:
  - match: { exact: { type: video } }
  - match: { exact: { type: audio, language: en } }
"#;
    let batch = plan_one(p, "Show.S01E01.mp4", SERIES);
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
    let batch = plan_one(P_VIDEO_AUDIO, "Show.S01E01.mkv.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
fn empty_rendered_name_when_template_renders_to_dot() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
output:
  filename: { template: '.' }
tracks:
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
fn plan_two_same_output(policy: Option<CollisionPolicy>) -> Batch {
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
    std::mem::forget(root);
    batch
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
        let batch = plan_two_same_output(policy);
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
fn plan_one_with_existing_output(policy: CollisionPolicy) -> Batch {
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
    std::mem::forget(root);
    batch
}

#[test]
fn on_disk_collision_under_skip_is_warning_and_drops_plan() {
    let batch = plan_one_with_existing_output(CollisionPolicy::Skip);
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
    let batch = plan_one_with_existing_output(CollisionPolicy::Overwrite);
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
    let batch = plan_one_with_existing_output(CollisionPolicy::Error);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S03E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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

// Task 7: `chapters: drop` resolves to `ChapterSource::Drop`.
#[test]
fn chapters_drop_keyword_resolves_to_drop() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
chapters: drop
tracks:
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
  - match: { exact: { type: video } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", SERIES);
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
    std::mem::forget(dir);

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
    std::mem::forget(dir);

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
    std::mem::forget(dir);

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
// drop` removes everything else, reducing to `Subset` of just the matched id.
#[test]
fn attachment_select_rule_keeps_matched_and_unmatched_drop_removes_rest() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
attachments:
  unmatched: drop
  rules:
    - select: { substring: { file_name: .ttf } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::Subset(vec![0])
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
  - match: { exact: { type: video } }
attachments:
  unmatched: keep
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
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
  - match: { exact: { type: video } }
attachments:
  unmatched: drop
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::DropAll
    );
}

// Task 8: a `drop` rule covers exactly one attachment (`cover.jpg`, id2);
// `unmatched: keep` keeps the other two, reducing to `Subset([0, 1])`.
#[test]
fn attachment_drop_rule_covers_one_and_unmatched_keep_keeps_the_rest() {
    let p = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
attachments:
  unmatched: keep
  rules:
    - drop: { substring: { file_name: cover } }
"#;
    let batch = plan_one(p, "Show.S01E01.mkv", WITH_ATTACHMENTS);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(
        plan.attachments.primary,
        muxsmith_core::planner::PrimaryAttachments::Subset(vec![0, 1])
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
    std::mem::forget(dir);

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
    std::mem::forget(dir);

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
    std::mem::forget(dir);

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
