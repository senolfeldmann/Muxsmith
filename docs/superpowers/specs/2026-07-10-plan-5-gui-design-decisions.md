# Plan 5 design decisions (GUI, run path)

Status: FINAL 2026-07-10 (Şenol reviewed; explicitly confirmed: D22 scope
split, D25 per-job cancel in core, D26 log format/scope, D27 framework
choice Vue 3 after vetoing React, D29 accessibility/test-attribute
addendum). The React->Vue change also amended the v1 spec (decision-log
stack row + section 7 architecture tree).

Plan 5 is the first GUI plan: the Tauri 2 shell and the run path (batch view,
job-queue view, persisted job logs, first-run detection, app settings). These
decisions implement spec sections 6 (persistence half), 7 (shell/frontend
architecture), 8.2 (batch view + job queue), 8.3 (tooltip baseline only),
8.4 (frontend i18n) and 10 (GUI testing). On any spec/memo conflict the spec
wins per repo convention.

Grounding: v1 design spec (authoritative); Plan 4 executor as committed
(JobEvent/JobOutcome/QueueOpts at crates/muxsmith-core/src/executor/); D13
(JobEvent designed for Tauri forwarding), D15 (exit codes, run --json job
shape), D17 (delete-partial); mkvtoolnix-gui source at ~/Downloads/mkvtoolnix
(jobs/job.cpp, jobs/model.cpp, jobs/mux_job.cpp); HANDOFF v1.x backlog items
that become Plan 5 tasks (JobEvent serde golden test; jobs[].index doc note;
delete_partial error surfacing; cancel-check before spawn).

## D22: Plan 5 ships the run path; profile editor, help mode and apply-suggestion move to Plan 6

**Decision.** (Şenol 2026-07-10.) Plan 5 = Tauri shell + batch view +
job-queue view + persisted job logs + first-run mkvmerge detection + app
settings. Plan 6 = profile editor + help-mode sidebar + one-click
apply-suggestion. In Plan 5 the batch view's diagnostics panel shows each
suggestion with its YAML fragment and a copy-to-clipboard button (the same
fragments the CLI prints); it does not modify the profile.

- Rationale: all Tauri integration risk (event streaming, process lifecycle,
  cancellation) retires first, and a usable GUI exists early; the profile
  editor is the largest, most design-heavy chunk and gets its own plan.
  One-click apply means comment-preserving YAML mutation; that machinery
  belongs to the editor (Plan 6), which owns profile mutation anyway.
- NOT deferred: the tooltip/inline-explanation baseline of spec 8.3 applies
  to Plan 5's views, and the i18n mechanism (Fluent catalogs, no hardcoded
  strings) ships from the first component; only the help-mode sidebar
  machinery (help-ids, per-topic markdown, hover/pin) waits for Plan 6.
- Alternatives rejected: full GUI in one plan (25+ tasks, design-heavy editor
  delays first usable GUI); editor first (highest-risk integration lands
  last, GUI not self-sufficient).

## D23: Thin event-forwarding shell; enumerated IPC surface; one run at a time

