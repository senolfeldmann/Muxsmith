# Plan 9 recon inventory

Read-only reconnaissance of the ten named design inputs in
`docs/ROADMAP.md:250-289` ("Plan 9: core/orchestration hoists + planner
seam"). Produced 2026-07-27 against a clean `master` at `c2514e7`.

**What this is:** an inventory of what is in the tree today, with every
line reference verified by content at the time of writing, and every count
recomputed from the enumeration it summarizes. It contains no design
proposals and no ranking. Where a designer will have to decide something,
it is recorded as an **OPEN QUESTION** with the facts that bear on it.

**Method notes.**
- All anchors were read, not pattern-matched. Line numbers were re-verified
  by printing the cited line after the enumeration was assembled.
- The shell binds `grep` to a `.gitignore`-respecting function; every search
  here used `git grep` or `command grep`.
- Negative results (an empty search) were only trusted after the identical
  invocation was re-run with a term known to be present. The two negatives
  that carry weight in this document are marked and their controls named.
- `.worktrees/plan8-*` contains full checkouts of `src-tauri`; every search
  excluded it. Nothing below refers to a worktree copy.

---

## Figure check: ROADMAP / ledger claims measured

| Claim | Source | Measured | Verdict |
|---|---|---|---|
| "four-copy planning pipeline" | ROADMAP:254 | 4 | correct |
| "~100 lines" | ledger `core-121` statement | 260 total / 199 non-comment lines for the `load -> plan_batch` stretch across the four sites; 322 / 246 including the `specs` gate where present | **wrong as stated** (see below) |
| "8 scattered render sites" | ROADMAP:285 | 8 (5 direct + 3 ref-fed) | correct |
| "JobsView.vue:150-200" | ROADMAP:280 | the `watch` block is `150-201`; the deviation itself is `160-181` + `194-196` | correct to within one line |
| "the single `eprintln!` in queue.rs" | ROADMAP:270, ledger `exec-36` | exactly 1 in `crates/muxsmith-core/src` | correct |
| "MUXSMITH_RUNS_ROOT ... (run.rs:306, D26)" | ROADMAP:1018-1021 | the CLI gate is at `crates/muxsmith-cli/src/commands/run.rs:330-335` | **stale line number** |
| find-X1's four spans (`dry_run.rs:38-120`, `run.rs:60-192`, `lib.rs:188-232`, `run.rs:249-350`) | `docs/process-journal/artifacts/idiomacy-review-sdd/find-X1.md:14-18` | the two CLI spans still hold verbatim; `lib.rs` is now `205-242` (+17), `src-tauri/run.rs` is now `254-321` (-)/`244-345` for the whole fn | **two of four stale** |
| find-X1's `run_batch` span `src-tauri/src/run.rs:808-829` | find-X1:50-51 | `782-804` | **stale (-26)** |
| find-X1's `resolve_runs_root` span `src-tauri/src/run.rs:853-864` | find-X1:73 | `826-838` | **stale (-27)** |

### On "~100 lines"

The figure originates in find-X1 as an **estimated net cut**, not a size:
"Estimated net lines cut: ~100 (four ~40-75-line code skeletons -> one
~70-line helper + thin per-surface mapping)"
(`docs/process-journal/artifacts/idiomacy-review-sdd/find-X1.md:42-43`).

Ledger `core-121-planner-seam-and-hoist` restates it as a property of the
duplication itself: "The four-copy planning pipeline (~100 lines across cli
dry_run.rs/run.rs and src-tauri lib.rs/run.rs)". Read that way it is wrong
by roughly a factor of 2.5 to 3. The measured spans are in section 1.

---

## 1. The four-copy planning pipeline

### 1.1 The four sites, measured

`code` counts exclude blank lines and whole-line `//` comments.

| # | Site | Function | `load -> plan_batch` | total / code | comparable span (incl. `specs` gate) | total / code |
|---|---|---|---|---|---|---|
| 1 | `crates/muxsmith-cli/src/commands/dry_run.rs` | `run` (sig `30-37`) | `38-120` | 83 / 58 | `38-130` (through the return) | 93 / 67 |
| 2 | `crates/muxsmith-cli/src/commands/run.rs` | `run` (sig `50-59`) | `60-154` | 95 / 69 | `60-192` | 133 / 98 |
| 3 | `src-tauri/src/lib.rs` | `dry_run_body` (sig `199-204`) | `205-240` | 36 / 29 | `205-242` | 38 / 30 |
| 4 | `src-tauri/src/run.rs` | `plan_run` (sig `244-250`) | `254-299` | 46 / 43 | `254-321` | 68 / 60 |
| | | **sum** | | **260 / 199** | | **322 / 246** |

Copy 4's whole function body is `251-344` (94 / 83); line `251` is the
settings read, which has no counterpart in the other three (see D-7 below).

Anchor lines verified by content:

- `dry_run.rs:38` `let profile = match load::from_file(profile_path) {`
- `dry_run.rs:120` `let batch = plan_batch(&profile, &run, &mut ident, &lang);`
- `dry_run.rs:130` `diag_exit_code(&config_diags, &batch)`
- `run.rs:60` `let profile = match load::from_file(profile_path) {`
- `run.rs:154` `let batch = plan_batch(&profile, &run_inputs, &mut ident, &lang);`
- `run.rs:192` closing `}` of the `specs.is_empty()` block
- `lib.rs:205` `let profile = match load::from_file(profile_path) {`
- `lib.rs:242` `report::json::batch_document(&config_diags, &batch, &ShellRenderer)`
- `src-tauri/run.rs:251` `let mkvmerge_override = crate::load_settings_from(settings_path.as_deref())?.mkvmerge_path;`
- `src-tauri/run.rs:254` `let profile = match load::from_file(&profile_path) {`
- `src-tauri/run.rs:299` `let batch = plan_batch(&profile, &run_inputs, &mut ident, &lang);`
- `src-tauri/run.rs:321` closing `}` of the `specs.is_empty()` block

### 1.2 Step-by-step, all four copies

Step numbering is used by the divergence table in 1.3.

| Step | 1 CLI dry-run | 2 CLI run | 3 GUI `dry_run_body` | 4 GUI `plan_run` |
|---|---|---|---|---|
| S0 settings read | absent | absent | absent (done by the command wrapper, `lib.rs:394`) | `251`, `?` on failure -> `IpcError` |
| S1 load profile | `38` | `60` | `205` | `254` |
| S1e load failure | `47-52`: json -> `config_only_document(&[d], None)`; human -> `renderer.diagnostic(&d)`; `return 2` | `72-80`: json -> `run_document(config_only_document(&[d], None), &[], &[])`; human same; `return 2` | `207`: `return config_only_document(&[d], None, &ShellRenderer)` | `256-259`: `return Ok(Soft(run_document(config_only_document(&[d], None, &ShellRenderer), &[], &[])))` |
| S2 config diagnostics | `59-60` | `84-85` | `210-211` | `262-263` |
| S3 resolve mkvmerge | `62` `Mkvmerge::locate()` | `87` `Mkvmerge::locate()` | `213` `Mkvmerge::detect(mkvmerge_override)` | `265` `Mkvmerge::detect(mkvmerge_override.as_deref().map(Path::new))` |
| S3e resolve failure | `69-80`: json -> `config_only_document(cfg, Some(false))`; human -> `severity_sorted` diags to stdout + `eprintln!("mkvmerge-not-found")`; `return 2` | `94-109`: same, wrapped in `run_document` | `215-217`: `return config_only_document(cfg, Some(false), ...)` | `267-274`: `return Ok(Soft(run_document(config_only_document(cfg, Some(false)), &[], &[])))` |
| S4 `list_languages` | `83` | `112` | `219` | `276` |
| S4e query failure | `95-106`: `Some(true)` + `eprintln!("mkvmerge-query-failed")`; `return 2` | `125-140`: same, wrapped | `221-223`: `Some(true)` | `278-285`: `Soft(... Some(true) ...)` |
| S5 `RunInputs` | `110-114`, `source.unwrap_or(PathBuf::from("."))`, `output`, `on_collision` (caller param) | `143-148`, identical | `230-235`, same defaults, `on_collision: None` | `288-294`, `source: Option<String>` mapped, `on_collision: None` |
| S6 `LiveIdentifier` | `116-119` | `150-153` | `236-239` | `295-298` |
| S7 `plan_batch` | `120` | `154` | `240` | `299` |
| S8 present the batch | `122-129`: json -> `batch_document`; human -> sorted config diags + `print_batch_human` | `156-166`: human only (json deferred to the terminal `run_document`) | `242`: `return batch_document(...)` | absent |
| S9 `specs` gate | absent | `170-178` | absent | `304-312` |
| S10 empty-specs branch | absent | `180-192`: json -> `run_document(batch_document(...), &[], &[])`; `return diag_exit_code(...)` | absent | `314-321`: `return Ok(Soft(run_document(batch_document(...), &[], &[])))` |
| S11 terminal | `130` `diag_exit_code(&config_diags, &batch)` -> `i32` | falls through to the queue | `242` -> `serde_json::Value` | `323-344` builds `ReadyPlan` -> `Result<PlanOutcome, IpcError>` |

