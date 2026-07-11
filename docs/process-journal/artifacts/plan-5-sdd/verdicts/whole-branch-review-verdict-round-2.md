<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 2 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_016qmNddDHZXfpWfWmMJruCM
  agent_id:           a4cc466b850c3126e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a4cc466b850c3126e.jsonl
  dispatch_desc:      Whole-branch review Plan 5
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-10T18:09:02.869Z
  continuation_trigger: The coordinator sent a message while you were working: Re-verification request for your wh
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

Fix-wave diff read in full (all 1653 lines, all 12 changed files). Verification results:

---

# Re-Verification: Fix Wave 945ee96..7a2bc15

## Per-finding closure

**(1) Critical #1 — start_run mkvmerge resolution: CLOSED.**
`plan_run` (src-tauri/src/run.rs) now loads settings inside the blocking pass (`crate::load_settings_from(settings_path)` → `mkvmerge_override`) and resolves via `Mkvmerge::detect(override)`, replacing `Mkvmerge::locate()`. The RED/GREEN test `plan_run_honors_the_settings_mkvmerge_override_not_just_path` is genuinely discriminating: the fake mkvmerge sits **off PATH**, wired in purely through a settings file, and its invocation counter proves it was actually spawned even with a real working mkvmerge on the test machine's PATH — under the pre-fix code this test lands in `Ready` (real PATH binary, counter 0) and fails. The settings-failure path is also covered (`plan_run_propagates_a_settings_load_failure_as_an_ipc_error`), and the error return correctly releases the reservation (explicit `drop` on the `Err` arm; the `spawn_blocking` JoinError `?` path is covered by the `Reservation`'s own RAII drop). The cancel-during-planning invariant survives the refactor: `cancel_flag` is the reservation's own `Arc`, cloned before the pass, and `QueueControl::new` consumes it inside `plan_run`.

**(2) Important #2 — sync command / main-thread planning: CLOSED.**
`start_run` is now `async` with `plan_run` on `tauri::async_runtime::spawn_blocking`; only the O(1) acquire/commit and the thread spawn stay on the calling thread. The updated event-ordering doc is accurate under the async shape: the soft-outcome `run-finished` emit still happens before the command's `Result` reaches the frontend (the internal `.await` is unobservable from JS — a promise resolution is always "afterwards"), so the subscribe-first contract is unchanged, and JobsView's `ensureListeners()` discipline still matches it. Bonus correctness: the doc now correctly notes that `cancel_run`/`on_close_requested` are actually reachable during planning, which makes the previously-unreachable Reservation cancel machinery live in production.

**(3) Important #3 — D23 Run-disable + double-Run wipe: CLOSED, deviation APPROVED (see below).**
Both halves landed: (a) `JobsView` emits `update:runActive`, App brokers it, `BatchView` gates Run first on `runActive` with the new `batch-run-tooltip-run-active` key (checked before every other disable reason — correct precedence); (b) the destructive reset is fixed via `startingFresh = !runActive` gating.

**(4) Important #4 — runner-panic wedge: CLOSED.**
`TeardownGuard` is armed at the top of the runner thread and is now the thread's *only* `finish_teardown` call site, so normal completion and unwind converge on exactly one teardown. The `catch_unwind` test asserts the two things that matter: slot cleared on unwind, pending quit still exits (`vec![0]`). The panic source (`run_batch`'s `handle.join().expect`) is unchanged and correctly propagates into the guard's Drop; the scoped threads are already joined by then, so no orphaned workers at teardown time.

**(5) Important #5 — atomic settings writes: CLOSED.**
`settings::save` stages to a same-directory pid-suffixed temp file and publishes via `fs::rename` as the sole touch of the final name; failed rename cleans up the temp. Both new tests are well-chosen (no temp leftover on success; failed publish leaves the pre-existing target untouched *and* leaks no temp). One phrasing nit, non-blocking: "rename is atomic on ... Windows" is slightly generous — Rust's `MoveFileExW(MOVEFILE_REPLACE_EXISTING)` is replace-capable and effectively atomic on NTFS, but not POSIX-guaranteed. The behavior is right either way.

**(6) Triage-14 — Homebrew Apple Silicon: CLOSED.**
`/opt/homebrew/bin/mkvmerge` added to the macOS candidates with the Homebrew-docs citation and the Finder-doesn't-inherit-shell-PATH rationale in the doc comment; the platform-candidates test asserts it; the Flatpak exclusion is preserved explicitly.

