use std::collections::HashMap;

use serde::Deserialize;

use muxsmith_core::identify::Identification;
use muxsmith_core::planner::{Batch, RunInputs, StructuredEdit, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::profile::match_expr::{MatchExpr, Scalar};
use muxsmith_core::report::DiagCode;

mod support;
use support::{FakeIdent, lang};

const SERIES: &str = include_str!("fixtures/identify/series-s01e01.json");

const P_AMBIGUOUS: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles, codec_kind: srt, language: en } }
"#;

fn plan(profile_yaml: &str) -> (muxsmith_core::planner::Batch, tempfile::TempDir) {
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
    (batch, dir)
}

#[test]
fn ambiguous_rule_gets_a_validated_suggestion() {
    let (batch, _dir) = plan(P_AMBIGUOUS);
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
    let (batch, _dir) = plan(P_AMBIGUOUS);
    for s in &batch.suggestions {
        let edited = apply_edit_to_first_rule(&s.edit);
        let (re, _dir) = plan(&edited);
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
            "exact: {{ type: subtitles, codec_kind: srt, language: en }}\n        not:\n          - exact: {{ {property}: {value} }}"
        ),
        StructuredEdit::AddSubstring { value } => format!(
            "exact: {{ type: subtitles, codec_kind: srt, language: en }}\n        substring: {{ track_name: {value} }}"
        ),
        StructuredEdit::AddNotSubstring { value } => format!(
            "exact: {{ type: subtitles, codec_kind: srt, language: en }}\n        not:\n          - substring: {{ track_name: {value} }}"
        ),
    };
    format!(
        "profile_version: 1\ninput: {{ pattern: 'S(?<s>\\d{{2}})E(?<e>\\d{{2}})', extensions: [mkv] }}\ntracks:\n  rules:\n    - match:\n        {inner}\n"
    )
}

// Plans an arbitrary set of named fixture files (name -> -J JSON), unlike
// `plan()` which is wired to the single-file SERIES fixture.
fn plan_multi(profile_yaml: &str, files: &[(&str, &str)]) -> (Batch, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let mut by_name = HashMap::new();
    for (name, json) in files {
        std::fs::write(dir.path().join(name), b"x").unwrap();
        by_name.insert(
            (*name).to_string(),
            Identification::from_json(json).unwrap(),
        );
    }
    let profile = from_str(profile_yaml, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    (batch, dir)
}

// --- (a) with_rule_match must not clobber an existing constraint (bug C) ---

const P_NO_CLOBBER: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles }, substring: { track_name: Foo } }
"#;

// File 1: two subtitle tracks both containing "Foo" in track_name, both
// otherwise identical -- ambiguous under the rule above, and the only source
// of discriminator candidates (candidate generation only looks at files
// whose current matched set has >= 2 tracks).
const AMBIGUOUS_FOO: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E01.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } },
    { "codec": "SubRip/SRT", "id": 1, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en", "track_name": "Foo Director Commentary" } },
    { "codec": "SubRip/SRT", "id": 2, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en", "track_name": "Foo Forced" } }
  ]
}
"#;

// File 2: already unambiguous under the ORIGINAL rule (only the "Foo" track
// matches). Its second track is a decoy that shares no "Foo" but does share
// "Director" with file 1's track -- exactly the token candidate generation
// draws from file 1's ambiguity. A clobbering with_rule_match silently drops
// the "Foo" constraint, so the edited rule would resolve file 2 to the decoy
// instead of the original track, without any new diagnostic anywhere.
const GUARDED_FOO: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E02.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } },
    { "codec": "SubRip/SRT", "id": 1, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en", "track_name": "Foo Only" } },
    { "codec": "SubRip/SRT", "id": 2, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en", "track_name": "Director Extended" } }
  ]
}
"#;

fn no_clobber_batch() -> (Batch, tempfile::TempDir) {
    plan_multi(
        P_NO_CLOBBER,
        &[
            ("Show.S01E01.mkv", AMBIGUOUS_FOO),
            ("Show.S01E02.mkv", GUARDED_FOO),
        ],
    )
}

