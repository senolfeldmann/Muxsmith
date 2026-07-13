# Idiomacy review - cross-cutting DUPLICATION sweep (find-X1)

Scope: whole tree at HEAD, one dimension only (dup: near-duplicate reimplementations).
Method: module map -> line counts -> full read of every claimed pair (all quoted line
ranges below were read in this session, none pattern-matched). Crates covered:
muxsmith-core, muxsmith-cli, xtask, src-tauri; TS side: src/, e2e/, scripts/.

## Findings (ranked)

### X1-1 (dup, high): the planning pipeline exists in FOUR copies

Sites, all read in full:

1. `crates/muxsmith-cli/src/commands/dry_run.rs:38-120` (`run`)
2. `crates/muxsmith-cli/src/commands/run.rs:60-192` (`run`, its own doc: "identical to
   `dry-run` through `plan_batch`")
3. `src-tauri/src/lib.rs:188-232` (`dry_run_body`)
4. `src-tauri/src/run.rs:249-350` (`plan_run`)

Each repeats the same skeleton: `load::from_file` -> `validate::validate` +
`lint::provable_overlaps` -> resolve mkvmerge (`locate` CLI / `detect(override)` GUI) ->
`list_languages` -> `RunInputs` with the `PathBuf::from(".")` source default ->
`LiveIdentifier { IdentifyCache::new(), &mkv }` -> `plan_batch`; plus the three identical
soft-failure branches (load failed -> `config_only_document(..., None, ...)`; mkvmerge
missing -> `Some(false)`; query failed -> `Some(true)`); plus (copies 2 and 4) the same
`filter_map(|f| f.plan.as_ref())` -> `JobSpec { argv: command(p), output }` gate.

The spec itself mandates the copies stay behaviorally identical (spec 5.5 "identical to
dry-run through plan_batch"; spec 7 "neither owns logic") - which is the strongest
argument for one shared implementation instead of four mirrors kept in sync by comments.
The report/json documents were already hoisted for exactly this reason (Plan 5 T2);
orchestration was left behind. ROADMAP notes the injectable-planner-seam question for
`start_run` as "never-decided", so no recorded decision blocks this.

Replacement: one core function, e.g.
`planner::plan_pipeline(profile_path, resolve_mkvmerge: impl FnOnce() -> Result<Mkvmerge, _>, run_inputs) -> PipelineOutcome`
with `PipelineOutcome::{LoadFailed(Diagnostic), ConfigOnly{config_diags, mkvmerge_found: Option<bool>}, Planned{config_diags, batch, specs}}`;
each surface maps outcomes to its own presentation (CLI: human/json print + exit code;
Tauri: `run_document` wrapping / `PlanOutcome`). The locate-vs-detect split and the CLI's
`on_collision` stay caller-side parameters, so no recorded behavior changes.

Estimated net lines cut: ~100 (four ~40-75-line code skeletons -> one ~70-line helper +
thin per-surface mapping).

### X1-2 (dup, medium): the run_queue drain plumbing is duplicated across crates

- `crates/muxsmith-cli/src/commands/run.rs:231-262`: mpsc channel + `thread::scope` +
  spawn `run_queue` + drain loop teeing `logger.on_event(&event)` + per-event callback +
  `handle.join().expect("queue worker thread panicked")`.
- `src-tauri/src/run.rs:808-829` (`run_batch`): the identical construct, already factored
  as a generic `fn run_batch(specs, spawner, opts, ctl, logger, on_event) -> (Vec<JobOutcome>, Option<RunLogger>)`.

The Tauri version is exactly the shape the CLI needs (its milestone rendering and
`--json` gating fit the `on_event: impl FnMut(&JobEvent)` closure). Replacement: hoist
`run_batch` into `muxsmith_core::executor` (beside `run_queue`) and have both surfaces
call it. Estimated net lines cut: ~20.

### X1-3 (dup, medium): config-time diagnostics collection duplicated verbatim

- `crates/muxsmith-cli/src/commands/validate.rs:47-56` (`collect`)
- `src-tauri/src/lib.rs:159-167` (`validate_profile_body`'s diags block)

Both: `match load::from_file(path) { Err(d) => vec![d], Ok(profile) => { validate::validate + extend(lint::provable_overlaps) } }`.
The two-line `validate + lint` pair additionally appears at the four X1-1 sites (covered
there). Replacement: `profile::validate::config_diagnostics_from_file(path) -> Vec<Diagnostic>`
(and/or `config_diagnostics(&Profile)`) in core; both bodies become one call. This is the
drift-critical one: a future second lint must currently be added at six call sites.
Estimated net lines cut: ~8.

### X1-4 (dup, medium): runs-root resolution (incl. debug-only env seam) duplicated

- `crates/muxsmith-cli/src/commands/run.rs:314-335` (`create_logger`'s resolution half)
- `src-tauri/src/run.rs:853-864` (`resolve_runs_root`, its doc: "see the CLI's identical
  gate for the rationale")

Both: `#[cfg(debug_assertions)] env::var_os("MUXSMITH_RUNS_ROOT").map(PathBuf::from).or_else(default_runs_root)`,
release: `default_runs_root()`. NOT a challenge to D26's debug-only decision (recorded
non-finding) - only to the mechanism existing twice. Replacement: hoist as
`joblog::resolve_runs_root()` next to `default_runs_root` in core; both callers call it
(workspace dev/release profiles apply to core identically, so `cfg(debug_assertions)`
semantics are unchanged). Estimated net lines cut: ~8.

### X1-5 (dup, low): five copies of the spawn_blocking + internal-task-failed wrapper

`src-tauri/src/lib.rs:296-301, 312-331, 337-349, 367-376` and `src-tauri/src/run.rs:447-451`:
each async command repeats `tauri::async_runtime::spawn_blocking(move || ...).await
.map_err(|e| IpcError::new("internal-task-failed").with("detail", e.to_string()))`.
Replacement: one `async fn on_blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T, IpcError> + Send + 'static) -> Result<T, IpcError>`
in `error.rs` or `lib.rs`; each command keeps only its own closure body. Estimated net
lines cut: ~12.

### X1-6 (dup, low): CLI validate.rs re-implements three helpers that exist shared

`crates/muxsmith-cli/src/commands/validate.rs`:

- line 19: inline `sort_by_key(Reverse(d.severity))` duplicates `commands::severity_sorted`
  (`commands/mod.rs:21-25`).
- lines 27-34: inline per-diagnostic `serde_json::to_value(d)` + `v["rendered"] = ...`
  duplicates `report::json::rendered_diags` (`report/json.rs:141-153`, currently private -
  make `pub` and call it).
- lines 20-24: the `worst_severity -> 2/1/0` match duplicates the identical fold inside
  `diag_exit_code` (`commands/mod.rs:46-52`); a shared `fn severity_exit(Option<Severity>) -> i32`
  serves both.

Estimated net lines cut: ~8.

### X1-7 (dup, low, TS): `defaultAppSettings()` duplicated verbatim in two components

- `src/views/BatchView.vue:26-34`
- `src/components/SettingsDialog.vue:24-32`

Byte-identical function returning the default `AppSettings` literal. Replacement: export
`defaultAppSettings()` from `src/ipc.ts` beside the `AppSettings` interface; import in
both. (The TS-vs-Rust `AppSettings::default` mirror itself is boundary-necessary, like
B11; the finding is only the two TS copies.) Estimated net lines cut: ~9.

### X1-8 (dup, low, TS): `RunRequest` re-declared instead of imported

`src/views/JobsView.vue:35-40` declares a local `RunRequest` interface identical to the
`RunRequest` exported from `src/ipc.ts:251-256`, which `src/App.vue:8` already imports.
The in-file comment ("the two sides reconcile trivially since this file owns its whole
contents") describes a parallel-task artifact whose reconciliation never happened.
Replacement: `import type { RunRequest } from "../ipc";`. Estimated net lines cut: ~8.

### X1-9 (dup, low, tests, borderline): `fake_mkvmerge_that_fails_queries` duplicated in-crate

- `crates/muxsmith-cli/tests/dry_run_cli.rs:576-590`
- `crates/muxsmith-cli/tests/run_cli.rs:498-512`

Verbatim-identical helper (same script text, same chmod dance) in two test binaries of
the same crate, which already has `tests/support/mod.rs` for shared helpers. Caveat for
the merge stage: the run_cli.rs copy carries an in-code note "kept local per this file's
existing per-file-helper convention" - a documented choice, but one that predates
support/mod.rs being the established shared home, and it is distinct from the tracked
three-copy fake-mkvmerge decision (that one covers core/mkvmerge_runtime.rs +
src-tauri lib.rs + src-tauri run.rs, cross-crate, trigger >3; this pair is same-crate
where sharing costs one `mod support;` line each). Estimated net lines cut: ~16.

## Considered and rejected (not findings)

- **counting_fake_mkvmerge / fake_mkvmerge / spawn_count / minimal_profile_yaml family**
  (core `tests/mkvmerge_runtime.rs`, `src-tauri/src/lib.rs` tests, `src-tauri/src/run.rs`
  tests): recorded decision, tracked trigger at >3 copies; current count of the counting
  variant is exactly 3, trigger not fired.
- **RECENT_PROFILES_CAP TS/Rust** (`BatchView.vue:82` / `settings.rs:29`): tracked B11.
- **`numeric_diagnostic_params` (i18n.rs:195-202) vs `NUMERIC_DIAGNOSTIC_PARAMS`
  (`src/diagnosticFluentParams.ts:14-17`)**: necessary cross-language mirror, guarded by
  a lockstep unit test (`i18n.rs::numeric_diagnostic_params_list_is_mirrored_to_ts_side`);
  stronger guard than B11's comment-only one. No shared-item option short of codegen.
- **`identify_document` (src-tauri/lib.rs:241-253) vs `print_identify_json`
  (CLI commands/identify.rs:40-55)**: byte-identical JSON shape, but the Tauri copy
  carries an explicit recorded decision (T2 scoped the report::json hoist to the three
  document functions; "below the threshold"). Documented, not re-litigated here.
- **`src/ipc.ts` type mirrors of Rust structs**: the deliberate, documented IPC boundary
  layer (standard Tauri pattern, no codegen pipeline in this repo).
- **`MESSAGE_ID_RE`/`scanIds` mirrored between `scripts/check-i18n.mjs` and
  `e2e/i18n-en.ts:64-75`**: documented mirror with cross-references both ways; consumers
  live in different toolchains (plain-node gate script vs typed Playwright util) and the
  shared surface is ~10 lines. Below threshold.
- **`scalar_display` (planner.rs:858) vs `value_str` (command.rs:241)**: superficially
  similar Scalar stringifiers, semantically different by design (diagnostic rendering
  `true`/`false` vs mkvmerge flag encoding `1`/`0`). Not a duplicate.
- **MatchExpr walkers** (`walk_exact_languages`, `collect_raw_props`, validate's
  `validate_expr`): distinct traversal purposes, recursion boilerplate per walker is
  ~6 lines; a generic visitor is an abstraction the current scale has not earned.
- **`Mkvmerge::locate` vs `detect`**: no duplication - `detect` delegates to `locate`;
  the CLI/GUI resolution split is documented behavior.
- **e2e `smoke.spec.ts` settings literals (`DE_AT_SETTINGS`/`DE_SETTINGS`)**: named,
  purpose-differentiated test fixtures whose explicitness is the point.
- **exact version pins / SHA-pinned actions / TS 6.0.3 / MUXSMITH_RUNS_ROOT debug-only /
  per-matches() regex compilation**: all on the recorded non-findings list, untouched.

## Routed (incidental non-dup observations)

None. No correctness/security/performance issues surfaced during the sweep reads.