**(7) Minors: ALL CLOSED.**
- #6 `identify-failed` in gui-common.ftl with `{ $detail }` — correct choice of param. One doc nit (non-blocking): the comment claims the error "never" carries a `file` param, but `identify_body` in lib.rs *does* attach one via `.with("file", ...)` after the `From` conversion; the `From` impl alone doesn't. Message renders correctly regardless since it only uses `$detail`, which is always present.
- #7 `ShellRenderer` deduplicated to one `pub(crate)` definition in lib.rs; run.rs imports it; test imports adjusted.
- #8 `IpcError::code` removed, every call site migrated to `::new`, tests renamed.
- #9 BUILDING.md: stale "not yet implemented" replaced with an accurate `test:e2e` description, `check:i18n` documented, CI paragraph updated.
- #11 `batch-browse-dir-tooltip` ("folder picker") on both dir browse buttons; `browse-button-tooltip` correctly reserved for the genuine file pickers, with the split documented in the ftl header.

**(8) Regression sweep: none found.**
All `IpcError::code` call sites migrated (a miss would fail compilation; gates green). New ftl keys are either literal in src (`batch-browse-dir-tooltip`, `batch-run-tooltip-run-active` — check-i18n hard-pass) or fall into the documented warn-only IpcError bucket (`identify-failed`). The async `start_run` is invisible to the mocked-IPC e2e layer (3/3 green confirms). `ConcurrencyTracker` `#[doc(hidden)]` is doc-only, stays `pub` for its cross-crate test consumers. App.vue/BatchView prop wiring uses `withDefaults` so BatchView still type-checks standalone, consistent with the JobsView `pendingRun?` precedent.

## Deviation judgment: `!runActive` gating vs. literal "reset after resolves Ok"

**The implementer was right to deviate, and the deviation is correct.** My literal suggestion is in fact deterministically broken on soft outcomes: `run-finished` is emitted inside the Rust command *before* its promise resolves (the module's own documented contract, which I verified in the first review), so `onRunFinished` writes the summary/rows first and a reset-after-resolve would clobber exactly what it just rendered. The reset must precede `startRun`; the only sound question is *when to skip it*, and `runActive` is the right gate. I traced the orderings:

- **Fresh real run**: reset → `runActive=true` → events flow → `run-finished` clears. Correct.
- **Fresh soft outcome**: reset before invoke; `onRunFinished` fills and clears `runActive` mid-await; the post-resolve `ensureJobsLength(0)` is a no-op. Correct — this is the case the literal reading breaks.
- **Fresh rejection** (settings error, or the documented microsecond `finish_teardown` race): `startingFresh` guard sets `runActive=false`, error banner shown, retry recovers. The prior finished-run display is lost, which is acceptable (history intact).
- **Double-dispatch against an active run**: no reset, `startRun` rejected `run-already-active`, and — the key part — the `catch` no longer flips `runActive` (guarded by `startingFresh`), so the live run stays visible and cancellable with the error banner on top. This is precisely the failure I flagged, closed.
- **Interleaved rapid dispatches**: the check-then-set (`startingFresh` → `runActive=true`) executes within one synchronous continuation segment, so JS's single-threaded FIFO microtask ordering makes it effectively atomic; the second dispatch always observes the first's `runActive=true`.

The one residual (frontend believes `runActive=true` while the backend has no run, requiring a lost `run-finished` event) is unreachable within the design's own contracts (listeners registered before any dispatch, views never unmounted mid-run). The watcher-based `update:runActive` emit is also the right mechanism — it covers every mutation site without per-site emits. **Deviation approved; the implemented form is strictly better than my literal wording.**

## Final verdict

**Ready to declare Plan 5 done? Yes.**

**Reasoning:** All five blocking findings are closed with discriminating RED/GREEN tests rather than assertion-only patches, the one deliberate deviation is an improvement over the review's literal wording, and the full-wave sweep shows no regressions against any layer contract from the original review. The two residual nits (ftl comment imprecision on the `file` param, Windows rename-atomicity phrasing) are documentation-level and carry no behavior. The immediately-following full-matrix 3-OS run remains the last verification step per the coordinator's plan — a red Windows/macOS leg is handled in-plan and does not gate this verdict.