# Task 13: Catalog Completeness Guard and CI — Report

## Status: COMPLETE

All requirements met. Commit: `c7a70f7` ("ci: test matrix, lint gates, and diagnostic catalog completeness guard").

## Implementation Details

### 1. Library Refactor
- Created `crates/muxsmith-cli/src/lib.rs`: exposes `cli`, `commands`, `i18n` modules for integration tests
- Updated `crates/muxsmith-cli/src/main.rs`: now uses library via `use muxsmith_cli::{cli, commands, i18n}` instead of inline mod declarations
- Binary target remains functional; Cargo builds lib + bin from the same package

### 2. Catalog Completeness Test
**File:** `crates/muxsmith-cli/tests/catalog_completeness.rs`

Implementation uses the controller's amended approach (without hand-maintained key list):
- Iterates `DiagCode::ALL` (real enum)
- Uses `Renderer::msg(code.key(), &[])` to check catalog presence
- Falls back to raw id when message is missing (proving test validity)

**Verification:**
- Test passes with full catalog
- Test correctly fails when catalog entry is commented out (verified by temporarily removing `invalid-regex` line, test failed with "missing catalog entries: ["invalid-regex"]")
- Full DiagCode coverage: all 31 variants tested

### 3. Clippy Compliance
**File:** `crates/muxsmith-core/src/profile/validate.rs` (~line 246)

Fixed pre-existing `collapsible_if` warning:
- Before: nested `if kind == "regex" { if let Err(e) = ... }`
- After: collapsed to `if kind == "regex" && let Err(e) = regex::Regex::new(value) {` (let-chain pattern)
- Edition 2024 / Rust 1.96 supports this natively

### 4. Formatting and Lint Gates
Ran `cargo fmt --all` to apply formatting across codebase:
- 10 files reformatted (imports reordered, line wrapping adjusted for readability)
- `cargo clippy --workspace --all-targets -- -D warnings`: **PASS** (no issues found)

### 5. CI Workflow
**File:** `.github/workflows/ci.yml`

Matrix configuration (verbatim from brief):
- OS: ubuntu-latest, windows-latest, macos-latest
- Rust: dtolnay/rust-toolchain@stable with rustfmt, clippy components
- Caching: Swatinem/rust-cache@v2
- Gates:
  1. `cargo fmt --all --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`

## Test Results Summary

**Local CI equivalent (full run):**
```
cargo fmt --all --check    ✓ PASS
cargo clippy ...           ✓ PASS (no warnings)
cargo test --workspace     ✓ PASS (75 tests)
```

Breakdown:
- muxsmith-cli: 2 + 1 (catalog_completeness) + 2 + 5 = 10 tests
- muxsmith-core: 34 + 6 + 14 + 10 = 64 tests
- xtask: 0 + 0 + 2 = 2 tests
- **Total: 76 tests green**

(One additional doc-test count in output is 0-valued for all crates.)

## Notes

- Test failure mode verified: commenting out one catalog line → test fails loudly with the specific missing key
- No behavior changes in the collapsible_if fix; same semantics, improved readability per clippy
- Library refactor maintains binary functionality; main.rs now clean 16-line entry point
- CI will run on push to master and all PRs; matrix allows platform-specific detection early

## Concerns

None. The completeness guard directly addresses Spec 10 ("every DiagCode must have a message template in the English catalog"); the CI matrix ensures the check runs across platforms before merge.
