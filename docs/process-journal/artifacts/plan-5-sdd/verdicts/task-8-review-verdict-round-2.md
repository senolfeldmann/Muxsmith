<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-8  (round 2 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01LM2pUR9dJSTjjm3XL9dUdE
  agent_id:           a684692789984465e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a684692789984465e.jsonl
  dispatch_desc:      Review Task 8 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-10T14:45:04.824Z
  continuation_trigger: The coordinator sent a message while you were working: Re-review request for your Task 8 f
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Verdict: Approved

### Spec Compliance (re-review scope: 20abae1 + e7cb673)
- ✅ **Finding 1 closed (test counts truthful).** Independently counted at HEAD e7cb673: `error.rs` 3 + `run.rs` 36 = **39** `#[test]` functions, matching the report's "39/39 (error.rs 3 + run.rs 36)" exactly. The fix-round report also honestly retro-corrects the original "37" to the true 27 and shows its arithmetic for the 27 -> 31 -> 39 progression (2 removed with `lock_for_start`, 6 reservation tests added, 8 D31 tests added). Bookkeeping is now trustworthy.
- ✅ **Finding 2 closed (event-ordering documented).** `start_run`'s rustdoc carries an explicit "Event-ordering contract (frontend requirement)" section (subscribe to both events before invoking, diff lines 268-276); `finish_without_queue` and `emit_run_finished` each identify themselves as the emit sites behind that contract. No behavior change, as claimed.
- ✅ **Finding 3 closed (lock never held across planning; close handler cannot stall).** `Reservation::acquire` holds the mutex only for the check-and-insert (diff 205-218); every remaining acquisition is O(1): `commit` (pointer swap), `Drop` (clear), `close_decision` (`is_some`), `abort_and_quit`/`do_cancel_*` (atomic store or `cancel_all`, itself one atomic store). `on_close_requested`'s only lock touch is `close_decision`, so a close during a long planning pass can no longer freeze the event loop. The RAII `Drop`-without-`commit` covers all four soft-outcome early returns plus a mid-planning panic — verified each `drop(guard)` site became `drop(reservation)` — so no path leaks into permanent `run-already-active`. Beyond the ask: a cancel/close landing mid-planning is *honored*, because the reservation's flag is the very `Arc<AtomicBool>` handed to `QueueControl::new` (diff 367-371), so the queue is born already-cancelled; pinned by `cancel_run_during_planning_reaches_the_later_queue`.
- ✅ **Finding 4 closed (D31 sound).** Verified against the D31 memo on master (`docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md:242`); the implementation matches every memo point (prevent_close, confirmation, Yes = cancel + quit-after-teardown, No = stay open, idle window closes normally, suppression preference correctly deferred to v1.x). Detail verification below.
- ✅ **No-exit-before-summary.json:** the runner thread's sequence is `run_batch` -> `run_document` -> `finalize_joblog` -> `emit_run_finished` -> `finish_teardown` (diff 407-414); the slot-clear moved out of `run_batch` (diff 696-698) into `finish_teardown`, which runs strictly after finalize+emit. `abort_and_quit`'s direct-exit arm fires only on slot `None`, and slot `None` now provably means finalize completed. The one interleaving where the queue has finished but `finish_teardown` hasn't cleared yet: `abort_and_quit` sees `Running`, does a no-op `cancel_all`, defers to the flag — `finish_teardown` then consumes it after the (already-done) finalize. No path exits before `summary.json` on the real-run path. On soft paths an exit can precede the `run-finished` emit in one narrow interleaving, but no joblog exists there and the app is quitting; immaterial.
- ✅ **Exactly-once exit:** `quit_if_requested`'s atomic `swap` is the single consume point; all three completion paths (`finish_teardown`, `finish_without_queue`, `abort_and_quit`'s direct arm) funnel through it. Traced the racy interleavings (Yes concurrent with teardown completing; Yes after teardown; double-Yes from stacked dialogs): at most one `swap` observes `true`. Pinned by `quit_flag_plus_teardown_completion_exits_exactly_once` with an injected exit counter.
- ✅ **Reserved-case quit:** both sub-cases work. Planning ends soft: reservation drops, `finish_without_queue`'s `quit_if_requested` exits (tested, `abort_and_quit_during_planning_exits_after_a_soft_outcome`). Planning succeeds: the born-cancelled queue drives every job to `Cancelled` without spawning, a proper cancelled-run `summary.json` is written, then `finish_teardown` quits — arguably better than mkvtoolnix (history preserved).
- ✅ **`':'` rejection:** one predicate added with the Windows drive-prefix rationale in the doc; tests cover `"C:"`, `"C:x"`, `"a:b"`. My prior Minor closed.
- ✅ **Dialog semantics verified from the pinned dependency source** (not the report's context7 claim): `tauri-plugin-dialog-2.7.1/src/lib.rs:323` — `show(bool)` yields `true` iff the pressed button matches `OkCancelCustom`'s first label, including the Linux manual `Ok -> Custom(ok_label)` mapping in `desktop.rs:228-240`. So `abort == true` really is the "Abort jobs and quit" button — the one load-bearing, untestable semantic, confirmed. Also confirms `show` schedules via `run_on_main_thread` + detached thread; the event loop is never blocked and the callback runs off-loop, where `abort_and_quit`'s O(1) lock use is harmless.
- ✅ **`.ftl` route:** four single-line entries, wording faithfully derived from the mkvtoolnix reference named in D31 (with the honest "and quit" extension); constraint pinned three ways (ftl comment, rustdoc, tests). `ftl_message` prefix-match hazard is tested (`close-abort` must not match `close-abort-title`), empty-value multiline breakage is caught by the `!value.is_empty()` assertion, missing key degrades to the key (prose-free, no panic).
- ✅ Capabilities/plugin registration claims check out: `tauri_plugin_dialog::init()` pre-existing in `lib.rs`; Rust-side `app.dialog()` does not cross the IPC permission layer; `lib.rs`'s doc now says so explicitly.

### Strengths
- The `Reservation` RAII design is the right shape: one mechanism covers all four soft outcomes and panics, and reusing its flag as the queue's batch flag turns "cancel during planning is tolerated" into "cancel during planning is honored" — a semantic improvement the original review didn't even ask for.
- The teardown-ordering inversion (slot-clear after finalize+emit) is exactly the invariant D31 needed, and its cost (the microsecond `run-already-active` on instant restart) is identified, weighed, and documented at the code site rather than discovered later by T11.
- Every D31 state-machine path has a dedicated test with an injected exit closure; the exactly-once property is pinned behaviorally, not asserted in prose.
- Both fix-round reports are honest to the point of self-incrimination (the test-count retro-correction, the named untestable gaps).

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
1. **Runner-thread panic now wedges the app unclosable.** If the detached thread panics between `commit` and `finish_teardown` (realistic sources are few — `run_batch`'s `join().expect(...)` fires only if the queue thread itself panicked, e.g. via a poisoned `killers` mutex; the serialize `expect`s are genuine never-fails), the slot stays `Running` forever. Pre-D31 that only meant permanent `run-already-active`; post-D31 it also means every close is prevented and Yes never exits (`slot` never empties, the flag is never consumed) — the user must kill the process. A drop-guard in the thread body mirroring `Reservation` (clear slot + consume quit flag on unwind) would close it for ~10 lines. Low probability, amplified consequence.
2. **`ftl_message` truncation gap for `close-abort-message`.** The test pins exact wording only for the title; a future edit making the *message* a Fluent multiline (value on first line + continuation) ships a silently truncated dialog text — non-empty and non-key, so both existing assertions pass. Pin exact wording for all four keys (they are fixed strings anyway).
3. **Dialog stacking (disclosed residual): agree cosmetic, with one sharper edge worth knowing.** The idempotence argument holds for the flag and exit paths. But a stale stacked dialog surviving across a run boundary can have its Yes clicked after a *new* run started: `abort_and_quit` then cancels run 2 and quits after it — the flag-discard in `acquire` protects against the orphaned *flag*, not an orphaned live *callback*. Requires the user to deliberately keep a stale dialog open and confirm it, and the dialog's text ("abort all currently running jobs and quit") still describes what happens, so the behavior is defensible. The eventual fix (dialog-open bookkeeping) covers both this and the plain stacking; fine to defer.
4. **Instant-restart `run-already-active` window (disclosed residual): accept.** Correctly weighed — a recoverable retry against unrecoverable joblog loss is not a close call. Recommend the constraint ("do not invoke `start_run` synchronously from the `run-finished` handler") be carried into T11's brief rather than living only in `finish_teardown`'s rustdoc.
5. Carry-over from the first review, unchanged: universal `lock().unwrap()` on `AppState.active` has no poison-recovery; with `close_decision` now on the event-loop thread, a poisoned mutex would panic the close handler itself. Same low probability as before; noted for completeness.

### Assessment
**Task quality:** Approved
**Reasoning:** All four prior findings are verifiably closed in the diff (counts recounted, contract documented, O(1) lock discipline confirmed at every acquisition site, D31's ordering and exactly-once invariants traced through the racy interleavings and pinned by tests), and the one untestable load-bearing semantic (dialog button polarity) was verified against the plugin's own source at the pinned version. The remaining items are low-probability robustness polish, none of which blocks trust in the branch.