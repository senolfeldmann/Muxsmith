# Amendment 3 report: the per-rule detail editor (Task 13b)

**Date:** 2026-07-16
**Scope:** one file edited - `docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`. No code, no commit; working-tree change for delta review.
**Status:** DONE.

## The gap (premises verified against the trees)

| Premise | Source | Verified |
| --- | --- | --- |
| Spec 8.2 promises "detail editor per rule" | `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:374` | Yes - "track-rule grid (...), detail editor per rule, panels for attachments/chapters/tags/title" |
| Design assigns `tracks.rules` the editable `list { item: "trackRule", reorderable: true }` | `docs/superpowers/specs/2026-07-15-plan-6-design.md:925` | Yes - row `| | rules | list { item: "trackRule", reorderable: true } |` |
| `trackRule` registry has 4 fields | design `:817`, `src/editor/registries.ts:139-153` | Yes - `source` (keywordOrBlock), `match` (section of matchExpr), `optional` (**bool**), `changes` (propertyMap settable/scalar) |
| Task 11's grid is the read-only summary occupying that slot | worktree `src/views/EditorView.vue:2-41,318-370` | Yes - bespoke `<table>` of `<tr data-testid="editor-rule-row">`, `optional` cell a `disabled` checkbox, no per-field editing |
| No task builds per-rule track editing | plan grep | Yes - the gap is silence, not an out-of-scope claim (only `profile_version` FixedField is "read-only"; grep for `read-only\|not editable\|out of scope\|per-rule` returns nothing about track rules) |
| Confirmed plan-coverage gap, routed for amendment | `registry-slot-capability-delta`, `docs/decision-ledger.yaml:3562-3575` | Yes - Task-12 review Q2 occurrence, `status: settled`, "routed for a plan amendment". (The `task-12-verdict.md` file was consumed after the 854be3b harvest; its Q2 adjudication survives verbatim in this ledger occurrence.) |
| Owner ruling: option (a), detail panel beneath grid, mkvtoolnix-gui style | task brief (Şenol, 2026-07-16) | Recorded; expandable rows rejected (DOM churn, nested table forms) |

Worktree `plan6-e` HEAD is `5b230a2` - **Task 13 is committed** (its review Approved), working tree clean. Task 13b was written against that committed `EditorView.vue`; the panel insertion point is re-checked against `5b230a2` and holds: `<template v-if="model">` at `:309`, the track-rule `</fieldset>` at `:370`, the save-note `<p>` at `:372`. The plan keeps the anchor's T13-dependent label as defensive wording, but Task 13 is settled.

## The mechanism (with tree evidence)

**Panel = `SectionWidget` over `trackRule`, the identical path `ListWidget` uses for AttachmentRule items.**

- `SectionWidget.vue` resolves `registryByName[of]`, iterates its `EditableField`s, and dispatches each through `FieldWidgetDispatcher`; model `Record<string, unknown> | null`, immutable write `{ ...(model.value ?? {}), [key]: value }`.
- `ListWidget.vue:27-40` synthesizes `{ kind: "section", of: props.spec.widget.item }` and mounts `SectionWidget` per item, writing each back with `setItemValue`. `attachments.rules` (`list { item: "attachmentRule" }`) is exactly this - which is why attachment rules are editable and track rules are not.
- `registryByName["trackRule"] === trackRuleFields` (`src/editor/widgets/shared.ts:51`). So a synthesized `{ labelKey: "editor-tracks-rules", widget: { kind: "section", of: "trackRule", optional: false } }` renders the four `TrackRule` fields through the same dispatcher. Track-rule editing becomes the same code path as attachment-rule editing - closing the capability delta with the registry's own mechanism rather than a bespoke form.
- Write-through: `setRuleValue` rebuilds `tracks.rules` immutably, mirroring `ListWidget.setItemValue` and this file's own `onDrop` (`EditorView.vue:265-278`). Because `model.value` is reassigned to a fresh object, Task 13's `watch(model, ...)` (`EditorView.vue:121-138`) fires validate-on-edit for free; and Task 12's typed cells cover `changes` (settable/scalar) and `match.exact` (matchable/scalar) with no new code.

**Selection = native `<button>` + `:aria-current`, the house precedent.**

- `RunHistory.vue:168-173` selects a run via `<button type="button" data-testid="jobs-history-run" :aria-current="... ? 'true' : undefined" @click="selectRun(...)">` - keyboard-reachable for free, `:aria-current` marks selected. No `tabindex`/`role`/`@keydown` hand-rolled anywhere in the house (grep of JobsView/RunHistory/ResolutionTable/JobRow found none).
- Task 13b wraps the grid's `source` cell text in that same button shape (`data-testid="editor-rule-select"`), so it is keyboard-reachable and axe-clean. The suite runs axe (`smoke.spec.ts:20,75-83`, "serious"+ fails the test); a `<tr>` carrying `tabindex`+`role`+`aria-selected` is exactly what axe flags, which is the concrete reason the button precedent is chosen over an interactive row.
- The button's label is the already-rendered `sourceSummary(rule)` (a real profile token/path, D27 `no-raw-text`-clean like the current `<td>`), so no key is added.

