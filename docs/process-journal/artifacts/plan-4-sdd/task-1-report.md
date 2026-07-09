# Task 1 report: Executor spawn seam

## What was implemented

- `crates/muxsmith-core/src/executor/mod.rs`: module doc comment (spec 6,
  D13) plus `pub mod spawn;`.
- `crates/muxsmith-core/src/executor/spawn.rs`: the `Spawn`/`RunningJob`/
  `Killer`/`SpawnError` interfaces, `LiveSpawner`/`LiveJob` (real
  `mkvmerge --gui-mode` child process), `FakeSpawner`/`FakeJob` (scripted
  fake for unit tests), all per the brief's verbatim signatures. Every pub
  item carries a doc comment (`#![deny(missing_docs)]`).
- `crates/muxsmith-core/src/lib.rs`: added `pub mod executor;` (alphabetical
  slot between `discovery` and `identify`).
- `crates/muxsmith-core/tests/executor_live.rs` (new): gated live grammar
  test, locate-or-skip idiom mirroring `identify_live.rs` /
  `command_integration.rs`.

## Observed `--gui-mode` grammar (verbatim, mkvmerge v100.0 "Do Hot Girls
Like Chords" 64-bit, installed at `/home/linuxbrew/.linuxbrew/bin/mkvmerge`)

Probed in a throwaway dir outside the repo
(`/tmp/.../scratchpad/gm-probe`, not committed).

Successful 1-track SRT-to-MKV mux:

```
$ printf '1\n00:00:00,000 --> 00:00:01,000\nHello\n' > seed.srt
$ mkvmerge --gui-mode -o gm-probe.mkv seed.srt ; echo "exit=$?"
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
'seed.srt': Using the demultiplexer for the format 'SRT subtitles'.
'seed.srt' track 0: Using the output module for the format 'text subtitles'.
The file 'gm-probe.mkv' has been opened for writing.
#GUI#progress 100%
#GUI#progress 100%
The cue entries (the index) are being written...
Multiplexing took 0 seconds.
exit=0
```

Warning (missing track id on `--default-track-flag 9:1`, exit 1, output
still produced):

```
$ mkvmerge --gui-mode -o gm-probe2.mkv --default-track-flag 9:1 seed.srt ; echo "exit=$?"
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
'seed.srt': Using the demultiplexer for the format 'SRT subtitles'.
'seed.srt' track 0: Using the output module for the format 'text subtitles'.
#GUI#warning 'seed.srt': A track with the ID 9 was requested but not found in the file. The corresponding option will be ignored.
The file 'gm-probe2.mkv' has been opened for writing.
#GUI#progress 100%
#GUI#progress 100%
The cue entries (the index) are being written...
Multiplexing took 0 seconds.
exit=1
```

Error (nonexistent input file, exit 2, no output; a second probe against an
unwritable output directory produced the same `#GUI#error <message>` shape
with no leading quoted filename):

```
$ mkvmerge --gui-mode -o gm-probe3.mkv does-not-exist.srt ; echo "exit=$?"
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
#GUI#error The file 'does-not-exist.srt' could not be opened for reading: open file error.
exit=2
```

Grammar recorded in `spawn.rs`'s `LiveSpawner` doc comment (verbatim, ready
for Task 2 to lift as parser fixtures):

- `#GUI#progress NN%` -- one or more per run, final line always
  `#GUI#progress 100%` on success.
- `#GUI#warning '<file>': <message>` -- exit 1, output kept.
- `#GUI#error <message>` -- exit 2, no leading quoted filename (unlike
  warning).

## What was tested

- `spawn.rs` unit tests (`FakeSpawner`/`FakeJob`), from the brief verbatim:
  `fake_spawner_scripts_lines_and_exit`, `fake_killer_ends_stream_and_wait_returns_none`.
- `tests/executor_live.rs::live_gui_mode_progress_reaches_100_percent`:
  spawns a real mkvmerge via `LiveSpawner` on the SRT fixture pattern,
  collects every stdout line plus the exit code, asserts exit `Some(0)`, at
  least one `#GUI#progress ` line, and that the last progress line is
  exactly `#GUI#progress 100%`. Self-skips (`eprintln!` + return) when
  `Mkvmerge::locate()` fails.

