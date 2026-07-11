<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-8  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01LM2pUR9dJSTjjm3XL9dUdE
  agent_id:           a684692789984465e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a684692789984465e.jsonl
  dispatch_desc:      Review Task 8 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T14:16:38.140Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Verdict: Needs fixes (documentation/robustness gaps, no correctness blockers found in the core IPC contract)

### Spec Compliance
- ✅ Thin event-forwarding shell: `start_run` re-plans through the same core calls the CLI uses, builds `JobSpec`s via the identical error-severity-skip `filter_map` (`run.rs:390-398`, near-verbatim comment match to `commands/run.rs:156-166`), runs `run_queue` on a std thread, drains the mpsc channel single-threaded (`run_batch`, `run.rs:372-399`), tees `RunLogger::on_event` before the caller's own sink.
- ✅ `JobEvent` forwarded verbatim: `app_bg.emit("muxsmith://job-event", event)` passes the core `&JobEvent` reference straight through (`run.rs:230-231` region); no re-shaping. `JobEvent` is `queue.rs`'s own `#[serde(tag = "event", rename_all = "snake_case")]` enum, so the D24 golden shape is preserved.
- ✅ Terminal `muxsmith://run-finished` carries the core `run_document` plus a codes-only `joblog_status` (`Complete`/`Incomplete`/`Unavailable`, `#[serde(rename_all = "snake_case")]`) spliced in after persistence, never before (`emit_run_finished`, `run.rs:351-357`; `finalize_joblog` called with the pre-splice document, `run.rs:228-234` region, so `summary.json` on disk never contains `joblog_status` itself, which is correct: that field describes the outcome of persisting the very document it would otherwise be embedded in).
- ✅ **Concern #1 resolved**: side-by-side comparison against `crates/muxsmith-cli/src/commands/run.rs`'s four early-return branches (load failure, mkvmerge-not-found, mkvmerge-query-failed, empty specs) shows the shell's `run_document(config_only_document(...))`/`run_document(batch_document(...))` calls use **exactly** the same diagnostics, `mkvmerge_found` value (`None`/`Some(false)`/`Some(true)`), and base-document constructor per branch as the CLI's `--json` output for the same condition. The document-as-contract design decision is sound and correctly ported.
- ✅ ONE-run-at-a-time: `lock_for_start` (`run.rs:320-321`) holds the guard for the check, rejects with `"run-already-active"` if occupied.
- ✅ `WindowEvent::CloseRequested` → `cancel_all()` (`on_close_requested`, `run.rs:302-309`), wired via `.on_window_event(...)` in `lib.rs`.
- ✅ `cancel_run`/`cancel_job` map to `QueueControl::cancel_all`/`cancel_job`, `"no-active-run"` when idle.
- ✅ `list_runs` reads `runs_root`, skips unreadable dirs (`run_meta_from_dir`'s `?`-chained `Option`), returns `RunMeta{run_id, started_at, summary}`.
- ✅ `get_job_log(run_id, index)` reads `job-<index>.json`.
- ✅ `run_id` traversal guard (`valid_run_id`) checked *before* any path join, rejects empty/`.`/`..`/`/`/`\`.
- ✅ Empty-batch project decision: no branch that returns `total_jobs: 0` ever calls `RunLogger::create` — confirmed no run directory is created on any soft-outcome path.
- ✅ No prose: every `IpcError::code(...)` call site uses a stable kebab-case identifier; no third-party error text is embedded anywhere (conservative, but compliant).
- ✅ Public items documented; ASCII-only punctuation confirmed by direct grep for `— " " ' ' …` across the diff (zero hits).
- ⚠️ **Gate claim unverifiable as stated** — see Issues; the report's "37 new tests" does not match the diff.
- ⚠️ Whether T11's frontend can actually distinguish "run finished with document" from "run never started" is not verifiable from this diff (no frontend code here); the underlying event-ordering hazard is real and documented below.

### Strengths
- Document-shape parity with the CLI is exact, not approximate — verified line-by-line against all four of `run.rs`'s early-return branches, not just asserted.
- `RunLogger`'s documented single-threaded-writer invariant is honored precisely: `on_event` is only ever called from the drain loop's own thread, never from the queue's scoped worker threads (`run_batch`, `run.rs:372-399`).
- Scoped-thread pattern (`std::thread::scope` around `run_queue`, borrowing `specs`/`ctl` by reference) is a correct, direct mirror of the CLI's identical pattern in `commands/run.rs:227-250` — sound, not reinvented.
- Every event-send/emit call downstream of `run_queue` is best-effort (`let _ = ...`), consistent with `run_queue`'s own documented "receiver gone = caller stopped listening" philosophy (`queue.rs`'s `run_queue` doc).
- `valid_run_id` is checked before any path join, not after — the traversal guard cannot be bypassed by a short-circuit ordering bug.
- Test suite exercises the right seams: FIFO ordering, single-run rejection, active-flag clearing, joblog population, both `finalize_joblog` branches (including a portable, root-safe EISDIR trick mirroring `executor::job`'s own pattern), traversal rejection, and the newest-first sort's collision-suffix edge case.

### Issues

#### Critical (Must Fix)
None found.

#### Important (Should Fix)

1. **Report's test count does not match the diff.** The report claims "37 total across `error.rs` + `run.rs`" and, separately, "`error::tests::*` (3) and `run::tests::* (27)`" (3+27=30, itself inconsistent with "37"). Direct count from the diff: `error.rs` has 3 `#[test]` functions, `run.rs` has **24**, total **27** — matching neither claimed figure exactly (it happens to match the TDD section's incidental "27/27 green" milestone, but not the two count-claims that bracket it). Per "Do Not Trust the Report," this is a verifiable factual discrepancy in the one thing the controller was told not to re-verify ("do not re-run" the gate). It does not indicate the *tests* are wrong — the 27 present are well-targeted — but the report's own bookkeeping cannot be trusted at face value, and the controller should reconcile before treating the gate as confirmed green on the numbers given.

2. **Undocumented event-ordering hazard for soft-outcome `start_run` paths.** For the three "planning never reached a queue" branches (`run.rs:134-163`, `finish_without_queue` at `run.rs:296-301`), `muxsmith://run-finished` is emitted **synchronously, inside the same command invocation**, before `start_run`'s `Result` is returned to Tauri's IPC layer — i.e., the event reaches the webview bridge *before* the frontend's `invoke('start_run').then(...)` callback fires. Any frontend that registers its `run-finished` listener only after awaiting `start_run` (a common but not universal pattern) will silently miss this event for every profile-load failure, missing-mkvmerge, and zero-planned-jobs case — exactly the cases the controller singled out for scrutiny. For the real-run path this is a non-issue (the event necessarily arrives after the command returns, since the queue takes real time). Nothing in `run.rs` documents this asymmetry for T11's benefit. Recommend a doc-comment note on `start_run` stating the listener must be registered before any `start_run` call, not after.

3. **`lock_for_start` holds the mutex across all of `start_run`'s synchronous planning work, which can block `on_close_requested` on Tauri's main event-loop thread.** `lock_for_start` (`run.rs:320-321`) acquires `state.active` and the guard is held through profile load, validate/lint, `Mkvmerge::locate`, `list_languages`, and `plan_batch` (which does real per-file `mkvmerge --identify` subprocess calls) until the first `drop(guard)` (`run.rs:137/149/161/198`) or the real-run assignment (`run.rs:217`). `on_close_requested` (`run.rs:307`) also calls `state.active.lock().unwrap()`, and Tauri's `on_window_event` callback runs synchronously on the event loop. If the user attempts to close the window while a `start_run` call is still inside its planning pass (large batch, many files to identify), the whole app's event loop stalls for the duration of that pass — not a deadlock (it self-resolves), but a real, user-visible freeze exactly during the one action (closing the app) D23's cooperative-cancel design was meant to keep responsive. The lock-across-planning choice is correct for the single-run invariant itself; the interaction with `on_close_requested` is the gap.

4. **Plan-mandated: cooperative cancel does not block app exit, risking an orphaned mkvmerge process or a truncated joblog write.** `on_close_requested` never calls `CloseRequestApi::prevent_close()` (it doesn't even destructure `api` from the `WindowEvent::CloseRequested` pattern) — confirmed by inspection of `lib.rs`/`main.rs`/`tauri.conf.json` outside the diff: this is a single-window app with no `RunEvent::ExitRequested` override, so Tauri's default behavior applies (process exits shortly after the last window closes). `cancel_all()` only sets an `AtomicBool` that the queue's watcher thread polls every 50ms (`CANCEL_POLL`, `queue.rs`) before invoking each in-flight job's `Killer`; the detached `std::thread::spawn` running `run_batch` (`run.rs:228`) is never joined anywhere. If process exit races ahead of that 50ms poll-and-kill (very plausible: Rust drops all threads unconditionally on process exit, no grace period), a live `mkvmerge` child can be orphaned and keep writing its output file after the user believes they closed the app and cancelled the run, and/or `RunLogger::finish`'s `summary.json` write never happens, leaving a run directory with only partial `job-<index>.json` files and no `summary.json` at all (a case `list_runs`'s `run_meta_from_dir` correctly skips, but which represents lost job history nonetheless). This follows directly from the brief's literal "cancel_all... cooperative teardown... never blocks the close," so it is **plan-mandated**, not an implementer deviation — flagging for the controller to decide whether v1 accepts this risk or whether a bounded wait (e.g., `prevent_close()` + a short timeout before allowing the actual close) belongs in a follow-up task.

#### Minor (Nice to Have)
- Universal `Mutex::lock().unwrap()` usage on `AppState.active` (`run.rs:321,393,434,445,307`) has no poison-recovery path: if any call inside a held-lock critical section ever panicked (all such calls are currently protected by "never fails" invariants elsewhere in core, e.g. `RUN_ID_FORMAT.format(...).expect(...)`), the mutex would poison and every subsequent run-lifecycle command would panic for the rest of the app's life. Low probability given the guarded invariants, but a single point of total, silent failure for the whole run-lifecycle surface.
- `valid_run_id` (`run.rs` near `started_at_from_run_id`) rejects `/`, `\`, `.`, `..`, empty — covers the brief's literal "no separators/.." requirement, but a bare Windows drive-prefix component like `"C:"` would pass all four checks and could redirect `PathBuf::join` onto a different drive root on Windows (`Path::push`'s prefix-replacement semantics). Narrow: only reachable via a compromised webview already inside Tauri's trust boundary, and Windows-only.
- `RunMeta.summary` (`list_runs`) is read straight from `summary.json`, which never carries `joblog_status` (by design, see Strengths). This means a run whose `summary.json` wrote fine but whose per-job logs were incomplete (`JoblogStatus::Incomplete`) is indistinguishable from a fully clean run once read back through `list_runs` — that nuance exists only in the live `run-finished` event, which item 2 above shows a frontend could miss entirely. Not a T8 defect per se, but worth flagging for T11's history view design.

### Assessment
**Task quality:** Needs fixes
**Reasoning:** The IPC contract itself (document parity with the CLI, event forwarding, single-run gate, traversal guard, joblog lifecycle) is correctly and carefully implemented with strong test coverage; nothing here blocks merge on correctness grounds. What needs fixing before this can be trusted as reported: reconcile the report's self-contradictory test-count claims against the diff's actual 27, and document the two real window-close/event-ordering hazards (items 2-4) so T11 doesn't inherit them silently.