## Zero-new-keys verification

- Catalog is **45/45** en+de (`grep -cE '^[a-z].*=' locales/{en,de}/gui-editor.ftl` = 45 each). Budget unchanged.
- The four `trackRuleFields` labels already exist: `editor-track-rule-source`/`-match-expr`/`-optional`/`-changes` (`locales/en/gui-editor.ftl:46-49`); `SectionWidget` dispatches them via each field's `labelKey`.
- Panel legend reuses `editor-tracks-rules` ("Rules", `:68`) - already the grid `<h2>`/`<caption>`; cross-use reuse is the file's own documented pattern (`EditorView.vue:58-71`).
- Panel labelled by `aria-labelledby` -> the selected grid row `id` (`editor-rule-row-<index>`), a zero-key mechanism naming the panel by the rule it edits.
- `check:i18n` stays green: no new key; the four labels and `editor-tracks-rules` were already counted as used. Testid collision check for `editor-rule-detail`/`editor-rule-select`: NONE in the worktree.
- Escape hatch stated in the task: if a distinct key were ever unavoidable, that is NEEDS_CONTEXT, not an invented or reused-unrelated key (`generic-action-keys` precedent).

## Plan hunks (all in the one file)

1. **Architecture `:7`** - "frontend (Tasks 9-13, plus the detail-editor Task 13b)"; Wave 4 "sequenced after Task 13b".
2. **Amendment 3 block** (new `## Amendment 3 (2026-07-16)`, `:41`) - 4 sentences: gap, owner option-(a) ruling + rejection, vehicle (Task 13b via `SectionWidget` over `trackRule`), marker note.
3. **Wave-3 intro `:1017`** - chain extended `... -> Task 13 -> Task 13b`; edge `13 -> 13b -> 14`; Task 14 "sequenced after Task 13b".
4. **New `### Task 13b` section** (`:1477`) - Files/Interfaces/Read-first/6 binding points/7 TDD steps (red at Step 2, green at Step 6, anti-vacuity `optional` -> real boolean + grid-summary follow, review-check at Step 7). Step 5 also retires two dead directory-picker forward-references - `DirectoryPathWidget.vue:2-7` and the `smoke.spec.ts:701` test title - that mislabelled a never-promised picker as "Task 13's job" (Task-13 review Q3, folded 2026-07-16; the design's `directoryPath` is a plain textbox, D45 widgets prop-fed/zero-IPC). Comment-only, no new key; `DirectoryPathWidget.vue` is added to the task's Files and commit. Its own gate+commit step is for the future executor.
5. **Wave-4 intro `:1585`** - "Task 13b merges to master".
6. **Task 14 sequencing `:1589`** - "after Task 13b" (shared-writer note updated).

Every new/changed passage carries "(amended 2026-07-16, detail-editor routing)". ASCII typography confirmed (the sole AI-tell-glyph grep hit is the pre-existing literal grep pattern at Task 7 `:733`, not amendment text).

## Fork closures / falsifiability

- Selection mechanism: one choice (native button + `:aria-current`), no "or"; interactive-`<tr>` alternative explicitly rejected with the axe rationale.
- Reorder x selection interaction closed: `onDrop` clears `selectedIndex`; out-of-range after Open inert via `rules[selectedIndex] ?? null`.
- Absence-expectation step ("no selection, no panel", count 0) made non-vacuous by pairing with the presence-on-selection assertion in the same test; the RED run exercises the presence branch.
- Integer typed-cell arm marked exemplary (rides Task 12's exhaustive `PropScalarType` switch, no own fixture) - consistent with Task 12's own treatment.

## Concerns

- **Anchor confirmed against committed Task 13 (`5b230a2`, review Approved).** The panel insertion point ("inside `<template v-if="model">`, beneath the grid `</fieldset>` at `:370`, above the save-note `<p>` at `:372`") is present in the committed file; the plan keeps the T13-dependent label as defensive wording, with the stated invariant (directly beneath the grid, above the save note) as fallback. The grid `<table>`/`editor-rule-row` and the tracks `<fieldset>` are Task 11's, stable.
- **Two "Optional"-named checkboxes** (grid summary, disabled; panel BoolWidget, enabled) - the test scopes to `getByTestId("editor-rule-detail")` to disambiguate; called out in Step 1.
- **`SectionWidget` legend redundancy** - the panel shows a "Rules" legend alongside the grid's `<h2>`/`<caption>` "Rules". Cosmetic; the `aria-labelledby`-to-row gives the panel its rule-specific accessible name. Removing the legend would require modifying `SectionWidget` (out of scope).
- **No spec 8.2 edit** - the task fulfils the existing promise; verified the plan carries no out-of-scope claim to retract.

---
Source: session 2274c4dc (Peter), plan-6 amendment 3, created 2026-07-16.
