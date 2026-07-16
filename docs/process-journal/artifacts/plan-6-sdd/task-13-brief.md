### Task 13: D45 - the editor view, part c: open/save, the save note, nav, and ipc signatures

**Files:**
- Modify: `src/views/EditorView.vue` (open/save wiring + the save-surface note + validate-on-edit)
- Modify: `src/App.vue` (the `View` union at `:10`, the nav at `:71-96`, the mount block at `:98-112`)
- Modify: `src/ipc.ts` (hand-written command signatures for the four new commands)
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 8's four commands, Task 12's composed `EditorView.vue`.
- Produces: the complete editor surface, reachable from the nav.

Binding points:
- **`ts-rs` types the model only; command signatures stay hand-written in `src/ipc.ts`** as they are today (D44's explicit out-of-scope). `load_profile`/`validate_profile_model` return the untyped `serde_json::Value` document; the frontend reads `config_diagnostics` and (for `load_profile`) `profile` off it.
- The save-surface note is a **standing note, not a modal**, stated once at the save surface, with **no detection** of whether comments are present (that would need the parser to see them). Its text is `editor-save-note` from Task 9.
- Validation runs through `validate_profile_model` on every edit (spec 7's "every profile edit"), and Save is disabled while errors exist - the one sanctioned frontend affordance.
- Apply-suggestion lives in the **batch view**, not here (Task 14). D41 records why the plan-scope pairing of editor+apply is not a UI-location one: they share the in-memory model's ownership.
- **The editor ships WITHOUT tooltips in Plan 6.** Spec 8.3's editor tooltip/inline-explanation baseline defers to Plan 7 (owner ruling 2026-07-16, folded into the design by Task 1, carried in `docs/ROADMAP.md:74-84`). Do **not** add tooltip keys; `gui-editor.ftl` stays 45 keys (the 43 labels-plus-note Task 9 built, plus the 2 generic action keys Task 12 added; the generic-action-keys ruling revised the budget 43 -> 45, Amendment 2). This task adds no editor catalog keys.
- Follow `App.vue:98-104`'s recorded reason for `v-show` over `v-if` when adding the third view (both views stay mounted so JobsView's live run listeners survive tab switches) - do **not** switch the block to `v-if`.
- **The Tasks 10-12 mount-harness specs (`e2e/mount.ts`) keep running and stay green alongside this task's real-app tests** (amendment 2026-07-16, mount-harness routing): they are neither deleted nor ported, so `EditorView` must stay mountable from an injected `modelValue`, and Task 13's `load_profile` wiring feeds that same model through the app's open flow rather than an unconditional on-mount fetch.

- [ ] **Step 1: Write the failing e2e test**

Extend `e2e/smoke.spec.ts`: the nav gains an editor tab; saving calls `save_profile`; the save note is visible at the save surface; Save is disabled while a diagnostic of severity error is present and enabled when clean; the editor tab stays mounted across a switch to Jobs and back.

- [ ] **Step 2: Run to confirm it fails**

```bash
pnpm test:e2e
```
Expected: FAIL - no nav entry, no save wiring.

- [ ] **Step 3: Implement open/save, the note, the nav entry, and the ipc signatures**

Wire `load_profile`/`save_profile`/`validate_profile_model` into `EditorView.vue`; add the four command signatures to `src/ipc.ts`; add the editor to `App.vue`'s `View` union, nav and `v-show` mount block; render the `editor-save-note`; disable Save while any error-severity diagnostic is present.

- [ ] **Step 4: Run the suite**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS. `pnpm lint` includes the D27 `no-raw-text` rule - every string in the template comes from `$t`.

- [ ] **Step 5: Full gate, then commit**

- **Review-check (mount-harness coverage survives, amendment 2026-07-16, mount-harness routing):** confirm `git diff <task-12-commit> -- e2e/smoke.spec.ts` shows no mount-harness spec deleted, ported to the served app, or guarded/skipped, and that they pass in this task's `pnpm test:e2e`. Confirm `EditorView` mounts from `modelValue` alone (no unconditional `load_profile` in `onMounted`; `load_profile` feeds the model through the app's open flow). A mount spec made green by an on-mount fetch or an injected IPC mock is a wave-3 coverage regression, not a passing gate.

```bash
git add src/views/EditorView.vue src/App.vue src/ipc.ts e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the editor's open/save with its standing note, nav entry and ipc signatures (D45, D41)"
```

---

## Wave 4

Task 13 merges to master, gate green. Then:

