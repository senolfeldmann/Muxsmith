# Final-review fix wave report

Base commit: `93d1a6b`. Four findings from the whole-branch review, one commit
per finding, TDD where behavior changed. Foreground execution only.

## Fix 1 (Important): Windows kill mapping - killed flag on LiveJob

**Commit:** `75c075f` fix(executor): killed jobs report wait()=None
cross-platform via killed flag (D16/D17)

**Changed:** `crates/muxsmith-core/src/executor/spawn.rs`.
`LiveJob` gained a `killed: Arc<AtomicBool>` field, set by `killer()`
(`Ordering::SeqCst`) *before* it calls `child.kill()`. `wait()` now computes
the raw OS exit code first, then folds it through a new pure function
`resolve_wait(killed: bool, raw_code: Option<i32>) -> Option<i32>` that
returns `None` whenever the flag is set, regardless of what the OS reported.
Mirrors `FakeJob`'s existing `killed: AtomicBool` mechanism. Updated the
`RunningJob::wait` trait doc to state the cross-platform guarantee explicitly
(Windows `TerminateProcess` always yields `Some`, unlike Unix's signal
death), and the `LiveJob`/`killer()` doc comments to explain the
set-before-kill ordering.

**RED route taken:** the unit-level seam, per the finding's fallback branch.
Empirically confirmed first (`/tmp/.../probe_kill.rs`, a throwaway harness
outside the repo) that `std::process::Child::kill` sends `SIGKILL` on Unix:
a script that traps `TERM`/`INT` and would `exit 0` never got the chance -
the process died by an untrappable signal (`code: None, signal: Some(9)`).
That means the trap-based live RED is not demonstrable on Unix: the raw
`wait()` already returns `None` from a signal death, independent of any
flag, so a live script test cannot distinguish the pre-fix code from the
fix.

Route taken instead: extracted `resolve_wait` as a brand-new, directly
unit-testable pure function. RED was the natural TDD state for new code -
the test module was written first, calling `resolve_wait(true, Some(1))`
etc.; `cargo test -p muxsmith-core --lib executor::spawn` failed to compile
(`error[E0425]: cannot find function 'resolve_wait'`, 6 errors) because the
function did not exist yet. Implementing `resolve_wait` plus wiring the
`killed` flag into `LiveJob` turned this GREEN.

A second, gated `#[cfg(unix)]` test (`live_killer_then_wait_returns_none`)
was added alongside, per the finding's request for a live-wiring check: it
spawns a scripted fake `mkvmerge` (a `#!/bin/sh` script that echoes one line
then sleeps 30s, written/chmod'd exactly like
`crates/muxsmith-cli/tests/run_cli.rs`'s `fake_mkvmerge_that_fails_queries`
stub), reads one line to prove the process is alive, invokes `killer()`,
and asserts `wait() == None`. This test is GREEN both before and after the
fix (SIGKILL alone already yields `None`), so it does not itself pin the
regression - it only guards that the flag is actually wired into `LiveJob`
end-to-end and that `LiveSpawner` still works against a scripted process.
The `resolve_wait` unit tests are what carry the real RED/GREEN evidence.

**Gate:** `cargo test --workspace` green (all suites, incl. the 2 new
`spawn` tests), `cargo fmt --all --check` clean, `cargo clippy --workspace
--all-targets -- -D warnings` clean, `cargo deny check` clean.

## Fix 2 (Important): mkvmerge_found asserted on paths that never checked

**Commit:** `db9f559` fix(cli): emit mkvmerge_found only when the lookup
actually ran

**Changed:** `crates/muxsmith-cli/src/commands/dry_run.rs`,
`crates/muxsmith-cli/src/commands/run.rs`,
`crates/muxsmith-cli/tests/dry_run_cli.rs`,
`crates/muxsmith-cli/tests/run_cli.rs`.

`dry_run::config_only_json` now takes `mkvmerge_found: Option<bool>`:
`None` omits the key from the JSON document entirely, `Some(bool)` includes
it. All three call sites per file were updated:

- Profile-load failure (`dry_run.rs:48`, `run.rs:71`): `None` - the lookup
  never ran on this path, so the field is now absent.
- `Mkvmerge::locate()` failure (`dry_run.rs:70`, `run.rs:93`): `Some(false)`
  - unchanged behavior, the locate actually ran and failed.
- `list_languages()` failure (`dry_run.rs:88`, `run.rs:115`): `Some(false)`
  - unchanged behavior. Left out of scope deliberately (see note below).

**Test evidence (RED/GREEN):** added an assertion to both existing
profile-load-failure tests (`dry_run_json_emits_a_document_on_profile_load_failure`,
`run_json_emits_a_document_on_profile_load_failure`) that
`report.get("mkvmerge_found").is_none()`. Confirmed RED first: both failed
with the field present as `false` (`... got:
{"...","mkvmerge_found":false,...}`). After parameterizing
`config_only_json` and updating the call sites, both went GREEN. The
existing locate()-failure assertions (`assert_eq!(report["mkvmerge_found"],
false)` in both files) were left untouched and still pass, confirming that
path's behavior is unchanged.

**Scope note (not fixed, flagged for awareness):** the `list_languages()`
failure path also passes `Some(false)`, even though `locate()` *did* succeed
there (mkvmerge was found; only the subsequent query failed) - the same
shape of inaccuracy the finding flagged, just not the arm the finding named.
Left unchanged per the "exactly the files/lines named" scope constraint; no
test currently pins this field's value on that path either way. Flagging
for a future finding rather than folding it into this fix.

