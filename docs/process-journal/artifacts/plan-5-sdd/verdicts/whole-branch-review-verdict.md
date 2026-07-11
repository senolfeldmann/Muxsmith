<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_016qmNddDHZXfpWfWmMJruCM
  agent_id:           a4cc466b850c3126e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a4cc466b850c3126e.jsonl
  dispatch_desc:      Whole-branch review Plan 5
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T17:32:41.081Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

All verification done. Here is the whole-branch review.

---

# Whole-Branch Review: Plan 5 (GUI run path), 735c723..945ee96

## Strengths

- **The D31/quit machinery is genuinely well-engineered.** `Reservation` as an RAII slot hold (src-tauri/src/run.rs:109-163), the `swap`-based exactly-once `quit_if_requested`, and the "slot empty = teardown fully complete, joblog included" invariant are correct under every interleaving I traced (abort-vs-teardown races, stale-flag discard on `acquire`, dialog-open-across-run-end). The unit tests cover exactly the racy paths, not just the happy ones.
- **The D25 mid-spawn race closure is exemplary**: the lost-cancellation window (cancel between pre-spawn check and killer registration) is closed by a post-insert re-check (queue.rs `RegisteringSpawner::spawn`), and the test drives the exact window deterministically with a gate spawner instead of sleeps (queue.rs `cancel_job_during_spawn_window_is_not_lost`).
- **Wire contracts are pinned, not assumed**: the `JobEvent` serde golden test, `src/ipc.ts` mirroring Rust structs field-for-field, and the e2e fixtures typed `satisfies JobEvent` against the same union give three layers that fail loudly on drift.
- **Evidence discipline throughout**: `MIN_SUPPORTED = (86, 0)` derived from the v19→v20 schema diff with the derivation recorded in the const doc; platform candidates cited to mkvtoolnix's own packaging files; the Playwright IPv6 bind pin documented with the observed `ss` evidence; deny.toml ignores each justified with reachability analysis (the quick-xml build-time-only argument is exactly right).
- **Security fundamentals in the shell are sound**: `valid_run_id` traversal guard including the Windows drive-prefix hazard, capability file at the actual minimum with the dialog→fs-scope trust chain documented, no `v-html` anywhere in the frontend (all untrusted mkvmerge output/filenames render through escaped text interpolation).
- **The e2e mock harness design** (bundling the real `@tauri-apps/api/mocks` + `emit` instead of reimplementing the event wire contract) is the right call and keeps the smoke honest about the documented subscribe-before-invoke ordering.

## Issues

### Critical (Must Fix)

1. **`start_run` resolves mkvmerge via PATH-only `Mkvmerge::locate()` and never reads the settings override** — src-tauri/src/run.rs:251. Every other mkvmerge-touching command (`dry_run`, `identify`, `detect_mkvmerge`, lib.rs:6355/6418/6432) deliberately substitutes `Mkvmerge::detect(settings.mkvmerge_path)`; `dry_run_body`'s doc even calls the substitution out as the design. T8 mirrored the CLI's `run.rs` verbatim and missed T7's substitution — the exact cross-task drift a task-scoped review cannot see. Consequence: on Windows (standard `%ProgramFiles%\MKVToolNix` install, not on PATH) and for any user who set a manual override in first-run, detection succeeds, dry-run succeeds, the Run button enables (its gate reads `dry_run`'s `mkvmerge_found`), and then **every actual run soft-fails** with `mkvmerge_found: false`. The primary Windows run path is broken end-to-end. Fix: load settings in `start_run` and use `Mkvmerge::detect(override)`.

### Important (Should Fix)

2. **`start_run` is a synchronous command, so the entire planning pass runs on the main thread** — src-tauri/src/run.rs:224-245. Tauri v2 docs (verified today): "Commands without the async keyword are executed on the main thread." The codebase itself documents this model at `detect_mkvmerge` (lib.rs:6504-6517) and made `dry_run`/`identify` async+`spawn_blocking` for far cheaper work — yet `start_run` shells out `mkvmerge -J` per source file synchronously. A large batch freezes the window for the whole planning pass, and it makes the Reservation's raison d'être partially moot: `cancel_run` and the D31 close dialog cannot fire during planning because the event loop is occupied — the carefully tested cancel-during-planning paths are unreachable from the production UI. Fix: `async fn` + planning inside `spawn_blocking`, consistent with `dry_run`; the Reservation design then actually earns its keep.