S9 is byte-comparable between copies 2 and 4:

```
    let specs: Vec<JobSpec> = batch
        .files
        .iter()
        .filter_map(|f| f.plan.as_ref())
        .map(|p| JobSpec {
            argv: command(p),
            output: p.output.clone(),
        })
        .collect();
```

(`crates/muxsmith-cli/src/commands/run.rs:170-178` and
`src-tauri/src/run.rs:304-312`; the only textual difference is the
surrounding comment.)

### 1.3 Divergence enumeration, classified

Classification key: **(a)** genuine parameter, **(b)** accident of copying,
**(c)** deliberate behavioural divergence.

#### (c) Deliberate behavioural divergences

**D-1. `Mkvmerge::locate()` vs `Mkvmerge::detect(override)` (S3).**
Copies 1+2 use `locate()` (PATH only); copies 3+4 use `detect(override)`
(settings override + PATH + platform candidates, D28).
Evidence, all in-tree:
- `src-tauri/src/lib.rs:182-190` (the `dry_run_body` doc): "mirrors
  `muxsmith-cli`'s `dry-run` orchestration ... with one deliberate
  substitution -- `Mkvmerge::detect` ... in place of the CLI's PATH-only
  `Mkvmerge::locate`".
- `src-tauri/src/run.rs:222-233` (the `plan_run` doc): "**Fix (CRITICAL):
  honors the settings mkvmerge override.** This used to resolve mkvmerge
  via `Mkvmerge::locate` (PATH only), unlike every other mkvmerge-touching
  command". So copy 4 was, at one point, an unnoticed copy of the CLI's
  resolver, and that was classified as a critical bug.
- find-X1:166-167 records the split as "documented behavior", not a
  duplication finding.

This difference is expressible as a parameter (an injected resolver), which
is what find-X1:39-40 proposes. It is listed under (c) because the two
surfaces genuinely behave differently, not because the difference resists
parameterization.

**D-2. `mkvmerge_found: Some(false)` carries different meanings per surface.**
`src-tauri/src/lib.rs:192-198`: "`Some(false)` when `Mkvmerge::detect`
itself fails for ANY reason, **including `TooOld`** -- a too-old mkvmerge is
exactly as unusable for planning as a missing one". The CLI copies reach
`Some(false)` only from `locate()`'s not-found. The same wire field in the
same shared document therefore means "absent from PATH" on one surface and
"absent, or present but too old" on the other.
This is a **real semantic divergence in a shared contract**, not merely a
resolver parameter.

**D-3. `on_collision` (S5).** Copies 1+2 pass the caller's
`Option<CollisionPolicy>` (the `--on-collision` flag); copies 3+4 hardcode
`on_collision: None` (`lib.rs:234`, `src-tauri/run.rs:293`). There is no GUI
control for it. I found no ledger entry or memo recording this as a
decision; it reads as "the GUI has no such control yet". find-X1:40 assumes
it "stays caller-side".

**D-4. Human-mode diagnostic ordering vs JSON ordering.** Copies 1+2 print
config diagnostics through `severity_sorted` (`dry_run.rs:75, 101, 125`;
`run.rs:104, 135, 157`) before the batch report; both copies' JSON paths
emit the same vector in collection order. See input 7 for the full picture.
Copies 3+4 have no human mode at all.

**D-5. Return shape.** `i32` exit code (1), `i32` + falls through to the
queue (2), `serde_json::Value` (3), `Result<PlanOutcome, IpcError>` (4).
Deliberate; it is the presentation boundary.

**D-6. Only copies 2+4 build `specs` (S9) and gate on empty (S10).**
The empty-specs branch differs in what it emits: copy 2 prints the document
only under `--json` and returns `diag_exit_code`; copy 4 always returns
`Soft(run_document(...))`, which the caller emits on
`muxsmith://run-finished`.

**D-7. `plan_run` reads settings first and can fail before any pipeline step**
(`src-tauri/src/run.rs:251`, `?` -> `IpcError`). This is the only path of the
four where a failure is neither a diagnostic nor a document. `dry_run_body`
has no such step because its command wrapper does the settings read
(`src-tauri/src/lib.rs:392-401`); the two GUI copies therefore place the same
read at different layers.

#### (b) Accidents of copying

**A-1. Four inlined `validate::validate` + `lint::provable_overlaps` pairs
despite the funnel existing.** `muxsmith_core::profile::validate::config_diagnostics(&Profile)`
has existed since Plan 5.6 T11 (`crates/muxsmith-core/src/profile/validate.rs:193-197`):

```
pub fn config_diagnostics(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = validate(profile);
    diags.extend(lint::provable_overlaps(profile));
    diags
}
```

`lint::provable_overlaps` call sites outside core (recomputed by
enumeration, `git grep -n "lint::provable_overlaps" -- crates src-tauri`):
`dry_run.rs:60`, `run.rs:85`, `lib.rs:211`, `src-tauri/run.rs:263` -- all
four pipeline copies, none migrated. The funnel's own callers are
`crates/muxsmith-cli/src/commands/validate.rs:21` (`_from_file`),
`src-tauri/src/lib.rs:178` (`_from_file`), `:311` and `:342`.

So find-X1's X1-3 ("a future second lint must currently be added at six call
sites") was closed only for the two `validate`-shaped consumers. Four
consumers remain. ROADMAP:256-257 already anticipates that "plan_pipeline
consumes profile::validate::config_diagnostics".

**A-2. Near-verbatim rationale comments duplicated between copies 1 and 2.**
`dry_run.rs:41-46` / `run.rs:63-71`; `dry_run.rs:64-68` / `run.rs:90-93`;
`dry_run.rs:85-94` / `run.rs:115-124`. In copy 1's spans these are 25 of 83
lines; in copy 2's, 26 of 95.

**A-3. Input types differ for the same values.** Copies 1+2+3 take
`Option<PathBuf>` for `source`/`output`; copy 4 takes `Option<String>` and
converts inside (`src-tauri/run.rs:289-292`). Copy 4 also takes `profile:
String` and builds the `PathBuf` itself (`:253`), where copy 3's wrapper
does it (`lib.rs:396`).

**A-4. The `detect` argument is prepared at different layers.** `lib.rs:213`
receives an already-prepared `Option<&Path>`; `src-tauri/run.rs:265` does
`mkvmerge_override.as_deref().map(Path::new)` inline.

**A-5. `RunInputs` construction, `IdentifyCache::new()`, and
`LiveIdentifier { cache, mkv }` are byte-identical in all four** (S5, S6);
each copy creates a fresh cache that is dropped at the end of the call.

**A-6. The runs-root `cfg` gate exists in two syntactic forms.** See input 3.

#### (a) Genuine parameters

`profile_path`; `source`; `output`; `on_collision` (see D-3);
`json` vs no-json; the renderer (`&Renderer` vs `&ShellRenderer`); the
mkvmerge resolver (see D-1); whether the document is wrapped in
`run_document`; whether `specs` are built; the terminal mapping.

