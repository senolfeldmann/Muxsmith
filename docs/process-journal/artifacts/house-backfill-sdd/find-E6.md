# E6 (Plan 5, GUI run path, 2026-07-10) — decision-history reconstruction

Era E6 = Plan 5 "Tauri 2 GUI run path" plus the go-public tail (session 6).
Sources: design memo D22-D31 (`specs/2026-07-10-plan-5-gui-design-decisions.md`),
plan (`plans/2026-07-10-plan-5-gui-run-path.md`), the 12 task verdicts +
whole-branch review (rounds 1-2) under `plan-5-sdd/verdicts/`, the Plan-5 final
fix wave (`plan-5-sdd/final-fix-wave-plan5-report.md`), and the journal entries
`2026-07-10 | Plan 5 complete + go-public`, the 3-OS addendum, and the session-6
close. One record per (topic, approach) occurrence; recurrences across the trail
are kept as separate records, not pre-merged.

Legend: kind = pattern (adopted) / restraint (rejected, steelman = case for the
loser) / non-decision (deferred, blocked_on). occ_kind = decided / reinforced /
violated-corrected / deferred.

---

## Core / executor

1. **Raw output-line JobEvent variant (D24)** — pattern / decided / executor.
   `JobEvent`/`JobProgress` gain an additive raw `Output{index,line}` variant;
   every non-`#GUI#progress` line emitted verbatim, tagged warn/error lines
   also keep their stripped variant; progress ticks not persisted. occ_ref: memo D24.
   Evidence: "It carries every line mkvmerge writes that is not a `#GUI#progress` tick... The raw stream feeds the job-queue view's live log pane and the persisted job log."

2. **Full output into JobOutcome instead of a stream event** — restraint / decided / executor.
   Rejected: accumulating full output into `JobOutcome`. steelman: keeps all
   output in one already-returned struct, no new event variant. Losing because
   it "bloats the in-memory outcome vector for large batches and duplicates what
   the incremental log writer already persisted." occ_ref: memo D24 (alternative rejected).

3. **JobEvent serde golden test (wire shape = contract)** — pattern / decided / testing.
   A golden test pins tag/variant/field names because the GUI now consumes the
   stream. occ_ref: plan T1 / memo D29. Evidence (T1 verdict): "golden test asserts `{\"event\":\"output\",\"index\":0,...}` verbatim... pins the concrete required behavior."

4. **JobEvent wire contract layered (golden ↔ ipc.ts ↔ e2e fixtures)** — pattern / reinforced / cross.
   occ_ref: whole-branch verdict (strengths). Evidence: "the JobEvent serde golden test, src/ipc.ts mirroring Rust structs field-for-field, and the e2e fixtures typed satisfies JobEvent give three layers that fail loudly on drift."

5. **Per-job cancel lands in core (D25)** — pattern / decided / executor.
   Kill-by-index via registered Killer + queued-skip set; skipped jobs become
   Cancelled. occ_ref: memo D25 (confirmed Şenol). Evidence: "per job cancel in core needs to be in core now."

6. **Batch-cancel-only, per-job deferred** — restraint / decided / executor.
   Rejected. steelman: contains Plan 5 scope, per-job cancel is UI-only sugar
   deferrable to a later plan. Losing: "a deviation from spec 8.2 that Şenol declined." occ_ref: memo D25 (alternative rejected).

7. **Lost-cancellation race in the mid-spawn window (D25)** — violated-corrected / executor.
   `cancel_job` between the pre-spawn check and killer registration silently
   dropped the cancel with no error/trace. Fixed by a post-insert re-check
   (commit e06bda0). occ_ref: task-5-review-verdict.md (Important). Evidence: "the explicit cancel request is dropped with no error and no observable trace... this task's own review brief calls out by name."

8. **Mid-spawn race closure verified exemplary** — pattern / reinforced / executor.
   occ_ref: whole-branch verdict (strengths). Evidence: "the test drives the exact window deterministically with a gate spawner instead of sleeps (queue.rs cancel_job_during_spawn_window_is_not_lost)."

9. **Skip-queued job emits Finished{Cancelled} but NO Started** — pattern / decided / executor.
   Deviation from never-dequeued silence because the GUI needs the confirmation;
   documented in `run_queue` rustdoc. occ_ref: plan T5 step1 / task-5 verdict.
   Evidence: "asserts Started == [0,1] and a Finished{2, Cancelled} event."

