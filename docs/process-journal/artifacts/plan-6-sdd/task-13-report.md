# Task 13 report: D45 - the editor view, part c: open/save, the save note, nav, and ipc signatures

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan6-e`, branch `plan6-e`.
Base: `0ba894a` (Task 12 commit). Result: `5b230a2`.

## What was implemented

- **`src/ipc.ts`**: the four D42/D43 command signatures. `loadProfile(path)` returns a new `LoadProfileDocument` (`ReportDocument` extended with `profile: Profile | null`, mirroring the existing `RunDocument extends ReportDocument` pattern already in the file for the exact same `config_only_document`-envelope Rust construction). `saveProfile(path, profile)`, `validateProfileModel(profile)` (returns plain `ReportDocument`, byte-identical envelope per `src-tauri/src/lib.rs`'s own doc), and `applySuggestion(profile, configPath, edit: StructuredEdit)` (its signature only, for Task 14; `StructuredEdit` is the existing ts-rs-generated type from `src/bindings/profile.ts`). Top-of-file doc comment updated to name T13's consumers, matching the file's existing per-task attribution style.
- **`src/views/EditorView.vue`**: owns open/save/validate state internally (`currentPath`, `diagnostics`, `opening`/`saving`, an ipc-error pair) rather than App.vue, per the brief's own "wire ... into EditorView.vue" step. An Open button (`pickAndOpen`) drives `@tauri-apps/plugin-dialog`'s `open()` then `loadProfile`, setting `currentPath`/`diagnostics`/`model.value` together. A `watch(model, ...)` re-validates through `validateProfileModel` on every model change, **gated on `currentPath` being set** so a bare mount-harness `EditorView` (Tasks 10-12's specs, fed `modelValue` directly, never clicking Open) never issues an IPC call. A `validationGeneration` counter discards a stale response (validation runs on a Tauri blocking-task thread pool, so out-of-order completion under rapid edits is real, not hypothetical). Save (`doSave`) is disabled while `hasErrors` (any `severity === "error"` diagnostic), while `saving`/`opening`, or while there is no open model/path - the one sanctioned frontend affordance. The save-surface note (`editor-save-note`, Task 9's key) renders as a standing `<p>`, not a modal. All existing field-section/rule-grid markup now sits inside a new `<template v-if="model">` wrapper (unchanged content, just gated so the empty-editor state before Open doesn't render every widget against `undefined`).
- **`src/App.vue`**: `View` union gains `"editor"`; a third nav button (`data-testid="nav-editor"`) sets `activeView = 'editor'`; `<EditorView v-show="activeView === 'editor'" />` joins Batch/Jobs as a third permanently-mounted sibling in `<main>`, following the file's own recorded `v-show`-over-`v-if` reason (extended in comment to mention EditorView's own state surviving a tab switch).
- **`e2e/smoke.spec.ts`**: two new real-app tests (`page.goto("/")`, `installTauriMocks`) in a new `test.describe("editor view: open/save (Task 13, D45/D41)")` block, appended after the existing Task 12 block. No existing test touched.

## The "no new Fluent keys" finding (read before the rest)

The brief's Files list has no `.ftl` file, and I verified this is deliberate, not an omission: the design doc's own catalog-additions table (`docs/superpowers/specs/2026-07-15-plan-6-design.md` section 2, "No new user-facing string outside the catalogs") enumerates every key the whole plan creates, by owner task, and Task 13 has zero entries there. Both amendment rounds (mount-harness, owner-rulings) touched Task 13's binding points without adding a `.ftl` file or a table row. I treated this as intentional and built the nav entry / Open / Save / diagnostics heading entirely from **existing** keys:

- Nav tab + Open button + current-path line + file-dialog filter name: reuse `batch-profile-heading`/`batch-profile-pick`/`batch-profile-current`/`batch-profile-filter-name` (BatchView's own "choose + show a profile path" affordance; content is generic, not batch-specific).
- Save button: reuse `settings-save` (`SettingsDialog.vue`).
- Diagnostics region heading: reuse `batch-diagnostics-heading`.
- Save note: `editor-save-note` (Task 9's own key, as the brief names it).

Precedent for cross-view key reuse already exists in-tree twice: `browse-button`'s documented reuse across BatchView/FirstRun/SettingsDialog, and `JobsView.vue`'s own `<h2>` reusing `nav-jobs` instead of a bespoke heading key. `check-i18n`'s three checks all pass with this design (see Gate results); no `.ftl` file was touched, confirmed by `git diff 0ba894a --stat` showing exactly the four brief-mandated files changed and nothing else.

This was a genuine judgment call under the standing structural-conformance grant (`proc-latitude-clause-boundary`, `docs/process-conventions.yaml`), not a NEEDS_CONTEXT stop: the nav entry is explicitly mandated by the brief's own Step 3, the reuse is zero-outward-effect (no new symbol/API surface, no new catalog id, nothing a user perceives as "off"), and two in-tree precedents for exactly this kind of reuse already existed. Flagged here for the reviewer's explicit sign-off since the Files list technically didn't name it. Full reasoning trail (including the reuse candidates considered and rejected, e.g. why `batch-profile-pick`'s tooltip text was *not* pulled in) lives in `EditorView.vue`'s and `App.vue`'s own doc comments at the change sites.

**Second, smaller finding, not mine to fix:** `src/editor/widgets/DirectoryPathWidget.vue`'s own doc comment and the Task-10 mount-harness test at `e2e/smoke.spec.ts:701` ("no IPC dialog -- Task 13's job") both forward-reference a directory-picker dialog wiring that is absent from Task 13's actual brief (no such file in Files:, no such step). Treated the same as the per-rule detail editor: a stale forward-reference from before final scope was locked, not touched, surfaced here for the controller to route.

## TDD evidence

1. Wrote the two new tests in `e2e/smoke.spec.ts` against the Task-12 code (`LoadProfileDocument`, `nav-editor`, etc. did not exist yet).
2. Confirmed RED: `git stash push -- src/App.vue src/ipc.ts src/views/EditorView.vue` (kept the new tests), ran `pnpm test:e2e`:
   ```
   e2e/smoke.spec.ts(34,3): error TS2305: Module '"../src/ipc"' has no exported member 'LoadProfileDocument'.
   [ELIFECYCLE] Command failed with exit code 2.
   ```
3. `git stash pop` to restore the implementation.
4. Confirmed GREEN: full `pnpm test:e2e` run, 25/25 passed (see below).

## Review-check bullet (mount-harness coverage survives)

Exact commands the brief's Step 5 review-check names:

```
$ git diff 0ba894a -- e2e/smoke.spec.ts | grep -E "^-" | grep -v "^--- "
(no output)
```
Zero deletions - the diff against the Task-12 commit is purely additive (confirmed by eye against the actual diff, not just the empty grep: the only "delete+re-add"-looking hunk is the import-list edit, which the diff itself shows as two pure `+` insertions into the existing list, not a removal). No mount-harness spec was deleted, ported, or guarded/skipped.

```
$ grep -n "onMounted" src/views/EditorView.vue
(no match, exit 1)
$ grep -n "loadProfile" src/views/EditorView.vue
84:import { loadProfile, saveProfile, validateProfileModel } from "../ipc";
155:    const doc = await loadProfile(picked);
```
`EditorView.vue` has no `onMounted` hook at all; `loadProfile` is called exactly once, inside `pickAndOpen()`, which only runs on the Open button's `@click`. `EditorView` mounts from an injected `modelValue` alone; the `validate_profile_model` watcher is gated on `currentPath` (only Open's own IPC round trip sets it), so a bare mount never issues any IPC call either. All Tasks 10-12 mount-harness specs ran and passed in this task's own `pnpm test:e2e` (see below) with no injected IPC mock.

## Gate results (all foreground, all green)

Rust side (workspace-wide, unaffected by this frontend-only diff, run for full-gate discipline):
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo test --workspace`: all passed (81 lib tests + integration suites, 0 failed).
- `cargo doc --workspace --no-deps`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

