# Plan 9: core/orchestration hoists + planner seam

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **House deviation from the skill text:** progress NEVER enters this document. No box in this file is ever ticked; the checkbox syntax is structure, not a tracking surface. The tracker is `.superpowers/sdd/plan-9/progress.md`.
>
> **Execution starts only on the owner's plan approval** (standing gate; same rule the Plan-8.5 header carries).

**Goal:** execute the owner-approved Plan-9 design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`, D91-D105) exactly: hoist the four-copy planning pipeline into the core seam `pipeline::plan_pipeline` carrying the half-done funnel migration (D91-D95), hoist `run_batch` into core and delete the src-tauri runs-root seam (D96-D97), make the worker-panic payload travel as typed data and render on the CLI and the GUI job row (D98-D100), reject bare `raw:` at error severity (D101), sort `config_diagnostics` errors-first centrally and re-key BatchView's parse-diagnostic fetch (D102-D103), land the ruled D23 tests on a minimally extended mount harness (D104), and run the D49 G1/G2 removal experiment (D105). Amendment 1 (owner rulings 2026-07-28, design commit `064da74`) is folded in: the two formerly producer-less GUI consequences ship with their e2e scenarios in this plan, and the GUI identification session cache is out - the seam constructs its own per-call cache. The eight spec amendments S-1..S-8 land with the tasks whose code they describe.

**Architecture:** seven strictly serial tasks on `master` in the main worktree - no branches, no worktrees (ruling and reasoning in the sequencing section). Tasks 1-6 each commit; Task 7 mutates and restores and leaves the tree byte-unchanged, so it commits nothing. One ten-part gate run, foreground, before the single push at the plan close (the gate's binding site is pre-push; this plan has no merges). No task edits any house-knowledge YAML; ledger writes are controller close actions.

**Tech Stack:** Rust workspace (toolchain pinned via `rust-toolchain.toml`, currently 1.96.1), Tauri 2 / Vue 3 / TypeScript frontend, Playwright e2e with the in-repo mock+mount harness (`e2e/`), Fluent catalogs under `locales/`, insta snapshots for CLI human output. No new dependency of any kind, cargo or npm; `tempfile` is already a core dev-dependency (used by `crates/muxsmith-core/tests/joblog.rs`).

## Global Constraints

- **Ground truth and precedence:** the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative on conflict; below it the Plan-9 design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`) is the executable contract - **its D-entries plus EVERY entry in its `## Amendment log` bind this plan, at the log's state at EXECUTION time, not at plan-authoring.** Membership is deliberately not enumerated here: the pointer IS the contract (house ruling, plan-8.5 Task-2 review round). The ROADMAP Plan-9 anchor (`docs/ROADMAP.md`) carries the owner's eight IN rulings, two OUT rulings, the recorded promotion close action and the harness-scope correction, all binding. The four house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`) are ground truth alongside them: cite entries by id; re-verify any `:line` before relying on it.
- **No design decision is re-opened, softened, or "improved".** The design's section 5 enumerates what the implementer must not decide; every task below inherits that list in full and names the entries that touch it. A contradiction discovered on code contact is refuted with evidence or returned, never silently absorbed.
- **Every fork in this plan is closed.** No task brief, verdict or fix-round dispatch may carry a design-latitude clause, in either form: an explicit permission, or an omission - an unenumerated set in a normative position, a list ending open, a "one per X" with no X list, a step that requires inventing a name, a string, or a file that is not written down somewhere the implementer can read (this plan or the design's own fences). A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** (options, costs against the named invariants, a recommendation) and is routed by the controller, never resolved at the keyboard (`proc-latitude-clause-boundary`).
- **Ten-part gate** per BUILDING.md ("The Rust gate" six parts + the four frontend checks; Tier-2 `gate-includes-cross-target-lint-for-the-unrun-os`), run foreground, no subsets, **before any push and after every merge**: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`; `cargo deny check`; `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`; `pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e`. This plan has no merges (serial commits on master); the gate's mandatory site is the pre-push run in the close actions. Per-task verification below names the subset each task must run green before committing; that subset is a task exit bar, not a gate substitute.
- **Pins:** no new runtime or product dependency in either ecosystem (cargo or npm); no GitHub workflow is modified by any task; nothing to SHA-pin.
- **SI-4 (restate in every dispatch that expects a commit; `dispatch-restates-the-standing-commit-grant`):** commits and pushes on this repo are standing-authorized by the owner; agent commits are deliberately unsigned - `git -c commit.gpgsign=false commit ...` - with exactly one trailer, `Co-Authored-By: Claude <model> <noreply@anthropic.com>`, where `<model>` is the canonical model name **derived from that dispatch's explicit model parameter, never written as a literal in this plan or a task brief** (`agent-commit-trailer-set`; no `Claude-Session` line, no context-window suffix). Stage files explicitly, **never `git add -A`**; every push gets a `gh-log.md` entry.
- **No task edits any house-knowledge YAML** (`docs/decision-ledger.yaml`, the three Tier-2 files). The controller is the single writer; a task that finds something ledger-worthy SURFACES it in its report. The spec and the design are documents this plan's tasks DO edit where a task below says so (S-1..S-8); nothing else under `docs/` is touched by any task.
- **No task creates a tag, publishes or edits a release, or resolves a README `placeholder(1.0)` comment.**
- **No new test scenarios beyond the ruled D23 tests (D104), the two amendment-1 scenarios (D101/D103), and the tests the design's D-entries pin** (enumerated per task below). The amendment-1 boundary, stated so it is not misread in either direction: new test SCENARIOS on the existing Playwright + mock-IPC harness are in scope; new test INFRASTRUCTURE is not - no Vitest, no `tauri::test`/`mock_builder`, no `src-tauri/tests/` directory, no IpcError funnel work; the two OUT items and `e2e/mount.ts` are untouched.
- **Counts are recomputed from their enumerations** (`proc-normative-count-recomputed`): every count in this plan was recomputed from its list at plan-authoring (2026-07-28); a task that changes a set re-recounts and updates the consuming line in the same change.
- **Verification steps whose expected result is an absence are fire-verified AND have a reachable green state** (`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`): break it or run it on the pre-state where it must hit, watch it fire, then reach the pass on the intended end state - each absence check below carries both halves, with the green state argued member-by-member against the enumerated survivors where it cannot be run at authoring time.
- **Evidence lines carry pasted output** (`design-empirical-claims-reproducible`): every observed value in a task report is pasted from the run that produced it, never recalled, and never attributed to a command that was not the one run.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, in this plan, in every code comment, and in every string it prescribes. The design's Fluent fences contain correct German orthography (`ä`, `ü`, `ß`); those letters are orthography, not AI-tell glyphs, and are copied exactly.
- **Implementer preamble, verbatim in every dispatch:** subagents never call session-relocation tools (EnterWorktree/ExitWorktree or any equivalent); absolute paths; foreground runs only; work on `master` in the main worktree.

## Execution method (binding)

Subagent-driven development (`superpowers:subagent-driven-development`): a fresh implementer subagent per task, an independent reviewer per task grading against this plan, the plan brief, the design (including its amendment log at execution time), the ROADMAP anchor and the spec, and a whole-branch review at the plan close before the close actions. Progress lives in `.superpowers/sdd/plan-9/progress.md`. `feedback_use_specified_execution_method` applies.

## Model tiers (proc-03-model-assignment)

Every task reviewer runs the mid tier; the whole-branch review at the plan close runs the top tier; the controller loop runs mid. The controller sets the model parameter explicitly at every dispatch - an omitted parameter inherits the session default, which is not an assignment. **No task in this plan qualifies for the cheap tier:** the cheap tier is reserved for work this plan carries verbatim (transcription), and every task here either weaves design-fixed code into existing files or writes tests the design pins by scenario rather than by patch text.

| task | tier | ground |
|---|---|---|
| 1 (planner seam + 4 migrations) | mid | four call-site migrations with byte-behavior preservation are judgment; the seam types are design-fixed but the surrounding presentation mapping is composed |
| 2 (run_batch hoist + runs-root delete) | mid | the CLI on_event integration and test relocation are judgment; the deletion table is fixed |
| 3 (worker-panic path end to end) | mid | cross-layer wire change (Rust, TS, Fluent, e2e fixtures) with a compiler-driven sweep |
| 4 (EmptyRawProperty) | mid | function and strings are design-fixed; the pinned core and subprocess tests are composed against existing test idioms |
| 5 (central sort + BatchView) | mid | the re-export cutover and the discriminating order/parity tests are composed |
| 6 (D23 e2e tests + harness hook) | mid | Playwright harness extension and mock composition are judgment |
| 7 (D49 experiment) | mid | live experiment evaluation and anomaly recognition are judgment (plan-8.5 experiment-task precedent) |

## Authoring-time verification (2026-07-28)

The design's own anchor re-verification ran at master on 2026-07-28. Since then exactly two commits landed, both doc-only (pasted from `git show --stat`): `b4daed6` (`docs/ROADMAP.md`, `docs/decision-ledger.yaml`) and `39d7c42` (`docs/ROADMAP.md`, `docs/process-conventions.yaml`, the design file). No code file moved, and every anchor this plan makes load-bearing was re-run at plan-authoring; observed values below are pasted from those runs.