10. **Pre-spawn cancel check (HANDOFF backlog closed)** — pattern / decided / executor.
    Cancelled flag set before spawn → Cancelled, spawner never called, nothing
    deleted. occ_ref: plan T5 step1.3.

11. **delete_partial error surfacing (HANDOFF backlog closed)** — pattern / decided / executor.
    A failing partial delete pushes `"delete_partial_failed: <io error>"` into
    `outcome.errors` (third-party-detail passthrough exception). occ_ref: plan T5 step1.4 / task-5 verdict (spec compliance).

12. **delete_partial boundary reaffirmed (only jobs that actually ran)** — pattern / reinforced / executor.
    D17 applies only to ran processes; spawn-failure/skip deletes nothing.
    occ_ref: task-5-review-verdict.md (D17 boundary respected). Evidence: "Pre-spawn cancel constructs the outcome directly, bypassing finish/delete_partial entirely."

13. **Job logs = JSON per job + batch summary, written by core for BOTH surfaces (D26)** — pattern / decided / executor.
    `executor::joblog` writes `runs/<run-id>/{summary.json, job-<index>.json}`;
    CLI run and GUI run both persist unconditionally; dry-runs persist nothing.
    occ_ref: memo D26 (format+scope Şenol). Evidence: spec 6 "phrases persistence as a job-engine property, so GUI-only persistence would diverge from the spec."

14. **Plain-text log + index / GUI-only / single NDJSON journal** — restraint / decided / executor.
    Rejected three alternatives. steelman: NDJSON-per-run is one append-only
    artifact and matches a future event stream. Losing: two-artifact consistency
    burden, spec divergence / CLI loses post-mortem logs, and "NDJSON events are
    deferred to v1.x anyway." occ_ref: memo D26 (alternatives rejected).

15. **No run-log pruning in v1** — non-decision / deferred / executor.
    Location documented, a `prune` facility is a v1.x candidate. occ_ref: memo D26.
    blocked_on: external (product / v1.x). Evidence: "No pruning in v1: the location is documented; a prune facility is a v1.x candidate."

16. **Silent mid-run joblog write loss → false 'logs written'** — violated-corrected / executor.
    A `job-<index>.json` write failure left no trace and `finish()` still
    returned Ok, printing a false success. Fixed by tracking `had_write_error`
    and reflecting it in `finish` (commit f54cbab). occ_ref: task-6-review-verdict.md (Important). Evidence: "a false-positive success message despite silent, partial data loss in exactly the artifact D26 exists for."

17. **MUXSMITH_RUNS_ROOT env override gated to debug builds** — violated-corrected / executor.
    Shipped as a permanent, undocumented release-build knob; corrected to
    debug-only (commit f54cbab). occ_ref: task-6-review-verdict.md (Minor 2). Evidence: "a permanent, undocumented env-var surface read unconditionally in the shipped binary... worth gating behind #[cfg(debug_assertions)]."

18. **Hoist batch/config/run JSON assembly into core::report (spec 7 DRY)** — pattern / decided / core.
    `report::json::{config_only,batch,run}_document` lifted 1:1 from the CLI so
    CLI and GUI render byte-identical structures; neither owns logic. occ_ref: plan T2 / spec 7. Evidence (T2 verdict): "character-identical except for the renamed function/param... byte-identical output claim holds structurally."

19. **DiagnosticRenderer port/adapter keeps core prose-free** — pattern / decided / core.
    Core reserves a `"rendered"` slot filled by an injected renderer; core never
    originates or localizes the text. occ_ref: task-2-review-verdict.md (design deviation). Evidence: "the standard hexagonal port/adapter shape... core never originates, hardcodes, or has i18n knowledge of that text."

## Capability / mkvmerge detection

20. **Detection ladder + per-OS first-run guidance (D28)** — pattern / decided / core.
    `detect_mkvmerge` probes override → PATH → platform-standard locations; on
    failure a per-OS guidance screen with a manual picker; min version floor
    enforced with a clear error. occ_ref: memo D28. Evidence: "parked from Plan 2 precisely for the GUI; the CLI's existing behavior is unchanged."

21. **Override-authoritative (early return, no silent fallthrough)** — pattern / decided / core.
    Override arm hard-fails rather than masking a config error by falling to PATH.
    occ_ref: task-3-review-verdict.md (strengths). Evidence: "config errors shouldn't be silently masked by an automatic fallthrough."

