# Verdict (extracted)

### Verdict
**Amendment:** APPROVED

Every load-bearing claim verified against committed `bf46932` and the e2e harness. The reviewed diff is byte-identical to the working-tree change that this review gates (`git diff HEAD` == `amendment-recents.diff`), and it touches only the plan file (`+177/-1`). No critical or important issues; two minor notes below.

### Executability checks

**BatchView recents mechanism + the re-point (behavior-identical).** Confirmed at `bf46932:src/views/BatchView.vue`: `RECENT_PROFILES_CAP = 10` (:91), local `async rememberRecentProfile(path): Promise<void>` (:93) delegating to `updateSettings` (:79), `updateSettings` also used by `persistDir` (:109) so it correctly *stays*, and the single call site `await rememberRecentProfile(path);` in `selectProfile` directly before `await runValidate();` (:157), with the `dir_memory` read (`settings.value.dir_memory[path]`, :154) preceding it. The prescribed re-point (`settings.value = (await rememberRecentProfile(path)) ?? settings.value;`) reproduces the old semantics exactly: old `updateSettings` set `settings.value = next` as its last line on success and left it untouched on a caught throw; the new `?? settings.value` gives `next` on success and the unchanged ref on the `null` failure return. get_settings call count (one) and `dir_memory`-ordering are preserved. Behavior-identity holds; Step 7's `git diff` review-check gates it.

**Settings IPC mechanism.** `src/ipc.ts:249-254` is exactly `getSettings()->invoke<AppSettings>("get_settings")` and `setSettings(settings)->invoke<void>("set_settings",{settings})` as cited; `AppSettings.recent_profiles: string[]` declared (:34), `AppSettings` exported. The module's `...current` spread preserves all sibling fields; the enumerated never-clobber set (`mkvmerge_path`/`default_jobs`/`locale`/`dir_memory`) is the complete set of the other four fields.

**Editor anchors.** `bf46932:src/views/EditorView.vue`: `pickAndOpen` (:163) has the `if (opening.value || saving.value) return;` guard, dialog, then the `opening`/`ipcErrorCode`/`try`/`finally` with `model.value = doc.profile ?? undefined;` (:181) — the split into `openPath` + dialog wrapper is faithful, and the one-funnel wiring (both pick-after-dialog and the recents `@click` call `openPath`) is correct. Pre-Open surface order is `<button editor-open>` -> `<p v-if="currentPath">` -> `<p v-if="ipcErrorCode">` -> `<section aria-labelledby="editor-diagnostics-heading">` (:350-375). Imports to add are genuine additions: `onMounted` is absent from the vue import (`computed, ref, watch`, :96) and `getSettings` absent from the ipc import (:107).

**Distinct id / duplicate-id hazard.** BatchView's `<h4 id="batch-recents-heading">` (:365) is *unconditional* — the `v-if="settings.recent_profiles.length"` sits on the `<ul>` (:369), not the heading. `App.vue` mounts all views with `v-show` (:123/128/133), so that id is always live in the DOM; a second element reusing the id would be a real duplicate-id/ambiguous-`aria-labelledby` violation. The distinct `editor-recents-heading` + reused text key `batch-recents-heading` is byte-for-byte the existing `editor-diagnostics-heading`/`batch-diagnostics-heading` precedent (:371). Correct.

**Key existence / zero new keys.** `batch-recents-heading` exists bilingual: `locales/en/gui-batch.ftl:21`, `locales/de/gui-batch.ftl:16` (both exact matches to the cited lines). No `recent` key in `gui-editor.ftl` (grep empty, control-confirmed). Reuse is a `gui-batch` key, so the Amendment-2 45-key `gui-editor.ftl` budget is untouched — no contradiction. `RECENT_PROFILES_CAP=10` mirror verified against `src-tauri/src/settings.rs:29`.

**e2e prescriptions.** `installTauriMocks(page,{commands})`->`recorded` (`{cmd,args}[]`), `resolveWith`, `plugin:dialog|open`, `set_settings` args `{settings}`, and `recorded.filter(r=>r.cmd===...)` are all the established shapes (`e2e/smoke.spec.ts`, `e2e/mocks.ts`). Distinct fixture values (`RECENT_PATH != OPENED_PATH`), echo asserted against the recorded `set_settings`/`load_profile` calls (not a UI echo), paired present/absent control on one selector — all present. The flagged `get_settings` queue-depth concern is **benign**: `nextResult` uses `q.length > 1 ? q.shift() : q[0]` (`mocks.ts:89`), so a single seeded response repeats for every call — since each prescribed test seeds one `get_settings` value, queue depth cannot change the outcome. Author correctly marked it re-verify-at-dispatch.

