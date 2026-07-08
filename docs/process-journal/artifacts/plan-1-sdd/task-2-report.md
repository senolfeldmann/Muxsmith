# Task 2 Report: Diagnostic types (report module)

## Summary

Implemented stable diagnostic type catalog for Muxsmith core. All interfaces per brief: `Severity` (ordered Info < Warning < Error), `DiagCode` enum with 28 variants and kebab-case keys, `Diagnostic` struct with builder pattern, and `worst_severity()` utility. Serialization via serde with snake_case severity and kebab-case code keys.

**Commit:** `9518aa1 feat(core): diagnostic types with stable kebab-case catalog keys`

---

## TDD Evidence

### Step 1: Dependencies Added
```bash
cargo add -p muxsmith-core serde --features derive
cargo add -p muxsmith-core serde_json --dev
```
Status: **PASS** — serde 1.0.228 and serde_json 1.0.150 added to Cargo.toml

### Step 3: RED — Tests Failing (Before Implementation)

Command:
```bash
cargo test -p muxsmith-core report
```

Output (abbreviated):
```
error[E0433]: cannot find type `Severity` in this scope
 --> crates/muxsmith-core/src/report.rs:7:17

error[E0433]: cannot find type `DiagCode` in this scope
 --> crates/muxsmith-core/src/report.rs:13:20

error[E0433]: cannot find type `Diagnostic` in this scope
 --> crates/muxsmith-core/src/report.rs:20:17
```

**Result:** FAIL — All 5 tests failed to compile (types undefined).

### Step 4: Implementation

Implemented in `crates/muxsmith-core/src/report.rs`:
- `Severity` enum: Info, Warning, Error (ordered via `Ord` derive)
- `DiagCode` enum: 28 variants (config-time + planning-time) via macro, `.key()` method returns `&'static str`
- `Diagnostic` struct: code, severity, config_path, optional file, params (BTreeMap), optional suggestion_ref
- Builder methods: `::error()`, `::warning()`, `::info()`, `.with()`, `.for_file()`
- `worst_severity()` function: takes slice, returns max severity or None

Serialization: `#[serde(rename_all = "snake_case")]` on Severity, `#[serde(rename_all = "kebab-case")]` on DiagCode enum variants.

### Step 5: GREEN — Tests Passing (After Implementation)

Command:
```bash
cargo test -p muxsmith-core report
```

Output:
```
running 5 tests
test report::tests::diag_code_keys_are_kebab_case ... ok
test report::tests::severity_orders_info_warning_error ... ok
test report::tests::diagnostic_builder_sets_fields ... ok
test report::tests::worst_severity_picks_error_over_warning ... ok
test report::tests::diagnostic_serializes_with_snake_case_severity_and_kebab_code ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Result:** PASS — All 5 tests pass. Full workspace test also passes (0 failures).

---

## Verification Against Brief

| Interface | Status | Evidence |
|-----------|--------|----------|
| `Severity` with Ord (Info < Warning < Error) | ✓ | `#[derive(Ord, PartialOrd)]`; test_severity_orders passes |
| `DiagCode` enum + `.key()` returning kebab-case | ✓ | 28 variants via macro; test_diag_code_keys_are_kebab_case passes |
| `Diagnostic` struct (code, severity, config_path, file, params, suggestion_ref) | ✓ | All fields defined; test_diagnostic_builder_sets_fields passes |
| Builders: `.error()`, `.warning()`, `.info()` | ✓ | All three methods implemented; builders tested |
| Chaining: `.with()`, `.for_file()` | ✓ | Implemented as consuming methods; test_diagnostic_builder_sets_fields verifies chaining |
| `worst_severity(&[Diagnostic])` → Option<Severity> | ✓ | Uses `.max()` on severity iterator; test passes |
| Serialization: snake_case severity, kebab-case code | ✓ | `#[serde(...)]` attributes applied; test_diagnostic_serializes passes |

---

## Artifact Changes

**Files modified:**
- `crates/muxsmith-core/Cargo.toml`: serde (with derive), serde_json (dev)
- `crates/muxsmith-core/src/report.rs`: 266 insertions (full implementation + 5 tests)

**No breaking changes.** Cargo.lock updated; all existing tests still pass.

---

## Notes

- DiagCode catalog stable for future tasks (Plan 2 planning-time codes included now).
- Macro-based code generation avoids manual impl duplication.
- BTreeMap for params ensures deterministic JSON serialization.
- Optional fields (file, suggestion_ref) correctly skipped when None via `#[serde(skip_serializing_if)]`.

---

## Fix: Reviewer follow-up (key/serde consistency enforcement)

**Risk flagged:** `DiagCode::key()` literals and `#[serde(rename_all = "kebab-case")]` are two independent sources of the wire string; nothing enforced their agreement.

**Changes** (`crates/muxsmith-core/src/report.rs`):
- `diag_codes!` macro now also emits `pub const ALL: &'static [DiagCode]` (exhaustive, generated from the same variant list).
- New test `all_keys_match_serde_encoding`: for every code in `DiagCode::ALL`, `serde_json::to_value(code)` must equal `Value::String(code.key())`.
- New test `all_keys_are_unique`: keys collected into a `BTreeSet` must have the same length as `DiagCode::ALL`.

**Commands:**
```bash
cargo test -p muxsmith-core report   # 7 passed; 0 failed
cargo test --workspace               # 7 passed; 0 failed (core), 0 elsewhere
```

**Commit:** `a7c0d89 test(core): enforce DiagCode key/serde consistency and uniqueness`
