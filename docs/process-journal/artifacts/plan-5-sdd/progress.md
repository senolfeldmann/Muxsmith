# Plan 5 SDD progress ledger

Plan: docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md (T0-T13, 7 waves)
Started: 2026-07-10. Controller: Peter (Fable 5). Design memo: 2026-07-10-plan-5-gui-design-decisions.md (FINAL).
Wave 1 (T1-T4) runs as parallel worktree streams under .worktrees/plan5-tN, branches plan5-tN,
BASE for all wave-1 branches: c822a17. Merge order T1 -> T2 -> T3 -> T4 (T1/T2 both touch
run.rs in different regions - MilestoneState arm vs run_json_document hoist; resolve trivially),
full gate re-run per merge. Plan-4 ledger archived as progress-plan4.md.

## Status

Task 0: complete (Şenol: dnf webkit2gtk4.1/librsvg2/libappindicator; mise.toml node 26.5.0 + pnpm 11.10.0 = 656449c; verified webkit 2.52.4)
Pre-wave pinning (Şenol 2026-07-10): rust 1.96.1 + ctrlc 3.5.2 = 45e941a; CI SHA-pins + ubuntu-26.04 (preview) / windows-2025 / macos-15 + checkout v7 = 2ee2d0c; full gate green.

## Backlog additions (Şenol 2026-07-10)

- Pin mkvtoolnix version in CI (currently floating apt) - backlog, not Plan 5.

## Load-bearing constraints for implementer briefs (from Plan 4)

- run_job MUST drain next_line() to EOF before wait() (LiveJob::wait holds child mutex across waitpid; Killer during wait stalls).
- gui-mode grammar v100: progress `#GUI#progress NN%`; warning `#GUI#warning '<file>': <message>`; error `#GUI#error <message>`.
- read_line Err(_) = EOF is plan-mandated; do not "fix" ad hoc.
- ALL cargo/pnpm runs FOREGROUND; pnpm/node via `mise exec --` (no shell activation in subagent shells).
Task 1: complete (branch plan5-t1, commits c822a17..d6725c8, review clean/approved, no findings; MERGED eeb45f9, full gate green post-merge)
Session note (Şenol): stop after Wave 1 completes - no Wave-2 dispatch until his go (context7 auth restart); NO handoff update, session continues.
Task 2: complete (branch plan5-t2, commits c822a17..42ecc34, review clean/approved; DiagnosticRenderer trait = controller-signed design deviation, port/adapter, byte-identical preserved; MERGED c14a153, full gate green post-merge, 28 suites ok)
Task 3: complete (branch plan5-t3, commits c822a17..85cdc62, review clean/approved, minors only; MIN_SUPPORTED=(86,0) empirically derived, reviewer re-verified the whole evidence chain; MERGED, full gate green post-merge)
T7 carry-forward (from T3 review): consider /opt/homebrew/bin/mkvmerge candidate (Apple Silicon) with homebrew-core citation - outside mkvtoolnix packaging tree, product-completeness call at T7 dispatch. Also: ladder-exhaustion NotFound path has no direct unit test (disclosed, low risk).
Task 4: complete (branch plan5-t4, commits c822a17..63fdfc4 incl. 2 fix rounds: eslint stale-pin Important -> 9.39.4 -> controller-decided 10.6.0 newest-major, both re-reviewed Approved; MERGED, full six-gate green on master incl. tauri compile + pnpm lint/build)
Controller gate-infra fix on master: b020bb7 eslint ignores .worktrees/ (rustdoc JS; only reproducible in main checkout). Verified lint+build green.
Controller adjudications: 'Muxsmith' literal in productName/title = branding exemption (like mkvtoolnix); csp:null carried to T7/T8 (set a real CSP when IPC surface lands); T2 DiagnosticRenderer trait signed off.
WAVE 1 COMPLETE: T1-T4 merged (eeb45f9, c14a153, c7ef52a, T4-merge, b020bb7), worktrees removed, branches deleted, review diffs salvaged to main .superpowers/sdd/.
PAUSED by Şenol for Claude Code restart (context7 auth). Resume: Wave 2 = T5 per-job cancel THEN T6 joblog (serial, same files); context7 available again for all dispatches.
Task 5: complete (master, commits b020bb7..e06bda0 incl. race fix e06bda0; review found Important lost-cancellation race in mid-spawn window -> fixed with insert-then-recheck handshake + deterministic MidSpawnGateSpawner test, re-review Approved with two-sided ordering argument verified; controller gate re-run green)
Whole-branch-review notes from T5: (a) fix also closes Plan-4's batch-cancel post-sweep window (unclaimed improvement, verify in final review); (b) cancel_job invokes Killer while holding killers mutex (pre-existing pattern, bounded) - final-review note; (c) double-invocation path rests on documented idempotent Killer contract, untested directly.
Task 6: complete (master, commits e06bda0..f54cbab incl. fix f54cbab: had_write_error -> finish() Err + run-joblog-incomplete key, MUXSMITH_RUNS_ROOT gated debug_assertions; re-review Approved; controller gate green, 32 suites)
Controller adjudications T6: empty batch creates NO run dir (consistent with empty-batch-silent-success decision; T8 history simply sees no entry); single run-joblog-incomplete message for both failure flavors accepted (richer error type = v1.x/GUI refinement).
WAVE 2 COMPLETE. Wave 3 next: T7 read-only IPC + T8 run lifecycle, PARALLEL worktrees plan5-t7/plan5-t8 off f54cbab; both append invoke_handler in src-tauri/src/lib.rs - trivial merge, order T7 then T8.
Whole-branch-review notes from T7 review: (a) atomic-write hardening (write-temp-then-rename) for settings.rs save AND joblog writes together - inherited non-atomic pattern, joint pass candidate; (b) controller-approved additive core API: Mkvmerge caches validated version pair from detect/enforce_floor (kills the double --version spawn on GUI startup) - plan-sketch shortcoming, flag to Şenol in wave summary.
Task 7: complete (branch plan5-t7, commits f54cbab..54ff874 incl. fix 54ff874: core version-pair cache (controller-approved additive API), spawn_blocking settings read, Some(true)+RuntimeError-mapping tests; re-review Approved; MERGED, six gates green on master)
Whole-branch notes from T7 fix round: counting_fake_mkvmerge/spawn_count now mirrored in 2 places (3rd mirrored helper - shared test-support crate starts to earn itself); detect(None) PATH rung still double-spawns (locate+enforce_floor, pre-existing); save_settings lacks the load_settings_from symmetry.
Task 8: complete (branch plan5-t8, commits f54cbab..e7cb673: base + review fixes 20abae1 (RAII reservation, lock-free planning, cancel-during-planning honored, ':' rejected, event-ordering doc) + D31 e7cb673 (close dialog + quit-after-finished, ftl single-source strings); re-review Approved incl. dialog-button semantics verified against plugin source; reconciliation-MERGED 7ce88db by dedicated subagent, unified IpcError/AppState, 72 gui tests, nothing lost; controller six-gate re-run green)
D31 decision (Şenol 2026-07-10): close-with-active-run = full mkvtoolnix parity (dialog + abort-then-quit); memo amended 8c10386.
T11 carry-forward (from T8 reviews): register run-finished/job-event listeners BEFORE invoking start_run (soft outcomes emit synchronously); do NOT invoke start_run synchronously from the run-finished handler (microsecond run-already-active window); RunMeta/summary.json cannot express joblog_status=incomplete - history view nuance.
Whole-branch notes from T8: runner-thread panic wedges app unclosable post-D31 (drop-guard ~10 lines, low probability); ftl_message wording pinned only for title key; stale-dialog-across-run-boundary edge (defensible); lock().unwrap() poison-recovery absent (carried).
WAVE 3 COMPLETE: T7+T8 merged (dabf26c, 7ce88db), worktrees removed, branches deleted, artifacts salvaged.
Task 9: complete (master, commits 7ce88db..65d7065, review Approved first pass; deviations verified: plugin-os 2.3.2 exact both sides (brief's platform() from api was stale), eslint no-raw-text attributes gap found+fixed empirically; controller six-gate green)
WAVE 4 COMPLETE. Minors carried: locale primary-subtag normalization when locale #2 lands; browse() lacks busy-state; ipc.ts camelCase param names vs Rust signatures unverified for 8 commands - T10/T11 must verify on first real call.
Wave 5 contract (controller-defined, both briefs carry it): T10 owns ALL App.vue edits (startRun emit handling, view switch, pendingRun ref passed as prop); T11 only implements JobsView.vue against prop pending-run: RunRequest { profile: string; source: string | null; output: string | null; jobs: number | null }, emits 'consumed' to clear. Merge order T10 -> T11.
Task 10: complete (branch plan5-t10, commits 65d7065..638eda2 incl. fix 638eda2: client MRU cap + v-show both views; re-review Approved; MERGED, six gates green)
Whole-branch/T12 notes from T10: default_jobs staleness window on Run-click after settings change (fix: re-read settings in emitStartRun or resolve jobs:null against fresh settings T11-side); RECENT_PROFILES_CAP duplicated TS/Rust (mirror comment only guard); resolvedTrackLabel punctuation outside Fluent (Plan-6 locale-formatting revisit); background settings writes fail silently to console.warn.
Controller sign-off: jobs = settings.default_jobs in run payload (spec 8.2 default parallelism).
Task 11: complete (branch plan5-t11, commits 65d7065..66419f5 incl. fix 66419f5: per-action busy flags + self-found capture-before-await fix, filter reset, fs rationale; re-review Approved incl. v-show-compat verification - no JobsView change needed; MERGED clean, six gates green)
New dep signed off: @tauri-apps/plugin-fs + tauri-plugin-fs 2.5.1 exact, scope-minimal (fs:allow-write-text-file + dialog:allow-save only; dialog-save injects path into fs scope = Tauri-owned trust chain).
Load-bearing frontend contract (documented in code): App.vue MUST clear pendingRun to null between runs (watcher fires on reference transition); listeners registered mount-once, ensureListeners cached-promise.
WAVE 5 COMPLETE: T10+T11 merged, worktrees removed, branches deleted, artifacts salvaged. Remaining: T12 (playwright+axe+i18n gate+CI), T13 (whole-branch review, journal, HANDOFF).
Task 12: complete (master, commits 8ea3af3..945ee96 incl. fix 945ee96: e2e type-check wired into test:e2e with RED/GREEN drift probe, i18n parser constraint documented; re-review Approved; controller eight-gate green)
New dev deps signed off: @playwright/test 1.61.1, @axe-core/playwright 4.12.1, @types/node 26.1.1 (exact).
WAVE 6 COMPLETE. ALL TASKS T0-T12 DONE. Remaining: T13 close-out (whole-branch review on fable, fix wave, CI verify, journal, artifacts salvage, HANDOFF, push).
