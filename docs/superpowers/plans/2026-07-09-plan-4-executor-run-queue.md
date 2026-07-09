# Plan 4: executor + run + job queue Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** The process layer (D7 split): a testable executor (spawn/progress/exit-mapping/kill), a FIFO job queue with `--jobs N` and soft `--fail-fast`, the `muxsmith run` subcommand (milestone lines + `--json` final document + SIGINT cleanup), plus the D18 riders.

**Architecture:** Core gains an `executor` module (spawn behind a `Spawn` trait mirroring `Identify`, per-job runner, `JobEvent` mpsc queue - D13). The CLI gains `run` (mirrors `dry_run.rs`: re-plan, then execute) and `--on-collision` on both planning commands (D15). SIGINT via `ctrlc` in the CLI setting a shared cancel flag the queue honors (D16). All human text in Fluent; core stays prose-free.

**Tech Stack:** Rust (edition 2024, stable), std threads only (NO async runtime, D7), `ctrlc` (new CLI dep, D16), mkvmerge v100 external (`--gui-mode` line protocol), Fluent.

## Global Constraints

- Per-commit gate, run all four, do NOT skip fmt: `cargo test --workspace` AND `cargo fmt --all --check` AND `cargo clippy --workspace --all-targets -- -D warnings` AND `cargo deny check`.
- Core emits no user-facing prose: diagnostics/events are codes + params; human text lives in `locales/en/*.ftl`. A new `DiagCode` without a Fluent message fails `catalog_completeness`.
- `#![deny(missing_docs)]` on lib crates: every new public item needs a doc comment.
- Commits authorized. GPG blocks agent commits: `git -c commit.gpgsign=false commit ...`. Trailer final line: `Co-Authored-By: <session model> <noreply@anthropic.com>`. Pushes logged in `gh-log.md`.
- Typography: ASCII punctuation only; umlauts and `Ş` intact.
- Confirm mkvmerge behavior by RUNNING the binary (v100 installed), never from memory (SI-3). Gated tests self-skip when mkvmerge is absent (`Mkvmerge::locate().ok()` + eprintln + return idiom, `command_integration.rs:213-230`).
- Design memo: `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md` (D13-D18). Spec wins on conflict.
- Exit-code contract (D15): 0 clean / 1 worst is warning (diag or job) / 2 any error or failed job / 130 SIGINT-cancelled batch.

## Dependency graph and execution waves (SI-1: parallelize independent work)

```
Wave 1 (PARALLEL, separate worktrees):  T1 (executor seam)   T4 (--on-collision)   T5 (tests/support + tempdir)   T6 (CI mkvtoolnix)   T7 (richer gated test)
Wave 2:                                 T2 (job runner)      [depends T1]
Wave 3:                                 T3 (queue + events)  [depends T2]
Wave 4:                                 T8 (run subcommand)  [depends T3, T4]
Wave 5:                                 T9 (--json)          [depends T8]
Wave 6:                                 T10 (SIGINT)         [depends T8]
Wave 7:                                 T11 (gated e2e run)  [depends T8-T10]
```

Wave-1 conflict notes: T5 and T7 both touch `command_integration.rs` (different regions: T5 replaces the FakeIdent block at :78-98, T7 appends a test) - merge order T5 then T7, resolve trivially if needed. T4 touches `cli.rs`/`dry_run.rs`, untouched by other wave-1 tasks. After each worktree branch passes its task review, merge to master sequentially, re-running the full gate per merge.

---

### Task 1: Executor spawn seam (`Spawn` trait, live + fake, gui-mode grammar)

**Files:**
- Create: `crates/muxsmith-core/src/executor/mod.rs`, `crates/muxsmith-core/src/executor/spawn.rs`
- Modify: `crates/muxsmith-core/src/lib.rs` (add `pub mod executor;`)
- Test: unit tests in `spawn.rs` (fake); gated grammar check in `crates/muxsmith-core/tests/executor_live.rs` (new)

**Interfaces:**
- Produces: `executor::spawn::{Spawn, RunningJob, Killer, LiveSpawner, FakeSpawner, SpawnError}` exactly as below; later tasks consume these verbatim.

