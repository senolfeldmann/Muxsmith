//! Spec 8.2 / D41: saving writes canonical YAML from the model. These tests
//! pin the correctness floor (a saved profile reloads equal) and the
//! format-from-extension contract; the default-omission behaviour is D48's
//! and is tested in its own guards.

use muxsmith_core::profile::load::{Format, from_file, from_str};
use muxsmith_core::profile::save::{SaveError, to_file, to_string};

const REFERENCE: &str = include_str!("fixtures/reference.yaml");

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
