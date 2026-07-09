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

