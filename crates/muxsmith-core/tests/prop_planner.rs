//! Property-based tests for the planner (spec 5; spec 10 deferred property
//! suite, #1). Three guarantees:
//!
//! * determinism: identical inputs produce a byte-identical serialized batch;
//! * rendered-name invariants (D4): a produced plan's output name carries no
//!   path separator, always ends in `.mkv`, and never escapes its output
//!   directory;
//! * D6: an accepted suggestion, re-applied to the profile and re-planned,
//!   resolves the ambiguity it targets and introduces no new diagnostic.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use proptest::collection::vec as prop_vec;
use proptest::prelude::*;
use proptest::sample::select;
use serde::Deserialize;

use muxsmith_core::capability::TYPE_VALUES;
use muxsmith_core::identify::{Identification, PropValue, Track};
use muxsmith_core::planner::{Batch, RunInputs, plan_batch};
use muxsmith_core::profile::match_expr::{MatchExpr, Scalar};
use muxsmith_core::profile::model::{
    AttachmentsCfg, ChaptersCfg, CollisionPolicy, FilenameCfg, Input, KeepDrop, OutputCfg, Profile,
    SourceCfg, TagsCfg, TemplateBlock, TitleCfg, TrackRule, TracksCfg,
};
use muxsmith_core::report::DiagCode;

mod support;
use support::{FakeIdent, lang};

const FILE_NAMES: &[&str] = &[
    "Show.S01E01.mkv",
    "Show.S01E02.mkv",
    "Show.S01E03.mkv",
    "Show.S01E04.mkv",
];

// --- builders -------------------------------------------------------------

fn exact_one(prop: &str, val: Scalar) -> MatchExpr {
    let mut map = BTreeMap::new();
    map.insert(prop.to_string(), val);
    MatchExpr {
        exact: Some(map),
        ..Default::default()
    }
}

fn substring_one(prop: &str, val: String) -> MatchExpr {
    let mut map = BTreeMap::new();
    map.insert(prop.to_string(), val);
    MatchExpr {
        substring: Some(map),
        ..Default::default()
    }
}

fn rule_of(expr: MatchExpr, optional: bool) -> TrackRule {
    TrackRule {
        source: SourceCfg::primary(),
        match_expr: expr,
        optional,
        changes: None,
    }
}

fn mk_profile(rules: Vec<TrackRule>, filename: FilenameCfg) -> Profile {
    Profile {
        profile_version: 1,
        meta: None,
        input: Input {
            pattern: r"S(?<s>\d{2})E(?<e>\d{2})".to_string(),
            extensions: vec!["mkv".to_string()],
            recursive: true,
        },
        output: OutputCfg {
            directory: None,
            filename,
            on_collision: CollisionPolicy::Error,
        },
        tracks: TracksCfg {
            unmatched: KeepDrop::Drop,
            rules,
        },
        attachments: AttachmentsCfg::default(),
        chapters: ChaptersCfg::default(),
        tags: TagsCfg::default(),
        title: TitleCfg::default(),
    }
}

fn ident(tracks: Vec<Track>) -> Identification {
    Identification {
        file_name: String::new(),
        format_version: 20,
        container_recognized: true,
        container_supported: true,
        tracks,
        attachments: vec![],
        chapters: 0,
    }
}

fn video_track(id: u64) -> Track {
    let mut properties = BTreeMap::new();
    properties.insert(
        "codec_id".to_string(),
        PropValue::Str("V_MPEG4/ISO/AVC".to_string()),
    );
    Track {
        id,
        kind: "video".to_string(),
        codec: String::new(),
        properties,
    }
}

// Plans `files` (name -> identification) in a throwaway source directory and
// returns the batch plus the output directory the plan paths hang under.
fn run_plan(profile: &Profile, files: &[(String, Identification)]) -> (Batch, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let mut by_name = HashMap::new();
    for (name, id) in files {
        std::fs::write(dir.path().join(name), b"x").unwrap();
        by_name.insert(name.clone(), id.clone());
    }
    let out_dir = dir.path().join("out");
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(out_dir.clone()),
        on_collision: None,
    };
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(profile, &run, &mut ident, &lang());
    (batch, out_dir)
}

