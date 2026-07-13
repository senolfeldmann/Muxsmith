# House-knowledge clusters - domain `executor`

Reconstructed from 67 occurrence records spanning eras E0-E8. Records were grouped by identical `(topic, approach)`; occurrences merged, identical `date+ref` deduped, distinct `date+ref` kept as genuine recurrence. `promoted = count >= 3`.

**Date basis** (resolved against the repo; see closing notes): spec = 2026-07-08; Plan-4 decision memos (D7/D13/D14/D17) = 2026-07-09; Plan-4 self-review = 2026-07-10; Plan-5 decision memos (D24/D25/D26) = 2026-07-10; Plan-4/Plan-5 task & whole-branch review verdicts = 2026-07-11; Plan-5.5 (D35, plan T4.5, `task-*-verdict.md`, `whole-branch-verdict.md`) = 2026-07-12. Fix commits carry their own `git` date: `f394f61`/`b9960c5` = 07-09, `75c075f`/`4b1ddbf`/`e06bda0`/`f54cbab` = 07-10. A violated-corrected occurrence is dated by its fix commit where one is cited.

**Three clusters reach the promotion threshold**, all decision *threads* that were deferred one or more times before being adopted: persisted per-job logs (exec-40, count 3), `delete_partial` error-surfacing (exec-19, count 3), and run-log 14-day auto-prune (exec-44, count 3). Everything else is a single- or double-touchpoint decision. No count was padded: distinct rejected-alternative restraints and distinct implementation bugs are kept as their own clusters instead of being folded into the adopted-decision count.

---

## exec-01-output-naming - Output name is keep-source or rendered template; collision defaults to error
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Output name is keep-source or a rendered template; filesystem collisions resolve by `error | skip | overwrite`, defaulting to `error`.
- **Steelman:** null (the in-place-replacement restraint that borders this is exec-02).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §2 row 4 + §4.8 | "error \| skip \| overwrite, default error. In-place replacement excluded." |

---

## exec-02-no-in-place - Source files never modified; in-place replacement is a hard exclusion
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Muxsmith never writes over its inputs; in-place replacement is excluded as a hard, non-configurable rule, and an output path equal to any input is always a hard error.
- **Steelman:** In-place metadata editing (mkvpropedit-style) would save disk space and avoid a separate output directory for small tweaks.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §1 + §2 row 4 + §11 | "Source files are never modified or overwritten. Hard rule, not configurable." |

---

## exec-03-mkvpropedit-fastpath - mkvpropedit metadata-only fast path deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** A mkvpropedit fast path for metadata-only changes is deferred; the full remux path ships first.
- **Blocked on:** v1.x optimization; full remux path ships first.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | deferred | spec §11 | "mkvpropedit fast path for metadata-only changes." (v1 non-goal) |

---

## exec-04-spawn-trait-seam - Process spawn behind a Spawn trait mirroring Identify
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Process execution sits behind a `Spawn` trait mirroring the existing `Identify` injection (live spawner + scripted fake), so progress-parse, state-machine and exit-code mapping are unit-testable without a real binary; real mkvmerge only in the gated integration tier.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D7 (Plan 4 forward decisions) | "Executor process spawn is abstracted behind a trait, mirroring the existing Identify injection ... real mkvmerge only in the gated integration tier." |
| 2026-07-11 | reinforced | plan-4 task-1-review-verdict.md | "Spawn/RunningJob/Killer/LiveSpawner/FakeSpawner match the brief; the scripted fake makes the job runner and queue unit-testable, mirroring Identify (D7/D13)." |

---

## exec-05-std-threadpool - `--jobs N` uses a bounded std thread pool, no async runtime
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `--jobs N` uses a bounded std thread pool with no async runtime (tokio), because muxing is external-process / I-O-bound so std suffices and the dependency surface stays minimal.
- **Steelman:** An async runtime (tokio) is the conventional, ecosystem-default way to supervise many concurrent child processes with backpressure and cancellation.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D7 (Plan 4 forward decisions) | "--jobs N uses a bounded std thread pool, no async runtime (tokio): muxing is external-process / I-O-bound, std suffices, dependency surface stays minimal." |

---

