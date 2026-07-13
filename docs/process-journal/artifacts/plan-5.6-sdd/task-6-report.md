# Task 6 report: src-tauri shell (Stream E, first)

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-e` (branch `plan-5.6-e`).
Commit: `3d58afe` — `refactor(shell): plan-5.6 T6 idiomacy fixes (yagni/dup/idiom/native/doc)`.

Status: **DONE_WITH_CONCERNS** (see "Concern" below — one out-of-scope
downstream file needs a one-line follow-up fix before `pnpm test:e2e` is
green again).

## Setup

`mise install` (all pinned tools already present) + `pnpm install
--frozen-lockfile` (222 packages, lockfile verified) in the worktree.

## Items implemented

1. **yagni — `ActiveRun` wrapper (`run.rs:84`).** Deleted the single-field
   `struct ActiveRun { ctl: Arc<QueueControl> }`; `RunSlot::Running` now
   carries `Arc<QueueControl>` directly. Updated every match arm: `commit`,
   `abort_and_quit`, `do_cancel_run`, `do_cancel_job`, and the `running()`
   test helper. Updated `RunSlot::Running`'s variant doc (used to point at
   `ActiveRun`'s own doc, now inlines the same rationale since there is no
   longer a separate type to document). Verified no remaining `ActiveRun`
   reference anywhere (`grep -rn ActiveRun src-tauri/ src/` → empty).

2. **dup — `on_blocking` helper (`lib.rs:296-301,312-331,337-349,367-376` +
   `run.rs:447-451`).** Added one
   `pub(crate) async fn on_blocking<T: Send + 'static>(f: impl FnOnce() ->
   Result<T, IpcError> + Send + 'static) -> Result<T, IpcError>` at crate
   root in `lib.rs` (chose `lib.rs` over `error.rs`: `error.rs` currently has
   no Tauri dependency and its module doc frames it as a pure IPC-error
   contract; `lib.rs` already imports `tauri::State`/`#[tauri::command]`, so
   this keeps `error.rs` Tauri-decoupled). All five call sites
   (`validate_profile`, `dry_run`, `identify`, `detect_mkvmerge` in `lib.rs`;
   `start_run`'s planning pass in `run.rs`) now call `on_blocking(...).await`
   with only their own closure body as the difference; the repeated
   `spawn_blocking` + `.map_err("internal-task-failed")` wrapper is gone.

3. **yagni — `meets_minimum` (`lib.rs:147`).**
   Pre-check: `grep -rn meets_minimum src/` → sole hit `src/ipc.ts:40` (the
   mirror), as expected — proceeded with the deletion. Removed the
   `meets_minimum: bool` field from `MkvmergeInfo`, its doc line, the
   `pair >= MIN_SUPPORTED` computation in `detect_mkvmerge_body` (and the
   now-unused `MIN_SUPPORTED` import), the three `assert!(info.meets_minimum
   ...)` test assertions, and the `src/ipc.ts:40` mirror line. Rewrote
   `MkvmergeInfo`'s and `detect_mkvmerge_body`'s doc comments to state the
   new invariant directly (`Mkvmerge::detect` already refuses too-old
   candidates, so every `Ok` clears the minimum by construction) instead of
   describing a field that no longer exists.
   - Minor discretionary follow-through beyond the letter of the brief:
     renamed the test `detect_mkvmerge_body_success_reports_version_and_meets_minimum`
     to `..._and_path` (the old name was now a lie), and left the third test
     (`detect_mkvmerge_body_finds_the_real_mkvmerge_when_available`) with no
     assertion beyond `.expect("detect")` since removing its sole assertion
     left nothing else to check (per the brief's literal instruction — did
     not invent a replacement assertion).
   - **Concern (see below): a real reader outside the pre-check's `src/`
     scope.** `e2e/smoke.spec.ts:80` also has a `meets_minimum: true,` field
     in a `MkvmergeInfo`-typed mock literal, which the brief's grep (scoped
     to `src/`, not `e2e/`) didn't surface. This is out of this task's
     exclusive file list (`e2e/` is a frontend file the brief reserves for
     the follow-up task), so it was **not** touched, per instructions.

4. **yagni — unused `_state` param (`run.rs:548,557`).** Pre-check:
   `grep -rn "list_runs\|get_job_log" src/ --include="*.ts" --include="*.vue"`
   confirmed the only two frontend invoke sites
   (`invoke<RunMeta[]>("list_runs")`, `invoke<JobLogRecord>("get_job_log",
   { runId, index })`) pass no state-shaped args. Dropped `_state:
   State<AppState>` from both `list_runs` and `get_job_log`, and removed
   the two doc paragraphs explaining the parameter's unusedness.

5. **idiom — `let outcome = outcome?;` (`run.rs:453`).** Replaced the
   `match outcome { Ok(o) => o, Err(e) => { drop(reservation); return
   Err(e); } }` block with `let outcome = outcome?;`; `Drop` on
   `Reservation` fires on the early return exactly as the `on_blocking`
   call two lines above already relies on for its own early return.

6. **native — `settings.rs:320`.** Replaced
   `path.ends_with("muxsmith/settings.json") || path.to_string_lossy()
   .ends_with("muxsmith\\settings.json")` with a single
   `assert!(path.ends_with("muxsmith/settings.json"))`: `Path::ends_with`
   only matches whole path components, and `std::path`'s Windows component
   parser accepts `/` as a separator too, so the backslash fallback was
   dead code on every platform (Linux/macOS/Windows).

7. **doc (seed T4-m1) — `lock_active` (`run.rs:91-95`).** Rewrote the
   "Recovery is sound because ..." sentence to the exact text specified in
   the brief (slot is only ever replaced wholesale by a single
   non-panicking assignment; the remaining critical sections only read the
   slot and call into its contents, never mutate it in place; so even a
   panic while the guard is held, e.g. inside `cancel_all`, cannot leave a
   half-applied `Option<RunSlot>`).

