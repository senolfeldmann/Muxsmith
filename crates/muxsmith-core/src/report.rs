//! Diagnostics as data. Core never produces user-facing prose; renderers
//! (CLI, GUI) map `DiagCode::key()` + `params` to Fluent messages.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

macro_rules! diag_codes {
    ($($variant:ident => $key:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum DiagCode {
            $($variant),+
        }

        impl DiagCode {
            pub fn key(self) -> &'static str {
                match self {
                    $(DiagCode::$variant => $key),+
                }
            }
        }
    };
}

diag_codes! {
    // Config-time (validate)
    UnsupportedProfileVersion => "unsupported-profile-version",
    ParseError => "parse-error",
    NoTrackRules => "no-track-rules",
    EmptyMatchExpression => "empty-match-expression",
    EmptyExtensions => "empty-extensions",
    InvalidRegex => "invalid-regex",
    UnknownProperty => "unknown-property",
    NotStringProperty => "not-string-property",
    ValueTypeMismatch => "value-type-mismatch",
    UnknownSettableProperty => "unknown-settable-property",
    InvalidKeyword => "invalid-keyword",
    LocatorConflict => "locator-conflict",
    InvalidTemplate => "invalid-template",
    UnknownTemplateField => "unknown-template-field",
    UnknownTemplateFilter => "unknown-template-filter",
    PathSeparatorInTemplate => "path-separator-in-template",
    AttachmentRuleShape => "attachment-rule-shape",
    ProvableOverlap => "provable-overlap",
    // Planning-time (produced from Plan 2 on; defined now for a stable catalog)
    AmbiguousRule => "ambiguous-rule",
    OverlappingRules => "overlapping-rules",
    MissingTrack => "missing-track",
    MissingExternal => "missing-external",
    AmbiguousExternal => "ambiguous-external",
    OutputCollision => "output-collision",
    SourceOverwrite => "source-overwrite",
    DuplicateIdentifier => "duplicate-identifier",
    DonorIsPrimary => "donor-is-primary",
    IgnoredFile => "ignored-file",
    MultipleIdentifierMatches => "multiple-identifier-matches",
    UnknownPropertySkew => "unknown-property-skew",
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub severity: Severity,
    pub config_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<PathBuf>,
    pub params: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_ref: Option<usize>,
}

impl Diagnostic {
    fn new(code: DiagCode, severity: Severity, config_path: impl Into<String>) -> Self {
        Diagnostic {
            code,
            severity,
            config_path: config_path.into(),
            file: None,
            params: BTreeMap::new(),
            suggestion_ref: None,
        }
    }

    pub fn error(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Error, config_path)
    }

    pub fn warning(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Warning, config_path)
    }

    pub fn info(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Info, config_path)
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn for_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.file = Some(path.into());
        self
    }
}

pub fn worst_severity(diags: &[Diagnostic]) -> Option<Severity> {
    diags.iter().map(|d| d.severity).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_orders_info_warning_error() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
    }

    #[test]
    fn diag_code_keys_are_kebab_case() {
        assert_eq!(DiagCode::InvalidRegex.key(), "invalid-regex");
        assert_eq!(DiagCode::AmbiguousRule.key(), "ambiguous-rule");
        assert_eq!(DiagCode::UnknownSettableProperty.key(), "unknown-settable-property");
    }

    #[test]
    fn diagnostic_builder_sets_fields() {
        let d = Diagnostic::error(DiagCode::InvalidRegex, "input.pattern")
            .with("detail", "unclosed group");
        assert_eq!(d.severity, Severity::Error);
        assert_eq!(d.config_path, "input.pattern");
        assert_eq!(d.params["detail"], "unclosed group");
        assert!(d.file.is_none());
    }

    #[test]
    fn worst_severity_picks_error_over_warning() {
        let diags = vec![
            Diagnostic::warning(DiagCode::ProvableOverlap, "tracks[0]"),
            Diagnostic::error(DiagCode::InvalidRegex, "input.pattern"),
        ];
        assert_eq!(worst_severity(&diags), Some(Severity::Error));
        assert_eq!(worst_severity(&[]), None);
    }

    #[test]
    fn diagnostic_serializes_with_snake_case_severity_and_kebab_code() {
        let d = Diagnostic::error(DiagCode::UnknownProperty, "tracks[1].match.exact.foo")
            .with("property", "foo");
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["severity"], "error");
        assert_eq!(json["code"], "unknown-property");
        assert_eq!(json["params"]["property"], "foo");
    }
}
