# Task 3 report: robust event-stream reads - no truncation, no hang (#9)

## Summary

`LiveJob::next_line` used `BufReader::read_line` into a `String`, which
fails on the first byte sequence that is not valid UTF-8. The old code
mapped that `Err` to `None` (same as true EOF), so a single malformed
mkvmerge output line silently ended the event stream: `run_job`'s drain
loop (`while let Some(line) = running.next_line()`) stopped there, the
child process kept running, and once the (now undrained) OS pipe buffer
filled, the child blocked on `write()` forever - and `run_job`'s later
`running.wait()` (a blocking `waitpid`) then hung indefinitely waiting for
a child that could never exit.

Fixed by reading byte-wise (`read_until(b'\n', ...)`) and lossily decoding
with `String::from_utf8_lossy`, so a non-UTF-8 line degrades to a line
containing `U+FFFD` instead of ending the stream. Only a true `Ok(0)` (EOF)
ends it now.

## Files changed

- `crates/muxsmith-core/src/executor/spawn.rs`: extracted `read_next_line<R:
  BufRead>(r: &mut R) -> Option<String>` out of `LiveJob::next_line` (now a
  thin delegate); added a unit test.
- `crates/muxsmith-core/tests/executor_no_hang_live.rs` (new): live,
  gated-by-platform regression test pinning the no-hang behavior end to end
  through `run_job`.

`RunningJob::next_line` signature is unchanged; only its behavior changed,
as specified.

## TDD evidence

### Step 1/2 - RED then GREEN (spawn.rs unit test)

RED - added `read_next_line_survives_non_utf8_bytes_without_truncating`
(drives the not-yet-existing `read_next_line` against a `Cursor` containing
`b"ok line\n\xFF\xFE broken\nafter\n"`), ran:

```
cargo test -p muxsmith-core --lib executor::spawn::tests::read_next_line_survives_non_utf8_bytes_without_truncating
```

```
error[E0425]: cannot find function `read_next_line` in this scope
   --> crates/muxsmith-core/src/executor/spawn.rs:275:20
...
error: could not compile `muxsmith-core` (lib test) due to 4 previous errors
```

Expected: the function did not exist yet, only the test did - a clean
compile-time RED.

GREEN - implemented `read_next_line` (`read_until` + lossy decode) and made
`next_line` delegate to it, ran:

```
cargo test -p muxsmith-core --lib executor::spawn::
```

```
running 6 tests
test executor::spawn::tests::fake_killer_ends_stream_and_wait_returns_none ... ok
test executor::spawn::tests::fake_spawner_scripts_lines_and_exit ... ok
test executor::spawn::tests::read_next_line_survives_non_utf8_bytes_without_truncating ... ok
test executor::spawn::tests::resolve_wait_returns_none_when_killed_even_if_the_os_reports_a_code ... ok
test executor::spawn::tests::resolve_wait_passes_the_raw_code_through_when_not_killed ... ok
test executor::spawn::tests::live_killer_then_wait_returns_none ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 99 filtered out; finished in 0.00s
```

### Step 3 - RED then GREEN (live no-hang regression, `run_job` end to end)