Frontend side:
- `pnpm lint`: clean (D27 `no-raw-text` included; every template string routes through `$t`).
- `pnpm build` (`vue-tsc --noEmit && vite build`): clean.
- `pnpm check:i18n`: `check-i18n: ok (35 source files scanned, 231 catalog ids, 17 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s))`. The 17 warnings are all pre-existing dynamic-`$t()` IpcError/diagnostic codes (including `profile-save-io-failed`/`profile-save-failed`, which this task now wires but only via the generic `$t(err.code, err.params)` pattern BatchView already established, so the static scanner still can't see the literal reference - warning-only, matches the file's own documented residual-false-positive category).
- `pnpm test:e2e`: **25/25 passed**, including the 2 new Task 13 tests and all 12 pre-existing Task 10-12 mount-harness/section-composition/rule-grid tests.

## Files changed

```
e2e/smoke.spec.ts        | 155 +++++++++++++++++++++++++
src/App.vue              |  30 ++++-
src/ipc.ts               |  53 ++++++++-
src/views/EditorView.vue | 295 +++++++++++++++++++++++++++++++++++++----------
4 files changed, 465 insertions(+), 68 deletions(-)
```
Exactly the brief's Files list, nothing else (confirmed via `git diff 0ba894a --stat`).

## Self-review against the brief's review-check bullet

