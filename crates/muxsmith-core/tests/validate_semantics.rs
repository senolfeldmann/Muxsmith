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
fn zero_rules_with_unmatched_keep_is_a_passthrough_info() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  unmatched: keep
  rules: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    let diags = validate(&p);
    assert!(
        !diags.iter().any(|d| d.code == DiagCode::NoTrackRules),
        "keep + zero rules is a legal passthrough (D38), not NoTrackRules"
    );
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::PassthroughProfile)
        .expect("passthrough must be announced");
    assert_eq!(d.severity, Severity::Info);
    assert_eq!(d.config_path, "tracks.rules");
}

#[test]
fn zero_rules_with_unmatched_drop_stays_an_error() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  unmatched: drop
  rules: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::NoTrackRules));
    assert!(!codes(&p).contains(&DiagCode::PassthroughProfile));
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

// D32 / Task 16: `raw:` opt-in acceptance cases B-1..B-4 (config-time).

// B-1 (regression pin): a bare (unprefixed) typo still hard-rejects exactly as
// before; the raw: feature must not weaken plain-namespace typo protection.
#[test]
fn b1_bare_typo_still_hard_rejects_unchanged() {
    let p = profile("  - match: { exact: { langauge: de } }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::UnknownProperty)
        .expect("bare typo must still be UnknownProperty");
    assert_eq!(d.severity, Severity::Error);
    assert_eq!(d.config_path, "tracks[0].match.exact.langauge");
    assert_eq!(d.params["property"], "langauge");
    assert!(!codes(&p).contains(&DiagCode::RawProperty));
}

// B-2: a raw:-prefixed unknown property under `exact` emits RawProperty (info),
// no UnknownProperty, and no ValueTypeMismatch (the integer value is accepted
// untyped). config_path keeps the literal `raw:`-prefixed key; the `property`
// param carries the bare (stripped) name.
#[test]
fn b2_raw_unknown_exact_is_raw_property_info_untyped() {
    let p = profile("  - match: { exact: { raw:dolby_complexity_index: 3 } }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::RawProperty)
        .expect("raw: unknown property must emit RawProperty");
    assert_eq!(d.severity, Severity::Info);
    assert_eq!(
        d.config_path,
        "tracks[0].match.exact.raw:dolby_complexity_index"
    );
    assert_eq!(d.params["property"], "dolby_complexity_index");
    let cs = codes(&p);
    assert!(!cs.contains(&DiagCode::UnknownProperty));
    assert!(!cs.contains(&DiagCode::ValueTypeMismatch));
}

// B-3: raw: under `substring` on an unknown property emits RawProperty (info),
// no UnknownProperty and no NotStringProperty (untyped, assumed string-capable
// for substring).
#[test]
fn b3_raw_unknown_substring_is_raw_property_info_no_type_error() {
    let p = profile("  - match: { substring: { raw:new_text_field: foo } }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::RawProperty)
        .expect("raw: under substring must emit RawProperty");
    assert_eq!(d.severity, Severity::Info);
    assert_eq!(d.params["property"], "new_text_field");
    let cs = codes(&p);
    assert!(!cs.contains(&DiagCode::UnknownProperty));
    assert!(!cs.contains(&DiagCode::NotStringProperty));
}

// B-4: raw: on a KNOWN property with special matching semantics (language /
// codec_kind) emits RawOnKnownProperty (warning) instead of RawProperty; the
// prefix bypasses ISO-639/BCP-47 normalization (language) or alias expansion
// (codec_kind), degrading the match to byte-literal equality.
#[test]
fn b4_raw_on_language_is_raw_on_known_property_warning() {
    let p = profile("  - match: { exact: { raw:language: de } }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::RawOnKnownProperty)
        .expect("raw:language must emit RawOnKnownProperty");
    assert_eq!(d.severity, Severity::Warning);
    assert_eq!(d.params["property"], "language");
    let cs = codes(&p);
    assert!(!cs.contains(&DiagCode::RawProperty));
    assert!(!cs.contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn b4_raw_on_codec_kind_is_raw_on_known_property_warning() {
    let p = profile("  - match: { exact: { raw:codec_kind: srt } }");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::RawOnKnownProperty)
        .expect("raw:codec_kind must emit RawOnKnownProperty");
    assert_eq!(d.severity, Severity::Warning);
    assert_eq!(d.params["property"], "codec_kind");
    // The bare codec_kind exact-only guard must not fire on the raw: form; raw:
    // bypasses the alias machinery entirely.
    assert!(!codes(&p).contains(&DiagCode::CodecKindExactOnly));
}

// D101: a `raw:` key whose bare property name is EMPTY is a config-time
// error (`EmptyRawProperty`), emitted from the one funnel both validate arms
// share, so the `exact` arm and the `substring`/`regex` arm are covered by
// construction. Always a typo, never expressible intent: `get("")` answers
// `None` in every `Matchable`, so the rule could never match. The
// discriminating controls are B-2/B-3 directly above - a NON-empty `raw:`
// key still yields `RawProperty` info - which is why no duplicate control is
// written here (reuse before writing).

#[test]
fn empty_bare_raw_exact_is_empty_raw_property_error() {
    let p = profile("  - match: { exact: { 'raw:': eng } }");
    let diags = validate(&p);
    let empties: Vec<_> = diags
        .iter()
        .filter(|d| d.code == DiagCode::EmptyRawProperty)
        .collect();
    assert_eq!(empties.len(), 1, "expected exactly one EmptyRawProperty");
    assert_eq!(empties[0].severity, Severity::Error);
    assert_eq!(empties[0].config_path, "tracks[0].match.exact.raw:");
    let cs = codes(&p);
    assert!(!cs.contains(&DiagCode::RawProperty));
    assert!(!cs.contains(&DiagCode::UnknownProperty));
}

#[test]
fn empty_bare_raw_substring_is_empty_raw_property_error() {
    let p = profile("  - match: { substring: { 'raw:': en } }");
    let diags = validate(&p);
    let empties: Vec<_> = diags
        .iter()
        .filter(|d| d.code == DiagCode::EmptyRawProperty)
        .collect();
    assert_eq!(empties.len(), 1, "expected exactly one EmptyRawProperty");
    assert_eq!(empties[0].severity, Severity::Error);
    assert_eq!(empties[0].config_path, "tracks[0].match.substring.raw:");
    let cs = codes(&p);
    assert!(!cs.contains(&DiagCode::RawProperty));
    assert!(!cs.contains(&DiagCode::UnknownProperty));
}

// D46: the `Keyword(String)` arm keeps its `String` so a misspelled keyword
// stays reachable as InvalidKeyword (with `found`/`allowed`) instead of
// falling through to serde's untagged-enum error.
#[test]
fn misspelled_chapters_keyword_is_invalid_keyword_with_const_derived_allowed() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
chapters: kepp
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::InvalidKeyword)
        .expect("misspelled chapters keyword must be InvalidKeyword");
    assert_eq!(d.config_path, "chapters");
    assert_eq!(d.params["found"], "kepp");
    assert_eq!(d.params["allowed"], "keep, drop");
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