Wrote `crates/muxsmith-core/tests/executor_no_hang_live.rs`:
`run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging`.
It writes a scripted `#!/bin/sh` fake child (chmod +x) that prints `ok
line`, then a line with raw invalid-UTF-8 bytes (`\377\376`, POSIX printf
octal escapes), then ~140KB of filler (`yes | head -n 2000`, comfortably
above a pipe's usual 64KiB OS buffer), then `TAIL_MARKER`, spawns it through
the real `LiveSpawner` (not `FakeSpawner`, which is pure in-memory and
cannot reproduce OS pipe backpressure), and runs `run_job` on a worker
thread with a 10s `recv_timeout` bound (so a real regression fails the test
cleanly instead of hanging `cargo test` itself).

RED - verified this reproduces the actual pre-fix hang, not just a
theoretical one: stashed the `spawn.rs` fix only (`git stash push -- .../spawn.rs`), keeping the new test, and ran:

```
cargo test -p muxsmith-core --test executor_no_hang_live
```

```
thread 'run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging' panicked at crates/muxsmith-core/tests/executor_no_hang_live.rs:76:69:
run_job did not complete within 10s - the pipe-full hang this test pins (#9) is back: Timeout

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; finished in 10.00s
```

Confirmed no leaked/zombie child process after the timeout (`ps aux` showed
none related; the OS closes the test binary's pipe fds on process exit,
which delivers `SIGPIPE`/`EPIPE` to the still-blocked child and kills it).

GREEN - restored the fix (`git stash pop`), reran:

```
cargo test -p muxsmith-core --test executor_no_hang_live
```

```
running 1 test
test run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Completes in well under 1s (not the 10s timeout), and asserts: `outcome.state
== JobState::Ok`; the pre-bad-line `"ok line"` survived; a decode-degraded
line containing `U+FFFD` was captured; and the very last captured line is
`"TAIL_MARKER"` - proving the full >64KiB tail after the bad line was
drained, not truncated.

## Gate results

All run from the worktree root (`/home/senol/Git/Muxsmith/.worktrees/stream-a`,
branch `plan55-stream-a`), foreground, once after the implementation was
final:

- `cargo fmt --all --check` - clean, no output.
- `cargo clippy --workspace --all-targets -- -D warnings` - clean.
- `cargo test --workspace` - all green: 224 tests in `muxsmith-core`
  (including the real-mkvmerge-backed `executor_live.rs` and the new
  `executor_no_hang_live.rs`), plus all `muxsmith-cli` and `muxsmith-gui`
  suites; zero failures anywhere.
- `cargo deny check` - `advisories ok, bans ok, licenses ok, sources ok`.
- `pnpm lint` - clean (eslint, no findings).
- `pnpm build` - `vue-tsc --noEmit && vite build` succeeded.
- `pnpm check:i18n` - `ok (16 source files scanned, 171 catalog ids, 12
  unused warning(s))` - the 12 warnings are pre-existing unused catalog ids
  unrelated to this change.
- `pnpm test:e2e` - 3/3 Playwright smoke tests passed.

`pnpm install --frozen-lockfile` was run once first since `node_modules`
was missing in this worktree.

## Self-review

- **Completeness**: all four brief steps done - extracted testable
  `read_next_line`, implemented lossy decode + EOF-only termination,
  live no-hang regression pinning the `job.rs` post-`None` `wait()` path,
  full gate, commit with the exact specified message and trailer.
- **Quality**: `read_next_line`'s doc comment states the *why* (mkvmerge can
  echo non-UTF-8 filenames/OS strings) and the exact contract (only `Ok(0)`
  ends the stream), not a name echo. The `Err(_) => None` arm is kept
  distinct from `Ok(0) => None` per the brief's explicit ask, with a comment
  explaining both currently map to the same return but are conceptually
  different (decode issues can no longer reach this arm at all).
- **YAGNI**: no new public API surface; `read_next_line` is private,
  `next_line`'s signature is unchanged as required. No speculative options
  (chunk size, error-reporting channel, etc.) added beyond the brief's ask.
- **Test realism**: the Step 3 test exercises the real `LiveSpawner` /
  `LiveJob` / OS pipe path, not `FakeSpawner` (which is pure in-memory and
  structurally cannot reproduce pipe backpressure) - this is the only way
  to actually pin the `wait()`-blocks-on-a-stuck-child hang, not just the
  decode logic. Verified this deliberately: reran it against the pre-fix
  code via a scoped `git stash` and watched it fail with the exact 10s
  timeout message, then restored the fix and watched it pass in <1s. No
  leaked/zombie processes after the RED run.
- **Design decision worth flagging**: the orchestrating instructions said
  "gated tests self-skip when mkvmerge is absent... mirror [that] pattern,"
  pointing at `mkvmerge_runtime.rs`/`executor_live.rs`. I did not gate this
  test on `Mkvmerge::locate()`. Reasoning: those two files test real
  mkvmerge's actual `--gui-mode` grammar/version behavior, which genuinely
  requires the real binary. This bug is about OS pipe/process mechanics
  (non-UTF-8 bytes, pipe-full backpressure), not mkvmerge's grammar, and the
  brief itself offers "a scripted fake child (or the fake-mkvmerge helper)"
  as the mechanism - i.e., it does not need real mkvmerge at all. The
  precedent that actually matches this test's nature is already in
  `spawn.rs` itself: `live_killer_then_wait_returns_none`, which spawns a
  chmod'd shell script directly through `LiveSpawner` and is gated only
  `#[cfg(unix)]`, with no mkvmerge-presence check. I mirrored that pattern
  instead and documented the reasoning in the new test file's own doc
  comment. Gating on `Mkvmerge::locate()` while never actually invoking the
  located binary would have been a misleading gate (self-skip on a
  precondition the test doesn't depend on) - I judged this the better
  engineering call, but flagging it explicitly since it deviates from the
  orchestrator's literal wording.
- **Pristine output**: `git status --short` after the commit is clean;
  only the two intended files are in the commit (verified via explicit
  `git add <path> <path>`, no `-A`/`.`).

## Concerns

- None blocking. The one item worth a second pair of eyes is the gating
  decision above - if project convention actually wants every "(gated,
  live)"-labeled test to carry the `Mkvmerge::locate()` skip-guard
  regardless of whether it uses the real binary, this test would need that
  guard added (harmless to add, just redundant given the fake script is
  what's actually exercised).

## Fix wave (file-level cfg gate)

Added `#![cfg(unix)]` inner attribute after the module doc comment in
`executor_no_hang_live.rs` to gate the entire test module (imports + test fn)
on Unix only, preventing unused imports on non-Unix targets that trigger
`cargo clippy -D warnings`.

**Verification results** (from `/home/senol/Git/Muxsmith/.worktrees/stream-a`):

1. `cargo clippy --workspace --all-targets -- -D warnings`
   - Result: `Finished dev profile [unoptimized + debuginfo] target(s) in 0.18s`
   - Status: ✓ PASS

2. `cargo test -p muxsmith-core --test executor_no_hang_live`
   - Result: `test run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging ... ok`
   - Status: ✓ PASS (1 passed; completed in 0.00s)

3. `cargo fmt --all --check`
   - Result: no output (clean)
   - Status: ✓ PASS
