# Task 11 report: gated end-to-end `run` test

## What was implemented

New file `crates/muxsmith-cli/tests/run_live.rs`, two gated (locate-or-skip)
tests driving the real `muxsmith` binary against real mkvmerge (both
self-skip with `eprintln!` when `mkvmerge --version` fails, mirroring
`run_cli.rs`'s `have_mkvmerge` idiom):

1. `live_run_muxes_two_sources_and_reports_exit_zero`: builds two tiny
   single-subtitle-track source MKVs (SRT fixture pattern, no media codecs
   needed) in a temp source dir, a minimal profile (`SxxExx` pattern,
   `keep`-default subtitle rule), invokes
   `run --source <dir> --output <dir>`, and asserts: exit 0; both outputs
   exist; both re-identify via a direct `mkvmerge -J` spawn as
   `container.recognized == true` / `container.type == "Matroska"` (values
   confirmed by actually running `mkvmerge -J` against a built fixture
   before writing the assertion, not from memory); stdout contains the
   exact `run-summary` line `"2 ok, 0 warning, 0 failed, 0 cancelled"`.

2. `live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched`:
   runs the same batch once to create the outputs, then reruns it with
   `--on-collision skip` and asserts: exit 1 (a Skip-policy collision on an
   on-disk file is diagnostic-only, Warning severity -> `diag_exit_code` 1,
   confirmed by reading `planner.rs::detect_output_collisions`); stdout
   contains no `" ok, "` fragment (the collision empties `specs`, so `run`
   returns before ever touching the queue -- no job starts, no summary
   prints); both outputs are byte-for-byte and mtime-for-mtime unchanged.

## How the untouched-assertion is non-vacuous

Two independent checks, not one:

- **Byte-content equality** (`fs::read` before/after): catches any
  truncate/rewrite/delete-and-recreate regardless of the filesystem's mtime
  resolution.
- **Backdated mtime equality**: before the rerun, each output's mtime is
  deliberately set an hour into the past via `std::fs::File::set_modified`
  (stable since Rust 1.75; no new dependency, in-scope for the test file
  only). The *actual* stored value is re-read via `fs::metadata` right
  after backdating and used as the comparison reference, so any precision
  the filesystem's `utimes` truncates to is already baked in on both sides
  of the later comparison -- no reliance on `SystemTime::now()`'s
  full-precision value surviving a lossy round trip. Any real write at all
  after that point -- even one that reproduces the exact same bytes -- sets
  a fresh mtime at "now" (2026, present wall clock), unmistakably different
  from the one-hour-stale reference, independent of whether the filesystem
  tracks mtime at nanosecond or 1-second granularity.

This combination specifically closes the gap a same-second rerun would
otherwise leave open (content-only comparison cannot tell "never touched"
from "touched but produced identical bytes"; a live, non-backdated mtime
comparison on a 1-second-granularity filesystem could pass vacuously if
both runs land in the same clock second).

## Test evidence (RED then GREEN)

1. **Broad demonstration**: flipped five assertions to deliberately wrong
   expected values at once (Matroska type string, the summary count, the
   rerun exit code, content `assert_eq!` -> `assert_ne!`, mtime reference ->
   `SystemTime::now()`), ran `cargo test -p muxsmith-cli --test run_live`,
   and got two real failures with mkvmerge's actual JSON and the actual
   exit code in the panic output (not a compile error, not a vacuous pass).
2. **Targeted probe on the critical mechanism**: restored the file, then
   inserted a temporary line right after the backdating step that silently
   rewrote `out1` with the *exact same bytes* (`fs::write(&out1,
   &content1)`) -- simulating "a job silently reran and reproduced an
   identical file", the one case content-equality alone cannot catch. Ran
   only the rerun test: it failed specifically on the mtime assertion
   (`out1 mtime changed: rerun touched an existing output`, with the actual
   pre/post `SystemTime` values in the panic), confirming the mtime check
   is the one doing the real work here, not a redundant no-op.
3. Removed the probe, diffed the file byte-for-byte against the
   pre-probe version to confirm a clean restore, then reran: both tests
   green.

## Files changed

- `crates/muxsmith-cli/tests/run_live.rs` (new, 234 lines after `cargo fmt`)

## Self-review findings

- **Completeness**: both brief cases present with every named assertion
  (exit codes, `-J` identification, summary line, untouched outputs).
- **Non-vacuous untouched-assertion**: verified above, not just asserted.
- **Hermeticity**: each test owns its own `tempfile::tempdir()`; no shared
  mutable state, no `PATH` mutation, no reliance on execution order; both
  self-skip identically and independently when mkvmerge is absent.
- **Output pristine**: `cargo fmt --all --check` clean, `cargo clippy
  --workspace --all-targets -- -D warnings` clean (zero warnings, not just
  zero errors), `cargo deny check` clean, `cargo test --workspace` fully
  green (grepped the full run for `FAILED`/`error[`/`test result: FAILED`
  and confirmed none, not just eyeballing the tail). File is pure ASCII
  (checked with `grep -P '[^\x00-\x7F]'`).
- Diffed the committed file against my working draft: identical except for
  one `cargo fmt` reformat of a multi-line `assert!` call, confirming no
  leftover RED-step probe code made it into the commit.

## Issues or concerns

None. This is Plan 4's last task; per the plan's own self-review section,
close-out (whole-branch review, SI-2 journal, `.superpowers/sdd/` salvage,
HANDOFF refresh, push) is the controller's job, not this task's.