### 1.4 OPEN QUESTIONS for the designer

- **`PathBuf::from(".")` as the source default in the GUI.** All four copies
  carry it. `src-tauri/src/lib.rs:226-229` documents it as questionable:
  "No natural 'current directory' for a bundled desktop app, but kept for
  parity with the CLI's own fallback (dry_run.rs); in practice the batch
  view (T10) always supplies an explicit source directory". `plan_run`
  (`:289-291`) has the same fallback with no such note. Whether the seam
  should keep the default, require the caller to supply it, or make it a
  parameter is a design decision, and it touches both GUI copies.
- **D-2's `mkvmerge_found` semantics.** A single shared pipeline has to pick
  one meaning for the field, or keep the resolver's failure reason as data.
- **`IdentifyCache` lifetime.** Every copy builds and drops one per call, so
  a GUI dry-run followed by a run re-identifies every file. Whether the seam
  owns the cache or takes it is a decision the hoist forces.
- **Where the specs gate lives.** It is duplicated between copies 2 and 4
  but is arguably planner output, not orchestration; `batch.files` already
  carries `plan: None` for error-severity files
  (`crates/muxsmith-cli/src/commands/run.rs:168-169`).

---

## 2. `run_batch`

### 2.1 The two sites

**Factored (GUI):** `src-tauri/src/run.rs:782-804`, doc `760-781`.

```
fn run_batch(
    specs: &[JobSpec],
    spawner: &(dyn Spawn + Sync),
    opts: QueueOpts,
    ctl: &Arc<QueueControl>,
    mut logger: Option<RunLogger>,
    mut on_event: impl FnMut(&JobEvent),
) -> (Vec<JobOutcome>, Option<RunLogger>)
```

Body: `mpsc::channel()` -> `std::thread::scope` -> `scope.spawn(run_queue)`
-> `for event in rx { logger.on_event(&event); on_event(&event); }` ->
`handle.join().expect("queue worker thread panicked")` -> returns
`(outcomes, logger)`.

**Inlined (CLI):** `crates/muxsmith-cli/src/commands/run.rs:241-272`
(`mpsc::channel()` at `241`, `std::thread::scope` at `249-272`), with the
logger created at `239` and consumed at `286-304`.

Measured: the GUI function body is 23 lines (`782-804`); the CLI's
equivalent stretch is 32 lines (`241-272`), of which 9 are comments. find-X1
cited `run.rs:231-262` for the CLI side; that has drifted by +10.

### 2.2 Divergence analysis

| Aspect | CLI inline `241-272` | GUI `run_batch` `782-804` | Class |
|---|---|---|---|
| Channel + scope + spawn + join | identical, including the `expect` string `"queue worker thread panicked"` (`run.rs:271` / `src-tauri/run.rs:801`) | identical | (b) |
| Logger tee | inline `if let Some(logger) = logger.as_mut() { logger.on_event(&event); }` (`258-260`) | same, inside the function (`794-796`) | (b) |
| Per-event work | inline: `if json { continue; }` then `milestones.render(...)` + `println!` (`261-268`) | delegated to `on_event: impl FnMut(&JobEvent)` (`797`) | (a) |
| Logger ownership | `let mut logger` lives in the caller's scope and is used after (`286-304`) | moved in, returned back out with the outcomes | (a) |
| Spawner type | concrete `LiveSpawner` moved into the closure (`204-206`) | `&(dyn Spawn + Sync)` | (a) |
| `specs` | `Vec<JobSpec>` moved into the queue closure (`251`) | `&[JobSpec]` | (a) |
| `QueueOpts` | `QueueOpts { jobs, fail_fast }` from CLI flags (`207`) | `QueueOpts { jobs, fail_fast: false }` hardcoded at the call site (`src-tauri/run.rs:482-485`) | **(c)** |
| Calling thread | the CLI's own main thread inside `thread::scope` | a detached `std::thread` spawned by `start_run` (`src-tauri/run.rs:468`) | (a) |
| Panic containment | none beyond `expect` | wrapped by `TeardownGuard` (`src-tauri/run.rs:479`), whose doc (`471-478`) says it exists so that a `run_batch` join-panic still clears the slot and honors a pending quit | (c), but it is around the call, not inside it |

**The single behavioural divergence is `fail_fast`.** The GUI never sets it;
the CLI exposes it. find-X1:53 asserts the Tauri version "is exactly the
shape the CLI needs"; that holds for the closure shape, and `fail_fast`
already travels inside `QueueOpts`, so it is caller-side either way.

### 2.3 OPEN QUESTION

`TeardownGuard` and the joblog finalization sit around the GUI call
(`src-tauri/run.rs:479-492`), not inside `run_batch`; the CLI's equivalent
post-processing is at `crates/muxsmith-cli/src/commands/run.rs:274-304`
(`run_document`, summary print, `logger.finish`, the
`run-joblog-written` / `run-joblog-incomplete` split). A hoist has to decide
whether the shared function ends where `run_batch` ends today, or absorbs
the `run_document` + `finish` pair that both surfaces also duplicate in
different shapes (`finalize_joblog`, `src-tauri/run.rs:814-822`, folds the
same `Ok`/`Err` into a `JoblogStatus` enum; the CLI folds it into two
different stderr/stdout messages).

---

## 3. Runs-root resolution (D26 debug-only seam)

### 3.1 The two sites

**CLI:** `crates/muxsmith-cli/src/commands/run.rs:324-345` (`create_logger`),
resolution half at `330-335`, doc at `316-323`.

```
    #[cfg(debug_assertions)]
    let runs_root = std::env::var_os("MUXSMITH_RUNS_ROOT")
        .map(PathBuf::from)
        .or_else(default_runs_root);
    #[cfg(not(debug_assertions))]
    let runs_root = default_runs_root();
```

