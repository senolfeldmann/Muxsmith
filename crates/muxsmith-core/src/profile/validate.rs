//! Semantic validation (spec 5.4 static checks, config-time part).
//! Task 9 extends this file with input/locator/template validation.

use std::collections::BTreeMap;
use std::path::Path;

use crate::capability::{self, PropType};
use crate::report::{DiagCode, Diagnostic};
use crate::template::{Template, TemplateError};

use super::match_expr::{MatchExpr, Scalar};
use super::model::{
    AttachmentRule, ChaptersCfg, FilenameCfg, KeepDrop, Locator, Profile, SourceCfg, TitleCfg,
};
use super::{lint, load};

/// Config-time semantic validation (spec 5.4): profile version, regex
/// compilation, template well-formedness, property existence/type checks
/// for every `match`/`changes`/locator, and keyword validity. Returns every
/// diagnostic found rather than stopping at the first (no fail-fast);
/// touches no filesystem beyond the profile itself and never replaces the
/// planner's runtime checks (5.1-5.3).
pub fn validate(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if profile.profile_version != 1 {
        diags.push(
            Diagnostic::error(DiagCode::UnsupportedProfileVersion, "profile_version")
                .with("found", profile.profile_version.to_string())
                .with("supported", "1"),
        );
    }

    // input.pattern must compile; its groups define the template fields.
    let mut template_fields: Vec<String> = vec!["match".into()];
    match regex::Regex::new(&profile.input.pattern) {
        Err(e) => diags.push(
            Diagnostic::error(DiagCode::InvalidRegex, "input.pattern")
                .with("detail", flatten_regex_error(&e)),
        ),
        Ok(re) => {
            for (i, name) in re.capture_names().enumerate() {
                if i == 0 {
                    continue; // group 0 is the whole match
                }
                template_fields.push(format!("g{i}"));
                if let Some(n) = name {
                    template_fields.push(n.to_string());
                }
            }
        }
    }

    if profile.input.extensions.is_empty() {
        diags.push(Diagnostic::error(
            DiagCode::EmptyExtensions,
            "input.extensions",
        ));
    }

    if profile.tracks.rules.is_empty() {
        match profile.tracks.unmatched {
            // Discarding everything with no rule selecting anything is a
            // profile that can never produce output.
            KeepDrop::Drop => {
                diags.push(Diagnostic::error(DiagCode::NoTrackRules, "tracks.rules"));
            }
            // Legal pure passthrough (D38): announce it so an accidental
            // delete-all-rules edit stays visible.
            KeepDrop::Keep => diags.push(Diagnostic::info(
                DiagCode::PassthroughProfile,
                "tracks.rules",
            )),
        }
    }

    for (i, rule) in profile.tracks.rules.iter().enumerate() {
        let base = format!("tracks[{i}]");
        if rule.match_expr.is_empty() {
            // Suppress the generic EmptyMatchExpression when the emptiness is
            // caused by an empty top-level `any`/`not` list, which already
            // gets its own, more specific EmptyMatchList error for the same
            // node (validate_expr below). Otherwise a `{ any: [] }` match
            // double-reports.
            let empty_list_here = rule.match_expr.any.as_ref().is_some_and(|v| v.is_empty())
                || rule.match_expr.not.as_ref().is_some_and(|v| v.is_empty());
            if !empty_list_here {
                diags.push(Diagnostic::warning(
                    DiagCode::EmptyMatchExpression,
                    format!("{base}.match"),
                ));
            }
        }
        validate_expr(
            &rule.match_expr,
            &format!("{base}.match"),
            track_prop_type,
            &mut diags,
        );
        if let Some(changes) = &rule.changes {
            validate_changes(changes, &format!("{base}.changes"), &mut diags);
        }

        match &rule.source {
            SourceCfg::Keyword(k) if SourceCfg::KEYWORDS.contains(&k.as_str()) => {}
            SourceCfg::Keyword(k) => diags.push(
                Diagnostic::error(DiagCode::InvalidKeyword, format!("{base}.source"))
                    .with("found", k.clone())
                    .with("allowed", domain_hint(SourceCfg::KEYWORDS)),
            ),
            SourceCfg::External(block) => {
                validate_locator(
                    &block.external,
                    &format!("{base}.source.external"),
                    &template_fields,
                    &mut diags,
                );
            }
        }
    }

    for (i, rule) in profile.attachments.rules.iter().enumerate() {
        let base = format!("attachments.rules[{i}]");
        validate_attachment_rule(rule, &base, &template_fields, &mut diags);
    }

    // output.filename
    match &profile.output.filename {
        FilenameCfg::Keyword(k) if FilenameCfg::KEYWORDS.contains(&k.as_str()) => {}
        FilenameCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "output.filename")
                .with("found", k.clone())
                .with("allowed", domain_hint(FilenameCfg::KEYWORDS)),
        ),
        FilenameCfg::Template(block) => {
            let mut fields = template_fields.clone();
            fields.push("source_stem".into());
            validate_template(
                &block.template,
                "output.filename.template",
                &fields,
                true,
                &mut diags,
            );
        }
    }

    match &profile.chapters {
        ChaptersCfg::Keyword(k) if ChaptersCfg::KEYWORDS.contains(&k.as_str()) => {}
        ChaptersCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "chapters")
                .with("found", k.clone())
                .with("allowed", domain_hint(ChaptersCfg::KEYWORDS)),
        ),
        ChaptersCfg::External(block) => {
            validate_locator(
                &block.external,
                "chapters.external",
                &template_fields,
                &mut diags,
            );
        }
    }

    match &profile.title {
        TitleCfg::Keyword(k) if TitleCfg::KEYWORDS.contains(&k.as_str()) => {}
        TitleCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "title")
                .with("found", k.clone())
                .with("allowed", domain_hint(TitleCfg::KEYWORDS)),
        ),
        TitleCfg::Template(block) => {
            let mut fields = template_fields.clone();
            fields.push("source_stem".into());
            validate_template(
                &block.template,
                "title.template",
                &fields,
                false,
                &mut diags,
            );
        }
    }

    diags
}

