# Plan 4 SDD progress ledger

Plan: docs/superpowers/plans/2026-07-09-plan-4-executor-run-queue.md (11 tasks, 7 waves)
Started: 2026-07-09. Controller: Peter (Fable 5). Wave 1 (T1, T4, T5, T6, T7) runs as
parallel worktree streams under .worktrees/plan4-tN; merge order T5 -> T7 -> T4 -> T6 -> T1,
full gate re-run per merge. Base for all wave-1 branches recorded below.

## Status

Task 6: complete (branch plan4-t6, commits 1b55d98..5aefde1, review clean)
Task 4: complete (branch plan4-t4, commits 1b55d98..d654585, review clean, 1 Minor below)
Task 7: complete (branch plan4-t7, commits 1b55d98..f527475, review clean; reviewer's two Minors self-assessed as convention parity, no action)

Task 1: complete (branch plan4-t1, commits 1b55d98..3e1ebf3, review clean; latent constraints for T2/T3 below)
Task 5: complete (branch plan4-t5, commits 1b55d98..65eef3c, review clean; two report-accuracy nits, no code defects)
Wave 1 MERGED to master: 4dfd3a7 (T5), 35c95e5 (T7), 283a089 (T4), 0816ea2 (T6), bde442c (T1); full gate green after every merge, final 220 passed 0 failed. Worktrees removed, branches deleted, artifacts salvaged to main .superpowers/sdd/.
Dependent chain runs serially on master in the main tree: T2 (base bde442c) -> T3 -> T8 -> T9 -> T10 -> T11.
Task 2: complete (commits bde442c..f394f61 incl. Important-fix f394f61, re-review Approved; controller gate re-run green, 227 passed)
Task 3: complete (commits f394f61..b9960c5, review Approved, all 5 named risks pass; controller gate re-run green, 233 passed)
Task 8: complete (commits b9960c5..2478520, review Approved, all 5 named risks + both judgment calls pass; controller gate re-run green, 249 passed)
RESOLVED (Şenol 2026-07-10: Fluent plural form): plural fix landed as 79f0447 incl. i18n.rs msg_with_count (numeric FluentValue; $count was a string, [one] never matched) + memo amendment line. TDD RED/GREEN, controller gate green (256 passed). Covered by final whole-branch review, no task-review loop of its own.
Task 9: complete (commits 2478520..77317a0, review Approved; keep-mode expectation fix verified spec-correct against planner.rs:642-648 + spec 4.8)
Task 10: complete (commits 3f66a4e..a7c76ae, review Approved, all 4 named risks verified incl. ctrlc source; controller gate green, 262 passed)
Task 11: complete (commits a7c76ae..93d1a6b, review Approved, all 4 named risks traced to source; final controller gate green, 264 passed)
ALL 11 PLAN TASKS + 2 fixes complete and task-reviewed. Whole-branch review (fable, 7aec492..93d1a6b, 20 commits): READY TO PUSH with 2 Important findings; independent gate re-verification by reviewer green (264, 0 gated skips).
Final fix wave dispatched (4 commits): Windows kill->Warning mapping (killed flag on LiveJob, D16/D17); mkvmerge_found asserted on never-checked paths (field only when lookup ran); stale exit-code doc cli.rs (add 130); cap queue workers at spec count.

## v1.x backlog from final-review triage (carry into HANDOFF)

- delete_partial: report failed removes via outcome.errors (channel exists since spawn-error fix).
- Warning-path output-kept assertion + WarningLine/ErrorLine on_progress test.
- Cancellation narrowing: run_job checks cancel immediately before spawn -> Cancelled without spawning (closes most of the register-after-sweep window without touching the one-shot watcher).
- JobEvent serde golden test when Plan 5's Tauri consumer lands (that is when the shape becomes load-bearing).
- fail-fast-with-failing-job-not-first test.
- ctrlc full-version pin next time Cargo.toml is touched.
- read_line Err=EOF: read_until + from_utf8_lossy if non-UTF-8 output ever matters.
- Empty-batch human output is silent success (spec-level gap, D15 silent; Şenol decision for v1.x).
- CLI test-helper duplication (have_mkvmerge/muxsmith x3): consolidate when next test file appears.
- ConcurrencyTracker is public API surface (pure test instrumentation): #[doc(hidden)] or relocate before go-public.
- jobs[].index indexes the queue not files: doc sentence or source field before Plan 5 consumes it.
- Windows kill-mapping fix (fix wave) must be verified before enabling the 3-OS matrix / go-public claim on Windows cancellation.
Fast-follow COMPLETE (3f66a4e): json document now emitted on profile-load and list-languages failures in both run and dry-run; 4 bug-reproducing tests RED->GREEN incl. real fake-mkvmerge stub for the list-languages arm (#[cfg(unix)]); controller gate green, 262 passed. Covered by final whole-branch review.

## Constraints for T2/T3 dispatches (from T1 review)

- run_job MUST drain next_line() to EOF before calling wait(): LiveJob::wait() holds the child mutex across blocking waitpid, so a Killer invoked during wait() stalls until natural exit. Never wait() on a live process in one thread while relying on Killer from another.
- Spawn trait carries no Send+Sync supertrait; the queue adds the bound at the use site (plan's run_queue already types `&(dyn Spawn + Sync)`).
- read_line Err(_) treated as EOF is plan-mandated; known latent robustness gap, do not "fix" ad hoc in T2/T3.
- Observed v100 gui-mode grammar (T1 report, doc comment spawn.rs:75-90): progress `#GUI#progress NN%` (final always 100%), warning `#GUI#warning '<file>': <message>` (exit 1), error `#GUI#error <message>` (exit 2, no leading filename). T2's parser encodes THESE.

## For HANDOFF refresh at close-out

- SI-4 update (Şenol 2026-07-09): agent commits deliberately UNSIGNED as policy (signature = his authorship claim), not merely because GPG blocks; gpgsign=false on every agent commit AND merge even when signing would succeed. Peter memory project_muxsmith already updated.

## Minor findings carried to final review

- T4: tests/dry_run_cli.rs default-branch case asserts only exit 2, not the output-collision diagnostic's error severity (skip branch does assert severity).
- T2: delete_partial ignores ALL io errors, not just NotFound (job.rs:164-166); failed remove leaves broken partial that on_collision=skip would accept. No JobOutcome channel to report it; plan-level question.
- T2: percent parser passes 101-255 unclamped (job.rs:133-137); real grammar caps at 100, robustness nit, matches brief's literal parser spec.
- T2: exit-1 warning path lacks output-kept assertion; WarningLine/ErrorLine surfacing via on_progress untested.
- T3: best-effort cancellation window (plan-mandated one-shot watcher): a job registering its killer after the sweep runs to natural completion under cancel, recorded Ok/Warning/Failed not Cancelled; bounded to one job per worker. Design note for Şenol if stronger cancel ever wanted (loop-and-re-sweep watcher).
- T3: worker panic swallowed (queue.rs:171 let _ = handle.join()); slot backfills Cancelled silently, poisoned mutex would panic at into_inner. Defensive-robustness only.
- T3: ConcurrencyTracker::exit underflows if wait() called twice (test-helper fragility).
- T3: coverage gaps: fail-fast with failing job not first; JobEvent serde shape (T8's renderer covers).
- T8: catalog_completeness only guards DiagCode keys, NOT general cli.ftl run-* keys; the run-* keys are protected by milestone unit tests asserting rendered text. A completeness test over all cli.ftl keys would close the future-keys gap (out of T8 scope).
- T9-fastfollow: config_only_json always sets mkvmerge_found:false, imprecise on the list-languages-failure path (found but broken) and profile-load path (never checked); schema reuse kept per scope. Candidate for a schema nuance later.
- T9: stray trailing comma formatting nit at run.rs:78 call site.
- T10: ctrlc = "3" pins major only; sibling deps pin full patch versions. Style inconsistency, brief-literal.