- Mount specs untouched and green: yes (diff-verified additive-only, all 12 pass).
- `EditorView` mounts from `modelValue` alone: yes (`defineModel<Profile>()` unchanged; no prop added).
- No on-mount fetch: yes (`onMounted` absent from the file; `loadProfile` reachable only via the Open button; `validate_profile_model` reachable only once `currentPath` is set, which only Open sets).
- `ts-rs` types the model only, command signatures hand-written: yes, `ipc.ts`'s four new functions are hand-written; only `Profile`/`StructuredEdit` (already ts-rs-generated) are imported as types.
- Save-surface note is a standing note, not a modal, no comment detection: yes, a plain `<p>` inside the `v-if="model"` block, always visible once a profile is open, no conditional text about comment presence.
- Validation runs on every edit, Save disabled while errors exist: yes, via the `watch(model, ...)` + `hasErrors`/`saveDisabled` computed chain.
- Apply-suggestion not implemented here: correct, only its `ipc.ts` signature was added, no UI.
- No tooltips added in the editor: confirmed - no `:title`/`aria-label`-as-tooltip on any new editor element (Open/Save buttons carry no `:title`).
- `gui-editor.ftl` untouched, still 45 keys: confirmed (`git diff 0ba894a --stat` shows no locale file touched; `grep -c "^editor-" locales/en/gui-editor.ftl` still 45).
- `v-show` over `v-if` in App.vue preserved and extended to the third view: yes.

## Concerns

1. **The "no new Fluent keys" design decision** (detailed above) is the main judgment call in this task. I'm confident in the reasoning (two independent authoritative sources, two in-tree reuse precedents, zero-outward-effect grant) but it is a real deviation from what a first read of "add a nav entry" would suggest, and the reviewer should explicitly weigh in rather than rubber-stamp it.
2. **`DirectoryPathWidget.vue`'s stale forward-reference** to a Task-13 IPC dialog that isn't in this task's brief - flagged for the controller to route (or to confirm it's dead language to clean up), not fixed here.
3. The `validate_profile_model` watcher fires once redundantly right after every successful Open (the model assignment triggers it, on top of `loadProfile`'s own already-authoritative diagnostics). Harmless (identical results, one extra local IPC round trip) and deliberately not special-cased for simplicity - noted in case a reviewer wants it optimized away.

No other deviations from the brief.