- [ ] **Step 1: Empirically capture the `--gui-mode` line grammar (SI-3)**

Run against the installed mkvmerge v100 (throwaway dir):

```bash
printf '1\n00:00:00,000 --> 00:00:01,000\nHello\n' > /tmp/seed.srt
mkvmerge --gui-mode -o /tmp/gm-probe.mkv /tmp/seed.srt ; echo "exit=$?"
```

Record the observed `#GUI#...` lines (progress, and whatever warning/error tag format appears; provoke a warning if cheap, e.g. `--default-track-flag 9:1` on a missing track id provokes an error line - probe what v100 actually prints). These observed strings become the doc comment in `spawn.rs` and the fixtures for Task 2's parser tests. Do not proceed on assumed grammar.

- [ ] **Step 2: Write the failing fake-spawner test**

In `spawn.rs` `#[cfg(test)]`:

```rust
#[test]
fn fake_spawner_scripts_lines_and_exit() {
    let fake = FakeSpawner::script(vec!["#GUI#progress 50%".into()], Some(0));
    let mut job = fake.spawn(&["--output".into(), "x.mkv".into()]).unwrap();
    assert_eq!(job.next_line().as_deref(), Some("#GUI#progress 50%"));
    assert_eq!(job.next_line(), None);
    assert_eq!(job.wait(), Some(0));
    assert_eq!(fake.spawned(), vec![vec!["--output".to_string(), "x.mkv".into()]]);
}

#[test]
fn fake_killer_ends_stream_and_wait_returns_none() {
    let fake = FakeSpawner::script(vec!["line".to_string(); 100], Some(0));
    let mut job = fake.spawn(&[]).unwrap();
    let kill = job.killer();
    job.next_line();
    kill();
    assert_eq!(job.next_line(), None);
    assert_eq!(job.wait(), None);
}
```

- [ ] **Step 3: Run to verify failure** - `cargo test -p muxsmith-core fake_spawner` - FAIL (module absent).

- [ ] **Step 4: Implement the seam**

`executor/mod.rs`:

```rust
//! Process execution (spec 6, D13): spawning mkvmerge behind a testable
//! seam, per-job state, and the FIFO queue. Prose-free like the rest of
//! core; all human text lives in the CLI's Fluent catalogs.

pub mod spawn;
```

`executor/spawn.rs` (doc comments on every pub item; the gui-mode grammar comment uses Step 1's OBSERVED lines):

```rust
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Why a spawn error is not a job failure: failing to even start mkvmerge
/// (binary vanished, non-UTF-8 path) is an environment problem, reported
/// distinctly from a mux that ran and failed.
#[derive(Debug, Clone, PartialEq)]
pub struct SpawnError(pub String);

/// A clonable handle that kills its job when invoked (idempotent,
/// best-effort). Lives separately from [`RunningJob`] so the queue's
/// cancellation path can kill in-flight jobs it does not own.
pub type Killer = Arc<dyn Fn() + Send + Sync>;

/// Abstracts process execution so the job runner and queue are
/// unit-testable with a scripted fake, mirroring `Identify` (D7/D13).
pub trait Spawn {
    /// Spawns one mux with `argv` (the pure `command(&Plan)` vector; the
    /// live impl prepends `--gui-mode`).
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError>;
}

/// One running (or scripted) mkvmerge process.
pub trait RunningJob: Send {
    /// Next stdout line; `None` at EOF (or after a kill). Blocking.
    fn next_line(&mut self) -> Option<String>;
    /// Waits for exit; `None` when the process died without a code (killed).
    fn wait(&mut self) -> Option<i32>;
    /// A [`Killer`] for this job.
    fn killer(&self) -> Killer;
}

/// The production [`Spawn`]: runs the resolved mkvmerge with `--gui-mode`
/// prepended for machine-readable progress. Observed v100 line grammar:
/// <PASTE STEP 1 OBSERVATIONS HERE, e.g. `#GUI#progress 100%`>.
pub struct LiveSpawner {
    /// Resolved mkvmerge binary path (from `Mkvmerge::path()`).
    pub mkvmerge: PathBuf,
}

impl Spawn for LiveSpawner {
    fn spawn(&self, argv: &[String]) -> Result<Box<dyn RunningJob>, SpawnError> {
        let mut child = Command::new(&self.mkvmerge)
            .arg("--gui-mode")
            .args(argv)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| SpawnError(e.to_string()))?;
        let stdout = child.stdout.take().expect("stdout was piped");
        Ok(Box::new(LiveJob {
            reader: BufReader::new(stdout),
            child: Arc::new(Mutex::new(child)),
        }))
    }
}