## exec-06-sigint-cleanup - SIGINT cleanup ships in Plan 4 (kill children + delete partials)
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** On Ctrl-C the executor kills in-flight children and deletes their partial outputs; this is scoped into Plan 4 rather than deferred.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D7 (Plan 4 forward decisions) | "SIGINT cleanup ships in Plan 4 (not deferred): on Ctrl-C, kill in-flight children and delete their partial outputs." |

---

## exec-07-queue-in-core - FIFO job queue lives in core's executor layer, not the CLI
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** The FIFO job queue lives in muxsmith-core's executor layer so the Plan-5 GUI reuses it as-is; interfaces (`JobEvent`/`QueueOpts`/`run_queue`) matched the brief verbatim and the seam chain was drift-free.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 | "The FIFO job queue lives in muxsmith-core's executor layer, not in the CLI." |
| 2026-07-11 | reinforced | plan-4 task-3-review-verdict.md | "JobEvent/QueueOpts/run_queue match the brief verbatim in executor/queue.rs; T1->T2->T3->T8 seam chain drift-free." |

---

## exec-08-jobevent-mpsc - Serializable `JobEvent` data enum over `std::sync::mpsc`
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Workers emit a serializable `JobEvent` data enum over `mpsc` (serde `tag="event"`, snake_case: Started/Progress/Warning/Error/Finished), serving the CLI renderer now, the Tauri payload in Plan 5, and the deferred NDJSON stream; per-index event ordering is guaranteed on the single channel.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 | "An event enum as data serves all three planned consumers: the CLI renderer now, the Tauri event payload in Plan 5, and the deferred NDJSON stream (D15)." |
| 2026-07-11 | reinforced | plan-4 task-3-review-verdict.md | "variants Started/Progress/Warning/Error/Finished verbatim; per-index event ordering guaranteed on one channel." |

---

## exec-09-event-delivery-callback-rejected - Callback/sink trait rejected for event delivery
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A callback/sink trait was rejected in favour of the mpsc event stream.
- **Steelman:** Workers call directly into a sink; conceptually simpler wiring, but the sink must be `Send+Sync` and the renderer must lock stdout internally.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 (alternative rejected) | "a callback/sink trait (workers call into the sink, which must be Send+Sync, and the renderer must lock stdout internally)." |

---

## exec-10-event-delivery-polled-rejected - Polled shared state rejected for event delivery
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Polled shared state was rejected in favour of the event stream.
- **Steelman:** Simplest to reason about, but loses event granularity and forces a poll cadence.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 (alternative rejected) | "polled shared state (loses event granularity, imposes a poll cadence)." |

---

## exec-11-rendering-drain-caller-thread - Caller drains the receiver and owns all rendering
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The caller drains the receiver on its own thread and owns all rendering, keeping stdout single-threaded.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 | "The drain-on-caller-thread model keeps stdout ownership single-threaded." |

---

## exec-12-job-state-set - Terminal job states mirror mkvtoolnix-gui one-to-one
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Terminal job states are `Pending -> Running -> {Ok, Warning, Failed, Cancelled}`, mirroring mkvtoolnix-gui's DoneOk/DoneWarnings/Failed/Aborted one-to-one (mux_job.cpp:154-159), which independently confirms the spec-6 exit-code mapping.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 | "The state set mirrors mkvtoolnix-gui's DoneOk/DoneWarnings/Failed/Aborted one-to-one (mux_job.cpp:154-159)." |

---

## exec-13-output-parent-dir - Executor creates the output parent dir before spawning
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** The executor ensures the output's parent directory exists (`create_dir_all`) before spawning; mkvmerge's own directory-creation is not relied on.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D13 | "The executor ensures the output's parent directory exists before spawning; mkvmerge's own directory-creation behavior is not relied on." |
| 2026-07-11 | reinforced | plan-4 task-2-review-verdict.md | "parent_dir_created_before_spawn drives an output in a not-yet-existing subdir; create_dir_all(parent) runs before spawn." |

---

## exec-14-failfast-soft - `--fail-fast` is soft (stop dequeuing, in-flight finish, queued -> Cancelled)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** `--fail-fast` is soft: on the first Failed job the queue dequeues nothing further, already-running jobs finish, and still-queued jobs become Cancelled.
- **Steelman:** A hard-kill variant would stop wasted work sooner on systemic failures (captured as the deferred exec-16 and the rejected default exec-15).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D14 | "On the first Failed job, the queue dequeues nothing further; already-running jobs finish; jobs still queued become Cancelled." |
| 2026-07-11 | reinforced | plan-4 task-3-review-verdict.md | "soft_fail_fast_cancels_queued_but_not_inflight asserts [Failed,Cancelled,Cancelled], exactly one Started AND fake.spawned().len()==1." |

