## Task 3: Match Expression Model — TDD Completion Report

### Status
**COMPLETE** — All 5 tests passing, full workspace test suite green.

### Commits
- **c6a2df2** `feat(core): match expression model with recursive any/not`
  - Created `crates/muxsmith-core/src/profile/mod.rs`
  - Created `crates/muxsmith-core/src/profile/match_expr.rs` (Scalar enum + MatchExpr struct)
  - Modified `crates/muxsmith-core/src/lib.rs` to export profile module
  - Updated `Cargo.toml`: added schemars v1, yaml_serde v0.10 (dev)

### TDD Evidence

**Step 1: Dependencies**
```bash
$ cargo add -p muxsmith-core schemars
  Adding schemars v1.2.1 to dependencies

$ cargo add -p muxsmith-core --dev yaml-serde
warning: translating `yaml-serde` to `yaml_serde`
  Adding yaml_serde v0.10.4 to dev-dependencies
```

**Step 2: Tests (Red)**
```bash
$ cargo test -p muxsmith-core match_expr
error[E0425]: cannot find type `MatchExpr` in this scope
error[E0425]: cannot find type `Scalar` in this scope
  → compilation fails as expected (RED)
```

**Step 3: Implementation**
- `Scalar`: untagged enum with Bool(bool) | Int(i64) | Float(f64) | Str(String)
- `MatchExpr`: struct with exact, substring, regex, any, not fields (all Option<...>)
- Both derive: Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema
- MatchExpr: deny_unknown_fields, all fields skip_serializing_if = "Option::is_none"

**Step 4: Tests (Green)**
```bash
$ cargo test -p muxsmith-core match_expr
running 5 tests
test profile::match_expr::tests::empty_expression_reports_empty ... ok
test profile::match_expr::tests::scalar_type_names ... ok
test profile::match_expr::tests::rejects_unknown_keys ... ok
test profile::match_expr::tests::parses_flat_expression ... ok
test profile::match_expr::tests::parses_nested_any_and_not ... ok

test result: ok. 5 passed; 0 failed
```

**Step 5: Full Workspace**
```bash
$ cargo test --workspace
running 12 tests
  [5 new match_expr tests + 7 existing report tests]
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured
```

### Verification vs Brief

| Requirement | Status |
|---|---|
| Scalar untagged enum | ✓ Bool \| Int \| Float \| Str |
| Scalar::type_name() → "boolean" \| "integer" \| "float" \| "string" | ✓ |
| MatchExpr fields (exact, substring, regex, any, not) | ✓ All Option<...> |
| MatchExpr::is_empty() | ✓ Returns true iff all fields None or empty |
| Derives (Deserialize, Serialize, JsonSchema, Debug, Clone, PartialEq) | ✓ Both types |
| deny_unknown_fields on MatchExpr | ✓ |
| Test: flat expression parsing | ✓ PASS |
| Test: nested any/not parsing | ✓ PASS |
| Test: unknown field rejection | ✓ PASS |
| Test: empty expression detection | ✓ PASS |
| Test: scalar type names | ✓ PASS |

### Notes
- yaml_serde auto-translated from yaml-serde (hyphen → underscore). Import path `yaml_serde::from_str()` works correctly.
- No GPG issues encountered; commit succeeded with trailer.
- No breaking changes to existing tests.
