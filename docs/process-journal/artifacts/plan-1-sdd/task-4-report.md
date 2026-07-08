# Task 4 Implementation Report

## Summary

Task 4: Full profile model and loader completed successfully following TDD steps exactly as specified.

**Commit:** `3ab1cdc` – feat(core): full profile model with loader and reference fixture

## TDD Process

### Step 1: Add Dependencies

Executed:
```bash
cargo add -p muxsmith-core yaml_serde serde_json serde_path_to_error
```

Removed `yaml_serde` from `[dev-dependencies]` to live under `[dependencies]` only.

### Step 2: Create Reference Fixture

Created `/crates/muxsmith-core/tests/fixtures/reference.yaml`:
- Copied spec 4.1 example verbatim
- Replaced `# analogous forced / plain / SDH rules for language: de omitted for brevity` comment with three German subtitle rules
- German rules mirror English rules (lines 77–94 of spec) with `language: de` and track names `German forced`, `German`, `German SDH`
- Set `directory: null` under `output`
- Total tracks: 10 (1 video + 2 audio + 6 subs + 1 external)

### Step 3: Write Failing Tests

Created `/crates/muxsmith-core/tests/profile_load.rs` with 4 tests:
- `reference_profile_parses` – validates complete fixture parsing
- `json_profile_parses_identically_to_yaml` – ensures both formats round-trip identically
- `defaults_apply_when_sections_absent` – verifies Default implementations work correctly
- `unknown_key_is_parse_error_with_path` – confirms deny_unknown_fields and serde_path_to_error integration

### Step 4: Verify Tests Fail

Output:
```
error[E0432]: unresolved import `muxsmith_core::profile::model`
error[E0432]: unresolved import `muxsmith_core::profile::load`
```

Confirmed: tests fail as expected due to missing modules.

### Step 5: Implement Model and Loader

**Created `/crates/muxsmith-core/src/profile/model.rs`:**
- 228 lines implementing all required types:
  - `Profile` (root, derives Debug/Clone/PartialEq/Deserialize/Serialize/JsonSchema, deny_unknown_fields)
  - `Meta { name, description }` (optional fields)
  - `Input { pattern, extensions, recursive: bool = true }`
  - `OutputCfg { directory: Option, filename, on_collision }` with Default
  - Untagged enums: `FilenameCfg`, `ChaptersCfg`, `TitleCfg` (Template or Keyword variant, struct first per spec note)
  - `CollisionPolicy { Error, Skip, Overwrite }` (lowercase serde)
  - `KeepDrop { Keep, Drop }` (lowercase serde)
  - `TrackRule { source, match_expr (renamed "match"), optional, changes }` with defaults
  - `SourceCfg` untagged enum (External or Keyword with primary default)
  - `Locator { path, recursive, extensions, match_to_source, match_pattern, case_sensitive }`
  - `AttachmentsCfg { unmatched, rules }` with Keep default
  - `AttachmentRule { select, drop, add }` (all optional)
  - `TagsCfg { global, track }` (both Keep default)

**Created `/crates/muxsmith-core/src/profile/load.rs`:**
- 45 lines with public API:
  - `Format { Yaml, Json }` enum
  - `from_str(text: &str, format: Format) -> Result<Profile, Diagnostic>` – dispatches to yaml_serde or serde_json, wraps errors via serde_path_to_error
  - `from_file(path: &Path) -> Result<Profile, Diagnostic>` – detects format by extension, reads file, calls from_str
  - `parse_error(err)` – converts serde_path_to_error::Error to Diagnostic with detail and at params

**Updated `/crates/muxsmith-core/src/profile/mod.rs`:**
- Added `pub mod load;` and `pub mod model;`
- Added `pub use model::Profile;`

### Step 6: Verify Tests Pass

Focused test output:
```
running 4 tests
test defaults_apply_when_sections_absent ... ok
test unknown_key_is_parse_error_with_path ... ok
test json_profile_parses_identically_to_yaml ... ok
test reference_profile_parses ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

Full workspace test output:
```
running 12 tests (muxsmith-core lib)
[all 12 existing unit tests still pass]

