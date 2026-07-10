### Task 8: Shell IPC - run lifecycle (D23): start_run, events, cancel, history

**Files:**
- Create: `src-tauri/src/run.rs`
- Modify: `src-tauri/src/lib.rs` (state + handler registration)
- Test: `src-tauri/src/run.rs` unit tests with `FakeSpawner` (factor the thread body so it takes a `&dyn Spawn` and an event sink closure; assert single-run rejection, event forwarding order, joblog files written)

**Interfaces:**
- Produces:

```rust
#[tauri::command] fn start_run(app: AppHandle, state, profile: String, source: Option<String>, output: Option<String>, jobs: Option<usize>) -> Result<StartedRun, IpcError>
    // StartedRun { run_id: String, total_jobs: usize, run_dir: Option<String> }
    // rejects with code "run-already-active" if a run is active (D23 single-run)
#[tauri::command] fn cancel_run(state) -> Result<(), IpcError>      // QueueControl::cancel_all
#[tauri::command] fn cancel_job(state, index: usize) -> Result<(), IpcError>
#[tauri::command] fn list_runs(state) -> Result<Vec<RunMeta>, IpcError>  // read runs_root: RunMeta { run_id, started_at, summary: Value } from summary.json (skip unreadable dirs)
#[tauri::command] fn get_job_log(state, run_id: String, index: usize) -> Result<Value, IpcError>  // job-<index>.json
```

- Events to the window: each `JobEvent` re-emitted as `muxsmith://job-event` (payload = the D24-golden serde shape verbatim); terminal `muxsmith://run-finished` with the full `run_document`. Thread: re-plan, build specs, `QueueControl::new`, `RunLogger`, `run_queue` on a std thread, drain loop tees `logger.on_event` + `app.emit`. On `WindowEvent::CloseRequested`: `cancel_all()` (cooperative teardown, D23).
- Consumed by: T11 (jobs view + history).

- [ ] **Step 1: TDD** the factored runner with FakeSpawner: events arrive in order Started->Output->Finished; second start rejected while active; active flag clears after finish; joblog dir populated.
- [ ] **Step 2:** Implement; managed state `AppState { active: Mutex<Option<ActiveRun>>, ... }`. Any error-severity diagnostic file skips exactly as the CLI (re-plan path shared through core - no shell-side planning logic).
- [ ] **Step 3:** Full gate + pnpm build green. **Commit** `feat(gui): run lifecycle IPC - start/cancel/events/history`

---

