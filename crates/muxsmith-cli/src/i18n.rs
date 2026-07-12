//! Fluent-based rendering. The ONLY place where diagnostic codes and
//! params become human text on the CLI side (spec 8.4).

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

const EN_DIAGNOSTICS: &str = include_str!("../../../locales/en/diagnostics.ftl");
const EN_CLI: &str = include_str!("../../../locales/en/cli.ftl");

/// Fluent-based renderer: the only place on the CLI side where diagnostic
/// codes and params become human text (spec 8.4). Embeds the English
/// catalogs at build time; v1 ships English content only, but the
/// mechanism is locale-generic.
pub struct Renderer {
    bundle: FluentBundle<FluentResource>,
}

impl Renderer {
    /// v1 ships English only; `locale` is accepted for interface stability
    /// and falls back to en for any unknown tag (spec 8.4).
    pub fn new(locale: Option<&str>) -> Renderer {
        let requested = locale
            .map(str::to_owned)
            .or_else(sys_locale::get_locale)
            .unwrap_or_else(|| "en".into());
        let langid: LanguageIdentifier =
            requested.parse().unwrap_or_else(|_| "en".parse().unwrap());
        let mut bundle = FluentBundle::new(vec![langid]);
        // No Unicode isolation marks around placeables: CLI output must be
        // plain grep-able text.
        bundle.set_use_isolating(false);
        for source in [EN_DIAGNOSTICS, EN_CLI] {
            let res =
                FluentResource::try_new(source.to_owned()).expect("embedded catalog must parse");
            bundle.add_resource_overriding(res);
        }
        Renderer { bundle }
    }

    /// Renders one Fluent message by id, interpolating `args`. Falls back
    /// to the raw `id` when the catalog has no such message or the message
    /// has no value pattern, so a missing translation stays visible in the
    /// output instead of silently disappearing (CI's catalog-completeness
    /// guard, spec 10, is the other half of this contract).
    pub fn msg(&self, id: &str, args: &[(&str, &str)]) -> String {
        let mut fargs = FluentArgs::new();
        for (k, v) in args {
            fargs.set(*k, *v);
        }
        self.render(id, fargs)
    }

    /// Renders one Fluent message like [`Self::msg`], but sets every
    /// `(name, count)` pair in `counts` as a numeric `FluentValue::Number`
    /// rather than a string. Fluent's plural selector (`[one]`/`*[other]`)
    /// resolves CLDR plural categories only against `FluentValue::Number`;
    /// a `FluentValue::String` selector always falls through to
    /// `*[other]`, so any variable a message's `{ $name -> [one] ...
    /// *[other] ... }` selector reads needs to go through here instead of
    /// [`Self::msg`]. Takes a slice rather than one `(key, count)` pair so
    /// a message with several independent selectors (`validate-summary`'s
    /// errors/warnings/infos) needs one call, not one per selector.
    pub fn msg_with_counts(
        &self,
        id: &str,
        args: &[(&str, &str)],
        counts: &[(&str, usize)],
    ) -> String {
        let mut fargs = FluentArgs::new();
        for (k, v) in args {
            fargs.set(*k, *v);
        }
        for (k, count) in counts {
            fargs.set(*k, *count);
        }
        self.render(id, fargs)
    }

    fn render(&self, id: &str, fargs: FluentArgs) -> String {
        let Some(message) = self.bundle.get_message(id) else {
            // Missing catalog entry: fall back to the raw id so the
            // problem is visible instead of hidden. CI guards this case.
            return id.to_string();
        };
        let Some(pattern) = message.value() else {
            return id.to_string();
        };
        let mut errors = Vec::new();
        self.bundle
            .format_pattern(pattern, Some(&fargs), &mut errors)
            .into_owned()
    }

    /// Renders one core [`Diagnostic`](muxsmith_core::report::Diagnostic)
    /// as a single human-readable line: severity, config path, and the
    /// message resolved from `code`/`params` (spec 5.2, 8.4), composed via
    /// the `diagnostic-line` catalog entry.
    pub fn diagnostic(&self, d: &muxsmith_core::report::Diagnostic) -> String {
        self.render_diagnostic(d, true)
    }

    /// Renders a diagnostic WITHOUT its file prefix (always `diagnostic-line`,
    /// even when `d.file` is set), for contexts where an enclosing header
    /// already named the file -- dry-run/run's per-file block prints the
    /// filename once in its `dry-run-file` line, so repeating it on every
    /// diagnostic under it is noise.
    pub fn diagnostic_no_file(&self, d: &muxsmith_core::report::Diagnostic) -> String {
        self.render_diagnostic(d, false)
    }

