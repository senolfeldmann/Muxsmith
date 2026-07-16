### Task 11: D45 - the editor view, part a: the rule grid and drag-reorder

**Files:**
- Create: `src/views/EditorView.vue` (the rule grid + drag-reorder; sections and open/save follow in Tasks 12-13)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 5's `profile.ts` types (`Profile`, `TrackRule` (`model.rs:198`), ...).
- Produces, for Tasks 12-13: the `EditorView.vue` scaffold holding the rule list.

Binding points:
- The view holds the model as data; drag-reorder emits a **reordered model**, not a mutation of the DOM. `tracks.rules` is output track order (`reorderable` in the registry), so reordering is a semantic edit the user makes deliberately.
- No validation, no save, no widgets yet - those are Tasks 12 and 13. This task is the grid and its reordering only, so it stays reviewable as one unit.

- [ ] **Step 1: Write the failing e2e test (amended 2026-07-16, mount-harness routing)**

Extend `e2e/smoke.spec.ts`: mount `EditorView.vue` through the Task-10 harness (`e2e/mount.ts`), not the served app (which has no editor mount point until Task 13): `mountComponent(page, { component: "EditorView", props: { modelValue: <two-rule profile> } })`. Assert the rule grid renders both rows in order; perform a drag-reorder and assert both that the rendered rows swap and that `readModel(page)`'s `tracks.rules` reflects the new order (the harness round-trips `update:modelValue`). EditorView therefore takes the profile as its `modelValue` prop and emits `update:modelValue` on reorder - the natural pre-IPC shape; open/save IPC is Task 13.

- [ ] **Step 2: Run to confirm it fails (amended 2026-07-16, mount-harness routing)**

```bash
pnpm test:e2e
```
Expected: FAIL - `__muxsmithMount__` throws `unknown mount component "EditorView"` because `src/views/EditorView.vue` does not exist yet (the glob registry has no `EditorView`). That throw is the RED.

- [ ] **Step 3: Implement the rule grid and drag-reorder**

Create `src/views/EditorView.vue` with the rule list and its reordering. Match the house component conventions (read `src/views/BatchView.vue` first for the view-level prop/emit/`$t` shape). No nav wiring yet - `App.vue` is untouched until Task 13.

- [ ] **Step 4: Run the suite (amended 2026-07-16, mount-harness routing)**

```bash
pnpm build && pnpm lint && pnpm test:e2e
```
Expected: PASS - the harness rebuild picks up the new `EditorView.vue` and the mount assertions render green. `pnpm lint` includes the D27 `no-raw-text` rule - every string in the template comes from `$t`.

- [ ] **Step 5: Full gate, then commit**

```bash
git add src/views/EditorView.vue e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the profile editor's rule grid with drag-reorder (D45)"
```

---

