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

