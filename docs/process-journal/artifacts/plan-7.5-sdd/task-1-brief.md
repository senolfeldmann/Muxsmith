### Task 1: D65-D70/D72 - the Add/Remove affordance and `e2e/editor-rule-add-remove.spec.ts` (cases 1-8)

**Stream A** (`.worktrees/plan75-a`). Implementer: **mid tier**; reviewer: **mid tier**. Read D65, D66, D67, D69, D70, D72 in full, design sections 1 (ground truth), 5 (cases 1-8) and 8, and `core-83-zero-rule-keep-passthrough` (`docs/product-boundaries.yaml:389`).

**Files:**
- Modify: `src/views/EditorView.vue`
- Test: `e2e/editor-rule-add-remove.spec.ts` (new)

**Interfaces:**
- Consumes: `editor-action-add`/`editor-action-remove` (existing catalog ids, `locales/en/gui-editor.ftl:138-139`, `locales/de/gui-editor.ftl:142-143`; no `.tooltip` attribute in either locale); EditorView's existing `model`/`rules`/`selectedIndex` state and immutable-swap mutation pattern; the mount harness (`mountComponent`/`readModel`, `e2e/mount.ts`); the mock layer (`installTauriMocks`/`resolveWith`, recorded invokes, `e2e/mocks.ts`); `Diagnostic`/`LoadProfileDocument`/`ReportDocument`/`MkvmergeInfo` (`src/ipc.ts`), `Profile` (`src/bindings/profile.ts`), `en` (`e2e/i18n-en.ts`).
- Produces: `addRule`/`removeSelectedRule` and the two buttons (`data-testid="editor-rule-add"`/`"editor-rule-remove"`) - Task 2's case 9 and Task 4's amendment 1 assert they exist.

Existing-spec invariant (design section 5): the buttons render outside the `<table>`, so the Task-11 grid assertions and the Task-13b detail-panel spec (`e2e/smoke.spec.ts`) and `e2e/editor-markers.spec.ts` see no DOM change inside their selectors; every existing spec stays green and none is edited by this task.

- [ ] **Step 1: Re-verify the template anchor at the execution tree** (quoted text, not line number): the rules `<table>` closes directly before the fieldset closes -

```bash
grep -n -A1 '</table>' /home/senol/Git/Muxsmith/.worktrees/plan75-a/src/views/EditorView.vue
# Expected: exactly one hit, and the following line is `</fieldset>`
# (measured at plan-authoring: :598/:599). A second </table> or different
# successor line is a NEEDS_CONTEXT stop, not an adaptation.
```

