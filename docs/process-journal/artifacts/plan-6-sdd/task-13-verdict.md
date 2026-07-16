# Verdict (extracted from the reviewer transcript)

I have everything needed. All checks are complete and cross-verified against the governing documents.

---

### Spec Compliance

- ✅ **Files touched = brief's Files list exactly** (`EditorView.vue`, `App.vue`, `ipc.ts`, `smoke.spec.ts`); `--stat` confirms nothing else, no `.ftl` in the diff.
- ✅ **`gui-editor.ftl` untouched, 45/45 keys** (en count = 45; no locale file in diff).
- ✅ **No new Fluent keys anywhere**; every added template label routes through `$t(...)` (heuristic raw-text scan surfaced only comments/code; `no-raw-text` lint passed per gate).
- ✅ **`v-show`, not `v-if`** for the third view; the recorded reason was extended, not replaced.
- ✅ **Four `ipc.ts` signatures match the Rust command shapes** (`src-tauri/src/lib.rs:454/466/481/496`): `load_profile(path)→Value` typed as `LoadProfileDocument extends ReportDocument`; `save_profile(path,profile)→()`; `validate_profile_model(profile)→Value` typed `ReportDocument`; `apply_suggestion(profile,config_path,edit)→Profile`. `applySuggestion` passes `{ profile, configPath, edit }` — the camelCase→snake_case house pattern is confirmed by the existing `get_job_log` call (`{ runId, index }` → `run_id`). Hand-written, only `Profile`/`StructuredEdit` imported as ts-rs types.
- ✅ **No tooltips added**: Open/Save/nav-editor carry no `:title`; `settings-save-tooltip` and `batch-profile-pick-tooltip` correctly *not* pulled in.
- ✅ **Save-note is a standing `<p>`, no comment-detection**; **validate-on-edit** via `watch(model,…)` + `hasErrors`/`saveDisabled`.
- ✅ **`EditorView` stays mountable from `modelValue` alone** (`defineModel<Profile>()` unchanged, no prop added).
- ⚠️ **Key-reuse at new sites** is user-visible surface the brief's Files list did not name — real, disclosed, adjudicated in Q2.

### Adjudications

**Q1 — Review-check (executed): PASS.**
- `git diff 0ba894a..5b230a2 -- e2e/smoke.spec.ts` → **zero deletion lines**; purely additive. Negative-check applied: the same grep *fires* on `EditorView.vue` (`-import { computed }…`), so the empty smoke result is genuine, not a malformed pattern. No spec deleted, ported, guarded, or `.skip`/`.only`/`xit`'d. `e2e/mount.ts` untouched.
- Committed `EditorView.vue`: **no `onMounted` in the file**; `loadProfile` called exactly once inside `pickAndOpen` (Open `@click`); the validate watcher early-returns unless `currentPath` is set, and only Open sets it. A bare mount issues no IPC. Mounts from `modelValue` alone.
- Additive-only diff = no pre-existing assertion text changed; consistent with the 25/25 gate and the "12 mount/section/grid specs still pass" claim (2 new + 23 pre-existing = 25).

**Q2 — Key-reuse judgment call.**

*(a) Was the fork open?* Partly, and the distinction matters. Design section 2 ("No new user-facing string outside the catalogs", `:1768`) plus a catalog table (`:1774`) that carries **zero rows** for the nav-tab / Open / Save / diagnostics-heading surfaces = the plan mandates *zero new keys* for them. The brief mandates the nav entry; `no-raw-text` forces a `$t` label. So reuse is **forced** — a new key is forbidden, a label is required. But the plan **never enumerates which** existing keys → an unenumerated set in a normative position, i.e. `proc-latitude-clause-boundary`'s OMISSION form. No document tells the implementer which key (no latitude-free *content* path existed), but a latitude-free *procedural* path did: NEEDS_CONTEXT stop and route the which-key question. The implementer resolved it at the keyboard.

