# cluster-gui — house-knowledge clusters for the `gui` domain

Clustered from the per-occurrence `find-E*.md` records (one record per
(topic, approach) occurrence). Records that are the SAME (topic, approach)
across eras are merged here into one cluster; distinct dates/refs are kept as
the recurrence signal, never collapsed.

Legend: kind = pattern (adopted) / restraint (rejected, steelman = case for the
loser) / non-decision (deferred, blocked_on). occ_kind = decided / reinforced /
violated-corrected / deferred. status = settled / contested / blocked.
promoted = count >= 3 (promoted_at = 3). Era→date anchors: E0 = 2026-07-08
(spec), E6 = 2026-07-10 (Plan 5), E7/E8 = 2026-07-12 (Plan 5.5 / go-public tail).

**Merges (4):** every other cluster is a singleton.

- `gui-15` Window-close teardown = T8 risk flag (deferred) + D31 resolution.
- `gui-27` T21.5 hint wording = veto-pending (deferred) + veto-declined (decided).
- `gui-28` CSP = E6 csp:null deferral + E8 D34 strict-block adoption.
- `gui-34` Vue reactive-props destructure = idiomacy refute + CONVENTIONS restraint.

No (topic, approach) recurs 3+ times: **no cluster is promoted.** The four
merges each reach count 2 (deferral→resolution threads, or refute→codify).

---

## Patterns (adopted)

### gui-01 — Discoverability: help mode with hover-to-explain sidebar
pattern · settled · count 1
Every non-obvious control carries a tooltip and inline explanation, plus an
integrated help mode that swaps a sidebar to each element's long-form
explanation; extra UI surface accepted deliberately.
- 2026-07-08 · decided · spec 2026-07-08 §2 row 11 + §8.3 + commit a671949 — "More UI surface, accepted deliberately: modern tools underinvest here."

### gui-03 — Frontend framework: Vue 3 + TypeScript (D27)
pattern · settled · count 1
Vue 3 + TS, Composition API, plain reactive state, no Pinia, no component
library, fluent-vue for i18n. Scale-appropriate defaults over React.
- 2026-07-10 · decided · memo D27 (React vetoed, spec §7 amended) — "Vue is in Şenol's stack, SFC templates avoid JSX, and the ecosystem comfortably covers Plan 6's editor."

### gui-05 — App settings in config dir, no profile-YAML mutation (D27)
pattern · settled · count 1
App settings (override, jobs, locale, recents MRU-10, dir memory) live in the
platform config dir keyed by profile path; per-profile source/output memory is
never written into user YAML.
- 2026-07-10 · decided · memo D27 / plan T7 — "app-side memory delivers that without mutating user files."

### gui-06 — Recents list must truncate to the MRU cap
pattern · settled · count 1
updateSettings assigned the pre-truncation array to reactive state, growing the
rendered recents past the D27 MRU cap of 10 within a session; corrected.
- 2026-07-10 · violated-corrected · task-10-review-verdict.md (Important); fix commit 638eda2 — "a real, demonstrable divergence from the D27 MRU cap, in exactly the axis flagged for scrutiny."

### gui-07 — Tauri thin event-forwarding shell (D23)
pattern · settled · count 1
src-tauri is commands + job event stream with no logic: start_run spawns
run_queue on a std thread, re-emits each JobEvent, cancels via a shared
Arc<AtomicBool>, reusing the CLI drain pattern and JobEvent's existing Serialize.
- 2026-07-10 · decided · memo D23 — "zero new concurrency machinery; JobEvent is Serialize and was designed for this consumer (D13)."

### gui-09 — Single-run invariant in managed state (D23)
pattern · settled · count 1
Exactly one run at a time, enforced in Tauri managed state; a second start_run is
rejected, mirroring mkvtoolnix-gui's one queue executor. UI additionally disables Run.
- 2026-07-10 · decided · memo D23 — "Single-run mirrors mkvtoolnix-gui's one queue executor (Model::startNextAutoJob)."

### gui-10 — Frontend does zero semantic validation (D23)
pattern · settled · count 1
Every profile check round-trips through core's validate_profile; the frontend
performs no semantic validation.
- 2026-07-10 · decided · memo D23 / spec 7 — "Frontend performs zero semantic validation (spec 7); every profile check round-trips through validate_profile."

