# Task 6 report: persisted job logs (`executor::joblog`, D26) + CLI wiring

## What was implemented

**`crates/muxsmith-core/src/executor/joblog.rs`** (new): `RunLogger`, exactly
the brief's signature.

- `default_runs_root() -> Option<PathBuf>`: `dirs::data_dir()?/muxsmith/runs`.
- `make_run_id(now: SystemTime) -> String`: UTC `"YYYYMMDD-HHMMSSZ"`. Built via
  a hand-assembled `&[time::format_description::BorrowedFormatItem]` (`Year`
  full-standard-range, numerical month/day/hour/minute/second components,
  literals for `-`/`Z`) rather than the `format_description!` macro or
  runtime string parsing, since the brief pins only the `formatting` cargo
  feature on `time` (not `macros`/`parsing`) -- see "Dependency versions"
  below for how this was confirmed against the actual 0.3.53 source rather
  than assumed.
- `RunLogger::create(runs_root, run_id, specs)`: `mkdir -p runs_root`, then
  creates the leaf directory via `fs::create_dir` (not `create_dir_all`,
  specifically so `AlreadyExists` is observable) in a loop that appends
  `-2`, `-3`, ... on collision. Seeds one accumulator (`argv`, `output`,
  empty `lines`, `started_at: None`) per spec index.
- `RunLogger::on_event(&mut self, ev)`: `Started` records `started_at`
  (RFC3339 UTC, via `time::format_description::well_known::Rfc3339`);
  `Output` appends the raw line to `lines`; `Finished` writes
  `job-<index>.json` (via a private `JobRecord` struct: `index`, `output`,
  `argv`, `state`, `exit_code`, `warnings`, `errors`, `duration_ms`, `lines`,
  `started_at` (nullable), `finished_at`) and drops that job's accumulator;
  `Progress`/`Warning`/`Error` are no-ops (their information already reaches
  the record two other ways: verbatim inside `lines`, and structured inside
  the `Finished` outcome). Write failures are swallowed (mirrors the queue's
  own "event send failures are ignored" contract) since `on_event` has no
  channel back to a caller and a log-write hiccup must never affect the mux
  itself.
- `RunLogger::finish(self, run_document: &Value) -> io::Result<PathBuf>`:
  writes `run_document` verbatim to `summary.json`, returns the log dir.
  Never rebuilds the document itself.
- `RunLogger::dir(&self) -> &Path`.
- No `Mutex` anywhere: the module doc states the single-threaded-writer
  invariant and that every drain-loop surface (CLI today, a future GUI) must
  tee its own events through one `RunLogger` rather than sharing one across
  threads.

**Scope decision on the "zero events" case**: a spec that is part of the
batch (seeded into `RunLogger::create`'s accumulator map) but never receives
even a `Finished` event -- i.e. never dequeued under BATCH cancellation
(D16, silent by design) -- gets no `job-<index>.json` at all; only
`summary.json`'s `run_document.jobs` array (already index-aligned to every
outcome, unconditionally, per T2/D15) covers it. This follows directly from
`on_event` only ever writing on `Finished`, which the queue never sends for
that case. Confirmed as intentional (not an oversight) with a dedicated test
(`a_job_with_zero_events_never_gets_a_file`) and covered again implicitly by
`skipped_job_without_started_still_writes_a_record_with_empty_lines`'s
companion assertion that job 0 (the non-cancelled sibling in that test, also
given zero events) gets no file either.

**CLI wiring** (`crates/muxsmith-cli/src/commands/run.rs`):

- New `create_logger(renderer, specs) -> Option<RunLogger>` helper: resolves
  `runs_root` from `MUXSMITH_RUNS_ROOT` (env override) or else
  `default_runs_root()`, then `RunLogger::create`s. Either step failing
  renders `run-joblog-unavailable` to stderr (unconditionally, both `--json`
  and human mode -- it's an operational warning on stderr, orthogonal to
  `--json`'s stdout-purity contract) and returns `None`.
- Called right before the queue's `mpsc::channel()`/`thread::scope`, i.e.
  after the `specs.is_empty()` early-return (so a batch with nothing to mux
  never touches the logger at all, matching D26's "dry-runs persist nothing"
  spirit) and after every earlier planning-failure early return (load
  failure, mkvmerge-not-found, query-failed -- none of those ever build
  `specs` in the first place).
- Drain loop: `logger.on_event(&event)` runs unconditionally, before the
  `if json { continue; }` guard -- so persistence happens under `--json`
  too, exactly like the brief requires.
- After `outcomes` is obtained: `run_document(...)` is now built
  unconditionally (previously only inside the `--json` branch) and bound to
  `document`, printed as before under `--json`, and also handed to
  `logger.finish(&document)`. `finish`'s `Ok` prints `run-joblog-written`
  with the directory in human mode only (no stdout output under `--json`,
  preserving the existing "exactly one stdout line" contract those tests
  pin); `finish`'s `Err` reuses `run-joblog-unavailable` on stderr (the
  brief only names two Fluent keys, and "log persistence isn't available"
  is the correct user-facing meaning for both the create-time and the
  finish-time failure).