struct LiveJob {
    reader: BufReader<std::process::ChildStdout>,
    child: Arc<Mutex<Child>>,
}

impl RunningJob for LiveJob {
    fn next_line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(line.trim_end().to_string()),
        }
    }
    fn wait(&mut self) -> Option<i32> {
        self.child.lock().unwrap().wait().ok().and_then(|s| s.code())
    }
    fn killer(&self) -> Killer {
        let child = Arc::clone(&self.child);
        Arc::new(move || {
            let _ = child.lock().unwrap().kill();
        })
    }
}

/// Scripted fake for unit tests: yields the scripted lines, then EOF, then
/// the scripted exit code; records every argv it was asked to spawn.
pub struct FakeSpawner {
    lines: Vec<String>,
    exit: Option<i32>,
    spawned: Mutex<Vec<Vec<String>>>,
}
// impl: FakeSpawner::script(lines: Vec<String>, exit: Option<i32>) -> FakeSpawner;
// fn spawned(&self) -> Vec<Vec<String>> (clones the record).
// FakeJob: a line cursor + Arc<AtomicBool> killed flag; next_line returns None
// once killed is set (else the next scripted line); wait returns None if
// killed, else the scripted exit. killer() clones the flag into a closure.
// The Step 2 tests are the binding contract for this fake.
```

Complete `FakeSpawner`/`FakeJob` per the test's contract (the test IS the spec for the fake). Note `#![deny(missing_docs)]` applies to all pub items including `FakeSpawner` methods.

- [ ] **Step 5: Gated grammar test** - `tests/executor_live.rs`: locate-or-skip idiom, spawn a real 1-track mux via `LiveSpawner` (SRT fixture pattern from `command_integration.rs:232-245`), collect all lines + exit; assert exit `Some(0)`, at least one line starting `#GUI#progress `, and that the final progress line is `#GUI#progress 100%` (adjust ONLY if Step 1 observed otherwise; the assertion must encode observed reality).

- [ ] **Step 6: Suite + gate** - `cargo test --workspace`, fmt, clippy, deny - all green.

- [ ] **Step 7: Commit** - `feat(executor): Spawn seam with live mkvmerge and scripted fake (D13)`

---

### Task 2: Per-job runner (progress parse, exit mapping, delete-partial)

**Files:**
- Create: `crates/muxsmith-core/src/executor/job.rs`
- Modify: `crates/muxsmith-core/src/executor/mod.rs` (`pub mod job;`)
- Test: unit tests in `job.rs` against `FakeSpawner`

**Interfaces:**
- Consumes: Task 1's `Spawn`/`RunningJob`/`Killer`.
- Produces: `executor::job::{JobSpec, JobState, JobOutcome, JobProgress, run_job}`:

```rust
/// What to execute: the pure argv plus the output path the argv writes
/// (needed for parent-dir creation and delete-partial).
#[derive(Debug, Clone, PartialEq)]
pub struct JobSpec {
    /// `command(&Plan)` vector (no program name, no `--gui-mode`).
    pub argv: Vec<String>,
    /// The plan's rendered output path.
    pub output: PathBuf,
}

/// Terminal job state (spec 6; mirrors mkvtoolnix-gui DoneOk/DoneWarnings/
/// Failed/Aborted, D13).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState { Ok, Warning, Failed, Cancelled }

/// One finished job.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JobOutcome {
    pub state: JobState,
    /// mkvmerge's exit code; `None` when killed.
    pub exit_code: Option<i32>,
    /// Captured warning lines (tag stripped).
    pub warnings: Vec<String>,
    /// Captured error lines (tag stripped).
    pub errors: Vec<String>,
    pub duration_ms: u64,
}

/// Mid-job signal surfaced to the caller (the queue re-emits as JobEvent).
#[derive(Debug, Clone, PartialEq)]
pub enum JobProgress {
    Percent(u8),
    WarningLine(String),
    ErrorLine(String),
}

/// Runs one job to completion: ensures the output's parent dir exists
/// (D13), spawns, streams lines through the gui-mode parser, maps the exit
/// code (0 ok / 1 warning, output kept / 2 or abnormal failed, partial
/// deleted / killed while `cancel` set = cancelled, partial deleted).
pub fn run_job(
    spawner: &dyn Spawn,
    spec: &JobSpec,
    cancel: &AtomicBool,
    on_progress: &mut dyn FnMut(JobProgress),
) -> JobOutcome
```

