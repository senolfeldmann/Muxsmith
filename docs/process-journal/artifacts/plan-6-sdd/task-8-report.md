# Task 8 report: D42 - the editor's IPC surface

**Branch:** plan6-w2, worktree `/home/senol/Git/Muxsmith/.worktrees/plan6-d`
**Commit:** `997666a` "shell: load/save/validate-model/apply commands and their error codes (D42, D43, D49)"

## What was implemented

Four new Tauri commands on `src-tauri/src/lib.rs`'s `invoke_handler`, plus their error mappings and bilingual catalog entries:

| command | signature | on_blocking reason |
|---|---|---|
| `load_profile` | `async fn(path: String) -> Result<serde_json::Value, IpcError>` | disk (load::from_file) |
| `save_profile` | `async fn(path: String, profile: Profile) -> Result<(), IpcError>` | disk (save::to_file) |
| `validate_profile_model` | `async fn(profile: Profile) -> Result<serde_json::Value, IpcError>` | CPU-bound on every keystroke (config_diagnostics compiles regexes/templates), NOT disk |
| `apply_suggestion` | `async fn(profile: Profile, config_path: String, edit: StructuredEdit) -> Result<Profile, IpcError>` | same CPU-bound-per-edit reason as validate_profile_model |

Each command is a thin `on_blocking` wrapper around a testable free function (`load_profile_body`, `save_profile_body`, `validate_profile_model_body`, `apply_suggestion_body`), mirroring the file's existing pattern for `validate_profile`/`dry_run`/`identify`/`detect_mkvmerge` (all four already had `_body` twins before this task; the brief's Step 3 prose names only `load_profile_body` explicitly, but I gave the other three the same split for consistency with the dominant local pattern -- flagged below as a minor judgment call, not a design fork, since it changes no external behavior, wire format, or test).

`load_profile_body(path: &Path) -> serde_json::Value` returns **no bespoke struct**: on `load::from_file(path)` -> `Ok(profile)`, builds `report::json::config_only_document(&validate::config_diagnostics(&profile), None, &ShellRenderer)` and injects `doc["profile"] = serde_json::to_value(&profile).expect(...)`; on `Err(d)` (a `ParseError` diagnostic), builds the document from `&[d]` and sets `doc["profile"] = Value::Null`.

`validate_profile(path)` (the pre-existing command) is untouched -- confirmed via `git diff` that no line in its body, doc comment, or registration changed except the surrounding module doc's task count.

`error.rs` gained two `From` impls:
- `From<SaveError> for IpcError`: `Io` -> `profile-save-io-failed` (+ `detail`), `Serialize` -> `profile-save-failed` (+ `detail`).
- `From<ApplyError> for IpcError`: verbatim from D49 :597-616 (`UnparsableConfigPath` -> `apply-unparsable-config-path` + `path`; `RuleIndexOutOfRange` -> `apply-rule-index-out-of-range` + `index`/`rules` as `.to_string()`; `EditChangedNothing` -> `apply-edit-changed-nothing` + `index`/`property`).

Five bilingual catalog entries landed in `locales/{en,de}/gui-common.ftl`, byte-diffed against the brief's text after stripping the added `## D42/D43` en-only header comment (de doesn't carry per-block `##` headers anywhere in this file, matching its existing convention) -- confirmed identical with a `diff` against a scratch copy of the brief's exact lines, both locales.

## TDD evidence

**RED** (`cargo test -p muxsmith-gui --lib`, after adding only the tests from Step 1, before any implementation): 7 compile errors -- `From<SaveError>`/`From<ApplyError>` unsatisfied trait bounds in `error.rs`'s two new tests, `load_profile_body` not found (2 call sites) in `lib.rs`'s new test.