---

## exec-15-batch-no-hardkill-default - Hard-kill on first failure rejected as the batch default
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Hard-kill of in-flight jobs on first failure was rejected as the default; a failure does not abort the batch. mkvtoolnix-gui has no fail-fast (model.cpp:337-380 calls startNextAutoJob unconditionally); make/cargo/GNU-parallel stop scheduling and let running jobs finish; kill+delete-partial is reserved for mkvmerge error or user cancel.
- **Steelman:** On systemic failures (disk full, unwritable dir) a hard-kill stops the siblings immediately instead of letting them run seconds longer.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D14 | "mkvtoolnix-gui has NO fail-fast ... make/cargo/GNU-parallel stop scheduling and let running jobs finish." |

---

## exec-16-failfast-now-deferred - `--fail-fast=now` (kill in-flight too) deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** A `--fail-fast=now` value that also kills in-flight jobs (GNU-parallel `now,fail=1` analog) is deferred to v1.x pending real usage demand.
- **Blocked on:** external - real usage demand.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | memo D14 | "Considered and deferred: a --fail-fast=now value ... as an additive v1.x flag if real usage asks for it." |

---

## exec-17-delete-partial-on-fail - Partial outputs deleted on mkvmerge error and on cancel
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Partial outputs are deleted on mkvmerge error (exit 2) and on cancellation, a deliberate divergence from mkvtoolnix-gui's keep-by-default, because Muxsmith's bulk contract treats any file in the output tree as a valid output and `on_collision:skip` would otherwise silently accept a broken partial.
- **Steelman:** The GUI is interactive, so keeping the partial lets the user see the red job and use the partial for diagnosis; Muxsmith's bulk contract plus the skip-footgun overrides that.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D17 | "GUI keeps partials by default (settings.cpp:621); Muxsmith keeps spec 6 unchanged: partial deleted on exit 2 and on cancel." |
| 2026-07-11 | reinforced | plan-4 task-2-review-verdict.md | "exit_two_is_failed_and_partial_deleted and killed_under_cancel_is_cancelled_and_partial_deleted drive run_job through a real tempdir and assert the partial is gone." |

---

## exec-18-delete-partial-only-if-ran - `delete_partial` applies only to jobs that actually ran
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** `delete_partial` applies only to jobs that actually ran: a spawn failure (and later a skipped/pre-spawn-cancelled job) deletes nothing, so a pre-existing valid output under `overwrite` is never destroyed. Originally violated (spawn failure routed through delete-partial, destroying a pre-existing output), fixed to assemble the Err-arm outcome inline, then reaffirmed under D25's new skip/pre-spawn paths.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | plan-4 task-2-review-verdict.md + commit f394f61 | "Spawn failure routed through delete-partial, silently destroying a pre-existing valid output; fix returns the Err-arm outcome inline; pinned by spawn_failure_is_failed_but_keeps_preexisting_output." |
| 2026-07-11 | reinforced | plan-5 task-5-review-verdict.md | "Pre-spawn cancel constructs the outcome directly, bypassing finish/delete_partial entirely; delete_partial's trigger condition is unchanged." |

---

