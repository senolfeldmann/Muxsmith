# Task 1 report: D38 core - conditional NoTrackRules, PassthroughProfile diagnostic, catalogs, e2e

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-a`, branch `plan58-a`. All steps executed in order; no scope beyond the brief's file list.

## Step 1: failing validate tests

Added `zero_rules_with_unmatched_keep_is_a_passthrough_info` and `zero_rules_with_unmatched_drop_stays_an_error` to `crates/muxsmith-core/tests/validate_semantics.rs`, verbatim per the brief, directly after `empty_tracks_list_is_rejected` (which stays untouched, pinning the default-drop shape).

## Step 2: verify failure

```
$ cargo test -p muxsmith-core --test validate_semantics
error[E0599]: no variant, associated function, or constant named `PassthroughProfile` found for enum `DiagCode`
  --> crates/muxsmith-core/tests/validate_semantics.rs:74:39
error[E0599]: no variant, associated function, or constant named `PassthroughProfile` found for enum `DiagCode`
  --> crates/muxsmith-core/tests/validate_semantics.rs:91:44
error: could not compile `muxsmith-core` (test "validate_semantics") due to 2 previous errors
```
Matches the brief's expected compile error exactly.

## Step 3: DiagCode variant + NoTrackRules rustdoc reword

`crates/muxsmith-core/src/report/mod.rs`: reworded the `NoTrackRules` doc comment and added `PassthroughProfile => "passthrough-profile"` immediately after it, verbatim per the brief.

## Step 4: conditional validate

`crates/muxsmith-core/src/profile/validate.rs`: added `KeepDrop` to the existing `use super::model::{...}` import, and replaced the unconditional `NoTrackRules` push with the `match profile.tracks.unmatched { Drop => ..., Keep => ... }` block, verbatim per the brief.

## Step 5: bilingual catalog entries

`locales/en/diagnostics.ftl` and `locales/de/diagnostics.ftl`: replaced the `no-track-rules` line and inserted the new `passthrough-profile` line directly after it in both files, verbatim per the brief and the decisions doc's approved wording.

## Step 6: catalog_completeness fixture

`crates/muxsmith-cli/tests/catalog_completeness.rs`: added `DiagCode::PassthroughProfile => vec![],` directly after the `NoTrackRules` arm in the exhaustive `fixture_args` match.

## Step 7: unit layers

```
$ cargo test -p muxsmith-core --test validate_semantics
running 21 tests
test zero_rules_with_unmatched_keep_is_a_passthrough_info ... ok
test zero_rules_with_unmatched_drop_stays_an_error ... ok
... (19 more)
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p muxsmith-cli --test catalog_completeness
running 4 tests
test every_diag_code_has_a_catalog_message ... ok
test every_diag_code_renders_without_leftover_placeholders ... ok
test every_cli_ftl_key_is_a_diag_code_or_allowlisted ... ok
test invalid_changes_language_diagnostic_renders_without_placeholder_leak ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Step 8: gated e2e passthrough test - adaptations made

Read `crates/muxsmith-cli/tests/run_live.rs` and `dry_run_cli.rs` in full before writing. Reused the file's existing `muxsmith()` and `have_mkvmerge()` helpers and inserted the new test directly before `backdate_mtime`/the rerun test (kept the two pre-existing tests untouched). Three deliberate adaptations from the brief's literal draft, all within the "adapt harness idioms" mandate and none touching the fixed contract (exit codes, the two diagnostic-code checks, recognized container, track count 2):

1. **`report["diagnostics"]` -> `report["config_diagnostics"]`.** The brief's draft snippet reads `report["diagnostics"]`, but that field does not exist at the JSON top level. `dry_run_cli.rs` (e.g. `dry_run_surfaces_config_time_invalid_regex`, `dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing`) establishes the real shape: config-time (`validate()`-produced) diagnostics live under top-level `config_diagnostics`; per-file diagnostics live under `files[i]["diagnostics"]`. `PassthroughProfile`/`NoTrackRules` are both emitted by `validate()`, i.e. config-time, so the correct field is `config_diagnostics`. Using the brief's literal field name would have produced a passing-looking test that silently checks an always-empty/absent array (`.unwrap()` on a null array would in fact panic, so this would have been caught immediately - but it is exactly the kind of drift the "read the neighbors first" instruction exists to prevent). Added a comment in the test explaining the field choice with a cross-reference to the sibling test that established it.
2. **Directory scan -> direct filename join for the run output.** The brief's draft globs `outdir` for the first `.mkv` file found. The two pre-existing tests in the same file (and `output.filename` defaults to `keep`, per spec 4.8: file_stem + `.mkv`) instead compute the expected path directly as `output_dir.join("Show.S01E01.mkv")`, exactly as the module doc comment states and as `live_run_muxes_two_sources_and_reports_exit_zero` does for its two outputs. Switched to that direct join to match the local idiom instead of introducing a new directory-scan pattern for one test.
3. **Added `MUXSMITH_RUNS_ROOT` env var on the `run` invocation.** The brief's draft omits it, but both existing `run_live.rs` tests set `.env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))` on every real `run` invocation, with an explicit comment citing Task 6 (D26): a real mux reaches the queue and would otherwise persist job logs into the real platform data dir. Omitting it would have made the new test the one live-run test in the file that pollutes the real platform data directory on every CI/dev run. Added it with the same comment, for the same reason.