3. **D23 divergence: "the UI additionally disables Run while active" is not implemented, and the failure mode is destructive.** BatchView's Run gate (src/views/BatchView.vue:243) knows nothing about run state; clicking Run during an active run makes JobsView wipe `jobs`/`logLines`/`finishedSummary` *before* `start_run` rejects (src/views/JobsView.vue:161-173), then the catch sets `runActive = false` — the still-running batch's rows are gone, the cancel-batch button disables (JobsView.vue:237), and the run is invisible and uncancellable until `run-finished` reconciles. Two fixes, both wanted: (a) in JobsView, reset state only after `startRun` resolves Ok; (b) surface run-active state to BatchView and disable Run, per the memo's explicit sentence.

4. **Runner-thread panic wedges the app unclosable post-D31** (triage item 2, promoted): `run_batch`'s `handle.join().expect(...)` (run.rs:633) panics the detached thread; `finish_teardown` never runs, the slot stays `Running`, `close_decision` forever answers `ConfirmAbort`, and the confirmed quit's flag is never consumed — only SIGKILL ends the process, and `summary.json` is lost. The ~10-line drop-guard (clear slot + `quit_if_requested` in a `Drop` impl on the runner thread) is cheap insurance against turning any future core bug into an unclosable app.

5. **A torn/corrupt `settings.json` bricks the GUI with no in-app recovery.** `settings::save` is a plain `fs::write` (settings.rs:143; triage item 3), and settings writes happen constantly (every profile pick, every dir change). After a torn write: `detect_mkvmerge` → `settings-parse-failed` → App mounts FirstRun → FirstRun's own recovery (`attempt(true)`) calls `getSettings()` first, which throws the same error — a closed loop with no exit short of manually deleting a hidden file. Fix the settings half now: write-temp-then-rename is ~5 lines and removes the torn-file case entirely.

### Minor (Nice to Have)

6. **`identify-failed` renders as a bare key if ever hit in the GUI**: `error.rs` maps `IdentifyError::Json/Stat` to `identify-failed`, which exists only in `cli.ftl` — excluded from the frontend catalog set by design. Latent only (no view calls `identify()` today), but the command is registered IPC surface; add the key to `gui-common.ftl` or note the gap.
7. **`ShellRenderer` is defined twice** (lib.rs:6207 and run.rs:7817, identical). One `pub(crate)` definition suffices.
8. **`IpcError::new` / `IpcError::code` duplicate constructors** — self-documented as a merge artifact (error.rs:5951-5957); collapse to one in the next pass.
9. **BUILDING.md is stale at HEAD**: line 83 still says `pnpm test:e2e # Playwright e2e suite - not yet implemented (later task)`; `check:i18n` is unmentioned. Update alongside the fix wave.
10. **TS type drift, harmless today**: `get_settings` omits `None` fields (`skip_serializing_if`) so `AppSettings.mkvmerge_path` arrives as `undefined`, while `src/ipc.ts` types it `string | null`. Every consumer uses `??`, so no behavior bug; either drop `skip_serializing_if` or widen the TS types.
11. **`browse-button-tooltip` ("Choose the file with a file picker.") is reused for directory pickers** in BatchView — wrong noun for the dir case.
12. **`get_job_log` reads a potentially large log file synchronously on the main thread** — same sync-command consideration as #2, much smaller blast radius.

## Triage of the 16 accumulated items