// Rebuild the edited profile YAML the same way a literal/naive splice of the
// structured edit would (what a buggy overwrite-based with_rule_match, or a
// GUI/CLI apply mirroring it, would produce): AddExact/AddNotExact/
// AddNotSubstring add alongside the existing `exact`/`substring` map or into
// an independent `not` list; AddSubstring replaces the substring map's
// `track_name` entry wholesale, since that is what overwriting a BTreeMap key
// via `extend` actually does.
fn apply_edit_to_no_clobber_rule(edit: &StructuredEdit) -> String {
    let inner = match edit {
        StructuredEdit::AddExact { property, value } => format!(
            "exact: {{ type: subtitles, {property}: {value} }}\n        substring: {{ track_name: Foo }}"
        ),
        StructuredEdit::AddNotExact { property, value } => format!(
            "exact: {{ type: subtitles }}\n        substring: {{ track_name: Foo }}\n        not:\n          - exact: {{ {property}: {value} }}"
        ),
        StructuredEdit::AddSubstring { value } => {
            format!("exact: {{ type: subtitles }}\n        substring: {{ track_name: {value} }}")
        }
        StructuredEdit::AddNotSubstring { value } => format!(
            "exact: {{ type: subtitles }}\n        substring: {{ track_name: Foo }}\n        not:\n          - substring: {{ track_name: {value} }}"
        ),
    };
    format!(
        "profile_version: 1\ninput: {{ pattern: 'S(?<s>\\d{{2}})E(?<e>\\d{{2}})', extensions: [mkv] }}\ntracks:\n  rules:\n    - match:\n        {inner}\n"
    )
}

#[test]
fn with_rule_match_never_widens_an_existing_substring_constraint() {
    let (batch, _dir) = no_clobber_batch();
    assert!(
        batch.files[0]
            .diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule),
        "expected file 1 to be ambiguous under the baseline profile"
    );
    assert!(!batch.suggestions.is_empty(), "expected suggestions");

    // The specific clobber vector (bug C): overwriting the existing
    // `substring: { track_name: Foo }` with a new value is a no-op under
    // insert-only semantics, so it can never resolve the ambiguity and must
    // never be accepted/emitted.
    assert!(
        !batch.suggestions.iter().any(|s| matches!(
            &s.edit,
            StructuredEdit::AddSubstring { value } if value == "Director"
        )),
        "a candidate overwriting the existing track_name substring must \
         never be accepted: found {:?}",
        batch
            .suggestions
            .iter()
            .map(|s| &s.edit)
            .collect::<Vec<_>>()
    );

    // Every suggestion that IS emitted must, once applied, still resolve
    // file 2 to its original track (id 1, "Foo Only"), never the "Director"
    // decoy (id 2) -- the behavioral form of "never widens."
    for s in &batch.suggestions {
        let edited = apply_edit_to_no_clobber_rule(&s.edit);
        let (re, _dir) = plan_multi(
            &edited,
            &[
                ("Show.S01E01.mkv", AMBIGUOUS_FOO),
                ("Show.S01E02.mkv", GUARDED_FOO),
            ],
        );
        assert!(
            !re.files[0]
                .diagnostics
                .iter()
                .any(|d| d.code == DiagCode::AmbiguousRule),
            "suggestion {:?} did not resolve the ambiguity",
            s.edit
        );
        let assignment = &re.files[1].plan.as_ref().unwrap().assignments[0];
        assert_eq!(
            assignment.track_id,
            Some(1),
            "suggestion {:?} redirected file 2's resolution to the decoy track",
            s.edit
        );
    }
}

// --- (b) yaml_fragment must emit valid, round-trippable YAML (bug D) ---

const P_COLON_AMBIGUOUS: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles } }
"#;

// Two subtitle tracks, ambiguous under a bare `type: subtitles` rule; one
// track_name contains a colon, which breaks a hand-formatted YAML fragment.
const COLON_TRACK_NAMES: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E01.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } },
    { "codec": "SubRip/SRT", "id": 1, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "track_name": "Chapter 1: Intro" } },
    { "codec": "SubRip/SRT", "id": 2, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "track_name": "Extras" } }
  ]
}
"#;

#[derive(Deserialize)]
struct MatchFragmentDoc {
    #[serde(rename = "match")]
    match_expr: MatchExpr,
}

// --- (b) yaml_fragment must emit valid, round-trippable YAML (bug D) ---

#[test]
fn yaml_fragment_round_trips_a_value_containing_a_colon() {
    let (batch, _dir) = plan_multi(P_COLON_AMBIGUOUS, &[("Show.S01E01.mkv", COLON_TRACK_NAMES)]);

    let target = batch
        .suggestions
        .iter()
        .find(|s| {
            matches!(
                &s.edit,
                StructuredEdit::AddExact { property, value }
                    if property == "track_name" && value == "Chapter 1: Intro"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "expected an AddExact suggestion on the colon-bearing track_name, got {:?}",
                batch
                    .suggestions
                    .iter()
                    .map(|s| &s.edit)
                    .collect::<Vec<_>>()
            )
        });

    let parsed: MatchFragmentDoc = yaml_serde::from_str(&target.yaml_fragment)
        .expect("yaml_fragment must be valid, parseable YAML");
    assert_eq!(
        parsed.match_expr.exact.unwrap().get("track_name"),
        Some(&Scalar::Str("Chapter 1: Intro".to_string())),
        "round-tripped fragment did not carry the intended edit"
    );
}

