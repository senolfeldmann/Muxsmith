//! Diagnostics as data. Core never produces user-facing prose; renderers
//! (CLI, GUI) map `DiagCode::key()` + `params` to Fluent messages.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;

/// Diagnostic severity (spec 5.2). `Ord`-derived in declaration order
/// (`Info < Warning < Error`), giving [`worst_severity`] and the CLI's
/// error-first sort a total order to compare/sort on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Informational; does not affect exit code or plan validity.
    Info,
    /// Non-fatal; the CLI exit code becomes 1 if this is the worst
    /// severity present (spec 8.1).
    Warning,
    /// Fatal for the affected file/rule: a file with any error-severity
    /// diagnostic produces no plan (spec 5.1); CLI exit code becomes 2.
    Error,
}

/// Generates `DiagCode` from a flat list of `variant => key` pairs.
/// Per-variant semantics live in the spec 5.2 catalog table rather than in
/// per-variant doc comments, since this macro has no channel to carry prose
/// distinct from the wire key without changing every call site below.
macro_rules! diag_codes {
    ($($variant:ident => $key:literal),+ $(,)?) => {
        /// Diagnostic code identifying the condition that produced a
        /// [`Diagnostic`] (spec 5.2). Every variant corresponds to exactly
        /// one row of the spec 5.2 catalog table (condition + severity).
        /// [`DiagCode::key`] returns that row's kebab-case wire string,
        /// which equals the `#[serde(rename_all = "kebab-case")]` JSON
        /// encoding below (`all_keys_match_serde_encoding` test enforces
        /// this equality; CI's catalog-completeness guard, spec 10, checks
        /// every code has a Fluent message template).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum DiagCode {
            $(
                #[doc = "See the spec 5.2 catalog table for this code's condition and severity."]
                $variant
            ),+
        }

        impl DiagCode {
            /// Every defined diagnostic code, in declaration order. Backs
            /// the tests (and CI's catalog-completeness guard, spec 10)
            /// that check for unique keys and key/serde agreement across
            /// the whole catalog.
            pub const ALL: &'static [DiagCode] = &[$(DiagCode::$variant),+];

            /// The kebab-case wire string for this code (e.g.
            /// `"ambiguous-rule"`), identical to its serde JSON encoding.
            /// Renderers key the Fluent message/hint templates on this
            /// string (spec 5.2, 8.4).
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

/// One diagnostic (spec 5.2): a data record, never prose. `code` +
/// `params` select and fill a message/hint template at presentation time
/// (spec 8.4); core itself never renders text.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Diagnostic {
    /// Which condition produced this diagnostic (spec 5.2 catalog).
    pub code: DiagCode,
    /// Error/warning/info; error means the affected file produces no plan
    /// (spec 5.1).
    pub severity: Severity,
    /// Dotted/bracketed path into the profile this diagnostic refers to
    /// (e.g. `tracks[1].match.exact.language`), independent of `file`.
    pub config_path: String,
    /// The source file this diagnostic is about, when it is planning-time
    /// (per-file) rather than config-time; `None` for static validate-time
    /// diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<PathBuf>,
    /// Structured values a renderer interpolates into the message/hint
    /// template selected by `code` (spec 5.2, 8.4); keys are template
    /// placeholder names, e.g. `"detail"`, `"property"`, `"found"`.
    pub params: BTreeMap<String, String>,
    /// Index into the batch's suggestion list when the suggestion engine
    /// (spec 5.3) produced a structured fix for this diagnostic.
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

    /// Builds an error-severity diagnostic: the affected file (if any)
    /// produces no plan (spec 5.1).
    pub fn error(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Error, config_path)
    }

    /// Builds a warning-severity diagnostic: reported but does not block
    /// planning.
    pub fn warning(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Warning, config_path)
    }

    /// Builds an info-severity diagnostic: purely advisory (e.g.
    /// `IgnoredFile`, `MultipleIdentifierMatches`).
    pub fn info(code: DiagCode, config_path: impl Into<String>) -> Self {
        Self::new(code, Severity::Info, config_path)
    }

    /// Attaches one param key/value, overwriting any prior value for
    /// `key`. Builder-style: chain calls to fill every placeholder the
    /// message/hint template for `code` expects (spec 8.4).
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Sets `file`, turning a config-time diagnostic into a per-file
    /// (planning-time) one.
    pub fn for_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.file = Some(path.into());
        self
    }
}

/// The highest severity present, or `None` for an empty slice.
/// `Some(Severity::Error)` is what turns a validate/dry-run/run exit code
/// into 2 rather than 0/1 (spec 5.1, 8.1).
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
        assert_eq!(
            DiagCode::UnknownSettableProperty.key(),
            "unknown-settable-property"
        );
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

    #[test]
    fn all_keys_match_serde_encoding() {
        for &code in DiagCode::ALL {
            assert_eq!(
                serde_json::to_value(code).unwrap(),
                serde_json::Value::String(code.key().to_string()),
                "key()/serde mismatch for {code:?}"
            );
        }
    }

    #[test]
    fn all_keys_are_unique() {
        let mut map: BTreeMap<&str, DiagCode> = BTreeMap::new();
        for &code in DiagCode::ALL {
            assert!(
                map.insert(code.key(), code).is_none(),
                "duplicate key: {}",
                code.key()
            );
        }
    }
}
