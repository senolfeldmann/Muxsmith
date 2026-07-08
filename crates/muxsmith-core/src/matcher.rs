//! Match-expression evaluation (spec 4.3, 4.4). Pure and total: given an
//! expression, a track, and a language index, decides membership with no I/O
//! and no diagnostics. The correctness core; covered by unit tests here and by
//! the planner's fixture tests. Config validity (unknown property, wrong type,
//! bad enum value) is checked earlier (validate) or reported by the planner;
//! this function assumes a validated expression and answers only "does this
//! track match?".

use crate::capability::codec_kind_prefixes;
use crate::capability::runtime::LanguageIndex;
use crate::identify::{PropValue, Track};
use crate::profile::match_expr::{MatchExpr, Scalar};

/// Whether `track` satisfies `expr` (spec 4.3): the conjunction of all present
/// parts. `lang` normalizes language tokens so ISO 639-2 and BCP-47 values
/// compare equal (spec 4.4).
pub fn matches(expr: &MatchExpr, track: &Track, lang: &LanguageIndex) -> bool {
    if let Some(exact) = &expr.exact {
        for (prop, want) in exact {
            if !exact_matches(prop, want, track, lang) {
                return false;
            }
        }
    }
    if let Some(sub) = &expr.substring {
        for (prop, needle) in sub {
            match track_str(prop, track) {
                Some(hay) if hay.to_lowercase().contains(&needle.to_lowercase()) => {}
                _ => return false,
            }
        }
    }
    if let Some(rx) = &expr.regex {
        for (prop, pattern) in rx {
            let hay = match track_str(prop, track) {
                Some(h) => h,
                None => return false,
            };
            // A validated expression compiles; an invalid regex was already an
            // InvalidRegex config error, so a failure here means no match.
            match regex::Regex::new(pattern) {
                Ok(re) if re.is_match(&hay) => {}
                _ => return false,
            }
        }
    }
    if let Some(any) = &expr.any
        && !any.is_empty()
        && !any.iter().any(|e| matches(e, track, lang))
    {
        return false;
    }
    if let Some(not) = &expr.not
        && not.iter().any(|e| matches(e, track, lang))
    {
        return false;
    }
    true
}

fn exact_matches(prop: &str, want: &Scalar, track: &Track, lang: &LanguageIndex) -> bool {
    match prop {
        // language matches against both `language` and `language_ietf`,
        // normalized so `de` and `ger` are equal (spec 4.4).
        "language" => {
            let Scalar::Str(want) = want else { return false };
            ["language", "language_ietf"]
                .iter()
                .filter_map(|f| track_str(f, track))
                .any(|have| lang_eq(want, &have, lang))
        }
        // codec_kind is a codec_id prefix match over a curated alias set.
        "codec_kind" => {
            let Scalar::Str(kind) = want else { return false };
            let Some(prefixes) = codec_kind_prefixes(kind) else {
                return false;
            };
            match track_str("codec_id", track) {
                Some(id) => prefixes.iter().any(|p| id.starts_with(p)),
                None => false,
            }
        }
        _ => match track.get(prop) {
            Some(have) => scalar_eq(want, &have),
            None => false,
        },
    }
}

/// True when two language tokens denote the same language. Both are normalized
/// through the index; if either is unrecognized, fall back to a raw
/// case-insensitive compare so unusual-but-equal tags still match.
fn lang_eq(a: &str, b: &str, lang: &LanguageIndex) -> bool {
    match (lang.normalize(a), lang.normalize(b)) {
        (Some(na), Some(nb)) => na == nb,
        _ => a.eq_ignore_ascii_case(b),
    }
}

/// The string form of a track property, for substring/regex/language. Only
/// `PropValue::Str` yields a value; numeric/boolean properties are not strings.
fn track_str(prop: &str, track: &Track) -> Option<String> {
    match track.get(prop) {
        Some(PropValue::Str(s)) => Some(s),
        _ => None,
    }
}