// --- (c) the cap-3 truncation must be logged, not silent (D6) ---

#[test]
fn suggestion_cap_truncation_is_logged_not_silent() {
    // P_AMBIGUOUS's two conflicting subtitle tracks (English forced /
    // English SDH) differ across enough matchable properties and track_name
    // tokens that well over 3 candidates are accepted for tracks[0]; the cap
    // truncates the emitted list to 3 and must record the rest.
    let (batch, _dir) = plan(P_AMBIGUOUS);

    let cap_diag = batch
        .batch_diagnostics
        .iter()
        .find(|d| d.code == DiagCode::SuggestionsCapped)
        .expect("expected a logged suggestion cap for tracks[0]'s conflict");
    assert_eq!(cap_diag.config_path, "tracks[0].match");
    let dropped: usize = cap_diag
        .params
        .get("dropped")
        .expect("cap diagnostic must carry a dropped count")
        .parse()
        .expect("dropped count must be a number");
    assert!(
        dropped > 0,
        "dropped count must be non-zero, was silently 0"
    );
}

// --- Task 13 step 1: external-source rules get suggestions (#12ii) ---

// Primary is a bare video file: it never matches the subtitle rule itself, so
// the rule's ambiguity lives entirely in the located donor. Candidate
// generation must therefore draw discriminators from the donor's tracks, not
// the primary's -- the source-agnostic requirement.
const PRIMARY_VIDEO_ONLY: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E01.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } }
  ]
}
"#;

// Donor with two subtitle tracks, ambiguous under `exact: { type: subtitles }`;
// they split on forced_track, so a suggestion exists.
const DONOR_TWO_SUBS: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "SRT" },
  "errors": [], "file_name": "Donor.S01E01.srt", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "SubRip/SRT", "id": 0, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": true,
        "language": "eng", "language_ietf": "en", "track_name": "Forced" } },
    { "codec": "SubRip/SRT", "id": 1, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en", "track_name": "Full" } }
  ]
}
"#;

const P_EXTERNAL_SUBS: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - source: { external: { path: '.', extensions: [srt], match_to_source: true } }
      match: { exact: { type: subtitles } }
"#;

#[test]
fn ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule() {
    let (batch, _dir) = plan_multi(
        P_EXTERNAL_SUBS,
        &[
            ("Show.S01E01.mkv", PRIMARY_VIDEO_ONLY),
            ("Donor.S01E01.srt", DONOR_TWO_SUBS),
        ],
    );

    assert!(
        batch.files[0]
            .diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule),
        "expected the external rule's donor conflict to be ambiguous: {:?}",
        batch.files[0].diagnostics
    );
    assert!(
        !batch.suggestions.is_empty(),
        "an external-source rule must get suggestions like a primary one"
    );
    assert!(
        batch.suggestions.iter().all(
            |s| s.resolves == DiagCode::AmbiguousRule && s.config_path.starts_with("tracks[0]")
        ),
        "suggestions: {:?}",
        batch
            .suggestions
            .iter()
            .map(|s| &s.edit)
            .collect::<Vec<_>>()
    );
}

// --- Task 13 step 2: codec and id are narrowing dimensions (R1 iv) ---

// Two subtitle tracks identical in every matchable property EXCEPT `codec`
// (and `id`, which always differs per track). `codec_id` is deliberately the
// same on both, so the ambiguity is resolvable ONLY via the top-level `codec`
// or `id` fields -- exactly the dimensions candidate generation used to omit.
const CODEC_ID_ONLY: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E01.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } },
    { "codec": "SubRip/SRT", "id": 1, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en" } },
    { "codec": "SubStationAlpha/ASS", "id": 2, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en" } }
  ]
}
"#;

const P_SUBS_BY_LANGUAGE: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles, language: en } }
"#;