No other departure from the brief's assertions: same two `dry-run` diagnostic-code checks (`passthrough-profile` present, `no-track-rules` absent), same `run` exit-code check, same `mkvmerge -J` container-recognized + track-count-2 checks on the produced output.

## Step 9: e2e test run (foreground, real mkvmerge)

`mkvmerge --version` on this machine: `mkvmerge v100.0`, at `/home/linuxbrew/.linuxbrew/bin/mkvmerge`.

```
$ cargo test -p muxsmith-cli --test run_live zero_rule_keep -- --nocapture
running 1 test
test zero_rule_keep_profile_is_a_pure_passthrough ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in 0.28s
```
No `mkvmerge not found; skipping` marker on stderr (checked explicitly) - the test ran the real mkvmerge path, not the self-skip branch. Confirmed again in the full-workspace run below, where all 3 tests in `run_live.rs` (not filtered) execute and pass, listing the new test by name.

## House pre-commit gate (process-conventions.yaml, "cargo test --workspace, cargo fmt --all --check, cargo clippy --workspace --all-targets -D warnings and cargo deny check must all pass before every commit")

Ran all four, foreground, before staging:

```
$ cargo fmt --all --check
(no output - clean)

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 22.04s
(zero warnings across muxsmith-core, muxsmith-cli, muxsmith-gui, xtask)

$ cargo test --workspace
... every test binary reports "test result: ok. N passed; 0 failed"
     Running tests/run_live.rs
running 3 tests
test live_run_muxes_two_sources_and_reports_exit_zero ... ok
test zero_rule_keep_profile_is_a_pure_passthrough ... ok
test live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok
(exit code 0)
```

Note on invocation: the process-conventions.yaml wording `cargo clippy --workspace --all-targets -D warnings` is not a literal runnable command (clippy needs `-- -D warnings` to pass the flag through to rustc); ran it as `cargo clippy --workspace --all-targets -- -D warnings`, which is the correct spelling of the same intent. Surfacing this as a minor documentation imprecision, not a deviation from the rule's substance.

## Commit

Staged exactly the brief's seven files, committed unsigned with the exact message and trailer:

```
[plan58-a f8e863e] feat: legalize zero-rule keep passthrough with PassthroughProfile info (D38)
 7 files changed, 179 insertions(+), 5 deletions(-)
```
`git status` after commit: working tree clean. No push performed.

## Surfaced items (deviations / concerns / new patterns)

- **Step 8 field-name correction is the only substantive adaptation**; documented in the test file itself via a comment, and above. This is not a deviation from the brief's *contract* (the two diagnostic-code assertions), only from its literal-but-incorrect draft field path.
- **Nothing else new**: no new abstraction, no new test helper duplicated, no pattern invented outside what `docs/conventions.yaml` / the file's existing idioms already establish. The `KeepDrop` match added to `validate.rs` follows the same shape as every other `match profile.tracks.unmatched`/`match profile.attachments.unmatched` site already in the codebase (`planner.rs`).
- **Scope check**: this task's file list, diagnostic wire shape (code + `tracks.rules` config_path + info severity, no params), and catalog wording all match the decisions doc (D38) and brief exactly; no wire/interface surprises for Task 2 to react to.

## Status

DONE

Commit: `f8e863e4660a5ab400155fc8e8cd6a08265cb762` (branch `plan58-a`, worktree `/home/senol/Git/Muxsmith/.worktrees/plan58-a`)

Test summary: `cargo test --workspace` all green (every binary "0 failed"); the gated e2e `zero_rule_keep_profile_is_a_pure_passthrough` ran for real against mkvmerge v100 (not self-skipped) and passed; `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check` all clean.
