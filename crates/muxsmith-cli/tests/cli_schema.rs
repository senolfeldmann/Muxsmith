#[allow(dead_code)]
mod support;

/// The one legal way to invoke `muxsmith schema` and parse its stdout; every
/// test in this file shares it rather than re-inlining the invocation.
fn schema_json() -> serde_json::Value {
    let out = support::muxsmith_bare()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&out).unwrap()
}

#[test]
fn schema_prints_json_schema_and_exits_zero() {
    let schema = schema_json();
    let text = schema.to_string();
    assert!(text.contains("profile_version"));
    assert!(text.contains("tracks"));
}

#[test]
fn no_args_shows_usage_and_fails() {
    support::muxsmith_bare().assert().failure();
}

#[test]
fn keyword_domains_project_as_closed_enums_not_bare_strings() {
    let schema = schema_json();
    let cases = [
        ("FilenameCfg", vec!["keep"]),
        ("SourceCfg", vec!["primary"]),
        ("ChaptersCfg", vec!["keep", "drop"]),
        ("TitleCfg", vec!["keep", "clear"]),
    ];
    for (ty, expected) in cases {
        let branches = schema["$defs"][ty]["anyOf"].as_array().unwrap_or_else(|| {
            panic!("{ty} must still project anyOf (D46 narrows the string branch only)")
        });
        let string_branch = branches
            .iter()
            .find(|b| b["type"] == "string")
            .unwrap_or_else(|| panic!("{ty} must keep a string branch"));
        let got: Vec<&str> = string_branch["enum"]
            .as_array()
            .unwrap_or_else(|| {
                panic!("{ty}'s string branch must carry an enum, not a bare string type")
            })
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(got, expected, "{ty} keyword domain");
    }
}