**Mount specs untouched.** `e2e/mount.ts` installs *no* Tauri IPC mock ("No Tauri IPC mock is installed here … `EditorView` [is] fed [its] model as a prop"). The new tolerant `onMounted` `getSettings()` therefore rejects in the harness, the `catch` swallows, `recents` stays `[]`, and `v-if="!currentPath && recents.length"` keeps the section unrendered — mount specs are genuinely unaffected. Step 6 runs the full suite and Step 7's review-check pins "EditorView still mounts from modelValue alone," so a regression would surface.

### Coverage
- **Placement:** Task 13c section sits after Task 14's commit, before "Triggers this plan creates" (:1660). Correct.
- **Fix-wave sequencing / post-verdict framing:** both the Amendment 5 block and the `## Post-verdict amendment` paragraph state it runs after the eight-item fix wave in `plan6-e`, serial/same-worktree, judged by the resumed reviewer against its own verdict — not a fresh task review.
- **Architecture ripple:** the architecture line gained the clause that 13c is a delta not a fifth build wave and leaves Task 14 the last wave-dependent task. No contradiction: Task 14's own text says only "Task 13b is the new last wave-3 task" (:1597) — a wave-3 ordering claim, not an absolute-last one.
- **Markers:** `(amended 2026-07-16, recents routing)` on the task header and all seven steps; distinct from the apply-wiring / detail-editor / mount-harness routing labels.
- **Amendment block length:** four sentences (within five).
- **TDD:** Step 1 writes failing assertions, Step 2 runs RED with named failures (`editor-recent-profile` matches nothing; no `set_settings` write carrying `OPENED_PATH` at front), Step 6 GREEN. Cross-view-staleness deferral correctly cites the real Task-14 items (`docs/…:1618-1619`).
- **Reused-key enumeration:** closed set of 7, matching verdict §c's "six reused keys … `batch-recents-heading` becomes a seventh." Banned-glyph scan of added lines clean (positive control fired).

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **Recents-section visibility is scoped pre-Open-only; the owner's ruling didn't say so.** Option-1a said "renders a recents list"; the amendment renders it only in the `!currentPath` empty state, hiding it once a profile is open. The choice is documented, defensible, and effectively *forced* by the owner's one-reused-key scope (an always-visible list would want `batch-recents-empty` too — a second reused key) plus the established `currentPath`-hides-empty-state model, and it usefully removes a "click-a-recent-discards-the-open-model" footgun. It is a user-visible-surface decision the amendment takes at the keyboard while routing lesser ones (tooltip, empty-state string) to NEEDS_CONTEXT; a one-line owner heads-up at dispatch would close the asymmetry, but it does not block the commit.
- **"Directly after the `<p v-if="currentPath">` line" is slightly imprecise.** The `<p v-if="ipcErrorCode" role="alert">` sits between that line and the diagnostics `<section>`, so the prescribed insertion range contains an intervening element. Inconsequential (the recents section only renders when `!currentPath`), but the "directly after" phrasing over-specifies a point the surrounding markup doesn't make unique.

### HARVEST
- **Behavior-identical extraction with a call-site side-effect reconstruction is a review-checkable pattern.** When a helper's side effect (`settings.value = next`, buried in `updateSettings`) is pulled out into a pure/return-based shared module, the equivalence lives entirely in the `?? oldValue` reconstruction at the call site — success path, swallow path, and pre-read ordering each need explicit checking, and a `git diff` gate on the untouched neighbors (`updateSettings`/`persistDir`/template) is the right falsifier. Generalizes the existing echo-mock/distinct-values discipline to refactor-identity claims.
- **A harness's queue-exhaustion policy decides whether "one response per call" is load-bearing.** `q.length > 1 ? q.shift() : q[0]` (consume-then-repeat-last) means a single seeded value is robust to call-count drift, so a plan's "queue N responses" instruction is only a real constraint when the seeded values differ across calls. Worth reading the mock's `nextResult` before trusting or distrusting a queue-depth note.