// --- strategies -----------------------------------------------------------

// A compact realistic match expression over the planner's vocabulary.
fn arb_plan_expr() -> impl Strategy<Value = MatchExpr> {
    prop_oneof![
        select(TYPE_VALUES.to_vec()).prop_map(|t| exact_one("type", Scalar::Str(t.to_string()))),
        select(vec!["eng", "ger", "tur"])
            .prop_map(|l| exact_one("language", Scalar::Str(l.to_string()))),
        (select(vec!["default_track", "forced_track"]), any::<bool>())
            .prop_map(|(p, b)| exact_one(p, Scalar::Bool(b))),
        select(vec!["Forced", "SDH", "Commentary", "Main"])
            .prop_map(|s| substring_one("track_name", s.to_string())),
    ]
}

fn arb_track(id: u64) -> impl Strategy<Value = Track> {
    (
        select(TYPE_VALUES.to_vec()),
        select(vec!["eng", "ger", "tur"]),
        select(vec![
            "S_TEXT/UTF8",
            "A_AAC",
            "V_MPEG4/ISO/AVC",
            "S_TEXT/ASS",
        ]),
        select(vec![
            "Forced",
            "SDH",
            "Commentary",
            "Main",
            "English",
            "German",
        ]),
        any::<bool>(),
        any::<bool>(),
    )
        .prop_map(move |(kind, language, codec_id, name, def, forced)| {
            let mut properties = BTreeMap::new();
            properties.insert("language".to_string(), PropValue::Str(language.to_string()));
            properties.insert("codec_id".to_string(), PropValue::Str(codec_id.to_string()));
            properties.insert("track_name".to_string(), PropValue::Str(name.to_string()));
            properties.insert("default_track".to_string(), PropValue::Bool(def));
            properties.insert("forced_track".to_string(), PropValue::Bool(forced));
            Track {
                id,
                kind: kind.to_string(),
                codec: String::new(),
                properties,
            }
        })
}

fn arb_identification() -> impl Strategy<Value = Identification> {
    (1usize..5)
        .prop_flat_map(|n| (0..n).map(|i| arb_track(i as u64)).collect::<Vec<_>>())
        .prop_map(ident)
}

fn arb_files() -> impl Strategy<Value = Vec<(String, Identification)>> {
    (1usize..=FILE_NAMES.len()).prop_flat_map(|k| {
        FILE_NAMES[..k]
            .iter()
            .map(|name| arb_identification().prop_map(move |id| (name.to_string(), id)))
            .collect::<Vec<_>>()
    })
}

fn arb_filename() -> impl Strategy<Value = FilenameCfg> {
    prop_oneof![
        Just(FilenameCfg::keep()),
        Just(FilenameCfg::Template(TemplateBlock {
            template: "{match}".to_string()
        })),
        Just(FilenameCfg::Template(TemplateBlock {
            template: "{s}-{e}".to_string()
        })),
    ]
}

fn arb_profile() -> impl Strategy<Value = Profile> {
    (prop_vec(arb_plan_expr(), 1..3), arb_filename()).prop_map(|(exprs, filename)| {
        let rules = exprs.into_iter().map(|e| rule_of(e, false)).collect();
        mk_profile(rules, filename)
    })
}

// One video (id 0) plus 0..3 non-video tracks, so a `type: video` rule
// resolves to exactly one track (a plan is always produced).
fn arb_ident_one_video() -> impl Strategy<Value = Identification> {
    prop_vec(arb_nonvideo_track(), 0..3).prop_map(|extras| {
        let mut tracks = vec![video_track(0)];
        for (i, mut t) in extras.into_iter().enumerate() {
            t.id = (i + 1) as u64;
            tracks.push(t);
        }
        ident(tracks)
    })
}

fn arb_nonvideo_track() -> impl Strategy<Value = Track> {
    (
        select(vec!["audio", "subtitles"]),
        select(vec!["eng", "ger", "tur"]),
        select(vec!["S_TEXT/UTF8", "A_AAC"]),
    )
        .prop_map(|(kind, language, codec_id)| {
            let mut properties = BTreeMap::new();
            properties.insert("language".to_string(), PropValue::Str(language.to_string()));
            properties.insert("codec_id".to_string(), PropValue::Str(codec_id.to_string()));
            Track {
                id: 0,
                kind: kind.to_string(),
                codec: String::new(),
                properties,
            }
        })
}

