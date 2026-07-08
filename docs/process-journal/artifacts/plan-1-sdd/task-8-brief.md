### Task 8: Semantic validation of match expressions and changes

**Files:**
- Create: `crates/muxsmith-core/src/profile/validate.rs`
- Modify: `crates/muxsmith-core/src/profile/mod.rs`
- Test: `crates/muxsmith-core/tests/validate_semantics.rs`

**Interfaces:**
- Consumes: `Profile` model (Task 4), `capability` (Task 6), `report` (Task 2).
- Produces: `profile::validate::validate(&Profile) -> Vec<Diagnostic>`. Task 9 EXTENDS this same function (input/locator/template checks); Task 12's CLI calls it. `config_path` strings use the shape `tracks[3].match.exact.language`, `tracks[9].source.external`, `attachments.rules[0]`.

- [ ] **Step 1: Write the failing tests**

`crates/muxsmith-core/tests/validate_semantics.rs`:

```rust
use muxsmith_core::profile::load::{from_str, Format};
use muxsmith_core::profile::validate::validate;
use muxsmith_core::report::{DiagCode, Severity};

fn profile(tracks_yaml: &str) -> muxsmith_core::profile::Profile {
    let y = format!(
        r#"
profile_version: 1
input: {{ pattern: 'S(?<season>\d{{2}})E(?<episode>\d{{2}})', extensions: [mkv] }}
tracks:
{tracks_yaml}
"#
    );
    from_str(&y, Format::Yaml).unwrap()
}

fn codes(p: &muxsmith_core::profile::Profile) -> Vec<DiagCode> {
    validate(p).into_iter().map(|d| d.code).collect()
}

#[test]
fn reference_profile_validates_clean() {
    let text = include_str!("fixtures/reference.yaml");
    let p = from_str(text, Format::Yaml).unwrap();
    let errors: Vec<_> = validate(&p)
        .into_iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert_eq!(errors, vec![], "reference profile must have zero errors");
}

#[test]
fn wrong_profile_version_is_rejected() {
    let mut p = profile("  - match: { exact: { type: video } }");
    p.profile_version = 2;
    assert!(codes(&p).contains(&DiagCode::UnsupportedProfileVersion));
}

#[test]
fn empty_tracks_list_is_rejected() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::NoTrackRules));
}

#[test]
fn unknown_match_property_is_flagged_with_path() {
    let p = profile("  - match: { exact: { colour_depth: 10 } }");
    let diags = validate(&p);
    let d = diags.iter().find(|d| d.code == DiagCode::UnknownProperty).unwrap();
    assert_eq!(d.config_path, "tracks[0].match.exact.colour_depth");
    assert_eq!(d.params["property"], "colour_depth");
}

#[test]
fn substring_on_boolean_property_is_flagged() {
    let p = profile("  - match: { substring: { forced_track: 'yes' } }");
    assert!(codes(&p).contains(&DiagCode::NotStringProperty));
}

#[test]
fn exact_value_type_mismatch_is_flagged() {
    let p = profile("  - match: { exact: { forced_track: 'yes' } }");
    assert!(codes(&p).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn integer_accepted_for_float_property_but_not_reverse() {
    // audio_sampling_frequency is number (Float) in the schema.
    let ok = profile("  - match: { exact: { audio_sampling_frequency: 48000 } }");
    assert!(!codes(&ok).contains(&DiagCode::ValueTypeMismatch));
    let bad = profile("  - match: { exact: { audio_channels: 5.1 } }");
    assert!(codes(&bad).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn invalid_condition_regex_is_flagged() {
    let p = profile("  - match: { regex: { track_name: '([' } }");
    assert!(codes(&p).contains(&DiagCode::InvalidRegex));
}

#[test]
fn nested_any_and_not_are_validated_recursively() {
    let p = profile(
        "  - match:\n      any:\n        - exact: { nonexistent_prop: 1 }",
    );
    let diags = validate(&p);
    let d = diags.iter().find(|d| d.code == DiagCode::UnknownProperty).unwrap();
    assert_eq!(d.config_path, "tracks[0].match.any[0].exact.nonexistent_prop");
}

#[test]
fn empty_match_expression_is_warning() {
    let p = profile("  - match: {}");
    let diags = validate(&p);
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::EmptyMatchExpression)
        .unwrap();
    assert_eq!(d.severity, Severity::Warning);
}

#[test]
fn unknown_change_property_is_flagged() {
    let p = profile(
        "  - match: { exact: { type: video } }\n    changes: { bitrate: 5000 }",
    );
    assert!(codes(&p).contains(&DiagCode::UnknownSettableProperty));
}

#[test]
fn change_value_type_mismatch_is_flagged() {
    let p = profile(
        "  - match: { exact: { type: video } }\n    changes: { default_track: 'yes' }",
    );
    assert!(codes(&p).contains(&DiagCode::ValueTypeMismatch));
}

#[test]
fn attachment_rule_must_have_exactly_one_action() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
attachments:
  rules:
    - select: { substring: { content_type: font } }
      drop: { substring: { file_name: cover } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::AttachmentRuleShape));
}

#[test]
fn attachment_match_uses_attachment_property_set() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: video } }
attachments:
  rules:
    - select: { exact: { language: en } }
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    // "language" is a track property, not an attachment property.
    assert!(codes(&p).contains(&DiagCode::UnknownProperty));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core --test validate_semantics`
