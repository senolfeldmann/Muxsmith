# Task 6 review verdict: src-tauri shell (Stream E, first)

Base 0b3149a..3d58afe.

### Spec Compliance
- ✅ `run.rs:84` yagni - `ActiveRun` deleted, `RunSlot::Running(Arc<QueueControl>)` direct; all five call sites (`commit` run.rs:400, `abort_and_quit` run.rs:503, `do_cancel_run` run.rs:528, `do_cancel_job` run.rs:548, `running()` test helper run.rs:574) updated.
- ✅ `lib.rs`/`run.rs` dup - one `on_blocking` helper (lib.rs:62-68) replaces all five `spawn_blocking` + `map_err("internal-task-failed")` wrappers (`validate_profile`, `dry_run`, `identify`, `detect_mkvmerge`, `run::start_run`). Hand-traced the type flow for every site (see Code Quality): the flattening behavior is byte-identical to the pre-refactor manual chains in every case, including the one site (`validate_profile`) whose closure returned a bare value rather than a `Result`.
- ✅ `lib.rs:147` yagni - `meets_minimum` field, its doc, the `pair >= MIN_SUPPORTED` computation, the now-dead `MIN_SUPPORTED` import, all three `assert!(info.meets_minimum...)` test assertions, and the `ipc.ts:40` mirror line are all gone. Pre-check evidence (`grep -rn meets_minimum src/`) is in the report.
- ✅ `run.rs:548,557` yagni - `_state: State<AppState>` and its two doc paragraphs dropped from `list_runs`/`get_job_log`. The mandated frontend-invoke grep is documented in the report (both commands, `src/**/*.ts`+`*.vue`, only two invoke sites, neither passes args).
- ✅ `run.rs:453` idiom - `match outcome { Ok(o)=>o, Err(e)=>{drop(reservation); return Err(e);} }` replaced by `let outcome = outcome?;`. Confirmed the redundant-drop reasoning is sound: `reservation` is a local still in scope at the early return, so `?`'s implicit unwind drops it in the same order the explicit `drop()` did.
- ✅ `settings.rs:320` native - single `assert!(path.ends_with("muxsmith/settings.json"))`. `Path::ends_with` is component-wise and Rust's non-verbatim Windows path parser (`is_sep_byte`) treats `/` and `\` as equivalent separators, so the deleted `to_string_lossy()` backslash branch was dead on every platform, not just the ones under test. Verified against known std::path behavior, not re-derived.
- ✅ `run.rs:91-95` doc (seed T4-m1) - `lock_active`'s "Recovery is sound because..." sentence matches the brief's prescribed text verbatim (rendered with rustdoc backticks, no wording drift).
- ✅ Full gate run, foreground, workspace-wide; commit message `refactor(shell): plan-5.6 T6 idiomacy fixes (yagni/dup/idiom/native/doc)` matches the required prefix.
- ⚠️ Cannot verify from diff alone: commit is unsigned (`-c commit.gpgsign=false`) and staged by explicit filename (no `git add -A`). The diff file has no commit-metadata or staging evidence; the report's claimed `git log -1 --format="%G?"` -> `N` is plausible but outside what this diff-only review can check.

Task-scope boundary honored: exactly `lib.rs`, `run.rs`, `settings.rs`, and one deleted line in `ipc.ts` touched; `e2e/smoke.spec.ts` correctly left alone per the brief's explicit file-ownership split, with the resulting red `pnpm test:e2e` gate part documented as a plan-mandated, known, deferred-to-follow-up consequence rather than silently swallowed.

