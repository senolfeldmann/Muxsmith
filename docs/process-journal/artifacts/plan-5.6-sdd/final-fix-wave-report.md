# Plan 5.6 final fix wave: report

Ground truth: `whole-branch-verdict.md` funnel item 1 (MUST-FIX) + four
zero-risk riders from the Assessment section. Applied directly on `master`,
no other file touched.

## MUST-FIX: D36 wire-contract test

**Location:** `crates/muxsmith-core/src/report/mod.rs`, `#[cfg(test)] mod
tests`, immediately before `all_keys_match_serde_encoding`.

**Rationale for placement:** `report/mod.rs` already carries the house
wire-shape idiom for `Diagnostic` at
`diagnostic_serializes_with_snake_case_severity_and_kebab_code` (builds a
`Diagnostic`, calls `serde_json::to_value`, asserts on the resulting
`serde_json::Value` fields) - the new test is the same idiom applied to
`with_claimants`. `report/json.rs` (the other `rendered_diags`/core-85 call
site) carries no unit tests at all; `mod.rs`'s existing test module is the
sibling wire-shape suite, not a new one.

**Test added** (`with_claimants_populates_structural_field_and_json_from_one_slice`):

```rust
let d = Diagnostic::error(DiagCode::OverlappingRules, "tracks[0]").with_claimants(&[0, 2]);
assert_eq!(d.claimants, vec![0, 2]);
assert_eq!(d.params["rules"], "tracks[0], tracks[2]");

let json = serde_json::to_value(&d).unwrap();
assert_eq!(json["claimants"], serde_json::json!([0, 2]));

let without = Diagnostic::error(DiagCode::InvalidRegex, "input.pattern");
let json_without = serde_json::to_value(&without).unwrap();
assert!(
    json_without.as_object().unwrap().get("claimants").is_none(),
    "claimants must be omitted via skip_serializing_if when empty"
);
```

Covers all three asked-for assertions: (1) `with_claimants` populates both
the structural `claimants` field and the rendered `rules` param from one
slice; (2) `claimants` present in JSON for an `OverlappingRules`-shaped
diagnostic; (3) `claimants` key absent (not `null`, genuinely missing from
the object) for a diagnostic that never called `with_claimants`, proving
`skip_serializing_if` actually elides it rather than serializing an empty
array.

## Riders

**R1 - `src-tauri/src/run.rs::start_run`:** collapsed the two-step
`let outcome = ...await; let outcome = outcome?;` into one
`let outcome = on_blocking(...).await?;`. Added `use crate::on_blocking;`
alongside the file's other `use crate::...` imports (`AppState`,
`ShellRenderer`, `error::IpcError`) and dropped the `crate::` qualifier at
the call site, matching `lib.rs`'s own unqualified calls to the function it
defines.

**R2 - `.github/workflows/ci.yml`:** added `shell: bash` to the "Install
pinned Rust toolchain" step (the only change). Closes the pwsh
partial-failure-masking gap on Windows: bash's `run: |` blocks execute
under `set -e`, so a failed `rustup toolchain install` now fails the step
immediately instead of being masked by a later `rustup component add`
succeeding.

**R3 - `crates/muxsmith-cli/src/commands/validate.rs`:** confirmed
`collect()` had exactly one caller (`run`), inlined
`validate::config_diagnostics_from_file(profile_path)` at that call site,
and deleted the now-empty wrapper function.

All four are pure structural/diagnosability changes; none alter rendered
output, exit codes, JSON shape, or CI job semantics beyond the stated
Windows-diagnosability fix.

## Nine-part gate (this HEAD, foreground, mkvmerge on PATH via
`/home/linuxbrew/.linuxbrew/bin/mkvmerge`)

| # | Check | Result |
|---|-------|--------|
| 1 | `cargo fmt --all --check` | clean |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| 3 | `cargo test --workspace` | all green, incl. new `report::tests::with_claimants_populates_structural_field_and_json_from_one_slice` |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean |
| 5 | `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| 6 | `pnpm lint` | clean |
| 7 | `pnpm build` | vue-tsc + vite build clean |
| 8 | `pnpm check:i18n` | ok (12 pre-existing unused-key warnings, unrelated to this wave, non-blocking) |
| 9 | `pnpm test:e2e` | 7/7 passed |

## Commit

Single commit, unsigned, explicit staging of the four touched files:
`.github/workflows/ci.yml`, `crates/muxsmith-cli/src/commands/validate.rs`,
`crates/muxsmith-core/src/report/mod.rs`, `src-tauri/src/run.rs`.
