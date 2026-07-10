### Task 6: Persisted job logs (`executor::joblog`, D26) + CLI wiring

**Files:**
- Create: `crates/muxsmith-core/src/executor/joblog.rs`
- Modify: `crates/muxsmith-core/src/executor/mod.rs`, `crates/muxsmith-core/Cargo.toml` (`dirs = "6"`, `time = { version = "0.3", features = ["formatting"] }`), `crates/muxsmith-cli/src/commands/run.rs` (tee + finish), `locales/en/cli.ftl` (`run-joblog-unavailable`, `run-joblog-written`)
- Test: `crates/muxsmith-core/tests/joblog.rs` (new, tempdir-based)

**Interfaces:**
- Produces:

```rust
pub struct RunLogger { /* dir, per-job accumulators (argv, output path, lines, started_at) */ }
impl RunLogger {
    pub fn create(runs_root: &Path, run_id: &str, specs: &[JobSpec]) -> std::io::Result<RunLogger>; // mkdir -p runs_root/<run_id>, collision suffix "-2", "-3", ...
    pub fn on_event(&mut self, ev: &JobEvent);      // accumulates Output lines; on Finished writes job-<index>.json
    pub fn finish(self, run_document: &serde_json::Value) -> std::io::Result<PathBuf>; // writes summary.json, returns dir
    pub fn dir(&self) -> &Path;
}
pub fn default_runs_root() -> Option<PathBuf>;      // dirs::data_dir()?/muxsmith/runs  (D26 location)
pub fn make_run_id(now: std::time::SystemTime) -> String; // UTC "YYYYMMDD-HHMMSSZ" via `time`
```

- `job-<index>.json` fields: `index`, `output`, `argv`, `state`, `exit_code`, `warnings`, `errors`, `duration_ms`, `lines` (raw Output lines; NO progress ticks per D24), `started_at`/`finished_at` (RFC3339 UTC). `summary.json` = the T2 `run_document`. Consumed by T8 (GUI runs + `list_runs`/`get_job_log` read these files).

- [ ] **Step 1: TDD** (tempdir as runs_root): create->on_event(Started/Output/Finished)->finish yields exactly `job-0.json` + `summary.json` with the fields above; collision suffix test (pre-create the dir); skipped-job case (Finished{Cancelled} without Started still writes a record with empty `lines`).
- [ ] **Step 2:** Implement; single-threaded writer (the caller's drain loop is single-threaded - no Mutex; say so in the rustdoc, with the invariant that BOTH surfaces must tee their drain loop through it so persistence stays unconditional, spec 6).
- [ ] **Step 3: CLI wiring** in run.rs: before the queue thread, `default_runs_root()` + `RunLogger::create` (failure -> render `run-joblog-unavailable` warning to stderr, continue without logs - a mux run never dies for a log dir); tee `logger.on_event(&event)` in the drain loop (also when `--json`); after join, `finish(&run_document)`; human mode prints `run-joblog-written` with the dir. Update `run_cli.rs`/`run_live.rs` expectations.
- [ ] **Step 4:** `cargo deny check` with the two new deps; full gate green. **Commit** `feat(core+cli): persisted per-job JSON logs under platform data dir (D26)`

---