- **Funnel-inline hit set** (`grep -rn "config_diags.extend(lint::provable_overlaps" crates src-tauri`): exactly 4 hits - `src-tauri/src/lib.rs:211`, `src-tauri/src/run.rs:263`, `crates/muxsmith-cli/src/commands/run.rs:85`, `crates/muxsmith-cli/src/commands/dry_run.rs:60`.
- **Source-default hit set** (`grep -rn 'PathBuf::from(".")' crates/muxsmith-cli/src src-tauri/src crates/muxsmith-core/src`): exactly 4 hits - `dry_run.rs:109`, CLI `run.rs:143`, `lib.rs:230`, src-tauri `run.rs:291`; none in core.
- **Core stdio** (`grep -rn "eprintln!\|println!\|print!(" crates/muxsmith-core/src`): exactly 2 hits - the comment mention `lib.rs:23` ("... literal at ~21 `eprintln!` sites ...") and the one call `executor/queue.rs:396`. Control: the same grep over `crates/muxsmith-cli/src` hits six files (`main.rs`, `commands/{identify,dry_run,validate,mod,run}.rs`).
- **Executor anchors:** `fn run_batch` at `src-tauri/src/run.rs:782`; the two behavior tests `run_batch_emits_started_output_finished_in_order` (`:1228`) and `run_batch_writes_job_log_files` (`:1334`); `fn resolve_runs_root` at `:827` with call sites `:326`, `:529`, `:535`; `handle.join().expect("queue worker thread panicked")` at CLI `run.rs:271` and src-tauri `run.rs:799`; the queue panic-recovery test `worker_panic_is_reported_as_failed_not_cancelled` at `queue.rs:738`.
- **`MUXSMITH_RUNS_ROOT` complete tree set** (grep excluding `.worktrees`, `docs`, generated output): 9 lines - CLI `run.rs:317` (doc) and `:331` (read), the five CLI test sites (`run_cli.rs:172`, `run_live.rs:110,245,370,468`), src-tauri `run.rs:825` (doc) and `:830` (read).
- **Other code anchors:** CLI `severity_sorted` defined at `commands/mod.rs:21`; `batch_document`/`config_only_document`/`rendered_diags` at `report/json.rs:35/:78/:176`; `run_document` at `report/json.rs:112`; `BatchView.vue:218` (`!doc.profile`) and `:225` (`config_diagnostics[0]`, the only such read in the file); `raw_opt_in_diagnostic` at `validate.rs:408-414` (two-branch form today) with the two arms at `:268`/`:310`; `"n/a"` literal at CLI `run.rs:487`; `delta_for`'s `AddExact` arm at `planner.rs:1820-1827` and `scalar_display` at `:856`; G1/G2/G3 at `suggestions.rs:1037/:1074/:1113`; `load_profile_body`'s `Err` arm at `lib.rs:316-320`; `Diagnostic::error` exists (`report/mod.rs:242`); `report/mod.rs` has an inline `#[cfg(test)]` module (`:305`); `worker-panicked` current lines at `locales/en/diagnostics.ftl:80` / `locales/de/diagnostics.ftl:87`; the frontend bundles `diagnostics.ftl` (`src/i18n/index.ts:18` glob); `WorkerPanicked => vec![]` fixture row at `catalog_completeness.rs:152`, `ALLOWLISTED_CLI_KEYS` at `:182`; smoke live-run describe at `e2e/smoke.spec.ts:477`; `installMockIPC` exported at `e2e/mocks.ts:84`; harness bundles at `e2e/.generated/{tauri-mock-harness.js,mount-harness.js}` (`mocks.ts:24`, `mount.ts:14`).
- **Amendment-1 anchors, re-verified at amendment time (2026-07-28, HEAD `064da74`; regions re-opened at the amendment fix round):** `hasErrors` at `BatchView.vue:282`, the `tooltip-errors` reason at `:304`, `data-testid="batch-run"` at `:511`, the `!doc.profile` early-return branch at `:218-227`; the localized tooltip text `.tooltip-errors = Fix every error-severity diagnostic before running.` at `locales/en/gui-batch.ftl:69`; the en `parse-error` template `The profile could not be parsed: { $detail }` at `locales/en/diagnostics.ftl:6`. In `e2e/smoke.spec.ts`: the single batch describe `batch view: dry run` opens at `:140` and is BOTH scenarios' home (the apply-flow test at `:406` and the plural-counts test at `:307` sit inside it; the next describe opens at `:477`); the enabled assertion `await expect(runButton).toBeEnabled()` is at `:511` (locator `:510`) INSIDE the `jobs view: live run` describe (`:477`, test `:491`), so it pairs the Run-gate scenario by assertion, not by location; `pluralReport` already feeds BatchView an error-severity document today (`severity: "error"` at `:272-277`, resolved via `dry_run` at `:315`); `smoke.spec.ts`'s only `toBeDisabled` is the editor Save at `:1241` (suite-wide: three, adding `editor-rule-add-remove.spec.ts:151`/`:317`; a `batch-run`-disabled assertion exists nowhere - measured at the amendment fix round).
- **Amendment-2 anchors, verified by opening each file at the line (2026-07-28):** the ROADMAP trigger line at `docs/ROADMAP.md:628` (its condition counts helper consumers, not spec files; carries the controller's NOT FIRED / consumed-early record since the amendment-2 fix); the e2e spec-file count today: `ls e2e/*.spec.ts | wc -l` -> 9 (so Task 6's new spec is the TENTH); the three identical local `name()` helpers with their doc comments at `e2e/smoke.spec.ts:53-62` (comment `:53-59` carrying the substring-collision rationale, function `:60-62`), `e2e/editor-markers.spec.ts:27-31`, `e2e/editor-rule-add-remove.spec.ts:39-43`; the shared module `e2e/i18n-en.ts` exporting `assertAllCatalogsParseCleanly` (`:125`), `en` (`:155`), `enAttr` (`:170`) with `FluentVariable` already imported (`:61`); `data-testid="cancel-batch"` at `src/views/JobsView.vue:263`. Fire measurement for Task 6's hoist check, pasted: `grep -rn "^function name(" e2e/*.spec.ts` -> `editor-markers.spec.ts:29`, `editor-rule-add-remove.spec.ts:41`, `smoke.spec.ts:60` (exactly 3); `grep -c "export function name(" e2e/i18n-en.ts` -> 0 today (the presence control's red state, 1 post-hoist); `FluentVariable` in each spec file has exactly two hits, the type import and the helper signature, so the import goes unused on deletion.
- **Already done, confirmed (re-creating either is a defect):** `gui-d23-reset-gating-form` exists in `docs/decision-ledger.yaml` (`:4537`; `:4535` at first authoring - the exec-43 field insertion of commit `2155c1d`, corrections table row 2, shifted later entries by two lines); the design's four triggers are mirrored in the ROADMAP Triggers section (the four lines tagged "Plan-9 design trigger 1..4"). `core-d49-g1g2-experiment` does NOT exist yet (grep over the ledger returns nothing; fired control: the same grep for `core-121-planner-seam-and-hoist` returns 1) - it is written by the CONTROLLER at the close from Task 7's measurement.
- **Fixture probes, run against the current `target/debug/muxsmith` (built from master):**
  - A profile rule `- match: { exact: { 'raw:': eng } }` parses (single-quoted YAML key), and `validate --json` today returns exactly one diagnostic: `raw-property`, severity `info`, `config_path` `tracks[0].match.exact.raw:`, `params.property` `""`, exit 0. The substring twin `- match: { substring: { 'raw:': en } }` returns the same at `tracks[0].match.substring.raw:`. This is the exact today-state D101 flips to error/exit 2, and it proves Task 4's fixture syntax and path assertions end to end.
  - The Task-5 parity fixture (rules in the order info, warning, error: `raw:x`, `raw:language`, `regex: { title: '[' }`) yields on the current tree: `validate --json` sorted `[unknown-property, invalid-regex, raw-on-known-property, raw-property]` (errors first), while `dry-run --json` (mkvmerge forced off PATH) yields collection order `[raw-property, raw-on-known-property, unknown-property, invalid-regex]`. The fixture therefore discriminates: the new parity test is red today and green exactly when D102 lands.

**Corrections to the brief found at plan-authoring** (`proc-57-briefs-not-ground-truth`; neither changes a ruling or a task):