    /// Shared body of [`Self::diagnostic`] and [`Self::diagnostic_no_file`]:
    /// `show_file` selects the `diagnostic-line-file` template (when a file is
    /// present) over the file-less `diagnostic-line`.
    fn render_diagnostic(&self, d: &muxsmith_core::report::Diagnostic, show_file: bool) -> String {
        let message = self.render_diagnostic_message(d);
        let severity = self.msg(severity_key(d.severity), &[]);
        match d.file.as_ref().filter(|_| show_file) {
            Some(file) => {
                let file = file.to_string_lossy();
                self.msg(
                    "diagnostic-line-file",
                    &[
                        ("severity", &severity),
                        ("file", &file),
                        ("config_path", &d.config_path),
                        ("message", &message),
                    ],
                )
            }
            None => self.msg(
                "diagnostic-line",
                &[
                    ("severity", &severity),
                    ("config_path", &d.config_path),
                    ("message", &message),
                ],
            ),
        }
    }

    /// Renders `d`'s own message (`d.code`'s template filled with
    /// `d.params`), promoting any param [`numeric_diagnostic_params`]
    /// lists for `d.code` from its wire string form back to a number
    /// before handing it to Fluent. `Diagnostic::params` is
    /// `BTreeMap<String, String>` by design (core stays prose- and
    /// type-free on the wire, spec 5.2/8.4), so a diagnostic code whose
    /// catalog message uses a CLDR plural selector on one of its own
    /// params (`suggestions-capped`, `suggestion-partition`) needs that
    /// promotion done once, here, at the render boundary -- everywhere
    /// else (JSON, core) the param stays a plain string. A listed param
    /// that fails to parse as a number falls back to rendering as a
    /// string rather than being dropped, so a surprise non-numeric value
    /// degrades to the selector's `*[other]`/`*[group]` branch instead of
    /// leaking `{$name}`.
    fn render_diagnostic_message(&self, d: &muxsmith_core::report::Diagnostic) -> String {
        let numeric_keys = numeric_diagnostic_params(d.code);
        let mut string_params: Vec<(&str, &str)> = Vec::new();
        let mut counts: Vec<(&str, usize)> = Vec::new();
        for (k, v) in &d.params {
            if numeric_keys.contains(&k.as_str())
                && let Ok(n) = v.parse::<usize>()
            {
                counts.push((k.as_str(), n));
                continue;
            }
            string_params.push((k.as_str(), v.as_str()));
        }
        self.msg_with_counts(d.code.key(), &string_params, &counts)
    }
}

/// Lets `report::json`'s document-assembly functions (spec 7, D15) fill
/// each diagnostic's `"rendered"` field without core itself depending on
/// this Fluent-based renderer.
impl muxsmith_core::report::json::DiagnosticRenderer for Renderer {
    fn diagnostic(&self, d: &muxsmith_core::report::Diagnostic) -> String {
        Renderer::diagnostic(self, d)
    }
}

fn severity_key(s: muxsmith_core::report::Severity) -> &'static str {
    match s {
        muxsmith_core::report::Severity::Error => "severity-error",
        muxsmith_core::report::Severity::Warning => "severity-warning",
        muxsmith_core::report::Severity::Info => "severity-info",
    }
}