- [ ] **Step 2: Write the failing mount-harness cases 1-5** in the new `e2e/editor-rule-add-remove.spec.ts` - the file's eight cases under one describe titled `"editor rule add/remove (D65-D70, D72)"` so the step greps below select the file by title chain - following the Task-11/13b sibling pattern (`e2e/smoke.spec.ts`: `mountComponent(page, { component: "EditorView", props: { modelValue: ... } })`, model assertions via `readModel`, no IPC - the mount harness has no `currentPath`, so the `watch(model)` revalidation never fires, verified in the design's ground truth). Fixture: a two-rule profile with distinguishable match summaries (`exact: { type: "video" }` / `exact: { type: "audio" }`, the smoke sibling's own fixture shape); case 5 uses a one-rule variant of the same shape. The row-cell layout, measured at the current tree: cell 0 ordinal, cell 1 the `editor-rule-select` button (its text is `sourceSummary`), cell 2 match summary, cell 3 the disabled optional checkbox, cell 4 changes summary.

  1. **Add appends**: click `editor-rule-add`; `editor-rule-row` count 3; the new last row's ordinal cell renders "3", its select button text is "primary", its match cell (cell 2) and changes cell (cell 4) render empty, its checkbox is unchecked; anti-vacuity model assertion via `readModel`: `tracks.rules` has length 3 and member [2] `toEqual({ match: {} })` - the model value, not just the DOM.
  2. **Add auto-selects and opens the panel**: after Add, the new row's `editor-rule-select` button carries `aria-current="true"`; `editor-rule-detail` is visible and renders the four `trackRule` fields (accessible names `en("editor-track-rule-source")`, `-match-expr`, `-optional`, `-changes`).
  3. **Remove gating**: on a fresh mount, `editor-rule-remove` `toBeDisabled()`; after clicking a row's select button, `toBeEnabled()`.
  4. **Remove deletes the selected rule**: select row 0 ("video"), click Remove; row count 2 -> 1; the removed rule's summary text ("video") gone, the other rule's summary ("audio") still present (the right-rule assertion); selection cleared: no row carries `aria-current="true"`, `editor-rule-detail` has count 0.
  5. **Remove down to zero, Add from zero**: with one rule, select and Remove; zero `editor-rule-row` rows; the caption still renders (`en("editor-tracks-rules")` text) and the first columnheader still renders `en("editor-track-rule-order")`; `editor-rule-add` still present. Click Add: one row, its select button `aria-current="true"`, panel visible.

- [ ] **Step 3: Write the failing mocked-IPC cases 6-8** in the same file, following the `e2e/editor-markers.spec.ts` pattern (real app + `installTauriMocks`, nav to editor, open via `editor-open` with `"plugin:dialog|open"` resolving a path and `load_profile` resolving a `LoadProfileDocument`; markers located by `data-diag-path`, the marker/severity-class assertions as in that sibling). Mock-queue mechanics (documented in `e2e/mocks.ts`): entries are consumed per call and the last repeats, so each case queues `validate_profile_model: [resolveWith(cleanReport), resolveWith(<case report>)]` - the open fires the first, the Add/Remove model swap fires the second. Fixture diagnostics carry a `rendered` field only because the `Diagnostic` type requires it: it is wire ballast no frontend surface displays (`DiagnosticsPanel.vue` renders Fluent over `code`/`params`; `src/ipc.ts` documents the non-display) - diagnostic text assertions go through the `i18n-en` binding `en(code)`, never the `rendered` field (`e2e-diagnostic-rendered-is-wire-ballast`, `docs/decision-ledger.yaml:4110`).

  6. **Add wires the skeleton and the warning lands in the panel**: fixture profile with exactly one rule (`exact: { type: "video" }`); open, click `editor-rule-add`. Second validate response: `{ code: "empty-match-expression", severity: "warning", config_path: "tracks[1].match", params: {}, rendered: "empty-match-expression" }`. Assert: (a) the wire-truth payload - the LAST recorded `validate_profile_model` invocation's `args` carry a `profile` whose `tracks.rules` has length 2 with member [1] `toEqual({ match: {} })`; (b) the marker `[data-diag-path="tracks[1].match"]` renders with class `diag-marker--warning` INSIDE the open `editor-rule-detail`; (c) the grid ROW marker for the bare path - `[data-diag-path="tracks[1]"]` - has count 0 (the exact-anchoring negative, with (b) as its in-test positive control); (d) the diagnostics panel (`section[aria-labelledby="editor-diagnostics-heading"] li`) contains a line whose text includes `en("empty-match-expression")` - the Fluent-rendered en value, no args needed (the message has no placeables: `locales/en/diagnostics.ftl:9` is placeable-free), asserted via the catalog binding and never via the fixture's `rendered` field (`e2e-diagnostic-rendered-is-wire-ballast`); (e) **`editor-save` stays enabled** - D65's warning-severity consequence, pinned deliberately so a future core severity change fails loudly (design trigger 4).
  7. **Remove to zero under `drop`**: fixture one rule, `tracks.unmatched: "drop"`; open, select the row, click Remove. Second validate response: `{ code: "no-track-rules", severity: "error", config_path: "tracks.rules", params: {}, rendered: "no-track-rules" }`. Assert: the caption marker `[data-diag-path="tracks.rules"]` renders class `diag-marker--error`; `editor-save` `toBeDisabled()`.
  8. **Remove to zero under `keep`**: same with `tracks.unmatched: "keep"` and second response `{ code: "passthrough-profile", severity: "info", config_path: "tracks.rules", params: {}, rendered: "passthrough-profile" }`. Assert: caption marker class `diag-marker--info`; `editor-save` `toBeEnabled()`.

- [ ] **Step 4: Run the new spec to verify it fails**

Run: `pnpm test:e2e -- --grep "editor rule add/remove"`
Expected: FAIL every case - `editor-rule-add`/`editor-rule-remove` do not exist yet. This red run is the fire event for every later green.

- [ ] **Step 5: Implement - transcribe the two closed shapes.** In `src/views/EditorView.vue`:

(a) Script: insert both functions immediately after `onDragEnd`'s closing brace (the end of the rule-grid mutation region), before `</script>` - D67's shapes verbatim; no other state is introduced (design section 8):

```ts
function addRule() {
  if (!model.value) return;
  const next = [...rules.value, { match: {} }];
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, rules: next },
  };
  selectedIndex.value = next.length - 1;
}

function removeSelectedRule() {
  if (selectedIndex.value === null || !model.value) return;
  const next = [...rules.value];
  next.splice(selectedIndex.value, 1);
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, rules: next },
  };
  selectedIndex.value = null;
}
```

(If `pnpm lint`'s curly/prettier rules demand braces around the single-line guards, that mechanical reformat is the only permitted delta inside the function bodies - semantics byte-identical. A short comment in the file's register may note the D66 clearing rationale, placed ABOVE `removeSelectedRule` and outside both function bodies, so it never enters Step 6's extraction - the in-body `onDrop` comment is the register precedent, not a placement mandate; no line-number citations in comments, `code-comment-line-citations-drift`.)

(b) Template: insert between `</table>` and `</fieldset>` (Step 1's verified anchor) - D70's block verbatim, indented to the surrounding template depth:

```html
<button
  type="button"
  data-testid="editor-rule-add"
  @click="addRule"
>
  {{ $t("editor-action-add") }}
</button>
<button
  type="button"
  data-testid="editor-rule-remove"
  :disabled="selectedIndex === null"
  @click="removeSelectedRule"
>
  {{ $t("editor-action-remove") }}
</button>
```

Native `<button type="button">`, Add before Remove, visible `$t` text only - no `aria-label`, no `title` (D70/D72); Remove's only disable condition is `selectedIndex === null`; Add is never disabled while the grid renders.

- [ ] **Step 6: Diff the landed blocks against the design** (the D70/D72 enforcement and the anti-truncation check): extract the two functions and the template block from the landed file and diff against D67's and D70's blocks in the design, whitespace-insensitive (`diff -wB`), expecting semantic identity (the two permitted deltas: template indentation, guard-brace formatting; a Step-5 comment sits above the functions, outside the extracted blocks, and therefore never appears in this diff - the enumeration and Step 5's permission agree by construction). Any attribute delta - an added `title`, `aria-label`, a changed `:disabled` expression, swapped button order - is a defect, not an adaptation. State in the task report that the diff ran and what it showed.

- [ ] **Step 7: Run cases 1-8 to green, then the untouched-spec invariant**

Run: `pnpm test:e2e -- --grep "editor rule add/remove"` - expected PASS (all 8), then the full `pnpm test:e2e` - expected: every pre-existing spec green with zero edits to any existing spec file (`git status` shows exactly `src/views/EditorView.vue` modified and the one new spec file untracked).

- [ ] **Step 8: The focus-management negative, fire-verified** (D67; the design's section-1 negative re-run at the execution tree):

```bash
grep -rnE '\.focus\(|autofocus' /home/senol/Git/Muxsmith/.worktrees/plan75-a/src/
# Expected: no output (the file's `focusin` uses do not match this pattern).
# Fire-verify: temporarily add a line `el.focus()` to any src/ file, re-run,
# confirm the hit, revert, re-run to confirm empty.
```

- [ ] **Step 9: The zero-surface check, fire-verified** (D68 + the zero-new-surface constraint):

```bash
cd /home/senol/Git/Muxsmith/.worktrees/plan75-a && git diff --name-only && git status --porcelain
# Expected: the diff names exactly src/views/EditorView.vue; porcelain adds only
# e2e/editor-rule-add-remove.spec.ts. In particular NOTHING under locales/,
# help/, src/editor/, scripts/, src-tauri/, crates/.
# Fire-verify the scope-absence reading once: `git diff --name-only -- src/`
# must print src/views/EditorView.vue (the positive control proving the
# command surfaces changes), while `git diff --name-only -- locales/ help/`
# prints nothing.
```

- [ ] **Step 10: Frontend gate, foreground**

Run: `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e`
Expected: green. check-i18n sees byte-identical catalogs (id counts stay 46/46); eslint `no-raw-text` is satisfied - the buttons' only text nodes are `$t()` calls.

- [ ] **Step 11: Commit**

```bash
git add src/views/EditorView.vue e2e/editor-rule-add-remove.spec.ts
git -c commit.gpgsign=false commit -m "editor: Add/Remove for track rules on the bespoke grid - skeleton append + auto-select, selection-scoped unconfirmed remove, generic action keys (D65-D70, D72); e2e cases 1-8" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