## TDD evidence

RED -- `cargo test -p muxsmith-core fake_spawner` against a `spawn.rs`
containing only the `#[cfg(test)] mod tests` block (no `Spawn`/`FakeSpawner`
implementation yet), with `executor` already wired into `lib.rs`:

```
error[E0433]: cannot find type `FakeSpawner` in this scope
 --> crates/muxsmith-core/src/executor/spawn.rs:7:20
  |
7 |         let fake = FakeSpawner::script(vec!["#GUI#progress 50%".into()], Some(0));
  |                    ^^^^^^^^^^^ use of undeclared type `FakeSpawner`
...
error: could not compile `muxsmith-core` (lib test) due to 2 previous errors; 1 warning emitted
```

Expected: the fake's types did not exist yet, so the test file fails to
compile -- a genuine, observable RED (chosen over the weaker "0 tests
matched, exit 0" reading of "module absent", since that produces no useful
failure evidence).

GREEN -- after implementing `Spawn`/`RunningJob`/`LiveSpawner`/`FakeSpawner`
per the brief:

```
$ cargo test -p muxsmith-core executor::spawn
running 2 tests
test executor::spawn::tests::fake_killer_ends_stream_and_wait_returns_none ... ok
test executor::spawn::tests::fake_spawner_scripts_lines_and_exit ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 75 filtered out; finished in 0.00s
```

```
$ cargo test -p muxsmith-core --test executor_live
running 1 test
test live_gui_mode_progress_reaches_100_percent ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

## Gate (all four, final run before commit)

- `cargo test --workspace`: all green (`muxsmith-core` lib 77 passed,
  `executor_live` 1 passed, every other suite unaffected, doc-tests 0/0/0).
- `cargo fmt --all --check`: clean (one `cargo fmt --all` pass applied
  mid-task to reflow `LiveJob::wait`'s method-chain formatting; re-checked
  clean after).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/src/executor/mod.rs` (new)
- `crates/muxsmith-core/src/executor/spawn.rs` (new)
- `crates/muxsmith-core/src/lib.rs` (modified: `pub mod executor;`)
- `crates/muxsmith-core/tests/executor_live.rs` (new)

Commit: `3e1ebf3 feat(executor): Spawn seam with live mkvmerge and scripted
fake (D13)`.

## Self-review

- Completeness: all 7 brief steps done in order (probe, RED test, verified
  RED, implementation, gated live test, gate, commit).
- Quality: doc comments on every pub item (`FakeSpawner::script`,
  `FakeSpawner::spawned`, `SpawnError`, `Killer`, `Spawn`, `RunningJob`,
  `LiveSpawner` + its field, `LiveSpawner::mkvmerge` field); the `LiveSpawner`
  doc comment carries the full verbatim grammar rather than a placeholder.
  Private types (`LiveJob`, `FakeJob`) carry doc comments too even though
  not required by `deny(missing_docs)`, for readability parity with the
  pub items around them.
- Discipline: implementation matches the brief's interfaces exactly
  (`Spawn`, `RunningJob`, `Killer`, `LiveSpawner`, `FakeSpawner`,
  `SpawnError`); no queue, job-state, or progress-parsing logic added
  (that is Task 2/3's scope). `stderr(Stdio::null())` and `--gui-mode`
  argument order match the brief's `LiveSpawner::spawn` verbatim.
- Testing: the two brief-mandated fake tests pass unchanged from the brief
  text; the gated live test asserts real, observed behavior (exit code and
  exact final progress line), not an assumed shape. No stray warnings in
  any of the four gate commands.

## Issues or concerns

- The brief's Step 3 expected-failure note reads "FAIL (module absent)".
  I interpreted this as "the fake's implementation types are absent" and
  produced RED via a genuine compile error (undeclared `FakeSpawner`) with
  `executor` already wired into `lib.rs`/`mod.rs`, rather than leaving the
  module unwired (which would have made the test filter match 0 tests and
  exit 0 -- weak, unconvincing RED evidence). This produces stronger TDD
  evidence but is a documented judgment call in case a stricter literal
  reading was intended.
- No other deviations, ambiguities, or blockers encountered. mkvmerge v100
  was available and used for both the grammar probe and the gated test (it
  did not need to self-skip in this environment).
