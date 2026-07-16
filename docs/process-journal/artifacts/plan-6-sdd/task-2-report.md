# Task 2 report: D41 - the core profile writer

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan6-a`, branch `plan6-a`.
Commit: `81b4038` "core: profile::save writes canonical YAML/JSON from the model (D41)"

## What was implemented

`muxsmith_core::profile::save`, the canonical profile writer:

- `pub enum SaveError { Io(String), Serialize(String) }` - `#[derive(Debug, Clone, PartialEq)]`, plain data enum (no `std::error::Error`), matching `SettingsError`'s shape in `src-tauri/src/settings.rs`. Rustdoc explains why it is not a `Diagnostic` (owner ruling superseding D41's original draft signature, per the design doc's amendment).
- `pub fn to_string(profile: &Profile, format: Format) -> Result<String, SaveError>` - `Format::Yaml` arm calls `yaml_serde::to_string`, `Format::Json` arm calls `serde_json::to_string_pretty`; both map their error via `.to_string()` into `SaveError::Serialize`.
- `pub fn to_file(profile: &Profile, path: &Path) -> Result<(), SaveError>` - selects `Format` from the path extension with the identical match arms as `load::from_file:57-62` (`Some("json")` -> `Format::Json`, everything else -> `Format::Yaml`), serializes via `to_string`, then `fs::write`, mapping I/O errors into `SaveError::Io`.

Wired into `crates/muxsmith-core/src/profile/mod.rs`: `pub mod save;` added in alphabetical position after `pub mod model;`, and the module doc's opening sentence extended to name `save` beside `load` ("[`load`] parses YAML/JSON into the serde model, [`save`] writes the model back out canonically (D41), ...").

Dev-dependency check (Step 5): `tempfile = "3.27.0"` was already present in `crates/muxsmith-core/Cargo.toml`; no change made (confirmed no-op, `git add` on it staged nothing).

## TDD evidence

**RED** - `cargo test -p muxsmith-core --test profile_save`:
```
error[E0432]: unresolved import `muxsmith_core::profile::save`
 --> crates/muxsmith-core/tests/profile_save.rs:9:29
  |
9 | use muxsmith_core::profile::save::{SaveError, to_file, to_string};
  |                             ^^^^ could not find `save` in `profile`
```
Matches the brief's expected failure exactly.

**GREEN** - `cargo test -p muxsmith-core --test profile_save`:
```
running 4 tests
test canonical_json_round_trips_to_an_equal_model ... ok
test an_unwritable_path_is_an_io_error_not_a_panic ... ok
test canonical_yaml_round_trips_to_an_equal_model ... ok
test to_file_picks_json_from_the_extension_and_never_changes_format ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Gate results (all nine parts, foreground, no subsets)

1. `cargo fmt --all --check` - initially reformatted one match arm in `save.rs` (rustfmt's own line-length call on the `Json` arm); ran `cargo fmt --all`, then re-ran `--check`: clean, exit 0.
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean, exit 0, whole workspace including `muxsmith-gui`/tauri.
3. `cargo test --workspace` - all suites `ok`, 0 failed, including `profile_save` (4 passed) and the rest of `muxsmith-core`, `muxsmith-cli`, `muxsmith-gui`, `xtask`, doc-tests.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean, exit 0, no broken intra-doc links.
5. `cargo deny check` - `advisories ok, bans ok, licenses ok, sources ok`.
6. `pnpm install` (node_modules was missing in the worktree) - resolved from the existing lockfile, no changes; then `pnpm lint` (`eslint .`) - clean, exit 0.
7. `pnpm build` (`vue-tsc --noEmit && vite build`) - clean, exit 0.
8. `pnpm check:i18n` - `check-i18n: ok (17 source files scanned, 181 catalog ids, 12 unused warning(s), ...)`. The 12 "unused" catalog keys are pre-existing (unrelated to this task; this task adds no Fluent keys) and the script treats them as warnings, not failures - exit 0.
9. `pnpm test:e2e` (`tsc --noEmit -p e2e/tsconfig.json && vite build ... && playwright test`) - `7 passed (1.2s)`, exit 0.

## Files changed

- `crates/muxsmith-core/src/profile/save.rs` (new, 62 lines)
- `crates/muxsmith-core/src/profile/mod.rs` (module doc + `pub mod save;`)
- `crates/muxsmith-core/tests/profile_save.rs` (new, 60 lines)
- `crates/muxsmith-core/Cargo.toml` staged but unmodified (tempfile already present)

## Self-review findings

- Rustdoc present on every public item (`SaveError` and both variants, `to_string`, `to_file`); `#![deny(missing_docs)]` compiles clean and the doc gate's intra-doc links (`[Format::Yaml]`, `[super::load::from_file]`, `[SaveError::Serialize]`, `[crate::report::Diagnostic]`) all resolve.
- `to_file`'s extension-match arms are textually identical to `load::from_file:57-62`'s, per the binding point.
- Test output is pristine: no warnings, no stray prints, deterministic (tempdir-scoped, no shared state).
- Diffed the committed test file against the brief's verbatim block: identical except the one deliberate line removed (see Concerns).

## Concerns

**One deviation from the brief's "exact test code, use verbatim" instruction**, surfaced rather than silently absorbed: the brief's test file imports `use std::path::Path;` but the test bodies never reference `Path` directly (all paths are `PathBuf` from `.join()`, deref-coerced at the `to_file`/`from_file` call sites). Left in, it triggers `warning: unused import: 'std::path::Path'`, which fails the mandatory `cargo clippy --workspace --all-targets -- -D warnings` gate (rustc's `unused_imports` lint is included under clippy's `-D warnings`). This has exactly one sane resolution - delete the dead import - with no ripple cost, hidden consumer, or colliding invariant, so I fixed it inline rather than escalating as NEEDS_CONTEXT; every other line of the test file, and all binding-point signatures/behaviour, are verbatim/as specified.

No other concerns. No design latitude was exercised beyond that one mechanical fix.