fn arb_render_files() -> impl Strategy<Value = Vec<(String, Identification)>> {
    (1usize..=FILE_NAMES.len()).prop_flat_map(|k| {
        FILE_NAMES[..k]
            .iter()
            .map(|name| arb_ident_one_video().prop_map(move |id| (name.to_string(), id)))
            .collect::<Vec<_>>()
    })
}

// A single file with one video and two subtitle tracks of DISTINCT languages:
// `exact: { type: subtitles }` is guaranteed ambiguous, and the differing
// language guarantees at least one resolving discriminator exists.
fn arb_ambiguous_ident() -> impl Strategy<Value = Identification> {
    (
        0usize..3,
        1usize..3,
        select(vec!["Forced", "SDH", "Commentary", "Main"]),
        select(vec!["Forced", "SDH", "Commentary", "Main"]),
        any::<bool>(),
        any::<bool>(),
    )
        .prop_map(|(i0, off, n1, n2, f1, f2)| {
            let langs = ["eng", "ger", "tur"];
            let sub = |id: u64, language: &str, name: &str, forced: bool| {
                let mut properties = BTreeMap::new();
                properties.insert("language".to_string(), PropValue::Str(language.to_string()));
                properties.insert(
                    "codec_id".to_string(),
                    PropValue::Str("S_TEXT/UTF8".to_string()),
                );
                properties.insert("track_name".to_string(), PropValue::Str(name.to_string()));
                properties.insert("forced_track".to_string(), PropValue::Bool(forced));
                Track {
                    id,
                    kind: "subtitles".to_string(),
                    codec: String::new(),
                    properties,
                }
            };
            ident(vec![
                video_track(0),
                sub(1, langs[i0], n1, f1),
                sub(2, langs[(i0 + off) % 3], n2, f2),
            ])
        })
}

// --- suggestion application (mirrors planner::with_rule_match, insert-only) ---

#[derive(Deserialize)]
struct MatchFragmentDoc {
    #[serde(rename = "match")]
    match_expr: MatchExpr,
}