Expected: FAIL (`validate` not defined)

The `include_str!("fixtures/reference.yaml")` path resolves relative to the test file, so Task 4's fixture is already in place; nothing to copy.

- [ ] **Step 3: Implement**

`crates/muxsmith-core/src/profile/validate.rs`:

```rust
//! Semantic validation (spec 5.4 static checks, config-time part).
//! Task 9 extends this file with input/locator/template validation.

use std::collections::BTreeMap;

use crate::capability::{self, PropType};
use crate::report::{DiagCode, Diagnostic};

use super::match_expr::{MatchExpr, Scalar};
use super::model::{AttachmentRule, Profile};

pub fn validate(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if profile.profile_version != 1 {
        diags.push(
            Diagnostic::error(DiagCode::UnsupportedProfileVersion, "profile_version")
                .with("found", profile.profile_version.to_string())
                .with("supported", "1"),
        );
    }

    if profile.tracks.is_empty() {
        diags.push(Diagnostic::error(DiagCode::NoTrackRules, "tracks"));
    }

    for (i, rule) in profile.tracks.iter().enumerate() {
        let base = format!("tracks[{i}]");
        if rule.match_expr.is_empty() {
            diags.push(Diagnostic::warning(
                DiagCode::EmptyMatchExpression,
                format!("{base}.match"),
            ));
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
    }

    for (i, rule) in profile.attachments.rules.iter().enumerate() {
        let base = format!("attachments.rules[{i}]");
        validate_attachment_rule(rule, &base, &mut diags);
    }

    diags
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

fn validate_attachment_rule(rule: &AttachmentRule, base: &str, diags: &mut Vec<Diagnostic>) {
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
    // rule.add locator validation arrives with Task 9.
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
            match prop_type(prop) {
                None => diags.push(unknown_property(&p, prop)),
                Some(t) => {
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
    for (map, kind) in [(&expr.substring, "substring"), (&expr.regex, "regex")] {
        if let Some(map) = map {
            for (prop, value) in map.iter() {
                let p = format!("{path}.{kind}.{prop}");
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
                if kind == "regex" {
                    if let Err(e) = regex::Regex::new(value) {
                        diags.push(
                            Diagnostic::error(DiagCode::InvalidRegex, p)
                                .with("detail", e.to_string()),
                        );
                    }
                }
            }
        }
    }
    if let Some(any) = &expr.any {
        for (i, sub) in any.iter().enumerate() {
            validate_expr(sub, &format!("{path}.any[{i}]"), prop_type, diags);
        }
    }
    if let Some(not) = &expr.not {
        for (i, sub) in not.iter().enumerate() {
            validate_expr(sub, &format!("{path}.not[{i}]"), prop_type, diags);
        }
    }
}

fn validate_changes(
    changes: &BTreeMap<String, Scalar>,
    path: &str,
    diags: &mut Vec<Diagnostic>,
) {
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

fn type_label(t: PropType) -> &'static str {
    match t {
        PropType::String => "string",
        PropType::Boolean => "boolean",
        PropType::Integer => "integer",
        PropType::Float => "float",
    }
}
```

Add `pub mod validate;` to `crates/muxsmith-core/src/profile/mod.rs`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core --test validate_semantics`
Expected: PASS (14 tests)

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(core): semantic validation of match expressions, changes, attachment rules"
```

---