**GUI:** `src-tauri/src/run.rs:826-838` (`resolve_runs_root`), doc `822-825`
("mirroring the CLI's own `create_logger` ... see the CLI's identical gate
for the rationale"). Same semantics, written as `cfg`-attributed **blocks**
rather than `cfg`-attributed `let` statements.

`default_runs_root` itself is core:
`crates/muxsmith-core/src/executor/joblog.rs:51-53`,
`dirs::data_dir().map(|dir| dir.join("muxsmith").join("runs"))`.

### 3.2 Divergences

| Aspect | CLI | GUI | Class |
|---|---|---|---|
| `cfg` form | attribute on two `let` statements | attribute on two blocks | (b) |
| Scope of the function | `create_logger` also does `make_run_id`, `RunLogger::create`, and emits `run-joblog-unavailable` to stderr on failure (`341-343`) | `resolve_runs_root` is resolution only; the `create` happens at `plan_run:325-326`, silently | (c) |
| Call sites | 1 (`create_logger`) | 3: `plan_run:326`, `list_runs:529`, `get_job_log:535` | (a) |
| Failure surface | `eprintln!(renderer.msg("run-joblog-unavailable"))` | no message; the condition reaches the frontend as `joblog_status: "unavailable"` in `muxsmith://run-finished`, rendered at `src/views/JobsView.vue:227` | (c) |

### 3.3 What the debug-only gate actually gates

It gates **one env var read**, `MUXSMITH_RUNS_ROOT`, and only in
`debug_assertions` builds. In release builds the env var is not read at all
and `default_runs_root()` is unconditional.

Recorded rationale: ledger `exec-43-runsroot-debug-gated`
(`docs/decision-ledger.yaml:1117-1128`): "The MUXSMITH_RUNS_ROOT override
used by tests was shipping as an unconditional, undocumented env-var surface
read in the release binary; gated behind `#[cfg(debug_assertions)]`."
Status `settled`. `docs/ROADMAP.md:1018-1021` keeps a user-facing override as
an explicit v1.x decision, "not a side door" (Şenol 2026-07-11).

**Who actually uses the seam** (recomputed, `git grep -n MUXSMITH_RUNS_ROOT`,
excluding `.worktrees` and `docs/process-journal`):

- producers: `crates/muxsmith-cli/src/commands/run.rs:331`,
  `src-tauri/src/run.rs:830`
- consumers: `crates/muxsmith-cli/tests/run_cli.rs:172`,
  `crates/muxsmith-cli/tests/run_live.rs:110, 245, 370, 468`

**The GUI copy of the seam has no consumer.** Nothing under `e2e/` or
`src-tauri` sets `MUXSMITH_RUNS_ROOT`; the only `src-tauri` tests that touch
runs-root paths call `list_runs_in` / `get_job_log_in` with an explicit
`Option<&Path>` (`src-tauri/src/run.rs:1575, 1600, 1610, 1621, 1627, 1633`),
bypassing `resolve_runs_root` entirely.

> Negative-result control: the same `git grep -n MUXSMITH_RUNS_ROOT --
> ':!.worktrees' ':!docs/process-journal'` invocation returns the seven hits
> listed above, so the absence of an `e2e/` or `src-tauri` hit is a real
> absence, not a malformed pathspec.

### 3.4 OPEN QUESTION

A core-side `joblog::resolve_runs_root()` would put a `cfg(debug_assertions)`
env read inside the library rather than in the two binaries. find-X1:79-81
asserts the semantics are unchanged ("workspace dev/release profiles apply
to core identically"). That claim is worth re-verifying against
`Cargo.toml`/`rust-toolchain.toml` before it becomes load-bearing; this recon
did not verify it. It also changes who the gate protects: today each binary
gates its own surface; hoisted, one core gate serves both plus any future
consumer.

---

## 4. The single `eprintln!` in `queue.rs` (ledger `exec-36`)

### 4.1 The site

`crates/muxsmith-core/src/executor/queue.rs:396`, inside
`recover_panicked_worker` (`380-417`, doc `357-379`):

```
    eprintln!("muxsmith: worker thread panicked while running job {index}: {message}");
```

`message` is the downcast panic payload (`391-395`): `&str`, then `String`,
else the literal `"<non-string panic payload>"`.

The doc at `374-379` states the design intent: "The panic payload itself ...
is arbitrary developer-diagnostic content, not a stable, translatable code
-- like `job.rs`'s `delete_partial_failed` detail, it is core's one
deliberate prose-free exception (spec 6/7): logged here for triage, never
carried past this function into the user-facing `JobOutcome` beyond the
stable `worker-panicked` `DiagCode`."

Ledger `exec-36-core-stderr-logging` (`docs/decision-ledger.yaml:1069-1080`):
"The single `eprintln!` in core (queue.rs) is the first direct stderr I/O in
core; the idiomatic fix is a log/tracing facade the binaries route,
deferred." Status `blocked` on Plan 9.

### 4.2 Is it really one call site? Yes.

`git grep -n "eprintln!\|print!\|println!" -- 'crates/muxsmith-core/src/**'`
returns exactly two lines: `queue.rs:396` (the call) and `lib.rs:23` (a
comment about the CLI/GUI skip marker). Count of actual I/O calls in core
source, including inline `#[cfg(test)]` modules: **1**.

> Negative-result control: the same invocation against
> `crates/muxsmith-cli/src/**` returns 9 hits (below), so the pattern and
> pathspec are sound.

### 4.3 Every other core site a facade would want

Enumerated by hand from `git grep -n "let _ = " -- crates/muxsmith-core/src`
plus a sweep of `.ok()` / `unwrap_or_default()`. Test modules start at
`queue.rs:453`, `job.rs:212`, `spawn.rs:285`; `joblog.rs` has no inline test
module (its tests are `crates/muxsmith-core/tests/joblog.rs`).

**Silently discarded failures (production code):**

| Site | Code | What is lost |
|---|---|---|
| `executor/job.rs:113` | `let _ = std::fs::create_dir_all(parent);` | output-directory creation failure. The subsequent spawn will fail and produce a `JobOutcome::Failed` with mkvmerge's own message, so the user sees *a* failure but never the directory reason. |
| `executor/joblog.rs:131` | `let _ = fs::remove_dir_all(entry.path());` | stale-run pruning failure. Nothing else observes it. |
| `executor/spawn.rs:123` | `let _ = child.lock().unwrap().kill();` | kill failure inside the `Killer` closure. Doc at `120-122` explains the ordering, not the discard. |
| `executor/spawn.rs:106-111` | `.wait().ok().and_then(\|s\| s.code())` | the wait error; folded into `resolve_wait`'s `None`. |

**Deliberate, arguably not facade material:**

| Site | Code | Note |
|---|---|---|
| `executor/queue.rs:252, 272, 285, 413` | `let _ = events.send(...)` | the receiver has hung up; there is no consumer left to log to either. |

**Value-level fallbacks (not I/O, listed for completeness):**
`capability/runtime.rs:193`, `discovery.rs:146, 150`, `joblog.rs:80`,
`identify.rs:168, 173, 360`, `matcher.rs:179`, `planner.rs:902, 2105, 2124`.

### 4.4 The surface asymmetry that bears on the decision

- **CLI:** 9 `eprintln!` sites in `crates/muxsmith-cli/src`, every one of them
  routed through the localized `Renderer`:
  `dry_run.rs:78, 104`; `identify.rs:17, 25`;
  `run.rs:107, 138, 233, 299, 342`.
- **GUI (`src-tauri/src`):** **zero** non-test `eprintln!`/`println!`. The
  only three hits (`lib.rs:827, 886, 970`) are inside `#[cfg(test)]` and emit
  `MKVMERGE_SKIP_MARKER` for CI's no-silent-skip gate.

So core's one `eprintln!` is written to a stream a bundled desktop app
normally has no console for, while every CLI-side stderr line is a
translated catalog message. That is one call site, but two structurally
different consumers.

### 4.5 OPEN QUESTION

Whether one call site justifies a facade depends on whether the four
silently-discarded failures in 4.3 are in scope. If they are, the facade has
five producers; if they are not, it has one, and the honest alternative is
to move the payload out of core (return it, let the binary print it) rather
than add a dependency. Nothing in the tree records a preference.

---

## 5. `JobOutcome.errors` and the worker-panicked path (ledger `exec-37`)

### 5.1 Where the codes are produced

`JobOutcome.errors: Vec<String>` is declared at
`crates/muxsmith-core/src/executor/job.rs:52`, doc "Captured error lines (tag
stripped)".

Three producers, all in core:

1. **`crates/muxsmith-core/src/executor/queue.rs:408`** (in
   `recover_panicked_worker`):
   `errors: vec![format!("{}: job {index}", DiagCode::WorkerPanicked.key())]`
   -> the literal string `"worker-panicked: job 3"`. `DiagCode::WorkerPanicked`
   maps to `"worker-panicked"` at `crates/muxsmith-core/src/report/mod.rs:186`.
2. **`crates/muxsmith-core/src/executor/job.rs:128`**: `errors: vec![message]`
   -- the `SpawnError` message (prose from the OS, not a code).
3. **`crates/muxsmith-core/src/executor/job.rs:148`**: `errors.push(text.clone())`
   -- captured `#GUI#error` lines from mkvmerge, tag stripped (prose).
   Plus `job.rs:208`: `errors.push(format!("delete_partial_failed: {e}"))`,
   a second code-shaped-but-not-catalogued token.

So `errors` is a **mixed vector**: catalog-code tokens (`worker-panicked: job N`,
`delete_partial_failed: <io error>`) and raw third-party prose, with no
discriminator.

### 5.2 Which surfaces render it today

| Surface | Renders `errors`? | Anchor |
|---|---|---|
| CLI human `run` output | **No.** `render_finished`'s `JobState::Failed` arm renders `run-job-failed` with `index`/`total`/`output`/`code` only; `code` is `exit_code` or the literal `"n/a"`. A worker panic has `exit_code: None`, so the user sees `n/a`. | `crates/muxsmith-cli/src/commands/run.rs:483-497` |
| CLI `--json` | **Yes, verbatim.** `run_document` serializes the whole `JobOutcome` (`serde_json::to_value(outcome)`), so `jobs[].errors` carries `"worker-panicked: job N"` as an opaque string. | `crates/muxsmith-core/src/report/json.rs:117-127` |
| Persisted `job-<index>.json` / `summary.json` (D26) | **Yes, verbatim** (`JobLogRecord.errors`, mirrored at `src/ipc.ts:222`). | `crates/muxsmith-core/src/executor/joblog.rs` |
| GUI job rows | **No.** `JobRowData` (`src/jobRowState.ts:25-34`) has `index`, `output`, `state`, `warningCount` -- no `errors` field. `JobRow.vue` renders the state chip (`jobStateKey`) and the warning count only (`src/components/JobRow.vue:50-51`). | |
| GUI live-error events | **No.** `JobsView.vue:84-89` handles `case "error":` with a comment and no action: "Surfaced via the eventual `finished` outcome's `errors` and via LiveLog ... no separate row field beyond the warning-count badge". The `finished` handler (`:91-94`) reads only `outcome.warnings.length`. | |
| GUI run history | **No.** `RunHistory.vue` never touches `.errors`; its log export writes `record.lines.join("\n")` (`src/components/RunHistory.vue:86`), i.e. raw mkvmerge output lines. A worker panic produces no output line, so the export omits it too. | |
| GUI live log pane | **No** (same reason: raw `Output` events only). | |
| The `worker-panicked` Fluent message | **Never looked up.** It exists in both catalogs (`locales/en/diagnostics.ftl:80`, `locales/de/diagnostics.ftl:87`) and is exercised only by the CLI catalog-completeness test, which feeds it an empty param list (`crates/muxsmith-cli/tests/catalog_completeness.rs:152`: `DiagCode::WorkerPanicked => vec![]`). | |

> Negative-result control: `git grep -n "\.errors\|errors:" -- crates/muxsmith-cli/src src-tauri/src crates/muxsmith-core/src/report src/`
> returns 4 hits (`crates/muxsmith-cli/src/commands/run.rs:527` in a test
> fixture; `src/ipc.ts:163, 222` type declarations; `src/views/BatchView.vue:467`,
> which is `errors: diagnosticCounts.error` for the *diagnostics* summary line,
> unrelated to `JobOutcome`). The invocation demonstrably returns hits, so the
> absence of a rendering consumer is real.

### 5.3 Ledger and spec position

`exec-37-panicked-msg-catalog` (`docs/decision-ledger.yaml:1081-1092`): "The
rich worker-panicked message renders on no live surface (only JSON carries
the token); routing `JobOutcome.errors` codes through the diagnostics catalog
was deferred." Status `blocked` on Plan 9. Confirmed accurate.

Spec `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:289`:
`WorkerPanicked` severity is "n/a (job-error token, not a rendered
diagnostic) ... carried as a `worker-panicked: job N` token in
`JobOutcome.errors` (and its `--json` job encoding) instead, **rendered
through this same catalog entry at presentation time (6)**". The spec
therefore already promises presentation-time rendering that no surface
performs.