// Applies a suggestion's emitted YAML fragment to rule `ri`, exactly as a GUI
// or CLI "apply" would: parse the fragment's `match:` delta and splice it in
// with insert-only semantics (a suggestion may only narrow, never relax).
fn apply_suggestion(profile: &Profile, ri: usize, fragment: &str) -> Profile {
    let doc: MatchFragmentDoc =
        yaml_serde::from_str(fragment).expect("emitted yaml_fragment must parse");
    let delta = doc.match_expr;
    let mut p = profile.clone();
    let expr = &mut p.tracks.rules[ri].match_expr;
    if let Some(add) = &delta.exact {
        let map = expr.exact.get_or_insert_with(BTreeMap::new);
        for (k, v) in add {
            map.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
    if let Some(add) = &delta.substring {
        let map = expr.substring.get_or_insert_with(BTreeMap::new);
        for (k, v) in add {
            map.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
    if let Some(add) = &delta.not {
        expr.not.get_or_insert_with(Vec::new).extend(add.clone());
    }
    p
}

// Basename-scoped diagnostic signature (code|config_path|file basename), so
// two plans of the same logical file in different throwaway directories are
// comparable. Mirrors planner::diag_signature otherwise.
fn diag_sig(batch: &Batch) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    let all = batch
        .batch_diagnostics
        .iter()
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()));
    for d in all {
        let file = d
            .file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        set.insert(format!("{}|{}|{}", d.code.key(), d.config_path, file));
    }
    set
}

fn rule_index_of(path: &str) -> Option<usize> {
    let start = path.find("tracks[")? + "tracks[".len();
    let end = path[start..].find(']')? + start;
    path[start..end].parse().ok()
}

// --- properties -----------------------------------------------------------

proptest! {
    // Determinism (spec 5.5): two plan runs over identical inputs (same
    // directory, fresh identifier caches) serialize byte-for-byte identically.
    #[test]
    fn plan_is_byte_identical_across_runs(
        profile in arb_profile(),
        files in arb_files(),
    ) {
        let dir = tempfile::tempdir().unwrap();
        let mut by_name = HashMap::new();
        for (name, id) in &files {
            std::fs::write(dir.path().join(name), b"x").unwrap();
            by_name.insert(name.clone(), id.clone());
        }
        let run = RunInputs {
            source: dir.path().to_path_buf(),
            output: Some(dir.path().join("out")),
            on_collision: None,
        };
        let mut i1 = FakeIdent { by_name: by_name.clone() };
        let mut i2 = FakeIdent { by_name };
        let b1 = plan_batch(&profile, &run, &mut i1, &lang());
        let b2 = plan_batch(&profile, &run, &mut i2, &lang());
        prop_assert_eq!(
            serde_json::to_string(&b1).unwrap(),
            serde_json::to_string(&b2).unwrap()
        );
    }

    // Rendered-name invariants (D4, spec 4.8): every produced plan's output
    // filename has no path separator, ends in `.mkv` over a non-empty stem,
    // and sits directly in the output directory (no injected subdirectory).
    #[test]
    fn produced_plan_names_are_well_formed(
        files in arb_render_files(),
        filename in arb_filename(),
    ) {
        let profile = mk_profile(
            vec![rule_of(exact_one("type", Scalar::Str("video".to_string())), true)],
            filename,
        );
        let (batch, out_dir) = run_plan(&profile, &files);
        for f in &batch.files {
            let Some(plan) = &f.plan else { continue };
            let name = plan.output.file_name().and_then(|s| s.to_str());
            prop_assert!(name.is_some(), "output has no file name: {:?}", plan.output);
            let name = name.unwrap();
            prop_assert!(
                !name.contains('/') && !name.contains('\\'),
                "separator in rendered name {:?}",
                name
            );
            prop_assert!(name.to_lowercase().ends_with(".mkv"), "name not .mkv: {:?}", name);
            let stem = &name[..name.len() - 4];
            prop_assert!(
                !stem.is_empty() && stem != "." && stem != "..",
                "degenerate stem {:?}",
                stem
            );
            prop_assert_eq!(
                plan.output.parent(),
                Some(out_dir.as_path()),
                "output {:?} escaped its directory",
                plan.output
            );
        }
    }

    // D6 (spec 5.3): every suggestion the engine emits, re-applied to the
    // profile and re-planned, resolves its rule's ambiguity everywhere and
    // introduces no diagnostic absent from the pre-edit baseline.
    #[test]
    fn accepted_suggestion_survives_replan(scene in arb_ambiguous_ident()) {
        let profile = mk_profile(
            vec![rule_of(exact_one("type", Scalar::Str("subtitles".to_string())), false)],
            FilenameCfg::keep(),
        );
        let files = vec![("Show.S01E01.mkv".to_string(), scene)];
        let (batch, _) = run_plan(&profile, &files);

        // Meaningfulness guard: the scenario must actually be ambiguous and
        // actually yield suggestions, or there is nothing to test.
        prop_assume!(batch.files.iter().any(|f| f
            .diagnostics
            .iter()
            .any(|d| d.code == DiagCode::AmbiguousRule)));
        prop_assume!(!batch.suggestions.is_empty());

        let base = diag_sig(&batch);
        for s in &batch.suggestions {
            let ri = rule_index_of(&s.config_path).expect("suggestion names a rule");
            let edited = apply_suggestion(&profile, ri, &s.yaml_fragment);
            let (re, _) = run_plan(&edited, &files);

            let still_ambiguous = re.files.iter().flat_map(|f| f.diagnostics.iter()).any(|d| {
                d.code == DiagCode::AmbiguousRule && rule_index_of(&d.config_path) == Some(ri)
            });
            prop_assert!(
                !still_ambiguous,
                "suggestion {:?} did not resolve rule {}",
                s.edit,
                ri
            );

            for sig in diag_sig(&re) {
                prop_assert!(
                    base.contains(&sig),
                    "suggestion {:?} introduced new diagnostic {}",
                    s.edit,
                    sig
                );
            }
        }
    }
}