running 4 tests (profile_load integration)
[all 4 new tests pass]

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured
```

### Step 7: Commit

```
git add -A
git commit -m "feat(core): full profile model with loader and reference fixture"
```

Commit 3ab1cdc: 7 files changed, 452 insertions(+), 1 deletion(-)

## Self-Review Against Brief

✅ All model types derive Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema
✅ All structs have deny_unknown_fields
✅ Untagged enums properly ordered (struct variant before Keyword)
✅ Defaults implemented: Input.recursive=true, TrackRule.source=primary, OutputCfg defaults chain, KeepDrop defaults, etc.
✅ FilenameCfg and TitleCfg with keep() method
✅ TitleCfg with clear() default applied via Keyword("clear")
✅ ChaptersCfg Keyword default is "keep"
✅ serde_path_to_error integrated with detail and at params
✅ from_file detects format by extension (.json → JSON, else YAML)
✅ Fixture has exactly 10 tracks in correct order
✅ Reference fixture parses and validates against all 4 assertions
✅ No modifications to existing code broke any tests

## Test Evidence

Full `cargo test --workspace` output shows:
- 12 lib tests passing (existing profile::match_expr and report tests)
- 4 profile_load integration tests passing
- 0 failures, 0 ignored

No regressions introduced.

---

## Fix: unknown-key rejection inside untagged config blocks

**Commit:** `b5eaa3d` – fix(core): enforce unknown-key rejection inside untagged config blocks

### Problem (reviewer finding)

serde's `deny_unknown_fields` is silently ignored on inline struct variants of
untagged enums. `filename: { template: "x", oops: 1 }` parsed successfully,
violating spec 4 ("Unknown keys are errors, not warnings").

### Fix

- `model.rs`: added standalone `TemplateBlock { template: String }` and
  `ExternalBlock { external: Locator }` structs (full derive set +
  `deny_unknown_fields`); converted the four untagged enums to newtype
  variants over them:
  - `FilenameCfg::Template(TemplateBlock)`, `TitleCfg::Template(TemplateBlock)`
  - `SourceCfg::External(ExternalBlock)`, `ChaptersCfg::External(ExternalBlock)`
  - Struct variant still declared before `Keyword(String)`; helper fns
    (`FilenameCfg::keep()`, `SourceCfg::primary()`) and Default impls unchanged.
  - Wire format unchanged (newtype-over-struct serializes identically).
- `profile_load.rs`: destructuring updated to `SourceCfg::External(block)` /
  `block.external.*`; added regression tests
  `unknown_key_inside_filename_template_is_rejected` and
  `unknown_key_inside_source_external_is_rejected`; extended
  `defaults_apply_when_sections_absent` with `on_collision == Error`,
  `chapters`/`title` Keyword "keep", `tags.track == Keep`,
  `attachments.unmatched == Keep`; `unknown_key_is_parse_error_with_path`
  now also asserts `err.params.contains_key("at")`.
- `Cargo.toml`: removed the duplicate `serde_json` entry from
  `[dev-dependencies]`.

### Verification (verbatim `cargo test --workspace` output)

```
   Compiling muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
   Compiling muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/main.rs (target/debug/deps/muxsmith-c8714a03b341a895)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/muxsmith_core-c0b667a9c409e64b)

running 12 tests
test profile::match_expr::tests::scalar_type_names ... ok
test profile::match_expr::tests::empty_expression_reports_empty ... ok
test report::tests::all_keys_match_serde_encoding ... ok
test profile::match_expr::tests::rejects_unknown_keys ... ok
test report::tests::all_keys_are_unique ... ok
test profile::match_expr::tests::parses_flat_expression ... ok
test report::tests::diagnostic_builder_sets_fields ... ok
test report::tests::diag_code_keys_are_kebab_case ... ok
test profile::match_expr::tests::parses_nested_any_and_not ... ok
test report::tests::diagnostic_serializes_with_snake_case_severity_and_kebab_code ... ok
test report::tests::severity_orders_info_warning_error ... ok
test report::tests::worst_severity_picks_error_over_warning ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/profile_load.rs (target/debug/deps/profile_load-29d84195d43586f8)

running 6 tests
test unknown_key_inside_filename_template_is_rejected ... ok
test unknown_key_is_parse_error_with_path ... ok
test json_profile_parses_identically_to_yaml ... ok
test defaults_apply_when_sections_absent ... ok
test unknown_key_inside_source_external_is_rejected ... ok
test reference_profile_parses ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests muxsmith_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
