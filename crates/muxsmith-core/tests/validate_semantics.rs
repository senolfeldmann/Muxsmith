use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::{DiagCode, Severity};

fn profile(tracks_yaml: &str) -> muxsmith_core::profile::Profile {
    let indented: String = tracks_yaml
        .lines()
        .map(|l| format!("  {l}"))
        .collect::<Vec<_>>()
        .join("\n");
    let y = format!(
        r#"
profile_version: 1
input: {{ pattern: 'S(?<season>\d{{2}})E(?<episode>\d{{2}})', extensions: [mkv] }}
tracks:
  rules:
{indented}
"#
    );
    from_str(&y, Format::Yaml).unwrap()
}

fn codes(p: &muxsmith_core::profile::Profile) -> Vec<DiagCode> {
    validate(p).into_iter().map(|d| d.code).collect()
}

#[test]
fn reference_profile_validates_clean() {
    let text = include_str!("fixtures/reference.yaml");
    let p = from_str(text, Format::Yaml).unwrap();
    let errors: Vec<_> = validate(&p)
        .into_iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert_eq!(errors, vec![], "reference profile must have zero errors");
}

#[test]
fn wrong_profile_version_is_rejected() {
    let mut p = profile("  - match: { exact: { type: video } }");
    p.profile_version = 2;
    assert!(codes(&p).contains(&DiagCode::UnsupportedProfileVersion));
}

#[test]
fn empty_tracks_list_is_rejected() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::NoTrackRules));
}

#[test]
fn unknown_match_property_is_flagged_with_path() {
    let p = profile("  - match: { exact: { colour_depth: 10 } }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::UnknownProperty)
        .unwrap();
    assert_eq!(d.config_path, "tracks[0].match.exact.colour_depth");
    assert_eq!(d.params["property"], "colour_depth");
}

#[test]
fn substring_on_boolean_property_is_flagged() {
    let p = profile("  - match: { substring: { forced_track: 'yes' } }");
    assert!(codes(&p).contains(&DiagCode::NotStringProperty));
}

#[test]
fn exact_value_type_mismatch_is_flagged() {
    let p = profile("  - match: { exact: { forced_track: 'yes' } }");
    assert!(codes(&p).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn integer_accepted_for_float_property_but_not_reverse() {
    // audio_sampling_frequency is number (Float) in the schema.
    let ok = profile("  - match: { exact: { audio_sampling_frequency: 48000 } }");
    assert!(!codes(&ok).contains(&DiagCode::ValueTypeMismatch));
    let bad = profile("  - match: { exact: { audio_channels: 5.1 } }");
    assert!(codes(&bad).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn invalid_condition_regex_is_flagged() {
    let p = profile("  - match: { regex: { track_name: '([' } }");
    assert!(codes(&p).contains(&DiagCode::InvalidRegex));
}

#[test]
fn nested_any_and_not_are_validated_recursively() {
    let p = profile("  - match:\n      any:\n        - exact: { nonexistent_prop: 1 }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::UnknownProperty)
        .unwrap();
    assert_eq!(
        d.config_path,
        "tracks[0].match.any[0].exact.nonexistent_prop"
    );
}

#[test]
fn empty_match_expression_is_warning() {
    let p = profile("  - match: {}");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::EmptyMatchExpression)
        .unwrap();
    assert_eq!(d.severity, Severity::Warning);
}

#[test]
fn unknown_change_property_is_flagged() {
    let p = profile("  - match: { exact: { type: video } }\n    changes: { bitrate: 5000 }");
    assert!(codes(&p).contains(&DiagCode::UnknownSettableProperty));
}

#[test]
fn change_value_type_mismatch_is_flagged() {
    let p = profile("  - match: { exact: { type: video } }\n    changes: { default_track: 'yes' }");
    assert!(codes(&p).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn attachment_rule_must_have_exactly_one_action() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - select: { substring: { content_type: font } }
      drop: { substring: { file_name: cover } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::AttachmentRuleShape));
}

#[test]
fn attachment_match_uses_attachment_property_set() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
attachments:
  rules:
    - select: { exact: { language: en } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    // "language" is a track property, not an attachment property.
    assert!(codes(&p).contains(&DiagCode::UnknownProperty));
}