#[test]
fn ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions() {
    let (batch, _dir) = plan_multi(P_SUBS_BY_LANGUAGE, &[("Show.S01E01.mkv", CODEC_ID_ONLY)]);

    assert!(
        batch.files[0]
            .diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule),
        "expected the two same-language subtitle tracks to be ambiguous"
    );
    assert!(
        batch
            .suggestions
            .iter()
            .all(|s| s.resolves == DiagCode::AmbiguousRule),
    );

    let has_codec = batch.suggestions.iter().any(
        |s| matches!(&s.edit, StructuredEdit::AddExact { property, .. } if property == "codec"),
    );
    let has_id = batch
        .suggestions
        .iter()
        .any(|s| matches!(&s.edit, StructuredEdit::AddExact { property, .. } if property == "id"));
    assert!(
        has_codec && has_id,
        "expected both a codec-based and an id-based suggestion, got {:?}",
        batch
            .suggestions
            .iter()
            .map(|s| &s.edit)
            .collect::<Vec<_>>()
    );
}

// --- Task 13 step 4: the no-single-fix partition report (#5) ---

// File A: two subtitle tracks (ids 1, 2) separable only by forced_track.
const PART_A_FORCED: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E01.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } },
    { "codec": "SubRip/SRT", "id": 1, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": true,
        "language": "eng", "language_ietf": "en" } },
    { "codec": "SubRip/SRT", "id": 2, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en" } }
  ]
}
"#;

// File B: two subtitle tracks (ids 3, 4) separable only by language, both
// non-forced. The subtitle ids (3, 4) are disjoint from file A's (1, 2), so no
// single `id`-based narrowing resolves BOTH files -- forcing the no-single-fix
// case even though each file is individually resolvable.
const PART_B_LANG: &str = r#"
{
  "attachments": [], "chapters": [],
  "container": { "recognized": true, "supported": true, "type": "Matroska" },
  "errors": [], "file_name": "Show.S01E02.mkv", "global_tags": [],
  "identification_format_version": 20, "track_tags": [],
  "tracks": [
    { "codec": "AVC/H.264", "id": 0, "type": "video", "properties": { "codec_id": "V_MPEG4/ISO/AVC" } },
    { "codec": "AAC", "id": 1, "type": "audio", "properties": { "codec_id": "A_AAC", "language": "eng" } },
    { "codec": "AAC", "id": 2, "type": "audio", "properties": { "codec_id": "A_AAC", "language": "ger" } },
    { "codec": "SubRip/SRT", "id": 3, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "eng", "language_ietf": "en" } },
    { "codec": "SubRip/SRT", "id": 4, "type": "subtitles", "properties": {
        "codec_id": "S_TEXT/UTF8", "default_track": false, "forced_track": false,
        "language": "ger", "language_ietf": "de" } }
  ]
}
"#;

const P_SUBS_ANY: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles } }
"#;

fn partition_diags(batch: &Batch) -> Vec<&muxsmith_core::report::Diagnostic> {
    batch
        .batch_diagnostics
        .iter()
        .filter(|d| d.code == DiagCode::SuggestionPartition)
        .collect()
}

#[test]
fn no_single_fix_produces_a_two_group_partition() {
    let (batch, _dir) = plan_multi(
        P_SUBS_ANY,
        &[
            ("Show.S01E01.mkv", PART_A_FORCED),
            ("Show.S01E02.mkv", PART_B_LANG),
        ],
    );

    // Both files are ambiguous, and no single refinement resolves both.
    assert!(
        batch.files.iter().all(|f| f
            .diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule)),
        "both files must be ambiguous"
    );
    assert!(
        batch.suggestions.is_empty(),
        "no batch-wide suggestion should survive: {:?}",
        batch
            .suggestions
            .iter()
            .map(|s| &s.edit)
            .collect::<Vec<_>>()
    );

    let groups: Vec<_> = partition_diags(&batch)
        .into_iter()
        .filter(|d| d.params.get("kind").map(String::as_str) == Some("group"))
        .collect();
    assert_eq!(
        groups.len(),
        2,
        "expected a two-group partition, got {:?}",
        groups.iter().map(|d| &d.params).collect::<Vec<_>>()
    );
    assert!(groups.iter().all(|d| d.config_path == "tracks[0].match"
        && d.severity == muxsmith_core::report::Severity::Info),);

    // One group is the forced_track file, the other the language file, and the
    // files are partitioned accordingly.
    let forced = groups
        .iter()
        .find(|d| d.params["fix"].contains("forced_track"))
        .expect("a forced_track group");
    let lang = groups
        .iter()
        .find(|d| d.params["fix"].contains("language"))
        .expect("a language group");
    assert!(forced.params["files"].contains("S01E01"));
    assert!(!forced.params["files"].contains("S01E02"));
    assert!(lang.params["files"].contains("S01E02"));
    assert!(!lang.params["files"].contains("S01E01"));
}
