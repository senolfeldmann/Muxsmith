//! Template engine (spec 4.7). One parser, two render modes:
//! literal (output filenames, title) and regex pattern (external
//! locator match_pattern; interpolated values are regex-escaped).

use std::collections::BTreeMap;

/// Rendering filter applied to a template field's value (spec 4.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    /// No transformation; the captured value as-is (e.g. `03`).
    Raw,
    /// Strips leading zeros (`03` -> `3`); a non-empty all-zero value
    /// collapses to `"0"`, and a missing field still renders empty (never
    /// `"0"` for an absent value).
    Int,
    /// Zero-pads to at least 2 characters (`3` -> `03`).
    Pad2,
    /// Zero-pads to at least 3 characters (`3` -> `003`).
    Pad3,
}

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    Literal(String),
    Field { name: String, filter: Filter },
}

/// Parse errors. `pos` is a CHARACTER offset (index into the template's
/// `chars()` sequence), not a byte offset: do not byte-slice the template
/// string with it.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateError {
    /// A `{` was opened but never closed before the template's end.
    UnclosedBrace {
        /// Character offset of the opening `{` (see the `pos` contract above).
        pos: usize,
    },
    /// A field had no name, with or without a filter (e.g. `{}` or `{:int}`).
    EmptyField {
        /// Character offset of the opening `{` of the empty field.
        pos: usize,
    },
    /// A field used a filter name other than `int`, `pad2`, or `pad3`.
    UnknownFilter {
        /// The unrecognized filter name as written (without the `:`).
        name: String,
    },
}

/// A parsed template: literal segments and field references (spec 4.7).
/// Produced once by [`Template::parse`], rendered any number of times via
/// [`Template::render_literal`] or [`Template::render_regex_pattern`].
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    segments: Vec<Segment>,
}

/// Field values available to a template render (spec 4.7): `{match}`,
/// named/numbered capture groups, `{source_stem}`. A field with no
/// corresponding `set` call renders as empty string rather than erroring.
#[derive(Debug, Default)]
pub struct Ctx {
    values: BTreeMap<String, String>,
}

impl Ctx {
    /// An empty context; equivalent to [`Ctx::default`].
    pub fn new() -> Self {
        Ctx::default()
    }

    /// Binds `name` to `value` for later field interpolation; a repeated
    /// `set` for the same name overwrites the previous value.
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }

    fn get(&self, name: &str) -> &str {
        self.values.get(name).map(String::as_str).unwrap_or("")
    }
}

impl Template {
    /// Parses template source into a reusable [`Template`] (spec 4.7).
    /// `{{`/`}}` are literal braces; `{name}` and `{name:filter}` are
    /// fields, where `filter` must be one of `int`/`pad2`/`pad3` (default
    /// `Raw` if omitted). Fails on an unclosed `{`, an empty field name, or
    /// an unrecognized filter.
    pub fn parse(text: &str) -> Result<Template, TemplateError> {
        let mut segments = Vec::new();
        let mut literal = String::new();
        let mut chars = text.chars().peekable();
        let mut pos = 0usize;
        while let Some(c) = chars.next() {
            let start = pos;
            pos += 1;
            match c {
                '{' if chars.peek() == Some(&'{') => {
                    chars.next();
                    pos += 1;
                    literal.push('{');
                }
                '}' if chars.peek() == Some(&'}') => {
                    chars.next();
                    pos += 1;
                    literal.push('}');
                }
                '{' => {
                    let mut inner = String::new();
                    let mut closed = false;
                    for next in chars.by_ref() {
                        pos += 1;
                        if next == '}' {
                            closed = true;
                            break;
                        }
                        inner.push(next);
                    }
                    if !closed {
                        return Err(TemplateError::UnclosedBrace { pos: start });
                    }
                    if inner.is_empty() {
                        return Err(TemplateError::EmptyField { pos: start });
                    }
                    // Split name from filter BEFORE resolving the filter, so an
                    // empty name reports EmptyField even when a filter is present.
                    let (name, maybe_filter) = match inner.split_once(':') {
                        None => (inner.as_str(), None),
                        Some((n, f)) => (n, Some(f)),
                    };
                    if name.is_empty() {
                        return Err(TemplateError::EmptyField { pos: start });
                    }
                    let filter = match maybe_filter {
                        None => Filter::Raw,
                        Some("int") => Filter::Int,
                        Some("pad2") => Filter::Pad2,
                        Some("pad3") => Filter::Pad3,
                        Some(f) => {
                            return Err(TemplateError::UnknownFilter {
                                name: f.to_string(),
                            });
                        }
                    };
                    if !literal.is_empty() {
                        segments.push(Segment::Literal(std::mem::take(&mut literal)));
                    }
                    segments.push(Segment::Field {
                        name: name.to_string(),
                        filter,
                    });
                }
                c => {
                    literal.push(c);
                }
            }
        }
        if !literal.is_empty() {
            segments.push(Segment::Literal(literal));
        }
        Ok(Template { segments })
    }

