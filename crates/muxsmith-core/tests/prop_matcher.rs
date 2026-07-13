//! Property-based tests for the match algebra (spec 4.3, 4.4; spec 10
//! deferred property suite, #1). Exercises `matcher::matches` over generated
//! validated expressions and generated tracks: double-negation identity,
//! `any` singleton/order/disjunction laws, `not` NOR semantics, totality on
//! arbitrary UTF-8, and the standing invariant that the generator only
//! produces expressions the config-time validator accepts (so "invalid regex
//! is impossible post-validation" holds by construction).

use std::collections::BTreeMap;

use proptest::collection::vec as prop_vec;
use proptest::prelude::*;
use proptest::sample::select;

use muxsmith_core::capability::{CODEC_KIND_NAMES, TYPE_VALUES};
use muxsmith_core::identify::{PropValue, Track};
use muxsmith_core::matcher::matches;
use muxsmith_core::profile::match_expr::{MatchExpr, Scalar};
use muxsmith_core::profile::model::KeepDrop;
use muxsmith_core::profile::model::{
    AttachmentsCfg, ChaptersCfg, Input, OutputCfg, Profile, SourceCfg, TagsCfg, TitleCfg,
    TrackRule, TracksCfg,
};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::Severity;

mod support;
use support::lang;

// --- vocabulary -----------------------------------------------------------

// String-typed matchable properties. `codec`/`type`/`id` are handled via
// their own leaves (they read the top-level Track fields, not `properties`).
const STRING_PROPS: &[&str] = &[
    "track_name",
    "language",
    "language_ietf",
    "codec_id",
    "codec",
];
// Track `properties`-map string keys only (excludes `codec`, a Track field).
const STRING_PROPS_TRACK: &[&str] = &["track_name", "language", "language_ietf", "codec_id"];
const BOOL_PROPS: &[&str] = &[
    "default_track",
    "forced_track",
    "flag_hearing_impaired",
    "flag_commentary",
    "flag_original",
    "enabled_track",
    "flag_visual_impaired",
];
const INT_PROPS: &[&str] = &["audio_channels", "id", "number", "stream_id"];
// `properties`-map integer keys only (excludes `id`, a Track field).
const INT_PROPS_TRACK: &[&str] = &["audio_channels", "number", "stream_id"];
const CODEC_POOL: &[&str] = &["AAC", "AVC/H.264", "SubRip/SRT", ""];
// A small pool of string values, shared between track properties and
// expression leaves, so exact/substring frequently hit (non-vacuous
// disjunction/NOR cases) while still spanning misses.
const STRING_POOL: &[&str] = &[
    "eng",
    "ger",
    "tur",
    "en",
    "de",
    "tr",
    "English",
    "German",
    "Forced",
    "SDH",
    "Commentary",
    "S_TEXT/UTF8",
    "S_TEXT/ASS",
    "",
];
// Curated valid regex patterns: every one compiles, so a regex leaf built
// from these is a validated leaf (no InvalidRegex possible post-validation).
const REGEXES: &[&str] = &[
    "abc", "a.c", "^S_", "UTF8$", "(?i)eng", "[a-z]+", ".*", "x?y", "a|b", "\\d+", "", ".*SDH.*",
];

// --- MatchExpr constructors (struct-update to dodge field_reassign_with_default) ---

fn exact_one(prop: &str, val: Scalar) -> MatchExpr {
    MatchExpr {
        exact: Some(BTreeMap::from([(prop.to_string(), val)])),
        ..Default::default()
    }
}

fn substring_one(prop: &str, val: String) -> MatchExpr {
    MatchExpr {
        substring: Some(BTreeMap::from([(prop.to_string(), val)])),
        ..Default::default()
    }
}

fn regex_one(prop: &str, val: &str) -> MatchExpr {
    MatchExpr {
        regex: Some(BTreeMap::from([(prop.to_string(), val.to_string())])),
        ..Default::default()
    }
}

fn any_of(v: Vec<MatchExpr>) -> MatchExpr {
    MatchExpr {
        any: Some(v),
        ..Default::default()
    }
}

fn not_of(v: Vec<MatchExpr>) -> MatchExpr {
    MatchExpr {
        not: Some(v),
        ..Default::default()
    }
}

// --- strategies -----------------------------------------------------------

// Free-text string value: weighted toward the shared pool (for overlap with
// track values) but including genuinely arbitrary UTF-8 (for totality).
fn free_string() -> impl Strategy<Value = String> {
    prop_oneof![
        3 => select(STRING_POOL).prop_map(|s| s.to_string()),
        1 => any::<String>(),
    ]
}

// A validated MatchExpr: every leaf is type-correct, closed-domain-valid
// (`type`/`codec_kind`), string-only for substring/regex, codec_kind only
// under exact, regex patterns compile, and `any`/`not` lists are non-empty.
fn arb_expr() -> impl Strategy<Value = MatchExpr> {
    let leaf = prop_oneof![
        (select(STRING_PROPS), free_string()).prop_map(|(p, v)| exact_one(p, Scalar::Str(v))),
        select(TYPE_VALUES).prop_map(|t| exact_one("type", Scalar::Str(t.to_string()))),
        select(CODEC_KIND_NAMES).prop_map(|k| exact_one("codec_kind", Scalar::Str(k.to_string()))),
        (select(BOOL_PROPS), any::<bool>()).prop_map(|(p, b)| exact_one(p, Scalar::Bool(b))),
        (select(INT_PROPS), 0i64..8).prop_map(|(p, i)| exact_one(p, Scalar::Int(i))),
        (select(STRING_PROPS), free_string()).prop_map(|(p, v)| substring_one(p, v)),
        (select(STRING_PROPS), select(REGEXES)).prop_map(|(p, r)| regex_one(p, r)),
    ];
    leaf.prop_recursive(3, 32, 3, |inner| {
        prop_oneof![
            prop_vec(inner.clone(), 1..4).prop_map(any_of),
            prop_vec(inner, 1..3).prop_map(not_of),
        ]
    })
}

