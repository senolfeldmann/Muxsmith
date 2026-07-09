use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::DiagCode;

fn parse(y: &str) -> muxsmith_core::profile::Profile {
    from_str(y, Format::Yaml).unwrap()
}

fn codes(y: &str) -> Vec<DiagCode> {
    validate(&parse(y)).into_iter().map(|d| d.code).collect()
}

const BASE: &str = r#"
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;

#[test]
fn invalid_input_pattern_is_flagged() {
    let y = BASE.replace(r"S(?<season>\d{2})E(?<episode>\d{2})", "([");
    assert!(codes(&y).contains(&DiagCode::InvalidRegex));
}

#[test]
fn empty_extensions_flagged_for_input_and_locator() {
    let y = BASE.replace("extensions: [mkv]", "extensions: []");
    assert!(codes(&y).contains(&DiagCode::EmptyExtensions));
}

#[test]
fn locator_with_both_match_options_is_conflict() {
    let y = format!(
        "{BASE}    - source:\n        external: {{ path: '.', extensions: [srt], match_to_source: true, match_pattern: '{{match}}' }}\n      match: {{ exact: {{ type: subtitles }} }}\n"
    );
    assert!(codes(&y).contains(&DiagCode::LocatorConflict));
}

#[test]
fn match_to_source_false_is_rejected() {
    let y = format!(
        "{BASE}    - source:\n        external: {{ path: '.', extensions: [srt], match_to_source: false }}\n      match: {{ exact: {{ type: subtitles }} }}\n"
    );
    let c = codes(&y);
    assert!(c.contains(&DiagCode::InvalidKeyword));
    assert!(!c.contains(&DiagCode::LocatorConflict));
}

#[test]
fn match_to_source_false_with_pattern_is_not_conflict() {
    let y = format!(
        "{BASE}    - source:\n        external: {{ path: '.', extensions: [srt], match_to_source: false, match_pattern: '{{match}}' }}\n      match: {{ exact: {{ type: subtitles }} }}\n"
    );
    let c = codes(&y);
    assert!(c.contains(&DiagCode::InvalidKeyword));
    assert!(!c.contains(&DiagCode::LocatorConflict));
}

#[test]
fn match_pattern_with_unknown_field_is_flagged() {
    let y = format!(
        "{BASE}    - source:\n        external: {{ path: '.', extensions: [srt], match_pattern: 'x{{volume}}y' }}\n      match: {{ exact: {{ type: subtitles }} }}\n"
    );
    let c = codes(&y);
    assert!(c.contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn match_pattern_may_not_use_source_stem() {
    // source_stem is literal-mode only (spec 4.7).
    let y = format!(
        "{BASE}    - source:\n        external: {{ path: '.', extensions: [srt], match_pattern: '{{source_stem}}' }}\n      match: {{ exact: {{ type: subtitles }} }}\n"
    );
    assert!(codes(&y).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn filename_template_fields_checked_against_pattern_groups() {
    let good =
        BASE.to_string() + "output:\n  filename: { template: 'X S{season}E{episode:pad2}.mkv' }\n";
    assert!(!codes(&good).contains(&DiagCode::UnknownTemplateField));

    let bad = BASE.to_string() + "output:\n  filename: { template: 'X {show}.mkv' }\n";
    assert!(codes(&bad).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn filename_template_with_path_separator_is_flagged() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'sub/dir{season}.mkv' }\n";
    assert!(codes(&y).contains(&DiagCode::PathSeparatorInTemplate));
}

#[test]
fn bad_template_syntax_is_invalid_template() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'S{season' }\n";
    let ds = validate(&parse(&y));
    let d = ds
        .iter()
        .find(|d| d.code == DiagCode::InvalidTemplate)
        .expect("expected InvalidTemplate diagnostic");
    assert_eq!(
        d.params.get("kind").map(String::as_str),
        Some("unclosed-brace")
    );
    assert!(d.params.contains_key("pos"));
}

#[test]
fn unknown_template_filter_carries_name() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'S{season:frobnicate}.mkv' }\n";
    let ds = validate(&parse(&y));
    let d = ds
        .iter()
        .find(|d| d.code == DiagCode::UnknownTemplateFilter)
        .expect("expected UnknownTemplateFilter diagnostic");
    assert_eq!(d.params.get("name").map(String::as_str), Some("frobnicate"));
}

#[test]
fn unknown_keywords_are_flagged() {
    for (snippet, _section) in [
        ("chapters: discard\n", "chapters"),
        ("title: wipe\n", "title"),
    ] {
        let y = BASE.to_string() + snippet;
        assert!(
            codes(&y).contains(&DiagCode::InvalidKeyword),
            "expected InvalidKeyword for: {snippet}"
        );
    }
    let y = BASE.replace(
        "- match: { exact: { type: video } }",
        "- source: secondary\n      match: { exact: { type: video } }",
    );
    assert!(codes(&y).contains(&DiagCode::InvalidKeyword));
}

#[test]
fn numbered_group_fields_are_accepted() {
    let y = r#"
profile_version: 1
input: { pattern: 'S(\d{2})E(\d{2})', extensions: [mkv] }
output:
  filename: { template: 'S{g1}E{g2}.mkv' }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    assert!(!codes(y).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn empty_locator_extensions_flagged() {
    let y = format!(
        "{BASE}    - source:\n        external: {{ path: '.', extensions: [] }}\n      match: {{ exact: {{ type: subtitles }} }}\n"
    );
    let ds = validate(&parse(&y));
    let d = ds
        .iter()
        .find(|d| d.code == DiagCode::EmptyExtensions)
        .expect("expected EmptyExtensions diagnostic");
    assert!(d.config_path.ends_with(".source.external.extensions"));
}

#[test]
fn filename_keyword_misuse_flagged() {
    let y = BASE.to_string() + "output:\n  filename: wipe\n";
    assert!(codes(&y).contains(&DiagCode::InvalidKeyword));
}
