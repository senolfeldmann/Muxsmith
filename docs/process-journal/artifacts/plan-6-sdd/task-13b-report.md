# Task 13b Report: the per-rule detail panel beneath the track-rule grid

**Worktree:** `.worktrees/plan6-e`, branch `plan6-e`. **Base:** `ba5291b`. **Commit:** `a91e56f`.

## What was implemented

Row selection on Task 11's read-only `tracks.rules` grid, plus a per-rule detail panel beneath it that edits the selected rule through the *existing* registry composition (`SectionWidget` over the `trackRule` registry) - the same code path `ListWidget.vue` already uses for `attachments.rules` items. Closes the confirmed plan-coverage gap recorded as `registry-slot-capability-delta` (`docs/decision-ledger.yaml`): spec 8.2's "detail editor per rule" promise, unbuilt until now.

- **Selection:** each grid `<tr>` gets `:id="editor-rule-row-${index}"`; the `source` cell's summary text is wrapped in a native `<button data-testid="editor-rule-select" :aria-current="...">` (the `RunHistory.vue:168-173` house precedent) calling `selectRule(index)`. `selectedIndex = ref<number | null>(null)`.
- **Panel:** `<section v-if="selectedRule" data-testid="editor-rule-detail" :aria-labelledby="editor-rule-row-${selectedIndex}">` wraps a `SectionWidget` fed a synthesized `{ labelKey: "editor-tracks-rules", widget: { kind: "section", of: "trackRule", optional: false } }` spec. Rendered between the rule-grid `</fieldset>` and the save-note `<p>`, inside `<template v-if="model">`.
- **Model writethrough:** `selectedRule` (computed) and `setRuleValue` mirror `ListWidget.vue`'s `itemValue`/`setItemValue` exactly - immutable rebuild of `model.value.tracks.rules[selectedIndex]`, feeding the same `rules` computed the grid renders from, so panel edits and grid summaries share one model.
- **Selection cleared on reorder:** `onDrop` gained one additive line, `selectedIndex.value = null`, after the immutable rules rebuild.
- **Rider (comment-only, no behavior change):** `DirectoryPathWidget.vue:2-6` and the Task-10 `smoke.spec.ts:703` test title both reworded to retire the dead "wiring a real picker is Task 13's job" forward-reference; new text states the settled boundary (picker out of scope for Plan 6, D45 widgets are prop-fed/zero-IPC).
- **Header doc comment:** `EditorView.vue`'s Task-11 paragraph amended (grid stays read-only *of the row values*, now also carries selection + a detail panel); a new Task-13b paragraph added after the Task-13 paragraph, following the file's existing per-task documentation convention.

## TDD evidence

**RED** (`pnpm test:e2e -g "rule detail editor"`, before any implementation code):
```
1) [chromium] › e2e/smoke.spec.ts:1140:3 › editor view: rule detail editor (Task 13b, D45 / spec 8.2) › ...
   Error: locator.click: Test timeout of 30000ms exceeded.
   Call log:
     - waiting for getByTestId('editor-rule-select').first()
   > 1153 |     await page.getByTestId("editor-rule-select").first().click();
  1 failed, 25 passed (30.9s)
```
Exactly the named RED the brief predicted: the selection button does not exist yet, so the click times out and the panel never appears. All 25 pre-existing specs (including the retitled Task-10 directoryPath test) stayed green through the RED run, confirming the rider rename alone introduced no regression.

**GREEN** (`pnpm test:e2e`, after implementation): `26 passed (1.4s)`, all chromium. The Task 13b test's three in-order assertions (count-0 pre-selection, panel + four fields on select, `optional` writethrough with `.toBe(true)` plus the grid-row checkbox `toBeChecked()`) pass.

## Catalog key counts

- `locales/en/gui-editor.ftl`: **45** message ids (`grep -cE "^[a-zA-Z0-9-]+ =" `), unchanged.
- `locales/de/gui-editor.ftl`: **45**, unchanged.
- `git diff <task-13-commit> -- locales/`: empty (no locale file touched).
- `pnpm check:i18n`: `ok (35 source files scanned, 231 catalog ids, 17 unused warning(s), ...)` - the 17 warnings are pre-existing IPC-error-code false positives (unrelated `gui-common.ftl`/`gui-jobs.ftl` ids), not introduced by this task.