/// Every config-time diagnostic for an already-loaded `profile`: [`validate`]'s
/// static checks followed by [`lint::provable_overlaps`]'s cross-rule overlap
/// check, in that order. The one funnel both config-time consumers (CLI
/// `validate`, GUI `validate_profile`) call instead of each repeating the
/// two-call sequence.
pub fn config_diagnostics(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = validate(profile);
    diags.extend(lint::provable_overlaps(profile));
    diags
}

/// [`config_diagnostics`] for a profile not yet loaded from `path`: a load
/// failure (spec 4 I/O or deserialization error) short-circuits to that
/// single [`Diagnostic`] since there is no profile to run the funnel on; a
/// successful load runs [`config_diagnostics`] on it.
pub fn config_diagnostics_from_file(path: &Path) -> Vec<Diagnostic> {
    match load::from_file(path) {
        Err(d) => vec![d],
        Ok(profile) => config_diagnostics(&profile),
    }
}

fn flatten_regex_error(e: &regex::Error) -> String {
    e.to_string()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn track_prop_type(name: &str) -> Option<PropType> {
    capability::matchable_type(name)
}

fn attachment_prop_type(name: &str) -> Option<PropType> {
    capability::ATTACHMENT_PROPERTIES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, t)| *t)
}

fn validate_attachment_rule(
    rule: &AttachmentRule,
    base: &str,
    template_fields: &[String],
    diags: &mut Vec<Diagnostic>,
) {
    let actions = [
        rule.select.is_some(),
        rule.drop.is_some(),
        rule.add.is_some(),
    ]
    .iter()
    .filter(|b| **b)
    .count();
    if actions != 1 {
        diags.push(
            Diagnostic::error(DiagCode::AttachmentRuleShape, base.to_string())
                .with("found", actions.to_string()),
        );
    }
    if let Some(expr) = &rule.select {
        validate_expr(expr, &format!("{base}.select"), attachment_prop_type, diags);
    }
    if let Some(expr) = &rule.drop {
        validate_expr(expr, &format!("{base}.drop"), attachment_prop_type, diags);
    }
    if let Some(locator) = &rule.add {
        validate_locator(locator, &format!("{base}.add"), template_fields, diags);
    }
}