### 5.4 OPEN QUESTIONS

- **The vector is untyped.** Any catalog routing has to distinguish
  code tokens (`worker-panicked: job N`, `delete_partial_failed: <detail>`)
  from raw mkvmerge prose within the same `Vec<String>`. Today the only
  discriminator is a string prefix; `queue.rs:766` already does exactly that
  in a test (`.any(|e| e.starts_with(DiagCode::WorkerPanicked.key()))`).
- **`delete_partial_failed` is the second instance of the same shape** and is
  not mentioned by `exec-37`. `job.rs:198-208` documents it as core's other
  deliberate prose-carrying exception.
- **Two GUI surfaces would need a new field**, not just a renderer:
  `JobRowData` and the history row both drop `errors` before rendering.

---

## 6. Bare `raw:` with an empty property name at validate (T16-m1)

### 6.1 The validation sites

`raw:` is stripped in five places (`command grep -n 'strip_prefix("raw:")'`
over the tree):

| Site | Context |
|---|---|
| `crates/muxsmith-core/src/profile/validate.rs:268` | `expr.exact` map |
| `crates/muxsmith-core/src/profile/validate.rs:310` | `expr.substring` / `expr.regex` maps |
| `crates/muxsmith-core/src/matcher.rs:95` | `exact_matches` at match time |
| `crates/muxsmith-core/src/matcher.rs:186` | `strip_raw`, used by the substring/regex/language string lookup |
| `crates/muxsmith-core/src/planner.rs:796` | `collect_raw_props`, for the capability-version warning |

### 6.2 What happens today with the key `raw:` (empty bare name)

**At validate.** `"raw:".strip_prefix("raw:")` yields `Some("")`, so both
validate sites take the `raw:` branch and call
`raw_opt_in_diagnostic(&p, "")` (`validate.rs:408-414`):

```
fn raw_opt_in_diagnostic(path: &str, bare: &str) -> Diagnostic {
    if matches!(bare, "language" | "codec_kind") {
        Diagnostic::warning(DiagCode::RawOnKnownProperty, path.to_string()).with("property", bare)
    } else {
        Diagnostic::info(DiagCode::RawProperty, path.to_string()).with("property", bare)
    }
}
```

`""` is neither `language` nor `codec_kind`, so the result is an **info-severity
`RawProperty` diagnostic with `property: ""`**. Info severity does not affect
the exit code (`crates/muxsmith-cli/src/commands/mod.rs:29-36`,
`severity_exit`), so `validate` and `dry-run` still exit 0 on it. The rendered
message will interpolate an empty `$property`.

Note the `continue` at `validate.rs:275` (the `exact` arm): the `raw:` branch
short-circuits before any existence/type/domain check, so no
`UnknownProperty` is ever emitted for the empty name. The substring/regex arm
(`:310-315`) does not `continue`, so a regex value is still compile-checked
(`:337-338`).

