# Task 10 report: SIGINT via ctrlc - graceful cancel, kill in-flight, exit 130

## What was implemented

- `crates/muxsmith-cli/Cargo.toml`: added `ctrlc = "3"` to `[dependencies]` (resolved to `3.5.2`).
- `crates/muxsmith-cli/src/commands/run.rs`: installed the single-level SIGINT handler exactly
  as specified in the brief, placed right after `let cancel = Arc::new(AtomicBool::new(false));`
  and before the `mpsc::channel()` / `thread::scope` block that spawns `run_queue`:

  ```rust
  let cancel = Arc::new(AtomicBool::new(false));

  // Single-level SIGINT (D16): first Ctrl-C requests graceful cancel
  // (queue kills in-flight, partials deleted, summary printed, exit 130);
  // a second Ctrl-C during cleanup force-exits immediately.
  let handler_cancel = Arc::clone(&cancel);
  let _ = ctrlc::set_handler(move || {
      if handler_cancel.swap(true, Ordering::SeqCst) {
          std::process::exit(130);
      }
  });
  ```

  The pre-existing `queue_cancel = Arc::clone(&cancel)` (Task 8) is the same `cancel` this
  handler flips, so `run_queue` observes the flag from inside its worker loop with no further
  wiring needed. The post-queue check `if cancel.load(Ordering::SeqCst) { return 130; }` (also
  Task 8, already sitting after the summary print) now actually engages: previously `cancel`
  never flipped in this task's scope, so that branch was structurally present but dead code;
  it is live now.
- Updated the `run` function's doc comment (previously described `cancel` as "plain,
  never-flipped ... so the 130 branch is structurally present but currently unreachable") to
  describe the real SIGINT behavior (D16): first Ctrl-C requests graceful cancel, second
  force-exits during cleanup.
- Removed the now-stale "Task 10 wires SIGINT onto this; it never flips in this task's scope"
  comment above `let cancel = ...`, since Task 10 is this task.

No changes to `deny.toml` were needed (see below).

## ctrlc's dependency tree as cargo deny saw it

`cargo build --workspace` added 7 new crates to `Cargo.lock` (others `ctrlc` depends on, e.g.
`libc`, `bitflags`, `cfg-if`, were already present in the tree from other dependencies and did
not need adding):

| crate | version | license (per `cargo metadata`) |
|---|---|---|
| `ctrlc` | 3.5.2 | MIT/Apache-2.0 |
| `nix` | 0.31.3 | MIT |
| `cfg_aliases` | 0.2.1 | MIT |
| `block2` | 0.6.2 | MIT (macOS-only, via `ctrlc`'s `windows-sys`/`objc2` platform branches) |
| `dispatch2` | 0.3.1 | Zlib OR Apache-2.0 OR MIT |
| `objc2` | 0.6.4 | MIT |
| `objc2-encode` | 4.1.0 | MIT |

`cargo tree --target=all -p ctrlc` confirms the full cross-platform shape: on Unix `ctrlc` pulls
`nix` (signal/process features, itself pulling `libc`, `bitflags`, `cfg-if`, and the
build-dependency `cfg_aliases`); on macOS additionally `dispatch2` -> `block2` -> `objc2` ->
`objc2-encode`; on Windows `windows-sys` -> `windows-link` (both already MIT/Apache-2.0 and
already present in the lock file from other deps, so not newly added).

Every new license resolves to MIT, Apache-2.0, or a multi-license (`MIT/Apache-2.0`,
`Zlib OR Apache-2.0 OR MIT`) that is satisfied by picking MIT or Apache-2.0 - both already on
`deny.toml`'s allowlist (`allow = ["MIT", "Apache-2.0", "Unicode-3.0"]`). No new license string
needed adding. `cargo deny check` was run twice (once right after `cargo build`, once again
after the full gate) and printed `advisories ok, bans ok, licenses ok, sources ok` both times.

## What was tested and results

Per the brief's testing scope, this task did not add new unit tests: the cancel path through
the queue (flag flips -> in-flight jobs killed -> partials deleted -> queued jobs become
Cancelled -> summary prints -> exit folds to 130) is already covered by Task 3's queue-level
tests and Task 8/9's `run.rs` tests (`job_exit_code`, `run_json_document`,
`MilestoneState::render` for the `Cancelled` state, etc.), none of which needed touching. The
only thing genuinely new in this task is the SIGINT handler's own registration and the
force-exit-on-second-signal branch, which are OS signal-delivery mechanics, not pure logic:
there is nothing pure to unit-test in `ctrlc::set_handler(...)` itself (it is a thin wrapper
around signal registration) or in `std::process::exit(130)` (terminates the process, cannot be
asserted on from within the same process). A real SIGINT end-to-end test (send an actual
`SIGINT` to a running `muxsmith run` subprocess mid-batch and assert on exit code 130 plus
partial-file cleanup) is not cheaply automatable in this harness and was explicitly out of
scope per the brief; noted here as required.

Full gate, run before the commit and re-verified after:

- `cargo test --workspace`: all suites green (unit + integration + doctests across
  `muxsmith-core`, `muxsmith-cli`, `xtask`), no failures.
- `cargo fmt --all --check`: clean, no diff.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-cli/Cargo.toml` (+1 line: `ctrlc = "3"`)
- `crates/muxsmith-cli/src/commands/run.rs` (handler installation + doc-comment updates)
- `Cargo.lock` (7 new locked crates, via `cargo build`)

## Self-review findings

- Confirmed `run()` is only ever invoked once per process, from `main.rs`; the integration
  tests in `crates/muxsmith-cli/tests/run_cli.rs` spawn the compiled `muxsmith` binary as a
  subprocess via `assert_cmd`/`Command::cargo_bin`, they do not call `commands::run::run(...)`
  in-process. So there is no risk of `ctrlc::set_handler` being called twice within one test
  process (which would return `Err` on the second call; the code already ignores that via
  `let _ =`, matching the brief).
- Confirmed `run_queue`'s `cancel: &Arc<AtomicBool>` parameter (Task 8/3, in
  `crates/muxsmith-core/src/executor/queue.rs`) is the exact same `Arc` the handler clones and
  flips (`Arc::clone(&cancel)` at the `queue_cancel` binding, unchanged by this task) - no
  separate wiring was needed on the queue side.
- Verified the post-queue `if cancel.load(Ordering::SeqCst) { return 130; }` branch (unchanged
  by this task) sits after both the human-mode summary print and the `--json` document print,
  so "summary already printed" (the brief's inline comment on that branch) holds for both
  output modes without needing further comment changes there.
- `git status` after staging showed only the three in-scope files
  (`Cargo.toml`/`run.rs`/`Cargo.lock`); the pre-existing untracked `HANDOFF.md` in the repo root
  was left alone (out of scope, not part of this task).
- Double-checked the doc-comment edit on `run()` doesn't contradict Task 8/9's still-accurate
  claims about `--json` behavior, worst-of folding, or the specs-empty early return; only the
  cancellation-flag sentence needed correcting.

## Issues or concerns

None. `cargo deny check` stayed green with no `deny.toml` edits required - every transitive
license `ctrlc` pulls in across all three platforms was already covered by the existing
MIT/Apache-2.0 allowlist. No copyleft or unresolved-license dependency appeared.
