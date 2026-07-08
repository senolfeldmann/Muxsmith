use assert_cmd::Command;

#[test]
fn schema_prints_json_schema_and_exits_zero() {
    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let schema: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let text = schema.to_string();
    assert!(text.contains("profile_version"));
    assert!(text.contains("tracks"));
}

#[test]
fn no_args_shows_usage_and_fails() {
    Command::cargo_bin("muxsmith").unwrap().assert().failure();
}
