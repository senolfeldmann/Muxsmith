# Task 11: CLI scaffold and `schema` subcommand — Report

## Status
✓ COMPLETE. All TDD steps executed; CLI subcommands working; tests passing.

## Commits
```
bcf078b feat(cli): clap scaffold and schema subcommand
```

## Test Results

### Focused test (cli_schema)
```
running 2 tests
test no_args_shows_usage_and_fails ... ok
test schema_prints_json_schema_and_exits_zero ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Full workspace
```
test result: ok. 78 passed; 0 failed (across all crates)
```

## Implementation Verification

**Files created/modified:**
- `crates/muxsmith-cli/Cargo.toml`: dependencies added (clap, serde_json, schemars, assert_cmd, predicates)
- `crates/muxsmith-cli/src/cli.rs`: CLI scaffold with `Cli` struct and `Cmd` enum (Validate + Schema subcommands)
- `crates/muxsmith-cli/src/main.rs`: clap parser integration; `schema` prints JsonSchema of Profile; `validate` returns exit code 2 (stub for Task 12)
- `crates/muxsmith-cli/tests/cli_schema.rs`: test suite verifying schema output and no-args failure

**Design notes:**
- `schema` subcommand outputs JSON Schema to stdout via `schemars::schema_for!` macro and `serde_json::to_string_pretty`.
- `validate` intentionally exits 2 without output; Task 12 will add i18n renderer (hardcoded prose is banned per brief).
- clap's own usage/help text is library-generated (not prose we author).
- Test verifies schema contains `"profile_version"` and `"tracks"` fields.

## Concerns
None. Brief followed exactly; all tests pass; implementation matches specification verbatim.
