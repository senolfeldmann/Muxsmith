## Task 3: New creates a blank profile, and `currentPath` keeps only its path duties (W2)

Read first: this plan's D107 in full and the authoring section's seed measurement; `docs/ROADMAP.md`'s round-3 finding 2 through its rulings and its measured-cost block; `src/views/EditorView.vue` in full; `src/components/RunHistory.vue`'s `saveLog` (the capture-before-the-dialog-gap pattern this task conforms to); `src/components/DiagnosticsPanel.vue`'s doc comment (why the panel carries no empty state); `src/views/BatchView.vue`'s profile/recents block (the empty-state house pattern); `src/recentProfiles.ts`; `src/ipc.ts`'s `saveProfile`/`loadProfile`/`validateProfileModel`; `e2e/smoke.spec.ts`'s two editor describes (the fixtures, the a11y helper and the recents scenario this task's tests sit beside); the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src/views/EditorView.vue` (the seed factory, `sessionActive`, `createBlank`, the four `currentPath` duty replacements, `doSave`'s dialog branch, the template's New button, unsaved line, empty state, recents gate and diagnostics gate, **and the two doc-comment regions this task's own change falsifies**: the Task-13 block's `currentPath` gate explanation and the stale `gui-editor.ftl stays 45` count)
- Modify: `locales/en/gui-editor.ftl` (three new ids under a new section comment)
- Modify: `locales/de/gui-editor.ftl` (the same three)
- Modify: `e2e/smoke.spec.ts` (a new describe with the tests below, **and the stale `catalog budget is 45` comment**)

**Why the two comment regions are in scope rather than fenced off.** Both are references this task's own edit falsifies inside files it already modifies: the validation gate moves off `currentPath`, and the editor catalog grows. Widening the code while leaving its own description wrong is the defect class `comments-locate-by-symbol-never-by-line-number` and `proc-normative-count-recomputed` exist to prevent. **Both are NAMED regions**, so the Files enumeration stays exhaustive and the within-file qualifier still bites everywhere else (`latitude-carveout-zero-content-structural-forks`).

**Interfaces:**
- Consumes: nothing.
- Produces: `createBlank` and `sessionActive`, which Task 4 converts to a computed over its own snapshot ref.

- [ ] **Step 1: re-measure the seed before writing it.** Write the four candidate seeds and run the validator on each, exactly as the authoring section did, and paste the output. **If the measurement disagrees with the authoring result - if the chosen seed produces any error-severity diagnostic - stop and return NEEDS_CONTEXT with both runs pasted.** The seed is chosen by measurement, and a tree that has moved is a finding, not a licence to adjust the fence.

- [ ] **Step 2: `EditorView.vue`'s script.** Six edits, each fixed here.
  - The seed factory, at module level, with a doc comment stating why it is a function (a fresh object per call, matching the immutable-rebuild discipline of every write in this view) and why `extensions` carries a value at all (the validator: an empty list is an error, so a bare seed would greet the user with a disabled Save):

```ts
function blankProfile(): Profile {
  return {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: { rules: [{ match: {} }] },
  };
}
```

  - `const sessionActive = ref(false);` with a doc comment stating its duty (a profile entered the editor through one of its own funnels, `openPath` or `createBlank`) and why it exists rather than reusing `currentPath` (a created profile has no path, and the bare mount-harness case must keep firing no IPC).
  - `saveDisabled` drops `!currentPath.value`.
  - The `watch(model)` gate becomes `if (!sessionActive.value || !value) { return; }`, and the comment above it names `sessionActive` instead of `currentPath`, keeping its other two facts (why a shallow watch suffices, and what `validationGeneration` is for).
  - `openPath` sets `sessionActive.value = true;` beside `currentPath.value = path;`.
  - `createBlank`:

```ts
function createBlank(): void {
  if (opening.value || saving.value) {
    return;
  }
  ipcErrorCode.value = null;
  currentPath.value = null;
  diagnostics.value = [];
  sessionActive.value = true;
  model.value = blankProfile();
  selectedIndex.value = 0;
}
```

    The order is load-bearing and is commented as such: `sessionActive` is set before the model, so the watcher that fires on the assignment validates the seed instead of returning early; `diagnostics` is cleared first so a previous profile's findings never render against the new model; and index 0 is selected so the detail panel opens on the one field the warning names, mirroring Add's own behaviour (D67).

- [ ] **Step 3: `doSave`, with the dialog branch.** Replace the function body with exactly this shape, whose capture-before-the-await discipline mirrors `RunHistory.saveLog` and is commented as doing so:

```ts
async function doSave(): Promise<void> {
  if (saveDisabled.value || !model.value) {
    return;
  }
  // Captured before the dialog gap: the native save dialog can stay open
  // indefinitely, and the model may change underneath it -- what gets
  // written must be the profile that was in the editor when Save was
  // clicked (same rule as the job-log export in `RunHistory`).
  const profile = model.value;
  let path = currentPath.value;
  const needsPath = path === null;
  saving.value = true;
  ipcErrorCode.value = null;
  try {
    if (needsPath) {
      const picked = await saveDialog({
        defaultPath: "profile.yaml",
        filters: [{ name: fluent.$t("batch-profile-filter-name"), extensions: ["yaml", "yml"] }],
      });
      if (typeof picked !== "string") {
        return;
      }
      path = picked;
    }
    await saveProfile(path, profile);
    currentPath.value = path;
    if (needsPath) {
      // A profile that just acquired a path is exactly what the recents
      // memory is for; an already-pathed save leaves it alone, as before.
      const persisted = await rememberRecentProfile(path);
      if (persisted) {
        recents.value = persisted.recent_profiles;
      }
    }
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    saving.value = false;
  }
}
```

  The import becomes `import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";`. `dialog:allow-save` is already granted, so no capability file changes (authoring section).

- [ ] **Step 4: `EditorView.vue`'s template.** Five edits:
  - A New button immediately BEFORE the existing Open button (parity: New precedes Open in the reference tool's menu), `type="button"`, `data-testid="editor-new"`, `:disabled="opening || saving"`, `@click="createBlank"`, label `{{ $t("editor-action-new") }}`. No `title` attribute: no button in this view carries one.
  - After the existing `<p v-if="currentPath">` path line, `<p v-else-if="sessionActive" data-testid="editor-unsaved">{{ $t("editor-unsaved") }}</p>`.
  - An empty-state paragraph, `v-if="!model"`, `data-testid="editor-empty"`, rendering `{{ $t("editor-empty") }}`, placed after the path/unsaved lines and before the recents section.
  - The recents section's gate changes from `!currentPath && recents.length` to `!model && recents.length`.
  - The diagnostics `<section>` gains `v-if="diagnostics.length"`. Its heading, id and `DiagnosticsPanel` mount are otherwise untouched, and `DiagnosticsPanel` itself is NOT edited.

- [ ] **Step 5: the two falsified comment regions.**
  - The Task-13 doc block's sentence explaining that the validate-on-edit watcher is gated on `currentPath` because only Open sets it is rewritten to name `sessionActive` and to state both funnels that set it. The block's other content is preserved verbatim.
  - The sentence stating `gui-editor.ftl stays 45` is corrected to the recomputed number. **Recompute it from the file rather than copying this plan:** `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl` after Step 6, which must equal 49 (46 measured at authoring plus this task's three). The same correction applies to the `catalog budget is 45` comment in `e2e/smoke.spec.ts`, whose decomposition becomes 42 labels + 1 save-surface note + 2 generic action keys + 1 rule-grid ordinal + this task's 3. **If the recount disagrees with 49, that is a finding: return NEEDS_CONTEXT with both numbers pasted.** Task 4 raises both numbers again by two and Task 5 by three; each task recomputes rather than predicting.

- [ ] **Step 6: the catalogs, both locales, fenced.** Append to `locales/en/gui-editor.ftl`, after its existing generic-action section, exactly:

```
## Profile creation and the pre-session state

editor-action-new = New profile
editor-empty = No profile open. Create one with New profile, or choose an existing profile file.
editor-unsaved = New profile, not saved yet.
```

  and to `locales/de/gui-editor.ftl`, in the same position, exactly:

```
## Profilerstellung und der Zustand vor dem Öffnen

editor-action-new = Neues Profil
editor-empty = Kein Profil geöffnet. Erstelle eines mit Neues Profil oder wähle eine vorhandene Profildatei aus.
editor-unsaved = Neues Profil, noch nicht gespeichert.
```

  None of the six values carries a placeable or an attribute, so attribute-name and pattern-structure parity hold by construction. None is a registry `labelKey`, so D55 rule 3's tooltip duty does not reach them.

- [ ] **Step 7: the tests, a new describe in `e2e/smoke.spec.ts`**, placed after the recents describe, using that file's existing `MKVMERGE_INFO`, `en`/`name` helpers and `assertNoSeriousA11yViolations`. Fixtures: a `warnReport` carrying the measured seed diagnostic (`empty-match-expression`, warning, `tracks[0].match`) and a `cleanReport`; a `PICKED_PATH` distinct from every other path literal in the file, so an identity assertion cannot pass on a shared value.
  - **Case 1, New creates and validates.** Nav to the editor, click `editor-new`. Assert: one `editor-rule-row`; the pattern field carries `.*`; `editor-unsaved` visible and no `batch-profile-current` line; `editor-empty` gone; and the recorded `validate_profile_model` invocation exists, carrying the seed's `input.extensions` - the wire half of the decoupling, since no path exists.
  - **Case 2, the seed is warned, not blocked.** Same flow with `warnReport`: the diagnostics panel lists the `empty-match-expression` line through the en catalog, and `editor-save` is enabled.
  - **Case 3, the pre-session empty state.** Nav to the editor and assert nothing else: `editor-empty` visible, and **absence check E1**, `section[aria-labelledby="editor-diagnostics-heading"]` has count 0. **Its fire is in the same test:** click `editor-new` with `warnReport` mocked and the same locator must have count 1. Run `assertNoSeriousA11yViolations` on both states.
  - **Case 4, Save with no path.** New, then Save with `plugin:dialog|save` mocked to return `PICKED_PATH`. Assert the recorded `plugin:dialog|save` call; the recorded `save_profile` call with `path === PICKED_PATH` and a profile whose `tracks.rules` has length 1; then the `batch-profile-current` line for `PICKED_PATH`; then a recorded `set_settings` whose `recent_profiles[0]` is `PICKED_PATH`.
  - **Case 5, the cancelled dialog.** Same, with `plugin:dialog|save` returning `null`. **Absence check E2:** no `save_profile` call. **Its fire is the recorded `plugin:dialog|save` call in the same test** (the flow ran) plus case 4's non-zero counter for the same command.
  - **Case 6, an already-pathed save is unchanged.** Open a profile, edit, Save: no `plugin:dialog|save` call at all, and `save_profile` carries the opened path. This is the regression guard for the branch, and its fire is case 4.

- [ ] **Step 8: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green. Every pre-existing test passes unchanged; in particular the three `editor-save` assertions in `e2e/editor-rule-add-remove.spec.ts` all run after an Open (measured at authoring), so removing `currentPath` from `saveDisabled` cannot change them, and any change in a pre-existing test's behaviour is a defect signal -> NEEDS_CONTEXT. `git diff --stat` covers exactly the four files in the Files list.

- [ ] **Step 9: commit.**

```bash
git add src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "editor: New creates a blank profile, and the path stops gating validation and Save" -- src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the seed (measured, not chosen); which duty each replacement covers; that Save opens the dialog and no Save-as action is added; that New renders before Open; that the diagnostics section is gated on content rather than on the session; that no "no diagnostics" line is added; that `DiagnosticsPanel.vue` is not edited; that Batch gains no New button; the six fenced catalog values; that the recents memory is fed only when a path is newly established.

---

