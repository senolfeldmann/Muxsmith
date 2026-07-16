# Verdict (extracted from the reviewer transcript)

All premises verified. Let me finalize the assessment.

The dependency sweep confirms the load-bearing sequencing references are all updated (architecture waves line :7, wave-3 chain :1017 with `13 -> 13b -> 14` and `F4: not parallel with 9-13b`, wave-4 gate :1589, Task-14 sequencing :1593). The "Task 13" references left unchanged (:35, :1126, :1217, :1262, :1273, :1423, :1442) all correctly refer to nav/IPC landing in Task 13 proper, which 13b does not change. The AI-tell scan's only hit (:733) is a pre-existing literal grep pattern in Task 7, not amendment text. Two descriptive "Produces, for Tasks 9-13"/"for Task 13" lines (:761, :1361) were not extended to 13b, but 13b introduces no new binding/key.

---

### Verdict
**Amendment:** APPROVED

### Executability checks

**Panel mechanism (`SectionWidget`/`FieldWidgetDispatcher` over `trackRule`) — HOLDS.**
- `shared.ts:51` maps `trackRule: trackRuleFields` (plan6-e tree). `registryByName` is typed `Record<RegistryName, ...>` so `of: "trackRule"` is a valid section target.
- `registries.ts:139-153`: `trackRuleFields` carries the four fields with the existing labelKeys — `source` (keywordOrBlock), `match` (section of matchExpr), `optional` (**bool**), `changes` (propertyMap settable/scalar), labelKeys `editor-track-rule-source`/`-match-expr`/`-optional`/`-changes`.
- `ListWidget.vue:27-40` proves the composition for rule-shaped items: it synthesizes `{ kind: "section", of: item }`, mounts `SectionWidget` per item, and writes back immutably via `setItemValue`. `SectionWidget.vue:18-35` resolves `registryByName[of]`, iterates `EditableField`s through `FieldWidgetDispatcher`, model `Record<string,unknown>|null`, immutable write. `FieldWidgetDispatcher.vue:33-60` dispatches `section` and `bool` (and the rest). The panel's `ruleDetailSpec` reuses `EditableFieldOf<"section">` (exported `shared.ts:32`) — byte-identical to the AttachmentRule path.

**Row selection (RunHistory native button + `:aria-current`) — HOLDS.** `RunHistory.vue:168-173` is exactly `<button type="button" data-testid="jobs-history-run" :aria-current="... ? 'true' : undefined" @click=...>`. The cited house precedent exists as described; keyboard-reachable for free, no `tabindex`/`role`/`@keydown`.

**Zero-new-keys — HOLDS.** `locales/{en,de}/gui-editor.ftl` = 45/45. The four `trackRule` labels exist at `:46-49`; `editor-tracks-rules` at `:68`. No `editor-rule-detail`/`editor-rule-select` testid exists in tree (negative grep verified sound via control on `editor-rule-row`, present at `EditorView.vue:350` / `smoke.spec.ts:799`). Panel named via `aria-labelledby` → the selected `<tr id>`; budget statements stay 45 throughout; no catalog key added.

**T13-dependent anchors — HOLD at 5b230a2.** `<template v-if="model">` at `EditorView.vue:309`, track-rule `</fieldset>` at `:370`, save-note `<p>` at `:372`. Insertion point (between `:370` and `:372`, inside the `v-if="model"` block) is exactly as the plan describes. Header comment `:1-74` carries the "stays this bespoke, read-only-summary grid" sentence (`:7`) that Step 5.1 amends. `watch(model,...)` gated on `currentPath` at `:121-138`; `onDrop` at `:265-278`.

**Protected-spec boundary — HOLDS.** Task 11 grid spec (`smoke.spec.ts:785-825`) asserts row count (`toHaveCount(2)`), the `match`-cell text (`toContainText("video")`/`"audio"`) and drag-reorder — none touch the `source` cell, so wrapping its text in a button leaves them green. Task 12 composition spec (`:836-897`) mounts `tracks:{...,rules:[]}` (`:845`) so no row/panel renders. Axe helper (`:78-83`) fails on serious/critical only; native-button selection avoids the `<tr>`+tabindex+role shape axe flags. `smoke.spec.ts:701` reword is scoped to the test **title** only (assertions `:704-712` are independent of it).