Checked for a schema doc pinning the JSON document shape (per the finding's
instruction): none exists. `crates/muxsmith-cli/tests/cli_schema.rs` is
about the unrelated `--schema` CLI subcommand output, not this document's
shape. No further doc updates needed.

**Gate:** full suite green (95 -> unaffected core count, cli suites all
passing including the two updated tests), fmt clean, clippy clean, deny
clean.

## Fix 3 (Minor): stale exit-code doc

**Commit:** `841db0b` docs(cli): exit-code doc includes 130 (D16)

**Changed:** `crates/muxsmith-cli/src/cli.rs:12-13`. One doc-comment line:
`0 clean / 1 warnings / 2 errors (spec 8.1)` -> `0 clean / 1 warnings / 2
errors / 130 cancelled (spec 8.1, D16)`.

No behavior change, doc-only; no TDD applicable. Full gate re-run and green
regardless (workspace test suite, fmt, clippy, deny).

## Fix 4 (Minor): cap queue workers at spec count

**Commit:** `4b1ddbf` fix(executor): cap worker count at spec count

**Changed:** `crates/muxsmith-core/src/executor/queue.rs`. Extracted the
worker-count decision into a new pure function `worker_count(jobs:
usize, spec_count: usize) -> usize`, clamping `jobs` to >= 1 and then
capping at `spec_count.max(1)` (so an empty batch still gets one idle
worker rather than zero). `run_queue` now computes `let workers =
worker_count(opts.jobs, specs.len());` instead of the unclamped
`opts.jobs.max(1)`. Updated `QueueOpts::jobs`'s doc comment to describe the
cap.

**RED/GREEN:** wrote `worker_count_is_capped_at_spec_count` (four cases:
oversized jobs capped at spec count, jobs already below spec count
unaffected, `jobs: 0` clamped to 1 first, an empty batch still yields 1)
plus a queue-level regression test
`jobs_far_exceeding_spec_count_still_completes_with_correct_outcomes`
(`jobs: 100_000` against a 2-spec batch). Confirmed RED via compile failure
first (`cargo test -p muxsmith-core --lib executor::queue`: `error[E0425]:
cannot find function 'worker_count'`, 4 errors) - this also had the
practical benefit of making it structurally impossible for the risky
100,000-thread queue-level test to execute against the unfixed code (it
would have tried to spawn 100,000 real OS threads via
`std::thread::Scope::spawn`, which panics on a thread-creation failure and
is slow/heavy even when it succeeds). After implementing `worker_count` and
wiring it in, both tests went GREEN; the full `executor::queue` module (8
tests) ran in 0.05s, confirming only 2 workers actually spawn for the
100,000-job batch.

Noted per the finding: `ConcurrencyTracker::max()` alone cannot regression-
test the thread cap, since concurrent *spec* execution is inherently bounded
by the spec count regardless of worker-thread count (each worker dequeues a
distinct index once). The `worker_count` unit tests are the real regression
guard; the queue-level test demonstrates the requested end-to-end behavior
(correctness and speed) but is not on its own proof of the cap.

**Gate:** full suite green (core lib count now 95, +2 from this fix), fmt
clean, clippy clean, deny clean.

## Summary

| # | Severity | Commit | Behavior change | TDD |
|---|----------|--------|------------------|-----|
| 1 | Important | `75c075f` | Yes (Windows kill semantics) | Unit-level RED/GREEN on `resolve_wait`; live wiring test added, GREEN-only |
| 2 | Important | `db9f559` | Yes (JSON shape on profile-load failure) | RED/GREEN via CLI integration tests |
| 3 | Minor | `841db0b` | No (doc only) | N/A |
| 4 | Minor | `4b1ddbf` | Yes (worker cap) | RED (compile failure) / GREEN on `worker_count`; queue-level test added |

All four commits pass the full per-commit gate (`cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo deny check`). Nothing pushed. Scope stayed within the
files named in each finding plus their tests; the one adjacent issue noticed
(Fix 2's `list_languages()`-failure arm) was flagged, not fixed, since it
was not named by the finding.

## Fix 5 (follow-up, coordinator-accepted): mkvmerge_found true on query failure

**Commit:** `9009d34` fix(cli): mkvmerge_found is true when the binary was
found but the query failed

Closes the scope note from Fix 2: on the `list_languages()`-failure arm
(`dry_run.rs`, `run.rs`), `locate()` succeeded - mkvmerge WAS found, only
the subsequent query failed - so the truthful value is `Some(true)`, not
the `Some(false)` Fix 2 had carried over unchanged.

**Changed:** the two call sites flipped to `Some(true)` with their branch
comments updated; `config_only_json`'s doc now spells out the three-way
semantics (`Some(false)` = lookup ran and failed, `Some(true)` = binary
found but query failed, `None`/absent = lookup never ran).

**RED/GREEN:** added `assert_eq!(report["mkvmerge_found"], true, ...)` to
the two existing fake-mkvmerge-stub tests from the fastfollow
(`dry_run_json_emits_a_document_when_the_language_query_fails`,
`run_json_emits_a_document_when_the_language_query_fails`). Confirmed RED
first: both failed with `left: Bool(false), right: true`. After flipping
the call sites, both suites fully GREEN (dry_run_cli 9/9, run_cli 10/10).

**Gate:** full 4-command gate green (workspace tests, fmt, clippy -D
warnings, deny).

The summary table above gains a row:

| # | Severity | Commit | Behavior change | TDD |
|---|----------|--------|------------------|-----|
| 5 | Follow-up | `9009d34` | Yes (JSON field value on query-failure path) | RED/GREEN via the two existing stub tests |
