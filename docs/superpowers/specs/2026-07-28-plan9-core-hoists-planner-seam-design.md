# Plan 9 design: core/orchestration hoists + planner seam

Status: owner-approved 2026-07-28, after one four-eyes review round (four
blocking findings, five minor), one fix round, an APPROVED delta review by
the same reviewer, and two non-blocking wording notes closed in a second
fix round; every amendment is recorded in the `## Amendment log` section
at the end. Numbering starts at **D91** per the Plan-9 brief.
Verified 2026-07-28 against the current tree: a repo-wide grep for `D91`
(excluding `.git`, `.worktrees`, `node_modules`, generated output and the
brief itself) returns exactly one hit, a hypothetical mention in
`docs/process-journal/artifacts/plan-8.5-sdd/plan-amendment-verdict.md:60`
("a bound excluding a hypothetical future D91"), which reserves nothing;
the last ADR in use is D90 (`2026-07-22-plan8-packaging-release-design.md`,
whose `^## D` headings run D75-D90; Plan 7.5 used D65-D72).

Scope per the ROADMAP Plan-9 anchor (`docs/ROADMAP.md`, Plan-9 section,
commit `90bc3ae`): the **eight owner rulings of the 2026-07-28 S24 kickoff
are binding and not re-litigated here** - (1) the four-copy pipeline hoists
into a shared core `plan_pipeline()` that IS the planner seam, carrying the
half-done Plan-5.6 `config_diagnostics` funnel migration; (2) `run_batch`
hoists into `muxsmith_core::executor`; (3) the src-tauri runs-root copy is
DELETED, not hoisted, and the CLI gate stays; (4) the worker-panic path is
one item: the payload travels in the `JobOutcome`, the surfaces render it,
no logging facade in core; (5) bare `raw:` with an empty property name is
an ERROR at validate under its own new DiagCode; (6) `config_diagnostics`
sorts errors-first centrally in core's two document builders, and
BatchView's index-0 read moves to code-keyed detection in the same change;
(7) the D23 item is a test plus a ledger entry, not a code fix, with the
mount-glob widening as the only harness work; (8) the D49 G1/G2 removal
experiment runs in this plan. Each becomes a D-entry that designs its
mechanics. **Every fork in this document is closed.** No design-latitude
clause appears in it, in either form (explicit permission or omission);
every set an implementer needs is enumerated, every user-visible string is
written out verbatim in both locales.

Grounding: the v1 design spec `2026-07-08-muxsmith-v1-design.md` (sections
4.4, 5.1, 5.2, 5.4, 5.5, 6, 7, 8.1, 8.2, 8.4, 9, 10 read; authoritative
above this design); the ROADMAP Plan-9 anchor with its RECON block and
IN/OUT rulings; the recon inventory
`.superpowers/sdd/plan-9/recon-inventory.md` (produced 2026-07-27 against
`c2514e7`, two commits behind; where its evidence is load-bearing it is
quoted inline below, since `.superpowers/` is git-ignored and moves at the
plan close); the six ledger entries `core-121-planner-seam-and-hoist`,
`exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`,
`cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`,
`empty-bare-raw-property-rejected-at-validate` (read in
`docs/decision-ledger.yaml`; statements and steelmen honored, not
re-litigated); Tier-2 `docs/conventions.yaml` (`core-37-prose-free-core`,
`core-derive-dont-restate`, `code-comment-line-citations-drift`,
`exec-19-delete-partial-errsurface`, `i18n-05-plural-selectors`),
`docs/process-conventions.yaml` (`proc-latitude-clause-boundary`,
`latitude-carveout-presentation-tokens`, `proc-proposed-safeguard-stays`,
`proc-verification-step-must-be-falsifiable`, `proc-06-mkvtoolnix-parity`,
`proc-07-verify-against-source`), `docs/product-boundaries.yaml` (scanned
for touched boundaries; none is changed by this design); the tree itself
(every file the recon appendix names for the in-scope inputs was opened:
`crates/muxsmith-core/src/{executor/{queue,job,joblog,spawn},profile/validate,report/{mod,json},matcher,planner,identify,capability/runtime}.rs`,
`crates/muxsmith-cli/src/commands/{mod,dry_run,run,validate}.rs`,
`src-tauri/src/{lib,run}.rs`, `src/{views/{JobsView,BatchView},components/{JobRow,RunHistory},jobRowState.ts,ipc.ts,i18n/index.ts}`,
`e2e/{mount.ts,mount-entry.ts,mocks.ts,tauri-mock-entry.ts,vite.mount.config.ts,smoke.spec.ts,global.d.ts}`,
`locales/{en,de}/{diagnostics,cli,gui-jobs}.ftl`,
`crates/muxsmith-cli/tests/catalog_completeness.rs`,
`crates/muxsmith-core/tests/suggestions.rs`); SI-3 parity sources: the
mkvtoolnix source at `~/Downloads/mkvtoolnix` (version 100.0 per
`configure.ac` `AC_INIT([MKVToolNix],[100.0],...)` and `NEWS.md` header
`# Version 100.0 "Do Hot Girls Like Chords" 2026-07-05`, cited by symbol
anchor plus that version, per `code-comment-line-citations-drift`) and the
installed `mkvmerge v100.0`, run live 2026-07-28 (section 1).

**Anchor re-verification.** The recon was written two commits behind
master, so every line anchor this design makes load-bearing was re-verified
by printing the cited line on 2026-07-28: `validate.rs:268` and `:310`
(both `strip_prefix("raw:")` arms), `matcher.rs:95`, `planner.rs:796`
(`collect_raw_props`'s strip), `queue.rs:396` (the `eprintln!`) and `:408`
(the token construction), `job.rs:208` (`delete_partial_failed` push),
`BatchView.vue:225` (`doc.config_diagnostics[0]`), CLI `run.rs:331` and
src-tauri `run.rs:830` (the two `MUXSMITH_RUNS_ROOT` reads), CLI
`run.rs:487` (the `"n/a"` literal), src-tauri `run.rs:782-804`
(`run_batch`), `resolve_runs_root`'s three call sites (`run.rs:326`,
`:529`, `:535`), and the four pipeline spans (`dry_run.rs:38-130`,
CLI `run.rs:60-192`, `lib.rs:205-242`, src-tauri `run.rs:251-344`). None
drifted; the recon's measurements hold at master.

---

## 0. Notes and corrections to the brief

Each checked against the tree or a live run before anything was built on it.

| # | Brief/anchor statement | Reality |
|---|---|---|
| 1 | Recon inventory "1119 lines" (brief section 2.3, ROADMAP RECON block) | **The brief's and ROADMAP's figure is correct; this design's first draft mis-corrected it.** The draft asserted "`wc -l` reports 1120", attributing a reader-tool's line count to a command that was never run. Measured in fix round 1 (2026-07-28): `wc -l` and `awk 'END{print NR}'` both report **1119**, trailing newline present (`tail -c 1 \| xxd` = `0a`). Inverted rather than dropped so notes 2-5 keep their numbers and the mis-correction stays on record: an evidence line may only contain output that was pasted, not recalled. |
| 2 | Fork 16 premise (and ROADMAP anchor): BatchView "reads config_diagnostics[0] **to detect** a parse failure" and "one consumer **depends on the order** and moves in the same change" | **Partially refuted with evidence.** The detection is `!doc.profile` (`BatchView.vue:218-226`); index 0 is only the *fetch* of the explaining diagnostic. On that envelope the vector is always a **singleton**: `load_profile_body`'s `Err` arm builds `config_only_document(&[d], ...)` from exactly one diagnostic (`src-tauri/src/lib.rs:316-320`), and `load::from_file` can only produce `DiagCode::ParseError` - its two Diagnostic constructors are `load.rs:62` (I/O arm) and `load.rs:71` (deserialize arm), both `ParseError`. A one-element vector cannot be reordered, so **the central sort changes no BatchView behavior**. The ruled change is still made (D103) - code-keyed fetch is the robust form and the ruling is binding - but it is a hardening, not a behavior-preserving migration. |
| 3 | Fork 17 premise / ruling 7: "the mount-glob widening for `JobsView.vue` is the only test-harness work in scope" | **Insufficient as literally stated, surfaced rather than silently complied with or expanded.** The mount harness passes `spec.props` once, statically (`e2e/mount-entry.ts`, `h(Comp, { ...spec.props, ... })`); `JobsView`'s reset logic only runs on a *change* of the `pendingRun` prop, and its divergent branch (the reason ruling 7 exists) needs a *second* dispatch while `runActive` is true. A glob-only widening therefore makes JobsView mountable but the ruled test unwritable. D104 adds the minimal extra harness affordance (a reactive props hook, `window.__muxsmithSetProps__`) and records it as a deliberate, surfaced deviation from the brief's "only the glob" wording - the alternative satisfies the letter and defeats the item's purpose. No other harness work is added; the two OUT items stay untouched. |
| 4 | Recon 1.3 D-2: CLI `Some(false)` means "absent from PATH" | Slight simplification, no consequence for the ruling: `Mkvmerge::locate` maps a spawn failure to `NotFound` but passes `NonZero`/`Parse` errors through (`capability/runtime.rs:94-104`), and both CLI copies match `Err(_)` - so the CLI's `Some(false)` already covers "found but `--version` unusable" too. This *narrows* the semantic gap D92 has to close: on both surfaces the honest reading is "no usable mkvmerge was resolved"; only `TooOld` is GUI-exclusive. |
| 5 | Recon 5.4: "Two GUI surfaces would need a new field, not just a renderer: `JobRowData` and the history row both drop `errors`" | Half true for the live row. `JobRowData` needs **no new field**: its `finished` state already embeds the whole `JobOutcome` (`src/jobRowState.ts`, `{ kind: "finished"; outcome: JobOutcome }`), so once `JobOutcome` itself carries the payload (D98), `JobRow.vue` can render it with zero model change. The history table row is out of the ruled scope (ruling 4 names the live job row and rules the RunHistory *export* stays raw-output-only); the persisted record still gains the payload for triage (D98). |

---

## 1. SI-3 parity audit (the complete set, per the brief's section 6)

Licensing boundary (`proc-06-mkvtoolnix-parity`): behavior, facts and
interfaces only; no literal code or text is taken; no wording below is
modeled on mkvtoolnix text.

### 1.1 Per-job failure reporting (feeds D99/D100)

What mkvtoolnix-gui does (source, version 100.0, symbol anchors):

- `mtx::gui::Jobs::Job` keeps per-job `QStringList const &warnings()` and
  `errors()` (`src/mkvtoolnix-gui/jobs/job.h`, the two accessors), with
  `acknowledgeWarnings()` / `acknowledgeErrors()` alongside.
- The queue row itself signals the condition: the jobs model's status-icon
  column sets an error icon when `!job->errors().isEmpty()`, else a warning
  icon when warnings exist (`src/mkvtoolnix-gui/jobs/model.cpp`, the
  `setIcon(numErrors ? m_errorsIcon : numWarnings ? m_warningsIcon : ...)`
  line in the item-update path).
- The "job output" tool gives every job three panes - output, warnings,
  errors - and routes each read line by its `LineType`
  (`src/mkvtoolnix-gui/watch_jobs/tab.cpp`, the
  `Jobs::Job::WarningLine == type ? p->ui->warnings : p->ui->errors`
  routing; `mux_job.cpp` classifies lines via `Q_EMIT lineRead(line,
  WarningLine/ErrorLine)`).

Muxsmith today: the GUI job row shows a state chip plus a warning count and
drops `errors` entirely (`src/components/JobRow.vue` renders `stateKey` and
`warningCount` only); error *lines* reach the LiveLog pane and the
persisted job log as raw output (D24), but a worker panic produces no
output line at all, so it is invisible on every GUI surface.

