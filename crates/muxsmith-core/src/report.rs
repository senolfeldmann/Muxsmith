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

/// Generates `DiagCode` from a list of `variant => key` pairs, forwarding
/// each variant's doc attributes onto the generated enum so the wire
/// contract stays documented per code in rustdoc.
macro_rules! diag_codes {
    ($($(#[$meta:meta])* $variant:ident => $key:literal),+ $(,)?) => {
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
            $($(#[$meta])* $variant),+
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
    /// The profile declares a `profile_version` this build does not read; v1 accepts only 1 (spec 4).
    UnsupportedProfileVersion => "unsupported-profile-version",
    /// The profile file could not be read or deserialized; `detail` carries the underlying I/O or serde message, `config_path`/`at` the failing field when known.
    ParseError => "parse-error",
    /// `tracks` is empty: a profile must select at least one track to produce any output.
    NoTrackRules => "no-track-rules",
    /// A rule's match expression imposes no condition at all, so it would match every track of its source (warning).
    EmptyMatchExpression => "empty-match-expression",
    /// An `extensions` list (input or locator) is empty, so no file could ever qualify as a candidate.
    EmptyExtensions => "empty-extensions",
    /// A regex failed to compile (`input.pattern` or a `regex` condition); `detail` carries the compiler message.
    InvalidRegex => "invalid-regex",
    /// A match condition references a property absent from the capability model's matchable set (config-time error, spec 5.2).
    UnknownProperty => "unknown-property",
    /// `codec_kind` was used under a `substring`/`regex` condition; it is a curated alias and matchable only under `exact` (spec 4.4). Pattern-match `codec_id` instead. `condition` param names the misused condition.
    CodecKindExactOnly => "codec-kind-exact-only",
    /// An `exact` condition value lies outside a closed value domain: `type`/`codec_kind` are checked at config time, `language` at plan time (spec 4.4). `property`/`value` params carry the offender, `allowed` a hint sample.
    InvalidPropertyValue => "invalid-property-value",
    /// An `any` or `not` list is present but has zero sub-expressions (spec 4.3); such a list is either always-false (`any`) or always-true (`not`) and is almost certainly a mistake.
    EmptyMatchList => "empty-match-list",
    /// A `substring` or `regex` condition targets a non-string property; both are defined for string properties only (spec 4.3).
    NotStringProperty => "not-string-property",
    /// A condition or change value's type does not fit the property's declared type (an integer fits a float property, never the reverse).
    ValueTypeMismatch => "value-type-mismatch",
    /// A `changes` key is not in the curated settable table (spec 4.4); matchable-only properties cannot be set.
    UnknownSettableProperty => "unknown-settable-property",
    /// A keyword-position string is not an allowed keyword for that field; the `found`/`allowed` params spell out both.
    InvalidKeyword => "invalid-keyword",
    /// A locator sets both `match_to_source` and `match_pattern`, which are mutually exclusive (spec 4.6).
    LocatorConflict => "locator-conflict",
    /// Template source failed to parse (unclosed brace or empty field); `kind`/`pos` params identify the failure, `pos` as a char offset.
    InvalidTemplate => "invalid-template",
    /// A template references a field outside its context's allowed set (`{match}`, pattern capture groups, `{source_stem}` in literal mode only).
    UnknownTemplateField => "unknown-template-field",
    /// A template field uses a filter other than `int`, `pad2`, or `pad3` (spec 4.7).
    UnknownTemplateFilter => "unknown-template-filter",
    /// An output filename template contains a path separator; v1 never creates subdirectories (spec 4.8).
    PathSeparatorInTemplate => "path-separator-in-template",
    /// An attachment rule does not set exactly one of `select`/`drop`/`add` (spec 4.9).
    AttachmentRuleShape => "attachment-rule-shape",
    /// Static lint: one exact-only rule's conditions subsume another's, so any track matching the stricter rule must overlap the looser one (warning, spec 5.4).
    ProvableOverlap => "provable-overlap",
    // Planning-time (produced from Plan 2 on; defined now for a stable catalog)
    /// A rule matched two or more tracks of its source; strict uniqueness requires exactly one, and `optional` does not relax this (spec 5.2).
    AmbiguousRule => "ambiguous-rule",
    /// One track is claimed by two or more rules; every overlap is an error under strict independent uniqueness (spec 2, 5.2).
    OverlappingRules => "overlapping-rules",
    /// A non-optional rule matched zero tracks; the hint lists near-misses (same type/language) and which condition each failed (spec 5.2).
    MissingTrack => "missing-track",
    /// An external locator (track rule or chapters) found zero files for a non-optional use (spec 5.2).
    MissingExternal => "missing-external",
    /// An external locator (track rule or chapters) found two or more candidate files; exactly one donor is required (spec 4.6).
    AmbiguousExternal => "ambiguous-external",
    /// A discovered primary or resolved donor file exists but mkvmerge could not identify it (spec 5.2); `detail` carries the underlying error text.
    UnidentifiableSource => "unidentifiable-source",
    /// Rendered output path already exists or is produced by two plans; severity follows the `on_collision` policy (spec 4.8).
    OutputCollision => "output-collision",
    /// The rendered output filename contains a path separator (`/` or `\`); v1 never creates subdirectories, checked on the rendered name on all platforms (spec 4.8). `name` param carries the rendered name.
    PathSeparatorInRenderedName => "path-separator-in-rendered-name",
    /// The rendered output filename has an empty stem or is `.`/`..` (spec 4.8); the ".mkv appended if missing" rule would otherwise produce a hidden or invalid file. `name` param carries the rendered name.
    EmptyRenderedName => "empty-rendered-name",
    /// An output path equals an input path: a hard error regardless of collision policy, since sources are never modified (spec 4.8).
    SourceOverwrite => "source-overwrite",
    /// Two primaries yield the same identifier (e.g. 720p and 1080p copies): both are muxed, both attract the same external files, and templates may collide (warning).
    DuplicateIdentifier => "duplicate-identifier",
    /// An external donor file is itself a primary: it will be muxed as its own output and donate tracks (warning, spec 5.2).
    DonorIsPrimary => "donor-is-primary",
    /// A file's extension matches but `input.pattern` does not; the file is skipped (info).
    IgnoredFile => "ignored-file",
    /// `input.pattern` matched more than once in a basename; the first match is used as the identifier (info, spec 4.2).
    MultipleIdentifierMatches => "multiple-identifier-matches",
    /// Property reported by a newer mkvmerge identification schema than this build knows; matched untyped (spec 9.2).
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
    fn plan2_codes_are_registered_with_keys() {
        assert_eq!(DiagCode::CodecKindExactOnly.key(), "codec-kind-exact-only");
        assert_eq!(
            DiagCode::InvalidPropertyValue.key(),
            "invalid-property-value"
        );
        assert_eq!(
            DiagCode::PathSeparatorInRenderedName.key(),
            "path-separator-in-rendered-name"
        );
        assert_eq!(DiagCode::EmptyRenderedName.key(), "empty-rendered-name");
        for c in [
            DiagCode::CodecKindExactOnly,
            DiagCode::InvalidPropertyValue,
            DiagCode::PathSeparatorInRenderedName,
            DiagCode::EmptyRenderedName,
        ] {
            assert!(DiagCode::ALL.contains(&c), "{c:?} missing from ALL");
        }
    }

    #[test]
    fn f2_codes_are_registered_with_keys() {
        assert_eq!(DiagCode::EmptyMatchList.key(), "empty-match-list");
        assert_eq!(
            DiagCode::UnidentifiableSource.key(),
            "unidentifiable-source"
        );
        for c in [DiagCode::EmptyMatchList, DiagCode::UnidentifiableSource] {
            assert!(DiagCode::ALL.contains(&c), "{c:?} missing from ALL");
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