*(b) Actual rendered strings (en / de), each new site:*
| site | key | en | de | fit |
|---|---|---|---|---|
| nav editor tab | `batch-profile-heading` | "Profile" | "Profil" | debatable — sits beside activity labels "Batch/Stapel" and "Jobs"; "Profile" is the object, not the activity, and the *same* string already labels BatchView's internal section heading (two locations now render it) |
| Open button | `batch-profile-pick` | "Choose profile..." | "Profil auswählen..." | good — exactly the action |
| current-path line | `batch-profile-current` | "Selected profile: {path}" | "Ausgewähltes Profil: {path}" | acceptable — mild "selected" vs "opened/editing" batch-ish tint |
| file-dialog filter | `batch-profile-filter-name` | "Muxsmith profiles" | "Muxsmith-Profile" | perfect, generic |
| Save button | `settings-save` | "Save" | "Speichern" | perfect, generic |
| diagnostics heading | `batch-diagnostics-heading` | "Diagnostics" | "Meldungen" | good, generic |
| save note | `editor-save-note` | (Task 9's own key) | — | legitimate, brief-named |

None renders nonsense or a contradictory meaning; two (nav tab, current-path line) are debatable-but-acceptable.

*(c) Rule:* **acceptable-with-owner-sign-off.** No wrong-key choice needs a fix round now; every string renders coherent meaning. Route to the owner's plan-close rendered-surface pass, flagging the two debatable items. Note the owner may well conclude the nav tab wants a proper "Editor" label — which needs a *new key*, i.e. a plan amendment, reinforcing (d).

*(d) Process ruling:* **real boundary crossing, record it — not (solely) the over-restriction case.** Grant condition 4 explicitly excludes "catalog-key use at a new site" from the zero-outward-effect grant (`docs/process-conventions.yaml:343`), so by its letter this was a NEEDS_CONTEXT stop; the implementer proceeded-and-disclosed instead, and its report mislabels this as "covered by the grant" — it was not. Two things for the ledger: **(i)** record the boundary-proceed (disclosure was good, harm low, but the doctrine's correct move was NEEDS_CONTEXT → governing human, since user-visible); **(ii)** the root cause is upstream — the plan's blanket "no new string" never enumerated these labels → route the **design latitude gap** to the controller as a plan-coverage finding. Harvest calibration for the over-restriction watch: the boundary is *coarse* — it correctly caught 2 debatable reuses (nav tab, current-path) but needlessly caught 3 slam-dunk-generic ones (`settings-save`, `batch-diagnostics-heading`, `batch-profile-filter-name`). That split is the evidence the watch wants; it argues for a *narrower* carve-out (generic-content key, no `$param`, no semantic-domain shift), not a wholesale loosening.

**Q3 — Directory-picker forward-reference.**

*(a)* The design defines `directoryPath` only as a widget *kind* (`:744`, `:856`, `:907`, `:916`) — **a plain path textbox**. Nowhere does it promise a browse/picker dialog; the D45 widget architecture is prop-fed and zero-IPC (a picker is an IPC round trip, contradicting it). `grep -i picker` over the whole design: no hit. `DirectoryPathWidget.vue` built it as `<input type="text">`.

*(b)* Yes — an in-tree directory-pick mechanism exists: `BatchView.vue:160` `openDialog({ multiple: false, directory: true })` via `@tauri-apps/plugin-dialog`. But it is BatchView's own output-dir button, not wired into `DirectoryPathWidget`.

*(c)* **Stale comment only.** `DirectoryPathWidget.vue:6` and `smoke.spec.ts:701` ("...Task 13's job") are dead forward-references written at Task 10, before T13's scope locked. Task 13's brief lists no such file or step → **not a T13 scope shortfall**, and correctly left untouched (outside the Files list). Fix = reword both comments to drop the "Task 13's job" claim (e.g. "picker out of scope for Plan 6; directory field is text-entry only"). Whether the editor's directory fields eventually get a browse affordance is a Plan 7+ product question (same shelf as the deferred tooltips), not a Plan 6 gap. Route the comment cleanup to the controller.

### Strengths
- Genuine TDD evidence (stashed impl, captured the `TS2305` RED, restored, GREEN).
- The `validationGeneration` stale-response guard is correct and load-bearing: `validate_profile_model` runs on a Tauri blocking pool, so out-of-order completion under rapid edits is real, not theoretical.
- The `currentPath` gate on the watcher is the right mechanism to keep the bare mount-harness specs IPC-free without an on-mount fetch — it satisfies the amendment's review-check by construction, not by an injected mock.
- Wrapping the field/grid markup in `<template v-if="model">` cleanly prevents rendering widgets against `undefined` in the pre-Open empty state, and the diff shows the wrapped content is byte-identical (only re-indented).
- Disclosure discipline: both judgment calls surfaced for the reviewer rather than buried.

### Issues

#### Critical (Must Fix)
- None. Code is correct, tested, gate-green; nothing user-visible is outright wrong.

#### Important (Should Fix) — process/controller level, not code changes to this task
1. **Key-reuse boundary crossing (Q2d):** record the boundary-proceed and obtain owner sign-off on the six reused strings at the plan-close rendered-surface pass (esp. nav tab "Profile" and the "Selected profile:" line). The report's "grant-covered" self-classification is incorrect — flag it so the ledger records it as NEEDS_CONTEXT-that-was-skipped, not a grant-covered structural extension.
2. **Design latitude gap (Q2a):** the plan's "no new user-facing string" + nav mandate left the which-key set unenumerated. Route to the controller as a plan-coverage finding; the owner pass may conclude a dedicated nav-editor key is warranted (a plan amendment).

#### Minor (Nice to Have)
1. **Stale comments (Q3):** reword `DirectoryPathWidget.vue:6` and `smoke.spec.ts:701` to drop "Task 13's job." Controller-routed; correctly not touched by this task.
2. **Redundant post-Open validate round-trip** (report concern 3): harmless, one extra local IPC call, identical result. Leave as-is — special-casing it buys nothing at this scale.

### HARVEST
- **Cross-view catalog-key reuse is now an established house pattern** (verified precedents: `browse-button` across SettingsDialog/FirstRun/BatchView/EditorView; `JobsView.vue:240` `<h2>{{ $t("nav-jobs") }}</h2>`; now the editor's six). The pattern is idiomatic in-tree.
- **Over-restriction watch, calibration evidence (the inverse case the house-dimension asks for):** the grant's blanket "no catalog-key use at a new site" is coarse — here it correctly gated 2 debatable reuses and needlessly gated 3 unambiguously generic ones. Evidence for a *narrower* future carve-out (generic-content key, no interpolated `$param`, no semantic-domain shift), not a wholesale loosening. A found proceed-that-should-have-stopped (Q2d) plus a found stop-that-added-little-value, in the same task.
- **Brief-authoring gap:** a blanket "no new user-facing string" in a design does not discharge the enumeration duty for surfaces the plan itself later mandates (the nav entry). The count/enumeration coupling (`proc-count-recompute`) has a sibling here: a blanket *prohibition* over an unenumerated user-visible set is latitude just as an ellipsis is.

### Assessment
**Task quality:** Approved
**Reasoning:** The implementation is correct, faithful to every hard constraint (mount-harness coverage, `modelValue`-only mount, hand-written signatures, zero new keys, `v-show`, no tooltips) and gate-green; the only open items are a process boundary-proceed and a design latitude gap that route upstream to the controller/owner, plus two one-line stale-comment cleanups — none of which is a code defect in this task's diff.