Classification:

- **Genuine gap, partially closed by ruling 4's scope:** mkvtoolnix-gui
  surfaces per-job error state on the queue row and the error text in a
  per-job pane; Muxsmith's row shows nothing for a failed job beyond the
  chip. D100 closes the ruled part: the row carries the worker-panicked
  catalog message in its failure state.
- **Justified divergence, kept:** a dedicated per-job errors pane
  (mkvtoolnix's three-pane job output) is not built. Muxsmith's mkvmerge
  error lines already reach the user twice - live (`run-job-notice` lines
  on the CLI, LiveLog rows in the GUI, both fed by the tagged
  `JobEvent::Error`) and persisted (`job-<index>.json` `errors` plus raw
  `lines`) - so the remaining delta is presentation topology, not lost
  information. The one payload that reached *no* surface is the worker
  panic, which is exactly what ruling 4 fixes.
- **Justified divergence, kept:** mkvtoolnix's acknowledge-warnings/errors
  workflow has no Muxsmith analogue (no persistent queue across sessions to
  acknowledge against; Muxsmith's run history is read-only, D26).

### 1.2 Diagnostic ordering in mkvmerge's own output (feeds D102)

Empirical, installed `mkvmerge v100.0`, run 2026-07-28:

```
$ mkvmerge -o out.mkv --sub-charset 0:UTF-8 --stracks 99 in.srt
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
'in.srt': Using the demultiplexer for the format 'SRT subtitles'.
Warning: 'in.srt': A track with the ID 99 was requested but not found in the file. The corresponding option will be ignored.
The file 'out.mkv' has been opened for writing.
Progress: 100%Multiplexing took 0 seconds.
exit=1
```

```
$ mkvmerge -o out2.mkv --stracks 99 in.srt --chapters missing-chapters.xml
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
Error: Could not open 'missing-chapters.xml' for reading.
exit=2
```

Source confirmation (mkvtoolnix 100.0): `default_mxerror` in
`src/common/output.cpp` prints the error via `mxmsg(MXMSG_ERROR, ...)` and
immediately calls `mxexit(2)`.

Reading: mkvmerge output is **strictly chronological and error-terminal** -
warnings print inline where they occur, and the first error ends the
process, so an "errors-first" collation cannot even arise in its output.
mkvmerge has no report-document analogue (single-shot binary, fail-fast).
This is evidence *neither for nor against* errors-first in a batch report:
the comparable object does not exist upstream. What does exist upstream and
in-tree is the CLI's own `validate`, which already sorts errors-first for
both output modes (`crates/muxsmith-cli/src/commands/validate.rs:19-29`,
"Error-first, stable within a severity; both output modes share it") - the
parity that ruling 6 restores is Muxsmith-internal, and D102 records it as
such rather than claiming an mkvmerge precedent.

### 1.3 Nothing else

The pipeline hoist, the planner seam, the runs-root debug gate and the
`raw:` empty-name rejection have no mkvtoolnix analogue: mkvmerge is a
single-shot binary with no declarative profile, no planner seam and no
persistent runs root, and `raw:` is a Muxsmith-only construct. Parity was
consulted for exactly the two comparisons above and nowhere else.

---

## D91: The planner seam is `muxsmith_core::pipeline::plan_pipeline`; the four copies become presentation-only mappings of its outcome

**Decision (ruling 1's mechanics; closes forks 1 and 2's D-1/D-3/D-4/D-5/D-7).**

A new core module `crates/muxsmith-core/src/pipeline.rs` (module name
`pipeline`, added to the spec 7 module table by amendment S-4) owns the
shared orchestration `load -> config-diagnostics funnel -> resolve mkvmerge
-> list_languages -> RunInputs -> LiveIdentifier -> plan_batch`. This
function IS the injectable planner seam (S4/S5/S6, ledger `core-121`):

```rust
/// One shared planning pipeline for every surface (spec 5.5, 7): the
/// injectable planner seam. Each variant is data; presentation (documents,
/// exit codes, stderr lines, IPC shapes) stays with the caller.
pub enum PipelineOutcome {
    /// `load::from_file` failed; planning and the mkvmerge lookup never ran.
    LoadFailed { diagnostic: Diagnostic },
    /// The injected resolver produced no usable mkvmerge (D92 defines the
    /// shared meaning); config-time diagnostics were still collected.
    MkvmergeUnavailable { config_diags: Vec<Diagnostic> },
    /// mkvmerge resolved but `list_languages` failed (broken installation).
    QueryFailed { config_diags: Vec<Diagnostic> },
    /// Planning ran. Boxed for the same `large_enum_variant` reason as
    /// src-tauri's `PlanOutcome::Ready`.
    Planned(Box<PlannedPipeline>),
}

pub struct PlannedPipeline {
    pub config_diags: Vec<Diagnostic>,
    pub batch: Batch,
    /// The loaded profile, returned so callers keep presentation-side
    /// access (`print_batch_human` needs `profile.input.extensions`).
    pub profile: Profile,
    /// The effective source directory (the `.`-default of D95 applied),
    /// returned because `print_batch_human` renders it.
    pub source: PathBuf,
    /// The resolved mkvmerge, returned because the run surfaces need its
    /// path for the spawner.
    pub mkv: Mkvmerge,
}

pub fn plan_pipeline(
    profile_path: &Path,
    source: Option<PathBuf>,
    output: Option<PathBuf>,
    on_collision: Option<CollisionPolicy>,
    resolve_mkvmerge: impl FnOnce() -> Result<Mkvmerge, RuntimeError>,
) -> PipelineOutcome
```

Internal step order is exactly the recon's S1-S7, byte-behavior-preserving:
`load::from_file` (S1); `profile::validate::config_diagnostics(&profile)`
(S2 - this **is** the ruled A-1 funnel migration: all four copies stop
inlining `validate::validate` + `lint::provable_overlaps`; the migration's
completion check is stated exactly once, as acceptance observable 1 in
section 7, and is not restated here, per the ledger rule
`design-states-a-completion-check-once`); `resolve_mkvmerge()` (S3); `mkv.list_languages()`
(S4); `RunInputs { source: source.unwrap_or_else(|| PathBuf::from(".")),
output, on_collision }` (S5, see D95); `LiveIdentifier` over a
pipeline-constructed `IdentifyCache::new()`, dropped when the call returns
(S6; per-call cache, D93); `plan_batch(&profile, &run, &mut
ident, &lang)` (S7). The near-verbatim rationale comments duplicated
between the CLI copies (recon A-2, 25-26 lines per copy) consolidate into
this module's docs; the S3e/S4e per-branch rationale moves onto the
variants.

**What each of the four call sites becomes** (fork 1's last clause):

1. `crates/muxsmith-cli/src/commands/dry_run.rs::run` - calls
   `plan_pipeline(profile_path, source, output,
   on_collision, Mkvmerge::locate)` and matches: `LoadFailed`
   -> today's json/human branch, return 2; `MkvmergeUnavailable` ->
   `config_only_document(.., Some(false), ..)` / sorted human +
   `mkvmerge-not-found` stderr, return 2; `QueryFailed` -> same with
   `Some(true)` / `mkvmerge-query-failed`, return 2; `Planned` ->
   `batch_document` / sorted human + `print_batch_human(&batch,
   &planned.source, &planned.profile.input.extensions, renderer)`, then
   `diag_exit_code`. Nothing about the printed bytes or exit codes changes.
2. `crates/muxsmith-cli/src/commands/run.rs::run` - same mapping with the
   `run_document(...)` wrappers it has today, then `pipeline::job_specs`
   (D94), the empty-specs branch (unchanged presentation), and the queue
   via core `run_batch` (D96).
3. `src-tauri/src/lib.rs::dry_run_body` - `plan_pipeline(profile_path,
   source, output, None, || Mkvmerge::detect(mkvmerge_override))`
   mapped to the three `config_only_document` shapes and
   `batch_document`, exactly today's returns.
4. `src-tauri/src/run.rs::plan_run` - keeps its settings read first (D-7,
   below), then the same call with the detect closure; `LoadFailed`/
   `MkvmergeUnavailable`/`QueryFailed`/empty-specs map to
   `PlanOutcome::Soft` exactly as today; `Planned` feeds the existing
   `ReadyPlan` construction (`mkv_path` from `planned.mkv.path()`).

**The seven deliberate divergences, individually** (fork 2; a hoist may
only turn a divergence into a parameter, never flatten it):

| Divergence | Resolution |
|---|---|
| D-1 `locate()` vs `detect(override)` | **Becomes parameter** `resolve_mkvmerge: impl FnOnce() -> Result<Mkvmerge, RuntimeError>`. CLI passes `Mkvmerge::locate` (PATH-only, spec 8.1 defines no CLI override flag - verified: the only `Mkvmerge::` resolver calls in `crates/muxsmith-cli/src` are three `locate()` sites); GUI passes a `detect(override)` closure (spec 8.2 settings override, D28). The "Fix (CRITICAL)" history on src-tauri `run.rs` (its own doc: plan_run once silently used `locate`) is exactly why this is an explicit parameter at the seam rather than an internal default. |
| D-2 `mkvmerge_found` semantics | **Wire-contract decision, own entry: D92.** |
| D-3 `on_collision` | **Becomes parameter** `on_collision: Option<CollisionPolicy>`. CLI passes its `--on-collision` flag value; both GUI call sites pass `None` (no GUI control exists; recorded as a trigger in section 6, not decided here - the recon found no ledger entry ruling it and this design does not invent one). |
| D-4 human-vs-JSON ordering | **Stays per-surface**: `severity_sorted` printing is CLI presentation; after D102 the JSON side is centrally sorted, so the modes converge without the seam touching rendering. |
| D-5 return shape (`i32` / fall-through / `Value` / `Result<PlanOutcome, IpcError>`) | **Stays per-surface**: `PipelineOutcome` is data; each surface keeps its terminal mapping. This is the presentation boundary and the reason the seam returns an enum instead of printing or serializing anything (spec 5.2/7: core emits no prose). |
| D-6 specs gate on copies 2+4 only | **Shared derivation, per-surface gate: D94.** |
| D-7 `plan_run`'s settings-read-first (`?` -> `IpcError`) | **Stays caller-side.** The CLI has no settings; `dry_run`'s wrapper does its own read (`lib.rs:392-401`). Pulling an app-settings concept into core would invert the layering for one surface's convenience. The layer asymmetry between the two GUI copies (wrapper-side vs body-side read) is also left as is: both feed the same closure parameter, and unifying them is shell refactoring with no behavioral content. |

**Interface changes (memo-recorded here at decision time):** new public
core items `pipeline::plan_pipeline`, `pipeline::PipelineOutcome`,
`pipeline::PlannedPipeline`, `pipeline::job_specs` (D94), plus
`report::severity_sorted` (D102). `LiveIdentifier` is unchanged (D93,
amendment 1).
No serialized wire format changes in this entry (D92/D98 carry their own).

**Rejected alternatives.** (a) Hoisting into `planner.rs`: the pipeline
spans profile/capability/identify/planner/command concerns and planner.rs
is already the largest core module; a cross-module orchestrator is a new
module by the spec 7 module-responsibility table's own logic. (b) A
builder-style seam object (`Pipeline::new().with_resolver(..)`): four call
sites do not earn the indirection (scale-appropriate design); a function
with six explicit parameters is the simplest mechanism that closes every
divergence. (c) Taking a `&mut dyn Identify` parameter: the identifier
cannot exist before the pipeline resolves mkvmerge, since `LiveIdentifier`
borrows the resolved `Mkvmerge`; the pipeline therefore constructs its own
identifier over its own per-call cache (D93). The `Identify` seam stays
where it is, on `plan_batch`, for tests.

## D92: `mkvmerge_found: false` means "no usable mkvmerge was resolved", uniformly, on every surface; no new wire field

**Decision (fork 3, the wire-contract fork).**

The shared document field `mkvmerge_found` gets one meaning, documented on
`report::json::config_only_document` and normative for every surface: **`false`
= the surface's resolver produced no usable mkvmerge for planning**, where
"usable" is defined by the resolver the surface injects (D91/D-1): for the
CLI, PATH lookup failed or the found binary could not answer `--version`;
for the GUI, the detect ladder failed for any reason **including `TooOld`**.
`true` keeps its existing meaning (a usable mkvmerge resolved, the
subsequent `list_languages` query failed); the key stays absent on a
profile-load failure. `PipelineOutcome::MkvmergeUnavailable` deliberately
carries **no failure reason**: no surface consumes one today (both GUI
bodies fold every detect error to `Some(false)`; both CLI copies match
`Err(_)`), and an uncarried reason is speculative wire surface.

This adopts the GUI's documented reading (`src-tauri/src/lib.rs:192-198`:
"`Some(false)` when `Mkvmerge::detect` itself fails for ANY reason,
including `TooOld` - a too-old mkvmerge is exactly as unusable for planning
as a missing one") as the shared contract, which note 4 in section 0 shows
is already the honest description of the CLI side too.

**Cost accepted, named:** a JSON consumer cannot distinguish
absent-from-PATH from present-but-too-old from the report document alone.
The distinction is already served elsewhere on each surface: the GUI's
first-run/`detect_mkvmerge` flow carries `found`/`minimum` for `TooOld`,
and the CLI's human channel prints `mkvmerge-not-found`. A trigger in
section 6 names the observable event that would reopen this as a document
field.

**Rejected alternative (the fork's other arm):** carry the resolver's
failure reason as data (a `mkvmerge_error` field or a reason on the
variant). Rejected as unconsumed wire growth; the ADR records it as the
designated mechanism if the section-6 trigger ever fires, so reopening is
an addition, not a redesign.

No spec amendment needed: the spec never mentions `mkvmerge_found`
(verified: `grep -n "mkvmerge_found" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
returns nothing; the same grep over `src-tauri/src` returns the lib.rs doc
lines, so the pattern is sound). The builder doc comment in
`report/json.rs:66-73` is updated to the unified wording.

## D93: The seam constructs its own `IdentifyCache` per call; no session cache, no interface change (owner ruling, amendment 1)

**Decision (fork 4; re-ruled by the owner 2026-07-28, amendment 1: the GUI
identification session cache is out as overengineering).**

`plan_pipeline` takes no cache parameter. It constructs
`IdentifyCache::new()` internally at S6, wraps it in the `LiveIdentifier`
it builds over the resolved `Mkvmerge`, and drops both when the call
returns - exactly what all four copies do today (recon A-5, re-verified at
the four S6 anchors). Consequences, each an explicit non-change:
`LiveIdentifier` keeps owning its cache (no borrow change, no public
interface change); `crates/muxsmith-core/tests/command_integration.rs:231`
and `:493` stay untouched; `AppState` gains no field; no mutex, no `Arc`,
no state plumbing anywhere.

**Cost accepted, named (owner-accepted with the ruling; recorded so it is
not re-litigated as an oversight):** a GUI dry-run followed by a run
identifies every file twice - one `mkvmerge -J` process spawn per file per
command, rather than once per session. Within a single call the cache
still earns its keep: `plan_batch`'s own lookups and the suggestion
engine's re-simulation passes identify each unchanged file once per
invocation.

**Spec consequence, handled rather than hidden:** spec 5.5's sentence
"Identification cache: in-memory per session, keyed on path + mtime +
size, shared between dry-run and run" is false for the GUI under this
ruling, where dry-run and run are separate calls that share nothing. The
spec is authoritative and does not get left contradicted: amendment S-8
(section 3) replaces the sentence with what the product does - per-call in
the GUI, per-process in the CLI, where call and process coincide.

**Rejected alternative (the first draft's approved position, overturned by
this ruling; steelman kept honest):** a caller-owned cache
(`plan_pipeline(.., cache: &mut IdentifyCache)`, `LiveIdentifier`
borrowing) plus a GUI session cache in `AppState`
(`Arc<Mutex<IdentifyCache>>`, cloned into and locked inside the blocking
closures). It realized spec 5.5's sharing sentence as written, and the
path + mtime + size key made the reuse staleness-free; its costs were a
mutex serializing concurrent GUI planning and monotonic per-session
growth. Overturned because the owner judges the feature unnecessary at
this product's scale: the double identification is cheap, and a seam
parameter plus app-state plumbing to avoid it reads as overengineering -
the accepted price is the double `mkvmerge -J` spawn set named above.

## D94: The specs derivation moves to core as `pipeline::job_specs(&Batch)`; the empty-specs branch stays per-surface

**Decision (fork 5; resolves divergence D-6).**

The byte-identical S9 block of copies 2 and 4 (recon 1.2 quotes it; both
sites re-verified) becomes one core function in `pipeline.rs`:

```rust
/// Derives the executable job specs from a planned batch: files with an
/// error-severity diagnostic already carry `plan: None` (spec 5.1), so
/// this filter_map is also the "does this file get muxed" gate.
pub fn job_specs(batch: &Batch) -> Vec<JobSpec> {
    batch.files.iter().filter_map(|f| f.plan.as_ref())
        .map(|p| JobSpec { argv: command(p), output: p.output.clone() })
        .collect()
}
```

Only the two run surfaces call it (the dry-run surfaces have no use for
argv), so nothing is computed for nothing. The S10 empty-specs *branch* -
what to emit when nothing plans - stays per-surface, because the two
surfaces deliberately emit differently (copy 2: document only under
`--json`, `diag_exit_code` return; copy 4: always `Soft(run_document(..))`
emitted on `muxsmith://run-finished`); that difference is divergence D-6
and it survives as caller presentation, gating on `specs.is_empty()` of the
shared derivation.

**Rejected alternatives.** (a) Deriving specs inside `plan_pipeline` for
all four callers: computes argv on dry-run paths that never use it and
couples the seam's return to executor types for surfaces that never
execute. (b) Making specs a `Batch` field (planner output): inverts the
spec 7 module layering (`command` is downstream of `planner`; the planner
would import command generation), for no consumer gain.

## D95: The `PathBuf::from(".")` source default is applied once, inside the seam; GUI-observable behavior does not change

**Decision (fork 6).**

`plan_pipeline` takes `source: Option<PathBuf>` and applies the single
shared default `source.unwrap_or_else(|| PathBuf::from("."))` at S5. The
lib.rs note (its verbatim rationale: "No natural 'current directory' for a
bundled desktop app, but kept for parity with the CLI's own fallback
(dry_run.rs); in practice the batch view (T10) always supplies an explicit
source directory") moves onto the seam's doc, which fixes the recon's
observation that `plan_run` carries the same fallback without the note -
after the hoist there is exactly one fallback and one note.

**What a user would observe change: nothing.** All four copies carry the
identical default today (S5 anchors re-verified); one shared application of
it is byte-behavior-equal on every surface, CLI and GUI, including the
programmatic IPC edge (a `dry_run`/`start_run` invocation without `source`
still plans against `"."`).

**Rejected alternatives.** (a) Requiring `source` from the caller: either
re-duplicates the default at four call sites (the drift this plan exists to
remove) or forces a new GUI error path for an absent source - a new
user-visible string and a behavior change on a path the batch view never
exercises, with no ruling covering it. (b) A dedicated
`SourceDefault::Cwd | Required` parameter: an abstraction the two-value
scale has not earned.

## D96: `run_batch` hoists verbatim into `muxsmith_core::executor::queue`; the boundary is exactly where today's function ends

**Decision (ruling 2's mechanics; closes fork 7).**

The src-tauri `run_batch` (`src-tauri/src/run.rs:782-804`, signature and
23-line body re-verified) moves as-is to
`crates/muxsmith-core/src/executor/queue.rs`, public, keeping the exact
signature:

```rust
pub fn run_batch(
    specs: &[JobSpec],
    spawner: &(dyn Spawn + Sync),
    opts: QueueOpts,
    ctl: &Arc<QueueControl>,
    mut logger: Option<RunLogger>,
    mut on_event: impl FnMut(&JobEvent),
) -> (Vec<JobOutcome>, Option<RunLogger>)
```

**The boundary question, answered: the shared function ends where
`run_batch` ends today.** It does NOT absorb the `run_document` +
logger-finish pair. Rationale: the two surfaces deliberately fold that pair
differently - `finalize_joblog` (`src-tauri/run.rs:813-821`) folds
`finish`'s result into a `JoblogStatus` enum spliced into an emitted event,
while the CLI folds it into `run-joblog-written`/`run-joblog-incomplete`
stdout/stderr messages plus its own document/summary printing order
(`crates/muxsmith-cli/src/commands/run.rs:274-304`). Absorbing them would
either flatten a real presentation divergence or return a
both-shapes-carrying struct that each caller unpacks - more interface for
zero removed duplication (the two folds share no line of code today).
`run_batch`'s doc already states the reason the logger comes back out
still-open: "`finish` needs the very document it is about to persist".

**Caller-side facts, confirmed not moved (the fork's explicit checklist):**
`TeardownGuard` stays wrapped AROUND the GUI call on the runner thread
(`src-tauri/run.rs:479`; its doc: a join-panic must still clear the slot
and honor a pending quit) - the CLI has no analogue and gains none;
`fail_fast` stays caller-built inside `QueueOpts` (CLI from its flag,
`run.rs:207`; GUI hardcoded `false` at `src-tauri/run.rs:482-485`) - the
one behavioral divergence the recon's 2.2 table found, preserved as the
caller's value.

The CLI's inline block (`crates/muxsmith-cli/src/commands/run.rs:241-272`:
`mpsc::channel` at `:241`, scope at `:249`, the byte-identical
`expect("queue worker thread panicked")` at `:271`) is replaced by a call
whose `on_event` closure carries the per-event work the recon classed (a):
`if json { return; } for line in milestones.render(event, total, renderer)
{ println!("{line}"); }`. The logger tee order is preserved exactly: core
`run_batch` tees the logger before invoking `on_event`, which is the CLI's
current order (tee at `:258-260`, render at `:261-268`) - persistence stays
unconditional under `--json` (spec 6, D26).

The two existing `run_batch` behavior tests move with the function into
`queue.rs`'s inline test module:
`run_batch_emits_started_output_finished_in_order` and
`run_batch_writes_job_log_files` (currently `src-tauri/src/run.rs`, names
re-verified; both use core-owned `FakeSpawner`/`RunLogger`, and `tempfile`
is already a core dev-dependency via `tests/joblog.rs`). The shell keeps
every teardown/reservation/cancel test - those exercise shell composition,
not `run_batch`.

## D97: The src-tauri runs-root seam is deleted; the three call sites call `default_runs_root()` directly

**Decision (ruling 3's mechanics; closes fork 19).**

`resolve_runs_root` (`src-tauri/src/run.rs:827-838`) is deleted together
with its doc comment. Its THREE call sites (complete set, re-verified by
grep) and what each calls instead:

| Call site | Today | Becomes |
|---|---|---|
| `plan_run`, `src-tauri/src/run.rs:326` | `resolve_runs_root().and_then(\|root\| RunLogger::create(&root, &run_id, &specs).ok())` | `default_runs_root().and_then(...)`, unchanged tail |
| `list_runs`, `:529` | `list_runs_in(resolve_runs_root().as_deref())` | `list_runs_in(default_runs_root().as_deref())` |
| `get_job_log`, `:535` | `get_job_log_in(resolve_runs_root().as_deref(), ..)` | `get_job_log_in(default_runs_root().as_deref(), ..)` |

`default_runs_root` is already core
(`crates/muxsmith-core/src/executor/joblog.rs:51-53`). The CLI gate stays
byte-identical (`crates/muxsmith-cli/src/commands/run.rs:330-335`).

**Debug-build behavior lost, stated per site:** in `debug_assertions`
builds, setting `MUXSMITH_RUNS_ROOT` no longer redirects (a) where a GUI
run writes its logs, (b) which directory `list_runs` enumerates, (c) where
`get_job_log` reads records. Release builds lose nothing (the env var was
never read there).

**Evidence that nothing consumes it, reproduced independently of the recon
(not borrowed):** `grep -rn "MUXSMITH_RUNS_ROOT"` over the tree (excluding
`.worktrees` and `docs/`), run 2026-07-28, returns exactly the two
producers (`crates/muxsmith-cli/src/commands/run.rs:331`,
`src-tauri/src/run.rs:830`, plus their own doc comments) and five
consumers, all CLI tests: `crates/muxsmith-cli/tests/run_cli.rs:172` and
`run_live.rs:110, 245, 370, 468`. Nothing under `e2e/` or `src-tauri` sets
the variable; the src-tauri tests that touch runs-root paths call
`list_runs_in`/`get_job_log_in` with an explicit `Option<&Path>`
(`src-tauri/src/run.rs:1575-1633` test block), bypassing the deleted
function entirely, so no test changes. Fire-verification of that negative:
the identical invocation with the pattern `default_runs_root` returns
fifteen hits (measured 2026-07-28), so pathspec and pattern demonstrably
produce output.

The ledger entry `exec-43-runsroot-debug-gated` already carries the ruled
statement and the hoist-as-steelman; the plan close's promotion funnel
handles it (ROADMAP close action), nothing for this design to add.

**Rejected mechanics alternative:** keeping a trivial `resolve_runs_root`
wrapper that merely forwards to `default_runs_root()` - the name would
promise resolution logic that no longer exists, and an empty indirection at
three call sites is exactly the shape this plan deletes elsewhere.

## D98: The worker-panic payload travels as a new typed `JobOutcome.panic` field; the `errors` token and `delete_partial_failed` stay unchanged; the `eprintln!` is deleted

**Decision (ruling 4's mechanics; closes forks 8, 9 and 12).**

**Fork 8 - how the payload travels: a new typed field.**

```rust
/// The downcast panic payload when a queue worker died running this job
/// (`recover_panicked_worker`): developer-diagnostic text passed through
/// as data for the surfaces to render via the `worker-panicked` catalog
/// entry (spec 5.2/8.4). `None` for every job that did not panic.
pub panic: Option<String>,
```

on `JobOutcome` (`crates/muxsmith-core/src/executor/job.rs`), plainly
serialized (no `skip_serializing_if`: the field is always on the wire,
`null` for normal jobs, so consumers see one stable shape).
`recover_panicked_worker` sets `panic: Some(message)` (the existing
downcast: `&str`, then `String`, else the literal
`"<non-string panic payload>"`), keeps `errors:
vec![format!("{}: job {index}", DiagCode::WorkerPanicked.key())]` exactly
as today, and **deletes the `eprintln!` at `queue.rs:396`** - after which
core contains zero stdio calls (re-verified 2026-07-28: `grep -rn
"eprintln!\|println!\|print!(" crates/muxsmith-core/src` returns the one
call plus a comment mention in `lib.rs:23`; control: the same grep over
`crates/muxsmith-cli/src` returns hits in six files - `main.rs` and
`commands/{mod,validate,identify,dry_run,run}.rs`, measured 2026-07-28).
`recover_panicked_worker` is the only constructor that sets `Some`; every
other existing `JobOutcome` constructor the compiler flags sets
`panic: None` (the two new test fixtures this design itself pins - D99's
unit test and D104 item 4's e2e event - are new code, not flagged sites).

**Wire-contract memo (recorded here at decision time):** the serialized
`JobOutcome` gains `panic` in: `--json` `jobs[]` entries (`run_document`),
the `muxsmith://run-finished` payload, the persisted `job-<index>.json`
and `summary.json` (D26). Mirrors updated in the same change:
`src/ipc.ts` `JobOutcome` gains `panic: string | null` (required, so
`vue-tsc`/the e2e type-check forces every fixture literal - the
`JobOutcome`/`RunJobEntry` object literals in `e2e/smoke.spec.ts`'s
live-run scenario - to add `panic: null`);
`JobLogRecord` in `ipc.ts` and the Rust `JobRecord`
(`executor/joblog.rs:160-172`) gain the same field
(`panic: Option<&'a str>` via `outcome.panic.as_deref()`), so the payload
is persisted for triage - the record replaces the deleted stderr line as
the durable home of the panic detail.

**Rejected alternatives (fork 8's other shapes):** (a) a discriminated
entry in the untyped `Vec<String>` (extend the token to carry the payload):
keeps the mixed vector as the only carrier, makes rendering depend on
string parsing of a payload that may itself contain colons/newlines, and
silently changes the token format a test already pins by prefix
(`queue.rs` inline test, re-verified: `.any(|e|
e.starts_with(DiagCode::WorkerPanicked.key()))`); (b) converting `errors`
to a typed `Vec<JobError>` enum: the honest long-term shape, but it breaks
every wire consumer of `errors` (json, joblog, ipc.ts, e2e fixtures) for a
gain this plan does not need - the panic is the only entry that needs
discrimination, and one typed field discriminates it.

**Fork 9 - `delete_partial_failed`: unchanged, because** it is a different
class: a deliberate pass-through of a third-party OS error, recorded as
such twice (`exec-19-delete-partial-errsurface`, Tier 2; `job.rs:196-203`
doc), already visible verbatim on the surfaces that carry `errors` (`--json`,
persisted records), and never ruled into the worker-panic item (`exec-37`
does not mention it; the ruling's scope sentence names the panic path
only). Promoting it to a catalog-rendered code would need its own DiagCode,
two locale texts and a fixture row for a condition whose surrounding
failure (the mux failure or cancellation that produced the partial) is
already fully rendered. A section-6 trigger names the observable event that
would promote it, so the decision is recorded, not merely omitted.

**Fork 12 - what changes about `recover_panicked_worker` and the spec's
prose-free rule:** the function's signature and return type (`()`) are
unchanged; what changes is the outcome it writes (gains `panic:
Some(message)`) and the deletion of its stderr line. The code comment that
cited spec 6/7 as the eprintln's licence (`queue.rs:373-378`: "core's one
deliberate prose-free exception (spec 6/7): logged here for triage, never
carried past this function") is rewritten: the payload is now carried as
data and rendered through the catalog at presentation time, which is the
spec's normal path, not an exception - core still authors no user-facing
prose (`core-37-prose-free-core`). The spec's accepted-exception list in
8.4 gains the panic payload explicitly (amendment S-2), because the payload
is not third-party text in the strict sense (an `.expect()` string is
authored in this repo): it is developer-diagnostic content passed through
as a `$detail` param, untranslatable by nature, same category as the
regex/serde/I-O pass-throughs the list already names.

## D99: CLI human rendering of a panicked job: a `run-job-panicked` terminal line plus the `worker-panicked` catalog message with a new `$detail` param

**Decision (fork 10).**

`MilestoneState::render`'s `Finished` arm returns two lines instead of one
when the outcome carries a panic. `render_finished`'s `JobState::Failed`
arm branches on `outcome.panic` (typed, no string sniffing):

- `panic: None`: unchanged - `run-job-failed` with `code` = `exit_code` or
  `"n/a"` (the literal at `crates/muxsmith-cli/src/commands/run.rs:487`,
  re-verified).
- `panic: Some(detail)`: line 1 is a new `run-job-panicked` key (the
  milestone-line variant without the now-misleading `(exit n/a)` tail);
  line 2 renders the existing `worker-panicked` diagnostics-catalog entry -
  the entry spec 5.2 has promised presentation-time rendering through all
  along - which **gains a `$detail` param** carrying the payload.

The `worker-panicked` message is **not reusable as is** (the fork's
explicit question): its current text defers to "the application log", a
thing that stops existing when D98 deletes the stderr line, and it carries
no payload. Params change, so both locales' final text, verbatim:

`locales/en/diagnostics.ftl` (replaces the existing `worker-panicked` line):

```ftl
worker-panicked = A worker thread panicked while running this job: { $detail }. This is a Muxsmith bug, not an mkvmerge failure; the run's persisted job log carries the full record.
```

`locales/de/diagnostics.ftl` (replaces the existing `worker-panicked` line):

```ftl
worker-panicked = Ein Worker-Thread ist beim Ausführen dieses Jobs abgestürzt: { $detail }. Dies ist ein Fehler in Muxsmith, kein mkvmerge-Fehler; das persistierte Job-Protokoll dieses Laufs enthält den vollständigen Datensatz.
```

`locales/en/cli.ftl` (new key, next to the other `run-job-*` lines):

```ftl
run-job-panicked = [{ $index }/{ $total }] { $output } ... failed (worker panicked)
```

`locales/de/cli.ftl` (new key, same position):

```ftl
run-job-panicked = [{ $index }/{ $total }] { $output } ... fehlgeschlagen (Worker-Thread abgestürzt)
```

Catalog obligations, complete: `run-job-panicked` joins
`ALLOWLISTED_CLI_KEYS` in `crates/muxsmith-cli/tests/catalog_completeness.rs`
with fixture args `[("index", "1"), ("total", "3"), ("output",
"/out/movie.mkv")]` in `allowlisted_cli_key_args`; the `DiagCode::WorkerPanicked`
fixture row changes from `vec![]` to `vec![("detail", "queue worker thread
panicked")]` (the placeholder-leak guard then proves template and params
agree). The CLI test helper `outcome()` in `commands/run.rs`'s test module
gains `panic: None`, and a new unit test mirrors
`finished_failed_renders_exit_code` with `panic: Some(..)`, asserting both
lines render and neither contains `"n/a"`.

`$detail` is always present when the message renders: `recover_panicked_worker`
always synthesizes a payload string (fallback `"<non-string panic
payload>"`), and D100's GUI render site is gated on the field being
non-null, so no surface can render the message without the param.

**Rejected alternatives:** (a) reusing `run-job-failed` with the rendered
`worker-panicked` text stuffed into `$code`: params are data, not nested
prose (house catalog pattern throughout `diagnostics.ftl`); (b) keeping the
`(exit n/a)` line and appending the message: leaves the `n/a` the ruling
explicitly replaces ("renders the catalog message in place of today's
`n/a`", ledger `exec-37`).

## D100: The GUI job row renders the `worker-panicked` message in its failure state, keyed on the typed field; no `JobRowData` change

**Decision (fork 11).**

- **The field:** `JobOutcome.panic` (D98), reaching the row through both
  existing paths with zero model change - the live `finished` job-event
  (`JobsView.onJobEvent` stores the whole outcome into
  `state: { kind: "finished", outcome }`) and the `run-finished`
  reconciliation (`RunJobEntry extends JobOutcome`). `JobRowData` is
  untouched (section 0, note 5).
- **The render site:** `src/components/JobRow.vue`, the state cell (the
  `<td>` that renders `stateKey` and the warning-count span). A computed
  `panicDetail` returns `state.kind === "finished" ? state.outcome.panic :
  null`; the cell gains:

```html
<span
  v-if="panicDetail !== null"
  data-testid="job-panic"
>{{ $t("worker-panicked", { detail: panicDetail }) }}</span>
```

- **The exact user-visible result:** a failed row whose outcome carries a
  panic shows, next to its localized "Failed"/"Fehlgeschlagen" state chip,
  the full localized `worker-panicked` message of D99 including the payload
  - e.g. `Failed  A worker thread panicked while running this job: index
  out of bounds. This is a Muxsmith bug, not an mkvmerge failure; the run's
  persisted job log carries the full record.` A failed row without a panic
  is pixel-identical to today. Placement/styling of the span within the
  cell is presentation (`latitude-carveout-presentation-tokens`); the
  message key, its param, its gating condition and the testid are fixed
  here and are not implementer choices.

No new Fluent key is needed: `worker-panicked` lives in `diagnostics.ftl`,
which the frontend bundles (`src/i18n/index.ts`'s glob imports
`locales/*/diagnostics.ftl` alongside `gui-*.ftl`; verified), so `$t`
resolves it in both locales and `pnpm check:i18n` sees the key exists. The
`no-raw-text` lint is satisfied (the span's text node is a `$t` call).

Scope boundaries, restated from the ruling so nothing drifts in
implementation: the RunHistory **export** stays raw-output-only
(`record.lines.join("\n")`, which never contains the panic - a panicked
worker writes no output line); the history **table** is also unchanged
(the ruling names the live job row; the persisted `job-<index>.json` gains
the payload via D98, so history-side triage reads the record).

**Rejected mechanics alternative:** a dedicated new GUI key (e.g.
`jobs-row-panicked`) instead of reusing `worker-panicked` - it would
duplicate the very catalog entry spec 5.2 promises the rendering through,
in a second file, with a second pair of locale texts to keep in sync, while
`diagnostics.ftl` is already bundled by the frontend.

## D101: `EmptyRawProperty`, error severity, emitted from the shared `raw:` funnel for all validate arms; matcher and planner unchanged

**Decision (ruling 5's mechanics; closes forks 13 and 14).**

**Fork 13 - the new code, every obligation enumerated:**

- **Variant:** `EmptyRawProperty`, declared in `report/mod.rs`'s
  `diag_codes!` block next to `RawProperty`/`RawOnKnownProperty` (the
  config-time `raw:` group), doc comment: "A `raw:`-prefixed match key with
  an empty bare property name (config-time; 4.4, 9.2): always a typo,
  never expressible intent - `get(\"\")` can match nothing, so the rule
  would silently never match. Error severity, own code (ruled 2026-07-28):
  `UnknownProperty`'s message reads as nonsense with an empty name."
- **Catalog key:** `empty-raw-property` (the macro's serde/key equality
  test covers it automatically).
- **Spec 5.2 severity row:** amendment S-1 below, verbatim.
- **`catalog_completeness` fixture row:** `DiagCode::EmptyRawProperty =>
  vec![]` (the diagnostic carries no params: the config path already
  locates the offender, and an empty-string `property` param would render
  as visible nothing).
- **`locales/en/diagnostics.ftl`** (new line, after `raw-on-known-property`):

```ftl
empty-raw-property = The raw: prefix requires a property name: a bare "raw:" names no property, and the rule could never match any track. Add the property name after the colon (for example raw:dolby_complexity_index).
```

- **`locales/de/diagnostics.ftl`** (new line, same position):

```ftl
empty-raw-property = Das raw:-Präfix erfordert einen Eigenschaftsnamen: ein bloßes "raw:" benennt keine Eigenschaft, und die Regel könnte nie auf eine Spur zutreffen. Ergänze den Eigenschaftsnamen nach dem Doppelpunkt (zum Beispiel raw:dolby_complexity_index).
```

**Fork 14 - which arms, and what happens at match/plan time:**

The check lands in the one funnel both validate arms already share:
`raw_opt_in_diagnostic` (`validate.rs:408-414`) gains an empty-name first
branch:

```rust
fn raw_opt_in_diagnostic(path: &str, bare: &str) -> Diagnostic {
    if bare.is_empty() {
        Diagnostic::error(DiagCode::EmptyRawProperty, path.to_string())
    } else if matches!(bare, "language" | "codec_kind") {
        Diagnostic::warning(DiagCode::RawOnKnownProperty, path.to_string()).with("property", bare)
    } else {
        Diagnostic::info(DiagCode::RawProperty, path.to_string()).with("property", bare)
    }
}
```

That covers **both** arms by construction: the `exact` arm
(`validate.rs:268`, which `continue`s after the push - the `continue` is
kept, since there is nothing further to check on an empty name) and the
`substring`/`regex` arm (`validate.rs:310`, which does not `continue` - the
value-level regex compile check still runs afterwards, correctly: an
uncompilable pattern is a second, independent config error).

**Rejected mechanics alternative:** an empty-name check at each of the two
validate call sites instead of inside the shared funnel - two copies of the
same predicate that can drift independently, the exact per-copy-divergence
defect class this plan removes at pipeline scale.

At **match time** and **planning time**: unchanged, deliberately.
`matcher.rs:95`/`:186` keep returning never-match for the empty name
(`item.get("")` answers `None` in every `Matchable`), and
`planner.rs:796`'s `collect_raw_props` keeps inserting `""` into the
capability-warning set. Rationale: validation is advisory, not
load-blocking, in this architecture - `plan_batch` never consumes config
diagnostics (verified: no `validate::` reference in `planner.rs`), and a
CLI `run` on an erroring profile still muxes what plans (exit code folds to
2). The profile state is now loudly flagged at error severity everywhere
the profile is validated; adding defensive behavior changes at two more
layers for a state that is no longer silent would be code without a
consumer. The boundary is exact emptiness: a `raw:` key whose bare name is
whitespace (representable as a quoted YAML key) yields a non-empty bare
name and stays `RawProperty` info with that whitespace as its `property`
param; only `bare == ""` is the ruled typo class.

**Accepted consequences, named (ruled + derived):** profiles that load
today with bare `raw:` change exit codes - `validate`/`dry-run`/`run` exit
2 instead of 0 (ruled: pre-1.0 is the cheap moment). Derived from the same
severity on the GUI: the Batch view's Run gate counts error-severity
diagnostics (`hasErrors` over `diagnosticCounts`), so such a profile can no
longer start a GUI run and its editor Save gate behaves like any other
error - stated here so the exit-code acceptance is understood to include
the GUI gating, not discovered in review. **That gate consequence ships
with its test in this plan** (owner ruling 2026-07-28, amendment 1: a
feature's tests ship with the feature, never after it; this replaces the
round-1 "no producer, rides the v1.x item" routing, which was a controller
decision the owner overturned). The producer, fully enumerated: a new
scenario in `e2e/smoke.spec.ts`'s batch-view describe - the paired
negative of the existing enabled assertion at `smoke.spec.ts:510`
(`await expect(runButton).toBeEnabled()`) - that mocks `detect_mkvmerge`
-> `MKVMERGE_INFO`, `plugin:dialog|open` -> a profile path, and
`validate_profile` -> a report document with `mkvmerge_found: true`, empty
`files`/`batch_diagnostics`/`suggestions`, and `config_diagnostics`
carrying exactly one diagnostic: `code: "empty-raw-property"`,
`severity: "error"`, `config_path: "tracks[0].match.exact.raw:"`,
`params: {}`, `rendered: "empty-raw-property"` (the ruled condition
itself as the fixture). Flow: pick the profile (selection validates, T7
flow), no dry-run click needed. Assertions: `data-testid="batch-run"` is
disabled, and its `title` attribute equals the localized
`batch-run.tooltip-errors` text ("Fix every error-severity diagnostic
before running.", `locales/en/gui-batch.ftl`) - proving the gate fired for
the errors reason, not for a missing profile or missing mkvmerge (the
document's `mkvmerge_found: true` and the completed pick close those
earlier branches of `runDisabledReason` by construction).

**Boundary, stated so it is not misread in either direction:** new test
SCENARIOS on the existing Playwright + mock-IPC harness are in scope
(amendment-1 ruling A; the mechanism is the one the batch describes
already use, `installTauriMocks` + `resolveWith(<document>)`). New test
INFRASTRUCTURE - Vitest, `tauri::test`/`mock_builder`, a
`src-tauri/tests/` tree - stays out at 1.x (the kickoff's OUT ruling,
untouched). The v1.x "GUI test harness for the run path" ROADMAP item also
stays as it is: it covers `start_run`'s untested composition, which this
scenario does not reach.

Tests pinned by acceptance (section 7): one validate-level assertion per
arm (an `exact` map with key `raw:`, a `substring` map with key `raw:`,
each yielding exactly one `EmptyRawProperty` at error severity at the
expected path) plus the guard's control (the neighbouring non-empty
`raw:x` key still yields `RawProperty` info - the discriminating case that
proves the new branch fires on emptiness, not on `raw:` generally).

## D102: Errors-first, stable, applied to `config_diagnostics` inside the two core document builders via a hoisted `report::severity_sorted`

**Decision (ruling 6's mechanics; closes fork 15).**

**Site:** inside `batch_document` and `config_only_document`
(`report/json.rs:35` and `:78`), on the `config_diagnostics` vector only,
before rendering - not inside `rendered_diags`. The sort itself hoists to
core as the one shared ordering definition:

```rust
/// Diagnostics in error-first order (Severity is Info < Warning < Error,
/// so Reverse puts errors first), stable within a severity: ties keep the
/// caller's collection order (validate's traversal order, then the
/// overlap lints appended by config_diagnostics).
pub fn severity_sorted(diags: &[Diagnostic]) -> Vec<&Diagnostic>
```

in `report/mod.rs`; the CLI's `pub(crate) severity_sorted`
(`commands/mod.rs:21-25`) is deleted and replaced by the re-export
`pub(crate) use muxsmith_core::report::severity_sorted;` in
`commands/mod.rs` - the signature is identical (`&[Diagnostic] ->
Vec<&Diagnostic>`), so every `use crate::commands::severity_sorted` call
site compiles unchanged and exactly one ordering definition exists with
zero wrapper code (`core-derive-dont-restate`). The per-diagnostic JSON mapping factors into
a private `rendered_diag(d, renderer)` used by both `rendered_diags` (kept
for the unsorted arrays and the CLI `validate` path) and the builders'
sorted iteration - no second rendering implementation.

**Stability, defined (the fork's question):** Rust's `sort_by_key` is a
stable sort; "stable within a severity" therefore means diagnostics of
equal severity appear in exactly the collection order of the input vector,
which for `config_diagnostics` is `validate(profile)`'s traversal order
followed by `lint::provable_overlaps` appended (the funnel's documented
order). A new core unit test pins the contract discriminatingly: a fixture
vector `[info A, error B, warning C, error D]` must render as
`[B, D, C, A]` - proving both errors-first and the preserved B-before-D
tie order.

**Scope boundary, surfaced (deliberate deviation from full uniformity):**
`files[].diagnostics` and `batch_diagnostics` stay in collection order. The
ruling names `config_diagnostics`; widening the sort to the per-file arrays
would silently change surfaces nobody ruled on (per-file diagnostics carry
resolution-order meaning in the human dry-run rendering). Recorded here so
the non-uniformity is a decision, not an accident.

**Rejected mechanics alternative:** sorting inside `rendered_diags` itself -
one edit instead of two, but it would silently widen the sort to every
`rendered_diags` consumer (the per-file `diagnostics` arrays,
`batch_diagnostics`, and the CLI `validate` envelope), exactly the
beyond-the-ruling scope creep the boundary paragraph above declines.

**Consumers of the new order, swept:** CLI `validate` is unaffected (it
sorts its own flat `{diagnostics: [...]}` envelope already, now through the
re-exported core `severity_sorted` with identical ordering, and that
envelope never passes through the two builders). CLI human printing is unaffected (its `severity_sorted`
loops predate the builders). CLI `--json` `config_diagnostics` becomes
sorted, closing the `cli-08` parity gap; existing CLI JSON tests assert
membership (`.iter().any(...)` shapes in `dry_run_cli.rs`/`run_cli.rs`),
not position, and the workspace gate catches any positional assertion
deterministically. GUI: `validate_profile`/`load_profile`/
`validate_profile_model`/`dry_run`/`plan_run` documents all become sorted
at once (the ruled point: eight callers, two builders); user-visible GUI
consequence - `EditorView`'s diagnostics list and `DiagnosticsPanel`, plus
`BatchView`'s general-diagnostics list, now display errors first, warnings
then infos, instead of traversal order. e2e fixtures are mock documents
that never pass through core and are unaffected. `BatchView`'s
index-0 read: D103.

## D103: BatchView fetches the parse diagnostic by code, not by index

**Decision (fork 16).**

`src/views/BatchView.vue:225` changes from positional fetch to code-keyed
fetch inside the existing `!doc.profile` branch:

```ts
const parseDiagnostic = doc.config_diagnostics.find((d) => d.code === "parse-error");
```

**Which code identifies a parse failure, and why that is exhaustive for
what the index-0 read covered:** `profile: null` in the `load_profile`
envelope occurs on exactly one path, `load_profile_body`'s
`Err(d)` arm (`src-tauri/src/lib.rs:316-320`), and `load::from_file` emits
exactly one `DiagCode` across both of its failure modes - `ParseError` from
the I/O arm (`load.rs:62`, "Both an I/O failure and a parse failure surface
as a `ParseError` diagnostic", its own doc) and from the deserialize arm
(`load.rs:71`). Wire form `"parse-error"` (kebab-case serde encoding,
pinned by core's `all_keys_match_serde_encoding` test). So on every
document the branch can receive, the vector is the singleton
`[parse-error]`, `find` returns exactly what `[0]` returned, and the
existing else-branch (the documented-contract-violation `console.error`)
now additionally catches a future envelope that pairs `profile: null` with
diagnostics led by anything else - strictly more detection than today, no
lost case. Section 0 note 2 records the corrected premise: this is
hardening (the ruled form), not a behavior dependency being migrated - the
singleton could never be reordered by D102.

**Rejected mechanics alternatives:** a severity-keyed fetch (`find(d =>
d.severity === "error")`) - severity does not identify a parse failure, so
any future error-severity diagnostic in that envelope would masquerade as
one - and a `[0]`-fallback behind the code-keyed find, which would
reintroduce the positional read the ruling removes.

**Coverage ships in this plan** (owner ruling 2026-07-28, amendment 1;
the scenarios-in / infrastructure-out boundary stated in D101 applies here
identically and is not restated). The branch this entry edits had no e2e
producer - no scenario resolved `load_profile` with a parse-failure
document; the only `profile: null` fixture, in `e2e/help-mode.spec.ts`,
carries empty `config_diagnostics` and exercises the contract-violation
`console.error` branch, not the fetch. The producer, fully enumerated: a
new scenario in `e2e/smoke.spec.ts`'s batch-view apply describe, replaying
the existing apply flow (pick profile -> dry-run resolving the suggestion
report -> click the suggestion's apply button, exactly the
`smoke.spec.ts:406` scenario's scaffold) with one substitution:
`load_profile` resolves a parse-failure document - `profile: null`,
`config_diagnostics` carrying exactly one diagnostic mirroring core's
`ParseError` emitter (`code: "parse-error"`, `severity: "error"`,
`config_path: ""`, `params: { detail: "unknown field", at: "" }`,
`rendered: "parse-error"`), empty `files`/`batch_diagnostics`/
`suggestions`. Assertions: (a) the view's alert line (the `role="alert"`
paragraph rendering `$t(ipcErrorCode, ipcErrorParams)`) contains "The
profile could not be parsed" (the en text before the `$detail` placeable,
so Fluent's directional-isolate marks around substitutions cannot break
the match); (b) the recorded invoke log contains no `apply_suggestion` and
no `save_profile` call (the branch returns before either - real
invocation evidence, the house pattern `installTauriMocks` exists for).
This is the discriminating test for this entry's change: it goes red if
the code-keyed `find` misses `parse-error`. The change's correctness
additionally rests on the singleton-envelope evidence above.

## D104: The D23 item ships as three mount-harness tests plus a GUI-panic render test, a widened mount glob with a reactive-props hook, and a Tier-1 ledger entry recording the correction's form

**Decision (ruling 7's mechanics; closes fork 17).**

**No code fix**: the ruled position, resting on the round-2 adjudication
("The implementer was right to deviate, and the deviation is correct" ...
"the implemented form is strictly better than my literal wording" -
`docs/process-journal/artifacts/plan-5-sdd/verdicts/whole-branch-review-verdict-round-2.md`,
the deviation-judgment section at `:55`, quote re-verified against the
primary artifact, not taken from the recon) and on the re-verified
event-ordering premise (`start_run`'s doc and `finish_without_queue`'s doc
both state `muxsmith://run-finished` fires before the command's `Result`
reaches the caller; both read at master).

**Harness change, complete enumeration** (with section 0 note 3's surfaced
deviation):

- `e2e/mount-entry.ts`: the glob gains one entry and `resolvePath` one
  branch -

```ts
const modules = import.meta.glob<{ default: Component }>(
  ["../src/editor/widgets/*.vue", "../src/views/EditorView.vue", "../src/views/JobsView.vue"],
  { eager: true },
);

function resolvePath(component: string): string {
  if (component === "EditorView") {
    return "../src/views/EditorView.vue";
  }
  if (component === "JobsView") {
    return "../src/views/JobsView.vue";
  }
  return `../src/editor/widgets/${component}.vue`;
}
```

- `e2e/mount-entry.ts`: `spec.props` moves into a `ref`; the render
  closure spreads from the ref; a new global
  `window.__muxsmithSetProps__(partial: Record<string, unknown>)` merges
  into it (typed in `e2e/global.d.ts` next to `__muxsmithMount__`). This is
  the minimal affordance that makes a *second* `pendingRun` deliverable to
  a mounted JobsView; existing mount specs never call it and are
  unaffected.
- `e2e/mount.ts` is **not** changed: its deliberate no-IPC-mock stance
  stays for the editor-widget specs. The new spec composes the existing
  pieces itself: `page.setContent` -> `page.addScriptTag({ path:
  e2e/.generated/tauri-mock-harness.js })` -> `page.evaluate(installMockIPC,
  scenario)` (`installMockIPC` is already exported from `e2e/mocks.ts` for
  reuse beyond `installTauriMocks` - its doc names the layered
  second-scenario pattern - and is self-contained by its own documented
  constraint, so running it via `evaluate` on the prepared page is the
  same in-page execution `addInitScript` would perform on navigation) ->
  `page.addScriptTag({ path: mount-harness.js })` -> mount. For the soft-outcome case the spec
  installs its own page-side handler via `window.__muxsmithE2E__.mockIPC`
  that emits `muxsmith://run-finished` through
  `window.__muxsmithE2E__.emit` *before* resolving `start_run` - the same
  in-page `@tauri-apps/api/mocks` event plumbing the smoke already relies
  on (`shouldMockEvents`), reproducing the Rust command's documented
  emit-before-resolve ordering.

**Which of round-2's five traced orderings the new spec
(`e2e/jobsview-reset.spec.ts`) asserts** - three of five, plus the D100
render:

1. **Fresh soft outcome** (the ordering the round-2 verdict proved the
   literal "reset after resolve" reading breaks): dispatch `pendingRun`
   with a handler that emits a zero-jobs `run-finished` (empty `jobs`,
   zeroed `summary`, `joblog_status: "unavailable"`) before resolving
   `StartedRun { total_jobs: 0 }`; assert the finished summary is displayed
   after the promise resolves (not clobbered) and cancel-batch is disabled
   (`runActive` false).
2. **Fresh rejection**: `start_run` rejects (`rejectWith("run-already-active")`,
   a real backend code); assert the error alert renders and cancel-batch is
   disabled again (the `startingFresh` guard flipped `runActive` back).
3. **Double-dispatch against an active run** (the divergent branch,
   unreachable from the UI, the reason this item exists): first dispatch
   resolves `StartedRun { total_jobs: 2 }`, a `started` job-event fills row
   0; then `__muxsmithSetProps__({ pendingRun: R2 })` with `start_run`
   scripted to reject `run-already-active`; assert the existing row is NOT
   wiped, cancel-batch stays enabled (`runActive` stayed true), and the
   error alert shows on top.
4. **(D100 acceptance rider)** a `finished` job-event whose outcome carries
   `state: "failed"`, `panic: "boom"` renders the `job-panic` testid with
   the `worker-panicked` text in that row.

Orderings 1 (fresh real run) and 5 (interleaved rapid dispatches) are
deliberately not duplicated here: 1 is already exercised end-to-end by the
smoke's `jobs view: live run` describe block (`e2e/smoke.spec.ts:477`,
`start_run: [resolveWith(startedRun)]` with Playwright-driven events), and
5 is a same-continuation-segment scheduling property whose observable core
(the second dispatch sees `runActive === true`) is exactly what test 3
asserts; a timing-race harness for it would be nondeterministic decoration.

`runActive` is deliberately not passed as a prop in these mounts: absent
prop means `defineModel` falls back to local-ref semantics, which is the
view's real standalone behavior and keeps the internal transitions
assertable through the cancel-batch button's disabled state.

**The ledger entry** (Tier 1, written at implementation time with the
design-fixed content): id `gui-d23-reset-gating-form`, kind `pattern`,
domain `gui`, statement recording that JobsView's reset-before-invoke gated
on `runActive` (`startingFresh`) is the adjudicated, correct form of the
D23 fix - the plan-5 round-2 verdict ruled the literal "reset after resolve
Ok" deterministically broken on soft outcomes because `run-finished` fires
inside the command before its promise resolves - with the steelman carrying
that literal reading and why it fails. This closes the recon 8.5 gap ("No
ledger entry records the deviation as a decision; `gui-11` records the
defect and 'Corrected', not the form").

## D105: The D49 G1/G2 removal experiment: exact mutation, exact invocation, decision rule, recording

**Decision (ruling 8's mechanics; closes fork 18).**

Protocol, executed in this plan (the ROADMAP trigger FIRED 2026-07-28 and
is consumed by this task):

1. **Green control first** (`proc-check-green-state-reachable`):
   `cargo test -p muxsmith-core --test suggestions` passes on the unmutated
   tree; record the pass.
2. **The exact mutation:** in `crates/muxsmith-core/src/planner.rs`,
   `delta_for`'s `AddExact` arm (re-verified at `planner.rs:1820-1827`)
   changes `map.insert(property.clone(), value.clone());` to
   `map.insert(property.clone(), Scalar::Str(scalar_display(value)));`
   (`scalar_display` exists at `planner.rs:856`; verified). Nothing else is
   touched.
3. **The exact suite invocation:** `cargo test -p muxsmith-core --test
   suggestions` again. The three guards, by current name and location (all
   `crates/muxsmith-core/tests/suggestions.rs`, re-verified): G1 =
   `apply_splices_the_simulated_scalar_for_a_bool_property` (`:1037`), G2 =
   `apply_splices_the_simulated_scalar_for_an_int_property` (`:1074`), G3 =
   `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level`
   (`:1113`).
4. **The decision rule, verbatim from the registered trigger:** G1+G2+G3
   all fail -> they are load-bearing and **stay for good**; only G3 fails
   -> G1/G2 become **removal candidates as localizers**. Any other outcome
   (e.g. G3 passes under the mutation) is an anomaly: the experiment's own
   premise failed; no removal in any direction, the anomaly is recorded and
   routed to the controller as NEEDS_CONTEXT.
5. **Restore:** revert the mutation, run the suite green again (the restore
   is itself fire-verified by the step-3 reds).
6. **Where the result is recorded:** a new Tier-1 ledger entry, id
   `core-d49-g1g2-experiment`, kind `pattern`, domain `core`, whose
   statement is one of two design-fixed texts selected by the measurement
   (no implementer wording): *all-fail branch* - "The D49 G1/G2 removal
   experiment (mutate delta_for's AddExact arm to re-stringify, run the
   suggestions suite) measured G1, G2 and G3 all red: G1/G2 are
   load-bearing, not compiler-tautologies, and stay for good; the plan-6
   vacuity analysis is closed as refuted by measurement."; *only-G3
   branch* - "The D49 G1/G2 removal experiment measured only G3 red: G1/G2
   did not detect the type-degradation and are recorded as measured removal
   candidates (localizers) per the registered trigger; they stay in the
   tree until the owner rules on removal at a plan close - the experiment
   measures, it does not argue." In both branches the guards remain in the
   tree at this plan's end: the registered trigger's own wording is
   "candidates", and `proc-proposed-safeguard-stays` licenses removal only
   from a measurement plus a recorded decision, which the candidates branch
   routes to the owner rather than resolving at the keyboard.

The experiment measures; nothing in this plan argues the guards out. The
consumed ROADMAP trigger line is updated at the plan close (controller
action, mirrored per section 6).

---

## 2. What the hoists remove and keep, recounted

Counts recomputed from the enumerations above, not imported: the pipeline
hoist collapses the four `load -> plan_batch` copies measured by the recon
at 260 total / 199 non-comment lines (322/246 including the specs gate) -
figures re-anchored by the span re-verification in the grounding section -
into one seam plus four presentation mappings; the `run_batch` hoist
removes the CLI's 32-line inline block in favor of the 23-line core
function both surfaces call; the runs-root deletion removes one 12-line
function and its doc. The four silently-discarded executor failures the
recon's 4.3 enumerates (`job.rs:113` create_dir_all, `joblog.rs:131`
remove_dir_all, `spawn.rs:123` kill, `spawn.rs:106-111` wait) **stay
discarded**: that is the recorded steelman of `exec-36`'s ruled no-facade
position, restated here so no implementer "improves" them in passing.

---

## 3. Spec amendments (exact replacement text)

The spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is
amended by the implementing plan as follows. Each amendment names its
anchor; the contradiction sweep follows.

**S-1 - section 5.2, the `WorkerPanicked` row is replaced and one row is
added.** The row (current text verbatim, spec line 289):

```
| `WorkerPanicked` | n/a (job-error token, not a rendered diagnostic) | a queue worker thread panicked while running a job (a bug in this crate, never an mkvmerge failure); the job is reported `Failed`. Not a batch `Diagnostic`: carried as a `worker-panicked: job N` token in `JobOutcome.errors` (and its `--json` job encoding) instead, rendered through this same catalog entry at presentation time (6) |
```

is replaced by:

```
| `WorkerPanicked` | n/a (job-error condition, not a rendered diagnostic) | a queue worker thread panicked while running a job (a bug in this crate, never an mkvmerge failure); the job is reported `Failed`. Not a batch `Diagnostic`: the stable `worker-panicked: job N` token stays in `JobOutcome.errors` (and its `--json` job encoding) for scripts, and the downcast panic payload travels as the typed `JobOutcome.panic` field, rendered through this same catalog entry (with the payload as `$detail`) on the CLI human output and the GUI job row at presentation time (6); the persisted job log (6) carries both |
```

And directly below the `RawOnKnownProperty` row, a new row is inserted:

```
| `EmptyRawProperty` | error | a `raw:`-prefixed match key with an empty bare property name, in an `exact`, `substring` or `regex` map (config-time; 4.4, 9.2): always a typo, never expressible intent - the rule could never match any track |
```

**S-2 - section 8.4, first bullet, the accepted-exceptions sentence.** The
fragment "third-party error text passed through as a `detail` param (regex,
serde, I/O)" is extended to:

```
third-party error text passed through as a `detail` param (regex, serde, I/O), the worker-panic payload surfaced as the `worker-panicked` message's `$detail` param (developer-diagnostic content, untranslatable by nature; 5.2)
```

**S-3 - section 4.4, the `raw:` opt-in bullet.** After the sentence
"Config time flags the bypass (`RawProperty`, info; `RawOnKnownProperty`,
warning, on `language`/`codec_kind`), and plan time raises
`UnknownPropertySkew` per consumed `raw:` property (9.2).", insert:

```
A `raw:` key with an empty bare name (a bare `raw:`) is a config-time error (`EmptyRawProperty`): it names no property and the rule could never match, which is always a typo.
```

**S-4 - section 7, the module table.** A new row after `planner`:

```
| `pipeline` | the shared planning pipeline (load -> validate funnel -> resolve mkvmerge -> identify -> plan) every surface calls: the injectable planner seam; also derives executable job specs from a planned batch |
```

**S-5 - section 5.4, first sentence.** "regex/template compilation, type
errors, unknown properties (unless `raw:`-opted, 4.4/9.2)," becomes:

```
regex/template compilation, type errors, unknown properties (unless `raw:`-opted, 4.4/9.2; an empty bare `raw:` name is `EmptyRawProperty`),
```

**S-6 - section 9.2, the announcement sentence.** "The opt-in is announced
at config time by `RawProperty` (info), or by `RawOnKnownProperty`
(warning) when the bare name is a model property with special matching
semantics (`language`, `codec_kind`) that `raw:` degrades to byte-literal
equality." gains a trailing sentence:

```
An empty bare name is not an opt-in at all and is rejected at config time (`EmptyRawProperty`, error).
```

**S-7 - section 5.2, the paragraph above the table.** After "`--json`
output carries code and params plus the rendered message in the active
locale, so scripts key on codes, humans read text.", append:

```
In every report document, the `config_diagnostics` array is ordered errors-first (error, warning, info), stable within a severity (ties keep collection order); per-file `diagnostics` and `batch_diagnostics` keep collection order.
```

**S-8 - section 5.5, the identification-cache sentence (amendment 1,
Ruling B).** The sentence (current text verbatim, spec line 311):

```
Identification cache: in-memory per session, keyed on path + mtime + size, shared between dry-run and run. On-disk cache is a future candidate.
```

is replaced by:

```
Identification cache: in-memory, keyed on path + mtime + size, constructed per planning call and dropped with it. One call identifies each unchanged file once (run plans and executes within a single call, so its planning pass and the suggestion engine's re-simulations share one cache); separate calls re-identify, so a GUI dry-run followed by a run spawns `mkvmerge -J` per file in each (a per-session shared cache was ruled out 2026-07-28 as unnecessary). In the CLI, call and process coincide. On-disk cache is a future candidate.
```

**Contradiction sweep** (a spec amendment has contradicted a neighbouring
section in this project before; every `raw`/`worker-panicked`/ordering
mention in the spec was grepped and read): 4.3's "unless the property is
`raw:`-prefixed (4.4), which opts out of the type check" stays true (an
empty name is rejected, not opted out - S-3's sentence sits in 4.4, which
4.3 references); 5.4 and 9.2 are aligned by S-5/S-6; the 5.2 preamble
("Core emits no user-facing prose") stays true under S-1/S-2 - the payload
is data in a typed field, rendered at presentation time; 6's execution
bullets and 7's prose-free module table entry are untouched and consistent;
5.5's cache sentence is replaced by S-8 (amendment 1, Ruling B), and its
neighbouring run bullet ("re-plans immediately before execution
(identification is cheap and cached; a dry run can never be stale)") was
checked against the per-call cache: it stays true - identification is
cached within each call, "cheap" carries the re-plan justification, and
staleness protection never depended on cross-call reuse (the path + mtime +
size key re-identifies a changed file in any scheme);
8.1/8.2 surface lists are untouched. No other spec sentence mentions
`worker-panicked`, `raw:` emptiness, or `config_diagnostics` ordering
(verified by grep over the spec for `raw`, `worker-panicked`,
`config_diagnostics`, `mkvmerge_found`; the last returns nothing, section
D92).

---

## 4. Ledger obligations created (for the implementing plan)

- New Tier-1 entry `gui-d23-reset-gating-form` (D104, content fixed there).
- New Tier-1 entry `core-d49-g1g2-experiment` (D105, both branch texts
  fixed there).
- `core-121-planner-seam-and-hoist`: the seam interface is settled by
  D91-D95; at the plan close the entry's `blocked_on` clears and a decided
  occurrence records this design as the settlement (controller/close-funnel
  action, per the ROADMAP's recorded close action for the five touched
  entries; nothing else in this design writes ledger entries).

---

## 5. What the implementer must not decide

Every fork is closed above; a fork discovered on code contact returns as
NEEDS_CONTEXT with a decision memo (`proc-latitude-clause-boundary`).

- The seam's module (`crates/muxsmith-core/src/pipeline.rs`), the
  `plan_pipeline` signature, the `PipelineOutcome`/`PlannedPipeline` shapes
  and fields, and the four call-site mappings: as written in D91. No
  additional parameter, no builder object, no trait.
- The resolver closures each surface passes (CLI `Mkvmerge::locate`, GUI
  `Mkvmerge::detect(override)`), and `mkvmerge_found`'s unified meaning
  with **no** new document field (D92).
- The seam constructs its own `IdentifyCache` per call: no cache
  parameter, no `LiveIdentifier` change, no `AppState` field, no session
  cache in any form (D93, owner ruling amendment 1).
- `job_specs` lives in `pipeline.rs` with the doc comment as written; the
  empty-specs presentation stays per-surface (D94).
- The `"."` default is applied exactly once, in the seam (D95).
- `run_batch` moves verbatim with today's signature; it does not absorb
  `run_document`, `finalize_joblog`, or the CLI's joblog messages;
  `TeardownGuard` and `fail_fast` stay caller-side; the two named tests
  move to `queue.rs`'s test module (D96).
- The runs-root deletion touches exactly the function and three call sites
  of D97's table; the CLI gate is not touched.
- `JobOutcome.panic: Option<String>`, always serialized; the `errors`
  token format is byte-identical to today; `delete_partial_failed` is not
  touched; the `JobRecord`/`ipc.ts` mirrors gain the field as written
  (D98). No logging facade, no `log`/`tracing` dependency (ruled; also "no
  new runtime dependency" binds).
- The four Fluent texts of D99 and the one of D101, character for
  character, both locales; the `catalog_completeness` fixture rows and the
  `ALLOWLISTED_CLI_KEYS` addition as written. No other user-visible string
  changes anywhere in this plan.
- The `JobRow.vue` render condition, message key, param and
  `data-testid="job-panic"` (D100); styling inside the cell is
  implementer-owned (`latitude-carveout-presentation-tokens`), semantics
  are not.
- `raw_opt_in_diagnostic`'s three-branch form as written; no matcher or
  planner change (D101).
- `report::severity_sorted`'s contract and its exclusive application to
  `config_diagnostics` in the two builders; the CLI helper is deleted in
  favor of the D102 `pub(crate) use` re-export; per-file and batch arrays
  stay unsorted (D102).
- `BatchView`'s `find` predicate keys on `"parse-error"` exactly (D103).
- The two amendment-1 e2e scenarios exactly as enumerated (flow, mocked
  commands, document contents, assertion targets): D101's Run-gate
  scenario and D103's parse-failure apply scenario, both in
  `e2e/smoke.spec.ts`'s existing batch-view describes on the existing
  mock mechanism. No scenario beyond these and the D104 four; new test
  infrastructure stays out (the boundary stated in D101).
- The mount-glob entries, `resolvePath` branches, the `__muxsmithSetProps__`
  hook, the spec-local mock composition, and the four assertions of D104;
  `e2e/mount.ts` and the two OUT items are not touched; no Vitest, no
  `tauri::test`, no `src-tauri/tests/`, no IpcError funnel.
- The D49 experiment's mutation, invocation, decision rule, anomaly
  routing, and the two ledger statements verbatim; no guard is removed in
  this plan (D105).
- No product-boundary change, no release action, no README
  `placeholder(1.0)` resolution, no dependency added or removed, cargo or
  npm.

---

## 6. Triggers created (for the controller to mirror into the ROADMAP)

1. **A JSON consumer asks to distinguish absent-vs-too-old mkvmerge in a
   report document** (an issue, a script author's request) -> add the
   resolver failure reason as the document field D92 records as the
   designated mechanism (a reason on `MkvmergeUnavailable` surfaced as a
   `mkvmerge_error` value), as an additive wire change.
2. **A user-filed report about a stale partial output or an unexplained
   cleanup failure arrives** -> promote `delete_partial_failed` from a raw
   `errors` pass-through to a catalog-rendered condition, using D98-D100's
   typed-field-plus-catalog pattern as the template (D98's recorded
   deferral).
3. **A user asks for collision-policy control in the GUI** -> wire the
   existing `on_collision` seam parameter (D91/D-3) to a Batch-view
   control; the pipeline needs no change.
4. **The D49 experiment lands on the only-G3 branch** -> the owner rules on
   G1/G2 removal at a plan close, against the `core-d49-g1g2-experiment`
   entry (D105); until ruled, the guards stay.

Each trigger's event is an artifact arriving (an issue, a request, a
recorded measurement), not a state someone must notice.

---

## 7. Acceptance observables (per ruled item: what is observably true, and which emitter produces it)

1. **Pipeline hoist + funnel migration.** `grep -rn
   "config_diags.extend(lint::provable_overlaps" crates src-tauri` returns
   zero hits - fire-verified by first running it on the unmodified tree,
   where it must hit exactly the four copies (`dry_run.rs:60`, CLI
   `run.rs:85`, `lib.rs:211`, src-tauri `run.rs:263`; verified 2026-07-28
   that these four are the pattern's complete current hit set, while the
   funnel's own `diags.extend(lint::provable_overlaps(profile))` at
   `validate.rs:195` deliberately does not match it and remains the one
   call site). Emitters of
   behavior preservation: the existing CLI subprocess suites
   (`dry_run_cli.rs`, `run_cli.rs`, `run_live.rs`) and src-tauri inline
   tests pass unchanged except where D102's ordering and D98's field are
   the designed deltas; the workspace gate (BUILDING.md's six parts) is the
   producer.
2. **`run_batch` hoist.** `src-tauri/src/run.rs` no longer defines
   `run_batch` (grep, fire-verified against today's tree where it must
   hit `:782`); both surfaces call
   `muxsmith_core::executor::queue::run_batch`; the two moved tests run
   under `cargo test -p muxsmith-core`.
3. **Runs-root deletion.** `grep -rn "MUXSMITH_RUNS_ROOT" src-tauri`
   returns nothing (fire-verified: the same grep today returns `:825` and
   `:830`); `grep -rn "resolve_runs_root" src-tauri` returns nothing
   (fire-verified against today's four hits); the CLI gate lines
   (`run.rs:330-335`) are byte-identical; `cargo test -p muxsmith-gui`
   passes untouched (its runs-root tests inject explicit paths).
4. **Worker-panic path.** Emitter: the queue's panic-recovery test plus a
   new core assertion that a recovered outcome carries `panic:
   Some(payload)` AND the unchanged prefix token; the CLI unit test of D99
   (two lines, no `"n/a"`); the e2e assertion of D104 item 4 (the
   `job-panic` testid rendering the localized message); the
   `catalog_completeness` placeholder-leak guard rendering
   `worker-panicked` with `detail`. Observable to a user: a panicked job
   prints `[i/N] out ... failed (worker panicked)` plus the full message on
   the CLI, shows the message on its GUI row, and its `job-<index>.json`
   contains `"panic": "<payload>"`. Core stdio: the grep of D98 returns
   zero calls (fire-verified via today's one hit).
5. **Empty bare `raw:`.** Emitters: the two per-arm validate tests plus
   the non-empty control (D101); `muxsmith validate` on a profile with a
   bare `raw:` key exits 2 and renders the en/de text verbatim (subprocess
   test); the catalog fixture row renders without placeholder leaks; and
   the GUI Run-gate consequence's producer is D101's new batch-view
   scenario (amendment 1): `batch-run` disabled with the
   `tooltip-errors` title on an error-severity `empty-raw-property`
   document, green under `pnpm test:e2e`, red if the gate or the ruled
   severity regresses.
6. **Central sort + BatchView.** Emitters: the discriminating order test of
   D102 (`[B, D, C, A]`); CLI `validate --json` and `dry-run --json` now
   agree on ordering for one mixed-severity fixture (subprocess
   assertion); `BatchView.vue` contains no `config_diagnostics[0]` read
   (grep, fire-verified against today's `:225` hit); and the branch D103
   edits gains its producer, D103's new parse-failure apply scenario
   (amendment 1): the alert line surfaces "The profile could not be
   parsed" and neither `apply_suggestion` nor `save_profile` is invoked,
   green under `pnpm test:e2e`, red if the code-keyed `find` misses
   `parse-error`.
7. **D23 item.** Emitters: the three ordering tests plus the panic-render
   test in `e2e/jobsview-reset.spec.ts` (D104), green under `pnpm
   test:e2e`; the `gui-d23-reset-gating-form` ledger entry exists and
   `scripts/ledger-lint.py` passes.
8. **D49 experiment.** Emitters: the recorded green-mutate-restore run
   (step outputs quoted in the task report: which of G1/G2/G3 went red) and
   the `core-d49-g1g2-experiment` entry carrying the measured branch's
   fixed text.

---

## Amendment log

**Round 1 (2026-07-28), against
`.superpowers/sdd/plan-9/design-review-round-1.md`; all nine findings
fixed, none disputed:**

- **I-1:** deleted D91's inline funnel-migration grep (unreachable green:
  its predicted survivor set named `lint.rs`, which the qualified string
  never reaches, and missed the `dry_run.rs:21` doc-comment survivor) and
  the dangling "per section 8's rules" reference; D91 now cites acceptance
  observable 1 as the single stated completion check, per the new ledger
  rule `design-states-a-completion-check-once`.
- **I-2:** D101 now states the GUI Run-gate consequence as a real,
  user-visible behavior with NO test coverage today (no e2e feeds
  BatchView an error-severity config diagnostic or asserts `batch-run`
  disabled), riding the v1.x GUI-test-harness item; acceptance 5 no longer
  claims a producer for it.
- **I-3:** acceptance 6's "apply-suggestion parse-failure e2e path" claim
  replaced by the honest statement that the branch D103 edits has no e2e
  producer (the only `profile: null` fixture carries empty diagnostics);
  D103 gained the same coverage statement. No new e2e was added in either
  fix - new test scenarios are outside the ruled scope.
- **I-4:** D102's "one-line delegate (or a re-export)" latitude resolved to
  the re-export: `pub(crate) use muxsmith_core::report::severity_sorted;`,
  call sites unchanged; the two downstream references (the consumers-swept
  paragraph, section 5's bullet) updated in the same sweep.
- **M-1:** section 0 note 1 inverted: the recon/ROADMAP figure 1119 was
  correct and the first draft's "1120" was a reader-tool count attributed
  to `wc -l` without running it; the note now records the measured
  1119 (`wc -l`, `awk`, trailing newline confirmed) and keeps notes 2-5
  numbering stable.
- **M-2:** D98's stdio-grep control corrected to the measured six files
  for the quoted three-macro pattern (`main.rs`,
  `commands/{mod,validate,identify,dry_run,run}.rs`).
- **M-3:** the muddled `JobOutcome`-constructor enumeration replaced by
  the semantic statement: `recover_panicked_worker` is the only
  constructor setting `Some`; every other constructor the compiler flags
  sets `panic: None`.
- **M-4:** dropped "and german-locale" from the fixture-site claim; the
  `JobOutcome`-shaped literals live in the live-run scenario only.
- **M-5:** one labeled rejected-mechanics alternative added to each of
  D97 (retained trivial wrapper), D100 (dedicated new GUI key), D101
  (per-arm checks instead of the shared funnel), D102 (sorting inside
  `rendered_diags`), D103 (severity-keyed fetch / `[0]` fallback).

**Round 2 (2026-07-28), the round-1 delta review's two non-blocking notes;
nothing else changed:**

- **Note 1:** D98's M-3 replacement sentence gains the "existing"
  qualifier ("every other existing `JobOutcome` constructor the compiler
  flags sets `panic: None`") plus the parenthetical naming the two new,
  deliberately `Some`-setting test fixtures (D99's unit test, D104 item
  4's e2e event), removing the apparent contradiction for a reader who
  drops the compiler-flags qualifier.
- **Note 2:** the general clause "new GUI test scenarios are outside the
  ruled scope" scoped at both named sites (D101's consequence paragraph,
  D103's coverage paragraph) to "new GUI test scenarios beyond the ruled
  D23 tests (D104)", removing the tension with D104's three ruled-IN
  scenarios; acceptance 5's already-correctly-scoped instance untouched.

**Round 3 (2026-07-28), OWNER-RULED AMENDMENT 1, post-approval and before
execution (routing: `.superpowers/sdd/plan-9/amendment-1-brief.md`); two
rulings, nothing else touched:**

- **Ruling A (a feature's tests ship with the feature):** overturns the
  round-1 CONTROLLER routing of I-2/I-3 ("restate honestly, add no e2e
  tests"). D101's "no test coverage today / rides the v1.x item" paragraph
  replaced by a fully enumerated batch-view scenario in
  `e2e/smoke.spec.ts` (error-severity `empty-raw-property` document via
  `validate_profile`; `batch-run` disabled + `tooltip-errors` title; the
  paired negative of the `:510` enabled assertion), plus the explicit
  scenarios-in / infrastructure-out boundary statement. D103's "Coverage,
  stated plainly" paragraph replaced by a fully enumerated parse-failure
  apply scenario (`load_profile` -> `profile: null` + singleton
  `parse-error`; alert contains "The profile could not be parsed"; no
  `apply_suggestion`/`save_profile` invoked). Acceptance observables 5 and
  6 now name these producers; section 5 gains the two-scenarios bullet.
  The kickoff's infrastructure OUT ruling and the v1.x
  "GUI test harness for the run path" item are untouched.
- **Ruling B (the GUI identification session cache is out as
  overengineering):** D93 rewritten - the seam constructs its own
  `IdentifyCache` per call (today's behavior); no cache parameter, no
  `LiveIdentifier` borrow change, no test-site adaptations, no `AppState`
  field; the caller-owned-cache-plus-session-cache design becomes the
  rejected alternative with its honest steelman and the owner-accepted
  cost (a GUI dry-run followed by a run spawns `mkvmerge -J` per file
  twice). Ripple swept: D91's signature block, S6 step, call-site mappings
  1 and 3, interface-changes memo, and rejected alternative (c); section
  5's D93 bullet; spec amendment S-8 added replacing spec 5.5's
  "shared between dry-run and run" cache sentence with the per-call /
  per-process description, and the contradiction sweep updated with the
  checked neighbouring run bullet.
