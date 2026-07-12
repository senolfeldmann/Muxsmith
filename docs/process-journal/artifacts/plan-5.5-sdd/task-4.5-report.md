# Task 4.5 report: D35 run-log auto-prune, 14 days fixed

Commit: `3511efe` on `plan55-stream-a` (worktree `.worktrees/stream-a`).

## What was implemented

**`crates/muxsmith-core/src/executor/joblog.rs`**

- `pub fn run_id_timestamp(name: &str) -> Option<OffsetDateTime>`: the
  inverse of `make_run_id`. Parses the fixed `YYYYMMDD-HHMMSSZ` 16-byte
  prefix (tolerating a trailing `-N` collision suffix), returns `None` for
  anything that doesn't match -- including a digit-shaped but
  calendar-invalid value (month `13`, hour `99`), via `time::Date::
  from_calendar_date` / `Date::with_hms`'s own range validation. This is
  strictly stricter than the old shell parser (which only checked digit
  shape and would have happily emitted a nonsensical RFC3339 string for an
  out-of-range value); no existing test exercised that gap, so nothing
  regresses.
- `const RUN_LOG_RETENTION: time::Duration = time::Duration::days(14)`,
  comment citing D35 (fixed for v1, no config; IDEAS #7 parks
  configurability).
- `pub fn prune_stale_runs(runs_root: &Path, now: SystemTime)`: iterates
  `read_dir(runs_root)`, deletes (`remove_dir_all`) only entries that are
  (a) real directories per `DirEntry::file_type()` (does not follow
  symlinks -- a symlink is excluded outright, never handed to
  `remove_dir_all`) and (b) whose name `run_id_timestamp` parses to
  something older than `now - 14d`. Every I/O error along the way
  (`read_dir`, a `file_type()` call, `remove_dir_all`) is silently ignored,
  with rustdoc stating why: pruning is housekeeping, not part of a run's
  contract, and its only failure mode (old logs surviving longer) is
  harmless.
- `RunLogger::create` calls `prune_stale_runs(runs_root, SystemTime::now())`
  right after `create_dir_all(runs_root)` and before the leaf-directory
  creation loop. No signature change.

**`src-tauri/src/run.rs`**

- `started_at_from_run_id` is now a thin delegate:
  `run_id_timestamp(run_id)?.format(&time::format_description::well_known::Rfc3339).ok()`.
  Added `time = { version = "0.3.53", features = ["formatting"] }` to
  `src-tauri/Cargo.toml` (same version already pinned by muxsmith-core;
  `Cargo.lock` confirms a single resolved `time` entry, no duplication).

## TDD evidence

**RED** (before any implementation, `run_id_timestamp`/`prune_stale_runs`
did not exist yet -- test compiled against the not-yet-wired `create`):

```
test create_prunes_run_dirs_older_than_14_days_by_name_only ... FAILED
  left: ["20200101-000000Z", "20200101-000000Z-2", "20260711-202816Z", "keep-me", "notes.txt", "this-run"]
 right: ["20260711-202816Z", "keep-me", "notes.txt", "this-run"]
```

**GREEN** (after implementing the parser, retention const, prune fn, and
wiring it into `create`):

```
test create_prunes_run_dirs_older_than_14_days_by_name_only ... ok
```

Beyond the brief's mandated Step 1 test, three more focused tests were
added to cover Step 2's explicit safety requirements that the Step 1
scenario doesn't exercise on its own:

- `run_id_timestamp_round_trips_make_run_id_and_tolerates_the_collision_suffix`
- `run_id_timestamp_rejects_garbage_and_out_of_range_calendar_values`
- `prune_stale_runs_boundary_is_exactly_14_days` (13d23h59m59s survives,
  14d0h0m1s is pruned -- exercised directly against `prune_stale_runs`
  since `create` always calls `SystemTime::now()` internally and the
  cutoff itself needs a controlled `now` to test deterministically)
- `prune_stale_runs_leaves_a_stale_named_symlink_and_its_target_untouched`
  (`#[cfg(unix)]`, matching this codebase's existing symlink-test pattern
  in `discovery.rs`): a symlink named and dated exactly like a stale run
  dir, pointing at a directory outside `runs_root`, survives untouched --
  proves `remove_dir_all` is never handed a symlink.

All 12 tests in `crates/muxsmith-core/tests/joblog.rs` pass.

## Delegation proof (reuse acceptance criterion)

`git diff src-tauri/src/run.rs` touches only the import line and the
`started_at_from_run_id` function body -- the `#[cfg(test)] mod tests`
block is untouched. Running the shell's full test module:

```
cargo test -p muxsmith-gui --lib run::
```

-> 40/40 pass, including the three tests that are the actual reuse
acceptance criterion, unmodified:

- `started_at_from_run_id_parses_the_fixed_prefix`
- `started_at_from_run_id_parses_a_collision_suffixed_dir_name`
- `started_at_from_run_id_rejects_garbage`