### gui-11 — Run-gate while active (double-Run must not wipe the live run)
pattern · settled · count 1
D23's "UI disables Run while active" was left unimplemented; clicking Run during
an active run wiped JobsView state before start_run rejected, leaving the run
invisible and uncancellable. Corrected.
- 2026-07-10 · violated-corrected · whole-branch-review-verdict.md (Important #3); fix commits a32b159 + fix wave — "D23's own sentence is not implemented, and the failure mode is destructive."

### gui-12 — Run path must resolve mkvmerge via the settings override
pattern · settled · count 1
T8 mirrored the CLI verbatim and missed T7's detect(override) substitution, so
start_run resolved mkvmerge via PATH-only locate(): detect + dry-run passed but
every real run on Windows/override soft-failed. Cross-task drift; corrected.
- 2026-07-10 · violated-corrected · whole-branch-review-verdict.md (Critical #1); fix commit d9ad2fb — "the exact cross-task drift a task-scoped review cannot see... the primary Windows run path is broken end-to-end."

### gui-13 — Run planning must not run on the main thread
pattern · settled · count 1
start_run was a sync command shelling mkvmerge -J per file on the main thread,
freezing the window and making the tested cancel-during-planning paths
unreachable from production. Fixed via async + spawn_blocking.
- 2026-07-10 · violated-corrected · whole-branch-review-verdict.md (Important #2); fix commit d9ad2fb — "the carefully tested cancel-during-planning paths are unreachable from the production UI."

### gui-14 — Async command discipline (async + spawn_blocking)
pattern · settled · count 1
Long-running commands (dry_run / identify / detect_mkvmerge) are async +
spawn_blocking per the verified Tauri v2 threading model; non-async commands run
on the main thread.
- 2026-07-10 · decided · task-7-review-verdict.md (judgment call 3) — "Commands without the async keyword run on the main thread... The deviation is real, correctly reasoned, and documented."

### gui-15 — Window close with active run: prevent, confirm-abort, quit-after-finished (D31)
pattern · settled · count 2
Closing during a run never exits immediately: a confirm dialog aborts and quits
only after the runner thread finishes (full mkvtoolnix parity), superseding D23's
bare cancel_all. Merges the T8 data-loss flag that escalated the question with
its D31 resolution.
- 2026-07-10 · deferred · task-8-review-verdict.md (Important #4) — reviewer flagged the never-blocks-close teardown as data-loss risk (orphaned mkvmerge + lost summary.json) for the owner to rule on.
- 2026-07-10 · decided · memo D31 (amendment, Şenol) — mkvtoolnix-gui "IGNORES the close event, aborts each running job with setQuitAfterFinished(true), and the app quits itself after the abort completes."

### gui-18 — Runner-thread robustness: TeardownGuard RAII
pattern · settled · count 1
A queue-worker panic left the slot Running post-D31, so close forever answered
ConfirmAbort (app unclosable) and summary.json was lost; fixed with a
TeardownGuard RAII drop.
- 2026-07-10 · violated-corrected · whole-branch-review-verdict.md (Important #4, promoted); fix commit 8e3243c — "only SIGKILL ends the process, and summary.json is lost."

### gui-19 — Settings-file durability: write-temp-then-rename
pattern · settled · count 1
A torn settings.json (plain fs::write) sent App to FirstRun whose recovery
re-read the same broken file — a closed loop bricking the GUI; fixed with an
atomic write-temp-then-rename.
- 2026-07-10 · violated-corrected · whole-branch-review-verdict.md (Important #5); fix commit 5c504c1 — "a closed loop with no exit short of manually deleting a hidden file."

### gui-20 — Log save-as via the official fs+dialog plugin combo
pattern · settled · count 1
Save-as uses the dialog plugin, which injects the chosen path into the fs scope,
rather than reimplementing the trust chain in a bespoke Rust write command.
- 2026-07-10 · decided · task-11-review-verdict.md (strengths) — "the plugin route gets it from Tauri's own IPC scope layer for free."

### gui-23 — Job-queue: log export/copy from history (D30 parity gap closed)
pattern · settled · count 1
The one genuine mkvtoolnix parity gap — open a finished job's log as text for
support — is closed in the Jobs view with copy + save-as.
- 2026-07-10 · decided · memo D30 (genuine gap, closed); task-11-review-verdict.md — "exceeds the D30 open-as-text parity bar, not just meets it."

### gui-24 — Accessibility + test-attribute convention (D29)
pattern · settled · count 1
Priority-ordered a11y+test convention: semantic HTML first, Fluent-sourced
localized accessible names, live regions (role=log / progressbar), Playwright
getByRole primary with data-testid fallback and en-pinned locale.
- 2026-07-10 · decided · memo D29 (Şenol directive) — "every element properly identifiable for testing AND accessibility."

### gui-25 — German reachable from the UI immediately (T21.5)
pattern · settled · count 1
German added to the settings dropdown and the stale settings-locale-hint
corrected, failing-e2e-first; Şenol ruled fix immediately, not with Plan 6.
- 2026-07-12 · decided · task-21.5-verdict.md / plan T21.5 — "the settings dropdown lists only English and the settings-locale-hint claims only English... Şenol: fix immediately, not with Plan 6."

### gui-27 — T21.5 settings-hint wording: kept unchanged (veto declined)
pattern · settled · count 2
The T21.5 settings-hint texts were parked on Şenol's veto, then approved
unchanged when he declined the veto. Merges the deferral and its resolution.
- 2026-07-12 · deferred · journal 2026-07-12 Plan 5.5 entry (Open threads) — "Şenol veto pending on the T21.5 settings-hint wording (texts in the task report)."
- 2026-07-12 · decided · journal 2026-07-12 session 9 close addendum — "Owner approved the T21.5 settings-hint texts unchanged (veto declined)."

### gui-28 — Production CSP: strict explicit-directive block (D34)
pattern · settled · count 2
tauri.conf.json app.security.csp is a strict object-form block (default-src
'none' plus explicit script/style/img/connect/base-uri/form-action); Tauri IPC
nonce/hash injection left on, no devCsp. Merges the E6 csp:null deferral with the
E8 D34 adoption that resolved it.
- 2026-07-10 · deferred · task-4-review-verdict.md (Minor 3) / whole-branch rec #3 — csp:null scaffold default carried from T4; real CSP deferred to a pre-release gate. "worth a forward pointer before this ships with real IPC surface."
- 2026-07-12 · decided · memo D34 — "The strict default-src none base makes every future surface addition a conscious CSP edit instead of a silent allowance - explicit over magic."

### gui-32 — GUI report document shape: single report::json render path
pattern · settled · count 1
Every GUI report command returns the same report::json document
(config_only / batch / run_document) so the frontend has one rendering path; new
report commands conform.
- 2026-07-12 · reinforced · CONVENTIONS.md Patterns (b38a46f); idiomacy finding run.rs:L249 — origin Plan 5 T2; "report::json documents were already hoisted for this reason, orchestration was left behind."

### gui-33 — Vue props form: defineProps + props.x
pattern · settled · count 1
Prop-taking SFCs declare const props = defineProps<...>() and read props.x
(withDefaults for defaults); all SFCs conform and a lone deviation is the outlier.
- 2026-07-12 · reinforced · CONVENTIONS.md Patterns (b38a46f) — seeded from Plan 5; house form with withDefaults for defaults.

---

## Restraints (rejected — steelman kept)

### gui-02 — No per-file manual overrides in the GUI
restraint · settled · count 1
The GUI offers no per-file manual override; users needing that go to
mkvtoolnix-gui, with an "open in mkvtoolnix-gui" escape hatch left as a v1.x candidate.
steelman: users will sometimes hit one file the rules cannot express and must then leave the tool entirely.
- 2026-07-08 · decided · spec 2026-07-08 §11 — "Per-file manual overrides in the GUI (that is mkvtoolnix-gui's job...)."

### gui-04 — Frontend framework alternatives rejected (D27)
restraint · settled · count 1
Four rejected: React (author veto), Svelte 5, SolidJS, Leptos-WASM.
steelman: React is the most-staffed ecosystem and was in-stack; Leptos would keep a single Rust language end-to-end.
- 2026-07-10 · decided · memo D27 (alternatives rejected 2026-07-10) — "nothing in the GUI is React-specific"; Svelte = new framework while shipping + weaker raw-text linting; Solid still JSX; Leptos slow edit loop + thinnest Plan-6 ecosystem + hand-built lint/a11y.

### gui-08 — Tauri sidecar CLI / async-tokio bridge rejected (D23)
restraint · settled · count 1
Rejected a sidecar CLI process (parsing --json) and an async/tokio adapter runtime.
steelman: a sidecar reuses the CLI untouched; a tokio bridge yields idiomatic async Tauri commands.
- 2026-07-10 · decided · memo D23 (alternatives rejected) — "batch JSON arrives only at the end; live progress would need the NDJSON stream that is deferred"; "core is deliberately sync/mpsc; an adapter runtime is unearned complexity."

### gui-16 — Window close: dialog-less abort / immediate close rejected (D31)
restraint · settled · count 1
Rejected both a dialog-less abort and keeping the immediate close.
steelman: immediate close is the simplest and most responsive behavior with no extra dialog to maintain.
- 2026-07-10 · decided · memo D31 (alternatives rejected) — "accidental close kills a batch silently"; "immediate exit races the 50ms cancel poll - an orphaned mkvmerge can keep writing... summary.json may never be written."

### gui-21 — Job-queue: no user-assembled reorderable persistent queue (D30)
restraint · settled · count 1
Justified divergence from mkvtoolnix-gui: the batch is derived declaratively at
run time, so the Jobs view shows the live run + history, not an editable queue.
steelman: mkvtoolnix-gui's interactive build-and-reorder queue is a familiar, flexible UX users may expect.
- 2026-07-10 · decided · memo D30 (justified divergence) — "a queue outside a running batch has no Muxsmith meaning."

### gui-22 — Job-queue: no auto-prune of completed jobs in v1 (D30)
restraint · settled · count 1
Justified divergence: v1 keeps all run logs; pruning is v1.x.
steelman: mkvtoolnix-gui ships an auto-cleanup setting that bounds disk growth without user effort.
- 2026-07-10 · decided · memo D30 (justified divergence) — "Muxsmith v1 keeps all run logs, pruning is v1.x (D26)."

### gui-29 — Production CSP: docs-idiom 'self' baseline rejected (D34)
restraint · settled · count 1
The common Tauri default-src 'self'; connect-src ipc string baseline was considered and rejected.
steelman: defensible and common in well-maintained Tauri apps; less config to maintain.
- 2026-07-12 · decided · memo D34 rejected-alternatives — "defensible and common in well-maintained Tauri apps, but silently allows bundle-origin loads for every unenumerated directive."

### gui-30 — Production CSP: separate devCsp block rejected (D34)
restraint · settled · count 1
A separate devCsp block was initially proposed then withdrawn the same day.
steelman: would give development its own CSP separate from production.
- 2026-07-12 · decided · memo D34; journal session-8 close — source-verified vs tauri 2.11.5 that with devUrl set neither csp nor devCsp reaches the dev page — "the block would be dead config."

### gui-31 — Production CSP: csp:null status quo rejected (D34)
restraint · settled · count 1
Leaving the scaffold default csp:null (carried since Plan 5 T4) was rejected.
steelman: the shipped status quo; zero configuration.
- 2026-07-12 · decided · memo D34 — "no blast-radius cap if an injection path ever appears (a future v-html, a dependency regression)."

### gui-34 — Vue props form: reactive-props destructure rejected
restraint · settled · count 2
An idiomacy finder proposed switching to Vue 3.5 reactive-props destructure;
verification refuted it (would make the component the lone outlier vs its
siblings), and the refute was materialized as a standing CONVENTIONS restraint.
steelman: terser and 3.5-supported (reactive-props destructure enabled by default; toolchain supports it).
- 2026-07-12 · violated-corrected · idiomacy-review-findings.md Refuted, BatchView.vue:L53 — "fails adversarial ... internal consistency" — the named house-convention refute.
- 2026-07-12 · reinforced · CONVENTIONS.md Restraints (b38a46f) — "The steelman for destructure (terser, 3.5-supported) lost to internal consistency. (Idiomacy review 2026-07-12, refuted finding.)"

---

## Non-decisions (deferred — still open)

### gui-17 — Window close: dialog-suppression preference (D31)
non-decision · blocked · count 1 · blocked_on: external (product / v1.x)
The abort-confirmation-suppression setting (mkvtoolnix m_warnBeforeAbortingJobs)
is parked as a v1.x settings candidate, not v1 scope.
- 2026-07-10 · deferred · memo D31 — "The dialog-suppression preference mkvtoolnix offers is a v1.x settings candidate, not v1 scope."

### gui-26 — In-session locale switching (T21.5)
non-decision · blocked · count 1 · blocked_on: Plan 6 (internal)
No live in-session locale switch (bootstrap-once architecture; a restart notice
was added); a live switch was deferred as a Plan 6 candidate.
- 2026-07-12 · deferred · task-21.5-verdict.md / whole-branch funnel T21.5-m1 — "No live in-session locale switch (pre-existing bootstrap-once architecture); product call for later - Plan 6 candidate."