22. **MIN_SUPPORTED floor fixed empirically (86,0)** — pattern / decided / core.
    Derived from the schema v19→v20 diff traced to NEWS.md v86.0, evidence in the
    const doc comment; independently re-derived by the reviewer. occ_ref: plan T3 step1 / task-3 verdict / commit c7ef52a. Evidence: "the inductive step (v86.0 is therefore the release that moved 19->20) holds."

23. **Platform candidates verified against packaging, not memory** — pattern / decided / core.
    Each Windows/macOS/Linux path checked against mkvtoolnix's own NSIS installer /
    macOS bundle / deb-rpm packaging and cited; unused paths dropped (the brief's
    macOS glob guess was corrected). occ_ref: plan T3 step2 / task-3 verdict. Evidence: "packaging/macos/config.sh (APP_BUNDLE_NAME=\"MKVToolNix.app\", no version suffix — correctly contradicts the brief's glob guess)."

24. **Homebrew Apple-Silicon path deliberately not added at T3** — non-decision / deferred / core.
    `/opt/homebrew/bin` dropped because no Homebrew formula lives in the mkvtoolnix
    source (outside SI-3's evidentiary scope); flagged forward to T7. occ_ref: task-3-review-verdict.md (Minor). blocked_on: internal (T7 product-scoping). Evidence: "a product-completeness question the mkvtoolnix source tree genuinely cannot answer."

25. **Homebrew Apple-Silicon candidate added (Finder apps don't inherit PATH)** — violated-corrected / core.
    The exclusion rested on the wrong authority; a Finder-launched Tauri app does
    not inherit the shell PATH, so `/opt/homebrew` is a real gap on the common
    macOS install route. Added (commit 5e76a15), Homebrew docs cited. occ_ref: whole-branch verdict (triage-14, FIX-NOW). Evidence: "GUI apps launched from Finder do not inherit the shell PATH... detection fails for the most common macOS install route."

## GUI shell / frontend

26. **Vue 3 + TypeScript over React (D27)** — pattern / decided / gui.
    Composition API, `<script setup>`, plain `ref`/`reactive` + provide/inject, no
    Pinia, no component library at this scale, fluent-vue over @fluent/bundle.
    occ_ref: memo D27 (React vetoed, spec §7 amended). steelman n/a (adopted).
    Evidence: "Vue is in Şenol's stack, SFC templates avoid JSX, ecosystem comfortably covers Plan 6's editor."

27. **React / Svelte 5 / SolidJS / Leptos-WASM rejected** — restraint / decided / gui.
    steelman: React is the most-staffed ecosystem; Leptos keeps one Rust language
    end-to-end. Losing: "nothing in the GUI is React-specific" (author veto),
    Svelte = new framework while shipping + weaker raw-text linting, Solid still
    JSX, Leptos slow edit loop + thinnest Plan-6 ecosystem + hand-built lint/a11y.
    occ_ref: memo D27 (alternatives rejected 2026-07-10).

28. **No-hardcoded-strings CI gate via template raw-text lint (D27)** — pattern / decided / i18n.
    `@intlify/eslint-plugin-vue-i18n` `no-raw-text`, verified to fire without the
    vue-i18n runtime, else a custom check; covers `aria-label` too. occ_ref: memo D27 / plan T4 step2. Evidence: "the exact rule is verified at plan time - it must fire on bare template text without requiring vue-i18n as the runtime library."

29. **no-raw-text lint reinforced (workarounds relocate non-prose only)** — pattern / reinforced / i18n.
    occ_ref: task-9/task-10 verdicts. Evidence (T10): "both cited workarounds genuinely relocate non-prose data rather than smuggling real user-facing copy past the lint."

30. **App settings in platform config dir, never in user YAML (D27)** — pattern / decided / gui.
    mkvmerge override, default jobs, locale, recents (MRU cap 10), per-profile
    source/output memory keyed by profile path — never written into the user's
    profile YAML (mutation is Plan 6). occ_ref: memo D27 / plan T7. Evidence: "app-side memory delivers that without mutating user files."

31. **Client-side recents list never capped (diverges from server MRU cap)** — violated-corrected / gui.
    `updateSettings` assigned the pre-truncation array to reactive state, growing
    the rendered recents past 10 within a session. Fixed (commit 638eda2). occ_ref: task-10-review-verdict.md (Important). Evidence: "a real, demonstrable divergence from the D27 MRU cap, in exactly the axis flagged for scrutiny."

32. **Thin event-forwarding shell, no logic (D23)** — pattern / decided / gui.
    `start_run` spawns `run_queue` on a std thread (the CLI's drain pattern),
    re-emits each JobEvent as a Tauri window event; cancellation via a shared
    `Arc<AtomicBool>` behind `cancel_run`. occ_ref: memo D23. Evidence: "commands + job event stream, no logic; zero new concurrency machinery; JobEvent is Serialize and was designed for this consumer (D13)."

33. **Sidecar CLI (--json) / async-tokio bridge rejected** — restraint / decided / gui.
    steelman: a sidecar reuses the CLI as-is; a tokio bridge gives idiomatic async
    commands. Losing: "batch JSON arrives only at the end; live progress would need
    the NDJSON stream that is deferred," and "core is deliberately sync/mpsc; an
    adapter runtime is unearned complexity." occ_ref: memo D23 (alternatives rejected).

34. **Exactly one run at a time (D23)** — pattern / decided / gui.
    Enforced in Tauri managed state; a second `start_run` rejected with a
    diagnostic, UI additionally disables Run. Mirrors mkvtoolnix-gui's single queue
    executor. occ_ref: memo D23. Evidence: "Single-run mirrors mkvtoolnix-gui's one queue executor (Model::startNextAutoJob)."

35. **Frontend performs zero semantic validation (D23)** — pattern / decided / gui.
    Every profile check round-trips through `validate_profile`. occ_ref: memo D23 / spec 7.

36. **D23 'UI disables Run while active' was not implemented; double-Run destructive** — violated-corrected / gui.
    Clicking Run during an active run made JobsView wipe the live run's state
    before `start_run` rejected, leaving it invisible/uncancellable. Fixed:
    BatchView gates Run on `runActive`, reset made conditional (commits a32b159,
    d9ad2fb-wave). occ_ref: whole-branch verdict (Important #3). Evidence: "D23's own sentence... was simply not implemented."

37. **start_run resolved mkvmerge PATH-only, ignoring the settings override** — violated-corrected / gui.
    Cross-task drift: T8 mirrored the CLI verbatim and missed T7's `Mkvmerge::detect`
    substitution; Windows/override users passed detect+dry-run then every real run
    soft-failed. Fixed to `detect(override)` (commit d9ad2fb). occ_ref: whole-branch verdict (Critical #1). Evidence: "the exact cross-task drift a task-scoped review cannot see... the primary Windows run path is broken end-to-end." (journal: "the single strongest argument this session for the final cross-cutting review.")

38. **Sync start_run froze the event loop during planning** — violated-corrected / gui.
    Planning ran on the main thread, freezing the window and making the tested
    cancel-during-planning paths unreachable from production UI. Fixed: `async fn`
    + planning inside `spawn_blocking` (commit d9ad2fb). occ_ref: whole-branch verdict (Important #2). Evidence: "the carefully tested cancel-during-planning paths are unreachable from the production UI."

39. **Long-running commands async + spawn_blocking; sync stay off the main thread** — pattern / decided / gui.
    `dry_run`/`identify`/`detect_mkvmerge` are async+spawn_blocking (Tauri v2:
    non-async commands run on the main thread), verified against Tauri docs.
    occ_ref: task-7-review-verdict.md (judgment call 3). Evidence: "Commands without the async keyword run on the main thread... The deviation is real, correctly reasoned, and documented."

40. **Window close with an active run = full mkvtoolnix parity (D31)** — pattern / decided / gui.
    Close is prevented, a confirm dialog asks whether to abort; on Yes it issues
    `cancel_all` and quits only after the runner thread finishes (kills landed,
    joblog written); on No it stays open. Supersedes D23's bare cancel_all.
    occ_ref: memo D31 (amendment, Şenol). Evidence: mkvtoolnix-gui "IGNORES the close event, aborts each running job with setQuitAfterFinished(true), and the app quits itself after the abort completes."

41. **Abort-without-dialog / keep-immediate-close rejected (D31)** — restraint / decided / gui.
    steelman: immediate close is the simplest, most responsive behavior. Losing:
    "accidental close kills a batch silently" / "immediate exit races the 50ms
    cancel poll — an orphaned mkvmerge can keep writing... summary.json may never
    be written." occ_ref: memo D31 (alternatives rejected).

42. **D31 origin: T8 flagged the orphaned-mkvmerge / lost-summary risk** — non-decision / deferred / gui.
    The plan-mandated bare `cancel_all` "never blocks the close" risked an orphaned
    child and a truncated joblog; the reviewer flagged it for the controller to
    decide v1 acceptance, which escalated to Şenol → D31. occ_ref: task-8-review-verdict.md (Important #4). blocked_on: external (Şenol product decision "what does mkvtoolnix do"). Evidence: "flagging for the controller to decide whether v1 accepts this risk."

43. **D31 dialog-suppression preference → v1.x** — non-decision / deferred / gui.
    mkvtoolnix's `m_warnBeforeAbortingJobs` is a v1.x settings candidate, not v1
    scope. occ_ref: memo D31. blocked_on: external (product / v1.x).

44. **Runner-thread panic wedged the app unclosable post-D31** — violated-corrected / gui.
    A queue-worker panic left the slot Running so `close_decision` forever answered
    ConfirmAbort and summary.json was lost; fixed with a `TeardownGuard` RAII drop
    (commit 8e3243c). occ_ref: whole-branch verdict (Important #4, promoted from triage). Evidence: "only SIGKILL ends the process, and summary.json is lost."

45. **Torn settings.json bricks the GUI (no in-app recovery)** — violated-corrected / gui.
    Plain `fs::write` on every profile pick; a torn write sent App→FirstRun whose
    own recovery re-read the same broken file — a closed loop. Fixed:
    write-temp-then-rename (commit 5c504c1). occ_ref: whole-branch verdict (Important #5). Evidence: "a closed loop with no exit short of manually deleting a hidden file."

46. **fs+dialog plugin combo for save-as over a bespoke write command** — pattern / decided / gui.
    The dialog plugin injects the chosen path into the fs scope, so the official
    combo is safer than reimplementing the "path came from a real save dialog"
    trust chain. occ_ref: task-11-review-verdict.md (strengths). Evidence: "the plugin route gets it from Tauri's own IPC scope layer for free."

## mkvtoolnix parity (D30)

47. **No user-assembled, reorderable, persistent job queue** — restraint / decided / gui.
    steelman: mkvtoolnix-gui lets the user build and reorder a queue job by job.
    Losing: Muxsmith derives the whole batch declaratively from profile + source
    dir at run time, so "a queue outside a running batch has no Muxsmith meaning";
    the Jobs view shows the live run + history instead. occ_ref: memo D30 (justified divergence).

48. **No auto 'remove completed jobs after N days' in v1** — restraint / decided / gui.
    steelman: mkvtoolnix-gui ships this setting. Losing: "Muxsmith v1 keeps all run
    logs, pruning is v1.x (D26)." occ_ref: memo D30 (justified divergence).

49. **Log export/copy from history — parity gap closed in Plan 5** — pattern / decided / gui.
    mkvtoolnix-gui can open a finished job's log as text; Muxsmith's Jobs view
    includes copy/save-as export. occ_ref: memo D30 (genuine gap, closed). Evidence (T11 verdict): "exceeds the D30 'open as text' parity bar, not just meets it."

50. **mkvtoolnix as parity oracle, read from source (SI-3)** — pattern / reinforced / process.
    Only muxing semantics/output are parity targets (declarative-batch vs
    interactive); claims verified by reading mkvtoolnix's own source. occ_ref: memo D30/D31 (parity evidence, SI-3). Evidence: "MainWindow::beforeCloseCheckRunningJobs... main_window.cpp:492-548" cited directly.

## Testing / CI

51. **Accessibility + test-attribute convention (D29)** — pattern / decided / gui.
    Semantic HTML first; localized accessible names from Fluent (incl. aria);
    live regions (`role="log"`, `role="progressbar"`/native `<progress>`);
    Playwright `getByRole` primary, `data-testid` fallback, locale pinned `en`.
    occ_ref: memo D29 (Şenol directive). Evidence: "every element properly identifiable for testing AND accessibility."

52. **GUI tests stay shallow; logic lives in core (D29)** — pattern / decided / testing.
    Thin Playwright smoke via `mockIPC` (no tauri-driver), shell command tests via
    FakeSpawner, i18n completeness + axe a11y. occ_ref: memo D29 / spec 10. Evidence: "spec 10 keeps GUI tests shallow because logic lives in core."

53. **Packaging CI deferred until go-public** — non-decision / deferred / ci.
    msi/dmg/deb/rpm/AppImage on release tags remain deferred; tags trigger the 3-OS
    matrix + paid minutes while private. occ_ref: memo D29. blocked_on: external (go-public / paid-minutes cost).

54. **Pin-everything (SHA-pin actions, pin runners, pin rust, save-exact)** — pattern / reinforced / ci.
    Rust 1.96.1 over floating stable, all CI actions SHA-pinned, runner images
    pinned, ctrlc full-pin, `.npmrc save-exact`. occ_ref: commits 2ee2d0c, 45e941a / journal Plan 5. Evidence (journal): "pin-everything (rust 1.96.1 over floating stable, all CI actions SHA-pinned, runners pinned...)."

55. **Runtimes via mise, not dnf/corepack/rustup** — violated-corrected / process.
    Controller initially proposed dnf + corepack for node; Şenol corrected to mise
    (repo `mise.toml`). occ_ref: journal Plan 5 (friction) / commits cad84a3→656449c. Evidence: "Controller initially proposed dnf/corepack for node - Şenol corrected to mise (now a Peter memory + repo mise.toml)."

56. **Newest-over-LTS for dev-only runtimes** — violated-corrected / ci.
    Controller's node-24-LTS proposal overturned by Şenol's newest-when-nothing-
    blocks policy; repinned 24 → 26 → 26.5.0 exact, pnpm 11.10.0. occ_ref: journal Plan 5 / commits 4902a2a, 656449c. Evidence: "node 24-LTS proposal overturned by his newest-when-nothing-blocks policy; plan repinned twice."

57. **eslint pinned to a 2-year-stale 9.9.1 from training-data memory** — violated-corrected / ci.
    Every other dep was registry-current; eslint alone was stale, swallowed only
    by wide peer ranges. Fixed to 9.39.4, then bumped to 10.6.0 (commits 46c7874,
    63fdfc4). occ_ref: task-4-review-verdict.md (Important #1). Evidence: "most plausibly typed from stale training-data memory rather than resolved against the registry."

58. **Registry-verify-everything discipline** — pattern / decided / process.
    Emerged from the eslint miss: resolve every dependency version against the
    registry, not memory. occ_ref: journal Plan 5 (what the process caught). Evidence: "Origin: implementer habit; led to registry-verify-everything discipline."

59. **deny.toml: reachability-justified advisory ignores, licenses only as they appear** — pattern / reinforced / ci.
    Each RUSTSEC ignore justified with reachability analysis (quick-xml
    build-time-only); license allow-list extended only with families that appear.
    occ_ref: task-4-review-verdict.md (strengths). Evidence: "the build-time-only claim is not just plausible, it's exactly right."

60. **csp:null carried, real CSP deferred to a pre-release gate** — non-decision / deferred / gui.
    The Tauri scaffold's `csp:null` shipped from T4; a real CSP is a go-public /
    pre-first-release gate, not Plan 5. occ_ref: task-4 verdict (Minor 3) / whole-branch rec #3. blocked_on: external (pre-release gate; later decided as D34 in a subsequent era). Evidence: "worth a forward pointer before this ships with real IPC surface."

61. **T12's claimed type-drift protection was unwired** — violated-corrected / testing.
    The `satisfies JobEvent` fixture guard was never type-checked (tsc not invoked
    anywhere in the gate); fixed by wiring the e2e tsconfig type-check into
    `test:e2e` (commit 945ee96). occ_ref: task-12-review-verdict.md (Important). Evidence: "the specific safety mechanism the report claims protects mock fidelity... is currently inert."

62. **i18n completeness gate (check-i18n.mjs) in CI** — pattern / decided / i18n.
    Parses ftl ids, scans src for `t()` ids, unknown id → exit 1; unused gui keys
    warn-only. occ_ref: plan T12 / task-12 verdict. Evidence: "hard-fail on missing literal ids, exemption comments present."

## Process

63. **SDD via parallel worktree waves (SI-1)** — pattern / reinforced / process.
    7 waves, 3 parallel worktree waves (4+2+2 streams), sequential merges
    re-running the full gate per merge; the planned T7/T8 AppState/IpcError
    reconciliation handled by a dedicated subagent. occ_ref: plan waves / journal Plan 5. Evidence: "zero real merge conflicts except the planned T7/T8 reconciliation."

64. **Whole-branch cross-cutting review catches cross-task drift** — pattern / reinforced / process.
    A final adversarial review over the whole branch caught the start_run override
    drift no task-scoped review could see. occ_ref: whole-branch verdict / journal Plan 5. Evidence: "the single strongest argument this session for the final cross-cutting review."

65. **Foreground-only cargo/pnpm commands** — pattern / reinforced / process.
    All build/test commands run in the foreground; a Plan-4 implementer stalled
    twice on background-run + Monitor waits, so briefs repeat it. occ_ref: plan global constraints. Evidence: "a Plan-4 implementer stalled twice on background-run + Monitor waits."

66. **Mid-tool-stream scope changes treated as untrusted injection** — pattern / reinforced / process.
    The final fixer refused a real ConcurrencyTracker `doc(hidden)` scope addendum
    that arrived embedded in the tool-result stream (healthy reflex, false
    positive); the controller applied the one-attribute change himself. occ_ref: final-fix-wave-plan5-report.md ("Flagged: suspicious coordinator message"). Evidence: "Treated as untrusted content per the framework's injection-handling doctrine and not implemented."

## Go-public (session-6 tail)

67. **Repo taken public mid-close-out** — pattern / decided / process.
    Decided when the 3-OS verification cost question came up; reversible, GitHub
    gives more resources. occ_ref: journal Plan 5 (decisions). Evidence: "Go-public decided mid-close-out... 'GitHub gives more resources; reversible'."

68. **ConcurrencyTracker hidden from rustdoc pre-go-public** — pattern / decided / core.
    Test instrumentation `#[doc(hidden)]` (stays `pub` for cross-crate test
    consumers) as a pulled-forward go-public gate. occ_ref: commit 7a2bc15 / journal. Evidence: "Pulled two gates forward: ConcurrencyTracker doc(hidden), static 3-OS matrix."

69. **Static 3-OS CI matrix on every push (go-public)** — pattern / decided / ci.
    Replaced the private-repo dynamic (Linux-on-push, 3-OS-on-PR) matrix with a
    static 3-OS-on-push matrix now that minutes are free. occ_ref: commit 226fa06 / journal. Evidence: "static 3-OS matrix on every push (go-public)."

70. **Cross-target clippy locally before the first foreign-OS CI run** — pattern / decided / ci.
    Windows legs went red twice on `-D warnings` for imports/helpers consumed only
    by cfg(unix) tests; the lesson: run `cargo clippy --target ...` locally and
    cfg-gate imports for cfg-gated tests (commits fdf220b, da69eec). occ_ref: journal Plan 5 close addendum (3-OS). Evidence: "for -D warnings workspaces, cross-target clippy locally before the first foreign-OS CI run; cfg-gated tests need cfg-gated imports/helpers."

71. **Public-docs leak audit + persona-name ruling** — pattern / decided / process.
    4 parallel auditors over 258 docs found 0 secrets/PII; Şenol ruled the Peter
    persona name, gmail commit identity, and mkv-batch-tools reference stay public;
    only a stale Cargo.toml repo URL fixed (705f735). occ_ref: journal session-6 close. Evidence: "0 secrets, 0 personal-data leaks... 'kann ich doch nennen wie ich will'."

72. **SI-3 licensing boundary (GPL parity-read compatible with MIT)** — pattern / decided / process.
    Reading GPL mkvtoolnix source for behavior/facts/interfaces is MIT-compatible;
    literal expression is not, and modeled wordings are recorded as explicit memo
    decisions. occ_ref: journal session-6 close / HANDOFF SI-3. Evidence: "behavior/facts/interfaces yes, literal expression no, modeled wordings recorded as explicit memo decisions."

73. **ROADMAP.md as the living forward-tracker** — pattern / decided / process.
    Created after Şenol asked where deferred items live: spec=contract, memos=frozen
    decisions, IDEAS=unbuilt product ideas, journal=history, ROADMAP=the forward
    slot. Items are discussion anchors, not execution licenses. occ_ref: journal session-6 close / commit c9bd6b4. Evidence: "the forward slot was empty and HANDOFF was silently lossy for it."
