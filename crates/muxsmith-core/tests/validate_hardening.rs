use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::DiagCode;

fn codes(yaml: &str) -> Vec<DiagCode> {
    let p = from_str(yaml, Format::Yaml).expect("parses");
    validate(&p).into_iter().map(|d| d.code).collect()
}

const HEAD: &str =
    "profile_version: 1\ninput: { pattern: 'E(\\d+)', extensions: [mkv] }\ntracks:\n  rules:\n";

#[test]
fn codec_kind_under_substring_is_exact_only() {
    let y = format!("{HEAD}    - match: {{ substring: {{ codec_kind: srt }} }}\n");
    assert!(codes(&y).contains(&DiagCode::CodecKindExactOnly));
}

#[test]
fn codec_kind_under_regex_is_exact_only() {
    let y = format!("{HEAD}    - match: {{ regex: {{ codec_kind: 'sr.' }} }}\n");
    assert!(codes(&y).contains(&DiagCode::CodecKindExactOnly));
}

#[test]
fn codec_kind_under_exact_is_allowed() {
    let y = format!("{HEAD}    - match: {{ exact: {{ codec_kind: srt }} }}\n");
    let c = codes(&y);
    assert!(!c.contains(&DiagCode::CodecKindExactOnly));
    assert!(!c.contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn bad_type_value_is_invalid_property_value() {
    let y = format!("{HEAD}    - match: {{ exact: {{ type: vdieo }} }}\n");
    assert!(codes(&y).contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn bad_codec_kind_value_is_invalid_property_value() {
    let y = format!("{HEAD}    - match: {{ exact: {{ codec_kind: nope }} }}\n");
    assert!(codes(&y).contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn good_type_value_passes() {
    let y = format!("{HEAD}    - match: {{ exact: {{ type: subtitles }} }}\n");
    assert!(!codes(&y).contains(&DiagCode::InvalidPropertyValue));
}

#[test]
fn empty_any_list_is_empty_match_list() {
    let y = format!("{HEAD}    - match: {{ any: [] }}\n");
    assert!(codes(&y).contains(&DiagCode::EmptyMatchList));
}

#[test]
fn empty_not_list_is_empty_match_list() {
    let y = format!("{HEAD}    - match: {{ not: [] }}\n");
    assert!(codes(&y).contains(&DiagCode::EmptyMatchList));
}

#[test]
fn populated_any_and_not_are_not_empty_match_list() {
    let y = format!(
        "{HEAD}    - match: {{ any: [{{ exact: {{ type: subtitles }} }}], not: [{{ exact: {{ type: audio }} }}] }}\n"
    );
    assert!(!codes(&y).contains(&DiagCode::EmptyMatchList));
}
