# Task 9 report: `run --json` final document

## What I implemented

`muxsmith run --json` now emits exactly one final document on stdout on
every code path, instead of emitting nothing (the two `TODO(Task 9)` sites
from Task 8). The document is dry-run's base shape (`config_diagnostics`,
`files`, `batch_diagnostics`, `suggestions`, and on the mkvmerge-missing
path `mkvmerge_found`) extended with two fields (D15):

- `jobs`: one entry per executed `JobOutcome`, with `index` (position in
  the batch), `output` (rendered output path), and every `JobOutcome`
  field (`state`, `exit_code`, `warnings`, `errors`, `duration_ms`) via its
  existing `Serialize` impl.
- `summary`: `{ ok, warning, failed, cancelled }` counts, the same
  worst-of tally as the human `run-summary` line.

Both `TODO(Task 9)` sites are wired:

1. **mkvmerge not found** (`run.rs`, the `Mkvmerge::locate()` error arm):
   builds on `dry_run::config_only_json` (now `pub(crate)`), extended with
   an empty `jobs` array and a zeroed `summary` via the same builder.
2. **`specs.is_empty()`** (nothing plans cleanly enough to mux): builds on
   `dry_run::batch_json` (now `pub(crate)`), likewise extended with an
   empty `jobs` array and a zeroed `summary`.
3. **The main success path** (after the queue finishes): builds on
   `dry_run::batch_json` again, extended with the real `outcomes` and a
   cloned copy of `outputs` (the original is moved into `MilestoneState`
   for human-mode rendering, so json mode needs its own copy, cloned once
   up front regardless of mode).

New function `run_json_document(base, outcomes, outputs) -> serde_json::Value`
in `run.rs` is the single place that does the `jobs`/`summary` extension;
all three call sites just choose which base document to hand it.

In json mode all human lines stay suppressed exactly as before (Task 8's
existing `if !json` guards); the document is the only stdout output.
NDJSON event streaming was **not** built (deferred to v1.x per D15, as
instructed).

Exit-code folding (`diag_exit_code`, `job_exit_code`, the `cancel.load` ->
130 branch, the final `std::cmp::max`) is untouched; the diff only adds a
`dry_run` import for those symbols' surrounding module.

## What I tested and results

**Unit tests** (`crates/muxsmith-cli/src/commands/run.rs`, `mod tests`),
`serde_json::Value` equality throughout, no string comparison:

- `run_json_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty`:
  empty outcomes/outputs -> `jobs: []`, `summary` all zero, base fields
  passed through unchanged.
- `run_json_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary`:
  four outcomes (ok/warning/failed/cancelled) -> exact expected `jobs`
  array (matching the brief's example shape for entry 0: `index: 0`,
  `output`, `state: "ok"`, `exit_code: 0`, `warnings: []`, `errors: []`,
  `duration_ms: 12400`) and `summary: {ok:1, warning:1, failed:1,
  cancelled:1}`.

**CLI integration tests** (`crates/muxsmith-cli/tests/run_cli.rs`), all gated
appropriately on `have_mkvmerge()` where a real mkvmerge invocation is
needed:

- `run_json_on_a_real_mux_reports_a_populated_jobs_array_and_summary`: a
  real fixture mux (tone.wav + sub.srt seeds), `run --json`, asserts exit
  0, zero human lines in stdout, `files` has 1 entry, `jobs` has exactly
  one entry with `state: "ok"`, `exit_code: 0`, empty warnings/errors, a
  numeric `duration_ms`, the real rendered output path (`output.filename`
  defaults to `keep`, so `Show.S01E01.mkv`, not the identifier), the output
  file actually exists on disk, and `summary == {ok:1,...}`. Exercises the
  main success path end to end.
- `run_json_on_specs_empty_from_a_bad_regex_still_emits_a_document_with_empty_jobs`:
  invalid `input.pattern` regex -> exit 2, `config_diagnostics` carries
  `invalid-regex`, `jobs: []`, zeroed `summary`. Exercises the
  `specs.is_empty()` path via a planning error.
- `run_json_on_specs_empty_from_an_empty_source_dir_exits_clean_with_a_zeroed_summary`:
  clean profile, no matching files in the source dir -> exit 0, `files:
  []`, `jobs: []`, zeroed `summary`. Exercises the `specs.is_empty()` path
  via a genuinely empty (not erroring) batch, distinguishing that from the
  bad-regex case above.
- `run_json_surfaces_the_mkvmerge_not_found_document`: PATH forced empty
  -> exit 2, `mkvmerge_found: false`, `config_diagnostics` still carries
  `invalid-regex` (superset-of-validate guarantee), `jobs: []`, zeroed
  `summary`. Exercises the mkvmerge-not-found path. Does not depend on the
  test machine's actual mkvmerge presence.

