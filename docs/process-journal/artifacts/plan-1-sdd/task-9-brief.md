### Task 9: Input, locator, keyword and template validation

**Files:**
- Modify: `crates/muxsmith-core/src/profile/validate.rs`
- Test: `crates/muxsmith-core/tests/validate_structure.rs`

**Interfaces:**
- Consumes: everything from Task 8 (extends the same `validate` function), `template::Template` (Task 7).
- Produces: complete config-time validation. After this task, `validate` covers every check in spec 5.4's config-time scope except the overlap lint (Task 10).

- [ ] **Step 1: Write the failing tests**

`crates/muxsmith-core/tests/validate_structure.rs`:

```rust
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::DiagCode;

fn parse(y: &str) -> muxsmith_core::profile::Profile {
    from_str(y, Format::Yaml).unwrap()
}

fn codes(y: &str) -> Vec<DiagCode> {
    validate(&parse(y)).into_iter().map(|d| d.code).collect()
}

const BASE: &str = r#"
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
"#;

#[test]
fn invalid_input_pattern_is_flagged() {
    let y = BASE.replace(r"S(?<season>\d{2})E(?<episode>\d{2})", "([");
    assert!(codes(&y).contains(&DiagCode::InvalidRegex));
}

#[test]
fn empty_extensions_flagged_for_input_and_locator() {
    let y = BASE.replace("extensions: [mkv]", "extensions: []");
    assert!(codes(&y).contains(&DiagCode::EmptyExtensions));
}

#[test]
fn locator_with_both_match_options_is_conflict() {
    let y = format!(
        "{BASE}  - source:\n      external: {{ path: '.', extensions: [srt], match_to_source: true, match_pattern: '{{match}}' }}\n    match: {{ exact: {{ type: subtitles }} }}\n"
    );
    assert!(codes(&y).contains(&DiagCode::LocatorConflict));
}

#[test]
fn match_pattern_with_unknown_field_is_flagged() {
    let y = format!(
        "{BASE}  - source:\n      external: {{ path: '.', extensions: [srt], match_pattern: 'x{{volume}}y' }}\n    match: {{ exact: {{ type: subtitles }} }}\n"
    );
    let c = codes(&y);
    assert!(c.contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn match_pattern_may_not_use_source_stem() {
    // source_stem is literal-mode only (spec 4.7).
    let y = format!(
        "{BASE}  - source:\n      external: {{ path: '.', extensions: [srt], match_pattern: '{{source_stem}}' }}\n    match: {{ exact: {{ type: subtitles }} }}\n"
    );
    assert!(codes(&y).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn filename_template_fields_checked_against_pattern_groups() {
    let good =
        BASE.to_string() + "output:\n  filename: { template: 'X S{season}E{episode:pad2}.mkv' }\n";
    assert!(!codes(&good).contains(&DiagCode::UnknownTemplateField));

    let bad = BASE.to_string()
        + "output:\n  filename: { template: 'X {show}.mkv' }\n";
    assert!(codes(&bad).contains(&DiagCode::UnknownTemplateField));
}

#[test]
fn filename_template_with_path_separator_is_flagged() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'sub/dir{season}.mkv' }\n";
    assert!(codes(&y).contains(&DiagCode::PathSeparatorInTemplate));
}

#[test]
fn bad_template_syntax_is_invalid_template() {
    let y = BASE.to_string() + "output:\n  filename: { template: 'S{season' }\n";
    assert!(codes(&y).contains(&DiagCode::InvalidTemplate));
}

#[test]
fn unknown_keywords_are_flagged() {
    for (snippet, _section) in [
        ("chapters: discard\n", "chapters"),
        ("title: wipe\n", "title"),
    ] {
        let y = BASE.to_string() + snippet;
        assert!(
            codes(&y).contains(&DiagCode::InvalidKeyword),
            "expected InvalidKeyword for: {snippet}"
        );
    }
    let y = BASE.replace(
        "- match: { exact: { type: video } }",
        "- source: secondary\n    match: { exact: { type: video } }",
    );
    assert!(codes(&y).contains(&DiagCode::InvalidKeyword));
}

#[test]
fn numbered_group_fields_are_accepted() {
    let y = r#"
profile_version: 1
input: { pattern: 'S(\d{2})E(\d{2})', extensions: [mkv] }
output:
  filename: { template: 'S{g1}E{g2}.mkv' }
tracks:
  - match: { exact: { type: video } }
"#;
    assert!(!codes(y).contains(&DiagCode::UnknownTemplateField));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core --test validate_structure`
Expected: FAIL (new checks missing; some tests may pass incidentally, at least pattern/locator/template ones must fail)

