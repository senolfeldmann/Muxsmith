mod support;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))
}

/// Spec 10: human-rendered CLI text is snapshot-tested (insta), not pinned
/// via `predicate::str::contains` on fragments of the wording -- a copy
/// edit to the Fluent template used to fail unrelated-looking asserts here.
/// No path/version/duration redaction needed: `validate` never touches
/// mkvmerge or the queue, and its diagnostics render schema-relative
/// `config_path`s (e.g. `input.pattern`), never the profile file's own
/// filesystem location.
#[test]
fn valid_profile_exits_zero_with_ok_message() {
    let out = support::muxsmith(&["validate", &fixture("good.yaml")])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    insta::assert_snapshot!(String::from_utf8(out).unwrap());
}

#[test]
fn invalid_profile_exits_two_and_renders_messages() {
    let out = support::muxsmith(&["validate", &fixture("bad.yaml")])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    insta::assert_snapshot!(String::from_utf8(out).unwrap());
}

#[test]
fn warnings_only_exits_one() {
    // good.yaml plus an overlap warning: audio subset rule.
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: audio } }
    - match: { exact: { type: audio, language: en } }
"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("warn.yaml");
    std::fs::write(&path, y).unwrap();
    let out = support::muxsmith(&["validate", path.to_str().unwrap()])
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();
    insta::assert_snapshot!(String::from_utf8(out).unwrap());
}

/// D101: a `raw:` key with an EMPTY bare property name is a config-time
/// error (`EmptyRawProperty`), not the `RawProperty` info a non-empty
/// `raw:x` key still yields. The user-visible consequence this pins is the
/// exit code: the identical profile exited 0 with an info diagnostic
/// before, and exits 2 now (accepted consequence, ruled 2026-07-28).
#[test]
fn bare_raw_property_exits_two_and_renders_the_message() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { 'raw:': eng } }
"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bare-raw.yaml");
    std::fs::write(&path, y).unwrap();
    let out = support::muxsmith(&["validate", path.to_str().unwrap()])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    insta::assert_snapshot!(String::from_utf8(out).unwrap());
}

/// The de half of D101's catalog obligation, on the identical profile:
/// `--locale de` renders the German message and the exit code is still 2.
/// Invoked through [`support::muxsmith_localized`], not the en funnel: the
/// funnel appends `--locale en` after the caller's args and clap rejects a
/// repeated `--locale`, so passing `--locale de` through it would exit 2 on
/// clap's own usage error - passing this test's `.code(2)` while
/// snapshotting empty stdout. The SNAPSHOT is therefore the load-bearing
/// assertion here, not the exit code.
#[test]
fn bare_raw_property_renders_german_with_locale_flag() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { 'raw:': eng } }
"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bare-raw.yaml");
    std::fs::write(&path, y).unwrap();
    let out = support::muxsmith_localized(&["validate", path.to_str().unwrap()], "de")
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();
    insta::assert_snapshot!(String::from_utf8(out).unwrap());
}

#[test]
fn json_output_is_machine_readable() {
    let out = support::muxsmith(&["validate", &fixture("bad.yaml"), "--json"])
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
    // bad.yaml yields both errors and a warning; sorted output leads
    // with an error, same order as the text rendering.
    assert_eq!(first["severity"], "error");
}

#[test]
fn missing_file_is_parse_error_exit_two() {
    support::muxsmith(&["validate", "/nonexistent/profile.yaml"])
        .assert()
        .code(2);
}
