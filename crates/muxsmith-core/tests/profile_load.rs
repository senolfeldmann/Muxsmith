use muxsmith_core::profile::model::{ChaptersCfg, FilenameCfg, KeepDrop, SourceCfg, TitleCfg};
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::report::DiagCode;

const REFERENCE: &str = include_str!("fixtures/reference.yaml");

#[test]
fn reference_profile_parses() {
    let p = from_str(REFERENCE, Format::Yaml).unwrap();
    assert_eq!(p.profile_version, 1);
    assert_eq!(p.input.pattern, r"S(?<season>\d{2})E(?<episode>\d{2})");
    assert!(p.input.recursive);
    assert_eq!(p.tracks.len(), 10); // 1 video + 2 audio + 6 subs + 1 external
    assert!(matches!(p.output.filename, FilenameCfg::Keyword(ref k) if k == "keep"));
    assert!(matches!(p.chapters, ChaptersCfg::Keyword(ref k) if k == "keep"));
    assert!(matches!(p.title, TitleCfg::Keyword(ref k) if k == "clear"));
    assert_eq!(p.tags.global, KeepDrop::Drop);
    assert_eq!(p.tags.track, KeepDrop::Keep);
    assert_eq!(p.attachments.unmatched, KeepDrop::Keep);

    let last = p.tracks.last().unwrap();
    match &last.source {
        SourceCfg::External { external } => {
            assert_eq!(external.path, ".");
            assert_eq!(external.extensions, vec!["srt"]);
            assert_eq!(external.match_to_source, Some(true));
            assert!(external.match_pattern.is_none());
        }
        other => panic!("expected external source, got {other:?}"),
    }
    let changes = last.changes.as_ref().unwrap();
    assert!(changes.contains_key("language") && changes.contains_key("track_name"));
}

#[test]
fn json_profile_parses_identically_to_yaml() {
    let yaml = r#"
profile_version: 1
input: { pattern: 'S(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
"#;
    let json = r#"{
  "profile_version": 1,
  "input": { "pattern": "S(\\d+)", "extensions": ["mkv"] },
  "tracks": [ { "match": { "exact": { "type": "video" } } } ]
}"#;
    let a = from_str(yaml, Format::Yaml).unwrap();
    let b = from_str(json, Format::Json).unwrap();
    assert_eq!(a, b);
}

#[test]
fn defaults_apply_when_sections_absent() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(matches!(p.output.filename, FilenameCfg::Keyword(ref k) if k == "keep"));
    assert_eq!(p.tags.global, KeepDrop::Keep);
    assert!(matches!(p.tracks[0].source, SourceCfg::Keyword(ref k) if k == "primary"));
    assert!(!p.tracks[0].optional);
}

#[test]
fn unknown_key_is_parse_error_with_path() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
    optionall: true
"#;
    let err = from_str(y, Format::Yaml).unwrap_err();
    assert_eq!(err.code, DiagCode::ParseError);
    assert!(err.params["detail"].contains("optionall"));
}