**Fluent** (`locales/en/cli.ftl`): `run-joblog-unavailable`,
`run-joblog-written = Job logs written to { $dir }`. Not part of
`catalog_completeness.rs`'s gate (verified: that test iterates
`DiagCode::ALL` only, never touches CLI-only keys like the existing
`run-job-*`/`run-summary` family, so no gate needed updating).

**Test-pollution guard**: `run_cli.rs`'s
`run_json_on_a_real_mux_reports_a_populated_jobs_array_and_summary` and both
`run_live.rs` tests are the only three call sites across the whole CLI test
suite that actually reach the queue (every other `run` invocation in those
two files hits an earlier early-return -- profile load failure, mkvmerge
missing/broken, or a specs-empty path -- confirmed by reading each test's
own setup, not assumed); each now sets
`.env("MUXSMITH_RUNS_ROOT", <that test's own tempdir>/runs)`. Verified
`~/.local/share/muxsmith` does not exist after the full suite run.

## Dependency versions chosen + how resolved

- `dirs = "6.0.0"`, `time = { version = "0.3.53", features = ["formatting"] }`
  -- both queried live against the crates.io API (`max_stable_version`),
  full patch pin per the workspace's existing convention.
- The `time` 0.3.53 feature-gating question (can `make_run_id` use
  `OffsetDateTime::format` with only `formatting` enabled, i.e. without
  `macros`/`parsing`?) was verified against the actual crate source at tag
  `v0.3.53` on GitHub (`format_description/component.rs`,
  `format_description/modifier.rs`, `lib.rs`'s feature-gated `pub mod`
  lines, `Cargo.toml`'s `[features]` table) rather than assumed from
  training-data recall or a documentation summary alone -- context7's docs
  and a first docs.rs fetch were inconclusive/summarized away the field
  details, so raw source was pulled directly. Confirmed: `format_description`
  is public under `formatting OR parsing`; every non-deprecated `Component`
  modifier used here (`CalendarYearFullStandardRange`, `MonthNumerical`,
  `Day`, `Hour24`, `Minute`, `Second`) has a `const fn default()` (so usable
  in the `const RUN_ID_FORMAT` array) gated behind neither `macros` nor
  `parsing`; `OffsetDateTime::format`/`well_known::Rfc3339` need only
  `formatting`. The resulting code compiled and passed on the first attempt.

## TDD evidence (RED then GREEN)

1. Wrote `crates/muxsmith-core/tests/joblog.rs` (6 tests, covering the
   brief's exact Step 1 list plus `make_run_id`/`default_runs_root` unit
   coverage) against the not-yet-existing `joblog` module.
   `cargo test -p muxsmith-core --test joblog` failed to compile
   (`E0432: unresolved import ... joblog`) -- genuine RED, deps (`dirs`,
   `time`) already resolved and compiling cleanly at that point.
2. Implemented `joblog.rs` and wired `pub mod joblog;` into `executor/mod.rs`.
   All 6 tests passed on the first run after implementation (no
   iteration needed) -- GREEN.
3. CLI wiring done next (no separate CLI-level RED/GREEN cycle beyond
   updating the two existing subprocess test files' env overrides, since
   the brief's TDD list is scoped to the core module).
4. **Manual end-to-end verification** (not just unit tests): ran the built
   `muxsmith` binary via `cargo run` against a real one-file mux with
   `MUXSMITH_RUNS_ROOT` pointed at a scratch tempdir. Confirmed by
   inspection: run-id directory named `20260710-131418Z`; `job-0.json`
   contains the full real argv and all six real mkvmerge non-progress
   output lines verbatim in `lines`. (Correction from review: NO progress
   ticks appear in the real run's `lines`, correctly so -- `run_job`'s
   parser turns `#GUI#progress` lines into `Progress` events, so they can
   never arrive as `Output`. The unit test's progress-shaped `lines` entry
   is a synthetic `Output` event proving only that the writer does not
   filter by content; the original narration here conflated the two.)
   `started_at`/`finished_at` are real RFC3339 timestamps a few
   milliseconds apart, and `state`/`exit_code`/`duration_ms` all correct;
   `summary.json` matches the `--json` document shape; human-mode stdout
   ended with `Job logs written to <dir>` after the `run-summary` line;
   exit 0.

## Full gate (all green, foreground)

- `cargo test --workspace`: 32 test binaries, all `test result: ok`, 0
  failed (grepped the full log for `FAILED|error\[|error:|panicked`, no
  hits, not just eyeballing the tail).
- `cargo fmt --all --check`: clean (after one `cargo fmt --all` pass on the
  new test file, which the harness auto-applied and I incorporated).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, zero
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`;
  grepped specifically for `dirs`/`time` in any warning/error line -- no
  hits. `deny.toml` left unchanged (no entry needed).
- pnpm/frontend gates: not run, correctly -- this change touches only the
  Rust workspace (core + CLI), no `src-tauri`/frontend files.
- `~/.local/share/muxsmith` confirmed absent after the full suite run (no
  test pollution).

## Files changed

- New: `crates/muxsmith-core/src/executor/joblog.rs`,
  `crates/muxsmith-core/tests/joblog.rs`
- Modified: `crates/muxsmith-core/src/executor/mod.rs` (added `pub mod
  joblog;`), `crates/muxsmith-core/Cargo.toml` (`dirs`, `time` deps),
  `crates/muxsmith-cli/src/commands/run.rs` (logger wiring),
  `crates/muxsmith-cli/tests/run_cli.rs`, `crates/muxsmith-cli/tests/run_live.rs`
  (`MUXSMITH_RUNS_ROOT` overrides on the three queue-reaching tests),
  `locales/en/cli.ftl` (two new keys), `Cargo.lock`.

Commit: `7da714d` `feat(core+cli): persisted per-job JSON logs under
platform data dir (D26)`.

## Self-review findings

- **Interface match**: every signature (`RunLogger::create/on_event/finish/
  dir`, `default_runs_root`, `make_run_id`) matches the brief verbatim,
  including `make_run_id`'s injected-`SystemTime` testability requirement
  (no internal `now()` call).
- **D25 interaction correctly handled**: the per-job-skip case
  (`Finished{Cancelled}` without `Started`) writes a record with `lines: []`
  and `started_at: null`, tested directly; the batch-cancel-silence case
  (zero events at all) correctly produces zero per-job files, also tested
  directly rather than left implicit.
- **Field completeness**: `job-<index>.json` has exactly the brief's
  field list, no more, no less; `summary.json` is `run_document` written
  byte-identical (asserted via `serde_json::Value` equality in the test,
  not just presence).
- **Single-threaded-writer invariant**: documented in the module doc as
  required by the brief; no `Mutex`/`Arc` inside `RunLogger` itself (it is
  driven by-value/by-`&mut` from the one drain-loop thread, exactly like
  `MilestoneState` already is in the same function).
- **Prose-free core**: `joblog.rs` emits zero human-facing strings; the two
  new Fluent keys live only in the CLI catalog, per the crate-level
  contract.
- **ASCII-only**: grepped all six changed/new files for non-ASCII bytes,
  clean.
- **Test hermeticity**: re-verified by rereading each of the ten `run_cli.rs`
  tests and both `run_live.rs` tests individually to confirm which ones
  reach the queue before deciding where the env override was actually
  needed, rather than blanket-adding it everywhere or guessing.

## Concerns / things worth a second look

- **Silent on_event write failures**: `on_event` swallows
  `serde_json`/`fs::write` errors for `job-<index>.json` with no signal
  back to the caller at all (not even the `run-joblog-unavailable` warning
  `create`/`finish` failures get). This matches the brief's `on_event(&mut
  self, ev: &JobEvent)` signature (no `Result`) and the queue's own
  "ignore send failures" precedent, but it means a mid-run write failure
  (e.g. the log dir got removed or filled up mid-batch, after `create`
  already succeeded) is currently invisible to the user, unlike the
  create/finish failure paths which do warn. Flagging for whole-branch
  review judgment rather than silently deciding it's fine; a follow-up
  could have `finish` (which does return `Result` and does get surfaced)
  double as a coarse "did anything go wrong during the run" checkpoint if
  this is judged worth closing.
- **`specs.is_empty()` scope decision**: no log directory is created at all
  when nothing plans cleanly enough to mux (see "Scope decision" above).
  The brief doesn't explicitly address this path; I read "before the queue
  thread" as implying after that early return, and D26's "dry-runs persist
  nothing" as the closest analogous precedent. Worth confirming this
  matches intent, since GUI Task 8's history view will only ever see run
  directories for batches that had at least one job.
- **Stale prior report at this path**: `task-6-report.md` previously held an
  unrelated report ("mkvtoolnix in CI", commit `5aefde1`) from a different
  plan's own Task 6. Overwritten per this task's explicit instruction to
  write here; flagging in case that content needed to be preserved
  elsewhere first.

---

# Fix-round report (review follow-up on 7da714d)

Commit: `f54cbab` `fix(core+cli): surface lost mid-run job-log writes; gate
runs-root env override to debug builds`

## Fix 1 (Important): silent mid-run write loss

- `RunLogger` gained a private `had_write_error: bool`. `on_event` keeps its
  mandated no-return signature; a `serde_json::to_vec_pretty` or `fs::write`
  failure for `job-<index>.json` now sets the flag instead of disappearing.
- `finish()` writes `summary.json` FIRST (best-effort: persist what we can,
  then signal), then returns `Err(io::Error::other(...))` when the flag is
  set -- so a run whose per-job logs are incomplete can never be reported as
  cleanly persisted, even though the summary itself wrote fine. A
  `summary.json` write failure still propagates as before.
- CLI: the finish-`Err` branch now renders the NEW Fluent key
  `run-joblog-incomplete` (with `$dir`, captured from `logger.dir()` before
  `finish` consumes the logger) as a stderr warning, replacing both the
  would-be `run-joblog-written` false success and the previous reuse of
  `run-joblog-unavailable` (whose "continuing without persisted logs" text
  was wrong for a directory that exists but is partial). Key added to
  `locales/en/cli.ftl`.
- TDD: RED first --
  `a_failed_job_file_write_makes_finish_err_but_summary_still_writes`
  pre-creates a DIRECTORY at the `job-0.json` path (the same portable
  no-perms failure trick `delete_partial_failure_surfaces_into_errors` in
  job.rs uses), drives a `Finished` event, and asserts `finish()` errs while
  `summary.json` exists with the exact document. Failed as expected against
  the old code (`finish` returned `Ok`), passed after the implementation.
- Note on the `io::Error::other` message text: it is a programmer-facing
  error payload (the CLI never renders it -- it renders the Fluent key), the
  same class as `expect()` messages, so the "core emits no user-facing
  prose" contract holds.

## Fix 2 (Minor): MUXSMITH_RUNS_ROOT gated to debug builds

- The env read in `create_logger` is now behind `#[cfg(debug_assertions)]`;
  the `#[cfg(not(debug_assertions))]` arm uses `default_runs_root()` only.
  Comment added stating it is a test seam, not a feature, and that a real
  user-facing override would be a deliberate v1.x decision.
- Verified BOTH profiles compile (`cargo build -p muxsmith-cli` and
  `--release`), since the release arm is otherwise dead in every local
  test run. `run_cli.rs`/`run_live.rs` keep working unchanged
  (`assert_cmd::cargo_bin` runs the debug binary).

## Report-narrative correction (no code action assigned)

The "Manual end-to-end verification" paragraph above was corrected in place:
the real run's `lines` contains NO progress ticks (run_job parses
`#GUI#progress` into `Progress` events before `Output` exists); the original
narration conflated the synthetic unit-test `Output` event with the real
pipeline. The corresponding test assertion message in `joblog.rs` was also
rephrased to say precisely that (writer does not filter by content; real
ticks never arrive as `Output`).

## Gate (foreground, all green)

- Covering tests: joblog suite now 7/7 (new write-failure test included);
  `run_cli` 10/10; `run_live` 2/2.
- `cargo test --workspace`: 32 test binaries all `test result: ok`; full log
  grepped for `FAILED|error\[|error:|panicked`, zero hits.
- `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets
  -- -D warnings` clean; `cargo deny check` all four sections ok.
- Fluent key spelling cross-checked between `run.rs` call sites and
  `cli.ftl` (all three `run-joblog-*` keys present); changed files ASCII
  clean.

## Residual concerns

- `run-joblog-incomplete` covers two conditions with one message (per-job
  write lost mid-run vs. `summary.json` itself failed after per-job files
  wrote fine). The single key matches the review's instruction and both
  readings of "incomplete" are truthful; if the GUI later wants to
  distinguish them, `finish` would need a richer error type than
  `io::Error` -- deferred as not needed by any current consumer.
- The `specs.is_empty()` scope decision from the original report stands
  (no run directory for a batch with zero muxable jobs); unchanged by this
  fix round and still worth a controller confirmation for GUI Task 8's
  history-view expectations.