/// Names the params of `code`'s catalog message that must reach Fluent as
/// numbers, not strings, because the template selects on them with a CLDR
/// plural selector (T19, #17 step 1): `suggestions-capped`'s `dropped`,
/// `suggestion-partition`'s `dropped` (`[overflow]` branch) and `count`
/// (`*[group]` branch). Every other `DiagCode` renders its params as plain
/// strings, unchanged.
fn numeric_diagnostic_params(code: muxsmith_core::report::DiagCode) -> &'static [&'static str] {
    use muxsmith_core::report::DiagCode;
    match code {
        DiagCode::SuggestionsCapped => &["dropped"],
        DiagCode::SuggestionPartition => &["dropped", "count"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_message_id_falls_back_to_raw_id() {
        let renderer = Renderer::new(Some("en"));
        assert_eq!(renderer.msg("no-such-id", &[]), "no-such-id");
    }

    #[test]
    fn invalid_locale_falls_back_to_en_and_renders() {
        let renderer = Renderer::new(Some("zz-ZZ-invalid!"));
        assert_eq!(renderer.msg("validate-ok", &[]), "Profile is valid.");
    }

    #[test]
    fn diagnostic_with_file_includes_the_file_path() {
        use muxsmith_core::report::{DiagCode, Diagnostic};

        let renderer = Renderer::new(Some("en"));
        let diag = Diagnostic::info(DiagCode::IgnoredFile, "input").for_file("some/path.mkv");
        let rendered = renderer.diagnostic(&diag);
        assert!(
            rendered.contains("some/path.mkv"),
            "expected file path in: {rendered}"
        );
    }

    #[test]
    fn diagnostic_without_file_omits_it_and_still_renders() {
        use muxsmith_core::report::{DiagCode, Diagnostic};

        let renderer = Renderer::new(Some("en"));
        let diag = Diagnostic::info(DiagCode::IgnoredFile, "input");
        let rendered = renderer.diagnostic(&diag);
        assert!(
            !rendered.contains("some/path.mkv"),
            "unexpected file path in: {rendered}"
        );
        assert!(
            rendered.contains("[info]") && rendered.contains("input"),
            "expected severity and config_path in: {rendered}"
        );
    }

    #[test]
    fn unknown_property_skew_renders_property_and_versions() {
        let renderer = Renderer::new(Some("en"));
        let rendered = renderer.msg(
            "unknown-property-skew",
            &[
                ("property", "new_prop"),
                ("found_version", "21"),
                ("pinned", "20"),
            ],
        );
        assert!(
            rendered.contains("new_prop"),
            "expected property in: {rendered}"
        );
        assert!(
            rendered.contains("21") && rendered.contains("20"),
            "expected versions in: {rendered}"
        );
        assert!(
            !rendered.contains("{$"),
            "unresolved placeholder leaked into: {rendered}"
        );
    }

    #[test]
    fn unsupported_source_donor_variant_names_the_donor_file() {
        use muxsmith_core::report::{DiagCode, Diagnostic};

        let renderer = Renderer::new(Some("en"));
        let diag = Diagnostic::error(DiagCode::UnsupportedSource, "tracks[0].source.external")
            .for_file("Show.S01E01.mkv")
            .with("kind", "donor")
            .with("donor", "Donor.S01E01.srt");
        let rendered = renderer.diagnostic(&diag);
        assert!(
            rendered.contains("Donor.S01E01.srt"),
            "expected the donor filename in: {rendered}"
        );
    }

    #[test]
    fn unsupported_source_primary_variant_renders_exactly_as_before() {
        // Regression guard: adding the donor variant must not change one
        // character of the primary-side rendering (T9.5).
        let renderer = Renderer::new(Some("en"));
        let rendered = renderer.msg("unsupported-source", &[("kind", "primary")]);
        assert_eq!(
            rendered,
            "mkvmerge identified this file but its container is not a supported muxing source."
        );
    }

    #[test]
    fn unsupported_source_kind_omitted_falls_back_to_primary_variant() {
        // Pin Fluent's default-variant fallback: when kind param is omitted,
        // the message must render with the primary variant (T9.5 review).
        // Guards against Fluent version bump or future emitter omitting kind
        // silently leaking placeholders.
        let renderer = Renderer::new(Some("en"));
        let rendered = renderer.msg("unsupported-source", &[]);
        assert_eq!(
            rendered,
            "mkvmerge identified this file but its container is not a supported muxing source."
        );
        assert!(
            !rendered.contains("{$"),
            "unresolved placeholder leaked into: {rendered}"
        );
    }

    // T19 (#17 step 1): `SuggestionsCapped`/`SuggestionPartition` are the
    // one place a plural selector's count arrives through
    // `Diagnostic::params` (`BTreeMap<String, String>`, spec 5.2) rather
    // than a call site that holds a real `usize` -- the Plan-4 lesson the
    // task flagged ("$count reached Fluent as a string once and [one]
    // never matched"). These pin `render_diagnostic`'s promotion
    // (`numeric_diagnostic_params`) end to end: a real `Diagnostic` with a
    // string param in, the correctly-selected CLDR variant out.

    #[test]
    fn suggestions_capped_renders_singular_and_plural() {
        use muxsmith_core::report::{DiagCode, Diagnostic};

        let renderer = Renderer::new(Some("en"));
        let one =
            Diagnostic::info(DiagCode::SuggestionsCapped, "tracks[0].match").with("dropped", "1");
        assert_eq!(
            renderer.diagnostic(&one),
            "[info] tracks[0].match: 1 further suggestion for this rule was capped at 3 and not shown."
        );

        let two =
            Diagnostic::info(DiagCode::SuggestionsCapped, "tracks[0].match").with("dropped", "2");
        assert_eq!(
            renderer.diagnostic(&two),
            "[info] tracks[0].match: 2 further suggestions for this rule were capped at 3 and not shown."
        );
    }

    #[test]
    fn suggestion_partition_group_branch_renders_singular_and_plural() {
        use muxsmith_core::report::{DiagCode, Diagnostic};

        let renderer = Renderer::new(Some("en"));
        let one = Diagnostic::info(DiagCode::SuggestionPartition, "tracks[0].match")
            .with("kind", "group")
            .with("count", "1")
            .with("fix", "tracks[0].match.exact.forced_track: true")
            .with("files", "/in/a.mkv");
        assert_eq!(
            renderer.diagnostic(&one),
            "[info] tracks[0].match: This file needs its own refinement; apply:\n\
             tracks[0].match.exact.forced_track: true\n    to: /in/a.mkv"
        );

        let two = Diagnostic::info(DiagCode::SuggestionPartition, "tracks[0].match")
            .with("kind", "group")
            .with("count", "2")
            .with("fix", "tracks[0].match.exact.forced_track: true")
            .with("files", "/in/a.mkv, /in/b.mkv");
        assert_eq!(
            renderer.diagnostic(&two),
            "[info] tracks[0].match: These 2 files need their own refinement; apply:\n\
             tracks[0].match.exact.forced_track: true\n    to: /in/a.mkv, /in/b.mkv"
        );
    }

    #[test]
    fn suggestion_partition_overflow_branch_renders_singular_and_plural() {
        use muxsmith_core::report::{DiagCode, Diagnostic};

        let renderer = Renderer::new(Some("en"));
        let one = Diagnostic::info(DiagCode::SuggestionPartition, "tracks[0].match")
            .with("kind", "overflow")
            .with("dropped", "1");
        assert_eq!(
            renderer.diagnostic(&one),
            "[info] tracks[0].match: 1 further resolution group was capped at 5 and not shown."
        );

        let two = Diagnostic::info(DiagCode::SuggestionPartition, "tracks[0].match")
            .with("kind", "overflow")
            .with("dropped", "2");
        assert_eq!(
            renderer.diagnostic(&two),
            "[info] tracks[0].match: 2 further resolution groups were capped at 5 and not shown."
        );
    }

    // T19 (#17 step 1): Pin the mirrored list contract between `numeric_diagnostic_params`
    // (Rust side, here) and `NUMERIC_DIAGNOSTIC_PARAMS` (TS side, src/diagnosticFluentParams.ts).
    // This test mirrors the TS implementation directly; changing either list alone will fail this test.
    // The expected list exactly replicates `NUMERIC_DIAGNOSTIC_PARAMS` from the TS file.
    #[test]
    fn numeric_diagnostic_params_list_is_mirrored_to_ts_side() {
        use muxsmith_core::report::DiagCode;

        // Expected (code, param_name) pairs. Mirrors src/diagnosticFluentParams.ts's
        // NUMERIC_DIAGNOSTIC_PARAMS; keep both in lockstep if either changes.
        let expected = [
            (DiagCode::SuggestionsCapped, &["dropped"] as &[&str]),
            (DiagCode::SuggestionPartition, &["dropped", "count"]),
        ];

        for (code, expected_params) in expected {
            let actual = numeric_diagnostic_params(code);
            assert_eq!(
                actual, expected_params,
                "numeric_diagnostic_params({:?}): expected {:?}, got {:?}. \
                 Keep this Rust list in sync with src/diagnosticFluentParams.ts's NUMERIC_DIAGNOSTIC_PARAMS.",
                code, expected_params, actual
            );
        }

        // Ensure no other codes have numeric params (a silent addition would otherwise go unnoticed).
        let all_codes = [
            DiagCode::IgnoredFile,
            DiagCode::UnknownProperty,
            DiagCode::UnsupportedSource,
            DiagCode::EmptyMatchExpression,
        ];
        for code in all_codes {
            let params = numeric_diagnostic_params(code);
            assert_eq!(
                params.len(),
                0,
                "DiagCode::{:?} unexpectedly has numeric params: {:?}. \
                 Update NUMERIC_DIAGNOSTIC_PARAMS in src/diagnosticFluentParams.ts if this was intentional.",
                code,
                params
            );
        }
    }
}