## exec-19-delete-partial-errsurface - Failed partial-delete surfaces via `outcome.errors` (deferred x2, then adopted)
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Surfacing a failed partial-output delete via `outcome.errors` was flagged and deferred twice at Plan-4 review (no `JobOutcome` error channel existed yet, and a failed remove leaves a broken partial that `on_collision:skip` would accept), then implemented in T5 as a `"delete_partial_failed: <io error>"` passthrough into `outcome.errors`; `NotFound` stays silently ignored.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | plan-4 task-2-review-verdict.md | "A failed remove leaves a broken partial that on_collision:skip would accept; no JobOutcome channel existed to report it." |
| 2026-07-11 | deferred | plan-4 whole-branch-review-verdict.md (ledger #2) | "fix direction exists (push a line into outcome.errors, which already carries the spawn-error string)." |
| 2026-07-11 | decided | plan-5 T5 step1.4 / task-5-review-verdict.md | "matches the specified \"delete_partial_failed: <io error>\" format exactly (job.rs:204-211), NotFound still silently ignored." |

---

## exec-20-killed-wait-none - Killed job reports `wait()=None` cross-platform via a shared flag
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A killed job must report `wait()=None` cross-platform; otherwise on Windows `Child::kill` is `TerminateProcess(h,1)` so `ExitStatus::code()` is always `Some(1) -> Warning -> partial kept`, never Cancelled. Fixed with a shared `Arc<AtomicBool>` killed flag set before `kill()`, folded through a pure `resolve_wait` that returns None when set.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | plan-4 whole-branch-review-verdict.md + commit 75c075f | "Windows Child::kill is TerminateProcess(h,1) ... fix shares Arc<AtomicBool> killed flag set before kill(), wait() folds through pure resolve_wait returning None when set." |

---

## exec-21-stop-vs-cancel-flags - Fail-fast (stop) and SIGINT (cancel) kept as separate flags
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Fail-fast (stop) and SIGINT (cancel) are separate flags so the 130 exit override can only come from a real SIGINT; fail-fast never masquerades as cancellation. Held through T3, T8 and T10.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | commit b9960c5 (T3), cited in plan-4 whole-branch-review-verdict.md | "Fail-fast never masquerades as cancellation, so the 130 override can only come from a real SIGINT ... it held through T3, T8, and T10." |

---

## exec-22-prespawn-cancel-check - Cancel observed before spawn yields Cancelled, spawner never called
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** A cancel observed before spawn yields Cancelled without invoking the spawner and deletes nothing. Proposed at Plan-4 whole-branch review to close most of the register-after-sweep window and deferred to v1.x, then implemented in T5, closing the HANDOFF backlog item.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | plan-4 whole-branch-review-verdict.md (recommendation #3) | "closes most of the register-after-sweep window without touching the one-shot watcher design." |
| 2026-07-11 | decided | plan-5 T5 step1.3 | "cancelled flag set before spawn -> Cancelled, spawner never called, nothing deleted." |

---

## exec-23-loop-resweep-watcher - Loop-and-re-sweep watcher for stronger cancellation deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** A loop-and-re-sweep watcher that would narrow the register-after-sweep window further (a design change to the one-shot watcher) is deferred; today a job registering its killer after the sweep runs to natural completion under cancel, bounded to one job per worker and recorded honestly.
- **Blocked on:** internal - design change if stronger cancel is ever wanted.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | plan-4 task-3-review-verdict.md (named-risk 2) | "A job registering its killer after the sweep runs to natural completion under cancel; bounded to one job per worker, outcome recorded honestly." |

---

## exec-24-midspawn-cancel-not-lost - Per-job cancel caught in the mid-spawn window must not be lost
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** A `cancel_job` caught in the mid-spawn window (between the pre-spawn check and killer registration) must not be silently lost: it originally set the flag but issued no kill and nothing re-checked, fixed with a post-insert re-check, verified deterministically with a gate spawner (`cancel_job_during_spawn_window_is_not_lost`) rather than sleeps.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | plan-5 task-5-review-verdict.md + commit e06bda0 | "the explicit cancel request is dropped with no error and no observable trace ... Fixed via a post-insert re-check." |
| 2026-07-11 | reinforced | plan-5 whole-branch-review-verdict.md (strengths) | "the test drives the exact window deterministically with a gate spawner instead of sleeps (queue.rs cancel_job_during_spawn_window_is_not_lost)." |

---

## exec-25-perjob-cancel-in-core - Per-job cancellation lands in core's queue
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Core's queue gains per-job cancellation now (kill-by-index via the registered `Killer` plus a queued-skip set), not only batch cancel; confirmed by Şenol.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | memo D25 (confirmed Şenol) | "per job cancel in core needs to be in core now" (Şenol 2026-07-10). |

---

## exec-26-perjob-cancel-defer-rejected - Deferring per-job cancel to a later plan rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Shipping only batch cancel with per-job cancel deferred to a later plan was declined by Şenol (a deviation from spec 8.2).
- **Steelman:** Contains Plan 5 scope; per-job cancel is largely UI sugar and the seam could be extended later without reworking core.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | memo D25 (alternative rejected) | "a deviation from spec 8.2 that Şenol declined." |

---

## exec-27-skipped-job-finished-no-started - Skipped queued job emits Finished{Cancelled} with no Started
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A skipped queued job emits a `Finished{Cancelled}` event with no preceding `Started` event, deviating from never-dequeued silence because the GUI needs the confirmation; documented in the `run_queue` rustdoc.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | plan-5 T5 step1 / task-5-review-verdict.md | "asserts Started == [0,1] and a Finished{2, Cancelled} event." |

---

## exec-28-queue-watcher-done-flag - Watcher needs a `done` flag so `thread::scope` terminates on completion
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The queue watcher needs a `done` flag set after all workers join so `thread::scope` terminates on every non-cancelled batch; the plan's pinned watcher (exit only on cancel) would deadlock the scope on any successful run.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | violated-corrected | plan-4 task-3-review-verdict.md + journal Deltas | "done is set after all workers join, so the scope terminates on every non-cancelled batch; without it thread::scope never returns on a successful run." |

---

## exec-29-worker-count-cap - Worker count capped at the spec count
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Worker count is capped at the spec count via a pure `worker_count(jobs, spec_count)` that clamps `>=1` then caps at `spec_count.max(1)`, so `--jobs 100000` over 2 files does not spawn 100000 threads.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | plan-4 whole-branch-review-verdict.md + commit 4b1ddbf | "Extracted pure worker_count(jobs, spec_count) clamps >=1 then caps at spec_count.max(1); compile-failure RED prevented the 100k-thread test running against unfixed code." |

---

## exec-30-mkvmerge-empirical - mkvmerge behavior confirmed by running the binary, never from memory
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** mkvmerge behavior is confirmed by running the installed binary, never assumed from memory (SI-3); T1 empirically captured the v100 gui-mode grammar (`#GUI#progress`, `#GUI#warning` exit 1, `#GUI#error` exit 2) before the parser encoded it.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | reinforced | plan-4 task-1-review-verdict.md | "T1 empirically captured the v100 gui-mode grammar ... before T2's parser encoded it." |

---

## exec-31-eventstream-reads-lossy - Event-stream reads use `read_until` + `from_utf8_lossy`, only `Ok(0)` is EOF
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Live event-stream reads use `read_until(b'\n')` + `from_utf8_lossy` and treat only `Ok(0)` as EOF, surviving non-UTF-8 output lines with no truncation and no pipe-full hang. Flagged at Plan-4 review (observed v100 output is ASCII, so deferred) then implemented in T3.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | plan-4 task-1-review-verdict.md + whole-branch Minor | "A non-UTF-8 byte in a warning line's filename ends the stream early; observed v100 gui-mode output is ASCII so practical risk is low." |
| 2026-07-12 | decided | plan-5.5 task-3-verdict.md / plan T3 | "read_until + lossy decode, Ok(0)-only EOF, distinguishable Err arm." |

---

## exec-32-percent-clamp-wontfix - Clamping the 101-255 progress-percent range deliberately not done
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Clamping the 101-255 progress-percent range was deliberately not done (wontfix): the grammar caps at 100 and milestones only compare `>=`, so no observable harm today; clamp opportunistically only if the parser is next touched.
- **Steelman:** Defensive robustness against malformed progress lines; but the real grammar caps at 100 and milestones only compare `>=`, so no observable harm.
- **Blocked on:** internal - only if the parser is next touched.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | plan-4 whole-branch-review-verdict.md (#3) | "grammar caps at 100; milestones only compare >=; no observable harm. Clamp opportunistically if the parser is ever touched." |

---

## exec-33-worker-panic-handling - Explicit worker-panic handling (v1 non-goal at review, then implemented in T4)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Explicit worker-panic handling was declared a v1 non-goal at Plan-4 review (run_job is panic-free by construction) then implemented in T4: a panicking worker is recorded Failed with a `worker-panicked` code (not Cancelled), poisoned mutexes are recovered via `unwrap_or_else(|p| p.into_inner())` behind documented getters, and the panic payload flows only to `eprintln`.
- **Steelman:** Defensive robustness (a panicking worker backfills Cancelled silently, a poisoned mutex would panic at `into_inner`), but run_job is panic-free on valid input - acceptable for v1 at the time.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | plan-4 whole-branch-review-verdict.md (#6) | "a worker panic implies a bug in run_job, which is panic-free by construction today; acceptable for v1." |
| 2026-07-12 | decided | plan-5.5 task-4-verdict.md / plan T4 | "Failed + worker-panicked (not Cancelled) ... poison recovery centralized behind getters with soundness docs; payload downcast flows ONLY into eprintln." |

---

## exec-34-childproc-leak-panic - Panic recovery must invoke the killer before removing it
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `recover_panicked_worker` discarded the removed `Killer` without invoking it, leaking an mkvmerge child that kept writing an output the queue reported Failed; corrected (FIX-NOW) to invoke-then-remove.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | plan-5.5 task-4-verdict.md (m2) / whole-branch-fix-report.md (T4-m2) | "queue.rs:398 removes the killer without invoking it; invoke-then-remove is one line, leaked mkvmerge keeps writing an output the queue reported Failed." |

---

## exec-35-poison-testlocks-bare - Test-only lock sites left without poison recovery
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Test-only lock sites are deliberately left without poison-recovery: a poisoned test lock should fail loudly rather than be recovered by the `into_inner` idiom used in production code.
- **Steelman:** A uniform poison-recovery idiom applied at every lock site for consistency.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | plan-5.5 task-4-verdict.md (concern 2) | "test-only sites deliberately left bare (poisoned test lock should fail loudly)." |

---

## exec-36-core-stderr-logging - `eprintln!` in core; log/tracing facade deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The single `eprintln!` in core (queue.rs) is the first direct stderr I/O in core; the idiomatic fix is a log/tracing facade the binaries route, deferred.
- **Blocked on:** Plan 6 (internal).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | plan-5.5 task-4-verdict.md (I1) / whole-branch-verdict.md (I3) | "eprintln-in-core -> idiomatic fix is a log/tracing facade the binaries route." |

---

## exec-37-panicked-msg-catalog - Routing `JobOutcome.errors` codes through the diagnostics catalog deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The rich `worker-panicked` message renders on no live surface (only JSON carries the token); routing `JobOutcome.errors` codes through the diagnostics catalog was deferred.
- **Blocked on:** Plan 6 (internal).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | plan-5.5 task-4-verdict.md (I2) / whole-branch-verdict.md (I3) | "worker-panicked rich message unreachable from live surfaces; fix = routing JobOutcome.errors codes through the diagnostics catalog." |

---

## exec-38-raw-output-event-variant - Full mkvmerge output captured as an additive raw-line event variant
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Full mkvmerge output is captured as a new additive raw-line `Output{index,line}` `JobEvent`/`JobProgress` variant carrying every non-`#GUI#progress` line verbatim (warn/error lines also keep stripped variants); it feeds the job-queue live log pane and the persisted job log, while progress ticks stay transient.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | memo D24 | "It carries every line mkvmerge writes that is not a #GUI#progress tick ... The raw stream feeds the job-queue view's live log pane and the persisted job log." |

---

## exec-39-raw-output-accumulate-rejected - Accumulating full output into `JobOutcome` rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Accumulating full output into `JobOutcome` was rejected in favour of the raw event variant.
- **Steelman:** Keeps all output in one already-returned struct with no new event variant to design or version; rejected because it bloats the in-memory outcome vector for large batches and duplicates what the incremental log writer already persists.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | memo D24 (alternative rejected) | "bloats the in-memory outcome vector for large batches and duplicates what the incremental log writer already persisted." |

---

## exec-40-persist-perjob-logs - Persist per-job logs (deferred x2, then adopted as JSON-per-job + summary)
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Persisting per-job logs was deferred twice - at D7 (to Plan 5, the GUI job-queue view being the consumer, with the CLI streaming to stdout + `--json` meanwhile) and again at the Plan-4 self-review - before being adopted in D26 as `runs/<run-id>/{summary.json, job-<index>.json}` under the platform data dir, written unconditionally by core for both CLI and GUI runs; dry-runs persist nothing.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | memo D7 (Deferrals) | "Persisted per-job logs ... deferred to Plan 5: the job-queue view is their consumer, and the CLI run in Plan 4 streams progress/results to stdout + --json instead." |
| 2026-07-10 | deferred | plan-4 Self-review | "Deferred by decision: ... persisted job logs (Plan 5)." |
| 2026-07-10 | decided | memo D26 (format+scope Şenol) | spec 6 "phrases persistence as a job-engine property, so GUI-only persistence would diverge from the spec." |

---

## exec-41-persist-logs-format-rejected - Three log-persistence formats rejected in favour of JSON-per-job + summary
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Three persistence alternatives were rejected in favour of JSON-per-job + summary: plain-text log + JSON index (two-artifact consistency burden), GUI-only persistence (spec divergence + CLI loses post-mortem logs), and a single NDJSON journal per run (NDJSON events are deferred to v1.x anyway).
- **Steelman:** A single NDJSON-per-run journal is one append-only artifact and would align with a future event stream.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | memo D26 (alternatives rejected) | "two-artifact consistency burden; spec divergence + CLI loses post-mortem logs; NDJSON events are deferred to v1.x anyway." |

---

## exec-42-persist-logs-writefail - Silent mid-run log-write failure must reach `finish()`
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A silent mid-run `job-<index>.json` write failure was invisible to `finish()` and still returned Ok, printing a false "logs written" success despite partial data loss in exactly the artifact D26 exists for; fixed by tracking `had_write_error` and reflecting it in `finish()`.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | plan-5 task-6-review-verdict.md + commit f54cbab | "a false-positive success message despite silent, partial data loss ... Fixed by tracking had_write_error and reflecting it in finish." |

---

## exec-43-runsroot-debug-gated - `MUXSMITH_RUNS_ROOT` test override gated to debug builds
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The `MUXSMITH_RUNS_ROOT` override used by tests was shipping as an unconditional, undocumented env-var surface read in the release binary; gated behind `#[cfg(debug_assertions)]`.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | plan-5 task-6-review-verdict.md (Minor 2) + commit f54cbab | "a permanent, undocumented env-var surface read unconditionally in the shipped binary ... worth gating behind #[cfg(debug_assertions)]." |

---

## exec-44-runlog-14day-autoprune - Run logs auto-prune at 14 days fixed (deferred, then adopted)
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Run-log pruning was left out of v1 at D26 (location documented, prune facility parked), then reversed at D35 to an automatic 14-day fixed prune with no v1 config, implemented in core `executor/joblog` so CLI and GUI both inherit; age is decided by the parsed run-id name only (never mtime), pruning is best-effort and symlink-safe with every IO error ignored. Parity MATCH with mkvtoolnix `removeOldJobs=true`/14 days.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | memo D26 | "No pruning in v1: the location is documented; a prune facility is a v1.x candidate." |
| 2026-07-12 | decided | plan T4.5 (D35) / plan-5.5 task-4.5-verdict.md | "Age is decided by the PARSED NAME ONLY, never mtime ... every io error during pruning is IGNORED." |
| 2026-07-12 | decided | memo D35 | "for this tool class the log is needed right away or not at all - its value decays in days"; parity MATCH with mkvtoolnix removeOldJobs=true/14 days. |

---

## exec-45-runlog-config-deferred - Configurable retention period deferred to IDEAS #7
- **kind:** non-decision | **status:** blocked | **count:** 2 | **promoted:** no
- **Statement:** Making the run-log retention period configurable (disable / change days) is deferred to IDEAS #7; v1 ships a fixed 14-day prune with no setting or flag, the configuration surface not yet earned.
- **Blocked on:** internal - option surface not earned; parked to IDEAS #7 pending real demand.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | plan T4.5 | "configurability is parked as IDEAS #7." |
| 2026-07-12 | deferred | memo D35 | "configuration surface not earned yet; deferred to IDEAS #7." |

---

## exec-46-runlog-keepforever-prunefacility-rejected - Keep-forever + explicit prune facility overruled
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Keeping run logs forever with deletion as an explicit prune facility (Peter's recommendation) was overruled at D35: the audit-value premise does not hold for this tool class in Şenol's judgment and unbounded history clutter is the worse default.
- **Steelman:** Batch run logs are audit artifacts; deletion should be an explicit, deliberate act.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | memo D35 (rejected alternatives) | "the audit-value premise does not hold for this tool class in Şenol's judgment; unbounded history clutter is the worse default." |

---

## exec-47-runlog-keepforever-plain-rejected - No pruning at all rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Doing no pruning at all was rejected at D35 (unbounded growth, cleanup nobody performs).
- **Steelman:** Simplest possible behavior, zero mechanism.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | memo D35 | "unbounded growth, cleanup nobody performs." |

---

## exec-48-runlog-mkvtoolnix-immediate-rejected - mkvtoolnix immediate-removal policies not adopted
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** mkvtoolnix-gui's additional immediate-removal-on-completion / on-exit policies were deliberately not adopted; only the 14-day default is, because immediate removal would delete the post-mortem record spec 6 mandates persisting.
- **Steelman:** Full parity with the mkvtoolnix oracle whose 14-day default Muxsmith already matched.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | memo D35 (parity note) | "they would delete the post-mortem record spec 6 mandates persisting." |

---

## exec-49-runid-parser-to-core - `run_id_timestamp` parser moved to core, shell delegates
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Reuse before writing: the `run_id_timestamp` parser was moved to core and the shell's `started_at_from_run_id` became a 4-line delegate, keeping three existing shell tests byte-identical.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | plan T4.5 / plan-5.5 task-4.5-verdict.md | "delegation is a 4-line shell wrapper, three existing shell tests byte-identical." |

---

## Clustering notes (defensibility)

- **Three promoted clusters (exec-19, exec-40, exec-44) are decision *threads*, not padded counts.** Each was genuinely deferred at one or more distinct review/plan artifacts before being adopted at a distinct later one. Merging a deferral with its eventual adoption into one cluster is the recurrence the promotion signal exists to capture; the occurrences are distinct artifacts (memo -> self-review -> memo; two review docs -> plan step; memo -> plan+verdict -> memo). The terminal state is `pattern` (adopted) in all three.
- **`delete_partial` split into three honest clusters, not one inflated one:** exec-17 (the positive delete-on-fail/cancel behavior), exec-18 (the scope boundary "only jobs that ran", a violated-then-reaffirmed correctness invariant), and exec-19 (the deferred-then-adopted error-surfacing thread). These are distinct considerations that happen to touch the same function; folding them would fabricate a count.
- **Cancellation-window narrowing was deliberately NOT collapsed into one count-3 cluster.** exec-22 (pre-spawn check: deferred at plan-4 review, adopted in T5) and exec-23 (loop-and-re-sweep watcher, still deferred) share the topic "Cancellation narrowing" but are different approaches/mechanisms; the strict `(topic, approach)` rule keeps them apart, so neither promotes. exec-24 (mid-spawn-window cancel-not-lost) is a distinct bug+verification pair, not a re-attestation of either.
- **Worker-panic split:** exec-33 is the "do we handle worker panics at all" decision, which flipped from a v1 non-goal (plan-4 review) to implemented (T4) - one thread, count 2. exec-34 (child-process leak in the panic-recovery code) is a distinct FIX-NOW bug within that implementation, kept separate rather than counted as a third occurrence.
- **Rejected alternatives are their own restraint clusters** (exec-09/10 event delivery; exec-15 hard-kill default; exec-26 defer-per-job; exec-39 accumulate-into-outcome; exec-41 log formats; exec-46/47/48 retention alternatives), each with its own steelman, instead of being absorbed into the adopted decision's count. exec-46/47/48 all come from the single D35 memo but are three genuinely distinct rejected options.
- **Non-decisions carry `blocked_on` and status `blocked`** (exec-03, exec-16, exec-23, exec-36, exec-37, exec-45). exec-45 recurred across plan T4.5 and memo D35 (count 2) but stays a non-decision - it kept coming back and is still deferred, not adopted.
- **Date basis** is documented in the header. Verdict-file mtimes are uniformly 2026-07-11 (plan-4/plan-5) and 2026-07-11..12 (plan-5.5), while fix commits land 2026-07-09/10; a violated-corrected occurrence is therefore dated by its cited fix commit (`f394f61`=07-09, `75c075f`/`4b1ddbf`/`e06bda0`/`f54cbab`=07-10) where one exists, otherwise by the review verdict. Decision memos (D7/D13/D14/D17 = Plan-4, D24/D25/D26 = Plan-5, D35 = Plan-5.5) are dated to their plan's decision phase; D25 is anchored by Şenol's 2026-07-10 confirmation.