- [ ] **Step 3: Extend `validate`**

Add to `crates/muxsmith-core/src/profile/validate.rs` (new code, integrated into the existing `validate` function; new imports at top):

```rust
use crate::template::{Template, TemplateError};

use super::model::{ChaptersCfg, FilenameCfg, Locator, SourceCfg, TitleCfg};
```

Inside `validate`, after the version check, add input validation and compute the allowed template fields:

```rust
    // input.pattern must compile; its groups define the template fields.
    let mut template_fields: Vec<String> = vec!["match".into()];
    match regex::Regex::new(&profile.input.pattern) {
        Err(e) => diags.push(
            Diagnostic::error(DiagCode::InvalidRegex, "input.pattern")
                .with("detail", e.to_string()),
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
        diags.push(Diagnostic::error(DiagCode::EmptyExtensions, "input.extensions"));
    }
```

In the per-track loop, add source validation:

```rust
        match &rule.source {
            SourceCfg::Keyword(k) if k == "primary" => {}
            SourceCfg::Keyword(k) => diags.push(
                Diagnostic::error(DiagCode::InvalidKeyword, format!("{base}.source"))
                    .with("found", k.clone())
                    .with("allowed", "primary"),
            ),
            SourceCfg::External { external } => {
                validate_locator(external, &format!("{base}.source.external"), &template_fields, false, &mut diags);
            }
        }
```

After the attachments loop, add section keyword/template validation:

```rust
    // output.filename
    match &profile.output.filename {
        FilenameCfg::Keyword(k) if k == "keep" => {}
        FilenameCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "output.filename")
                .with("found", k.clone())
                .with("allowed", "keep"),
        ),
        FilenameCfg::Template { template } => {
            let mut fields = template_fields.clone();
            fields.push("source_stem".into());
            validate_template(template, "output.filename.template", &fields, true, &mut diags);
        }
    }

    match &profile.chapters {
        ChaptersCfg::Keyword(k) if k == "keep" || k == "drop" => {}
        ChaptersCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "chapters")
                .with("found", k.clone())
                .with("allowed", "keep, drop"),
        ),
        ChaptersCfg::External { external } => {
            validate_locator(external, "chapters.external", &template_fields, false, &mut diags);
        }
    }

    match &profile.title {
        TitleCfg::Keyword(k) if k == "keep" || k == "clear" => {}
        TitleCfg::Keyword(k) => diags.push(
            Diagnostic::error(DiagCode::InvalidKeyword, "title")
                .with("found", k.clone())
                .with("allowed", "keep, clear"),
        ),
        TitleCfg::Template { template } => {
            let mut fields = template_fields.clone();
            fields.push("source_stem".into());
            validate_template(template, "title.template", &fields, false, &mut diags);
        }
    }
```

In `validate_attachment_rule`, replace the `rule.add` comment with locator validation (pass `template_fields` through as a parameter; adjust the function signature and its call site accordingly):

```rust
    if let Some(locator) = &rule.add {
        validate_locator(locator, &format!("{base}.add"), template_fields, false, diags);
    }
```

New helper functions at the bottom of the file:

```rust
fn validate_locator(
    locator: &Locator,
    path: &str,
    template_fields: &[String],
    _reserved: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if locator.extensions.is_empty() {
        diags.push(Diagnostic::error(
            DiagCode::EmptyExtensions,
            format!("{path}.extensions"),
        ));
    }
    if locator.match_to_source.is_some() && locator.match_pattern.is_some() {
        diags.push(Diagnostic::error(DiagCode::LocatorConflict, path.to_string()));
    }
    if let Some(pattern) = &locator.match_pattern {
        // source_stem is literal-mode only: do NOT add it here (spec 4.7).
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
            let (code, detail) = match e {
                TemplateError::UnknownFilter { name } => (
                    DiagCode::UnknownTemplateFilter,
                    format!("unknown filter: {name}"),
                ),
                TemplateError::UnclosedBrace { pos } => {
                    (DiagCode::InvalidTemplate, format!("unclosed brace at {pos}"))
                }
                TemplateError::EmptyField { pos } => {
                    (DiagCode::InvalidTemplate, format!("empty field at {pos}"))
                }
            };
            diags.push(Diagnostic::error(code, path.to_string()).with("detail", detail));
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
```

Note the `validate_expr` / `validate_attachment_rule` signatures change to thread `&template_fields` where needed; update the call sites shown in Task 8 accordingly.

- [ ] **Step 4: Run all validation tests**

Run: `cargo test -p muxsmith-core --test validate_structure --test validate_semantics`
Expected: PASS (all tests, both files)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(core): input, locator, keyword and template validation"
```

---