## Gate results (nine parts, workspace-wide, foreground)

| # | Part | Result |
|---|------|--------|
| 1 | `cargo fmt --all --check` | clean |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| 3 | `cargo test --workspace` | 36 test-result blocks, all `ok`, 0 failed (exit 0); `muxsmith-gui` unit suite: 78 passed, 0 failed |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean, no warnings |
| 5 | `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` (exit 0); pre-existing `winnow` multi-version banner is unrelated noise, not new |
| 6 | `pnpm lint` | clean |
| 7 | `pnpm build` (vue-tsc + vite) | clean, type-checks and builds |
| 8 | `pnpm check:i18n` | `ok` (12 pre-existing "unused catalog key" warnings, unrelated to this task) |
| 9 | `pnpm test:e2e` | **fails to type-check** — `e2e/smoke.spec.ts:80` still has `meets_minimum: true,` in a `MkvmergeInfo`-typed literal |

Parts 1-8: green. Part 9 fails purely because of the out-of-scope
`e2e/smoke.spec.ts` mock (see Concern). Verified the diagnosis by
temporarily deleting that one line locally, running `pnpm test:e2e` (all 6
Playwright specs passed), then reverting via `git checkout --
e2e/smoke.spec.ts` before committing — confirmed via `git status
--porcelain e2e/smoke.spec.ts` that it is back to the pristine, unmodified,
un-staged state. No other file in the repo references `MkvmergeInfo`
(`grep -rln MkvmergeInfo --include="*.ts" --include="*.vue" .` → only
`src/ipc.ts` and `e2e/smoke.spec.ts`).

## Concern: e2e/smoke.spec.ts needs a one-line follow-up fix

The brief's mandatory pre-check for the `meets_minimum` item
(`grep -rn meets_minimum src/`) is scoped to `src/`, which does not cover
`e2e/`. `e2e/smoke.spec.ts:77-81` builds a `MKVMERGE_INFO: MkvmergeInfo`
mock literal that still sets `meets_minimum: true,`; with the field gone
from the `MkvmergeInfo` interface, `tsc --noEmit -p e2e/tsconfig.json`
(the first step of `pnpm test:e2e`) now fails with "Object literal may only
specify known properties, and 'meets_minimum' does not exist in type
'MkvmergeInfo'."

This task's brief explicitly scopes ownership to `src-tauri/src/{run,lib,
settings}.rs` plus exactly one line in `src/ipc.ts`, and says "Do not touch
any other frontend file — a follow-up task in your stream handles the
frontend after you." `e2e/smoke.spec.ts` is such a frontend file, so it was
deliberately left untouched. The fix for the follow-up task is a one-line
deletion, verified working:

```diff
 const MKVMERGE_INFO: MkvmergeInfo = {
   path: "/usr/bin/mkvmerge",
   version: "90.0.0",
-  meets_minimum: true,
 };
```

Until that lands, `pnpm test:e2e` (gate part 9) will fail on this branch in
isolation; every other gate part is green, and the Rust-side change plus
its `src/ipc.ts` mirror are complete and correct per the brief.

## Files changed

- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-e/src-tauri/src/run.rs`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-e/src-tauri/src/lib.rs`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-e/src-tauri/src/settings.rs`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-e/src/ipc.ts`

## Self-review

- Diff scoped to exactly the four intended files (`git diff --stat`
  confirms); no other file touched or left dirty.
- All 7 brief items implemented; re-verified every anchor line against the
  current file before editing (line numbers had drifted slightly from the
  brief's snapshot after the `ActiveRun` removal, re-read via `grep`/`Read`
  each time rather than trusting stale offsets).
- Zero behavior change: every test that existed before this change still
  exists and passes (78/78 in `muxsmith-gui`, no test deleted, three
  assertions removed exactly as directed, one test renamed for honesty
  after its assertion was removed).
- Commit is unsigned (`git -c commit.gpgsign=false commit`; verified via
  `git log -1 --format="%G?"` → `N`), staged explicitly by filename (no
  `git add -A`), not pushed.

## Patterns/deviations surfaced for the house ledger

- **New pattern, not yet in `docs/conventions.yaml`:** a single
  `on_blocking` dedup helper for the repeated `spawn_blocking` +
  `map_err("internal-task-failed")` wrapper around every blocking
  `#[tauri::command]` body. Checked `docs/conventions.yaml` and
  `docs/process-conventions.yaml` beforehand for an existing convention
  covering this shape (grepped for `spawn_blocking`, `on_blocking`,
  `IpcError`, `State<AppState>` in a house-convention context) — none
  exists; this is new house knowledge from this idiomacy wave, not a
  restated one. Placed in `lib.rs` rather than `error.rs` specifically to
  keep `error.rs` free of a `tauri` dependency (its module doc frames it as
  a pure error-shape contract); flagging this placement choice in case a
  future task wants it formalized as a house pattern.
- **Minor discretionary cleanup beyond the brief's literal text:** renamed
  one test function (`..._and_meets_minimum` → `..._and_path`) since the
  brief's instruction ("delete the assert") would otherwise leave a test
  name asserting something the test body no longer checks. Not a new
  pattern, just flagging the small deviation from "mechanical, don't
  improvise."
- **Gap in the brief's `meets_minimum` pre-check, not a deviation on my
  part:** the mandated grep was scoped to `src/`, missing the real reader
  in `e2e/smoke.spec.ts`. Recommend the pre-check's scope note be widened
  to `grep -rn meets_minimum src/ e2e/` for any future similar removal, so
  the frontend follow-up task's owner knows up front rather than
  discovering it via a red `test:e2e` run.
