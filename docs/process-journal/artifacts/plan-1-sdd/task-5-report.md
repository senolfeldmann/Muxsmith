# Task 5 Report: Capability Code Generator (xtask)

## Summary

**Status:** COMPLETE ✓

**Commit:** `830dc47` — feat(xtask): capability table generator from identification schema

**TDD Evidence:** All tests pass (2 integration tests in `crates/xtask/tests/gen.rs`)

## Changes

- Added `crates/xtask` to workspace members in root `Cargo.toml`
- Created `crates/xtask/Cargo.toml` with lib configuration and `serde_json` dependency
- Created `crates/xtask/src/lib.rs` (exports `gen` module)
- Created `crates/xtask/src/gen.rs` (pure `generate(schema_json) -> Result<String, String>` function)
- Created `crates/xtask/src/main.rs` (CLI: `cargo run -p xtask -- gen-capability <schema.json> <out.rs>`)
- Created `crates/xtask/tests/fixtures/mini-schema.json` (synthetic test fixture)
- Created `crates/xtask/tests/gen.rs` (2 tests)

## Edition Override Note

The brief's code uses `pub mod gen;` which conflicts with Rust 2024's reserved keyword `gen`. **Workaround:** Set xtask to edition `2021` explicitly in `Cargo.toml` (overriding workspace default). This is the minimal change required to make the brief's code compile and run verbatim.

## Test Results

### xtask tests (focused):
```
running 2 tests
test rejects_schema_without_track_properties ... ok
test generates_matchable_table_from_schema ... ok

test result: ok. 2 passed; 0 failed
```

### Full workspace tests:
```
test result: ok. 20 passed; 0 failed (muxsmith-core: 12 + muxsmith-cli: 0 + xtask: 2 + profile_load: 6)
```

## Verification

Both integration tests pass:

1. **`generates_matchable_table_from_schema`** — Verifies that:
   - Track-level fields (`type`, `codec`, `id`) are injected
   - Nested properties are extracted from schema
   - Output contains `GENERATED FILE` marker
   - PropType enum is referenced correctly

2. **`rejects_schema_without_track_properties`** — Verifies that:
   - Empty schema `"{}"` is rejected with `Err`

Generated output (example from test):
```rust
// GENERATED FILE - do not edit.
// Regenerate: cargo run -p xtask -- gen-capability <schema.json> <this file>
// Source: mkvmerge identification output schema (facts only, not the schema).

use super::PropType;

pub static MATCHABLE_PROPERTIES: &[(&str, PropType)] = &[
    ("audio_channels", PropType::Integer),
    ("codec", PropType::String),
    ("default_track", PropType::Boolean),
    ("display_dimensions", PropType::String),
    ("forced_track", PropType::Boolean),
    ("id", PropType::Integer),
    ("language", PropType::String),
    ("track_name", PropType::String),
    ("type", PropType::String),
];
```

## Ready for Task 6

The generator is ready to consume the real mkvmerge identification schema. Task 6 will run:
```bash
cargo run -p xtask -- gen-capability <schema.json> crates/muxsmith-core/src/capability/generated.rs
```

## Fix: module renamed gen -> codegen (edition-2024)

**Commit:** `e78847d` — fix(xtask): rename gen module to codegen for edition-2024 compatibility

The initial workaround (edition 2021 override for xtask) forked the workspace edition to preserve a module name nothing depends on. Root-cause fix per coordinator review:

- Renamed `crates/xtask/src/gen.rs` -> `codegen.rs`, `crates/xtask/tests/gen.rs` -> `codegen.rs`
- `src/lib.rs`: `pub mod codegen;`; test and `main.rs` now use `xtask::codegen::generate`
- `crates/xtask/Cargo.toml`: restored `edition.workspace = true` (whole workspace uniformly edition 2024)
- No comment/usage strings mentioned `gen.rs`; the `gen-capability` CLI subcommand is the interface Task 6 depends on and is unchanged

Test results after fix:

`cargo test -p xtask`:
```
     Running tests/codegen.rs (target/debug/deps/codegen-68ee51dd799b6bcc)

running 2 tests
test rejects_schema_without_track_properties ... ok
test generates_matchable_table_from_schema ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo test --workspace`: 20 passed, 0 failed (muxsmith-core 12 + profile_load 6 + xtask 2), all suites `ok`.
