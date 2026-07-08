//! Static overlap lint (spec 5.4): flags PROVABLE overlaps only.
//! Decidable case: two primary-source rules whose expressions are
//! exact-only, where one condition map is a subset of the other. Any
//! track matching the superset rule then necessarily matches the
//! subset rule. Everything else is left to the planner's dry run.

use crate::report::{DiagCode, Diagnostic};

use super::match_expr::MatchExpr;
use super::model::{Profile, SourceCfg};

pub fn provable_overlaps(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let exact_only: Vec<(usize, &MatchExpr)> = profile
        .tracks
        .iter()
        .enumerate()
        .filter(|(_, r)| matches!(&r.source, SourceCfg::Keyword(k) if k == "primary"))
        .filter(|(_, r)| is_exact_only(&r.match_expr))
        .map(|(i, r)| (i, &r.match_expr))
        .collect();

    for (ai, (a_idx, a)) in exact_only.iter().enumerate() {
        for (b_idx, b) in exact_only.iter().skip(ai + 1) {
            if subset_of(a, b) || subset_of(b, a) {
                diags.push(
                    Diagnostic::warning(DiagCode::ProvableOverlap, format!("tracks[{b_idx}]"))
                        .with("rule_a", a_idx.to_string())
                        .with("rule_b", b_idx.to_string()),
                );
            }
        }
    }
    diags
}

fn is_exact_only(e: &MatchExpr) -> bool {
    e.substring.is_none()
        && e.regex.is_none()
        && e.any.is_none()
        && e.not.is_none()
        && e.exact.as_ref().is_some_and(|m| !m.is_empty())
}

/// True if every condition in `a` also exists identically in `b`:
/// then any track matching `b` also matches `a`.
fn subset_of(a: &MatchExpr, b: &MatchExpr) -> bool {
    let (Some(a), Some(b)) = (&a.exact, &b.exact) else {
        return false;
    };
    a.iter().all(|(k, v)| b.get(k) == Some(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::load::{Format, from_str};
    use crate::report::DiagCode;

    fn lint(y: &str) -> Vec<crate::report::Diagnostic> {
        provable_overlaps(&from_str(y, Format::Yaml).unwrap())
    }

    #[test]
    fn subset_conditions_are_provable_overlap() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio } }
  - match: { exact: { type: audio, language: en } }
"#;
        let diags = lint(y);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagCode::ProvableOverlap);
        assert_eq!(diags[0].params["rule_a"], "0");
        assert_eq!(diags[0].params["rule_b"], "1");
    }

    #[test]
    fn identical_exact_rules_are_provable_overlap() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: en } }
  - match: { exact: { type: audio, language: en } }
"#;
        assert_eq!(lint(y).len(), 1);
    }

    #[test]
    fn disjoint_exact_values_are_not_flagged() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: en } }
  - match: { exact: { type: audio, language: de } }
"#;
        assert!(lint(y).is_empty());
    }

    #[test]
    fn rules_with_negations_or_regex_are_skipped() {
        // Not exact-only: undecidable statically, planner handles it.
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles } }
  - match:
      exact: { type: subtitles }
      not:
        - substring: { track_name: SDH }
"#;
        assert!(lint(y).is_empty());
    }

    #[test]
    fn reversed_direction_overlap_is_flagged() {
        // Superset-conditions rule first: locks the subset_of(b, a) branch.
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio, language: en } }
  - match: { exact: { type: audio } }
"#;
        let diags = lint(y);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].params["rule_a"], "0");
        assert_eq!(diags[0].params["rule_b"], "1");
    }

    #[test]
    fn rules_with_any_are_skipped() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles } }
  - match:
      exact: { type: subtitles }
      any:
        - substring: { track_name: SDH }
"#;
        assert!(lint(y).is_empty());
    }

    #[test]
    fn rules_with_substring_are_skipped() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles } }
  - match:
      exact: { type: subtitles }
      substring: { track_name: SDH }
"#;
        assert!(lint(y).is_empty());
    }

    #[test]
    fn external_source_rules_are_skipped() {
        let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: subtitles } }
  - source:
      external: { path: '.', extensions: [srt], match_to_source: true }
    match: { exact: { type: subtitles } }
"#;
        assert!(lint(y).is_empty());
    }
}
