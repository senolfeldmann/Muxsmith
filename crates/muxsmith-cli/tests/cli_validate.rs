use assert_cmd::Command;
use predicates::prelude::*;

fn muxsmith() -> Command {
    Command::cargo_bin("muxsmith").unwrap()
}

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn valid_profile_exits_zero_with_ok_message() {
    muxsmith()
        .args(["validate", &fixture("good.yaml")])
        .assert()
        .success()
        .stdout(predicate::str::contains("Profile is valid."));
}

#[test]
fn invalid_profile_exits_two_and_renders_messages() {
    muxsmith()
        .args(["validate", &fixture("bad.yaml")])
        .assert()
        .code(2)
        .stdout(
            predicate::str::contains("Invalid regular expression")
                .and(predicate::str::contains("input.pattern"))
                .and(predicate::str::contains("forced_track")),
        );
}

#[test]
fn warnings_only_exits_one() {
    // good.yaml plus an overlap warning: audio subset rule.
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  - match: { exact: { type: audio } }
  - match: { exact: { type: audio, language: en } }
"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("warn.yaml");
    std::fs::write(&path, y).unwrap();
    muxsmith()
        .args(["validate", path.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("provably overlap"));
}

#[test]
fn json_output_is_machine_readable() {
    let out = muxsmith()
        .args(["validate", &fixture("bad.yaml"), "--json"])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let diags = v["diagnostics"].as_array().unwrap();
    assert!(!diags.is_empty());
    let first = &diags[0];
    assert!(first["code"].is_string());
    assert!(first["severity"].is_string());
    assert!(first["rendered"].is_string());
}

#[test]
fn missing_file_is_parse_error_exit_two() {
    muxsmith()
        .args(["validate", "/nonexistent/profile.yaml"])
        .assert()
        .code(2);
}