- [ ] **Step 1: Failing tests** (fake-driven; grammar strings = Task 1's observed reality):

```rust
#[test]
fn exit_zero_is_ok_and_output_kept() { /* fake exit 0; state Ok; file written by test stays */ }
#[test]
fn exit_one_is_warning_with_captured_lines() { /* scripted warning line + exit 1; state Warning; warnings contains stripped text */ }
#[test]
fn exit_two_is_failed_and_partial_deleted() { /* test creates spec.output beforehand; fake exit 2; state Failed; assert !spec.output.exists() */ }
#[test]
fn killed_under_cancel_is_cancelled_and_partial_deleted() { /* cancel set + killed fake (wait None); state Cancelled; partial deleted */ }
#[test]
fn progress_lines_surface_as_percent() { /* scripted progress 25/50/100; on_progress collected == [25,50,100] */ }
#[test]
fn parent_dir_created_before_spawn() { /* spec.output in a not-yet-existing subdir; after run_job the parent exists */ }
```

- [ ] **Step 2: RED** - module absent. **Step 3:** implement `job.rs` (parser: `strip_prefix("#GUI#progress ")` + trailing `%` -> u8; warning/error tag prefixes per observed grammar; delete-partial = `std::fs::remove_file(&spec.output)` ignoring NotFound, ONLY on Failed/Cancelled). **Step 4:** GREEN + full gate. **Step 5: Commit** - `feat(executor): per-job runner with gui-mode parse and exit mapping (D13)`

---

### Task 3: FIFO queue, `JobEvent` stream, soft fail-fast, cancellation

**Files:**
- Create: `crates/muxsmith-core/src/executor/queue.rs`
- Modify: `executor/mod.rs` (`pub mod queue;`)
- Test: unit tests in `queue.rs` against `FakeSpawner`

**Interfaces:**
- Consumes: Task 2's `run_job`/`JobSpec`/`JobOutcome`/`JobState`.
- Produces: `executor::queue::{JobEvent, QueueOpts, run_queue}`:

```rust
/// Serializable job-engine event (D13): the CLI renders it, Plan 5's Tauri
/// shell forwards it, a future --json-events streams it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum JobEvent {
    Started { index: usize, output: PathBuf },
    Progress { index: usize, percent: u8 },
    Warning { index: usize, text: String },
    Error { index: usize, text: String },
    Finished { index: usize, outcome: JobOutcome },
}

/// Queue policy (spec 6, D14).
#[derive(Debug, Clone, Copy)]
pub struct QueueOpts {
    /// Worker count; clamped to >= 1. Default 1 (sequential).
    pub jobs: usize,
    /// Soft fail-fast (D14): on the first Failed, dequeue nothing further;
    /// in-flight jobs finish; queued jobs become Cancelled.
    pub fail_fast: bool,
}

/// Runs `specs` FIFO over a bounded std worker pool. Returns one outcome
/// per spec, index-aligned. `cancel` (set by the CLI's SIGINT handler)
/// stops dequeuing AND kills in-flight jobs via their Killers; their
/// partials are deleted by `run_job`, queued specs become Cancelled.
pub fn run_queue(
    specs: &[JobSpec],
    spawner: &(dyn Spawn + Sync),
    opts: QueueOpts,
    cancel: &Arc<AtomicBool>,
    events: &std::sync::mpsc::Sender<JobEvent>,
) -> Vec<JobOutcome>
```

Implementation shape (pin it): `std::thread::scope`; shared `AtomicUsize` next-index + `AtomicBool` stop-dequeuing; killer registry `Mutex<Vec<Option<Killer>>>` (one slot per worker); a watcher thread polls `cancel` every 50ms and, when set, flips stop-dequeuing and invokes all registered killers, then exits; workers: loop { if stop or cancel -> break; take index; register killer; run_job with an `on_progress` closure that forwards as JobEvents; emit Started/Finished; on Failed && fail_fast -> set stop }. After the scope: every spec with no outcome gets `JobOutcome { state: Cancelled, exit_code: None, warnings: vec![], errors: vec![], duration_ms: 0 }` (no Started event - it never started). Event send failures (`SendError`) are ignored (receiver gone = caller stopped listening).

- [ ] **Step 1: Failing tests:**

```rust
#[test] fn sequential_fifo_order_and_events() { /* jobs:1, 3 specs; Started/Finished index order 0,1,2 */ }
#[test] fn jobs_n_bounds_concurrency() { /* FakeSpawner variant with a live-jobs counter (Arc<AtomicUsize> max tracker); jobs:2, 4 specs; max observed <= 2 */ }
#[test] fn soft_fail_fast_cancels_queued_but_not_inflight() { /* jobs:1, 3 specs, first Failed; outcomes [Failed, Cancelled, Cancelled]; exactly one Started */ }
#[test] fn no_fail_fast_continues_past_failure() { /* [Failed, Ok, Ok] */ }
#[test] fn cancel_kills_inflight_and_cancels_queued() { /* long-scripted fake; set cancel after first Started; outcomes [Cancelled, Cancelled, ...]; killer invoked */ }
#[test] fn outcomes_index_aligned() { /* jobs:2 with different-length scripts; outcome[i] matches spec i */ }
```

(The concurrency-tracking fake extension: add to `FakeSpawner` a shared `Arc<AtomicUsize>` current/max counter incremented in `spawn`/decremented in `wait` - extend Task 1's fake here, doc-commented.)

- [ ] **Step 2: RED. Step 3: implement. Step 4: GREEN + full gate. Step 5: Commit** - `feat(executor): FIFO job queue with JobEvent stream, jobs N, soft fail-fast, cancellation (D13/D14)`

---

### Task 4: `--on-collision` flag on dry-run (and the CLI mapping type) [WAVE 1 - independent]

**Files:**
- Modify: `crates/muxsmith-cli/src/cli.rs` (add `CollisionArg` + flag on `DryRun`), `crates/muxsmith-cli/src/main.rs` (pass-through), `crates/muxsmith-cli/src/commands/dry_run.rs` (signature + `RunInputs.on_collision`)
- Test: `crates/muxsmith-cli/tests/dry_run_cli.rs` (flag reaches the planner)

**Interfaces:**
- Produces: `cli::CollisionArg` (clap ValueEnum: `error|skip|overwrite`) with `pub fn policy(self) -> CollisionPolicy`; `dry_run::run` gains `on_collision: Option<CollisionPolicy>` before `json`. Task 8 reuses `CollisionArg` for `run`.

- [ ] **Step 1: Failing test** - in `dry_run_cli.rs`: a profile whose output collides with a pre-existing file; default invocation exits 2 (`error`); with `--on-collision skip` exits 1 (warning) and the JSON carries the `output-collision` diagnostic at warning severity. Follow the file's existing invocation helper.
- [ ] **Step 2: RED** (unknown flag). **Step 3:** implement:

```rust
/// CLI value for the collision-policy override (spec 4.2 run input). Maps
/// to core's CollisionPolicy; a CLI-local type so core stays clap-free.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CollisionArg {
    /// Refuse the colliding output (default policy).
    Error,
    /// Skip the colliding output with a warning.
    Skip,
    /// Replace the pre-existing file.
    Overwrite,
}
impl CollisionArg {
    /// The core policy this argument selects.
    pub fn policy(self) -> muxsmith_core::profile::model::CollisionPolicy { /* 1:1 match */ }
}
```

DryRun variant gains `#[arg(long, value_enum)] on_collision: Option<CollisionArg>`; main.rs passes `on_collision.map(CollisionArg::policy)`; dry_run.rs threads it into `RunInputs { on_collision, .. }` (replacing the hardcoded `None` at dry_run.rs:78).
- [ ] **Step 4: GREEN + full gate. Step 5: Commit** - `feat(cli): --on-collision override for dry-run (D15)`

---

### Task 5: `tests/support` consolidation + tempdir-leak fix [WAVE 1 - independent]

**Files:**
- Create: `crates/muxsmith-core/tests/support/mod.rs`
- Modify: `crates/muxsmith-core/tests/planner_resolution.rs`, `suggestions.rs`, `command_integration.rs` (use the shared helpers; fix `std::mem::forget`)

**Interfaces:**
- Produces: `support::{FakeIdent, lang}` - `FakeIdent` exactly as today (`by_name: HashMap<String, Identification>` + `Identify` impl, from `command_integration.rs:78-90`), `lang()` returning the 3-row en/de/tr `LanguageIndex`.

- [ ] **Step 1:** Create `tests/support/mod.rs` (a `tests/` SUBDIRECTORY module - not compiled as its own test crate) with the two helpers, `#[allow(dead_code)]` on items where a consumer file uses only part. Each of the three test files: `mod support;` + `use support::{FakeIdent, lang};`, deleting the local copies.
- [ ] **Step 2:** Fix the 15 `std::mem::forget` sites (planner_resolution.rs:59,401,453,592,662,825,888,1153,1193,1235,1376,1416,1455; suggestions.rs:56,152): change the owning helpers (e.g. `plan_one`) to RETURN the `TempDir` alongside their value (callers bind `let (batch, _dir) = ...`), so directories are cleaned on drop instead of leaked. Mechanical; the compiler finds every caller.
- [ ] **Step 3:** Full gate green (this task is pure test refactor; zero behavior change - the suite itself is the spec).
- [ ] **Step 4: Commit** - `test: shared support module (FakeIdent, lang) and tempdir-leak fix (D18)`

---

### Task 6: mkvtoolnix in CI [WAVE 1 - independent]

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1:** Add to the `test` job, after checkout and before the cargo steps:

```yaml
      - name: Install mkvtoolnix (gated integration tests)
        if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y mkvtoolnix
```

Linux-only deliberately: branch pushes are Linux-only anyway (minute economy) and the gated tests self-skip elsewhere; macOS/Windows installs are a go-public follow-up. Comment this in the YAML.
- [ ] **Step 2:** Verification is the next push's CI run: the controller confirms the gated tests RAN (not skipped) in the Actions log - note in the report that this is verified post-merge.
- [ ] **Step 3: Commit** - `ci: install mkvtoolnix on Linux so gated tests run (D18)`

---

### Task 7: Richer gated live test (attachment + changes round trip) [WAVE 1 - independent]

**Files:**
- Test: `crates/muxsmith-core/tests/command_integration.rs` (append one gated test)

- [ ] **Step 1:** Following the file's existing gated pattern: build a primary MKV from an SRT plus `--attach-file` (a small .txt as attachment; confirm mkvmerge accepts a text attachment with an explicit `--attachment-mime-type text/plain` - probe the real binary first per SI-3); build a `Plan` (or drive `plan_batch` with a profile) that selects the subtitle track with `changes: { track_name: Renamed, default_track: true }` and keeps the attachment; run `command(&plan)` through real mkvmerge; `-J` the output and assert: track_name == "Renamed", default_track true, attachment present with the original file_name. This converts Plan 3's one-off manual v100 validation into a standing guard (D18).
- [ ] **Step 2:** Full gate green (test runs live locally; self-skips without mkvmerge).
- [ ] **Step 3: Commit** - `test(command): gated live guard for attachment + changes round trip (D18)`

---

### Task 8: `muxsmith run` subcommand (re-plan, queue, milestone lines, exit fold)

**Files:**
- Create: `crates/muxsmith-cli/src/commands/run.rs`
- Modify: `cli.rs` (Run variant), `main.rs` (dispatch), `commands/mod.rs`, `locales/en/cli.ftl` (new keys)
- Test: `crates/muxsmith-cli/tests/run_cli.rs` (new; fake-free CLI-level tests where possible, planning-failure paths), plus unit tests for the milestone renderer

**Interfaces:**
- Consumes: T3 `run_queue`/`JobEvent`/`QueueOpts`; T4 `CollisionArg`; `command(&Plan)`; `LiveSpawner` (T1); dry_run.rs's flow as the template (validate pass -> locate -> list_languages -> RunInputs -> plan_batch, dry_run.rs:35-85) and its `rendered_diags`/`all_diags` helpers (extract shared pieces into `commands/mod.rs` if cleaner - reviewer judges).
- Produces: `run::run(profile_path, source, output, on_collision, jobs, fail_fast, json, renderer) -> i32`.

- [ ] **Step 1:** cli.rs `Run` variant:

```rust
    /// Plan and execute the batch (spec 5.5 level 3).
    Run {
        profile: PathBuf,
        #[arg(long)] source: Option<PathBuf>,
        #[arg(long)] output: Option<PathBuf>,
        #[arg(long, value_enum)] on_collision: Option<CollisionArg>,
        /// Parallel mux jobs (default 1 = sequential).
        #[arg(long, default_value_t = 1)] jobs: usize,
        /// Stop dequeuing after the first failed job (in-flight finish).
        #[arg(long)] fail_fast: bool,
        #[arg(long)] json: bool,
        #[arg(long)] locale: Option<String>,
    },
```

- [ ] **Step 2:** `commands/run.rs` flow: identical to dry_run through `plan_batch` (re-plan immediately before execution, spec 5.5). Then: `let specs: Vec<JobSpec> = batch.files.iter().filter_map(|f| f.plan.as_ref()).map(|p| JobSpec { argv: command(p), output: p.output.clone() }).collect();` (error-severity files already have `plan: None`). Print planning diagnostics exactly like dry-run (human) FIRST. If `specs` is empty, fold diagnostics and exit like dry-run. Else run the queue with `LiveSpawner { mkvmerge: mkv.path().into() }`, `cancel` flag (plain `Arc<AtomicBool>` here; T10 wires SIGINT), draining events on the main thread.
- [ ] **Step 3:** Milestone renderer (human mode): per JobEvent - Started -> `run-job-start`; Progress -> print at 25/50/75 threshold crossings only (track last-milestone per index); Warning/Error lines -> `run-job-notice`; Finished -> `run-job-ok`/`run-job-warning`/`run-job-failed`/`run-job-cancelled` with duration; then `run-summary` with counts. New cli.ftl keys (exact texts, `{ $index }`/`{ $total }`/`{ $output }`/`{ $percent }`/`{ $seconds }`/`{ $ok }` etc. params):

```
run-job-start = [{ $index }/{ $total }] { $output } ... start
run-job-progress = [{ $index }/{ $total }] { $output } ... { $percent }%
run-job-notice = [{ $index }/{ $total }] { $output } ... { $text }
run-job-ok = [{ $index }/{ $total }] { $output } ... ok ({ $seconds }s)
run-job-warning = [{ $index }/{ $total }] { $output } ... warning ({ $count } warnings, { $seconds }s)
run-job-failed = [{ $index }/{ $total }] { $output } ... failed (exit { $code })
run-job-cancelled = [{ $index }/{ $total }] { $output } ... cancelled
run-summary = { $ok } ok, { $warning } warning, { $failed } failed, { $cancelled } cancelled
```

- [ ] **Step 4:** Exit fold: `max(diag_fold, job_fold)` where job_fold = 2 if any Failed, 1 if any Warning, else 0; if the cancel flag ended the batch -> 130 (overrides). Reuse/extend dry_run's `exit_code` shape.
- [ ] **Step 5:** Tests: unit-test the milestone thresholding (a pure fn over a JobEvent sequence -> rendered lines); CLI test for planning-error path (bad profile exits 2 without executing). The full execute path is T11's gated e2e.
- [ ] **Step 6:** Full gate (incl. `catalog_completeness` for the new keys - they are cli.ftl keys consumed via `renderer.msg`, same guard as always). **Step 7: Commit** - `feat(cli): muxsmith run - re-plan, execute queue, milestone lines, exit fold (D15)`

---

### Task 9: `run --json` final document

**Files:**
- Modify: `commands/run.rs`
- Test: `run_cli.rs` + a unit test on the document builder

**Interfaces:**
- Produces (D15): dry-run's document (`config_diagnostics`, `files`, `batch_diagnostics`, `suggestions` - reuse `batch_json`) EXTENDED with:

```json
"jobs": [ { "index": 0, "output": "...", "state": "ok", "exit_code": 0, "warnings": [], "errors": [], "duration_ms": 12400 } ],
"summary": { "ok": 2, "warning": 1, "failed": 0, "cancelled": 0 }
```

- [ ] **Step 1:** Failing unit test on the builder (given outcomes -> expected JSON, serde_json::Value equality). **Step 2:** implement: in json mode suppress ALL human lines (drain events silently; JobOutcome is already `Serialize`). NDJSON stays deferred (v1.x, D15). **Step 3:** GREEN + gate. **Step 4: Commit** - `feat(cli): run --json final document with per-job results (D15)`

---

### Task 10: SIGINT - ctrlc, kill in-flight, exit 130

**Files:**
- Modify: `crates/muxsmith-cli/Cargo.toml` (add `ctrlc = "3"`), `commands/run.rs` (install handler, wire the cancel flag, exit 130)
- Test: unit-level only (the cancel path through the queue is already covered by T3's tests; a real SIGINT e2e is not cheaply automatable - note this in the report)

- [ ] **Step 1:** In `run.rs` before `run_queue`: `let cancel = Arc::new(AtomicBool::new(false));` + 

```rust
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

After the queue returns: `if cancel.load(Ordering::SeqCst) { /* summary already printed */ return 130; }`.
- [ ] **Step 2:** `cargo deny check` green with the new dep (`ctrlc` is MIT/Apache-2.0; if a transitive dep trips a license not yet allowed, add THAT license individually with a comment - never blanket).
- [ ] **Step 3:** Full gate. **Step 4: Commit** - `feat(cli): SIGINT cancels the batch via ctrlc, exit 130 (D16)`

---

### Task 11: Gated end-to-end `run` test

**Files:**
- Test: `crates/muxsmith-cli/tests/run_live.rs` (new)

- [ ] **Step 1:** Gated (locate-or-skip): build 2 tiny source MKVs (SRT fixture pattern) in a temp source dir, a minimal profile, invoke the actual `muxsmith` binary (`env!("CARGO_BIN_EXE_muxsmith")`) with `run --source ... --output ...`; assert exit 0, both outputs exist and `-J`-identify as MKVs, and stdout contains the summary line. Add a second case: rerun with `--on-collision skip` exits 1 and leaves outputs untouched (mtime unchanged) - the rerun workflow guard (D14/D17).
- [ ] **Step 2:** Full gate. **Step 3: Commit** - `test(cli): gated end-to-end run over real mkvmerge (D15)`

---

## Self-review (controller, after all tasks)

- **Memo coverage:** D13 -> T1-T3 (+ parent-dir in T2); D14 -> T3 (+ rerun guard T11); D15 -> T4, T8, T9, T11; D16 -> T10; D17 -> T2 delete-partial (divergence already recorded); D18 -> T5, T6, T7. Deferred by decision: NDJSON `--json-events` (v1.x); persisted job logs (Plan 5); `--fail-fast=now` (v1.x); zero-track empty-plan warning (cleanup pass).
- **Waves:** wave 1 = T1+T4+T5+T6+T7 in parallel worktrees per SI-1; merges sequential with gate re-runs.
- **Close-out:** whole-branch review on the most capable model, SI-2 journal entry, salvage `.superpowers/sdd/` Plan-4 artifacts, HANDOFF refresh, push (verify T6's CI effect in the Actions log).