| # | Brief statement | Reality |
|---|---|---|
| 1 | Close actions include "the promotion sweep of the six owner-ruled entries the ROADMAP anchor records" | **The anchor records FIVE.** The ROADMAP's recorded close action enumerates `exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`, `cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`, `empty-bare-raw-property-rejected-at-validate` and itself calls them "the five ledger entries these rulings touched" (recomputed from the enumeration: 5). The sixth the brief likely counted is `core-121-planner-seam-and-hoist`, whose `blocked_on` clearing the brief already lists as its own separate close action. The close actions below carry five plus the separate `core-121` action. |
| 2 | ROADMAP close action premise (rider found while verifying #1): the five entries "now all carry `source: human`" | **Was false for one member at plan-authoring; found here, routed, and SATISFIED by the controller the same day.** At authoring (2026-07-28 05:04 state) `exec-43-runsroot-debug-gated` carried no `source` field while the other four carried `source: human`. Routed as NEEDS_CONTEXT (house YAML is controller-written); the controller resolved it at 05:06 in commit `2155c1d`, adding `source: human` and `nature: technical-code` to the entry (recorded in `.superpowers/sdd/plan-9/progress.md` under "Post-authoring ground-truth edits"). At EXECUTION time the anchor's premise is therefore true for all five. Rider: that 2-line insertion shifted every later ledger anchor by two lines (`gui-d23-reset-gating-form` `:4535` -> `:4537`); the one such citation in this plan is corrected below. |

## Design coverage map

Every design section -> the task or actor that implements it. This is the walk the plan reviewer repeats; a row missing here is a defect.

| Design section | Implemented by |
|---|---|
| Section 0 (notes/corrections to the design brief) | Informational; note 3's surfaced harness deviation is executed by Task 6, note 5 by Task 3 (no `JobRowData` change) |
| Section 1 (SI-3 parity audit) | Informational; feeds D99/D100/D102, no separate work |
| D91 seam + four call-site mappings | Task 1 |
| D92 `mkvmerge_found` unified meaning, builder doc | Task 1 |
| D93 per-call cache, no interface change (amendment 1) | Task 1 (the seam's internal `IdentifyCache::new()` at S6; everything else in D93 is an explicit non-change no task may undo) |
| D94 `job_specs`, per-surface empty-specs branch | Task 1 (derivation + both gates); end-state queue call lands in Task 2 |
| D95 one `"."` default in the seam | Task 1 |
| D96 `run_batch` hoist + two tests move | Task 2 |
| D97 runs-root deletion, three call sites | Task 2 |
| D98 `JobOutcome.panic`, eprintln deletion, wire mirrors | Task 3 |
| D99 CLI two-line panic rendering + catalog obligations | Task 3 |
| D100 GUI job-row render | Task 3 |
| D101 `EmptyRawProperty` + pinned tests + the amendment-1 Run-gate e2e scenario | Task 4 |
| D102 `severity_sorted` hoist + central sort | Task 5 |
| D103 BatchView code-keyed fetch + the amendment-1 parse-failure apply e2e scenario | Task 5 |
| D104 harness widening + `e2e/jobsview-reset.spec.ts` (test half of the ruled item) | Task 6 (the ledger half is already written, commit `b4daed6` - no task touches it) |
| D105 D49 experiment protocol | Task 7 (measurement + report); ledger entry is a controller close action |
| Section 2 (removal recount; the four discarded executor failures STAY discarded) | Constraint carried verbatim in Tasks 2 and 3 |
| Section 3 spec amendments | S-4 + S-8 -> Task 1; S-1 WorkerPanicked-row replacement + S-2 -> Task 3; S-1 EmptyRawProperty-row insertion + S-3 + S-5 + S-6 -> Task 4; S-7 -> Task 5 |
| Section 4 ledger obligations | Controller: `gui-d23-reset-gating-form` DONE (`b4daed6`); `core-d49-g1g2-experiment` and `core-121` `blocked_on` at the close |
| Section 5 (what the implementer must not decide) | Inherited whole via Global Constraints; referenced per task |
| Section 6 triggers | DONE - mirrored into the ROADMAP (`b4daed6`); no task mirrors them again |
| Section 7 acceptance observables | Mapped below; every check cited from there, not restated (`design-states-a-completion-check-once`) |
| Amendment log | Binds at execution time via the pointer contract (Global Constraints) |

## Sequencing, dependency graph, and the no-worktree ruling

**Strictly serial: Task 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.** Edges and their nature:

- 1 -> 2: same files (CLI `run.rs`, src-tauri `run.rs`). Task 1 migrates CLI `run`'s planning stretch onto the seam while **retaining the inline mpsc queue block byte-unchanged**; Task 2 replaces that block with the hoisted core `run_batch`. This staged cut is deliberate: both intermediate states compile and pass the suite, and the design's end state for copy 2 (D91 mapping 2: "the queue via core `run_batch` (D96)") is reached at Task 2.
- 2 -> 3: same file (core `executor/queue.rs`): Task 2 moves `run_batch` and the two tests in; Task 3 then edits `recover_panicked_worker` and runs its compiler-driven `JobOutcome` sweep over the tests in their final home.
- 3 -> 6: the e2e panic-render test needs `JobOutcome.panic` on the wire (D98) and the `JobRow.vue` render site (D100).
- 4, 5: no hard data edge to 3 or to each other, but they share the spec document (S-edits) and the CLI test crate with their neighbours, and - since amendments 1 and 2 - `e2e/smoke.spec.ts` with Tasks 3 and 6: four tasks write that file serially, each owning exactly one named region (Task 3 the fixture `panic: null` sweep, Task 4 the Run-gate scenario, Task 5 the apply scenario, Task 6 the `name()`-hoist deletion + import line), so ownership stays unambiguous under the serial order; ordered by D-number. The hoist sits in Task 6 deliberately: Task 6 creates the new spec file whose arrival prompted the owner's ruling, and it is the LAST smoke-writer slot, so the move cannot race the three earlier regions - both reasons hold independently of any trigger (the registered trigger never fired; see the amendment-2 note).
- 7: independent and residue-free (mutate, measure, restore); runs last so its green control doubles as a late whole-suite confirmation on the plan's end state.

**No worktrees, as a ruling.** The doctrine's handle is a comparison, not a count: a worktree stream costs a setup, a merge, a full ten-part gate on the merged state, and the controller choreography around both. Here tasks 1-3 are a chain on shared files, 4 and 5 share the spec file and CLI test crate, 6 depends on 3, and the only fully independent task (7) writes no file at all - so no stream buys concurrency that the file graph does not immediately take back, and no single task's own work exceeds the overhead of a second full gate run. **The serial ruling binds the CONTROLLER's dispatch concurrency too, not merely the task order** (`a-serial-ruling-binds-dispatch-concurrency-too`): no second writer is dispatched while a task is live. If the controller ever must dispatch a concurrent writer into this one tree anyway (a mid-plan amendment), pathspec-scoped commits (`git commit -- <paths>`) are mandatory, because one tree means one index (`concurrent-writers-need-pathspec-scoped-commits`).

Commits: Tasks 1-6 commit on master with explicit pathspecs (blocks per task). One push, at the close, after the ten-part gate (plan-8.5 precedent: the gate's binding site is pre-push, and serial commits on master have no merge sites).

## Acceptance observables map (design section 7)

Checks are cited from the design, never restated (`design-states-a-completion-check-once`). "MV" = machine-verifiable by a command or test named in the design/plan.

| Observable | Producer | MV |
|---|---|---|
| 1. Pipeline hoist + funnel migration | Task 1 (grep + workspace gate + unchanged suites) | yes |
| 2. `run_batch` hoist | Task 2 | yes |
| 3. Runs-root deletion | Task 2 | yes |
| 4. Worker-panic path | Task 3 (core assertion, CLI unit test, catalog guard, stdio grep) + Task 6 (e2e `job-panic` render, D104 item 4) | yes |
| 5. Empty bare `raw:` | Task 4 (two per-arm tests + existing B-2/B-3 controls, subprocess en/de tests, catalog row, and the amendment-1 Run-gate e2e scenario: `batch-run` disabled with the `tooltip-errors` title) | yes |
| 6. Central sort + BatchView | Task 5 (discriminating order test, CLI parity subprocess test, BatchView grep, and the amendment-1 parse-failure apply e2e scenario: alert surfaces the parse diagnostic, no `apply_suggestion`/`save_profile` invoked) | yes |
| 7. D23 item | Task 6 (the three ordering tests + panic-render test under `pnpm test:e2e`); the ledger half already exists (`b4daed6`) and `scripts/ledger-lint.py` runs in CI/at the gate | yes |
| 8. D49 experiment | Task 7 (recorded green-mutate-restore run with pasted outputs) + controller close (the `core-d49-g1g2-experiment` entry) | yes (the measurement); the entry is a close artifact |

**No observable is producer-less under amendment 1.** The two consequences the pre-amendment design carried as uncovered - the GUI Run-gate consequence of D101's error severity and the branch D103 edits - now ship with fully enumerated e2e scenarios (design D101/D103, amendment 1, ruling A: a feature's tests ship with the feature), implemented by Tasks 4 and 5 respectively. The boundary that ruling states explicitly binds every task: new test SCENARIOS on the existing Playwright + mock-IPC harness are in scope; new test INFRASTRUCTURE (Vitest, `tauri::test`/`mock_builder`, a `src-tauri/tests/` tree) stays out at 1.x, and the v1.x "GUI test harness for the run path" ROADMAP item (which covers `start_run`'s untested composition) stays as it is.

---

## Task 1: The planner seam - `pipeline.rs`, four call-site migrations (D91, D92, D93, D94, D95; spec S-4, S-8)

Read first: design D91-D95 in full, including D91's divergence table, the four call-site mappings, and the amended D93 (per-call cache, amendment 1); design section 5 (the seam bullets); spec sections 5.5 and 7. Model tier: mid.

**Files (EXHAUSTIVE):**
- Create: `crates/muxsmith-core/src/pipeline.rs`
- Modify: `crates/muxsmith-core/src/lib.rs` (module declaration only)
- Modify: `crates/muxsmith-cli/src/commands/dry_run.rs` (call-site mapping 1)
- Modify: `crates/muxsmith-cli/src/commands/run.rs` (call-site mapping 2; the inline queue block is NOT touched)
- Modify: `src-tauri/src/lib.rs` (call-site mapping 3 only)
- Modify: `src-tauri/src/run.rs` (call-site mapping 4 only)
- Modify: `crates/muxsmith-core/src/report/json.rs` (D92: the `config_only_document` doc comment at `:66-73` only)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (amendments S-4 and S-8 only)

**Interfaces:**
- Consumes: nothing from other tasks.
- Produces: `pipeline::{plan_pipeline, PipelineOutcome, PlannedPipeline, job_specs}`, which Tasks 2-6 build on. No other public interface changes: per the amended D93, `LiveIdentifier` and `IdentifyCache` are untouched, `crates/muxsmith-core/tests/command_integration.rs` is untouched, and `AppState` gains no field - each an explicit non-change this task may not undo.
- Carries verbatim into Task 2: "the inline CLI queue block (`mpsc::channel` -> scope -> `expect(\"queue worker thread panicked\")`) is retained byte-unchanged by Task 1 and replaced only by Task 2."

- [ ] **Step 1: the core module.** Create `crates/muxsmith-core/src/pipeline.rs` with `PipelineOutcome`, `PlannedPipeline`, `plan_pipeline` (the amended five-parameter signature - no cache parameter) and `job_specs` exactly as the design's D91 and D94 fences write them (types, fields, signatures, doc comments character for character; rustdoc line-wrapping is the only permitted difference). Internal step order is exactly S1-S7 as D91 fixes it, including the funnel call `profile::validate::config_diagnostics(&profile)` at S2, the single `source.unwrap_or_else(|| PathBuf::from("."))` at S5 (D95), and at S6 the pipeline-constructed `IdentifyCache::new()` wrapped in the `LiveIdentifier`, dropped when the call returns (amended D93 - per-call cache, no session cache in any form). The near-verbatim rationale comments duplicated between the CLI copies consolidate into this module's docs; the per-branch S3e/S4e rationale moves onto the enum variants (already present in the D91 fence's variant docs); the D95 note ("No natural 'current directory' for a bundled desktop app ...") moves verbatim onto the seam's source-default doc. Declare `pub mod pipeline;` in `crates/muxsmith-core/src/lib.rs`.
- [ ] **Step 2: call sites 1 and 2 (CLI).** Migrate `dry_run.rs::run` and `run.rs::run` per D91's mappings 1 and 2: each calls `plan_pipeline(profile_path, source, output, on_collision, Mkvmerge::locate)` (mapping 1 passes the CLI's `--on-collision` value; both CLI sites pass `Mkvmerge::locate` per D-1), and maps each `PipelineOutcome` variant to exactly today's printed bytes, stderr lines and exit codes. `run.rs` then derives `pipeline::job_specs(&batch)` (D94) and keeps its empty-specs presentation and the entire inline queue block byte-unchanged (Task 2 owns that block).
- [ ] **Step 3: call sites 3 and 4 (GUI).** Migrate `dry_run_body` (src-tauri `lib.rs`) and `plan_run` (src-tauri `run.rs`) per D91's mappings 3 and 4: `plan_pipeline(profile_path, source, output, None, || Mkvmerge::detect(mkvmerge_override))`, `plan_run` keeps its settings-read-first (D-7 stays caller-side, including the wrapper-vs-body layer asymmetry - deliberately not unified). `plan_run`'s empty-specs mapping to `PlanOutcome::Soft` gates on `pipeline::job_specs(&batch).is_empty()` exactly as today's derivation did (D94).
- [ ] **Step 4: D92 doc.** Update the `config_only_document` builder doc (`report/json.rs:66-73`) to carry D92's contract sentences verbatim: `false` = the surface's resolver produced no usable mkvmerge for planning (CLI: PATH lookup failed or the found binary could not answer `--version`; GUI: the detect ladder failed for any reason including `TooOld`); `true` = a usable mkvmerge resolved and the subsequent query failed; key absent on a profile-load failure. No wire change of any kind (D92: no new field, no reason on `MkvmergeUnavailable`).
- [ ] **Step 5: spec amendments S-4 and S-8.** Insert the `pipeline` module-table row and replace spec 5.5's identification-cache sentence exactly as the design's section 3 S-4 and S-8 fences write them (S-8 records the owner-ruled per-call cache; the design's contradiction sweep already checked the neighbouring run bullet).
- [ ] **Step 6: verification.**
  - **Acceptance observable 1, exactly as stated in the design's section 7 item 1** - run its grep with its recorded fire-verification (pre-edit it must hit exactly the four sites; the authoring-time run above measured those same four). Reachable green, argued member-by-member: post-migration the surviving `lint::provable_overlaps` references are the funnel's own call in `validate.rs` (receiver `diags`, which the qualified pattern `config_diags.extend(` cannot match) and any prose mention in `pipeline.rs`'s consolidated docs, which must not restate the qualified call text (write the doc so it does not; the check then passes on the real end state, not vacuously).
  - **D95 single-default check:** `grep -rn 'PathBuf::from(".")' crates/muxsmith-cli/src src-tauri/src` -> 0. Fire: the same grep pre-edit -> exactly the 4 hits pasted in the authoring section. Green state: the one surviving instance lives in `crates/muxsmith-core/src/pipeline.rs`, outside this grep's pathspec by construction; presence control `grep -c 'PathBuf::from(".")' crates/muxsmith-core/src/pipeline.rs` -> 1.
  - `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` - all green, foreground. The CLI subprocess suites (`dry_run_cli.rs`, `cli_validate.rs`, `run_cli.rs`, `run_live.rs`), `command_integration.rs` and the src-tauri inline tests pass UNCHANGED (byte-behavior preservation; under the amended D93 this task touches NO test file, so any test edit at all is a defect signal -> NEEDS_CONTEXT, not a local fix).
- [ ] **Step 7: commit.**

```bash
git add crates/muxsmith-core/src/pipeline.rs crates/muxsmith-core/src/lib.rs crates/muxsmith-cli/src/commands/dry_run.rs crates/muxsmith-cli/src/commands/run.rs src-tauri/src/lib.rs src-tauri/src/run.rs crates/muxsmith-core/src/report/json.rs docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "core: hoist the four-copy planning pipeline into the plan_pipeline seam (D91-D95, S-4/S-8)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the seam's module, signature, shapes and the four mappings - no additional parameter, no builder, no trait; the resolver closures; no new document field (D92); the per-call cache with no cache parameter, no `LiveIdentifier` change, no `AppState` field, no session cache in any form (D93, owner ruling amendment 1); `job_specs`' home and doc (D94); the `"."` default exactly once (D95).

---

## Task 2: `run_batch` hoists into core; the runs-root seam is deleted (D96, D97)

Read first: design D96 (including the caller-side checklist) and D97 (the three-row table); design section 2's "stay discarded" paragraph; design section 5 (the run_batch and runs-root bullets). Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `crates/muxsmith-core/src/executor/queue.rs` (gains `pub fn run_batch` + the two moved tests)
- Modify: `src-tauri/src/run.rs` (loses `run_batch`, the two tests, `resolve_runs_root`; three call sites -> `default_runs_root()`)
- Modify: `crates/muxsmith-cli/src/commands/run.rs` (inline queue block -> core `run_batch` call)

**Interfaces:**
- Consumes: Task 1's migrated CLI `run.rs` (the retained inline block is the exact region this task replaces).
- Produces: `muxsmith_core::executor::queue::run_batch`, which Task 3's compiler sweep and moved-test edits build on.
- Carries verbatim into Task 3: "the four silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs` remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded - the recorded steelman of `exec-36`'s ruled no-facade position; no task 'improves' them in passing."

- [ ] **Step 1: the move (D96).** Move `run_batch` (src-tauri `run.rs:782-804` at authoring; locate by `fn run_batch`) as-is into `crates/muxsmith-core/src/executor/queue.rs`, `pub`, exact signature per D96's fence, rustdoc moved with it. The boundary is exactly where today's function ends: it does NOT absorb `run_document`, `finalize_joblog`, or the CLI's joblog messages.
- [ ] **Step 2: GUI caller.** The runner thread calls the core function; `TeardownGuard` stays wrapped AROUND the call on the runner thread; `fail_fast` stays caller-built (`false` hardcoded in the GUI's `QueueOpts`). Nothing else in the shell's composition (reservation, cancel, teardown tests) moves.
- [ ] **Step 3: CLI caller.** Replace the inline block (locate: `mpsc::channel` through `handle.join().expect("queue worker thread panicked")`, `run.rs:241-272` at authoring) with a call to core `run_batch` whose `on_event` closure is exactly the design's classed-(a) per-event work: `if json { return; } for line in milestones.render(event, total, renderer) { println!("{line}"); }`. The logger tee order is preserved by the moved function itself (tee before `on_event`); persistence stays unconditional under `--json`.
- [ ] **Step 4: the two tests move** (`run_batch_emits_started_output_finished_in_order`, `run_batch_writes_job_log_files`) into `queue.rs`'s inline test module, adapted only in paths/imports. The shell keeps every teardown/reservation/cancel test.
- [ ] **Step 5: runs-root deletion (D97).** Delete `resolve_runs_root` and its doc comment; convert the THREE call sites exactly per D97's table (`plan_run` -> `default_runs_root().and_then(...)` unchanged tail; `list_runs` and `get_job_log` -> `default_runs_root().as_deref()`). The CLI gate stays byte-identical - locate it by content (`grep -n "MUXSMITH_RUNS_ROOT" crates/muxsmith-cli/src/commands/run.rs` must still return exactly its 2 hits, doc + read, with unchanged text), never by line number.
- [ ] **Step 6: verification.**
  - **Acceptance observable 2, as stated in the design's section 7 item 2** (fire: pre-edit `grep -n "fn run_batch" src-tauri/src/run.rs` hits `:782`, per the authoring run). Reachable green, member-by-member: post-state occurrences of `run_batch` in src-tauri are call expressions and doc mentions; none contains the definition pattern `fn run_batch`.
  - **Acceptance observable 3, as stated in the design's section 7 item 3** (fires: the authoring runs above - `MUXSMITH_RUNS_ROOT` in src-tauri hits `:825`/`:830` today; `resolve_runs_root` in src-tauri hits `:326`/`:529`/`:535`/`:827` today, 4 hits recomputed). Reachable green: the deletion removes all four `resolve_runs_root` sites and both `MUXSMITH_RUNS_ROOT` lines, and the authoring-time full-tree set shows no other src-tauri occurrence exists to survive. `cargo test -p muxsmith-gui` passes untouched (its runs-root tests inject explicit paths).
  - The two moved tests run under `cargo test -p muxsmith-core` (named in the run output).
  - `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` - green, foreground.
- [ ] **Step 7: commit.**

```bash
git add crates/muxsmith-core/src/executor/queue.rs src-tauri/src/run.rs crates/muxsmith-cli/src/commands/run.rs
git -c commit.gpgsign=false commit -m "executor: hoist run_batch into core, delete the src-tauri runs-root seam (D96, D97)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the verbatim move with today's signature; the not-absorbed pair; `TeardownGuard`/`fail_fast` caller-side; exactly the D97 table's function and three call sites; the CLI gate untouched. Debug-build behavior loss is ruled and stated per site in D97 - not re-weighed.

---

## Task 3: The worker-panic payload, end to end (D98, D99, D100; spec S-1 WorkerPanicked row, S-2)

Read first: design D98 (field, wire memo, fork 9, fork 12), D99 (both rejected alternatives, the four Fluent fences, catalog obligations), D100 (render site fence, scope boundaries), section 0 notes 4 and 5; design section 5 (panic bullets). Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `crates/muxsmith-core/src/executor/job.rs` (`JobOutcome.panic` field, D98's doc verbatim)
- Modify: `crates/muxsmith-core/src/executor/queue.rs` (`recover_panicked_worker` sets `Some`; the `eprintln!` deleted; the licence comment rewritten; the panic-recovery test extended; every flagged constructor in this file gains `panic: None`)
- Modify: `crates/muxsmith-core/src/executor/joblog.rs` (`JobRecord` gains `panic: Option<&'a str>` via `outcome.panic.as_deref()`)
- Modify: any other file the compiler flags for a `JobOutcome` literal (EXEMPLARY by nature - the compiler enumerates the set; every flagged existing constructor sets `panic: None`, and `recover_panicked_worker` stays the only `Some`-setter)
- Modify: `crates/muxsmith-cli/src/commands/run.rs` (`render_finished` panic branch; test helper `outcome()` gains `panic: None`; the new unit test)
- Modify: `crates/muxsmith-cli/tests/catalog_completeness.rs` (three edits per D99's obligations list)
- Modify: `locales/en/diagnostics.ftl`, `locales/de/diagnostics.ftl` (`worker-panicked` line replaced, both locales)
- Modify: `locales/en/cli.ftl`, `locales/de/cli.ftl` (`run-job-panicked` key added, both locales)
- Modify: `src/ipc.ts` (`JobOutcome` and `JobLogRecord` gain `panic: string | null`, required)
- Modify: `src/components/JobRow.vue` (D100's computed + span)
- Modify: `e2e/smoke.spec.ts` (the live-run scenario's `JobOutcome`/`RunJobEntry` object literals gain `panic: null` - the set `vue-tsc`/the e2e type-check flags)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (S-1's WorkerPanicked-row replacement and S-2 only; S-1's EmptyRawProperty row belongs to Task 4)

**Interfaces:**
- Consumes: Task 2's moved `run_batch` and tests (the compiler sweep covers them in their final home).
- Produces: the `panic` field on every serialized `JobOutcome` surface, which Task 6's e2e panic-render test consumes.
- Carries verbatim into Task 6: "the RunHistory export stays raw-output-only and the history table is unchanged; the ruled render surface is the live job row".

- [ ] **Step 1: the field (D98).** Add `panic: Option<String>` to `JobOutcome` with D98's doc comment verbatim; plainly serialized (no `skip_serializing_if`). `recover_panicked_worker` sets `panic: Some(message)` from the existing downcast chain, keeps `errors: vec![format!("{}: job {index}", DiagCode::WorkerPanicked.key())]` byte-identical, and deletes the `eprintln!` (locate by its literal text; `queue.rs:396` at authoring). Rewrite the function's licence doc block (the "core's one deliberate prose-free exception" passage) to state D98 fork 12's replacement rationale: the payload is now carried as data and rendered through the catalog at presentation time - the spec's normal path, not an exception; core still authors no user-facing prose (`core-37-prose-free-core`).
- [ ] **Step 2: the compiler sweep.** Build; every flagged existing `JobOutcome` constructor gains `panic: None`. `recover_panicked_worker` remains the only `Some`-setter among non-test code; the two new deliberately-`Some` fixtures are this task's CLI unit test and Task 6's e2e event (design round-2 note 1).
- [ ] **Step 3: the wire mirrors (D98's memo, complete):** `JobRecord` in `joblog.rs` gains `panic: Option<&'a str>` (`outcome.panic.as_deref()`); `src/ipc.ts` `JobOutcome` gains `panic: string | null` required (so `RunJobEntry extends JobOutcome` inherits it) and `JobLogRecord` gains the same; the smoke live-run fixture literals add `panic: null` (the type-check enumerates them - do not hunt by eye).
- [ ] **Step 4: CLI rendering (D99).** `render_finished`'s `JobState::Failed` arm branches on `outcome.panic` (typed, no string sniffing): `None` -> unchanged `run-job-failed`; `Some(detail)` -> line 1 `run-job-panicked`, line 2 the `worker-panicked` catalog message with `$detail`. Copy the four Fluent texts character for character from D99's four fences (en/de `diagnostics.ftl` replacement line; en/de `cli.ftl` new key next to the other `run-job-*` lines).
- [ ] **Step 5: catalog obligations (D99's list, complete):** `run-job-panicked` joins `ALLOWLISTED_CLI_KEYS` with fixture args `[("index", "1"), ("total", "3"), ("output", "/out/movie.mkv")]` in `allowlisted_cli_key_args`; the `DiagCode::WorkerPanicked` fixture row changes from `vec![]` to `vec![("detail", "queue worker thread panicked")]`.
- [ ] **Step 6: the pinned tests.** (a) Extend `worker_panic_is_reported_as_failed_not_cancelled` (queue.rs inline tests) to additionally assert the recovered outcome carries `panic: Some(..)` (the payload string) AND the unchanged `worker-panicked: job N` prefix token in `errors` - the acceptance-4 core assertion. (b) CLI: `outcome()` helper gains `panic: None`; new unit test `finished_panicked_renders_two_lines_without_na` in `commands/run.rs`'s test module, mirroring `finished_failed_renders_exit_code` with `panic: Some(..)`, asserting both lines render and neither contains `"n/a"`.
- [ ] **Step 7: GUI render (D100).** In `JobRow.vue`'s state cell: computed `panicDetail` (`state.kind === "finished" ? state.outcome.panic : null`) and the span exactly as D100's fence (`data-testid="job-panic"`, `$t("worker-panicked", { detail: panicDetail })`). Placement/styling inside the cell is implementer-owned (`latitude-carveout-presentation-tokens`); key, param, gating condition and testid are fixed. No `JobRowData` change; no history-table or RunHistory-export change.
- [ ] **Step 8: spec amendments** S-1 (the WorkerPanicked-row replacement text only) and S-2, exactly as the design's section 3 fences write them.
- [ ] **Step 9: verification.**
  - **Acceptance observable 4's task-3 emitters, as stated in the design's section 7 item 4** (the e2e emitter rides Task 6). The core-stdio absence check: `grep -rn "eprintln!\|println!\|print!(" crates/muxsmith-core/src` -> expected post-state EXACTLY one hit, the comment line in `lib.rs` reading "... `eprintln!` sites ..." - zero call sites. Fire: the same grep pre-edit returns exactly 2 hits (the comment + the `queue.rs` call; pasted in the authoring section). Control that pattern and pathspec produce output on a known-present case: the same grep over `crates/muxsmith-cli/src` hits six files (authoring run pasted).
  - `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (catalog placeholder-leak guard now proves template and params agree for `worker-panicked`; existing insta snapshots must NOT churn - the `panic: None` rendering path is byte-unchanged, so any snapshot diff is a defect).
  - `pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e` - green, foreground (the fixture sweep and the ipc.ts mirror are what these enforce).
- [ ] **Step 10: commit.**

```bash
git add crates/muxsmith-core/src/executor/job.rs crates/muxsmith-core/src/executor/queue.rs crates/muxsmith-core/src/executor/joblog.rs crates/muxsmith-cli/src/commands/run.rs crates/muxsmith-cli/tests/catalog_completeness.rs locales/en/diagnostics.ftl locales/de/diagnostics.ftl locales/en/cli.ftl locales/de/cli.ftl src/ipc.ts src/components/JobRow.vue e2e/smoke.spec.ts docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

plus any additional file the Step-2 compiler sweep touched (stage each by name; never `git add -A`), then:

```bash
git -c commit.gpgsign=false commit -m "executor+cli+gui: worker-panic payload travels as JobOutcome.panic and renders on both surfaces (D98-D100, S-1/S-2)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the field shape and always-serialized form; the byte-identical `errors` token; `delete_partial_failed` untouched (fork 9 - trigger-deferred, not forgotten); "the four silently-discarded executor failures (`job.rs` create_dir_all, `joblog.rs` remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded - the recorded steelman of `exec-36`'s ruled no-facade position; no task 'improves' them in passing" (design section 2; this task edits exactly the files where they live); no logging facade, no `log`/`tracing`; the four D99 Fluent texts and the D100 render semantics character for character; no other user-visible string changes.

---

## Task 4: `EmptyRawProperty` - bare `raw:` is a config-time error, with its Run-gate e2e scenario (D101; spec S-1 new row, S-3, S-5, S-6)

Read first: design D101 in full (both forks, the boundary paragraph, the accepted consequences, and the amendment-1 producer paragraph with its scenarios-in / infrastructure-out boundary); design section 5 (the raw bullets and the two-scenarios bullet); the authoring-time fixture probes and amendment-1 anchors above. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `crates/muxsmith-core/src/report/mod.rs` (`EmptyRawProperty` variant in `diag_codes!`, doc verbatim)
- Modify: `crates/muxsmith-core/src/profile/validate.rs` (`raw_opt_in_diagnostic` three-branch form verbatim)
- Modify: `locales/en/diagnostics.ftl`, `locales/de/diagnostics.ftl` (`empty-raw-property` line after `raw-on-known-property`, both locales, from D101's fences)
- Modify: `crates/muxsmith-cli/tests/catalog_completeness.rs` (`DiagCode::EmptyRawProperty => vec![]` fixture row)
- Modify: `crates/muxsmith-core/tests/validate_semantics.rs` (the two pinned per-arm tests, placed with the B-1..B-4 `raw:` block)
- Modify: `crates/muxsmith-cli/tests/cli_validate.rs` (the two pinned subprocess tests)
- Create: the two insta snapshot files those tests accept, named by the directory's existing `<test_file>__<test_fn>.snap` convention: `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap` and `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap`
- Modify: `e2e/smoke.spec.ts` (ONE addition: the amendment-1 Run-gate scenario in the `batch view: dry run` describe (`:140`); nothing else in the file - Task 3's fixture sweep and Task 5's apply scenario are those tasks' regions)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (S-1's EmptyRawProperty-row insertion, S-3, S-5, S-6)

**Interfaces:**
- Consumes: nothing from other tasks (deliberately: its fixtures use no Task-3/5 artifact; the e2e scenario's document is a mock and needs no Rust-side change beyond this task's own).
- Produces: the error-severity `EmptyRawProperty` diagnostics that any later mixed-severity fixture could rely on (Task 5's fixture deliberately does NOT, to keep the tasks decoupled).

- [ ] **Step 1: variant + funnel.** Add `EmptyRawProperty` to the `diag_codes!` block next to `RawProperty`/`RawOnKnownProperty` with D101's doc comment verbatim (key `empty-raw-property` follows from the macro; its serde/key equality test covers it automatically). Replace `raw_opt_in_diagnostic` with D101's three-branch fence verbatim. Both validate arms are covered by construction (the `exact` arm keeps its `continue`; the `substring`/`regex` arm keeps running the value-level regex compile check afterwards). No matcher or planner change; the boundary is exact emptiness (`bare == ""` only).
- [ ] **Step 2: locale lines + catalog row.** Copy the en/de `empty-raw-property` lines character for character from D101's fences, positioned after `raw-on-known-property` in each file; add the `DiagCode::EmptyRawProperty => vec![]` fixture row.
- [ ] **Step 3: the pinned core tests** (design D101's "Tests pinned by acceptance"), in `validate_semantics.rs` beside B-2/B-3:
  - `empty_bare_raw_exact_is_empty_raw_property_error`: profile rule `- match: { exact: { 'raw:': eng } }`; assert exactly one `EmptyRawProperty`, severity error, `config_path` `tracks[0].match.exact.raw:` (the authoring probe pins this path on the current tree).
  - `empty_bare_raw_substring_is_empty_raw_property_error`: `- match: { substring: { 'raw:': en } }`; path `tracks[0].match.substring.raw:`.
  - **The guard's control is the existing pair `b2_raw_unknown_exact_is_raw_property_info_untyped` / `b3_raw_unknown_substring_is_raw_property_info_no_type_error`** (non-empty `raw:` keys still yield `RawProperty` info): they must pass unchanged in the same run - the discriminating evidence that the new branch fires on emptiness, not on `raw:` generally. Do not write a duplicate control (reuse before writing).
- [ ] **Step 4: the pinned subprocess tests** (acceptance 5's emitter), in `cli_validate.rs`, following that file's documented snapshot idiom (insta, tempfile-written inline profile, like `warnings_only_exits_one`):
  - `bare_raw_property_exits_two_and_renders_the_message`: the exact profile of the authoring probe (`profile_version: 1`, `input: { pattern: 'E(\d+)', extensions: [mkv] }`, one rule `- match: { exact: { 'raw:': eng } }`); assert `.code(2)`; snapshot the stdout (the snapshot must contain the en text of D101's fence).
  - `bare_raw_property_renders_german_with_locale_flag`: same profile, args plus `--locale de`; `.code(2)`; snapshot (must contain the de text of D101's fence).
- [ ] **Step 5: the Run-gate e2e scenario** (amendment 1, ruling A - the feature's GUI consequence ships with its test). Implement D101's amendment-1 producer paragraph exactly as enumerated there: a new scenario in `e2e/smoke.spec.ts`'s `batch view: dry run` describe (`:140`), mocking `detect_mkvmerge`, the dialog open, and `validate_profile` resolving the design-fixed document (one `empty-raw-property` error-severity diagnostic, `mkvmerge_found: true`); assert `data-testid="batch-run"` disabled AND its `title` equals the localized `batch-run.tooltip-errors` text (`gui-batch.ftl:69`), which discriminates the errors reason from the missing-profile and missing-mkvmerge gates by construction. It is the paired negative of the enabled assertion at `smoke.spec.ts:511` (locator `:510`) - paired by assertion, not by location: that assertion sits in the `jobs view: live run` flow (`:477` describe, test `:491`). **What this test establishes (`a-new-test-says-whether-the-behavior-or-the-assertion-is-new`): the ASSERTION is new, not the behavior.** `hasErrors` gating exists today, and an error-severity document already reaches BatchView in the plural-counts test (`pluralReport`, `severity: "error"` at `:272-277`, fed via `dry_run` at `:315`) - but nothing anywhere asserts `batch-run` disabled (in `smoke.spec.ts` the only `toBeDisabled` is the editor Save, `:1241`; suite-wide the count is three, adding `editor-rule-add-remove.spec.ts:151`/`:317`, none of them on `batch-run`). The scenario therefore PASSES on today's tree; no red-today claim attaches to it. The ruled consequence is covered end to end by a two-link chain whose links are separate tests by construction, because the e2e mock supplies the severity by hand: this task's core and CLI tests prove a bare `raw:` now yields error severity (new behavior, red today), and this scenario proves an error-severity document disables the Run button (existing behavior, newly asserted). Flow, mocked commands, document contents and assertion targets are design-fixed - no implementer choices (design section 5, the two-scenarios bullet).
- [ ] **Step 6: spec amendments** S-1 (the new `EmptyRawProperty` row only), S-3, S-5, S-6, exactly as the design's section 3 fences write them.
- [ ] **Step 7: verification.** `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (new tests green; B-2/B-3 controls green; the catalog macro equality test and the placeholder-leak guard cover the new key/row); `pnpm check:i18n` (both locales stay in lockstep); `pnpm lint`; `pnpm test:e2e` (the new Run-gate scenario green, every pre-existing e2e suite unchanged). No absence-shaped check exists in this task; the exit-code flip that D101 accepts is asserted positively by the two `.code(2)` tests, whose red state is today's tree (the authoring probe measured exit 0 with an info diagnostic on the identical profile). The Run-gate scenario carries NO red-today claim: it passes on today's tree (Step 5 records why), and its value is the previously missing assertion plus regression protection on the gate.
- [ ] **Step 8: commit.**

```bash
git add crates/muxsmith-core/src/report/mod.rs crates/muxsmith-core/src/profile/validate.rs locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-cli/tests/catalog_completeness.rs crates/muxsmith-core/tests/validate_semantics.rs crates/muxsmith-cli/tests/cli_validate.rs crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap e2e/smoke.spec.ts docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "validate: bare raw: with an empty property name is an error, own DiagCode + Run-gate e2e (D101, S-1/S-3/S-5/S-6)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the three-branch funnel form; no per-call-site checks; no matcher/planner change; the two locale texts character for character; the Run-gate scenario exactly as enumerated in D101 - no scenario beyond it and no new test infrastructure (the amendment-1 boundary in Global Constraints).

---

## Task 5: Central errors-first sort + BatchView code-keyed fetch, with its parse-failure apply e2e scenario (D102, D103; spec S-7)

Read first: design D102 (site, stability contract, scope boundary, consumers sweep), D103 (evidence, rejected alternatives, and the amendment-1 producer paragraph), section 0 note 2; design section 5 (sort/fetch bullets and the two-scenarios bullet); the amendment-1 anchors above. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `crates/muxsmith-core/src/report/mod.rs` (`pub fn severity_sorted` with D102's doc; the new order unit test in its existing `#[cfg(test)]` module)
- Modify: `crates/muxsmith-core/src/report/json.rs` (both builders sort `config_diagnostics` before rendering; private `rendered_diag(d, renderer)` factored out and used by `rendered_diags` and the builders - no second rendering implementation)
- Modify: `crates/muxsmith-cli/src/commands/mod.rs` (the `pub(crate) fn severity_sorted` deleted; `pub(crate) use muxsmith_core::report::severity_sorted;` in its place; call sites compile unchanged)
- Modify: `crates/muxsmith-cli/tests/dry_run_cli.rs` (the pinned parity test)
- Modify: `src/views/BatchView.vue` (D103's `find` line verbatim)
- Modify: `e2e/smoke.spec.ts` (ONE addition: the amendment-1 parse-failure apply scenario in the `batch view: dry run` describe (`:140`), beside the apply-flow test (`:406`); nothing else in the file - Task 3's fixture sweep and Task 4's Run-gate scenario are those tasks' regions)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (S-7 only)

**Interfaces:**
- Consumes: nothing task-produced (its fixture is deliberately pre-Task-4-compatible, see below).
- Produces: sorted `config_diagnostics` in every builder-emitted document (GUI views and CLI `--json` alike).

- [ ] **Step 1: the hoist (D102).** Move the CLI's `severity_sorted` implementation to `report/mod.rs` as `pub fn severity_sorted(diags: &[Diagnostic]) -> Vec<&Diagnostic>` with D102's doc comment verbatim; replace the CLI definition with the re-export. Factor the per-diagnostic JSON mapping into a private `rendered_diag`; `batch_document` and `config_only_document` sort `config_diagnostics` only (never `files[].diagnostics`, never `batch_diagnostics` - D102's surfaced scope boundary), before rendering, not inside `rendered_diags`.
- [ ] **Step 2: the discriminating order test** (D102's pinned contract test), in `report/mod.rs`'s test module: `severity_sorted_orders_errors_first_stable_within_severity` - fixture vector `[info A, error B, warning C, error D]` must come back `[B, D, C, A]` (errors-first AND the preserved B-before-D tie order).
- [ ] **Step 3: the parity test** (acceptance 6's subprocess emitter), in `dry_run_cli.rs` beside `dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing` and reusing its `empty_path_dir` no-mkvmerge idiom (independent of `have_mkvmerge()`): `dry_run_and_validate_json_agree_on_config_diagnostics_ordering`. Profile, verbatim (the authoring probe's discriminating fixture - collection order is info, warning, error, error, so an unsorted side cannot pass):

```yaml
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { raw:x: 1 } }
    - match: { exact: { raw:language: de } }
    - match: { regex: { title: '[' } }
```

Run `validate --json` and `dry-run --json` (both with PATH pointed at the empty dir) on it; assert the `dry-run` document's `config_diagnostics` code sequence equals the `validate` envelope's `diagnostics` code sequence AND begins with the two error codes (`unknown-property`, `invalid-regex`, in that tie order - stability included). Expected sequence on the end state, from the authoring probe's sorted side: `unknown-property`, `invalid-regex`, `raw-on-known-property`, `raw-property`. This test is red on today's tree (the probe measured the dry-run side in collection order), which is its fire-verification.
- [ ] **Step 4: BatchView (D103).** Replace the positional fetch with D103's line verbatim: `const parseDiagnostic = doc.config_diagnostics.find((d) => d.code === "parse-error");` inside the existing `!doc.profile` branch; the existing else-branch `console.error` stays.
- [ ] **Step 5: the parse-failure apply e2e scenario** (amendment 1, ruling A - the discriminating test for this task's D103 change). Implement D103's amendment-1 producer paragraph exactly as enumerated there: a new scenario in `e2e/smoke.spec.ts`'s `batch view: dry run` describe (`:140`), replaying the existing apply flow (the `:406` test's scaffold) with the one substitution - `load_profile` resolves the design-fixed parse-failure document (`profile: null`, singleton `parse-error` error diagnostic). Assertions per the design: (a) the alert line contains "The profile could not be parsed" (the en text before the `$detail` placeable, so Fluent's directional isolates cannot break the match); (b) the recorded invoke log contains no `apply_suggestion` and no `save_profile` call. **What this test establishes (`a-new-test-says-whether-the-behavior-or-the-assertion-is-new`): the ASSERTION is new, not the behavior.** It PASSES on today's tree - the current `config_diagnostics[0]` fetch surfaces the same singleton `parse-error`, and the `!doc.profile` branch already returns before any apply or save (`BatchView.vue:218-227`, verified at amendment-fix time) - so no red-today claim attaches to it; its discriminating power targets a defective code-keyed rewrite (it goes red if the new `find` misses `parse-error`), plus regression protection on the branch. Flow, mocked commands, document contents and assertion targets are design-fixed - no implementer choices (design section 5, the two-scenarios bullet).
- [ ] **Step 6: spec amendment S-7** exactly as the design's section 3 fence writes it.
- [ ] **Step 7: verification.**
  - **Acceptance observable 6's emitters, as stated in the design's section 7 item 6** (including the new apply-scenario producer). The BatchView absence check: `grep -n 'config_diagnostics\[0\]' src/views/BatchView.vue` -> 0. Fire: the same grep pre-edit hits `:225` (authoring run pasted; also the file's only current hit, so the green state is the single replaced line - member-by-member trivial).
  - `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (the design's consumers sweep predicts existing CLI JSON tests pass unchanged - they assert membership, not position; a positional failure here is a real finding, not a test to relax: NEEDS_CONTEXT).
  - `pnpm lint`; `pnpm build`; `pnpm test:e2e` (the new apply scenario green; every PRE-EXISTING e2e suite passes unchanged - existing fixtures are mock documents that never pass through core, so outside this task's one added scenario any e2e diff is a defect signal).
- [ ] **Step 8: commit.**

```bash
git add crates/muxsmith-core/src/report/mod.rs crates/muxsmith-core/src/report/json.rs crates/muxsmith-cli/src/commands/mod.rs crates/muxsmith-cli/tests/dry_run_cli.rs src/views/BatchView.vue e2e/smoke.spec.ts docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "report: config_diagnostics sorts errors-first centrally; BatchView fetches parse-error by code + apply e2e (D102, D103, S-7)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): `severity_sorted`'s contract and exclusive application to `config_diagnostics` in the two builders; the re-export (no wrapper, no delegate); per-file and batch arrays stay unsorted; the `find` predicate keys on `"parse-error"` exactly (no severity-keyed fetch, no `[0]` fallback); the apply scenario exactly as enumerated in D103 - no scenario beyond it and no new test infrastructure (the amendment-1 boundary in Global Constraints).

---

## Task 6: The ruled D23 tests on the widened mount harness + the `name()` hoist (D104; amendment 2)

Read first: design D104 in full (harness enumeration, the four assertions, the deliberately-not-duplicated orderings 1 and 5, the `runActive`-not-passed rationale); the ROADMAP anchor's harness-scope correction paragraph; section 0 note 3; the ROADMAP trigger "A FOURTH e2e spec file needs the local `name()` helper" (`docs/ROADMAP.md:628` at amendment time - NOT FIRED, consumed EARLY by owner ruling; amendment 2 and the ROADMAP's own record carry the correction). Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `e2e/mount-entry.ts` (glob entry + `resolvePath` branch per D104's fence; `spec.props` into a ref; the `__muxsmithSetProps__` merge hook)
- Modify: `e2e/global.d.ts` (`__muxsmithSetProps__(partial: Record<string, unknown>): void` typed next to `__muxsmithMount__`)
- Create: `e2e/jobsview-reset.spec.ts`
- Modify: `e2e/i18n-en.ts` (gains the exported `name()` helper beside `en()`, amendment 2)
- Modify: `e2e/smoke.spec.ts` (ONE deletion + import line: the local `name()` copy and its doc comment, `:53-62` at amendment time; Tasks 3-5's regions are already committed when this task runs)
- Modify: `e2e/editor-markers.spec.ts` (same deletion + import; comment + copy at `:27-31`)
- Modify: `e2e/editor-rule-add-remove.spec.ts` (same deletion + import; comment + copy at `:39-43`)

**Interfaces:**
- Consumes: Task 3's `panic` wire field and `JobRow.vue` render (assertion 4).
- Produces: the D23 coverage half of the ruled item; the ledger half exists since `b4daed6`.
- Carries the ROADMAP scope-correction conditional: the reactive-props hook is controller-judged in-scope as mechanics the ruled test requires; **if the owner overturns that judgment, the spec drops its double-dispatch test and keeps the rest** - recorded here so the fallback needs no second planning round.

- [ ] **Step 1: harness widening.** Apply D104's `mount-entry.ts` fence verbatim (glob gains `../src/views/JobsView.vue`; `resolvePath` gains the `JobsView` branch); move `spec.props` into a `ref`, spread the render closure from the ref, and add `window.__muxsmithSetProps__(partial)` merging into it; type it in `global.d.ts`. `e2e/mount.ts` is NOT touched; existing mount specs never call the hook and stay green.
- [ ] **Step 2: the new spec**, `e2e/jobsview-reset.spec.ts`, composed exactly as D104 prescribes (`page.setContent` -> `addScriptTag` `e2e/.generated/tauri-mock-harness.js` -> `page.evaluate(installMockIPC, scenario)` -> `addScriptTag` `e2e/.generated/mount-harness.js` -> mount; the soft-outcome case installs its page-side handler via `window.__muxsmithE2E__.mockIPC` and emits `muxsmith://run-finished` through `window.__muxsmithE2E__.emit` BEFORE resolving `start_run`). Four tests, names fixed here, assertions per D104's numbered list 1-4:
  - `"fresh dispatch with a soft outcome keeps the finished summary"` (item 1)
  - `"fresh dispatch rejection renders the error and clears runActive"` (item 2, `rejectWith("run-already-active")`)
  - `"double dispatch against an active run does not wipe the live row"` (item 3, via `__muxsmithSetProps__`)
  - `"a finished event with a panic renders the worker-panicked message"` (item 4: outcome `state: "failed"`, `panic: "boom"`; assert the `job-panic` testid renders the localized `worker-panicked` text)
  `runActive` is deliberately not passed as a prop (D104's `defineModel` rationale); the internal transitions are asserted through the cancel-batch button's disabled state, as D104 fixes.
- [ ] **Step 3: the `name()` hoist** (amendment 2; the registered ROADMAP trigger's action, executed as an owner-ruled EARLY consumption - the trigger's condition, a fourth spec file that NEEDS the helper, is not met and this task does not meet it: `jobsview-reset` is the TENTH spec file (9 exist today) and consumes no `name()`; the owner judged three copies plus a new spec file the right moment rather than waiting for a fourth consumer that may never arrive). A pure move, behavior byte-identical:
  - Export the helper from `e2e/i18n-en.ts`, beside the `en()` it wraps (`:155`): `export function name(id: string, args?: Record<string, FluentVariable>): { name: string; exact: true } { return { name: en(id, args), exact: true }; }`. The `FluentVariable` type is already imported there (`:61`); nothing else is added.
  - **The rationale comment travels to the new home:** the doc comment on the export is smoke's copy verbatim (`smoke.spec.ts:53-59` - the "Run" / "Dry run" / `run-demo.yaml` substring-collision reason `exact: true` exists), not the two shorter mirror comments; losing that reason in the move is the real cost this step must not pay.
  - Delete all three local copies with their doc comments (`smoke.spec.ts:53-62`, `editor-markers.spec.ts:27-31`, `editor-rule-add-remove.spec.ts:39-43`; spans at amendment time - locate by the `function name(` line); add `name` to each file's existing `./i18n-en` import; drop each file's then-unused `import type { FluentVariable }` (in all three files the type's only use is the deleted helper signature - measured at amendment time, two hits per file - and `pnpm lint` enforces the cleanup).
  - **`e2e/jobsview-reset.spec.ts` does NOT consume the helper:** the D104 assertions target `data-testid` selectors (`cancel-batch`, `JobsView.vue:263`; `job-panic`, created by D100) and role-level queries without localized accessible-name matching, so the new spec imports nothing from `i18n-en.ts`'s name-matching surface - no fourth copy and no fourth consumer arises, which is exactly why the trigger's condition stays unmet; the hoist rests on the owner's early-consumption ruling, not on a fired trigger. If an assertion turns out on code contact to need a localized accessible-name match, the spec imports the SHARED helper - a local copy is a defect in every branch.
- [ ] **Step 4: verification.** `pnpm lint`; `pnpm test:e2e` green, foreground - the new spec's four tests pass AND every pre-existing e2e suite passes unchanged (the three migrated files unchanged in behavior is exactly the pure-move claim: same helper body, same call sites, only the definition's home moved). These tests codify already-adjudicated behavior (no code fix is in scope): **if any of the three ordering tests fails against the unmodified views, that contradicts the adjudicated premise and returns as NEEDS_CONTEXT with the failure pasted - it is never "fixed" at the keyboard, in the view or in the test.** One absence-shaped check (the hoist): `grep -rn "^function name(" e2e/*.spec.ts` -> 0. Fire: the same grep pre-edit hits exactly the three copies (`editor-markers.spec.ts:29`, `editor-rule-add-remove.spec.ts:41`, `smoke.spec.ts:60`; authoring run pasted in the amendment-2 anchors). Green state, member-by-member: the surviving definition reads `export function name(` in `e2e/i18n-en.ts` - excluded twice over, by the anchored pattern and by the `*.spec.ts` glob; presence control `grep -c "export function name(" e2e/i18n-en.ts` -> 1 (measured 0 pre-hoist, its red state). The harness hook's no-regression claim is carried by the existing mount specs passing in the same run.
- [ ] **Step 5: commit.**

```bash
git add e2e/mount-entry.ts e2e/global.d.ts e2e/jobsview-reset.spec.ts e2e/i18n-en.ts e2e/smoke.spec.ts e2e/editor-markers.spec.ts e2e/editor-rule-add-remove.spec.ts
git -c commit.gpgsign=false commit -m "e2e: JobsView mountable, reactive-props hook, the ruled D23 reset tests + panic render; hoist the shared name() helper (D104, amendment 2)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the glob entries, `resolvePath` branches, hook name, spec-local mock composition and the four assertions; `e2e/mount.ts` and the two OUT items untouched; no Vitest, no `tauri::test`, no `src-tauri/tests/`, no IpcError funnel. Amendment 2 adds: the hoist is a pure move - helper body byte-identical, home is `e2e/i18n-en.ts` beside `en()`, the smoke rationale comment travels verbatim, no local `name()` copy survives or is created anywhere. **This task does NOT write, edit or duplicate `gui-d23-reset-gating-form`** - the entry exists (commit `b4daed6`); a task writing it again is a duplicate-id defect.

---

## Task 7: The D49 G1/G2 removal experiment (D105)

Read first: design D105 in full - the protocol, decision rule and recording are design-fixed; this task transcribes and observes. Model tier: mid (live evaluation and anomaly recognition; plan-8.5 experiment-task precedent).

**Files (EXHAUSTIVE):** none. The mutation is applied and fully reverted inside the task; the tree ends byte-identical to its start. **This task writes NO ledger entry and NO repo file, and commits nothing** - the `core-d49-g1g2-experiment` entry is CONTROLLER-written at the plan close from this task's reported measurement, with the design-fixed branch text (D105 step 6). No implementer edits house YAML.

- [ ] **Step 1: green control** (`proc-check-green-state-reachable`): `cargo test -p muxsmith-core --test suggestions` on the unmutated tree; paste the pass.
- [ ] **Step 2: the exact mutation** per D105 step 2: in `delta_for`'s `AddExact` arm (`planner.rs:1820-1827` at authoring; locate by the `AddExact` match arm), `map.insert(property.clone(), value.clone());` becomes `map.insert(property.clone(), Scalar::Str(scalar_display(value)));`. Nothing else is touched.
- [ ] **Step 3: the suite invocation** again; record per guard (G1 `apply_splices_the_simulated_scalar_for_a_bool_property`, G2 `apply_splices_the_simulated_scalar_for_an_int_property`, G3 `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level`) whether it went red - pasted output.
- [ ] **Step 4: the decision rule, verbatim from D105 step 4** - all three fail -> load-bearing, stay for good; only G3 fails -> G1/G2 recorded as removal candidates (localizers); ANY other outcome is an anomaly -> no removal in any direction, NEEDS_CONTEXT to the controller with the pasted runs. In every branch the guards remain in the tree at this plan's end.
- [ ] **Step 5: restore and prove it.** Revert the mutation; `cargo test -p muxsmith-core --test suggestions` green again (the restore is fire-verified by Step 3's reds); `git status --porcelain` prints nothing and `git diff --stat` prints nothing (fire: during Step 2-3 the same commands show the `planner.rs` modification - paste both states).
- [ ] **Step 6: report.** The task report carries: the three pasted runs (control, mutated, restored), the per-guard red/green table, the selected branch of D105's decision rule, and the reminder that the controller writes `core-d49-g1g2-experiment` with that branch's design-fixed statement text. If the only-G3 branch was measured, the report also names ROADMAP trigger "Plan-9 design trigger 4" as now live for the owner's future ruling.

**Must not decide** (design section 5): the mutation, invocation, decision rule, anomaly routing, and the two ledger statements verbatim; no guard is removed in this plan.

---

## Plan close (controller actions, not tasks)

- **Entry condition:** Tasks 1-6 committed, Task 7 reported, working tree clean, and the **ten-part gate green, foreground, no subsets** (the pre-push site; Global Constraints list). Then the single push (SI-4; `gh-log.md` entry) and the push-triggered CI run green on the head SHA, including `ledger-lint`.
- **Whole-branch review** by an independent reviewer on the **top tier** (`proc-03-model-assignment`), against this plan, the plan brief, the design including its amendment log at execution state, the ROADMAP anchor and the spec - before any further close action.
- **Roll-up funnel** over every ledger-worthy and review-minor finding the task reports and reviews surfaced.
- **Promotion sweep of the FIVE owner-ruled entries** the ROADMAP anchor's recorded close action enumerates (`exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`, `cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`, `empty-bare-raw-property-rejected-at-validate`): promote into their nature files or record per entry why one stays Tier 1. **Premise verified for execution time (corrections table, row 2):** the exec-43 `source`-field gap found and routed at plan-authoring was satisfied by the controller on 2026-07-28 in commit `2155c1d` (`source: human`, `nature: technical-code`; recorded in `.superpowers/sdd/plan-9/progress.md` under "Post-authoring ground-truth edits") - all five entries carry `source: human`, and the promotion matrix applies directly, with no open routing.
- **`core-121-planner-seam-and-hoist`:** clear `blocked_on`, add the decided occurrence recording the Plan-9 design as the seam settlement (design section 4).
- **`core-d49-g1g2-experiment`:** write the entry with the design-fixed statement of the branch Task 7 measured (D105 step 6); update the consumed D49 ROADMAP trigger line, and if the only-G3 branch was measured, leave "Plan-9 design trigger 4" registered for the owner's ruling at a plan close.
- **ROADMAP bookkeeping:** mark the Plan-9 anchor executed (plan file path + close date); the IN items resolved against their commits. The `name()` trigger needs NO close action: the controller already recorded it on the trigger line itself at the amendment-2 fix - NOT FIRED (its condition, a fourth spec file that NEEDS the helper, was never met), consumed EARLY by owner ruling, executed by Task 6 Step 3 - with the ledger entry `a-triggers-condition-is-read-to-its-last-clause` carrying the lesson; the close only confirms that record stands. (No uncovered-consequence notes: under amendment 1 both former gaps ship with producers in this plan; the v1.x "GUI test harness for the run path" entry stays as it is, covering `start_run`'s composition.)
- **SDD salvage** of `.superpowers/sdd/plan-9/` per the standing salvage rule, with its `diff -r` re-check; the citation re-point sweep over the four house YAML files applies if any review round ledgered a plan-9 scratch basename.
- **Journal + HANDOFF snapshot** per the standing duty.

## Self-review (writing-plans skill duty, run at authoring)

Coverage: every design section D91-D105, all eight spec amendments (S-1..S-8, recounted after amendment 1 added S-8), both already-done items, all eight acceptance observables and both amendment-1 e2e producers appear in the coverage and acceptance maps with a named task or actor - walked section by section against the design on 2026-07-28, re-walked against the amended design at commit `064da74`. Placeholders: none (searched for TBD/TODO/"appropriate"/"similar to"; every string a task writes is either fenced in this plan or cited to a named design fence; the one derived value, the commit trailer's model name, has its derivation rule stated in Global Constraints). Counts recomputed at authoring and re-recomputed at amendment 1: 7 tasks, 15 D-entries, 8 spec amendments, 8 observables, 0 producer-less observables, 2 amendment-1 e2e scenarios, 5 promotion entries, 4 funnel-inline hits, 4 source-default hits, 2 core-stdio hits, 9 `MUXSMITH_RUNS_ROOT` lines, ten gate parts (6 Rust + 4 frontend); the plan nowhere states a seam parameter count - it cites the design's signature fence, which amendment 1 already re-counted to five - and a grep for the stale forms (`six parameter`, `cache: &mut`, `ident_cache`, `LiveIdentifier`, `command_integration`) over this plan returns only deliberate mentions: Task 1's per-call-cache wording and its non-change/verification/must-not lists, the amendment-1 note, and this sentence. Absence checks: each carries a fire (pasted authoring run or in-task pre-state run) and a reachable green state argued against enumerated survivors. Brief refutations: two, recorded in the corrections table with pasted evidence (five promotion entries, not six; `exec-43`'s missing `source` field - found at authoring, routed, and satisfied by the controller in commit `2155c1d`, so no routing is open at execution).

## Amendment 1 (2026-07-28, owner rulings after plan approval, before execution)

Routing: `.superpowers/sdd/plan-9/amendment-1-brief.md`; amended design at commit `064da74` (its round-3 amendment-log entry is the authoritative delta record). What moved in this plan, per ruling:

- **Ruling A (a feature's tests ship with the feature):** the two formerly producer-less observables gained design-enumerated e2e producers. The Run-gate scenario landed in **Task 4** and the parse-failure apply scenario in **Task 5** - in the tasks that own the features, not in the e2e task 6, because the ruling's own principle applies at task granularity: each task's verification proves its own user-visible consequence in the same change, rather than re-creating the tests-delivered-later shape one level down. `e2e/smoke.spec.ts` is now written by Tasks 3, 4 and 5 serially, each owning one named region (sequencing section). Acceptance rows 5/6, the coverage map, the Global-Constraints scenario boundary (scenarios-in / infrastructure-out) and the close actions follow.
- **Ruling B (no GUI identification session cache):** Task 1 lost the `LiveIdentifier` borrow change, the two `command_integration.rs` adaptations, the `AppState` field and all Arc/Mutex plumbing; its steps renumbered to seven, its files list shrank by two entries, and the seam is the amended five-parameter `plan_pipeline` constructing its own per-call `IdentifyCache` at S6. Spec amendment S-8 (spec 5.5's cache sentence) joins Task 1's spec edits beside S-4.

## Amendment 2 (2026-07-28, owner-ruled early trigger consumption, before execution)

The registered ROADMAP trigger "A FOURTH e2e spec file needs the local `name()` helper -> hoist it into a shared e2e helper module" (`docs/ROADMAP.md:628` at amendment time) **never fired, and this plan does not fire it**: its condition, read to its last clause, counts helper CONSUMERS, not spec files - `jobsview-reset.spec.ts` is the TENTH spec file (9 exist today, measured) and consumes no `name()`. The routing's first framing ("Task 6 creates the fourth spec file, the trigger's firing condition") was wrong on both counts; the amendment-2 fix round corrected it here, in the ROADMAP's own trigger record (NOT FIRED, consumed early) and in the ledger (`a-triggers-condition-is-read-to-its-last-clause`, verified present at `:4621` at fix time). The hoist stands anyway: the owner ruled it done NOW - an early consumption of the registered action, judged worthwhile with three copies plus a new spec file in hand rather than waiting for a fourth consumer that may never arrive. What moved in this plan:

- **The hoist landed in Task 6** (new Step 3; steps renumbered to five, files list grew from three to seven entries): Task 6 creates the new spec file whose arrival prompted the ruling, and under the smoke-region ownership scheme it is the last smoke-writer slot, so the hoist cannot race Tasks 3-5's already-committed regions - both placement reasons hold with no trigger fired. Home is `e2e/i18n-en.ts` beside the `en()` it wraps (no reason against it found: the module already imports `FluentVariable` and exports the sibling helpers); smoke's rationale comment (the substring-collision reason for `exact: true`) travels verbatim; the three local copies and their then-unused `FluentVariable` imports go; a pure move, `pnpm test:e2e` unchanged in behavior, with a fire-verified absence check on the spec glob.
- **`e2e/jobsview-reset.spec.ts` does not consume the helper** (its assertions are `data-testid`/role-based) - so no fourth consumer arises and the trigger's condition stays unmet even after this plan; if code contact proves a localized accessible-name match necessary, the spec imports the shared export, never a local copy.
- Sequencing ownership map updated (four smoke-writers, serial), the close actions note the trigger record already written controller-side, and the amendment-2 anchors joined the authoring-verification section with the pasted fire and count measurements.