fn validate_expr(
    expr: &MatchExpr,
    path: &str,
    prop_type: fn(&str) -> Option<PropType>,
    diags: &mut Vec<Diagnostic>,
) {
    if let Some(exact) = &expr.exact {
        for (prop, value) in exact {
            let p = format!("{path}.exact.{prop}");
            if let Some(bare) = prop.strip_prefix("raw:") {
                // Explicit `raw:` opt-in (D32, spec 9.2): bypass the
                // existence/type/domain checks and match untyped. The bare
                // name is not looked up in the capability model, so no
                // UnknownProperty / ValueTypeMismatch / InvalidPropertyValue.
                diags.push(raw_opt_in_diagnostic(&p, bare));
                continue;
            }
            match prop_type(prop) {
                None => diags.push(unknown_property(&p, prop)),
                Some(t) => {
                    if !scalar_fits(value, t) {
                        diags.push(
                            Diagnostic::error(DiagCode::ValueTypeMismatch, p.clone())
                                .with("property", prop.clone())
                                .with("expected", type_label(t))
                                .with("found", value.type_name()),
                        );
                    } else if let (Scalar::Str(s), Some(domain)) =
                        (value, capability::matchable_domain(prop))
                    {
                        // Closed-domain string property (type, codec_kind): the
                        // value must be a domain member (D2). language is
                        // deliberately absent from the domain map (validated at
                        // plan time), so this never fires for it.
                        if !domain.contains(&s.as_str()) {
                            diags.push(
                                Diagnostic::error(DiagCode::InvalidPropertyValue, p.clone())
                                    .with("property", prop.clone())
                                    .with("value", s.clone())
                                    .with("allowed", domain_hint(domain)),
                            );
                        }
                    }
                }
            }
        }
    }
    for (map, kind) in [(&expr.substring, "substring"), (&expr.regex, "regex")] {
        if let Some(map) = map {
            for (prop, value) in map.iter() {
                let p = format!("{path}.{kind}.{prop}");
                if let Some(bare) = prop.strip_prefix("raw:") {
                    // `raw:` opt-in (D32): bypass property existence/type
                    // checks (including the codec_kind exact-only guard, which
                    // raw: sidesteps entirely). A value-level regex-compile
                    // error is still reported below: an uncompilable pattern
                    // is a config error independent of the property.
                    diags.push(raw_opt_in_diagnostic(&p, bare));
                } else if prop == "codec_kind" && prop_type(prop).is_some() {
                    // codec_kind is a curated alias, matchable only under exact
                    // (D1). Guard before the string-type check so it reports
                    // CodecKindExactOnly rather than the misleading (codec_kind is
                    // String-typed) success. Only fires where codec_kind is a known
                    // property of this context (track rules, not attachments).
                    diags.push(
                        Diagnostic::error(DiagCode::CodecKindExactOnly, p.clone())
                            .with("condition", kind.to_string()),
                    );
                } else {
                    match prop_type(prop) {
                        None => diags.push(unknown_property(&p, prop)),
                        Some(PropType::String) => {}
                        Some(t) => diags.push(
                            Diagnostic::error(DiagCode::NotStringProperty, p.clone())
                                .with("property", prop.clone())
                                .with("actual_type", type_label(t))
                                .with("condition", kind.to_string()),
                        ),
                    }
                }
                if kind == "regex"
                    && let Err(e) = regex::Regex::new(value)
                {
                    diags.push(
                        Diagnostic::error(DiagCode::InvalidRegex, p)
                            .with("detail", flatten_regex_error(&e)),
                    );
                }
            }
        }
    }
    if let Some(any) = &expr.any {
        if any.is_empty() {
            diags.push(Diagnostic::error(
                DiagCode::EmptyMatchList,
                format!("{path}.any"),
            ));
        }
        for (i, sub) in any.iter().enumerate() {
            validate_expr(sub, &format!("{path}.any[{i}]"), prop_type, diags);
        }
    }
    if let Some(not) = &expr.not {
        if not.is_empty() {
            diags.push(Diagnostic::error(
                DiagCode::EmptyMatchList,
                format!("{path}.not"),
            ));
        }
        for (i, sub) in not.iter().enumerate() {
            validate_expr(sub, &format!("{path}.not[{i}]"), prop_type, diags);
        }
    }
}

fn validate_changes(changes: &BTreeMap<String, Scalar>, path: &str, diags: &mut Vec<Diagnostic>) {
    for (prop, value) in changes {
        let p = format!("{path}.{prop}");
        match capability::settable(prop) {
            None => diags.push(
                Diagnostic::error(DiagCode::UnknownSettableProperty, p)
                    .with("property", prop.clone()),
            ),
            Some((t, _option)) => {
                if !scalar_fits(value, t) {
                    diags.push(
                        Diagnostic::error(DiagCode::ValueTypeMismatch, p)
                            .with("property", prop.clone())
                            .with("expected", type_label(t))
                            .with("found", value.type_name()),
                    );
                }
            }
        }
    }
}

fn unknown_property(path: &str, prop: &str) -> Diagnostic {
    Diagnostic::error(DiagCode::UnknownProperty, path.to_string()).with("property", prop)
}

