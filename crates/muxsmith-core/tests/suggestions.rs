use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{RunInputs, StructuredEdit, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::report::DiagCode;

struct FakeIdent {
    by_name: HashMap<String, Identification>,
}
impl Identify for FakeIdent {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        let name = path.file_name().unwrap().to_str().unwrap();
        self.by_name
            .get(name)
            .cloned()
            .ok_or_else(|| IdentifyError::Json("no fixture".into()))
    }
}

fn lang() -> LanguageIndex {
    LanguageIndex::from_rows(&[["English", "eng", "eng", "en"]])
}

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");

const P_AMBIGUOUS: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles, codec_kind: srt, language: en } }
"#;

fn plan(profile_yaml: &str) -> muxsmith_core::planner::Batch {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
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
    batch
}

#[test]
fn ambiguous_rule_gets_a_validated_suggestion() {
    let batch = plan(P_AMBIGUOUS);
    assert!(
        batch.files[0]
            .diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule)
    );
    assert!(!batch.suggestions.is_empty(), "expected suggestions");
    assert!(
        batch
            .suggestions
            .iter()
            .all(|s| s.resolves == DiagCode::AmbiguousRule)
    );
    assert!(
        batch
            .suggestions
            .iter()
            .all(|s| s.config_path.starts_with("tracks[0]"))
    );
    assert!(batch.suggestions.len() <= 3);
}

#[test]
fn every_suggestion_survives_the_next_dry_run() {
    let batch = plan(P_AMBIGUOUS);
    for s in &batch.suggestions {
        let edited = apply_edit_to_first_rule(&s.edit);
        let re = plan(&edited);
        assert!(
            !re.files[0]
                .diagnostics
                .iter()
                .any(|d| d.code == DiagCode::AmbiguousRule),
            "suggestion {:?} did not resolve the ambiguity",
            s.edit
        );
        assert!(
            !re.files[0]
                .diagnostics
                .iter()
                .any(|d| d.code == DiagCode::MissingTrack),
            "suggestion {:?} over-narrowed into MissingTrack",
            s.edit
        );
    }
}

// Rebuild an edited profile YAML by inserting the structured edit into the
// single rule's match. Mirrors what the GUI/CLI apply would do.
fn apply_edit_to_first_rule(edit: &StructuredEdit) -> String {
    let inner = match edit {
        StructuredEdit::AddExact { property, value } => format!(
            "exact: {{ type: subtitles, codec_kind: srt, language: en, {property}: {value} }}"
        ),
        StructuredEdit::AddNotExact { property, value } => format!(
            "exact: {{ type: subtitles, codec_kind: srt, language: en }}\n      not:\n        - exact: {{ {property}: {value} }}"
        ),
        StructuredEdit::AddSubstring { value } => format!(
            "exact: {{ type: subtitles, codec_kind: srt, language: en }}\n      substring: {{ track_name: {value} }}"
        ),
        StructuredEdit::AddNotSubstring { value } => format!(
            "exact: {{ type: subtitles, codec_kind: srt, language: en }}\n      not:\n        - substring: {{ track_name: {value} }}"
        ),
    };
    format!(
        "profile_version: 1\ninput: {{ pattern: 'S(?<s>\\d{{2}})E(?<e>\\d{{2}})', extensions: [mkv] }}\ntracks:\n  - match:\n      {inner}\n"
    )
}