    /// Every field name referenced by the template, in order of
    /// appearance (duplicates included). `validate.rs` checks each against
    /// the allowed field set for the template's context, emitting
    /// `UnknownTemplateField` for anything not in that set.
    pub fn field_names(&self) -> Vec<&str> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                Segment::Field { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Literal-mode render (spec 4.7): field values interpolate as plain
    /// strings, no escaping. Used for output filenames and titles.
    pub fn render_literal(&self, ctx: &Ctx) -> String {
        self.render(ctx, false)
    }

    /// Render as a regex pattern: literal segments pass through as regex
    /// source, field values are escaped. Prefixes (?i) unless case_sensitive.
    pub fn render_regex_pattern(&self, ctx: &Ctx, case_sensitive: bool) -> String {
        let body = self.render(ctx, true);
        if case_sensitive {
            body
        } else {
            format!("(?i){body}")
        }
    }

    fn render(&self, ctx: &Ctx, escape_fields: bool) -> String {
        let mut out = String::new();
        for seg in &self.segments {
            match seg {
                Segment::Literal(l) => out.push_str(l),
                Segment::Field { name, filter } => {
                    let v = apply_filter(ctx.get(name), *filter);
                    if escape_fields {
                        out.push_str(&regex::escape(&v));
                    } else {
                        out.push_str(&v);
                    }
                }
            }
        }
        out
    }
}

fn apply_filter(value: &str, filter: Filter) -> String {
    match filter {
        Filter::Raw => value.to_string(),
        Filter::Int => {
            // Missing fields render as empty (Ctx contract); only non-empty
            // all-zero input collapses to "0".
            if value.is_empty() {
                return String::new();
            }
            let stripped = value.trim_start_matches('0');
            if stripped.is_empty() {
                "0".to_string()
            } else {
                stripped.to_string()
            }
        }
        Filter::Pad2 => format!("{value:0>2}"),
        Filter::Pad3 => format!("{value:0>3}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(pairs: &[(&str, &str)]) -> Ctx {
        let mut c = Ctx::new();
        for (k, v) in pairs {
            c.set(*k, *v);
        }
        c
    }

    #[test]
    fn parses_fields_and_filters() {
        let t = Template::parse("Show - S{season}E{episode:pad2}.mkv").unwrap();
        assert_eq!(t.field_names(), vec!["season", "episode"]);
    }

    #[test]
    fn rejects_unknown_filter_and_unclosed_brace() {
        assert!(matches!(
            Template::parse("{season:frobnicate}"),
            Err(TemplateError::UnknownFilter { .. })
        ));
        assert!(matches!(
            Template::parse("S{season"),
            Err(TemplateError::UnclosedBrace { .. })
        ));
        assert!(matches!(
            Template::parse("S{}"),
            Err(TemplateError::EmptyField { .. })
        ));
        assert!(matches!(
            Template::parse("{x:}"),
            Err(TemplateError::UnknownFilter { .. })
        ));
    }

    #[test]
    fn empty_name_with_filter_is_empty_field() {
        assert!(matches!(
            Template::parse("{:int}"),
            Err(TemplateError::EmptyField { .. })
        ));
        assert!(matches!(
            Template::parse("{:}"),
            Err(TemplateError::EmptyField { .. })
        ));
    }

    #[test]
    fn int_filter_on_missing_field_renders_empty() {
        let t = Template::parse("{n:int}").unwrap();
        assert_eq!(t.render_literal(&Ctx::new()), "");
    }

    #[test]
    fn renders_literal_with_filters() {
        let t = Template::parse("S{season:int}E{episode:pad3} of {show}").unwrap();
        let c = ctx(&[("season", "03"), ("episode", "1"), ("show", "X")]);
        assert_eq!(t.render_literal(&c), "S3E001 of X");
    }

    #[test]
    fn int_filter_keeps_single_zero() {
        let t = Template::parse("{n:int}").unwrap();
        assert_eq!(t.render_literal(&ctx(&[("n", "000")])), "0");
    }

    #[test]
    fn lone_closing_brace_is_literal_and_lone_open_is_error() {
        // A lone `}` cannot start a field, so it is a literal (D5).
        let t = Template::parse("a}b").expect("lone } is literal");
        assert_eq!(t.render_literal(&Ctx::new()), "a}b");
        // A lone unclosed `{` is a hard error (ambiguous, user mistake).
        assert!(matches!(
            Template::parse("a{b"),
            Err(TemplateError::UnclosedBrace { .. })
        ));
    }

    #[test]
    fn double_braces_are_literal() {
        let t = Template::parse("a{{b}}c").unwrap();
        assert_eq!(t.render_literal(&Ctx::new()), "a{b}c");
    }

    #[test]
    fn regex_mode_matches_spec_examples() {
        // Spec 4.7: primary matched as S03E01, targets use staffel naming.
        let t = Template::parse("staffel0*{season:int}episode0*{episode:int}").unwrap();
        let c = ctx(&[("season", "03"), ("episode", "01")]);
        let pattern = t.render_regex_pattern(&c, false);
        let re = regex::Regex::new(&pattern).unwrap();
        assert!(re.is_match("staffel03episode01"));
        assert!(re.is_match("staffel3episode01"));
        assert!(re.is_match("Staffel3Episode1"));
        assert!(!re.is_match("staffel4episode01"));
    }

    #[test]
    fn regex_mode_escapes_interpolated_values() {
        let t = Template::parse("{m}").unwrap();
        let c = ctx(&[("m", "a.b(c)")]);
        let pattern = t.render_regex_pattern(&c, true);
        let re = regex::Regex::new(&pattern).unwrap();
        assert!(re.is_match("xa.b(c)y"));
        assert!(!re.is_match("aXb(c)"));
    }

    #[test]
    fn case_sensitive_flag_controls_inline_i() {
        let t = Template::parse("abc").unwrap();
        assert!(
            t.render_regex_pattern(&Ctx::new(), false)
                .starts_with("(?i)")
        );
        assert!(
            !t.render_regex_pattern(&Ctx::new(), true)
                .starts_with("(?i)")
        );
    }
}