Test run (this machine has mkvmerge v100.0 installed, so every test ran,
none skipped):

```
$ cargo test -p muxsmith-cli
...
running 20 tests   (lib, includes the 2 new unit tests)          ... ok
running 6 tests    (dry_run_cli.rs, unaffected by this change)   ... ok
running 7 tests    (run_cli.rs: 3 pre-existing + 4 new)          ... ok
```

## TDD evidence

**RED** - added the two `run_json_document` unit tests before the function
existed:

```
$ cargo test -p muxsmith-cli --lib run_json_document
error[E0425]: cannot find function `run_json_document` in this scope
   --> crates/muxsmith-cli/src/commands/run.rs:633:19
    |
633 |         let doc = run_json_document(base, &[], &[]);
    |                   ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find function `run_json_document` in this scope
   --> crates/muxsmith-cli/src/commands/run.rs:657:19
```

Expected failure: the builder did not exist yet, so the test file could
not even compile. This confirms the tests exercise real, not-yet-built
behavior.

**GREEN** - after implementing `run_json_document`:

```
$ cargo test -p muxsmith-cli --lib run_json_document
running 2 tests
test commands::run::tests::run_json_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty ... ok
test commands::run::tests::run_json_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Then wired the two TODO sites plus the main success path, added the four
`run_cli.rs` integration tests, and reran the full crate suite (all green,
see above).

## Gate (all four, run before commit)

```
$ cargo test --workspace          -> 25 "test result: ok" blocks, 0 failures
$ cargo fmt --all --check         -> exit 0 (after `cargo fmt --all` fixed
                                      3 pre-fmt formatting diffs from my
                                      own edits)
$ cargo clippy --workspace --all-targets -- -D warnings   -> clean, 0 warnings
$ cargo deny check                -> advisories ok, bans ok, licenses ok, sources ok
```

## Files changed

- `crates/muxsmith-cli/src/commands/dry_run.rs`: `batch_json` and
  `config_only_json` changed from private to `pub(crate)`, each with a
  doc-comment line noting `run --json` (D15) reuses them. No behavior
  change to `dry-run` itself.
- `crates/muxsmith-cli/src/commands/run.rs`: `dry_run` added to the
  `crate::commands` import; new `run_json_document` builder + 2 unit
  tests; both `TODO(Task 9)` sites wired; main success path now prints the
  json document in the `else` arm alongside the existing `!json` ->
  `render_summary` arm; `outputs` cloned once (`json_outputs`) before it is
  moved into `MilestoneState`.
- `crates/muxsmith-cli/tests/run_cli.rs`: module doc updated to mention
  Task 9; `have_mkvmerge()` helper added (matches `dry_run_cli.rs`'s
  existing pattern); 4 new integration tests (see above).

## Self-review findings

- **Completeness**: both TODO sites handled; the main success path (which
  had no TODO marker but obviously also needed the document) is wired too.
  Document shape matches the brief's example exactly (field names,
  `state` values via `JobState`'s existing `snake_case` `Serialize`,
  `index`/`output` added on top of `JobOutcome`'s own fields).
- **Discipline**: no NDJSON/`--json-events` anywhere in the diff (grepped
  clean). No changes to `diag_exit_code`, `job_exit_code`, the `cancel`
  check, or the final `std::cmp::max` fold; the diff only threads a new
  `dry_run` import through that region. Nothing beyond the brief's scope
  was touched.
- **Testing**: the builder unit test compares `serde_json::Value` via
  `assert_eq!`, not string comparison, as the brief requires. The
  `run_json_document_maps_outcomes_...` test's expected `jobs[0]` entry is
  a literal transcription of the brief's own example JSON (down to
  `duration_ms: 12400`), so the shape is asserted against the spec text,
  not just against whatever the implementation happened to produce.
- **Output pristine**: verified via the real-mux integration test that
  stdout under `--json` contains no `"... start"` or `" ok, "` fragments
  (the human milestone/summary markers) and parses as a single JSON
  document with `serde_json::from_slice`.
- One design note worth flagging explicitly: I found and fixed a wrong
  assumption while writing the real-mux test - `output.filename` defaults
  to `keep` mode, which is `file_stem() + ".mkv"` (the *whole* source
  stem, e.g. `Show.S01E01.mkv`), not the parsed identifier (`S01E01`). The
  test asserts against the correct rendered path; I mention it because a
  wrong assumption here would have been a silent, misleading test.

## Issues or concerns

None. All four gate commands are green, both TODO sites are resolved, the
exit-code contract (D15's worst-of fold) is unchanged, and NDJSON stays
out of scope per the brief.
