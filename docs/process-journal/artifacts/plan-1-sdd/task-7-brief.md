### Task 7: Template engine

**Files:**
- Create: `crates/muxsmith-core/src/template.rs`
- Modify: `crates/muxsmith-core/src/lib.rs`
- Modify: `crates/muxsmith-core/Cargo.toml` (add regex)

**Interfaces:**
- Consumes: nothing.
- Produces (used by Tasks 9 and Plans 2-3):
  - `template::Filter { Raw, Int, Pad2, Pad3 }`
  - `template::Template` with `fn parse(text: &str) -> Result<Template, TemplateError>`, `fn field_names(&self) -> Vec<&str>`, `fn render_literal(&self, ctx: &Ctx) -> String`, `fn render_regex_pattern(&self, ctx: &Ctx, case_sensitive: bool) -> String`
  - `template::Ctx` wrapping `BTreeMap<String, String>` with `Ctx::new()`, `.set(name, value)`; missing fields render as empty string (validation prevents that from being reachable)
  - `template::TemplateError { UnclosedBrace { pos: usize }, EmptyField { pos: usize }, UnknownFilter { name: String } }`
  - Escapes: `{{` and `}}` are literal braces.

- [ ] **Step 1: Add regex dependency**

```bash
cargo add -p muxsmith-core regex
```

- [ ] **Step 2: Write the failing tests**

Tests at the bottom of the new `crates/muxsmith-core/src/template.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(pairs: &[(&str, &str)]) -> Ctx {
        let mut c = Ctx::new();
        for (k, v) in pairs {
            c.set(k, v);
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
        assert!(t.render_regex_pattern(&Ctx::new(), false).starts_with("(?i)"));
        assert!(!t.render_regex_pattern(&Ctx::new(), true).starts_with("(?i)"));
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core template`
Expected: FAIL

- [ ] **Step 4: Implement**

`crates/muxsmith-core/src/template.rs` above the tests:

```rust
//! Template engine (spec 4.7). One parser, two render modes:
//! literal (output filenames, title) and regex pattern (external
//! locator match_pattern; interpolated values are regex-escaped).

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Raw,
    Int,
    Pad2,
    Pad3,
}

#[derive(Debug, Clone, PartialEq)]
enum Segment {
    Literal(String),
    Field { name: String, filter: Filter },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateError {
    UnclosedBrace { pos: usize },
    EmptyField { pos: usize },
    UnknownFilter { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    segments: Vec<Segment>,
}

#[derive(Debug, Default)]
pub struct Ctx {
    values: BTreeMap<String, String>,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx::default()
    }

    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }

    fn get(&self, name: &str) -> &str {
        self.values.get(name).map(String::as_str).unwrap_or("")
    }
}

impl Template {
    pub fn parse(text: &str) -> Result<Template, TemplateError> {
        let mut segments = Vec::new();
        let mut literal = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '{' if chars.get(i + 1) == Some(&'{') => {
                    literal.push('{');
                    i += 2;
                }
                '}' if chars.get(i + 1) == Some(&'}') => {
                    literal.push('}');
                    i += 2;
                }
                '{' => {
                    let close = chars[i + 1..]
                        .iter()
                        .position(|&c| c == '}')
                        .ok_or(TemplateError::UnclosedBrace { pos: i })?;
                    let inner: String = chars[i + 1..i + 1 + close].iter().collect();
                    if inner.is_empty() {
                        return Err(TemplateError::EmptyField { pos: i });
                    }
                    let (name, filter) = match inner.split_once(':') {
                        None => (inner.as_str(), Filter::Raw),
                        Some((n, "int")) => (n, Filter::Int),
                        Some((n, "pad2")) => (n, Filter::Pad2),
                        Some((n, "pad3")) => (n, Filter::Pad3),
                        Some((_, f)) => {
                            return Err(TemplateError::UnknownFilter { name: f.to_string() })
                        }
                    };
                    if name.is_empty() {
                        return Err(TemplateError::EmptyField { pos: i });
                    }
                    if !literal.is_empty() {
                        segments.push(Segment::Literal(std::mem::take(&mut literal)));
                    }
                    segments.push(Segment::Field {
                        name: name.to_string(),
                        filter,
                    });
                    i += close + 2;
                }
                c => {
                    literal.push(c);
                    i += 1;
                }
            }
        }
        if !literal.is_empty() {
            segments.push(Segment::Literal(literal));
        }
        Ok(Template { segments })
    }

    pub fn field_names(&self) -> Vec<&str> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                Segment::Field { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

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
```

Add `pub mod template;` to `crates/muxsmith-core/src/lib.rs`.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core template`
Expected: PASS (8 tests)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(core): template engine with int/pad filters and dual render modes"
```

---

