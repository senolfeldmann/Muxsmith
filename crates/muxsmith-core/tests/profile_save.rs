//! Spec 8.2 / D41: saving writes canonical YAML from the model. These tests
//! pin the correctness floor (a saved profile reloads equal) and the
//! format-from-extension contract; the default-omission behaviour is D48's
//! and is tested in its own guards.

use muxsmith_core::profile::load::{Format, from_file, from_str};
use muxsmith_core::profile::save::{SaveError, to_file, to_string};
use serde_json::{Value, json};

const REFERENCE: &str = include_str!("fixtures/reference.yaml");
const ALL_NON_DEFAULT: &str = include_str!("fixtures/all-non-default.yaml");

#[test]
fn canonical_yaml_round_trips_to_an_equal_model() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Yaml).expect("model serializes");
    let p2 = from_str(&text, Format::Yaml).expect("canonical output re-parses");
    assert_eq!(p, p2, "a saved profile must reload to an equal model (D41)");
}

#[test]
fn canonical_json_round_trips_to_an_equal_model() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Json).expect("model serializes");
    let p2 = from_str(&text, Format::Json).expect("canonical JSON re-parses");
    assert_eq!(p, p2);
}

#[test]
fn to_file_picks_json_from_the_extension_and_never_changes_format() {
    let dir = tempfile::tempdir().unwrap();
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");

    let json_path = dir.path().join("profile.json");
    to_file(&p, &json_path).expect("writes");
    let text = std::fs::read_to_string(&json_path).unwrap();
    assert!(
        text.trim_start().starts_with('{'),
        "a .json path must save as JSON, not YAML: {text}"
    );
    assert_eq!(from_file(&json_path).unwrap(), p);

    let yaml_path = dir.path().join("profile.yaml");
    to_file(&p, &yaml_path).expect("writes");
    let text = std::fs::read_to_string(&yaml_path).unwrap();
    assert!(
        !text.trim_start().starts_with('{'),
        "a .yaml path must save as YAML: {text}"
    );
    assert_eq!(from_file(&yaml_path).unwrap(), p);
}

#[test]
fn an_unwritable_path_is_an_io_error_not_a_panic() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let dir = tempfile::tempdir().unwrap();
    let nope = dir.path().join("no-such-dir").join("profile.yaml");
    match to_file(&p, &nope) {
        Err(SaveError::Io(detail)) => assert!(!detail.is_empty(), "detail carries the io message"),
        other => panic!("expected SaveError::Io, got {other:?}"),
    }
}

/// D48 guard 1: every one of the 17 defaulted fields set to a NON-default
/// value must survive a save/load round trip. A predicate that skips a value
/// which is not the default silently destroys it - the core-83 passthrough
/// class of bug (`unmatched: keep` reloading as `drop`).
#[test]
fn all_non_default_fields_survive_the_round_trip() {
    let p = from_str(ALL_NON_DEFAULT, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Yaml).expect("serializes");
    let p2 = from_str(&text, Format::Yaml).expect("re-parses");
    assert_eq!(
        p, p2,
        "a non-default value must never be omitted (D48 guard 1)"
    );
}

/// The sharpest instance, called out because it is an owner-ruled-legal
/// profile (`core-83`) that a naive `is_default` turns into a NoTrackRules
/// error: zero rules plus `unmatched: keep` is a pure-passthrough remux.
#[test]
fn the_core83_passthrough_profile_survives_a_save() {
    let y = "profile_version: 1\ninput: { pattern: 'E(\\d+)', extensions: [mkv] }\ntracks:\n  unmatched: keep\n  rules: []\n";
    let p = from_str(y, Format::Yaml).expect("parses");
    let text = to_string(&p, Format::Yaml).expect("serializes");
    assert!(
        text.contains("unmatched: keep"),
        "tracks.unmatched defaults to DROP, so `keep` is not a default and must be written: {text}"
    );
    assert_eq!(from_str(&text, Format::Yaml).unwrap(), p);
}

/// D48 guard 2: schema-default honesty. Asserts, for each of the 17
/// defaulted fields (design D48), that the published JSON
/// Schema's `default` annotation equals the value below. The expected
/// values are hand-written literals, not calls into the model's own
/// default-producing functions: comparing the schema against a value
/// derived from the same function the `#[schemars(extend(...))]`
/// annotation calls would make the assertion `to_value(F()) ==
/// to_value(F())` - a tautology that can never fail (D48's guard-2
/// analysis; the guard stays despite that analysis, per
/// `proc-proposed-safeguard-stays`, until it is measured redundant
/// against the built code). Follows the house table-test shape
/// (`capability/mod.rs`'s `settable_maps_to_mkvmerge_options`): a literal
/// table asserted against the real thing, length first, then row by row.
#[test]
fn schema_defaults_match_the_serde_defaults() {
    let schema = serde_json::to_value(schemars::schema_for!(muxsmith_core::profile::Profile))
        .expect("schema serializes to a Value");

    // (schema location, field, expected `default`). `None` = the field
    // sits directly on `Profile`'s own `properties`; `Some(ty)` = it sits
    // on `$defs/<ty>/properties`.
    let expected: [(Option<&str>, &str, Value); 17] = [
        (None, "output", json!({})),
        (None, "attachments", json!({})),
        (None, "chapters", json!("keep")),
        (None, "tags", json!({})),
        (None, "title", json!("keep")),
        (Some("Input"), "recursive", json!(true)),
        (Some("OutputCfg"), "filename", json!("keep")),
        (Some("OutputCfg"), "on_collision", json!("error")),
        (Some("TrackRule"), "source", json!("primary")),
        (Some("TrackRule"), "optional", json!(false)),
        (Some("Locator"), "recursive", json!(false)),
        (Some("Locator"), "case_sensitive", json!(false)),
        (Some("AttachmentsCfg"), "unmatched", json!("keep")),
        (Some("AttachmentsCfg"), "rules", json!([])),
        (Some("TracksCfg"), "unmatched", json!("drop")),
        (Some("TagsCfg"), "global", json!("keep")),
        (Some("TagsCfg"), "track", json!("keep")),
    ];

    // Every `properties.*.default` anywhere in the schema (Profile's own
    // plus every `$defs` type) must be exactly this set: not a subset (a
    // field missing its `extend` annotation leaves a silent hole) and not
    // a superset (a row here that no longer matches anything in the
    // schema is stale).
    let schema_default_count = count_defaults(&schema["properties"])
        + schema["$defs"]
            .as_object()
            .into_iter()
            .flat_map(|defs| defs.values())
            .map(|ty| count_defaults(&ty["properties"]))
            .sum::<usize>();
    assert_eq!(
        schema_default_count,
        expected.len(),
        "schema's total defaulted-field count must match D48's 17-row table"
    );

    for row in &expected {
        let (in_type, field, want) = (row.0, row.1, &row.2);
        let pointer = match in_type {
            None => format!("/properties/{field}/default"),
            Some(ty) => format!("/$defs/{ty}/properties/{field}/default"),
        };
        let got = schema
            .pointer(&pointer)
            .unwrap_or_else(|| panic!("no default at {pointer} for {in_type:?}.{field}"));
        assert_eq!(got, want, "schema default mismatch for {in_type:?}.{field}");
    }
}

/// Counts how many entries of a schema `properties` object carry a
/// `default` key; the cross-check half of D48 guard 2.
fn count_defaults(properties: &Value) -> usize {
    properties
        .as_object()
        .into_iter()
        .flat_map(|m| m.values())
        .filter(|v| v.get("default").is_some())
        .count()
}
