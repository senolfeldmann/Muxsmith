### Task 16: D59 - the ordinal column

Read D59 in full.

**Files:**
- Modify: `src/views/EditorView.vue` (grid template), `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl`
- Test: extend the grid assertions in `e2e/smoke.spec.ts` (the "editor view: rule grid + drag-reorder" describe block - the one place the grid columns are asserted today)

**Interfaces:** consumes/produces nothing beyond the one key.

- [ ] **Step 1: Failing assertion**: the grid renders a leading `Order` header and 1-based cells matching row position; after a drag-reorder the ordinals re-render 1..n in the new order. Run the spec; expected FAIL.

- [ ] **Step 2: Add the key** (en `Order`; de draft `Reihenfolge`, rides the owner pass) - `gui-editor.ftl` en/de now 46/46 ids (45 + 1; the budget revision of `editor-generic-action-keys` is the controller's trigger-10 duty, not a task edit). The key gets NO `.tooltip` (it is not a registry label; the attribute-less set becomes: `editor-save-note`, `editor-action-add`, `editor-action-remove`, `editor-track-rule-order`).

- [ ] **Step 3: Add the column**: header `<th scope="col">{{ $t("editor-track-rule-order") }}</th>` first in the grid's header row; leading cell `<td>{{ index + 1 }}</td>` per row. **No data change**: order stays encoded solely as array position, drag mechanics untouched, no per-row Fluent key (a digit is locale-neutral data).

- [ ] **Step 4: Run the spec + full gate.** Expected: PASS / green.

- [ ] **Step 5: Commit**

```bash
git add src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "editor: presentation-only 1-based ordinal column on the rule grid (D59); gui-editor 46 ids" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Wave 3

Branch `.worktrees/plan7-g` (stream G, Tasks 17-19 serial - one file) and `.worktrees/plan7-h` (stream H, Task 20) from master after wave 2 merges. G and H are file-disjoint.

---

