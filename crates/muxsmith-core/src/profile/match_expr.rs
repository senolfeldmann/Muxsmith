//! Match algebra (spec 4.3): conjunction of exact/substring/regex maps
//! plus recursive `any` (at least one holds) and `not` (none hold).

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A scalar value inside `exact`/`changes` maps (spec 4.3/4.4).
/// `#[serde(untagged)]`: deserialization tries each variant in the order
/// declared below, so ordering is semantically load-bearing. A plain
/// numeric literal (`3`) matches `Int` before it ever reaches `Float`
/// (Int is tried first), so only literals with a decimal point become
/// `Float`; `Bool` is tried before `Str` so `true`/`false` bind as booleans
/// rather than one-word strings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Scalar {
    /// A boolean value, e.g. `forced_track: true`.
    Bool(bool),
    /// A whole-number value, e.g. a track `id`.
    Int(i64),
    /// A floating-point value, e.g. `min_luminance`.
    Float(f64),
    /// A string value, e.g. `language: en` or a `track_name`.
    Str(String),
}

impl Scalar {
    /// Human-readable type label matching [`crate::capability::PropType`]'s
    /// naming (`"boolean"`/`"integer"`/`"float"`/`"string"`); fills the
    /// `found` param of a `ValueTypeMismatch` diagnostic when a profile
    /// value's runtime type doesn't match the property's declared type.
    pub fn type_name(&self) -> &'static str {
        match self {
            Scalar::Bool(_) => "boolean",
            Scalar::Int(_) => "integer",
            Scalar::Float(_) => "float",
            Scalar::Str(_) => "string",
        }
    }
}

/// One match expression: a conjunction of up to five parts, all present
/// parts must hold (spec 4.3). Evaluated against every track/attachment of
/// a resolved source; `any`/`not` recurse to arbitrary depth (typical
/// profiles stay flat).
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MatchExpr {
    /// Property -> value equality after normalization; case-sensitive for
    /// strings (language values are normalized separately, spec 4.4).
    /// Multiple entries in the map are AND'd.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<BTreeMap<String, Scalar>>,
    /// Property -> substring, case-insensitive containment; string
    /// properties only ([`crate::capability::PropType::String`] required),
    /// otherwise a config-time type error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substring: Option<BTreeMap<String, String>>,
    /// Property -> regex, taken as written (use `(?i)` for case-insensitive
    /// matching); string properties only, same type-error rule as `substring`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<BTreeMap<String, String>>,
    /// At least one sub-expression must hold (logical OR over the list).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<MatchExpr>>,
    /// No sub-expression may hold (logical NOR over the list).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Vec<MatchExpr>>,
}

impl MatchExpr {
    /// True when every field is absent or holds an empty map/list, i.e. the
    /// expression imposes no condition at all. `validate.rs` flags this as
    /// the `EmptyMatchExpression` warning: a rule with an empty expression
    /// would match every track of its source.
    pub fn is_empty(&self) -> bool {
        self.exact.as_ref().is_none_or(|m| m.is_empty())
            && self.substring.as_ref().is_none_or(|m| m.is_empty())
            && self.regex.as_ref().is_none_or(|m| m.is_empty())
            && self.any.as_ref().is_none_or(|v| v.is_empty())
            && self.not.as_ref().is_none_or(|v| v.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_expression() {
        let y = r#"
exact: { type: subtitles, forced_track: true }
substring: { track_name: SDH }
"#;
        let e: MatchExpr = yaml_serde::from_str(y).unwrap();
        assert_eq!(
            e.exact.as_ref().unwrap()["type"],
            Scalar::Str("subtitles".into())
        );
        assert_eq!(
            e.exact.as_ref().unwrap()["forced_track"],
            Scalar::Bool(true)
        );
        assert_eq!(e.substring.as_ref().unwrap()["track_name"], "SDH");
        assert!(e.any.is_none() && e.not.is_none());
        assert!(!e.is_empty());
    }

    #[test]
    fn parses_nested_any_and_not() {
        let y = r#"
exact: { type: subtitles }
not:
  - substring: { track_name: SDH }
  - exact: { flag_hearing_impaired: true }
any:
  - regex: { track_name: '(?i)forced' }
"#;
        let e: MatchExpr = yaml_serde::from_str(y).unwrap();
        let not = e.not.as_ref().unwrap();
        assert_eq!(not.len(), 2);
        assert_eq!(not[0].substring.as_ref().unwrap()["track_name"], "SDH");
        assert_eq!(
            not[1].exact.as_ref().unwrap()["flag_hearing_impaired"],
            Scalar::Bool(true)
        );
        let any = e.any.as_ref().unwrap();
        assert_eq!(any.len(), 1);
        assert_eq!(any[0].regex.as_ref().unwrap()["track_name"], "(?i)forced");
    }

    #[test]
    fn rejects_unknown_keys() {
        let y = "exactt: { type: video }";
        assert!(yaml_serde::from_str::<MatchExpr>(y).is_err());
    }

    #[test]
    fn empty_expression_reports_empty() {
        let e: MatchExpr = yaml_serde::from_str("{}").unwrap();
        assert!(e.is_empty());
    }

    #[test]
    fn scalar_type_names() {
        assert_eq!(Scalar::Bool(true).type_name(), "boolean");
        assert_eq!(Scalar::Int(3).type_name(), "integer");
        assert_eq!(Scalar::Float(1.5).type_name(), "float");
        assert_eq!(Scalar::Str("x".into()).type_name(), "string");
    }
}
