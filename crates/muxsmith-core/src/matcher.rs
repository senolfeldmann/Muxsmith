//! Match-expression evaluation (spec 4.3, 4.4). Pure and total: given an
//! expression, a [`Matchable`] item, and a language index, decides membership
//! with no I/O and no diagnostics. The correctness core; covered by unit
//! tests here and by the planner's fixture tests. Config validity (unknown
//! property, wrong type, bad enum value) is checked earlier (validate) or
//! reported by the planner; this function assumes a validated expression and
//! answers only "does this item match?".

use crate::capability::codec_kind_prefixes;
use crate::capability::runtime::LanguageIndex;
use crate::capability::{PropType, matchable_type};
use crate::identify::{Attachment, PropValue, Track};
use crate::profile::match_expr::{MatchExpr, Scalar};

/// A property-bearing item the matcher can evaluate a [`MatchExpr`] against.
/// [`Track`] and [`Attachment`] implement it, each reusing the same match
/// algebra over its own flat property namespace.
pub trait Matchable {
    /// The value of a match property, or `None` if absent.
    fn get(&self, prop: &str) -> Option<PropValue>;
}

impl Matchable for Track {
    fn get(&self, prop: &str) -> Option<PropValue> {
        Track::get(self, prop)
    }
}

impl Matchable for Attachment {
    fn get(&self, prop: &str) -> Option<PropValue> {
        Attachment::get(self, prop)
    }
}

// `Iterator::filter`'s predicate takes `&Self::Item`, so filtering an
// iterator of `&Track` (e.g. `Vec<Track>::iter()`) hands the predicate a
// `&&Track`. Generic trait-bound unification does not apply deref coercion
// the way a concrete `&Track` parameter would, so without this blanket impl
// `M` would resolve to `&Track` and fail the `Matchable` bound at existing
// call sites. This keeps those sites compiling unchanged.
impl<M: Matchable> Matchable for &M {
    fn get(&self, prop: &str) -> Option<PropValue> {
        (**self).get(prop)
    }
}

