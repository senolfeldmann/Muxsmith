# Plan 9: core/orchestration hoists + planner seam

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **House deviation from the skill text:** progress NEVER enters this document. No box in this file is ever ticked; the checkbox syntax is structure, not a tracking surface. The tracker is `.superpowers/sdd/plan-9/progress.md`.
>
> **Execution starts only on the owner's plan approval** (standing gate; same rule the Plan-8.5 header carries).

**Goal:** execute the owner-approved Plan-9 design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`, D91-D105) exactly: hoist the four-copy planning pipeline into the core seam `pipeline::plan_pipeline` carrying the half-done funnel migration (D91-D95), hoist `run_batch` into core and delete the src-tauri runs-root seam (D96-D97), make the worker-panic payload travel as typed data and render on the CLI and the GUI job row (D98-D100), reject bare `raw:` at error severity (D101), sort `config_diagnostics` errors-first centrally and re-key BatchView's parse-diagnostic fetch (D102-D103), land the ruled D23 tests on a minimally extended mount harness (D104), and run the D49 G1/G2 removal experiment (D105). The seven spec amendments S-1..S-7 land with the tasks whose code they describe.

**Architecture:** seven strictly serial tasks on `master` in the main worktree - no branches, no worktrees (ruling and reasoning in the sequencing section). Tasks 1-6 each commit; Task 7 mutates and restores and leaves the tree byte-unchanged, so it commits nothing. One ten-part gate run, foreground, before the single push at the plan close (the gate's binding site is pre-push; this plan has no merges). No task edits any house-knowledge YAML; ledger writes are controller close actions.

**Tech Stack:** Rust workspace (toolchain pinned via `rust-toolchain.toml`, currently 1.96.1), Tauri 2 / Vue 3 / TypeScript frontend, Playwright e2e with the in-repo mock+mount harness (`e2e/`), Fluent catalogs under `locales/`, insta snapshots for CLI human output. No new dependency of any kind, cargo or npm; `tempfile` is already a core dev-dependency (used by `crates/muxsmith-core/tests/joblog.rs`).

## Global Constraints

- **Ground truth and precedence:** the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative on conflict; below it the Plan-9 design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`) is the executable contract - **its D-entries plus EVERY entry in its `## Amendment log` bind this plan, at the log's state at EXECUTION time, not at plan-authoring.** Membership is deliberately not enumerated here: the pointer IS the contract (house ruling, plan-8.5 Task-2 review round). The ROADMAP Plan-9 anchor (`docs/ROADMAP.md`) carries the owner's eight IN rulings, two OUT rulings, the recorded promotion close action and the harness-scope correction, all binding. The four house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`) are ground truth alongside them: cite entries by id; re-verify any `:line` before relying on it.
- **No design decision is re-opened, softened, or "improved".** The design's section 5 enumerates what the implementer must not decide; every task below inherits that list in full and names the entries that touch it. A contradiction discovered on code contact is refuted with evidence or returned, never silently absorbed.
- **Every fork in this plan is closed.** No task brief, verdict or fix-round dispatch may carry a design-latitude clause, in either form: an explicit permission, or an omission - an unenumerated set in a normative position, a list ending open, a "one per X" with no X list, a step that requires inventing a name, a string, or a file that is not written down somewhere the implementer can read (this plan or the design's own fences). A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** (options, costs against the named invariants, a recommendation) and is routed by the controller, never resolved at the keyboard (`proc-latitude-clause-boundary`).
- **Ten-part gate** per BUILDING.md ("The Rust gate" six parts + the four frontend checks; Tier-2 `gate-includes-cross-target-lint-for-the-unrun-os`), run foreground, no subsets, **before any push and after every merge**: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`; `cargo deny check`; `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`; `pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e`. This plan has no merges (serial commits on master); the gate's mandatory site is the pre-push run in the close actions. Per-task verification below names the subset each task must run green before committing; that subset is a task exit bar, not a gate substitute.
- **Pins:** no new runtime or product dependency in either ecosystem (cargo or npm); no GitHub workflow is modified by any task; nothing to SHA-pin.
- **SI-4 (restate in every dispatch that expects a commit; `dispatch-restates-the-standing-commit-grant`):** commits and pushes on this repo are standing-authorized by the owner; agent commits are deliberately unsigned - `git -c commit.gpgsign=false commit ...` - with exactly one trailer, `Co-Authored-By: Claude <model> <noreply@anthropic.com>`, where `<model>` is the canonical model name **derived from that dispatch's explicit model parameter, never written as a literal in this plan or a task brief** (`agent-commit-trailer-set`; no `Claude-Session` line, no context-window suffix). Stage files explicitly, **never `git add -A`**; every push gets a `gh-log.md` entry.
- **No task edits any house-knowledge YAML** (`docs/decision-ledger.yaml`, the three Tier-2 files). The controller is the single writer; a task that finds something ledger-worthy SURFACES it in its report. The spec and the design are documents this plan's tasks DO edit where a task below says so (S-1..S-7); nothing else under `docs/` is touched by any task.
- **No task creates a tag, publishes or edits a release, or resolves a README `placeholder(1.0)` comment.**
- **No new test scenarios beyond the ruled D23 tests (D104) and the tests the design's D-entries pin** (enumerated per task below). No Vitest, no `tauri::test`, no `src-tauri/tests/` directory, no IpcError funnel work; the two OUT items and `e2e/mount.ts` are untouched.
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
- **Other code anchors:** CLI `severity_sorted` defined at `commands/mod.rs:21`; `batch_document`/`config_only_document`/`rendered_diags` at `report/json.rs:35/:78/:176`; `run_document` at `report/json.rs:112`; `BatchView.vue:218` (`!doc.profile`) and `:225` (`config_diagnostics[0]`, the only such read in the file); `raw_opt_in_diagnostic` at `validate.rs:408-414` (two-branch form today) with the two arms at `:268`/`:310`; `"n/a"` literal at CLI `run.rs:487`; `delta_for`'s `AddExact` arm at `planner.rs:1820-1827` and `scalar_display` at `:856`; G1/G2/G3 at `suggestions.rs:1037/:1074/:1113`; `LiveIdentifier` non-pipeline constructions at `command_integration.rs:231/:493`; `AppState` at `lib.rs:89`, `load_profile_body`'s `Err` arm at `lib.rs:316-320`; `Diagnostic::error` exists (`report/mod.rs:242`); `report/mod.rs` has an inline `#[cfg(test)]` module (`:305`); `worker-panicked` current lines at `locales/en/diagnostics.ftl:80` / `locales/de/diagnostics.ftl:87`; the frontend bundles `diagnostics.ftl` (`src/i18n/index.ts:18` glob); `WorkerPanicked => vec![]` fixture row at `catalog_completeness.rs:152`, `ALLOWLISTED_CLI_KEYS` at `:182`; smoke live-run describe at `e2e/smoke.spec.ts:477`; `installMockIPC` exported at `e2e/mocks.ts:84`; harness bundles at `e2e/.generated/{tauri-mock-harness.js,mount-harness.js}` (`mocks.ts:24`, `mount.ts:14`).
- **Already done, confirmed (re-creating either is a defect):** `gui-d23-reset-gating-form` exists in `docs/decision-ledger.yaml` (`:4535` at authoring); the design's four triggers are mirrored in the ROADMAP Triggers section (the four lines tagged "Plan-9 design trigger 1..4"). `core-d49-g1g2-experiment` does NOT exist yet (grep over the ledger returns nothing; fired control: the same grep for `core-121-planner-seam-and-hoist` returns 1) - it is written by the CONTROLLER at the close from Task 7's measurement.
- **Fixture probes, run against the current `target/debug/muxsmith` (built from master):**
  - A profile rule `- match: { exact: { 'raw:': eng } }` parses (single-quoted YAML key), and `validate --json` today returns exactly one diagnostic: `raw-property`, severity `info`, `config_path` `tracks[0].match.exact.raw:`, `params.property` `""`, exit 0. The substring twin `- match: { substring: { 'raw:': en } }` returns the same at `tracks[0].match.substring.raw:`. This is the exact today-state D101 flips to error/exit 2, and it proves Task 4's fixture syntax and path assertions end to end.
  - The Task-5 parity fixture (rules in the order info, warning, error: `raw:x`, `raw:language`, `regex: { title: '[' }`) yields on the current tree: `validate --json` sorted `[unknown-property, invalid-regex, raw-on-known-property, raw-property]` (errors first), while `dry-run --json` (mkvmerge forced off PATH) yields collection order `[raw-property, raw-on-known-property, unknown-property, invalid-regex]`. The fixture therefore discriminates: the new parity test is red today and green exactly when D102 lands.

