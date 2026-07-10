# Task 8 merge report: plan5-t8 (run lifecycle IPC) into master

`git merge --no-ff --no-commit plan5-t8` on top of `dabf26c` (plan5-t7's own
merge). Four files conflicted: `src-tauri/src/error.rs`,
`src-tauri/src/lib.rs`, `src-tauri/Cargo.toml`, `Cargo.lock`. `run.rs`
(T8's complete run-lifecycle module) and `settings.rs` (T7's settings
persistence) were not conflicted. `locales/en/gui-common.ftl` auto-merged
cleanly; verified the D31 keys (`close-abort-title/message/confirm/dismiss`)
survived.

## Merge commit

`7ce88dbef3e7e7d7a22b3b418416cd8b22ede9ca` (parents `dabf26c` / `e7cb673`).

## The coupling that shaped the resolution

Both sides need one Tauri-managed `AppState` (Tauri resolves `State<T>`
once per managed type). T7's `AppState` (crate root, `lib.rs`) held
`settings_path`; T8's own `AppState` (defined *inside* `run.rs`) held
`active: Mutex<Option<RunSlot>>` and `quit_after_finished: AtomicBool`.
Since `run.rs` was off-limits except for strictly-required mechanical
renames, and the task explicitly framed "unified AppState" as part of
`lib.rs`'s resolution, the union struct now lives in `lib.rs` with all
three fields, field names/types taken verbatim from `run.rs`'s own usage
(the "non-conflicted authority").

## src-tauri/src/error.rs

Single `IpcError { code: String, params: HashMap<String, String> }`
(identical shape on both sides already). Kept:
- T7's three `From` impls (`RuntimeError`, `IdentifyError`, `SettingsError`
  -> `IpcError`) verbatim, including every code string
  (`mkvmerge-not-found`, `mkvmerge-too-old`, `mkvmerge-spawn-failed`,
  `mkvmerge-query-failed`, `identify-failed`, `settings-io-failed`,
  `settings-parse-failed`).
- Both constructors: `IpcError::new(code)` (T7's name, called throughout
  `lib.rs`) and `IpcError::code(code)` (T8's name, called throughout
  `run.rs`). Not a true duplicate (different names, different call sites
  depend on each), so both survive; `code()` now delegates to `new()` to
  avoid a literal body duplicate.
- One `with()` builder (T7's and T8's versions were a true duplicate:
  identical signature and body, doc wording merged).
- All 13 tests: T7's 10 (`not_found_has_no_params` ... `with_overwrites_a_
  prior_value_for_the_same_key`) + T8's 3 (`code_builds_with_empty_params`,
  `with_attaches_and_overwrites_params`, `serializes_as_code_and_params`).

## src-tauri/src/lib.rs

- `AppState` now carries `settings_path: Option<PathBuf>` (T7) plus
  `active: Mutex<Option<RunSlot>>` and `quit_after_finished: AtomicBool`
  (T8, names/types taken from `run.rs`). Custom `impl Default` combines
  T7's `settings::settings_path()` resolution with T8's zeroed run state.
- All of T7's command bodies, IPC commands (`validate_profile`, `dry_run`,
  `identify`, `detect_mkvmerge`, `get_settings`, `set_settings`), and their
  16 tests kept verbatim, except the three `AppState { settings_path: ... }`
  test-literal constructions, which now use `..AppState::default()` to
  fill the two new fields (semantically identical: those tests only ever
  cared about `settings_path`).
- `pub fn run()`: one `.manage(AppState::default())`, one
  `invoke_handler!` registering all 11 commands (T7's 6 + T8's 5:
  `start_run`, `cancel_run`, `cancel_job`, `list_runs`, `get_job_log`),
  T7's two plugin registrations, T8's `.on_window_event(run::on_close_requested)`.
- Module doc comment rewritten to describe the merged shell (both tasks'
  scope) instead of T7's now-stale "T8 extends this struct" note.

## src-tauri/src/run.rs (mechanical renames only, per the task's constraint)

Kept run.rs's entire logic, doc, and 39 tests untouched in substance. Four
purely mechanical edits, all required by the struct's move to `lib.rs`:

1. Deleted the local `pub struct AppState { active, quit_after_finished }`
   definition and its `#[derive(Default)]` (superseded by `lib.rs`'s
   unified struct); added `use crate::AppState;` so every existing
   unqualified `AppState` reference in `run.rs` resolves unchanged.
2. `enum RunSlot` -> `pub(crate) enum RunSlot`: `lib.rs` (the *parent*
   module) needs to name `RunSlot` in `AppState`'s field type; a module's
   private items are visible to its descendants, not its ancestors, so
   this bump is strictly required. Not exposed outside the crate.
3. `struct ActiveRun` -> `pub(crate) struct ActiveRun`: caught by
   `cargo check` as a `private_interfaces` warning (`RunSlot::Running`'s
   field reachable at `pub(crate)` but `ActiveRun` itself only
   `pub(self)`) once `RunSlot` went `pub(crate)`; would have failed the
   `-D warnings` clippy gate otherwise.
4. `use std::sync::{Arc, Mutex};` -> `use std::sync::Arc;` (`Mutex` was
   only referenced by the now-removed `AppState` struct); `cargo fmt`
   subsequently reordered this line among the `use std::sync::*` group.

No field names, no test bodies, no run-lifecycle logic changed.

## src-tauri/Cargo.toml / Cargo.lock

`Cargo.toml`: union of exactly one differing line, `dirs = "6.0.0"` (T7's
dependency for `settings::settings_path`; T8's branch never added it).
Kept.

`Cargo.lock`: same single-line union in `muxsmith-gui`'s own
`dependencies` array (`"dirs"`, already present as a locked package from
the earlier T7 merge -- no new package entries, no version changes).
Resolved by hand rather than regenerated; verified with a diff against
each parent's lockfile: the result is byte-identical to `HEAD`'s
(T7-side) `Cargo.lock` plus that one added line versus `plan5-t8`'s
lockfile. `cargo check`/`cargo test` afterward made no further changes to
`Cargo.lock`, confirming it is internally consistent with the merged
manifests.

## Gate results (all run in the foreground before committing)

| Gate | Result |
|---|---|
| `cargo test --workspace` | all green: 72 `muxsmith-gui` lib tests (13 `error::`, 39 `run::`, 20 `tests::`/`settings::`), plus every `muxsmith-core`/`muxsmith-cli`/`xtask` suite, 0 failures |
| `cargo fmt --all --check` | clean (after one `cargo fmt --all` pass reordering the `use std::sync::Arc;` line) |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` |
| `mise exec -- pnpm lint` | clean (`eslint .`) |
| `mise exec -- pnpm build` | clean (`vue-tsc --noEmit && vite build`) |

72 = T7's 33 baseline gui-lib tests plus T8's 39 (3 error.rs + 36 run.rs),
with zero true duplicates removed at the test level (the two `with`-style
tests in `error.rs` differ in name and body, so both were kept) and only
the `IpcError::with` builder itself deduped as a single true-duplicate
helper.

## Ambiguities encountered

None. No two helpers/impls collided with incompatible semantics; the only
non-trivial call was where the unified `AppState` struct should live,
resolved per the task's own framing (lib.rs's conflict resolution) and
confirmed compilable/testable end-to-end.

## Nothing lost

Every code string, every test, every doc-documented behavior from both
`plan5-t7`'s merge and `plan5-t8` survives. The only renames are the two
visibility bumps in `run.rs` (`RunSlot`, `ActiveRun`: private -> `pub(crate)`)
and the resulting import fix, none of which change behavior, only where
the types are nameable from.