**Decision.** src-tauri is a workspace member ("commands + job event stream,
no logic", spec 7). `start_run` spawns `run_queue` on a `std::thread`
(the CLI's drain pattern from crates/muxsmith-cli/src/commands/run.rs),
drains the mpsc receiver and re-emits each `JobEvent` as a Tauri window
event; cancellation is the same shared `Arc<AtomicBool>` behind a
`cancel_run` command. IPC commands: `validate_profile`, `dry_run`,
`start_run`, `cancel_run`, `cancel_job(index)`, `identify`,
`detect_mkvmerge`, `get_settings`, `set_settings`, `list_runs`,
`get_job_log`. Exactly one run at a time, enforced in Tauri managed state; a
second `start_run` is rejected with a diagnostic (the UI additionally
disables Run while active).

- Rationale: zero new concurrency machinery; `JobEvent` is `Serialize` and
  was designed for this consumer (D13). Single-run mirrors mkvtoolnix-gui's
  one queue executor (`Model::startNextAutoJob`, model.cpp:337-380).
- Alternatives rejected: sidecar CLI process parsing `--json` (batch JSON
  arrives only at the end; live progress would need the NDJSON stream that
  is deferred to v1.x; violates spec 7); async/tokio bridge (core is
  deliberately sync/mpsc; an adapter runtime is unearned complexity).
- Frontend performs zero semantic validation (spec 7); every profile check
  round-trips through `validate_profile`.

## D24: Full mkvmerge output is captured as a new raw-line event variant; progress ticks are not persisted

**Decision.** `JobProgress` and `JobEvent` gain a raw output-line variant
(additive serde variant, snake_case tag like the existing five). It carries
every line mkvmerge writes that is not a `#GUI#progress` tick (warning/error
lines appear both tag-stripped in their existing variants and verbatim in
the raw stream). The raw stream feeds the job-queue view's live log pane and
the persisted job log. `#GUI#progress` ticks remain transient UI signal and
are not persisted.

- Rationale: spec 6 requires "full command line and output of every job" to
  be persisted, but `JobOutcome` today carries only tag-stripped
  warning/error lines (job.rs:44-56); the live-log pane (spec 8.2) needs the
  same lines, so one capture serves both. Excluding progress ticks matches
  mkvtoolnix-gui, whose persisted job log stores the output text, not each
  progress update.
- Alternative rejected: accumulating full output into `JobOutcome` (bloats
  the in-memory outcome vector for large batches and duplicates what the
  incremental log writer already persisted).

## D25: Per-job cancel lands in core (kill-by-index + queued-skip set)

**Decision.** (Confirmed Şenol 2026-07-10.) Core's queue gains per-job
cancellation: kill a
specific in-flight job via its registered Killer (the registry seam at
queue.rs already registers per-job Killers) and skip a specific queued job
via a skip-set (skipped jobs become `Cancelled`, D17 delete-partial applies
only to ran processes, spawn-failure/skip deletes nothing). Batch cancel
keeps the existing flag semantics.

- Rationale: spec 8.2 lists "cancel per job or batch" for the job-queue
  view; the seam exists, the extension is contained.
- Alternative rejected: batch-cancel only in Plan 5 with per-job deferred -
  a deviation from spec 8.2 that Şenol declined (2026-07-10: "per job
  cancel in core needs to be in core now").

## D26: Job logs are JSON per job plus a batch summary, written by core for both surfaces

**Decision.** (Format and both-surfaces scope: Şenol 2026-07-10.) A new
`executor::joblog` module writes, per run, a directory
`<platform-data-dir>/muxsmith/runs/<run-id>/` containing `summary.json`
(the D15 run-json document) and `job-<index>.json` per job (extended D15 job
shape: argv, output path, raw output lines, tag-stripped warnings/errors,
state, exit code, start timestamp, duration). The queue layer writes
incrementally as events arrive, so CLI `run` and GUI runs both persist
unconditionally. Dry-runs persist nothing. No pruning in v1: the location is
documented; a `prune` facility is a v1.x candidate. `run-id` is
timestamp-derived with a collision suffix.

- Rationale: spec 6 phrases persistence as a job-engine property ("full
  command line and output of every job are persisted to the app data
  directory"), so GUI-only persistence would diverge from the spec; JSON
  matches the structured-report philosophy (core prose-free, structures
  serialize) and the GUI renders history from these files without a second
  format.
- Parity: mkvtoolnix-gui persists one self-contained file per job,
  `jobQueue/<uuid>.mtxcfg` under its config dir, settings plus embedded log
  (job.cpp:296-348 queueLocation/saveJob). Muxsmith matches the
  one-file-per-job model with JSON in the data dir.
- Alternatives rejected: plain-text log + JSON index (two artifacts to keep
  consistent); GUI-only persistence (spec divergence, CLI loses post-mortem
  logs); single NDJSON journal per run (per-job retrieval and retention
  clunkier, and NDJSON events are deferred to v1.x anyway).

## D27: Frontend stack (Vue 3 + TypeScript) and window model

**Decision.** (Framework confirmed Şenol 2026-07-10: React vetoed, Vue 3
chosen from a Vue/Svelte/Solid/Leptos comparison; spec section 7 amended
accordingly.) Vue 3 + TypeScript + Vite under src/, pnpm via corepack.
Composition API with `<script setup lang="ts">`; plain reactive state
(`ref`/`reactive` + provide/inject), no Pinia and no component library at
this scale. Fluent via fluent-vue (builds on @fluent/bundle, so spec 8.4's
catalog architecture is unchanged). The no-hardcoded-strings CI gate uses a
template-level raw-text lint (candidate:
@intlify/eslint-plugin-vue-i18n's `no-raw-text`); the exact rule is
verified at plan time - it must fire on bare template text without
requiring vue-i18n as the runtime library, else a small custom check
replaces it. Single window, mkvtoolnix-gui-style navigation between two
views (Batch, Jobs) plus a first-run screen and a settings dialog; no
wizard. App settings (mkvmerge path override, default parallelism, locale
override, recent profiles, per-profile source/output memory) live in the
platform config directory; per-profile source/output memory is keyed by
profile path in app settings and is never written into the user's YAML.

- Rationale: Vue is in Şenol's stack, SFC templates avoid JSX, and the
  ecosystem comfortably covers Plan 6's editor (drag-reorder grid, forms).
  Scale-appropriate defaults otherwise; every library is a dependency to
  justify later. Spec 8.2 requires source/output pickers "persisted per
  profile" - app-side memory delivers that without mutating user files
  (profile mutation is Plan 6).
- Alternatives rejected 2026-07-10: React (author veto; nothing in the GUI
  is React-specific); Svelte 5 (new framework while shipping, weaker
  raw-text linting); SolidJS (still JSX); Leptos/Rust-WASM (slow edit loop,
  thinnest component ecosystem for Plan 6, lint/a11y gates hand-built).
- Styling is decided at implementation time (frontend-design skill), not
  here; the memo fixes structure, not aesthetics.

## D28: First-run mkvmerge detection order and guidance

**Decision.** `detect_mkvmerge` probes: configured override first, then
PATH, then platform-standard install locations (Windows
`%ProgramFiles%\MkvToolNix\`, macOS `/Applications/MKVToolNix-*.app` bundle
binaries + Homebrew prefixes, Linux distro paths); on failure the GUI shows
a per-OS installation guidance screen with a manual path picker (writes the
settings override). Minimum supported mkvtoolnix version is enforced here
with a clear error (spec 8.2; the concrete minimum is fixed during
implementation).

- Rationale: parked from Plan 2 precisely for the GUI; the CLI's existing
  behavior (PATH + explicit override) is unchanged.
- Exact standard locations are confirmed during implementation against
  mkvtoolnix's own installer/packaging defaults, not assumed.

## D29: GUI testing and CI additions; packaging stays deferred

**Decision.** Tests: a serde golden test pinning the `JobEvent` wire shape
(tag names, variant names, field names - the GUI now consumes the stream,
so the shape is contract); shell command tests against `FakeSpawner`; a
thin Playwright smoke (launch, load fixture profile, dry-run renders a
resolution table; spec 10 keeps GUI tests shallow because logic lives in
core); i18n completeness checks extended to the frontend catalog and eslint
no-literal-string in CI. CI: pnpm install/build/eslint/Playwright appended
to the existing Linux job on branch pushes. Packaging artifacts
(msi/dmg/deb/rpm/AppImage on release tags) remain deferred until go-public:
tags trigger the 3-OS matrix and paid minutes while the repo is private.

**Accessibility and test attributes** (Şenol directive 2026-07-10: every
element properly identifiable for testing AND accessibility; attribute
choice delegated). The convention, in priority order:

1. **Semantic HTML first**: `<button>`, `<nav>`, `<main>`, `<table>`,
   `<progress>`, `<dialog>`, `<label for>`/`<input id>` - correct roles and
   keyboard behavior come free; no `<div onClick>` controls.
2. **Accessible names everywhere**, localized: visible text, `aria-label`,
   or `aria-labelledby`/`aria-describedby` wired via stable element `id`s.
   ARIA strings are user-facing strings - they come from the Fluent
   catalogs like any label (spec 8.4; the no-literal-string lint covers
   them).
3. **Live regions for the run path**: the live log pane is `role="log"`
   (implicit polite live region); per-job and batch progress use
   `role="progressbar"` with `aria-valuenow/max` (or native `<progress>`)
   and an accessible name; run-summary status changes announce via a polite
   `aria-live` region; error dialogs use `<dialog>`/`role="alertdialog"`.
4. **Playwright locator policy**: `getByRole(name)` is the primary locator,
   so tests enforce the accessibility tree as a side effect; `data-testid`
   (Playwright's default `testIdAttribute`) is the stable fallback for
   structural/repeated nodes (e.g. `data-testid="job-row"` plus a
   `data-index` attribute), never CSS-class or text-fragment selectors.
   Tests pin the locale to English so role names match the `en` catalog.

## D30: Parity classification for the job-queue UX (SI-3)

**Decision.** Recorded classification against mkvtoolnix-gui:

- **Match**: job states (D13, mux_job.cpp:154-159); failures do not abort
  the batch (D14, model.cpp:337-380); per-job progress + overall progress +
  warnings surfaced in the queue view; one-file-per-job persisted log
  (D26); single queue executor (D23).
- **Justified divergence**: no user-assembled, reorderable, persistent job
  queue. mkvtoolnix-gui is interactive - the user builds a queue job by job
  and reorders it; Muxsmith derives the whole batch declaratively from
  profile + source dir at run time. A "queue" outside a running batch has
  no Muxsmith meaning; the Jobs view shows the live run plus the history of
  past runs (from D26 files) instead.
- **Justified divergence**: no automatic "remove completed jobs after N
  days" setting in v1 (mkvtoolnix-gui has one); Muxsmith v1 keeps all run
  logs, pruning is v1.x (D26).
- **Genuine gap, closed in Plan 5**: mkvtoolnix-gui can open a finished
  job's log as text for copy/paste support requests; Muxsmith's equivalent
  (export/copy log from the history view) is cheap and included in Plan 5's
  Jobs view scope.

## D31: Window close with an active run - full mkvtoolnix parity (amendment 2026-07-10)

**Decision.** (Şenol 2026-07-10, supersedes D23's bare "cancel_all on
CloseRequested".) Closing the window while a run is active never exits
immediately: the close is prevented, a confirmation dialog asks whether to
abort the running batch (mkvtoolnix-gui wording as reference); on Yes the
shell issues cancel_all and exits only after the runner thread has finished
(kills landed, joblog finish written); on No the window stays open. With no
active run the window closes normally.

- Parity evidence (SI-3): mkvtoolnix-gui never closes immediately with
  running jobs - MainWindow::beforeCloseCheckRunningJobs shows a
  confirmation (gated by m_warnBeforeAbortingJobs, default on), then
  IGNORES the close event, aborts each running job with
  setQuitAfterFinished(true), and the app quits itself after the abort
  completes (main_window.cpp:492-548).
- Rationale: immediate exit races the 50ms cancel poll - an orphaned
  mkvmerge can keep writing after the app is gone, and summary.json may
  never be written (lost history, exactly what D26 exists for).
- The dialog-suppression preference mkvtoolnix offers
  (m_warnBeforeAbortingJobs) is a v1.x settings candidate, not v1 scope.
- Alternatives rejected: abort-then-quit without dialog (accidental close
  kills a batch silently); keep immediate close (deliberate divergence
  with data-loss risk, declined).