**Corrections to the brief found at plan-authoring** (`proc-57-briefs-not-ground-truth`; neither changes a ruling or a task):

| # | Brief statement | Reality |
|---|---|---|
| 1 | Close actions include "the promotion sweep of the six owner-ruled entries the ROADMAP anchor records" | **The anchor records FIVE.** The ROADMAP's recorded close action enumerates `exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`, `cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`, `empty-bare-raw-property-rejected-at-validate` and itself calls them "the five ledger entries these rulings touched" (recomputed from the enumeration: 5). The sixth the brief likely counted is `core-121-planner-seam-and-hoist`, whose `blocked_on` clearing the brief already lists as its own separate close action. The close actions below carry five plus the separate `core-121` action. |
| 2 | ROADMAP close action premise (rider found while verifying #1): the five entries "now all carry `source: human`" | **False for one member.** `exec-43-runsroot-debug-gated` carries NO `source` field at all (its fields run id, kind, tier, domain, statement, steelman, blocked_on, status, promoted_at, count, occurrences - checked in the entry body); the other four carry `source: human` as claimed. House YAML is controller-written, so no task touches this; it is ROUTED to the controller as part of the promotion close action (fix the field per the recorded ruling, or record why it stays), not resolved here. |

## Design coverage map

Every design section -> the task or actor that implements it. This is the walk the plan reviewer repeats; a row missing here is a defect.

| Design section | Implemented by |
|---|---|
| Section 0 (notes/corrections to the design brief) | Informational; note 3's surfaced harness deviation is executed by Task 6, note 5 by Task 3 (no `JobRowData` change) |
| Section 1 (SI-3 parity audit) | Informational; feeds D99/D100/D102, no separate work |
| D91 seam + four call-site mappings | Task 1 |
| D92 `mkvmerge_found` unified meaning, builder doc | Task 1 |
| D93 borrowed cache + GUI session cache | Task 1 |
| D94 `job_specs`, per-surface empty-specs branch | Task 1 (derivation + both gates); end-state queue call lands in Task 2 |
| D95 one `"."` default in the seam | Task 1 |
| D96 `run_batch` hoist + two tests move | Task 2 |
| D97 runs-root deletion, three call sites | Task 2 |
| D98 `JobOutcome.panic`, eprintln deletion, wire mirrors | Task 3 |
| D99 CLI two-line panic rendering + catalog obligations | Task 3 |
| D100 GUI job-row render | Task 3 |
| D101 `EmptyRawProperty` + pinned tests | Task 4 |
| D102 `severity_sorted` hoist + central sort | Task 5 |
| D103 BatchView code-keyed fetch | Task 5 |
| D104 harness widening + `e2e/jobsview-reset.spec.ts` (test half of the ruled item) | Task 6 (the ledger half is already written, commit `b4daed6` - no task touches it) |
| D105 D49 experiment protocol | Task 7 (measurement + report); ledger entry is a controller close action |
| Section 2 (removal recount; the four discarded executor failures STAY discarded) | Constraint carried verbatim in Tasks 2 and 3 |
| Section 3 spec amendments | S-4 -> Task 1; S-1 WorkerPanicked-row replacement + S-2 -> Task 3; S-1 EmptyRawProperty-row insertion + S-3 + S-5 + S-6 -> Task 4; S-7 -> Task 5 |
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
- 4, 5: no hard data edge to 3 or to each other, but they share the spec document (S-edits) and the CLI test crate with their neighbours; ordered by D-number.
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
| 5. Empty bare `raw:` | Task 4 (two per-arm tests + existing B-2/B-3 controls, subprocess en/de tests, catalog row) | yes |
| 6. Central sort + BatchView | Task 5 (discriminating order test, CLI parity subprocess test, BatchView grep) | yes |
| 7. D23 item | Task 6 (the three ordering tests + panic-render test under `pnpm test:e2e`); the ledger half already exists (`b4daed6`) and `scripts/ledger-lint.py` runs in CI/at the gate | yes |
| 8. D49 experiment | Task 7 (recorded green-mutate-restore run with pasted outputs) + controller close (the `core-d49-g1g2-experiment` entry) | yes (the measurement); the entry is a close artifact |

**Two consequences have NO producer, by design, and are carried here as named, uncovered consequences - not claimed, not dropped:**

1. **The GUI Run-gate consequence of D101's new error severity** (a bare-`raw:` profile can no longer start a GUI run; editor Save gates likewise): no e2e feeds BatchView an error-severity config diagnostic today and this plan adds none. Rides the v1.x "GUI test harness for the run path" ROADMAP item.
2. **The branch D103 edits** (BatchView's `!doc.profile` + parse-diagnostic fetch): no e2e scenario resolves `load_profile` with a parse-error document, and this plan adds none. Correctness rests on the design's singleton-envelope evidence; coverage rides the same v1.x item.

---

## Task 1: The planner seam - `pipeline.rs`, four call-site migrations, the session cache (D91, D92, D93, D94, D95; spec S-4)

Read first: design D91-D95 in full, including D91's divergence table and the four call-site mappings; design section 5 (the seam bullets); spec sections 5.5 and 7. Model tier: mid.

**Files (EXHAUSTIVE):**
- Create: `crates/muxsmith-core/src/pipeline.rs`
- Modify: `crates/muxsmith-core/src/lib.rs` (module declaration only)
- Modify: `crates/muxsmith-core/src/identify.rs` (`LiveIdentifier` borrow change, D93)
- Modify: `crates/muxsmith-core/tests/command_integration.rs` (the two constructions at `:231`/`:493`)
- Modify: `crates/muxsmith-cli/src/commands/dry_run.rs` (call-site mapping 1)
- Modify: `crates/muxsmith-cli/src/commands/run.rs` (call-site mapping 2; the inline queue block is NOT touched)
- Modify: `src-tauri/src/lib.rs` (call-site mapping 3; `AppState.ident_cache`; the `dry_run` wrapper's Arc clone)
- Modify: `src-tauri/src/run.rs` (call-site mapping 4; `plan_run`'s Arc clone)
- Modify: `crates/muxsmith-core/src/report/json.rs` (D92: the `config_only_document` doc comment at `:66-73` only)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (amendment S-4 only)

**Interfaces:**
- Consumes: nothing from other tasks.
- Produces: `pipeline::{plan_pipeline, PipelineOutcome, PlannedPipeline, job_specs}` and the borrowed-cache `LiveIdentifier`, which Tasks 2-6 build on.
- Carries verbatim into Task 2: "the inline CLI queue block (`mpsc::channel` -> scope -> `expect(\"queue worker thread panicked\")`) is retained byte-unchanged by Task 1 and replaced only by Task 2."

- [ ] **Step 1: the core module.** Create `crates/muxsmith-core/src/pipeline.rs` with `PipelineOutcome`, `PlannedPipeline`, `plan_pipeline` and `job_specs` exactly as the design's D91 and D94 fences write them (types, fields, signatures, doc comments character for character; rustdoc line-wrapping is the only permitted difference). Internal step order is exactly S1-S7 as D91 fixes it, including the funnel call `profile::validate::config_diagnostics(&profile)` at S2 and the single `source.unwrap_or_else(|| PathBuf::from("."))` at S5 (D95). The near-verbatim rationale comments duplicated between the CLI copies consolidate into this module's docs; the per-branch S3e/S4e rationale moves onto the enum variants (already present in the D91 fence's variant docs); the D95 note ("No natural 'current directory' for a bundled desktop app ...") moves verbatim onto the seam's source-default doc. Declare `pub mod pipeline;` in `crates/muxsmith-core/src/lib.rs`.
- [ ] **Step 2: the borrow change (D93).** `LiveIdentifier` in `crates/muxsmith-core/src/identify.rs` becomes the borrowing form of D93's fence. Adapt the two non-pipeline constructions the design enumerates (`command_integration.rs:231` and `:493`): `let mut cache = IdentifyCache::new();` then `cache: &mut cache`. The compiler enforces completeness of this sweep.
- [ ] **Step 3: call sites 1 and 2 (CLI).** Migrate `dry_run.rs::run` and `run.rs::run` per D91's mappings 1 and 2: each builds one local `IdentifyCache`, calls `plan_pipeline(profile_path, source, output, on_collision, Mkvmerge::locate, &mut cache)` (mapping 1 passes the CLI's `--on-collision` value; both CLI sites pass `Mkvmerge::locate` per D-1), and maps each `PipelineOutcome` variant to exactly today's printed bytes, stderr lines and exit codes. `run.rs` then derives `pipeline::job_specs(&batch)` (D94) and keeps its empty-specs presentation and the entire inline queue block byte-unchanged (Task 2 owns that block).
- [ ] **Step 4: call sites 3 and 4 (GUI).** Migrate `dry_run_body` (src-tauri `lib.rs`) and `plan_run` (src-tauri `run.rs`) per D91's mappings 3 and 4: detect closure `|| Mkvmerge::detect(mkvmerge_override)`, `on_collision: None`, `plan_run` keeps its settings-read-first (D-7 stays caller-side, including the wrapper-vs-body layer asymmetry - deliberately not unified). `plan_run`'s empty-specs mapping to `PlanOutcome::Soft` gates on `pipeline::job_specs(&batch).is_empty()` exactly as today's derivation did (D94).
- [ ] **Step 5: the session cache (D93).** `AppState` gains `ident_cache: Arc<Mutex<IdentifyCache>>`; the `dry_run` command wrapper and `plan_run` each clone the `Arc` before entering `spawn_blocking` and lock it inside, passing `&mut *guard`. The lock is never held across an `.await` on the dispatch thread; the cancel/close paths lock `active`, never this mutex (D93's disjoint-mutex invariant - restate it in the code comment at the new field).
- [ ] **Step 6: D92 doc.** Update the `config_only_document` builder doc (`report/json.rs:66-73`) to carry D92's contract sentences verbatim: `false` = the surface's resolver produced no usable mkvmerge for planning (CLI: PATH lookup failed or the found binary could not answer `--version`; GUI: the detect ladder failed for any reason including `TooOld`); `true` = a usable mkvmerge resolved and the subsequent query failed; key absent on a profile-load failure. No wire change of any kind (D92: no new field, no reason on `MkvmergeUnavailable`).
- [ ] **Step 7: spec amendment S-4.** Insert the `pipeline` module-table row exactly as the design's section 3 S-4 fence writes it.
- [ ] **Step 8: verification.**
  - **Acceptance observable 1, exactly as stated in the design's section 7 item 1** - run its grep with its recorded fire-verification (pre-edit it must hit exactly the four sites; the authoring-time run above measured those same four). Reachable green, argued member-by-member: post-migration the surviving `lint::provable_overlaps` references are the funnel's own call in `validate.rs` (receiver `diags`, which the qualified pattern `config_diags.extend(` cannot match) and any prose mention in `pipeline.rs`'s consolidated docs, which must not restate the qualified call text (write the doc so it does not; the check then passes on the real end state, not vacuously).
  - **D95 single-default check:** `grep -rn 'PathBuf::from(".")' crates/muxsmith-cli/src src-tauri/src` -> 0. Fire: the same grep pre-edit -> exactly the 4 hits pasted in the authoring section. Green state: the one surviving instance lives in `crates/muxsmith-core/src/pipeline.rs`, outside this grep's pathspec by construction; presence control `grep -c 'PathBuf::from(".")' crates/muxsmith-core/src/pipeline.rs` -> 1.
  - `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` - all green, foreground. The CLI subprocess suites (`dry_run_cli.rs`, `cli_validate.rs`, `run_cli.rs`, `run_live.rs`) and src-tauri inline tests pass UNCHANGED (byte-behavior preservation; any test edit beyond the two Step-2 constructions is a defect signal -> NEEDS_CONTEXT, not a local fix).
- [ ] **Step 9: commit.**

```bash
git add crates/muxsmith-core/src/pipeline.rs crates/muxsmith-core/src/lib.rs crates/muxsmith-core/src/identify.rs crates/muxsmith-core/tests/command_integration.rs crates/muxsmith-cli/src/commands/dry_run.rs crates/muxsmith-cli/src/commands/run.rs src-tauri/src/lib.rs src-tauri/src/run.rs crates/muxsmith-core/src/report/json.rs docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "core: hoist the four-copy planning pipeline into the plan_pipeline seam (D91-D95, S-4)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the seam's module, signature, shapes and the four mappings - no additional parameter, no builder, no trait; the resolver closures; no new document field (D92); the cache placement and locking (D93); `job_specs`' home and doc (D94); the `"."` default exactly once (D95).

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
- Carries verbatim into Task 6: "the RunHistory export stays raw-output-only and the history table is unchanged; the ruled render surface is the live job row"; and the Task-2 "stay discarded" constraint above.

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

**Must not decide** (design section 5): the field shape and always-serialized form; the byte-identical `errors` token; `delete_partial_failed` untouched (fork 9 - trigger-deferred, not forgotten); no logging facade, no `log`/`tracing`; the four D99 Fluent texts and the D100 render semantics character for character; no other user-visible string changes.

---

## Task 4: `EmptyRawProperty` - bare `raw:` is a config-time error (D101; spec S-1 new row, S-3, S-5, S-6)

Read first: design D101 in full (both forks, the boundary paragraph, the accepted consequences); design section 5 (the raw bullets); the authoring-time fixture probes above. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `crates/muxsmith-core/src/report/mod.rs` (`EmptyRawProperty` variant in `diag_codes!`, doc verbatim)
- Modify: `crates/muxsmith-core/src/profile/validate.rs` (`raw_opt_in_diagnostic` three-branch form verbatim)
- Modify: `locales/en/diagnostics.ftl`, `locales/de/diagnostics.ftl` (`empty-raw-property` line after `raw-on-known-property`, both locales, from D101's fences)
- Modify: `crates/muxsmith-cli/tests/catalog_completeness.rs` (`DiagCode::EmptyRawProperty => vec![]` fixture row)
- Modify: `crates/muxsmith-core/tests/validate_semantics.rs` (the two pinned per-arm tests, placed with the B-1..B-4 `raw:` block)
- Modify: `crates/muxsmith-cli/tests/cli_validate.rs` (the two pinned subprocess tests)
- Create: the two insta snapshot files those tests accept, under `crates/muxsmith-cli/tests/snapshots/`
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (S-1's EmptyRawProperty-row insertion, S-3, S-5, S-6)

**Interfaces:**
- Consumes: nothing from other tasks (deliberately: its fixtures use no Task-3/5 artifact).
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
- [ ] **Step 5: spec amendments** S-1 (the new `EmptyRawProperty` row only), S-3, S-5, S-6, exactly as the design's section 3 fences write them.
- [ ] **Step 6: verification.** `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (new tests green; B-2/B-3 controls green; the catalog macro equality test and the placeholder-leak guard cover the new key/row); `pnpm check:i18n` (both locales stay in lockstep). No absence-shaped check exists in this task; the exit-code flip that D101 accepts is asserted positively by the two `.code(2)` tests, whose red state is today's tree (the authoring probe measured exit 0 with an info diagnostic on the identical profile).
- [ ] **Step 7: commit.**

```bash
git add crates/muxsmith-core/src/report/mod.rs crates/muxsmith-core/src/profile/validate.rs locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-cli/tests/catalog_completeness.rs crates/muxsmith-core/tests/validate_semantics.rs crates/muxsmith-cli/tests/cli_validate.rs crates/muxsmith-cli/tests/snapshots docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "validate: bare raw: with an empty property name is an error, own DiagCode (D101, S-1/S-3/S-5/S-6)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the three-branch funnel form; no per-call-site checks; no matcher/planner change; the two locale texts character for character. The GUI Run-gate consequence is a named, uncovered consequence (acceptance map above) - this task neither tests it nor mentions a producer for it.

---

## Task 5: Central errors-first sort + BatchView code-keyed fetch (D102, D103; spec S-7)

Read first: design D102 (site, stability contract, scope boundary, consumers sweep), D103 (evidence and rejected alternatives), section 0 note 2; design section 5 (sort/fetch bullets). Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `crates/muxsmith-core/src/report/mod.rs` (`pub fn severity_sorted` with D102's doc; the new order unit test in its existing `#[cfg(test)]` module)
- Modify: `crates/muxsmith-core/src/report/json.rs` (both builders sort `config_diagnostics` before rendering; private `rendered_diag(d, renderer)` factored out and used by `rendered_diags` and the builders - no second rendering implementation)
- Modify: `crates/muxsmith-cli/src/commands/mod.rs` (the `pub(crate) fn severity_sorted` deleted; `pub(crate) use muxsmith_core::report::severity_sorted;` in its place; call sites compile unchanged)
- Modify: `crates/muxsmith-cli/tests/dry_run_cli.rs` (the pinned parity test)
- Modify: `src/views/BatchView.vue` (D103's `find` line verbatim)
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
- [ ] **Step 5: spec amendment S-7** exactly as the design's section 3 fence writes it.
- [ ] **Step 6: verification.**
  - **Acceptance observable 6's produced emitters, as stated in the design's section 7 item 6.** The BatchView absence check: `grep -n 'config_diagnostics\[0\]' src/views/BatchView.vue` -> 0. Fire: the same grep pre-edit hits `:225` (authoring run pasted; also the file's only current hit, so the green state is the single replaced line - member-by-member trivial).
  - `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (the design's consumers sweep predicts existing CLI JSON tests pass unchanged - they assert membership, not position; a positional failure here is a real finding, not a test to relax: NEEDS_CONTEXT).
  - `pnpm lint`; `pnpm build`; `pnpm test:e2e` (e2e fixtures are mock documents that never pass through core and are unaffected - any e2e diff is a defect signal).
- [ ] **Step 7: commit.**

```bash
git add crates/muxsmith-core/src/report/mod.rs crates/muxsmith-core/src/report/json.rs crates/muxsmith-cli/src/commands/mod.rs crates/muxsmith-cli/tests/dry_run_cli.rs src/views/BatchView.vue docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "report: config_diagnostics sorts errors-first centrally; BatchView fetches parse-error by code (D102, D103, S-7)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): `severity_sorted`'s contract and exclusive application to `config_diagnostics` in the two builders; the re-export (no wrapper, no delegate); per-file and batch arrays stay unsorted; the `find` predicate keys on `"parse-error"` exactly (no severity-keyed fetch, no `[0]` fallback). The D103 branch's missing e2e producer is a named, uncovered consequence (acceptance map above) - no producer is invented.

---

## Task 6: The ruled D23 tests on the widened mount harness (D104)

Read first: design D104 in full (harness enumeration, the four assertions, the deliberately-not-duplicated orderings 1 and 5, the `runActive`-not-passed rationale); the ROADMAP anchor's harness-scope correction paragraph; section 0 note 3. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `e2e/mount-entry.ts` (glob entry + `resolvePath` branch per D104's fence; `spec.props` into a ref; the `__muxsmithSetProps__` merge hook)
- Modify: `e2e/global.d.ts` (`__muxsmithSetProps__(partial: Record<string, unknown>): void` typed next to `__muxsmithMount__`)
- Create: `e2e/jobsview-reset.spec.ts`

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
- [ ] **Step 3: verification.** `pnpm lint`; `pnpm test:e2e` green, foreground - the new spec's four tests pass AND every pre-existing e2e suite passes unchanged. These tests codify already-adjudicated behavior (no code fix is in scope): **if any of the three ordering tests fails against the unmodified views, that contradicts the adjudicated premise and returns as NEEDS_CONTEXT with the failure pasted - it is never "fixed" at the keyboard, in the view or in the test.** No absence-shaped check exists in this task; the harness hook's no-regression claim is carried by the existing mount specs passing in the same run.
- [ ] **Step 4: commit.**

```bash
git add e2e/mount-entry.ts e2e/global.d.ts e2e/jobsview-reset.spec.ts
git -c commit.gpgsign=false commit -m "e2e: JobsView mountable, reactive-props hook, the ruled D23 reset tests + panic render (D104)"
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide** (design section 5): the glob entries, `resolvePath` branches, hook name, spec-local mock composition and the four assertions; `e2e/mount.ts` and the two OUT items untouched; no Vitest, no `tauri::test`, no `src-tauri/tests/`, no IpcError funnel. **This task does NOT write, edit or duplicate `gui-d23-reset-gating-form`** - the entry exists (commit `b4daed6`); a task writing it again is a duplicate-id defect.

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
- **Promotion sweep of the FIVE owner-ruled entries** the ROADMAP anchor's recorded close action enumerates (`exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`, `cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`, `empty-bare-raw-property-rejected-at-validate`): promote into their nature files or record per entry why one stays Tier 1. **Routed discrepancy from plan-authoring (corrections table, row 2):** `exec-43` carries no `source` field although the anchor's close action asserts `source: human` for all five - the controller adds the field per the recorded ruling or records the deviation, before the matrix is applied.
- **`core-121-planner-seam-and-hoist`:** clear `blocked_on`, add the decided occurrence recording the Plan-9 design as the seam settlement (design section 4).
- **`core-d49-g1g2-experiment`:** write the entry with the design-fixed statement of the branch Task 7 measured (D105 step 6); update the consumed D49 ROADMAP trigger line, and if the only-G3 branch was measured, leave "Plan-9 design trigger 4" registered for the owner's ruling at a plan close.
- **ROADMAP bookkeeping:** mark the Plan-9 anchor executed (plan file path + close date); the IN items resolved against their commits; the two uncovered consequences (acceptance map) noted on the v1.x "GUI test harness for the run path" entry so they are findable there.
- **SDD salvage** of `.superpowers/sdd/plan-9/` per the standing salvage rule, with its `diff -r` re-check; the citation re-point sweep over the four house YAML files applies if any review round ledgered a plan-9 scratch basename.
- **Journal + HANDOFF snapshot** per the standing duty.

## Self-review (writing-plans skill duty, run at authoring)

Coverage: every design section D91-D105, all seven spec amendments, both already-done items, all eight acceptance observables and both uncovered consequences appear in the coverage and acceptance maps with a named task or actor - walked section by section against the design on 2026-07-28. Placeholders: none (searched for TBD/TODO/"appropriate"/"similar to"; every string a task writes is either fenced in this plan or cited to a named design fence; the one derived value, the commit trailer's model name, has its derivation rule stated in Global Constraints). Counts recomputed at authoring: 7 tasks, 15 D-entries, 8 observables, 2 uncovered consequences, 5 promotion entries, 4 funnel-inline hits, 4 source-default hits, 2 core-stdio hits, 9 `MUXSMITH_RUNS_ROOT` lines, ten gate parts (6 Rust + 4 frontend). Absence checks: each carries a fire (pasted authoring run or in-task pre-state run) and a reachable green state argued against enumerated survivors. Brief refutations: two, recorded in the corrections table with pasted evidence (five promotion entries, not six; `exec-43`'s missing `source` field routed to the controller).