**GREEN** (same command, after Step 3 implementation): `test result: ok. 81 passed; 0 failed; 0 ignored`. Includes `save_errors_map_to_distinct_codes`, `apply_errors_map_to_distinct_codes`, and `load_profile_body_matches_validate_profile_diagnostics_and_adds_the_model` (asserts both halves: the loadable-invalid profile's `config_diagnostics`/`files` equality against `validate_profile_body`, `profile` present and non-null, `validate_profile_body`'s own output carrying no `profile` key; and the missing-file half's `config_diagnostics` equality plus `profile: null`).

Fixture dedup: `LOADABLE_INVALID_PROFILE` extracted as a module-level `const` from the inline literal that used to live inside `validate_profile_body_reports_validate_diagnostics_for_a_loadable_invalid_profile`; that test now reads the const, and the new load-shape test reuses it too. The missing-file half reuses the exact `dir.path().join("missing.yaml")` (never written) shape already used by `validate_profile_body_reports_load_failure_with_no_mkvmerge_key`, not a new fixture.

## Gate results (nine parts, foreground, no subsets)

1. `cargo fmt --all --check` -- FAIL on first run (brief's verbatim test bodies don't match this repo's rustfmt config for line-wrapping); ran `cargo fmt --all` to reformat (assertion logic/strings untouched, only wrapping), then check PASS.
2. `cargo clippy --workspace --all-targets -- -D warnings` -- PASS, zero warnings.
3. `cargo test --workspace` -- PASS, all crates green (muxsmith-gui: 81 lib tests; workspace total includes muxsmith-core, muxsmith-cli, xtask, codegen integration tests -- all `0 failed`).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- PASS, no broken intra-doc links (all my new `[\`...\`]` links resolved, including cross-crate links to `muxsmith_core::profile::save::SaveError`/`muxsmith_core::planner::ApplyError`).
5. `cargo deny check` -- PASS (`advisories ok, bans ok, licenses ok, sources ok`; no new dependencies added by this task).
6. `pnpm lint` (eslint) -- PASS, no output (no Rust/TS frontend files touched by this task, but re-run as required).
7. `pnpm build` (vue-tsc + vite build) -- PASS.
8. `pnpm check:i18n` -- PASS (exit 0). The five new codes appear in the "unused" warning list (expected, non-fatal: `IpcError` codes are reached only via `$t(err.code, err.params)`, never as a literal `t()` call, exactly as already documented for `settings-io-failed`/`mkvmerge-spawn-failed`/etc. in the script's own header comment).
9. `pnpm test:e2e` (tsc + vite build + Playwright) -- PASS, 7/7 smoke tests green.

## Files changed

- `src-tauri/src/lib.rs`: module doc updated (two tasks -> three), imports (`Profile`, `save`, `planner::{self, StructuredEdit}`), four new `_body` functions + four new `#[tauri::command]`s, `invoke_handler` registration, `run()` doc comment updated, test-module fixture dedup + new shape test.
- `src-tauri/src/error.rs`: two new imports, `From<SaveError>`, `From<ApplyError>`, two new test functions.
- `locales/en/gui-common.ftl`: five new keys + a `## D42/D43` header comment.
- `locales/de/gui-common.ftl`: five new keys (no header comment, matching existing de convention).

Committed staged explicitly (`git add src-tauri/src/lib.rs src-tauri/src/error.rs locales/en/gui-common.ftl locales/de/gui-common.ftl`), no `git add -A`. `git status` after commit: clean working tree.

## Self-review

- All four commands present in `invoke_handler`'s `tauri::generate_handler![...]` list (verified by `grep`/reading the diff): `load_profile, save_profile, validate_profile_model, apply_suggestion` inserted after `detect_mkvmerge`, before `get_settings`.
- Load-shape test asserts both halves (loadable-invalid and missing-file) -- confirmed by reading the test body in the diff.
- Catalog entries byte-diffed against the brief's verbatim blocks in both locales -- identical.
- `validate_profile(path)` unchanged: only the module doc comment and `invoke_handler` list around it changed, its own body/doc/registration line is untouched.
- `StructuredEdit` already derived `Deserialize` (Task 6, confirmed at `planner.rs:210`) and `Profile` already derived both `Serialize`/`Deserialize` (confirmed at `profile/model.rs:45`) -- no derive changes needed for command argument/return types.
- `apply-rule-index-out-of-range`'s `$rules` is a labelled value (`rule count: { $rules }` / `Regelanzahl: { $rules }`), never a counted noun -- matches the brief and D49 :643-659 verbatim; no plural selector added.
- Test output pristine: rebuilt with `touch` + `cargo build` to force recompilation, grepped for "warning" -- none.

## Concerns

- Minor judgment call (not a design fork, no external-behavior/wire-format/test impact): gave `save_profile`, `validate_profile_model`, and `apply_suggestion` their own `_body` free-function twins for testability, matching the file's pre-existing 4-for-4 pattern (`validate_profile_body`/`dry_run_body`/`identify_body`/`detect_mkvmerge_body`), even though the brief's Step 3 prose names only `load_profile_body` explicitly. No new tests were invented beyond the brief's Step 1 list; the extra `_body` splits exist purely as thin, directly-inlineable wrappers (e.g. `apply_suggestion_body` is exactly the one-liner the brief itself describes: `planner::apply_suggestion(profile, config_path, edit).map_err(Into::into)`) and are available for Tasks 9-14's own test additions if wanted.
- `cargo fmt` reformatted the brief's verbatim test bodies (line-wrapping only, to satisfy the fmt gate); no assertion logic, string literal, or test name was altered.
