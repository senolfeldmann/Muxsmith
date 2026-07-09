# Task 4 report: `--on-collision` flag on dry-run + `CollisionArg`

## What I implemented

- `crates/muxsmith-cli/src/cli.rs`: added `pub enum CollisionArg` (clap
  `ValueEnum`, variants `Error`/`Skip`/`Overwrite`, doc comments per
  variant per `#![deny(missing_docs)]`) with `pub fn policy(self) ->
  muxsmith_core::profile::model::CollisionPolicy` (1:1 match, no default
  arm). Added `on_collision: Option<CollisionArg>` to the `DryRun` variant
  of `Cmd`, `#[arg(long, value_enum)]`, positioned before `json` exactly
  as the brief's interface spec requires.
- `crates/muxsmith-cli/src/main.rs`: destructures `on_collision` out of
  `Cmd::DryRun` and passes `on_collision.map(cli::CollisionArg::policy)`
  into `commands::dry_run::run`, in the same position (before `json`).
- `crates/muxsmith-cli/src/commands/dry_run.rs`: `run` gains
  `on_collision: Option<CollisionPolicy>` before `json`; the hardcoded
  `on_collision: None` in the `RunInputs` literal (former line 78) is
  replaced with the threaded-through parameter. Added the
  `muxsmith_core::profile::model::CollisionPolicy` import.
- `crates/muxsmith-cli/tests/dry_run_cli.rs`: new test
  `dry_run_on_collision_flag_overrides_default_error_policy`, following
  the file's existing inline `Command::cargo_bin("muxsmith")...output()`
  invocation pattern (no separate helper function exists in this file;
  each test builds its own `Command` the same way, which I mirrored).

No other files touched; no other callers of `dry_run::run` existed
(verified via grep across `crates/`), so no ripple beyond the four listed
files.

## What I tested and results

Test fixture: an mkvmerge-built `Show.S01E01.mkv` (audio+subtitle) in a
dedicated source tempdir, matched by a one-rule (audio) profile, with a
pre-existing dummy file at the exact planned output path (`keep` filename
= source stem + `.mkv`) in a *separate* output tempdir.

- Default invocation (no `--on-collision`): exits 2 (collision policy
  falls back to the profile default `error`).
- `--on-collision skip --json`: exits 1, and the per-file diagnostics
  array carries a `code: "output-collision"` entry with `severity:
  "warning"`.

One fixture bug found and fixed during development: nesting the output
directory *inside* the source directory caused `discovery::scan_primaries`
(which walks recursively) to pick up the pre-existing collision file
itself as a second primary, producing an unrelated
`duplicate-identifier` + `source-overwrite` pair and masking the
intended `output-collision` signal entirely. Fixed by using two sibling
tempdirs (`src_dir`, `out_dir`) instead of `out_dir = src_dir.join("out")`.
This is a test-fixture artifact of the (correct) recursive-discovery
behavior, not a product bug; no core/CLI code needed a change for it.

Full workspace gate, all four green:

```
cargo test --workspace        -> all suites "ok" (0 failed each); see
                                  totals below
cargo fmt --all --check       -> no output (clean)
cargo clippy --workspace --all-targets -- -D warnings -> Finished, no
                                  warnings
cargo deny check               -> advisories ok, bans ok, licenses ok,
                                  sources ok
```

`cargo test --workspace` per-suite pass counts (no failures anywhere):
5, 0, 1, 2, 5, 6, 75, 13, 3, 1, 3, 49, 8, 5, 9, 14, 15, 0, 0, 2, 0, 0, 0
passed (doc-tests and a couple of 0-test unittest binaries account for
the zeros).

## TDD evidence

**RED** (before implementing the flag):

```
$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_on_collision_flag_overrides_default_error_policy -- --nocapture
...
thread 'dry_run_on_collision_flag_overrides_default_error_policy' panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:356:5:
assertion `left == right` failed: stdout: , stderr: error: unexpected argument '--on-collision' found

  tip: to pass '--on-collision' as a value, use '-- --on-collision'

Usage: muxsmith dry-run --source <SOURCE> --output <OUTPUT> <PROFILE>

  left: Some(2)
 right: Some(1)
test dry_run_on_collision_flag_overrides_default_error_policy ... FAILED
```

Exactly the "unknown flag" RED the brief specifies: clap rejects
`--on-collision` outright (exit 2, usage error) before the flag exists.

**GREEN** (after implementing `CollisionArg` + wiring):

```
$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_on_collision_flag_overrides_default_error_policy -- --nocapture
running 1 test
test dry_run_on_collision_flag_overrides_default_error_policy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.19s
```

## Files changed

- `crates/muxsmith-cli/src/cli.rs`
- `crates/muxsmith-cli/src/main.rs`
- `crates/muxsmith-cli/src/commands/dry_run.rs`
- `crates/muxsmith-cli/tests/dry_run_cli.rs`

## Self-review findings

- **Completeness**: all three interface elements from the brief present
  and matching the exact signature (`CollisionArg` enum + `policy()`,
  `on_collision: Option<CollisionPolicy>` before `json` in
  `dry_run::run`, flag on `DryRun` in `cli.rs`). Verified `--help` output
  renders sane docs for the new flag and its three possible values.
- **Discipline**: nothing beyond the brief. Did not touch `run` (Task 8),
  did not add validation logic in the CLI crate (core still owns
  `CollisionPolicy` semantics), did not touch locale/Fluent files (no new
  user-facing prose introduced, `CollisionArg` is a CLI-local mapping
  type only).
- **Testing**: the new test exercises real end-to-end CLI behavior
  through a spawned binary (not a unit test of the mapping function
  alone), covers both branches (default error-exit-2, explicit
  skip-exit-1-with-warning-diagnostic), and asserts on the actual JSON
  diagnostic shape rather than just the exit code.
- **Output pristine**: `cargo fmt --all --check` clean, clippy clean with
  `-D warnings`, `cargo deny check` clean.
- Confirmed via grep that no other crate/test calls `dry_run::run`, so
  the added parameter has no other call sites needing updates.
- Confirmed the diff contains no forbidden typography glyphs (em/en
  dash, curly quotes, ellipsis, NBSP).

## Issues or concerns

None. Task completed cleanly within the brief's scope.