/// The config-time diagnostic for a `raw:`-prefixed match property (D32, spec
/// 9.2). `EmptyRawProperty` (error, D101) when the bare name is empty: a bare
/// `raw:` names no property, so the rule could never match, and the
/// diagnostic carries no `property` param - an empty-string one would render
/// as visible nothing. `RawOnKnownProperty` (warning) when the bare name is
/// one of the two capability properties with special matching semantics -
/// `language` (ISO-639/BCP-47 normalization) and `codec_kind` (alias
/// expansion), exactly the arms `matcher::exact_matches` special-cases -
/// which `raw:` degrades to byte-literal equality; otherwise `RawProperty`
/// (info), the visible escape valve announcing the untyped bypass. `path`
/// keeps the literal `raw:`-prefixed key; the `property` param carries the
/// stripped bare name.
fn raw_opt_in_diagnostic(path: &str, bare: &str) -> Diagnostic {
    if bare.is_empty() {
        Diagnostic::error(DiagCode::EmptyRawProperty, path.to_string())
    } else if matches!(bare, "language" | "codec_kind") {
        Diagnostic::warning(DiagCode::RawOnKnownProperty, path.to_string()).with("property", bare)
    } else {
        Diagnostic::info(DiagCode::RawProperty, path.to_string()).with("property", bare)
    }
}

fn scalar_fits(value: &Scalar, t: PropType) -> bool {
    matches!(
        (value, t),
        (Scalar::Str(_), PropType::String)
            | (Scalar::Bool(_), PropType::Boolean)
            | (Scalar::Int(_), PropType::Integer)
            | (Scalar::Int(_), PropType::Float)
            | (Scalar::Float(_), PropType::Float)
    )
}

/// A short, deterministic sample of a closed value domain for the
/// `InvalidPropertyValue` hint: the full list if small, else the first few
/// plus an ellipsis marker. Keeps the message bounded for large domains.
fn domain_hint(domain: &[&str]) -> String {
    const MAX: usize = 8;
    if domain.len() <= MAX {
        domain.join(", ")
    } else {
        format!("{}, ...", domain[..MAX].join(", "))
    }
}

fn type_label(t: PropType) -> &'static str {
    match t {
        PropType::String => "string",
        PropType::Boolean => "boolean",
        PropType::Integer => "integer",
        PropType::Float => "float",
    }
}

fn validate_locator(
    locator: &Locator,
    path: &str,
    template_fields: &[String],
    diags: &mut Vec<Diagnostic>,
) {
    if locator.extensions.is_empty() {
        diags.push(Diagnostic::error(
            DiagCode::EmptyExtensions,
            format!("{path}.extensions"),
        ));
    }
    if matches!(locator.match_to_source, Some(true)) && locator.match_pattern.is_some() {
        diags.push(Diagnostic::error(
            DiagCode::LocatorConflict,
            path.to_string(),
        ));
    }
    if locator.match_to_source == Some(false) {
        diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, format!("{path}.match_to_source"))
                .with("found", "false")
                .with("allowed", "true"),
        );
    }
    if let Some(pattern) = &locator.match_pattern {
        // source_stem is literal-mode only: template_fields never contains it
        // here, so a match_pattern using {source_stem} is UnknownTemplateField
        // (spec 4.7).
        validate_template(
            pattern,
            &format!("{path}.match_pattern"),
            template_fields,
            false,
            diags,
        );
    }
}

fn validate_template(
    text: &str,
    path: &str,
    allowed_fields: &[String],
    forbid_path_separators: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let template = match Template::parse(text) {
        Ok(t) => t,
        Err(e) => {
            let diag = match e {
                TemplateError::UnknownFilter { name } => {
                    Diagnostic::error(DiagCode::UnknownTemplateFilter, path.to_string())
                        .with("name", name)
                }
                TemplateError::UnclosedBrace { pos } => {
                    Diagnostic::error(DiagCode::InvalidTemplate, path.to_string())
                        .with("kind", "unclosed-brace")
                        .with("pos", pos.to_string())
                }
                TemplateError::EmptyField { pos } => {
                    Diagnostic::error(DiagCode::InvalidTemplate, path.to_string())
                        .with("kind", "empty-field")
                        .with("pos", pos.to_string())
                }
            };
            diags.push(diag);
            return;
        }
    };
    for field in template.field_names() {
        if !allowed_fields.iter().any(|f| f == field) {
            diags.push(
                Diagnostic::error(DiagCode::UnknownTemplateField, path.to_string())
                    .with("field", field)
                    .with("allowed", allowed_fields.join(", ")),
            );
        }
    }
    if forbid_path_separators && (text.contains('/') || text.contains('\\')) {
        diags.push(Diagnostic::error(
            DiagCode::PathSeparatorInTemplate,
            path.to_string(),
        ));
    }
}