## Gate results (all foreground, in order)

1. `cargo fmt --all --check` - clean, no output.
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean.
3. `cargo test --workspace` - all passed (81 in `muxsmith-core`, plus GUI/CLI/xtask suites, plus 2 codegen tests), 0 failed.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean.
5. `cargo deny check` - `advisories ok, bans ok, licenses ok, sources ok`, exit 0.
6. `pnpm lint` - clean.
7. `pnpm build` (`vue-tsc --noEmit && vite build`) - clean, typechecks.
8. `pnpm check:i18n` - ok (see above).
9. `pnpm test:e2e` - `26 passed (1.4s)`.

## Files changed

- `src/views/EditorView.vue` - selection wiring, detail panel, model writethrough, header doc comment (98 insertions / 8 deletions net in diff stat).
- `src/editor/widgets/DirectoryPathWidget.vue` - comment-only rider (5 lines changed, no code).
- `e2e/smoke.spec.ts` - one additive describe block (`editor view: rule detail editor (Task 13b, D45 / spec 8.2)`) plus the comment-only Task-10 test-title reword. `git diff 5b230a2 -- e2e/smoke.spec.ts`: 59 insertions, 1 deletion - the 1 deletion is the retitled line, no assertion touched.

Commit: `a91e56f gui: the per-rule detail editor beneath the track-rule grid, via SectionWidget over trackRule (D45, spec 8.2; registry-slot-capability-delta), and retire the dead directory-picker forward-references (Task-13 review Q3)`.

## Self-review

- **Zero new catalog keys:** confirmed 45/45 en+de, zero `.ftl` diff against the Task-13 base.
- **Protected specs untouched-except-enumerated:** `git diff 5b230a2 -- e2e/smoke.spec.ts` shows only the additive describe block and the one comment-only title reword; no Task 10/11/12/13 mount-harness spec deleted, ported, guarded, or skipped. Task 11's grid assertions (`toHaveCount(2)`, `toContainText`, drag-reorder model check) and Task 12's composition assertions ran unmodified and green.
- **`EditorView` still mounts from `modelValue` alone:** `grep -n "onMounted\|load_profile" src/views/EditorView.vue` shows `loadProfile` called only inside the user-triggered `pickAndOpen`, no `onMounted` hook added.
- **Panel is pure registry composition:** `ruleDetailSpec` synthesizes `{ kind: "section", of: "trackRule" }` and dispatches through `SectionWidget` -> `FieldWidgetDispatcher`, no hand-built field markup; `source`/`optional`/`match`/`changes` render via `KeywordOrBlockWidget`/`BoolWidget`/`SectionWidget`(nested)/`PropertyMapWidget` exactly as the registry already declares (`registries.ts:139-153`).
- **Selection cleared on reorder:** `onDrop`'s added line verified by reading the diff; no dedicated test was added for this since the brief's Step 1 enumerated exactly three assertions for the one new test and did not list a reorder-clear assertion - implemented per the binding-point prose, not test-driven, to stay inside the brief's explicit enumeration (no unrequested test surface added).
- **Both rider comments reworded:** `DirectoryPathWidget.vue:2-6` and `smoke.spec.ts:703`'s test title, both diff-verified comment-only (no assertion or behavior line touched).
- **Files list exhaustive:** `git status --short` before commit showed exactly the three brief-authorized files; nothing else in the tree touched.

## Concerns

None. One deliberate scope note: the brief's binding points describe "selection cleared on reorder" and axe-safety as design rationale for the native-button choice, but Step 1's explicit RED-test enumeration is only the three assertions inside one test. I implemented the reorder-clear behavior (one line in `onDrop`) without adding a fourth, unrequested assertion for it, and did not add a new axe scan for the panel (the existing axe-covered fixtures render `tracks.rules: []`, so the added button markup is never exercised by an axe scan in the protected suite) - both choices stay strictly inside the brief's explicit enumeration rather than extrapolating additional test surface.
