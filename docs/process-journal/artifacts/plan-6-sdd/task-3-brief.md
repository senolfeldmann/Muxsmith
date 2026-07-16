### Task 3: D46 - keyword domains from one constant set

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs` (4 `KEYWORDS` consts, 4 `schema_with` projections)
- Modify: `crates/muxsmith-core/src/profile/validate.rs` (guards at `:105`, `:129`, `:149`, `:166`; the 4 `allowed` params)
- Test: `crates/muxsmith-cli/tests/cli_schema.rs`, `crates/muxsmith-core/tests/validate_semantics.rs`

**Interfaces:**
- Produces, for Task 5's emitter: `FilenameCfg::KEYWORDS`, `SourceCfg::KEYWORDS`, `ChaptersCfg::KEYWORDS`, `TitleCfg::KEYWORDS`, each `pub const &'static [&'static str]`.
- Consumes: nothing from other tasks.

**Read first:** design D46 (`:1103-1224`) in full. It carries the exact constant values, the placement rationale (`profile::model`, **not** `capability`), the `domain_hint` refactor, and the empirically verified schemars output.

Binding points:
- The `Keyword(String)` arm **keeps its `String`**. Do not "fix" it by typing the arm - that destroys the `InvalidKeyword` diagnostic and replaces it with serde's untagged error. D46 records the full steelman for the typed arm; it lost, and the reason is recorded so it is not re-litigated.
- The schema override emits **`enum`, not `oneOf`+`const`** (D46's rejected alternative).
- The `allowed` param goes through the existing `domain_hint` (`profile/validate.rs:430-437`). The four hand-typed strings today are `"primary"`, `"keep"`, `"keep, drop"`, `"keep, clear"`, and the const-derived values are byte-identical, so **the CLI snapshots must not move**. Any diff in `crates/muxsmith-cli/tests/snapshots/` means the refactor is wrong.

- [ ] **Step 1: Extract the schema helper and write the failing schema test**

`cli_schema.rs` today has **no** helper: both its tests inline `Command::cargo_bin("muxsmith").unwrap().arg("schema").assert().success()...` and parse the stdout. Extract that block into a `schema_json()` helper and re-point the existing `schema_prints_json_schema_and_exits_zero` test at it - both tests then share the one invocation (this is the only legal move: the file forbids a second way to run the command, and duplicating the block would be that). Read the file first to copy its exact invocation.

```rust
fn schema_json() -> serde_json::Value {
    let out = Command::cargo_bin("muxsmith")
        .unwrap()
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&out).unwrap()
}
```

Then add the new test, whose target shape D46 measured empirically (`:1112-1120`):

```rust
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
        let branches = schema["$defs"][ty]["anyOf"]
            .as_array()
            .unwrap_or_else(|| panic!("{ty} must still project anyOf (D46 narrows the string branch only)"));
        let string_branch = branches
            .iter()
            .find(|b| b["type"] == "string")
            .unwrap_or_else(|| panic!("{ty} must keep a string branch"));
        let got: Vec<&str> = string_branch["enum"]
            .as_array()
            .unwrap_or_else(|| panic!("{ty}'s string branch must carry an enum, not a bare string type"))
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(got, expected, "{ty} keyword domain");
    }
}
```

- [ ] **Step 2: Run it to confirm it fails**

```bash
cargo test -p muxsmith-cli --test cli_schema keyword_domains
```
Expected: FAIL - the string branch has no `enum` key today (design `:70-77` measured it as a bare `{type: "string"}`).

- [ ] **Step 3: Add the four constant sets**

In `profile/model.rs`, beside each enum it belongs to, exactly as design `:1156-1161` states. The shape deliberately copies the house pattern at `capability/mod.rs:55` (`pub static &[&str]` closed domain read by a lookup fn) and `:125-129` (`CODEC_KIND_NAMES` derived from `CODEC_KINDS` "so the two can never drift").

- [ ] **Step 4: Point the validate guards at the constants**

Replace the four bare-literal match guards (`validate.rs:105`, `:129`, `:149`, `:166`) with `k if FilenameCfg::KEYWORDS.contains(&k.as_str())` and its three siblings, and replace the four hand-typed `allowed` strings with `domain_hint` calls over the same constants.

- [ ] **Step 5: Add the schema projections**

Add one `schema_with` function per enum and attach it to the `Keyword` arm, per D46's measured output (`:1112-1116`). The variant's doc comment must survive as the branch `description`, merged rather than replaced - the design measured that it does.

- [ ] **Step 6: Verify the diagnostic still reaches the keyword**

`InvalidKeyword` must stay reachable with its `found` and `allowed` params - that is the whole reason the arm keeps its `String`. Confirm with the design's own probe (`:1118-1120`): `'kepp'` deserializes to `Keyword("kepp")` and validate rejects it. Add a test to `crates/muxsmith-core/tests/validate_semantics.rs` asserting a misspelled keyword yields `DiagCode::InvalidKeyword` with `allowed` equal to the const-derived hint (e.g. `"keep, drop"` for `chapters`).

- [ ] **Step 7: Run the tests and prove the snapshots did not move**

```bash
cargo test -p muxsmith-cli --test cli_schema
cargo test -p muxsmith-core --test validate_semantics
cargo test --workspace
git diff --exit-code crates/muxsmith-cli/tests/snapshots/
```
Expected: all pass, and the `git diff` exits **0** with no output. A moved snapshot means the `domain_hint` refactor changed user-visible output, which D46 says it must not.

- [ ] **Step 8: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/src/profile/validate.rs crates/muxsmith-cli/tests/cli_schema.rs crates/muxsmith-core/tests/validate_semantics.rs
git -c commit.gpgsign=false commit -m "core: keyword domains project into the schema from one constant set (D46)"
```

---