/// Whether `item` satisfies `expr` (spec 4.3): the conjunction of all present
/// parts. `lang` normalizes language tokens so ISO 639-2 and BCP-47 values
/// compare equal (spec 4.4).
pub fn matches<M: Matchable>(expr: &MatchExpr, item: &M, lang: &LanguageIndex) -> bool {
    if let Some(exact) = &expr.exact {
        for (prop, want) in exact {
            if !exact_matches(prop, want, item, lang) {
                return false;
            }
        }
    }
    if let Some(sub) = &expr.substring {
        for (prop, needle) in sub {
            match item_str(prop, item) {
                Some(hay) if hay.to_lowercase().contains(&needle.to_lowercase()) => {}
                _ => return false,
            }
        }
    }
    if let Some(rx) = &expr.regex {
        for (prop, pattern) in rx {
            let hay = match item_str(prop, item) {
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
        && !any.iter().any(|e| matches(e, item, lang))
    {
        return false;
    }
    if let Some(not) = &expr.not
        && not.iter().any(|e| matches(e, item, lang))
    {
        return false;
    }
    true
}

fn exact_matches<M: Matchable>(prop: &str, want: &Scalar, item: &M, lang: &LanguageIndex) -> bool {
    match prop {
        // language matches against both `language` and `language_ietf`,
        // normalized so `de` and `ger` are equal (spec 4.4).
        "language" => {
            let Scalar::Str(want) = want else {
                return false;
            };
            ["language", "language_ietf"]
                .iter()
                .filter_map(|f| item_str(f, item))
                .any(|have| lang_eq(want, &have, lang))
        }
        // codec_kind is a codec_id prefix match over a curated alias set.
        "codec_kind" => {
            let Scalar::Str(kind) = want else {
                return false;
            };
            let Some(prefixes) = codec_kind_prefixes(kind) else {
                return false;
            };
            match item_str("codec_id", item) {
                Some(id) => prefixes.iter().any(|p| id.starts_with(p)),
                None => false,
            }
        }
        // A boolean-typed property mkvmerge omitted is Matroska
        // false-when-absent (spec 4.4): the vanity flags (hearing-impaired,
        // commentary, ...) are only emitted when set, so absence must
        // compare equal to `false` for exact matching, same as a track that
        // reported the flag as `false` explicitly.
        _ => match item.get(prop) {
            Some(have) => scalar_eq(want, &have),
            None => match matchable_type(prop) {
                Some(PropType::Boolean) => scalar_eq(want, &PropValue::Bool(false)),
                _ => false,
            },
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

/// The string form of a matchable item's property, for substring/regex/
/// language. Only `PropValue::Str` yields a value; numeric/boolean
/// properties are not strings.
fn item_str<M: Matchable>(prop: &str, item: &M) -> Option<String> {
    match item.get(prop) {
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
        assert!(!matches(
            &expr("exact: { default_track: true }"),
            &t,
            &lang()
        ));
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
        assert!(matches(
            &expr("substring: { track_name: sdh }"),
            &t,
            &lang()
        ));
        assert!(matches(
            &expr("regex: { track_name: '(?i)^english' }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("regex: { track_name: '^SDH' }"),
            &t,
            &lang()
        ));
    }

    #[test]
    fn any_and_not_recurse() {
        let t = track(
            "subtitles",
            &[("track_name", PropValue::Str("English SDH".into()))],
        );
        assert!(matches(
            &expr(
                "any:\n  - substring: { track_name: SDH }\n  - substring: { track_name: forced }"
            ),
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

    #[test]
    fn absent_boolean_property_compares_equal_to_false() {
        // mkvmerge emits vanity flags only when set; Matroska defines them
        // false-when-absent (spec 4.4), so a track that never set the flag
        // must match `exact: { flag_hearing_impaired: false }` and must not
        // match `exact: { flag_hearing_impaired: true }`.
        let t = track("audio", &[]);
        assert!(matches(
            &expr("exact: { flag_hearing_impaired: false }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("exact: { flag_hearing_impaired: true }"),
            &t,
            &lang()
        ));
    }

    #[test]
    fn present_boolean_property_still_matches_its_real_value() {
        let t = track("audio", &[("flag_hearing_impaired", PropValue::Bool(true))]);
        assert!(matches(
            &expr("exact: { flag_hearing_impaired: true }"),
            &t,
            &lang()
        ));
        assert!(!matches(
            &expr("exact: { flag_hearing_impaired: false }"),
            &t,
            &lang()
        ));
    }

    #[test]
    fn absent_non_boolean_property_still_does_not_match() {
        let t = track("subtitles", &[]);
        assert!(!matches(&expr("exact: { track_name: X }"), &t, &lang()));
    }

    #[test]
    fn matches_is_generic_over_matchable() {
        fn check<M: Matchable>(m: &M) -> bool {
            matches(&expr("exact: { type: audio }"), m, &lang())
        }
        let t = track("audio", &[]);
        assert!(check(&t));
    }

    #[test]
    fn attachment_matching_uses_the_same_algebra() {
        use crate::identify::Attachment;
        let font = Attachment {
            id: 1,
            file_name: "Roboto.ttf".into(),
            size: 100,
            content_type: Some("font/ttf".into()),
            description: None,
            uid: None,
        };
        assert!(matches(
            &expr("substring: { file_name: robot }"),
            &font,
            &lang()
        ));
        assert!(matches(
            &expr("exact: { content_type: font/ttf }"),
            &font,
            &lang()
        ));
        assert!(matches(
            &expr("any:\n  - substring: { file_name: .ttf }\n  - substring: { file_name: .otf }"),
            &font,
            &lang()
        ));
        assert!(!matches(
            &expr("exact: { description: whatever }"),
            &font,
            &lang()
        ));
        assert!(!matches(
            &expr("substring: { content_type: pdf }"),
            &font,
            &lang()
        ));
    }
}