fn arb_prop_entry() -> impl Strategy<Value = (String, PropValue)> {
    prop_oneof![
        (select(STRING_PROPS_TRACK), free_string())
            .prop_map(|(p, v)| (p.to_string(), PropValue::Str(v))),
        (select(BOOL_PROPS), any::<bool>()).prop_map(|(p, b)| (p.to_string(), PropValue::Bool(b))),
        (select(INT_PROPS_TRACK), 0i64..8).prop_map(|(p, i)| (p.to_string(), PropValue::Int(i))),
    ]
}

fn arb_track() -> impl Strategy<Value = Track> {
    (
        select(TYPE_VALUES),
        select(CODEC_POOL),
        0u64..6,
        prop_vec(arb_prop_entry(), 0..6),
    )
        .prop_map(|(kind, codec, id, entries)| Track {
            id,
            kind: kind.to_string(),
            codec: codec.to_string(),
            properties: entries.into_iter().collect(),
        })
}

// A minimal valid profile carrying `expr` as its single track rule's match,
// so the config-time validator can be run against the generated expression.
fn profile_with_expr(expr: MatchExpr) -> Profile {
    Profile {
        profile_version: 1,
        meta: None,
        input: Input {
            pattern: r"S(\d{2})E(\d{2})".to_string(),
            extensions: vec!["mkv".to_string()],
            recursive: true,
        },
        output: OutputCfg::default(),
        tracks: TracksCfg {
            unmatched: KeepDrop::Drop,
            rules: vec![TrackRule {
                source: SourceCfg::primary(),
                match_expr: expr,
                optional: false,
                changes: None,
            }],
        },
        attachments: AttachmentsCfg::default(),
        chapters: ChaptersCfg::default(),
        tags: TagsCfg::default(),
        title: TitleCfg::default(),
    }
}

// --- properties -----------------------------------------------------------

proptest! {
    // Double negation is the identity of the algebra: `not(not(e))` decides
    // membership exactly as `e` does, for any track (spec 4.3).
    #[test]
    fn not_not_is_identity(track in arb_track(), e in arb_expr()) {
        let idx = lang();
        let wrapped = not_of(vec![not_of(vec![e.clone()])]);
        prop_assert_eq!(
            matches(&wrapped, &track, &idx),
            matches(&e, &track, &idx)
        );
    }

    // A singleton `any` is its element: `any([e]) == e` (spec 4.3).
    #[test]
    fn any_singleton_equals_inner(track in arb_track(), e in arb_expr()) {
        let idx = lang();
        prop_assert_eq!(
            matches(&any_of(vec![e.clone()]), &track, &idx),
            matches(&e, &track, &idx)
        );
    }

    // `any` is order-insensitive: the disjunction's truth does not depend on
    // the order of its clauses (reversal and rotation must all agree).
    #[test]
    fn any_is_order_insensitive(track in arb_track(), es in prop_vec(arb_expr(), 1..4)) {
        let idx = lang();
        let base = matches(&any_of(es.clone()), &track, &idx);

        let reversed: Vec<MatchExpr> = es.iter().rev().cloned().collect();
        prop_assert_eq!(base, matches(&any_of(reversed), &track, &idx));

        if es.len() > 1 {
            let mut rotated = es.clone();
            rotated.rotate_left(1);
            prop_assert_eq!(base, matches(&any_of(rotated), &track, &idx));
        }
    }

    // `any` is exactly the disjunction over its clauses.
    #[test]
    fn any_is_disjunction(track in arb_track(), es in prop_vec(arb_expr(), 1..4)) {
        let idx = lang();
        let expected = es.iter().any(|e| matches(e, &track, &idx));
        prop_assert_eq!(matches(&any_of(es), &track, &idx), expected);
    }

    // `not` is exactly the NOR over its clauses: it holds iff none hold.
    #[test]
    fn not_is_nor(track in arb_track(), es in prop_vec(arb_expr(), 1..3)) {
        let idx = lang();
        let expected = !es.iter().any(|e| matches(e, &track, &idx));
        prop_assert_eq!(matches(&not_of(es), &track, &idx), expected);
    }

    // Totality + determinism: matching an arbitrary validated expression
    // against an arbitrary track (arbitrary UTF-8 in values/patterns) never
    // panics and is a pure function of its inputs. Invalid regex is
    // impossible here because regex leaves are drawn from compiling patterns.
    #[test]
    fn matcher_is_total_and_deterministic(track in arb_track(), e in arb_expr()) {
        let idx = lang();
        let a = matches(&e, &track, &idx);
        let b = matches(&e, &track, &idx);
        prop_assert_eq!(a, b);
    }

    // The generator's contract: every expression it produces passes
    // config-time validation with no error under the rule's `match` path.
    // This is what makes "invalid regex is impossible post-validation" a fact
    // rather than an assumption for the properties above.
    #[test]
    fn generated_exprs_are_validation_clean(e in arb_expr()) {
        let profile = profile_with_expr(e);
        let bad: Vec<_> = validate(&profile)
            .into_iter()
            .filter(|d| d.severity == Severity::Error && d.config_path.starts_with("tracks[0].match"))
            .collect();
        prop_assert!(bad.is_empty(), "generated expr failed validation: {:?}", bad);
    }
}
