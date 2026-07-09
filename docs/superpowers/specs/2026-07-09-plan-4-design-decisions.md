# Plan 4 design decisions

Status: FINAL 2026-07-09 (Şenol confirmed the design). Plan 4 is the process
layer per the D7 split: the `executor` (spawn, progress parse, exit-code
mapping, kill + delete-partial, job states), the `run` subcommand + FIFO job
queue, and SIGINT cleanup. These decisions implement spec sections 5.5 (run),
6 (execution half), and 8.1 (run CLI). D14 and D15 fix CLI-contract details
the spec leaves open (fail-fast semantics, progress rendering, JSON shape,
the tool's own exit codes); the spec is not amended, this memo is the record.
On any spec/memo conflict the spec wins per repo convention.

Grounding: v1 design spec (authoritative); D7 forward decisions (executor
spawn behind a trait mirroring `Identify`; `--jobs N` on a bounded std thread
pool, no async runtime; SIGINT cleanup ships in Plan 4) - taken as given, not
re-recorded here; Plan 3 code as committed at a1283c4; mkvtoolnix-gui source
read for reference behavior (`src/mkvtoolnix-gui/jobs/model.cpp`,
`mux_job.cpp`, `job.cpp`, `util/settings.cpp` at `~/Downloads/mkvtoolnix/`).
The exact `--gui-mode` line grammar (`#GUI#progress NN%`, warning/error tags)
is confirmed against real mkvmerge v100 during implementation, not assumed
from memory.

## D13: Core-owned job queue; events flow as an mpsc stream of a serializable `JobEvent` enum

**Decision.** The FIFO job queue lives in muxsmith-core's executor layer, not
in the CLI. Worker threads emit a serializable `JobEvent` enum (job started /
progress / captured warning-error lines / job finished; exact variants fixed
at plan time) over `std::sync::mpsc`; the caller drains the receiver on its
own thread and owns all rendering. Job states:
`Pending -> Running -> {Ok, Warning, Failed, Cancelled}`.

- Rationale: the spec's architecture table puts "process spawn, progress
  parse, cancellation, job states" in core and makes the Tauri shell
  "commands + job event stream, no logic" - so the queue must be reusable
  as-is by the Plan 5 GUI. An event *enum as data* serves all three planned
  consumers: the CLI renderer now, the Tauri event payload in Plan 5, and the
  deferred NDJSON stream (D15) in v1.x. The drain-on-caller-thread model
  keeps stdout ownership single-threaded.
- Alternatives rejected: a callback/sink trait (workers call into the sink,
  which must be `Send + Sync`, and the renderer must lock stdout internally);
  polled shared state (loses event granularity, imposes a poll cadence).
- The state set mirrors mkvtoolnix-gui's `DoneOk / DoneWarnings / Failed /
  Aborted` one-to-one (`mux_job.cpp:154-159`), independently confirming the
  spec-6 exit-code mapping (0 ok / 1 warning, output kept / 2 error, partial
  deleted; abnormal process exit is Failed).
- The executor ensures the output's parent directory exists before spawning;
  mkvmerge's own directory-creation behavior is not relied on.

## D14: `--fail-fast` is soft: stop dequeuing, in-flight jobs finish

**Decision.** On the first `Failed` job, the queue dequeues nothing further;
already-running jobs finish on their own merits (ok/warning/failed); jobs
still queued become `Cancelled`. Only observable with `--jobs N > 1` -
sequentially there is nothing else in flight.

- Reference: mkvtoolnix-gui has **no** fail-fast at all.
  `Model::onStatusChanged` treats `Failed` like every terminal state and
  unconditionally calls `startNextAutoJob()` (`model.cpp:337-380`); no
  setting halts the queue on error. Our default (failures do not abort the
  batch) is exactly the reference behavior; `--fail-fast` in any form is an
  extension beyond it, and the soft reading is the smaller departure.
- Rationale: per-file failures (corrupt source, unreadable track) dominate
  and say nothing about sibling jobs, while killing a healthy mux at 90%
  discards real work; systemic failures (disk full, unwritable output dir)
  kill the siblings by themselves within seconds, so hard-kill's advantage
  window is small. Kill + delete-partial stays reserved for the two
  unambiguous intents: mkvmerge reported error, or the user cancelled.
  Completed outputs compose with the rerun workflow (`on_collision: skip`
  reruns exactly the remainder). Precedent: `make`, `cargo`, and GNU
  parallel `--halt soon,fail=1` all stop scheduling and let running jobs
  finish; the killing variant is a separate opt-in (`now,fail=1`).
- Considered and deferred: a `--fail-fast=now` value (kill in-flight too,
  the GNU-parallel `now,fail=1` analog) as an additive v1.x flag if real
  usage asks for it.

## D15: `run` CLI surface: milestone lines, `--json` final document, worst-of exit fold, 130 on SIGINT

**Decision.**

- Human mode renders dependency-free milestone lines - per job
  `[i/n] <output name> ... start / 25% / 50% / 75% / ok|warning|failed`
  (terminal state with duration; failed with exit cause), then a one-line
  batch summary (`N ok, N warning, N failed, N cancelled`). Pipe-safe, no
  TTY branching, interleaves correctly under `--jobs N`.
  Rejected: live ANSI bars via indicatif (new dependency, TTY/non-TTY
  special-casing, duplicates what the Plan 5 GUI job view does properly);
  quiet start/end-only lines (no liveness signal on long muxes).
- `--json` emits one final document: planning diagnostics (dry-run shape) +
  per-job results (output path, state, mkvmerge exit code, captured
  warnings/errors, duration) + summary counts. Human progress lines are
  suppressed in JSON mode. A streaming NDJSON event mode (`--json-events`,
  one `JobEvent` per line) is recorded as a v1.x candidate riding the D13
  enum - explicitly not built in Plan 4.
- Exit code of `muxsmith run` is the worst-of fold, mirroring dry-run: any
  error-severity diagnostic or any `Failed` job -> 2; else any
  warning-severity diagnostic or `Warning` job -> 1; else 0. A
  SIGINT-cancelled batch exits 130 (128 + SIGINT, shell convention), so
  scripts can distinguish user cancellation from mux failure.
- `--on-collision <error|skip|overwrite>` is added to both `run` and
  `dry-run` (parity: both plan). Spec 4.2 names the collision-policy
  override as a run input with CLI flags overriding profile defaults;
  `RunInputs.on_collision` has existed since Plan 2 but no flag exposes it,
  and `run` is where the rerun workflow (D14, D17) needs it.

## D16: SIGINT handling via the `ctrlc` crate, single-level

**Decision.** The `ctrlc` crate (cross-platform, including Windows console
events) installs a handler that sets the queue's shared cancellation flag.
The queue then stops dequeuing, kills in-flight children through the
executor's kill primitive, deletes their partial outputs, marks queued jobs
`Cancelled`, lets the renderer print the summary, and the process exits 130.
Single-level handling: a second Ctrl-C during cleanup forces immediate exit;
no two-stage graceful/hard scheme.

- Rationale: std has no stable signal API, and hand-rolled libc/WinAPI
  signal code across three OSes is exactly the fragile platform branching
  the project avoids. One small, widely-used dependency, vetted through
  `cargo deny` like everything else.

## D17: Partial outputs are deleted on failure and cancel - a deliberate divergence from mkvtoolnix-gui

**Reference.** The GUI removes the output file on `Failed`/`Aborted` only
behind the opt-in `m_removeOutputFileOnJobFailure` setting, default **off**
(`settings.cpp:621`, `job.cpp:497-510`): by default it keeps partial files.

**Decision.** Muxsmith keeps spec 6 unchanged: partial output deleted on
mkvmerge error (exit 2) and on cancellation.

- Rationale: the GUI is interactive - the user sees the red job immediately
  and a partial file aids diagnosis. Muxsmith's bulk contract is "a file in
  the output tree is a valid output"; a partial MKV plays up to the
  truncation point and looks like a success to scripts and library scanners.
  Sharpest argument: with `on_collision: skip`, a kept partial makes the
  rerun *skip* the broken file as already done - a silent-corruption footgun
  the GUI cannot have.

## D18: Plan 3 follow-up triage

**Into Plan 4** (each touches surface Plan 4 works on anyway):

- Richer gated live test (attachment + `changes`): converts the one-off
  manual mkvmerge-v100 validation from the Plan 3 review into a standing
  guard, in the same gated tier the executor tests extend.
- Install mkvtoolnix in CI: the gated integration tier (including the new
  executor/run tests) actually runs there instead of self-skipping.
- Dedup the `FakeIdent`/`lang()` test helpers (3 copies) into
  `tests/support.rs`; new executor/run tests want the same helpers.
- Replace `std::mem::forget(tempdir)` in tests (leaks temp dirs) with
  properly held guards, in the test files Plan 4 touches.

**Deferred to a later cleanup pass:** zero-track plan renders an empty MKV
with no diagnostic (planner "empty plan" warning); regex recompiled per
`matches` call; optional batch-level settable-language check; the standing
Plan 1/2 FINAL-review minors.
