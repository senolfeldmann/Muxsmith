//! Property-based tests for language normalization (spec 4.4; spec 10
//! deferred property suite, #1). Covers the `LanguageIndex` data structure
//! (idempotent normalization, case-insensitivity, per-row canonical
//! agreement, well-formed-tag acceptance) and the canonical language
//! equality the matcher exposes through `exact: { language: ... }`
//! (reflexivity and symmetry over ISO codes, BCP-47 tags, and arbitrary
//! UTF-8).

use std::collections::BTreeMap;

use proptest::collection::btree_set;
use proptest::prelude::*;
use proptest::sample::select;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{PropValue, Track};
use muxsmith_core::matcher::matches;
use muxsmith_core::profile::match_expr::{MatchExpr, Scalar};

mod support;
use support::lang;

// A lowercase base-26 alphabetic string, injective over `u32` (0 -> "a",
// 25 -> "z", 26 -> "aa", ...). Used to mint globally distinct synthetic
// language codes.
fn alpha(mut n: u32) -> String {
    let mut s = String::new();
    loop {
        s.insert(0, (b'a' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    s
}

// Rows for `LanguageIndex::from_rows`, one per distinct id, with globally
// unique codes (id*3 / id*3+1 / id*3+2 never collide across rows). Real
// `mkvmerge --list-languages` output has each ISO code appear once; disjoint
// codes mirror that invariant, which is what makes normalization idempotent
// (a shared code across two canonical groups would break it, but that input
// never occurs). The 639-3 code is always present (stable canonical key);
// 639-2 and 639-1 are optionally blanked.
fn arb_rows() -> impl Strategy<Value = Vec<[String; 4]>> {
    btree_set(0u32..40, 1..6).prop_map(|ids| {
        ids.into_iter()
            .map(|id| {
                let c3 = alpha(id * 3);
                // Vary which optional codes are present, deterministically per
                // id, so a single index spans rows carrying all three codes and
                // rows carrying only some.
                let c2 = if id % 2 == 0 {
                    alpha(id * 3 + 1)
                } else {
                    String::new()
                };
                let c1 = if id % 3 == 0 {
                    alpha(id * 3 + 2)
                } else {
                    String::new()
                };
                [format!("Lang{id}"), c3, c2, c1]
            })
            .collect()
    })
}

fn index_of(rows: &[[String; 4]]) -> LanguageIndex {
    let borrowed: Vec<[&str; 4]> = rows
        .iter()
        .map(|r| [r[0].as_str(), r[1].as_str(), r[2].as_str(), r[3].as_str()])
        .collect();
    LanguageIndex::from_rows(&borrowed)
}

// All non-empty codes appearing in the generated rows, plus a couple of
// arbitrary tokens, as normalization inputs.
fn tokens_of(rows: &[[String; 4]]) -> Vec<String> {
    let mut toks: Vec<String> = Vec::new();
    for r in rows {
        for code in &r[1..4] {
            if !code.is_empty() {
                toks.push(code.clone());
            }
        }
    }
    toks.push("zzz-not-a-code".to_string());
    toks
}

// A grammatically well-formed BCP-47 tag: `lang[-Script][-REGION]`, all
// subtags RFC 5646-shaped. `is_valid_value` must accept it (well-formedness
// only, D19), whether or not the language exists.
fn arb_wellformed_tag() -> impl Strategy<Value = String> {
    (
        "[a-z]{2,3}",
        proptest::option::of("[A-Z][a-z]{3}"),
        proptest::option::of("[A-Z]{2}"),
    )
        .prop_map(|(lang, script, region)| {
            let mut t = lang;
            if let Some(s) = script {
                t.push('-');
                t.push_str(&s);
            }
            if let Some(r) = region {
                t.push('-');
                t.push_str(&r);
            }
            t
        })
}

// A language token to feed the matcher's `exact: { language }` path: real
// ISO codes present in the shared `lang()` index, well-formed BCP-47 tags,
// and arbitrary UTF-8.
fn arb_lang_token() -> impl Strategy<Value = String> {
    prop_oneof![
        select(vec!["en", "eng", "de", "ger", "tr", "tur", "und", "zxx"])
            .prop_map(|s| s.to_string()),
        arb_wellformed_tag(),
        any::<String>(),
    ]
}

fn track_with_language(value: &str) -> Track {
    let mut properties = BTreeMap::new();
    properties.insert("language".to_string(), PropValue::Str(value.to_string()));
    Track {
        id: 0,
        kind: "subtitles".to_string(),
        codec: String::new(),
        properties,
    }
}

fn language_expr(value: &str) -> MatchExpr {
    let mut map = BTreeMap::new();
    map.insert("language".to_string(), Scalar::Str(value.to_string()));
    MatchExpr {
        exact: Some(map),
        ..Default::default()
    }
}

proptest! {
    // Normalization is idempotent: a token's canonical key is its own
    // canonical key. `normalize(normalize(x)) == normalize(x)` in the sense
    // that the canonical form is a fixed point.
    #[test]
    fn normalize_is_idempotent(rows in arb_rows(), extra in any::<String>()) {
        let idx = index_of(&rows);
        let mut toks = tokens_of(&rows);
        toks.push(extra);
        for t in toks {
            if let Some(c) = idx.normalize(&t) {
                prop_assert_eq!(idx.normalize(&c), Some(c.clone()),
                    "canonical key {:?} of {:?} is not a fixed point", c, t);
            }
        }
    }

    // Normalization is case-insensitive: flipping ASCII case cannot change
    // the canonical key.
    #[test]
    fn normalize_is_case_insensitive(rows in arb_rows(), t in any::<String>()) {
        let idx = index_of(&rows);
        let mut toks = tokens_of(&rows);
        toks.push(t);
        for tok in toks {
            let upper = tok.to_uppercase();
            prop_assert_eq!(idx.normalize(&tok), idx.normalize(&upper));
        }
    }

    // Every non-empty code within one row shares a single canonical key, so
    // the 639-1/2/3 spellings of a language all compare equal.
    #[test]
    fn codes_in_a_row_share_one_canonical(rows in arb_rows()) {
        let idx = index_of(&rows);
        for r in &rows {
            let codes: Vec<&String> = r[1..4].iter().filter(|c| !c.is_empty()).collect();
            let mut canonical: Option<String> = None;
            for code in codes {
                let n = idx.normalize(code);
                prop_assert!(n.is_some(), "row code {:?} did not normalize", code);
                match &canonical {
                    None => canonical = n,
                    Some(prev) => prop_assert_eq!(prev, &n.unwrap(),
                        "codes in one row disagree on canonical key"),
                }
            }
        }
    }

    // `is_valid_value` is a superset of `normalize`: anything that normalizes
    // is a valid value.
    #[test]
    fn valid_value_contains_normalizable(rows in arb_rows(), t in any::<String>()) {
        let idx = index_of(&rows);
        let mut toks = tokens_of(&rows);
        toks.push(t);
        for tok in toks {
            if idx.normalize(&tok).is_some() {
                prop_assert!(idx.is_valid_value(&tok));
            }
        }
    }

    // A grammatically well-formed BCP-47 tag is a valid language value even
    // against an empty index (accepted via the tag-grammar path, D19).
    #[test]
    fn wellformed_tags_are_valid_values(tag in arb_wellformed_tag()) {
        let idx = LanguageIndex::default();
        prop_assert!(idx.is_valid_value(&tag), "well-formed tag {:?} rejected", tag);
    }

    // Neither query panics on arbitrary UTF-8.
    #[test]
    fn queries_are_total_on_arbitrary_input(rows in arb_rows(), t in any::<String>()) {
        let idx = index_of(&rows);
        let _ = idx.normalize(&t);
        let _ = idx.is_valid_value(&t);
    }

    // Canonical language equality (as the matcher decides it) is reflexive:
    // a track's own language value always satisfies an exact match on it.
    #[test]
    fn language_equality_is_reflexive(x in arb_lang_token()) {
        let idx = lang();
        prop_assert!(matches(&language_expr(&x), &track_with_language(&x), &idx),
            "language {:?} did not match itself", x);
    }

    // Canonical language equality is symmetric: swapping which value sits in
    // the profile and which in the track cannot change the verdict.
    #[test]
    fn language_equality_is_symmetric(a in arb_lang_token(), b in arb_lang_token()) {
        let idx = lang();
        let ab = matches(&language_expr(&a), &track_with_language(&b), &idx);
        let ba = matches(&language_expr(&b), &track_with_language(&a), &idx);
        prop_assert_eq!(ab, ba, "language equality asymmetric for {:?} vs {:?}", a, b);
    }
}