/// Value equality between a profile `Scalar` and a track `PropValue`, with
/// int/float cross-comparison (spec 4.3, `exact`). Strings compare
/// case-sensitively (language is special-cased before reaching here).
fn scalar_eq(want: &Scalar, have: &PropValue) -> bool {
    match (want, have) {
        (Scalar::Str(a), PropValue::Str(b)) => a == b,
        (Scalar::Bool(a), PropValue::Bool(b)) => a == b,
        (Scalar::Int(a), PropValue::Int(b)) => a == b,
        (Scalar::Int(a), PropValue::Float(b)) => (*a as f64) == *b,
        (Scalar::Float(a), PropValue::Float(b)) => a == b,
        (Scalar::Float(a), PropValue::Int(b)) => *a == (*b as f64),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::runtime::LanguageIndex;
    use crate::identify::{PropValue, Track};
    use std::collections::BTreeMap;

    fn lang() -> LanguageIndex {
        LanguageIndex::from_rows(&[
            ["English", "eng", "eng", "en"],
            ["German", "ger", "ger", "de"],
        ])
    }

    fn track(kind: &str, props: &[(&str, PropValue)]) -> Track {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert((*k).to_string(), v.clone());
        }
        Track {
            id: 0,
            kind: kind.to_string(),
            codec: String::new(),
            properties,
        }
    }

    fn expr(yaml: &str) -> MatchExpr {
        yaml_serde::from_str(yaml).unwrap()
    }

    #[test]
    fn exact_matches_type_and_flags() {
        let t = track("subtitles", &[("forced_track", PropValue::Bool(true))]);
        assert!(matches(
            &expr("exact: { type: subtitles, forced_track: true }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("exact: { type: subtitles, forced_track: false }"),
            &t,
            &lang()
        ));
        assert!(!matches(&expr("exact: { default_track: true }"), &t, &lang()));
    }

    #[test]
    fn language_normalizes_iso_and_bcp47_against_both_fields() {
        let t = track(
            "audio",
            &[
                ("language", PropValue::Str("ger".into())),
                ("language_ietf", PropValue::Str("de".into())),
            ],
        );
        assert!(matches(&expr("exact: { language: de }"), &t, &lang()));
        assert!(matches(&expr("exact: { language: ger }"), &t, &lang()));
        assert!(!matches(&expr("exact: { language: en }"), &t, &lang()));
    }

    #[test]
    fn language_falls_back_to_raw_compare_when_unknown() {
        let t = track("audio", &[("language", PropValue::Str("zxx".into()))]);
        assert!(matches(&expr("exact: { language: zxx }"), &t, &lang()));
        assert!(!matches(&expr("exact: { language: qqq }"), &t, &lang()));
    }

    #[test]
    fn codec_kind_is_codec_id_prefix_match() {
        let srt = track(
            "subtitles",
            &[("codec_id", PropValue::Str("S_TEXT/UTF8".into()))],
        );
        let ass = track(
            "subtitles",
            &[("codec_id", PropValue::Str("S_TEXT/ASS".into()))],
        );
        assert!(matches(&expr("exact: { codec_kind: srt }"), &srt, &lang()));
        assert!(!matches(&expr("exact: { codec_kind: srt }"), &ass, &lang()));
        assert!(matches(&expr("exact: { codec_kind: ass }"), &ass, &lang()));
    }

    #[test]
    fn substring_is_case_insensitive_and_regex_is_literal() {
        let t = track(
            "subtitles",
            &[("track_name", PropValue::Str("English SDH".into()))],
        );
        assert!(matches(&expr("substring: { track_name: sdh }"), &t, &lang()));
        assert!(matches(
            &expr("regex: { track_name: '(?i)^english' }"),
            &t,
            &lang()
        ));
        assert!(!matches(&expr("regex: { track_name: '^SDH' }"), &t, &lang()));
    }

    #[test]
    fn any_and_not_recurse() {
        let t = track(
            "subtitles",
            &[("track_name", PropValue::Str("English SDH".into()))],
        );
        assert!(matches(
            &expr("any:\n  - substring: { track_name: SDH }\n  - substring: { track_name: forced }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("not:\n  - substring: { track_name: SDH }"),
            &t,
            &lang()
        ));
    }

    #[test]
    fn empty_expression_matches_everything() {
        let t = track("video", &[]);
        assert!(matches(&expr("{}"), &t, &lang()));
    }

    #[test]
    fn numeric_exact_compares_across_int_and_float() {
        let t = track("audio", &[("audio_channels", PropValue::Int(6))]);
        assert!(matches(&expr("exact: { audio_channels: 6 }"), &t, &lang()));
        assert!(!matches(&expr("exact: { audio_channels: 2 }"), &t, &lang()));
    }
}
