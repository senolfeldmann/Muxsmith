### Task 4: Worker-panic handling + mutex-poison hygiene (#10)

**Files:**
- Modify: `crates/muxsmith-core/src/executor/queue.rs:270,276` (join + into_inner sites; read the whole file first - the audit verified only these anchors)
- Modify: `src-tauri/src/` AppState.active lock sites (locate via `grep -rn "active" src-tauri/src/ | grep -i "lock\|mutex"`)
- Test: `crates/muxsmith-core/src/executor/queue.rs` tests

**Interfaces:** new job outcome variant or reuse of Failed with a distinct diagnostic code `worker-panicked` (params: job index). Catalog keys added EN-only (T21 translates).

- [ ] Step 1: Failing test: a scripted job whose worker closure panics; assert the job's outcome is Failed-with-worker-panic (not Cancelled), the remaining jobs complete, and the queue's final collection does NOT panic.
- [ ] Step 2: Implement: capture `handle.join()`'s `Err(payload)`; record Failed + `worker-panicked` (downcast payload to &str/String for a log line, never into user-facing params beyond the code); replace `into_inner().unwrap()` / `lock().unwrap()` on killers/outcomes with poison-recovery (`unwrap_or_else(|p| p.into_inner())`), commenting WHY recovery is sound (state written before any panic point is still consistent; the panicked worker's slot is handled explicitly).
- [ ] Step 3: Same poison-recovery treatment for AppState.active in the Tauri shell (the promoted v1.x half).
- [ ] Step 4: Full gate; commit `fix(executor): report worker panics as failures, recover poisoned mutexes (#10)`.

