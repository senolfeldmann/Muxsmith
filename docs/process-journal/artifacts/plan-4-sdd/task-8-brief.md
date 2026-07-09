### Task 8: `muxsmith run` subcommand (re-plan, queue, milestone lines, exit fold)

**Files:**
- Create: `crates/muxsmith-cli/src/commands/run.rs`
- Modify: `cli.rs` (Run variant), `main.rs` (dispatch), `commands/mod.rs`, `locales/en/cli.ftl` (new keys)
- Test: `crates/muxsmith-cli/tests/run_cli.rs` (new; fake-free CLI-level tests where possible, planning-failure paths), plus unit tests for the milestone renderer

**Interfaces:**
- Consumes: T3 `run_queue`/`JobEvent`/`QueueOpts`; T4 `CollisionArg`; `command(&Plan)`; `LiveSpawner` (T1); dry_run.rs's flow as the template (validate pass -> locate -> list_languages -> RunInputs -> plan_batch, dry_run.rs:35-85) and its `rendered_diags`/`all_diags` helpers (extract shared pieces into `commands/mod.rs` if cleaner - reviewer judges).
- Produces: `run::run(profile_path, source, output, on_collision, jobs, fail_fast, json, renderer) -> i32`.

- [ ] **Step 1:** cli.rs `Run` variant:

```rust
    /// Plan and execute the batch (spec 5.5 level 3).
    Run {
        profile: PathBuf,
        #[arg(long)] source: Option<PathBuf>,
        #[arg(long)] output: Option<PathBuf>,
        #[arg(long, value_enum)] on_collision: Option<CollisionArg>,
        /// Parallel mux jobs (default 1 = sequential).
        #[arg(long, default_value_t = 1)] jobs: usize,
        /// Stop dequeuing after the first failed job (in-flight finish).
        #[arg(long)] fail_fast: bool,
        #[arg(long)] json: bool,
        #[arg(long)] locale: Option<String>,
    },
```

- [ ] **Step 2:** `commands/run.rs` flow: identical to dry_run through `plan_batch` (re-plan immediately before execution, spec 5.5). Then: `let specs: Vec<JobSpec> = batch.files.iter().filter_map(|f| f.plan.as_ref()).map(|p| JobSpec { argv: command(p), output: p.output.clone() }).collect();` (error-severity files already have `plan: None`). Print planning diagnostics exactly like dry-run (human) FIRST. If `specs` is empty, fold diagnostics and exit like dry-run. Else run the queue with `LiveSpawner { mkvmerge: mkv.path().into() }`, `cancel` flag (plain `Arc<AtomicBool>` here; T10 wires SIGINT), draining events on the main thread.
- [ ] **Step 3:** Milestone renderer (human mode): per JobEvent - Started -> `run-job-start`; Progress -> print at 25/50/75 threshold crossings only (track last-milestone per index); Warning/Error lines -> `run-job-notice`; Finished -> `run-job-ok`/`run-job-warning`/`run-job-failed`/`run-job-cancelled` with duration; then `run-summary` with counts. New cli.ftl keys (exact texts, `{ $index }`/`{ $total }`/`{ $output }`/`{ $percent }`/`{ $seconds }`/`{ $ok }` etc. params):

```
run-job-start = [{ $index }/{ $total }] { $output } ... start
run-job-progress = [{ $index }/{ $total }] { $output } ... { $percent }%
run-job-notice = [{ $index }/{ $total }] { $output } ... { $text }
run-job-ok = [{ $index }/{ $total }] { $output } ... ok ({ $seconds }s)
run-job-warning = [{ $index }/{ $total }] { $output } ... warning ({ $count } warnings, { $seconds }s)
run-job-failed = [{ $index }/{ $total }] { $output } ... failed (exit { $code })
run-job-cancelled = [{ $index }/{ $total }] { $output } ... cancelled
run-summary = { $ok } ok, { $warning } warning, { $failed } failed, { $cancelled } cancelled
```

- [ ] **Step 4:** Exit fold: `max(diag_fold, job_fold)` where job_fold = 2 if any Failed, 1 if any Warning, else 0; if the cancel flag ended the batch -> 130 (overrides). Reuse/extend dry_run's `exit_code` shape.
- [ ] **Step 5:** Tests: unit-test the milestone thresholding (a pure fn over a JobEvent sequence -> rendered lines); CLI test for planning-error path (bad profile exits 2 without executing). The full execute path is T11's gated e2e.
- [ ] **Step 6:** Full gate (incl. `catalog_completeness` for the new keys - they are cli.ftl keys consumed via `renderer.msg`, same guard as always). **Step 7: Commit** - `feat(cli): muxsmith run - re-plan, execute queue, milestone lines, exit fold (D15)`

---

