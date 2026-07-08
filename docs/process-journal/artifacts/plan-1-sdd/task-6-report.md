# Task 6 Report: Capability module with generated table

## Status
COMPLETE

## Work Summary

1. **Schema Download:** Downloaded identification schema v20 (v21 returned 404; tried v21, v20, v19, v18)
2. **Code Generation:** Ran xtask generator → generated `crates/muxsmith-core/src/capability/generated.rs`
3. **Module Implementation:** Wrote `capability/mod.rs` with:
   - `PropType` enum (String, Boolean, Integer, Float)
   - `matchable_type()` function + virtual `codec_kind` property
   - `SETTABLE` curated list + `settable()` function
   - `CODEC_KINDS` aliases + `codec_kind_prefixes()` function
   - `ATTACHMENT_PROPERTIES` constant
4. **Integration:** Added `pub mod capability;` to `lib.rs`
5. **Testing:** All 4 tests pass; full `cargo test --workspace` passes (no regressions)
6. **Commit:** Generated files committed; schema not in tree

## Commit Details

- **Commit:** `4750abb`
- **Message:** `feat(core): capability model with generated matchable table and curated settable set`
- **Files:**
  - ✓ `crates/muxsmith-core/src/capability/generated.rs` (generated, committed)
  - ✓ `crates/muxsmith-core/src/capability/mod.rs` (module with tests, committed)
  - ✓ `crates/muxsmith-core/src/lib.rs` (module export added)
  - ✗ `/tmp/mkvmerge-schema.json` (correctly not committed)

## Test Results

```
running 4 tests
test capability::tests::attachment_properties_are_defined ... ok
test capability::tests::codec_kind_is_virtual_matchable ... ok
test capability::tests::matchable_types_from_generated_table ... ok
test capability::tests::settable_maps_to_mkvmerge_options ... ok

test result: ok. 4 passed; 0 failed
```

Full workspace test suite: **All pass** (12 tests total in muxsmith-core + xtask integration tests)

## Schema Version

Generated from identification schema **v20** (recorded in `capability/mod.rs` comment line 6)

## Concerns

None. All tests pass, generated output is valid, module integrates cleanly into lib.rs.