1. **DEFER** — self-heals on any subsequent profile pick (updateSettings re-fetches); worst case is a wrong parallelism default, not a correctness fault.
2. **FIX-NOW** — promoted to Important #4 above: unclosable app + lost summary.json is the exact loss D31 exists to prevent, for ten lines.
3. **SPLIT: FIX-NOW for settings.rs** (Important #5: torn file has no in-app recovery), **DEFER for joblog** (torn record degrades to one skipped history entry; readers are already lenient).
4. **DEFER** — bounded syscall under the lock, no lock inversion exists (`active`→`killers` is the only nesting direction).
5. **DEFER** — with item 2's drop-guard in place the realistic poisoning source is gone; the remaining critical sections are trivial assignments.
6. **DEFER** — one redundant subprocess on the PATH rung at startup; measurable but irrelevant.
7. **DEFER** — a shared test-support crate for 2-3 mirrored helpers is an abstraction the scale hasn't earned; the mirror comments keep them honest.
8. **DEFER** — all four keys are tested to resolve and be non-empty; pinning one exact wording is enough to catch the line-parser contract break.
9. **DEFER** — the confirmed-defensible reading holds: "abort running jobs and quit" does what it says to whatever is running, and `acquire()` already discards stale quit flags.
10. **DEFER** — documented in the script header; the warning half is non-failing by design.
11. **DEFER** — comment-mirrored on both sides; a single source would need codegen machinery the constant doesn't warrant.
12. **DEFER** — deliberate CLI parity, documented in ResolutionTable.vue.
13. **DEFER** — background bookkeeping must not block profile work; `console.warn` is proportionate.
14. **FIX-NOW** — one line, and the exclusion rests on the wrong authority: mkvtoolnix's packaging tree can't testify about Homebrew, whose own documented prefix is `/opt/homebrew` on Apple Silicon. The sharpening fact: **GUI apps launched from Finder do not inherit the shell PATH**, so "brew users have it on PATH" does not hold for a Tauri app — detection fails for the most common macOS install route, while `/usr/local/bin` already incidentally covers Intel brew. Cite Homebrew's docs in the comment.
15. **DEFER** — the documented rationale (can't safely hide PATH mkvmerge process-globally in a parallel test binary) is sound; a candidates-injection seam is Plan 6 material if ever.
16. **FIX-NOW as verification, not code** — one full-matrix run (workflow_dispatch or a PR) before declaring Plan 5 done. `src-tauri`, the `cfg(windows)`/`cfg(macos)` branches, and `windows_subsystem` have never compiled off Linux; "eight gates green" is currently a Linux-only statement.

## Recommendations

1. **One fix wave, then re-gate**: (a) `start_run` → settings override + `Mkvmerge::detect` + async/`spawn_blocking` (Critical #1 and Important #2 are the same function); (b) JobsView reset-after-success + BatchView Run disable (D23); (c) runner drop-guard; (d) settings temp+rename; (e) `/opt/homebrew/bin/mkvmerge` candidate; (f) BUILDING.md refresh. (a)+(b) need a small shell test (override honored by start_run) and ideally one smoke assertion (Run disabled while active).
2. **Full-matrix CI dispatch** after the fix wave; treat a red Windows/macOS leg as in-scope for Plan 5.
3. **Go-public gate list** (carry into the HANDOFF, not this plan): replace `"csp": null` (tauri.conf.json:21) with a real CSP — the IPC surface is real now and defense-in-depth costs one config line; the low-XSS posture (no `v-html`, escaped interpolation) is currently the only layer. Also there: log pruning, dialog-suppression setting, mkvtoolnix version pin in CI.
4. The layer contracts otherwise held up under cross-checking: JobEvent golden shape ↔ shell emit ↔ `ipc.ts` union ↔ e2e fixtures are consistent; `summary.json` (no `joblog_status`) vs the live event (spliced) is consistently typed and even documented in RunHistory; the i18n chain (core codes → diagnostics.ftl, shell codes → gui-*.ftl, check-i18n's documented blind spots) is coherent apart from the `identify-failed` gap.

## Assessment

**Ready to declare Plan 5 done?** **No — with fixes** (one focused wave, then yes).

**Reasoning:** The architecture, concurrency design, and test discipline are strong, but the branch ships one genuine end-to-end break no task-scoped review could see: the run path ignores the mkvmerge override and platform detection, which defeats the entire D28 first-run story on Windows and for override users — the feature works in every layer except the one that muxes. Together with the sync-command UI freeze in the same function, the missing D23 Run-gate, and the two cheap robustness fixes (panic drop-guard, atomic settings write), that is a half-day fix wave plus one full-matrix CI run; everything else on the list is legitimately deferrable.