### Strengths
- The `on_blocking` extraction is a genuinely correct dedup, not just a textual merge: the five original sites differed subtly in whether the wrapped closure returned a bare value (`validate_profile`) or already a `Result` (`dry_run`/`identify`/`detect_mkvmerge`/`start_run`'s `plan_run`), and the new helper's `Result<T, IpcError>` bound plus internal `?`-flatten reproduces both shapes without behavior drift. Worth checking by hand rather than trusting the "it compiles and tests pass" signal alone, since a subtly wrong flatten (e.g. swallowing the inner `Err` instead of the outer `JoinError`) would still type-check.
- `meets_minimum` removal is fully swept: field, doc, computation, dead import, three test assertions, and the one `ipc.ts` mirror line, with a self-disclosed gap (the brief's `grep -rn meets_minimum src/` pre-check missing `e2e/`) reported rather than hidden, plus a verified one-line fix for the follow-up task.
- Self-review is concrete and checkable (per-item grep evidence, explicit unstaged-`e2e`-file verification), not just an assertion of completeness.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
- **[plan-mandated]** `pnpm test:e2e` (gate part 9) is red on this branch in isolation, because `e2e/smoke.spec.ts:80` still sets the now-nonexistent `meets_minimum` field. This is the explicitly anticipated, brief-sanctioned consequence of the Stream E file split (task-6 owns `src-tauri`+the one `ipc.ts` line; the e2e fix is the follow-up task's job), not an implementer defect. Flagging per the calibration rule for plan-mandated defects; does not block this task's approval, but the follow-up task must land before merge to `master`/a green whole-branch gate.

#### Minor (Nice to Have)
- `run.rs`'s `start_run`: `let outcome = crate::on_blocking(move || plan_run(...)).await; let outcome = outcome?;` could collapse to one line, `let outcome = crate::on_blocking(...).await?;`, since `on_blocking`'s return type is already the single-level `Result<T, IpcError>` the brief's target snippet unwraps. The two-statement shadowing form is not wrong, and reads as a deliberate literal match to the brief's prescribed `let outcome = outcome?;` line rather than an unrequested extra simplification, but it is available headroom.
- `on_blocking` is called unqualified in `lib.rs` (same module) and as `crate::on_blocking` in `run.rs` (cross-module, no `use` added). Consistent with not touching an unrelated import line, but a `use crate::on_blocking;` in `run.rs` would read slightly cleaner given it is now called twice-ish more in future maintenance.
- `detect_mkvmerge_body_finds_the_real_mkvmerge_when_available`'s assertion was reduced to a bare `.expect("detect")` after the brief's mandated deletion of its `meets_minimum` assert left nothing else to check. Correct per the brief's literal instruction (no invented replacement), but as a byproduct this test now only guards against a panic, not against any behavioral property of the returned `MkvmergeInfo`. Not this task's defect to fix; flagging for whoever next touches `detect_mkvmerge` test coverage.

### House dimension

No deviations from `docs/conventions.yaml` / `docs/process-conventions.yaml` / `docs/product-boundaries.yaml` found; this task is pure internal GUI-shell cleanup (yagni/dup/idiom/native/doc), touches no product-scope or established core/executor pattern, and its per-commit gate discipline matches `ci-06-per-commit-gate` (the one red part is plan-mandated and documented, not silently skipped, consistent with the spirit of that convention rather than a violation of it).

Harvested for the decision ledger:
- **New pattern candidate (not yet promotable, count 1):** a single `on_blocking` helper collapsing the repeated `spawn_blocking` + `map_err("internal-task-failed")` wrapper around every blocking `#[tauri::command]` body, placed in `lib.rs` rather than `error.rs` specifically to keep `error.rs` free of a `tauri` dependency. Domain `gui`, source `agent-emergent` - needs two more independent occurrences (or an owner ruling) before Tier-2 promotion per the source×nature matrix; worth tracking if a similar dedup recurs.
- **Process gap surfaced, candidate refinement to `proc-09-idiomacy-review`:** a yagni field-removal pre-check grep scoped to `src/` missed a real reader in `e2e/smoke.spec.ts`. Recommend any future "verify no real reader before deleting a field/symbol" pre-check spans `src/` and `e2e/` (and any other frontend-adjacent test tree) rather than `src/` alone, so a red downstream gate is anticipated rather than discovered.
- **Repeated rejection reinforced, not new:** the `settings.rs` fix is a second instance of removing platform-branching code that was defensible-looking but dead given std's actual (both-separators-accepted) Windows path parsing - same shape as `core-89-homebrew-apple-silicon-path`'s "verify the platform assumption against the real authority, not the plausible-sounding one" lesson, here in the opposite direction (removing an unneeded branch instead of adding a needed one).

### Assessment
**Task quality:** Approved
**Reasoning:** All seven brief items are implemented correctly and match the prescribed text/behavior exactly where the brief specified it verbatim; the one gate-red item is an explicitly plan-mandated, well-documented consequence of the task's file-ownership split, not a defect in this task's own work.