This proves the delegation preserved the RFC3339-output and
None-for-foreign-names contract exactly.

## Files changed

- `crates/muxsmith-core/src/executor/joblog.rs` (new parser, retention
  const, prune fn, `create` wiring + rustdoc)
- `crates/muxsmith-core/tests/joblog.rs` (Step 1 test + 3 supporting tests)
- `src-tauri/src/run.rs` (delegate + import)
- `src-tauri/Cargo.toml` (new `time` dependency)
- `Cargo.lock`

## Gate results (all nine parts, foreground)

1. `cargo fmt --all --check` -- clean (one auto-fixable import-wrap diff,
   applied via `cargo fmt --all`, then re-verified clean).
2. `cargo clippy --workspace --all-targets -- -D warnings` -- clean.
3. `cargo test --workspace` -- all green, every crate, 0 failures (grepped
   the full log for `FAILED`/non-zero-failed lines: none).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- **fails
   at HEAD, but not from this diff.** All remaining errors are pre-existing
   broken intra-doc links in files this task never touched: `capability/
   runtime.rs` (`platform_candidates`), `executor/queue.rs`
   (`worker_count`), `cli/i18n.rs` (`msg`, x2), `src-tauri/src/lib.rs`
   (`run::on_close_requested`), and one in `joblog.rs` that predates this
   task (`create`'s existing `[JobAccumulator]` link, a sentence this diff
   did not modify). These exactly match commit `004e1e8` ("ci: cargo doc
   -D warnings as ninth gate part (#18b)") on the parallel, not-yet-merged
   `plan55-stream-d` branch, which fixes precisely this set. My own new
   rustdoc (`run_id_timestamp`, `prune_stale_runs`, the `create`/
   `started_at_from_run_id` doc additions) introduced one such error
   during development (`[RUN_LOG_RETENTION]` linking to a private const);
   fixed by de-linking to plain code font before the final commit. Isolated
   `cargo doc -p muxsmith-gui --no-deps` confirms the `run.rs` change adds
   zero new doc errors.
5. `cargo deny check` -- `advisories ok, bans ok, licenses ok, sources ok`
   (exit 0). Only pre-existing duplicate-version *warnings* from tauri's
   own tree; `Cargo.lock` shows a single resolved `time` entry (0.3.53),
   so the new dependency introduced no duplication.
6. `pnpm lint` -- clean.
7. `pnpm build` -- clean (vue-tsc + vite build).
8. `pnpm check:i18n` -- `ok` (12 pre-existing "unused" warnings, unrelated
   to this task -- diagnostic-code keys referenced dynamically).
9. `pnpm test:e2e` -- 3/3 Playwright specs pass.

## Self-review

- No signature change on `RunLogger::create`; both callers (`crates/
  muxsmith-cli/src/commands/run.rs:317`, `src-tauri/src/run.rs:329`) are
  untouched and still compile/pass.
- Retention window uses `time::Duration` (not `std::time::Duration`) for
  the `OffsetDateTime` subtraction, avoiding `SystemTime`'s panic-on-
  underflow `Sub` semantics near the Unix epoch.
- `run_id_timestamp`'s calendar validation is stricter than the original
  shell parser (rejects out-of-range digit-shaped values); this is a
  strict subset tightening, not a behavior change any existing caller or
  test depends on -- confirmed by the unchanged shell tests staying green.
- Symlink safety relies on `DirEntry::file_type()` not following symlinks
  (stdlib-documented behavior), the same technique `list_runs_in`
  (`src-tauri/src/run.rs`) already uses for its own dir-only filter --
  consistent with existing codebase style.
- Did not touch `capability/runtime.rs`, `executor/queue.rs`,
  `cli/i18n.rs`, or `src-tauri/src/lib.rs` to fix their pre-existing doc
  links, since that is explicitly the scope of the parallel
  `plan55-stream-d` task (commit `004e1e8`, unmerged); touching them here
  would risk a conflicting/duplicate fix at merge time.

## Concerns

- **Gate part 4 (`cargo doc`) is red at HEAD of this branch**, but for
  reasons entirely outside this task's diff (see above) -- it will go
  green once `plan55-stream-d`'s `004e1e8` merges alongside this commit.
  Flagging explicitly since the task brief asked for a full nine-part run;
  parts 1-3 and 5-9 are unconditionally green, part 4 is green for
  everything this task touched and red only for pre-existing, out-of-scope
  content.
- No Windows/macOS-specific verification was possible in this environment
  (Linux only); the symlink test is `#[cfg(unix)]`-gated, matching the
  codebase's existing convention for platform-specific symlink tests, so
  it simply won't run on the Windows CI leg (as intended -- Windows
  symlinks need elevated privileges, same reasoning as `discovery.rs`'s
  existing symlink tests).