**At match time.** `matcher.rs:95-105` takes the `raw:` branch and calls
`item.get("")`, which no `Matchable` implementation can answer, so the arm
returns `false`: the rule never matches. Doc at `96-101` describes this for
absent raw properties generally ("an absent raw: property simply does not
match (B-6)").

**At planning.** `planner.rs:796-798` inserts `""` into the
`BTreeSet<String>` of raw property names used for the capability-version
warning, so an empty name can appear in that diagnostic's param list.

### 6.3 Provenance

`docs/process-journal/artifacts/plan-5.5-sdd/task-16-verdict.md:36-37`:
"Minor: bare `raw:` (empty name) accepted, yields RawProperty with
property=\"\" and never matches; no panic on any path; cosmetic wart."

`docs/process-journal/artifacts/plan-5.5-sdd/whole-branch-verdict.md:92`:
"| T16-m1 (bare `raw:` empty property accepted) | DEFER | Plan 6: reject
empty bare name at validate; currently visible-but-odd, never silent |"

The deferral target says Plan 6; ROADMAP:274 now carries it under Plan 9.
There is **no ledger entry** for T16-m1 (unlike `exec-36`, `exec-37`,
`cli-08`): `git grep -n "T16-m1"` returns only `docs/ROADMAP.md:274` and the
whole-branch verdict line above.

### 6.4 OPEN QUESTION

"Reject at validate" is stated as an error-severity change in the deferral
note but neither the severity nor the code is fixed anywhere. A new
`DiagCode` has downstream obligations recorded in the tree: the
catalog-completeness fixture table (`crates/muxsmith-cli/tests/catalog_completeness.rs`),
both `locales/*/diagnostics.ftl`, and the spec's severity table
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` section 5.2).
Whether the existing `UnknownProperty` can carry it instead is undecided.

---

## 7. `config_diagnostics` JSON ordering (ledger `cli-08`)

### 7.1 The sorted emitter

`crates/muxsmith-cli/src/commands/validate.rs:19-29`:

```
    // Error-first, stable within a severity; both output modes share it.
    let diagnostics: Vec<Diagnostic> =
        severity_sorted(&validate::config_diagnostics_from_file(profile_path))
            .into_iter()
            .cloned()
            .collect();
    let exit = severity_exit(worst_severity(&diagnostics));

    if json {
        let entries = rendered_diags(&diagnostics, renderer);
        println!("{}", serde_json::json!({ "diagnostics": entries }));
```

One sorted vector feeds both modes, and the JSON envelope key is
`"diagnostics"`, not `"config_diagnostics"`.

`severity_sorted` is `crates/muxsmith-cli/src/commands/mod.rs:21-25`
(`sort_by_key(Reverse(d.severity))`, stable within a severity).

### 7.2 The unsorted emitters

The `config_diagnostics` array is built in exactly two places, both in core,
both from the caller's vector in collection order:

- `crates/muxsmith-core/src/report/json.rs:53` (`batch_document`)
- `crates/muxsmith-core/src/report/json.rs:84` (`config_only_document`)

```
        "config_diagnostics": rendered_diags(config_diags, renderer),
```

The vector reaching them is always `validate::validate(&profile)` followed
by `.extend(lint::provable_overlaps(&profile))`, i.e. static checks in
traversal order, then overlap lints appended. Producers:

| Caller | Line | Sorted before the call? |
|---|---|---|
| CLI `dry_run.rs` json path | `123` | **No** (`severity_sorted` is used only on the human path, `:125`) |
| CLI `dry_run.rs` config-only json paths | `72`, `98` | **No** (human paths at `:75`, `:101` do sort) |
| CLI `run.rs` json paths | `98`, `129`, `188`, `274-278` | **No** (human paths at `:104`, `:135`, `:157` do sort) |
| GUI `validate_profile_body` | `lib.rs:179` | **No** -- and this one is the direct analogue of CLI `validate`, which does sort |
| GUI `load_profile_body` | `lib.rs:311-312` | **No** |
| GUI `validate_profile_model_body` | `lib.rs:342` | **No** |
| GUI `dry_run_body` | `lib.rs:242` (and `216`, `222`) | **No** |
| GUI `plan_run` | `src-tauri/run.rs:269, 280, 316` (all via `run_document`) | **No** |

So the parity gap is not only CLI-internal: the GUI's `validate_profile`
returns an unsorted array where the CLI's `validate --json` returns a sorted
one, for the same profile.

### 7.3 Provenance

`docs/decision-ledger.yaml:2798-2809`, `cli-08-config-diags-json-ordering`:
"The flat config_diags JSON is unsorted, inconsistent with validate's sorted
JSON; sorting for parity was deferred because consumers key on the severity
field and ordering is cosmetic. Same theme as cli-01 but a distinct,
deliberately-deferred decision on a specific surface." Status `blocked` on
Plan 9.

Origin, `docs/process-journal/artifacts/plan-5.5-sdd/task-9-verdict.md:22-24`:
"(iv) JSON-unsorted defensible, BUT flat config_diags JSON is now
inconsistent with validate's sorted JSON (validate sorts one vec for both
modes). Minor; consumers sort by severity field." And `:37`: "cheap close if
wanted."

### 7.4 OPEN QUESTIONS

- **Where the sort belongs.** `severity_sorted` is `pub(crate)` in the CLI
  crate (`commands/mod.rs:21`), so the GUI cannot call it today. Sorting
  inside `rendered_diags` / the two document builders would change core's
  output for every consumer at once; sorting per caller reproduces the same
  drift at eight sites.
- **Frontend consumers that index the array.** At least one does:
  `src/views/BatchView.vue:225` reads `doc.config_diagnostics[0]` to detect a
  parse failure. A sort would change which diagnostic lands at index 0.
  (`:232` documents the empty-array case.) This is the one place where
  "ordering is cosmetic" is not obviously true.

---

## 8. The D23 frontend-contract deviation (`JobsView.vue`)

### 8.1 What the code does now

`src/views/JobsView.vue`, the `pendingRun` watcher, `150-201`. The relevant
part, verbatim (`160-181`):

```
    // Fix (D23 divergence): only reset the live-run display when this view
    // does not already believe a run is active. Resetting unconditionally
    // here used to wipe a currently-active run's rows the instant a SECOND
    // start_run landed (e.g. a stray double-dispatch) and got rejected
    // with run-already-active -- the catch branch then flipped runActive
    // to false too, disabling cancel while the first run kept executing
    // completely invisibly. `runActive` is this view's own single source
    // of truth for "a run is active" (set true only on a successful start,
    // cleared only by onRunFinished or a failed *fresh* start below), so
    // checking it first is self-consistent. The reset itself must still
    // run BEFORE calling startRun, not after: a soft-outcome run (zero
    // jobs planned, etc.) emits muxsmith://run-finished synchronously
    // inside the Rust command, before this command's own promise resolves
    // (run.rs's documented event-ordering contract) -- resetting AFTER a
    // successful await would clobber whatever onRunFinished just wrote for
    // that very run.
    const startingFresh = !runActive.value;
    if (startingFresh) {
      jobs.value = [];
      logLines.value = [];
      finishedSummary.value = null;
      runActive.value = true;
    }
```

and the catch arm (`193-196`):

```
    } catch (e) {
      startError.value = e as IpcError;
      if (startingFresh) {
        runActive.value = false;
      }
```

`runActive` is a `defineModel<boolean>("runActive")` (`:50`), cleared in
`onRunFinished` (`:119`), consumed upward via `App.vue:245, 250` and read by
`BatchView.vue:294` to disable the Run button.

### 8.2 What D23 says

The memo, `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md:46-58`,
contains no reset-ordering clause at all. Its only relevant sentence is:
"Exactly one run at a time, enforced in Tauri managed state; a second
`start_run` is rejected with a diagnostic (the UI additionally disables Run
while active)."

The wording "reset after resolve Ok" is **not** D23's. It comes from the
plan-5 whole-branch review's proposed fix,
`docs/process-journal/artifacts/plan-5-sdd/verdicts/whole-branch-review-verdict.md:41`:
"Two fixes, both wanted: (a) in JobsView, reset state only after `startRun`
resolves Ok; (b) surface run-active state to BatchView and disable Run, per
the memo's explicit sentence."

Ledger `gui-11` (`docs/decision-ledger.yaml:2053-2064`) records the original
defect and its correction; ledger `gui-09` (`:2030-2041`) records the
single-run rule. Neither records a reset-ordering rule.

### 8.3 The self-flag and the review that already answered it

`docs/process-journal/artifacts/plan-5-sdd/final-fix-wave-plan5-report.md:73-83`
(the implementer's own note): "Deliberately **not** literally 'reset after
`start_run` resolves Ok' as worded in the brief -- I checked and that reading
breaks soft-outcome runs (`muxsmith://run-finished` fires synchronously
inside the Rust command before the promise resolves ...). The reset stays
*before* the call, gated on `runActive`, which satisfies the same observable
requirement ... without the regression."

Residual concern, `:206-211`: "Worth a second look given it touches D23's own
frontend contract."

`docs/process-journal/artifacts/plan-5-sdd/verdicts/whole-branch-review-verdict-round-2.md:53-62`
already took that second look and ruled: "**The implementer was right to
deviate, and the deviation is correct.** My literal suggestion is in fact
deterministically broken on soft outcomes ... **Deviation approved; the
implemented form is strictly better than my literal wording.**" That verdict
traces five orderings (fresh real run, fresh soft outcome, fresh rejection,
double-dispatch against an active run, interleaved rapid dispatches) and
names one residual: "the frontend believes `runActive=true` while the backend
has no run, requiring a lost `run-finished` event", judged unreachable within
the design's contracts.

So the ROADMAP item is a **re-check of an already-reviewed and approved
deviation**, not an open defect. The event-ordering premise the deviation
rests on is still true in the code: `src-tauri/src/run.rs:411-422`
(`start_run`'s doc) and `:726-729` (`finish_without_queue`'s doc) both state
that `muxsmith://run-finished` fires before the command's `Result` reaches the
caller.

### 8.4 Is the deviation observable to a user?

The two behaviours differ only when a second `pendingRun` arrives while
`runActive` is `true`. Facts bearing on reachability:

- The Run button that produces `pendingRun` is disabled while `runActive`:
  `src/views/BatchView.vue:294-295` returns `"tooltip-run-active"` from
  `runDisabledReason`, checked "first, since it overrides every other"
  (`:290`).
- `runActive` reaches `BatchView` through `JobsView` -> `App.vue:245, 250`
  -> `BatchView` prop, so the disable is driven by the same flag the gate
  reads. There is no independent path.
- `runActive` is set to `true` synchronously in the same continuation
  segment as the `startingFresh` check (`:176-181`), which round-2's analysis
  relies on for the interleaving case.

**Under the current design the divergent branch is unreachable from the UI**;
it fires only on a programmatic double-dispatch. Where it *is* observable is
the fresh-rejection case, which both readings share: a rejected fresh start
clears the previous run's finished display (round-2 calls this "acceptable
(history intact)").

### 8.5 OPEN QUESTIONS

- The current form has **no test**. `JobsView.vue` cannot be mounted by the
  e2e mount harness (see input 10), and the live-run e2e never
  double-dispatches. The fix report itself flags this
  (`final-fix-wave-plan5-report.md:216-220`): "fix 3's regression coverage
  rests on the Playwright e2e suite, which is shallower than a targeted unit
  test would be for the exact reset-ordering logic."
- No ledger entry records the deviation as a decision. `gui-11` records the
  defect and "Corrected", not the form of the correction. A designer choosing
  to keep the current form has nothing to point at except the round-2 verdict
  in the process journal.

---

## 9. The eight `IpcError` render sites

Recomputed by content (`git grep -n '\$t(' -- 'src/**/*.vue'` filtered to
error-carrying expressions, then each file read). **Count: 8.** Matches
ROADMAP:285.

### 9.1 Direct sites (5): `$t(<err>.code, <err>.params)` inline

| # | Site | Expression | What feeds it |
|---|---|---|---|
| 1 | `src/views/FirstRun.vue:94` | `{{ $t(currentError.code, currentError.params) }}` | the first-run mkvmerge detection error |
| 2 | `src/components/RunHistory.vue:155` | `{{ $t(loadError.code, loadError.params) }}` | `list_runs` failure |
| 3 | `src/components/RunHistory.vue:241` | `{{ $t(jobLogError.code, jobLogError.params) }}` | `get_job_log` failure |
| 4 | `src/views/JobsView.vue:249` | `{{ $t(startError.code, startError.params) }}` | `start_run` rejection (`JobsView.vue:194`) |
| 5 | `src/views/JobsView.vue:255` | `{{ $t(actionError.code, actionError.params) }}` | `cancel_run` / `cancel_job` failure (`:209`, `:219`) |

### 9.2 Ref-fed sites (3): a `code` ref plus a `params` ref, assigned in catch arms

| # | Render site | Params ref declared | Assigned at |
|---|---|---|---|
| 6 | `src/components/SettingsDialog.vue:103` `{{ $t(errorCode, errorParams) }}` | `:17` `errorParams` | `:34`, `:79` |
| 7 | `src/views/BatchView.vue:445` `{{ $t(ipcErrorCode, ipcErrorParams) }}` | `:48` `ipcErrorParams` | `:119`, `:188`, `:228`, `:252` |
| 8 | `src/views/EditorView.vue:526` `{{ $t(ipcErrorCode, ipcErrorParams) }}` | `:124` `ipcErrorParams` | `:245`, `:277` |

5 + 3 = **8**.

### 9.3 What each does

All eight are the same shape: a Fluent lookup of the error's `code` with its
`params` object passed through unchanged, rendered inside a per-file alert
element. None transforms `params`; task-20's report verified this at source
(`docs/process-journal/artifacts/plan-7-sdd/task-20-report.md:78`: "All five
direct render sites ... and the three ref-fed `$t` sites forward `.params` to
`$t` unchanged; none consume a params value as a string").

The plan-7 round-1 harvest that produced the ROADMAP item:
`docs/process-journal/artifacts/plan-7-sdd/design-review-round-1.md:253-257`:
"eight scattered `$t(*.code, *.params)` IpcError render sites exist. A single
error-display funnel (component or composable) would have made D61's sweep
enumerable by construction - the house one-funnel pattern. Candidate for the
Plan-9-neighborhood registry/capability work, not a Plan-7 defect."

### 9.4 Not an `IpcError` site (explicitly excluded, and why)

`src/components/DiagnosticsPanel.vue:46-48` renders a **core `Diagnostic`**,
not an `IpcError`, and promotes numeric params via
`src/diagnosticFluentParams.ts`. The plan-7 review corrected an earlier recon
that had listed it (`docs/process-journal/artifacts/plan-7-sdd/review-0fea107..cc0e6d7.diff:3343`:
"`DiagnosticsPanel.vue` is not an IpcError site and needs no promotion
work"). Excluded here for the same reason.

### 9.5 OPEN QUESTION

Site 7 (`BatchView.vue`) is not purely an `IpcError` sink: `:228` assigns
`ipcErrorParams.value = parseDiagnostic.params` from
`doc.config_diagnostics[0]`, i.e. a core `Diagnostic`, into the same ref that
`:119`/`:188`/`:252` fill from `IpcError`. Any funnel typed strictly to
`IpcError` will have to deal with that one mixed consumer, or the
`Diagnostic` path has to move elsewhere. (The `Diagnostic` wire stays
`Record<string, string>` by design, per the same plan-7 note; `IpcError`
params were widened to `string | number` by D61/task-20.)

---

## 10. The GUI test-harness question

### 10.1 What exists today

**No Vitest, no `@vue/test-utils`, no `tauri::test` / `mock_builder`.**

> Negative-result control: `git grep -n "tauri::test\|mock_builder\|vitest\|@vue/test-utils" -- ':!.worktrees' ':!docs' ':!pnpm-lock.yaml'`
> returns nothing; the identical invocation with `playwright` added to the
> alternation returns hits in `.github/workflows/ci.yml`, `.gitignore`, and
> every `e2e/*.spec.ts`. The pathspec and pattern are sound, so the absence
> is real. `package.json`'s `devDependencies` likewise lists neither.

**Frontend / shell test infrastructure, complete inventory:**

| Layer | Artifact | What it is |
|---|---|---|
| E2E, full app | `e2e/smoke.spec.ts` + `e2e/mocks.ts` + `e2e/tauri-mock-entry.ts` + `e2e/vite.harness.config.ts` | Playwright against the **built** frontend (`vite preview` over `dist/`) in plain Chromium, with `@tauri-apps/api/mocks` standing in for the IPC bridge. `playwright.config.ts:1-9` states it explicitly: "no tauri-driver, no real webview window". |
| E2E, component | `e2e/mount.ts` + `e2e/mount-entry.ts` + `e2e/vite.mount.config.ts` | A Playwright-driven component mount harness (plan-6 wave-3 amendment): injects a standalone IIFE into a blank page and mounts one component via `window.__muxsmithMount__`. |
| Other specs | `catalogs`, `editor-dropdowns`, `editor-markers`, `editor-rule-add-remove`, `editor-tooltips`, `help-mode`, `help-topics`, `locale-switch` | mostly mount-harness driven |
| Rust shell unit tests | inline `#[cfg(test)] mod tests` in `src-tauri/src/{lib,run,error,settings}.rs` | no `src-tauri/tests/` directory exists |
| Static gates | `pnpm lint` (eslint), `pnpm build` (vue-tsc), `pnpm check:i18n` | `.github/workflows/ci.yml:119-120, 154` |

**Counts, recomputed** (`command grep -cE '^[[:space:]]*test\('` per file):

| File | tests |
|---|---|
| `e2e/smoke.spec.ts` | 30 |
| `e2e/editor-dropdowns.spec.ts` | 10 |
| `e2e/help-mode.spec.ts` | 9 |
| `e2e/editor-rule-add-remove.spec.ts` | 8 |
| `e2e/catalogs.spec.ts` | 1 |
| `e2e/editor-markers.spec.ts` | 1 |
| `e2e/editor-tooltips.spec.ts` | 1 |
| `e2e/help-topics.spec.ts` | 1 |
| `e2e/locale-switch.spec.ts` | 1 |
| **total** | **62** |

`#[test]` counts in the shell (`command grep -cE '^\s*#\[test\]'`):
`run.rs` 40, `lib.rs` 18, `error.rs` 16, `settings.rs` 8 -- **82** total.

**The mount harness cannot reach the run-path views.** `e2e/mount-entry.ts:26-29`:

```
const modules = import.meta.glob<{ default: Component }>(
  ["../src/editor/widgets/*.vue", "../src/views/EditorView.vue"],
  { eager: true },
);
```

and `resolvePath` (`:31-36`) maps any other name to
`../src/editor/widgets/<name>.vue`. `JobsView.vue`, `BatchView.vue`,
`JobRow.vue`, `LiveLog.vue`, `RunHistory.vue`, `SettingsDialog.vue` and
`FirstRun.vue` are **not** in the glob and cannot be mounted in isolation.
The harness's own doc (`:6-13`) explains why it was built that way: it
existed to reach editor UI that had no mount point in the running app.

### 10.2 What specifically is untested about `start_run`'s orchestration body

`start_run` is `src-tauri/src/run.rs:423-500`. Its body:

1. `Reservation::acquire(&state)?` (`:438`) -- synchronous single-run gate
2. clone `settings_path`, take `reservation.cancel_flag()` (`:439-440`)
3. `on_blocking(move || plan_run(...)).await?` (`:442-443`)
4. destructure `PlanOutcome`: `Soft` -> `drop(reservation)` +
   `finish_without_queue(...)` + early `return` (`:455-458`); `Ready` ->
   unbox (`:459`)
5. `reservation.commit(Arc::clone(&ctl))` (`:463`)
6. `jobs.unwrap_or(1)` (`:465`)
7. `std::thread::spawn(...)` (`:468-493`): `TeardownGuard::new` (`:479`),
   `LiveSpawner` (`:481`), `QueueOpts { jobs, fail_fast: false }` (`:482-485`),
   `run_batch(...)` with the window-emit closure (`:486-488`),
   `run_document` (`:490`), `finalize_joblog` (`:491`),
   `emit_run_finished` (`:492`)
8. `Ok(StartedRun { run_id, total_jobs, run_dir })` (`:495-499`)

**`start_run` itself is never called by anything except Tauri's generated
handler.** `git grep -n "start_run(" -- 'src-tauri/src' 'crates'` returns
exactly one line: `src-tauri/src/run.rs:424`, the definition. Its only
registration is `src-tauri/src/lib.rs:577` inside `generate_handler!`.

> Control for that negative: the same command with `plan_run(` returns four
> hits (definition `:244`, calls `:1074`, `:1109`, plus doc references), so
> the pattern finds call sites when they exist.

**What the 40 Rust tests in `run.rs` do cover** (each a *piece*, none the
composition):

| Piece | Tests |
|---|---|
| `plan_run` | `:1057` override honored, `:1104` settings failure -> `IpcError` |
| `Reservation` gate | `:1132` second start rejected mid-planning, `:1148` slot released on soft outcome, `:1159` rejected while running, `:1174` commit + teardown |
| cancel during planning | `:1202`, `:1218` |
| `run_batch` | `:1228` event ordering, `:1334` joblog files written |
| teardown / quit | `:1263`, `:1284`, `:1317`, `:1378`, `:1404`, `:1421`, `:1443` |
| `finalize_joblog` | `:1491`, `:1497`, `:1505` |
| history / job-log readers | `:1523`-`:1633` |
| `do_cancel_run` / `do_cancel_job` | `:1642`-`:1667` |

Several of these **simulate** what `start_run` does rather than invoke it;
`:1208` is explicit: `// What start_run does after planning: QueueControl::new with the ...`.

**Consequently untested:**

- the sequencing of steps 1-8 as a unit (acquire -> plan -> commit -> spawn),
  including that `commit` happens before `start_run` returns, which
  `do_cancel_job`'s doc (`:860-866`) relies on
- the `Soft` early-return path's `drop(reservation)` + `finish_without_queue`
  composition inside the real command
- the `on_blocking` / `spawn_blocking` boundary (`:443`) and the claim in the
  doc at `:382-397` that the reservation is held across the `.await` while
  not crossing into the closure
- the runner thread's body as a whole (`:468-493`): `TeardownGuard` arming,
  `run_batch`, `run_document`, `finalize_joblog`, `emit_run_finished` in that
  order, and that teardown completes only after the terminal event
- `jobs.unwrap_or(1)` (`:465`) and the hardcoded `fail_fast: false` (`:484`)
- the actual `muxsmith://job-event` / `muxsmith://run-finished` emissions
  (`:487`, `emit_run_finished:757`), since no test constructs an `AppHandle`

**And on the frontend side:** the only run-path e2e is
`e2e/smoke.spec.ts:477-609` (`test.describe("jobs view: live run")`, one
test at `:491`), which mocks `start_run` outright
(`:501` `start_run: [resolveWith(startedRun)]`) and drives job events from
the Playwright side. The Rust body never executes in e2e either.

### 10.3 Provenance of the ROADMAP wording

ROADMAP:259-262 records: "no Vitest component harness, no `tauri::test`
integration harness, `start_run`'s orchestration body untested ('to raise at
merge time'; the merge gate passed without it) (S4/S5/S6). (2026-07-11,
docs-tree sweep)". All three clauses verify as stated today, with one
qualification: a **Playwright-based** component mount harness has since
appeared (plan-6 wave-3), but it is scoped to editor widgets and
`EditorView.vue` and cannot mount the run-path views.

The same gap was flagged twice more in the fix-wave report:
`final-fix-wave-plan5-report.md:92-98` ("no Vitest/component-test harness
exists in this repo (only Playwright e2e + eslint + vue-tsc + check:i18n)")
and `:216-220`.

### 10.4 OPEN QUESTIONS

- **Whether `start_run`'s composition is testable at all without a harness
  that can build an `AppHandle`.** Every piece that does not need one has
  already been factored out (`plan_run`, `run_batch`, `finalize_joblog`,
  `finish_teardown`, `close_decision`); what remains in the body is exactly
  the part that touches `AppHandle` / `State`. `tauri::test::mock_builder`
  is the ecosystem answer and is absent from the tree; whether it is
  available for the pinned Tauri version and what it costs was not
  established by this recon.
- **Whether the mount harness's glob should be widened** rather than a second
  (Vitest) harness introduced. Widening reaches `JobsView.vue` (input 8's
  untested logic) at the cost of feeding it mocked IPC, which
  `e2e/mount.ts:6-8` explicitly does not install today ("No Tauri IPC mock is
  installed here").
- Neither question has a ledger entry; `git grep` for a Tier-1 entry on the
  GUI test harness found none under `docs/decision-ledger.yaml` while
  searching for the plan-9 inputs.

---

## Appendix: files touched by the ten inputs

Core (`crates/muxsmith-core/src`): `executor/queue.rs`, `executor/job.rs`,
`executor/joblog.rs`, `executor/spawn.rs`, `profile/validate.rs`,
`report/json.rs`, `report/mod.rs`, `matcher.rs`, `planner.rs`.

CLI (`crates/muxsmith-cli/src`): `commands/mod.rs`, `commands/dry_run.rs`,
`commands/run.rs`, `commands/validate.rs`.

Shell (`src-tauri/src`): `lib.rs`, `run.rs`.

Frontend (`src`): `views/JobsView.vue`, `views/BatchView.vue`,
`views/EditorView.vue`, `views/FirstRun.vue`, `components/RunHistory.vue`,
`components/SettingsDialog.vue`, `components/JobRow.vue`, `jobRowState.ts`,
`ipc.ts`.

Tests / harness: `e2e/*`, `crates/muxsmith-cli/tests/{run_cli,run_live,catalog_completeness}.rs`,
inline `#[cfg(test)]` modules in the shell.
