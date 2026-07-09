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
        let Some(message) = self.bundle.get_message(id) else {
            // Missing catalog entry: fall back to the raw id so the
            // problem is visible instead of hidden. CI guards this case.
            return id.to_string();
        };
        let Some(pattern) = message.value() else {
            return id.to_string();
        };
        let mut fargs = FluentArgs::new();
        for (k, v) in args {
            fargs.set(*k, *v);
        }
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
        let params: Vec<(&str, &str)> = d
            .params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let message = self.msg(d.code.key(), &params);
        let severity = self.msg(severity_key(d.severity), &[]);
        match &d.file {
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
}

fn severity_key(s: muxsmith_core::report::Severity) -> &'static str {
    match s {
        muxsmith_core::report::Severity::Error => "severity-error",
        muxsmith_core::report::Severity::Warning => "severity-warning",
        muxsmith_core::report::Severity::Info => "severity-info",
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
    fn unknown_property_skew_uses_only_the_supplied_version_param() {
        let renderer = Renderer::new(Some("en"));
        let rendered = renderer.msg("unknown-property-skew", &[("version", "42")]);
        assert!(rendered.contains("42"), "expected version in: {rendered}");
        assert!(
            !rendered.contains("{$property}") && !rendered.contains("$property"),
            "unresolved property placeholder leaked into: {rendered}"
        );
    }
}
