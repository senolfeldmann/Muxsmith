# Task 12: Fluent catalogs, renderer, and `validate` subcommand — Report

## Status
COMPLETE. All TDD steps executed; `validate` subcommand fully implemented; i18n renderer is the sole source of human text on the CLI side.

## Adaptations from the brief
- `render.rs` intentionally **not created** (brief's own note); rendering lives in `i18n.rs` (`Renderer`) and `commands/validate.rs` (assembly/printing).
- fluent-bundle resolved to `0.16.0`, unic-langid to `0.9.6`, sys-locale to `0.3.2`. The brief's `i18n.rs` code compiled **verbatim, no API adaptation needed** (`FluentBundle::new`, `add_resource_overriding`, `get_message`/`.value()`/`format_pattern`, `FluentArgs::set` all matched as written).
- Ran `cargo fmt -p muxsmith-cli` after implementation; it reordered one `use` statement (`Diagnostic, Severity, worst_severity`) and reflowed two multi-line expressions in `i18n.rs`. No semantic change.

## Files created
- `locales/en/diagnostics.ftl` — all 30 `DiagCode::key()` messages + 3 `severity-*` keys (33 total), copied verbatim from the brief.
- `locales/en/cli.ftl` — `validate-ok`, `validate-summary`, `diagnostic-line`.
- `crates/muxsmith-cli/src/i18n.rs` — `Renderer::new`, `Renderer::msg`, `Renderer::diagnostic`.
- `crates/muxsmith-cli/src/commands/mod.rs`, `crates/muxsmith-cli/src/commands/validate.rs`.
- `crates/muxsmith-cli/tests/fixtures/good.yaml`, `bad.yaml`.
- `crates/muxsmith-cli/tests/cli_validate.rs`.

## Files modified
- `crates/muxsmith-cli/Cargo.toml`: added `fluent-bundle`, `unic-langid`, `sys-locale` deps; `tempfile` dev-dep.
- `crates/muxsmith-cli/src/main.rs`: wired `commands` + `i18n` modules; `Validate` arm builds a `Renderer` and delegates to `commands::validate::run`.
- `Cargo.lock` (dependency resolution).

## TDD evidence

**Step 4 (tests written first, confirmed failing against the stub):**
```
test missing_file_is_parse_error_exit_two ... ok
test warnings_only_exits_one ... FAILED
test invalid_profile_exits_two_and_renders_messages ... FAILED
test json_output_is_machine_readable ... FAILED
test valid_profile_exits_zero_with_ok_message ... FAILED

failures:
    invalid_profile_exits_two_and_renders_messages
    json_output_is_machine_readable
    valid_profile_exits_zero_with_ok_message
    warnings_only_exits_one

test result: FAILED. 1 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
```
(4 of 5 failed as expected; the stub's unconditional exit-2 accidentally satisfied only the missing-file case.)

**Step 6, focused (after implementation):**
```
running 5 tests
test missing_file_is_parse_error_exit_two ... ok
test invalid_profile_exits_two_and_renders_messages ... ok
test json_output_is_machine_readable ... ok
test warnings_only_exits_one ... ok
test valid_profile_exits_zero_with_ok_message ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**`cargo test -p muxsmith-cli` (all CLI tests, incl. Task 11's schema tests):**
```
tests/cli_schema.rs:   2 passed; 0 failed
tests/cli_validate.rs: 5 passed; 0 failed
```

**Step 7, `cargo test --workspace`:**
```
muxsmith-cli   unittests:            0 passed
cli_schema.rs:                       2 passed; 0 failed
cli_validate.rs:                     5 passed; 0 failed
muxsmith-core  unittests (lib.rs):   34 passed; 0 failed
profile_load.rs:                     6 passed; 0 failed
validate_semantics.rs:               14 passed; 0 failed
validate_structure.rs:               10 passed; 0 failed
xtask          unittests x2:         0 passed
codegen.rs:                          2 passed; 0 failed

TOTAL: 73 passed; 0 failed across the workspace
```

## Self-review

- **No hardcoded prose**: grepped `commands/validate.rs` and `main.rs` for `println!`/`unwrap()`; every `println!` argument is either `renderer.msg(...)`, `renderer.diagnostic(...)`, or `serde_json` structured output. No literal user-facing string anywhere in `muxsmith-cli` source.
- **Isolation marks off**: verified real CLI output (`validate bad.yaml`) byte-for-byte for the Unicode isolate range U+2066-U+2069 — none present. `set_use_isolating(false)` confirmed effective.
- **Exit codes**: manually re-ran all three cases outside the test harness:
  - `good.yaml` -> `Profile is valid.`, exit 0
  - a warnings-only profile -> renders the overlap warning + summary line, exit 1
  - `bad.yaml` -> 2 errors + 1 warning rendered, exit 2
  - unknown `--locale de-DE` falls back to `en` silently (no crash, no `en` leakage in output)
- **`--json`** verified manually: `{"diagnostics": [...]}`, each entry has `code`, `severity`, `params`, and the added `rendered` string.
- **Catalog completeness**: `diagnostics.ftl` has exactly 33 `key = value` lines (30 `DiagCode` keys + 3 `severity-*`), matching `DiagCode::ALL`'s current length.
- **ASCII quotes only**: grepped both `.ftl` files for the Unicode curly-quote range (U+2018/2019/201C/201D) — none found.
- Pre-existing `clippy::collapsible_if` warning in `muxsmith-core/src/profile/validate.rs` is unrelated to this task (not touched here), left as-is.

## Concerns
None. The brief's code compiled against the resolved dependency versions without modification; only whitespace/import-order changes from `cargo fmt` were applied.

## Commit
```
51540e1 feat(cli): validate subcommand with Fluent-rendered diagnostics and exit codes
```

## Review fixes (post-review round)

**1. JSON/text ordering inconsistency (Important).** The `--json` branch emitted diagnostics in raw insertion order while the text branch sorted error-first. Fixed in `commands/validate.rs`: the vector is now sorted once (stable `sort_by_key` on `Reverse(severity)`) directly after `collect`, before branching; both JSON entries and printed lines derive from the same sorted vector. The redundant per-branch `sorted` clone is gone. Test `json_output_is_machine_readable` now additionally asserts `diags[0]["severity"] == "error"` (bad.yaml produces 2 errors + 1 warning, so sorted output must lead with an error). Verified against the real binary: JSON severities for bad.yaml are `['error', 'error', 'warning']`.

**2. Renderer safety-net unit tests (Minor).** Added `#[cfg(test)] mod tests` to `i18n.rs`:
- `unknown_message_id_falls_back_to_raw_id`: `renderer.msg("no-such-id", &[]) == "no-such-id"`.
- `invalid_locale_falls_back_to_en_and_renders`: `Renderer::new(Some("zz-ZZ-invalid!"))` renders `validate-ok` as "Profile is valid." without panic.

Deliberately NOT added here: the catalog-completeness CI guard (all `DiagCode::ALL` keys present in diagnostics.ftl) is Task 13's deliverable; the "CI guards this case" comment in `i18n.rs` stays as-is.

**Test results after fix:**
```
cargo test -p muxsmith-cli:
  unittests src/main.rs:  2 passed; 0 failed   (new i18n fallback tests)
  tests/cli_schema.rs:    2 passed; 0 failed
  tests/cli_validate.rs:  5 passed; 0 failed

cargo test --workspace:  75 passed; 0 failed total
  (cli 2+2+5, core 34+6+14+10, xtask 2)
```

**Fix commit:**
```
ad841b0 fix(cli): sort diagnostics before json output; renderer fallback tests
```