### Coverage
- **Sequencing 13 → 13b → 14:** architecture line `:7`, wave-3 chain `:1017` (`13 -> 13b -> 14`; `F4: not parallel with 9-13b`), wave-4 gate `:1589`, Task-14 `:1593` all re-pointed with markers. Dependency/architecture text updated.
- **Amendment-3 summary block** present (`:43`), 4 sentences (≤5): gap, owner option-(a) ruling + rejection rationale, vehicle, marker note.
- **TDD red/green with named failures:** Step 1 writes assertions; Step 2 RED names the failure (`editor-rule-select` matches nothing → click times out, panel never appears); Step 6 GREEN.
- **Model-writethrough + anti-vacuity:** Step 1.3 checks the panel `optional` (fixture row 0 has it unset → real transition), asserts `readModel().tracks.rules[0].optional` `.toBe(true)` (real boolean, not `"true"`) **and** the grid row summary checkbox `toBeChecked()`. BoolWidget `v-model` binds a boolean and its `<label for>` gives the accessible name, so both arms are genuine. Two "Optional" checkboxes disambiguated by scoping to `editor-rule-detail`.
- **Rider folded:** `DirectoryPathWidget.vue` in Files (`:44`) and git-add (`:142`); Step 5.2 rewords both dead references; Step 7 review-check names the comment-only reword. The dead "wiring a real picker is Task 13's job" clause confirmed present at `DirectoryPathWidget.vue:6-7`.

### No latitude
Empty-state stated without a key: "No selection renders no panel … `v-if` on the selected rule" plus Step 1.1's count-0 assertion. Reorder×selection closed (`onDrop` clears `selectedIndex`; out-of-range inert via `rules[selectedIndex] ?? null`). "Nothing else in the grid changes" enumerates the preserved set (testid, drag handlers, summary cells). No unenumerated normative set or unmarked exemplary list; the Integer typed-cell arm is explicitly exemplary (rides Task 12's exhaustive `PropScalarType` switch).

### Scope discipline
One file (`git status` shows only the plan modified). Markers "(amended 2026-07-16, detail-editor routing)" on every new/changed passage. No contradiction with amendments 1-2: budget stays 45 (no `43`/`44` reintroduced), typed cells consumed from Task 12/Ruling 2, mount-harness terms re-affirmed (Step 7 review-check). No settled decision reopened; spec/design not edited. Typography clean (only AI-tell hit is the pre-existing literal grep pattern at `:733`).

### Falsifiability
Absence assertion (Step 1.1, count 0) made non-vacuous by the paired presence assertion (Step 1.2) on the **same** `editor-rule-detail` selector — the known-present case that proves the selector resolves, so count-0 cannot pass by misspelling; both pass in GREEN only if the panel is genuinely selection-gated. RED (Step 2) names its failure. Fork closures explicit (single selection mechanism, interactive-`<tr>` rejected with axe rationale).

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **`smoke.spec.ts:701` cited; actual line is `:703`** (Files list and Step 5.2). Off-by-2 at 5b230a2; the report propagated the same number. Content anchor is exact (the quoted title fragment `"(no IPC dialog -- Task 13's job)"` is unique), so the executor resolves it correctly — cosmetic accuracy only.
- **Wave-level handoff enumerations not extended to 13b:** `:761` "Produces, for Tasks 9-13" (Task 5 bindings) and `:1361` "Produces, for Task 13" (Task 12 output). Task 13b is a downstream consumer but introduces no new binding, key, or field, so no contract changes — strict dependency-sweep completeness only, not a correctness defect.

### HARVEST
- **Well-constructed non-vacuous absence assertion pattern:** pairing a count-0 (pre-condition) assertion with a presence assertion on the *same selector* satisfies the fire-once discipline for negatives without a deliberate-break step — the presence branch is the known-present control that proves the selector resolves. Reusable template for "absence unless triggered" e2e checks.
- **Registry-composition reuse to close a capability gap:** the amendment closes a spec gap by routing the read-only slot through the *existing* `SectionWidget`/dispatcher path (the same one an adjacent editable slot already uses) rather than a bespoke form — "reuse before writing" applied at the plan-amendment level, with the precedent component cited by file:line as the proof the path composes.
