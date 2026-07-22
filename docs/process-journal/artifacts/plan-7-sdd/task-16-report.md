# Task 16 report - D59 ordinal column on the rule grid

**Verdict:** DONE

**Worktree:** `/home/senol/Git/Muxsmith/.worktrees/plan7-f` (branch `plan7-f`)
**Commit:** `8f6400f2d13c18a575975b4ba6f2c8d7c1734dcc`

## What changed

A presentation-only 1-based ordinal column was added as the leading column
of the profile editor's track-rule grid (D59). Four files, +28 lines, no
deletions:

- `src/views/EditorView.vue`: leading header
  `<th scope="col">{{ $t("editor-track-rule-order") }}</th>` in the grid's
  `<thead>` row (before `editor-track-rule-source`), and a leading per-row
  cell `<td>{{ index + 1 }}</td>` (before the source cell). Header formatted
  multiline to match the four existing `<th>` siblings; the cell inline to
  match the existing simple cells (`matchSummary`/`changesSummary`).
- `locales/en/gui-editor.ftl`: new key `editor-track-rule-order = Order`.
- `locales/de/gui-editor.ftl`: new key `editor-track-rule-order = Reihenfolge`
  (draft per Global Constraint - rides the owner's rendered-surface pass;
  declarative-noun register, consistent with the de catalog header rules).
- `e2e/smoke.spec.ts`: extended the existing single test in the
  `editor view: rule grid + drag-reorder (Task 11, D45)` describe block
  (no restructuring). Two additive assertion groups: initial state (first
  columnheader is "Order"; row 0/1 leading cells are "1"/"2") and post-
  reorder state (leading cells stay "1"/"2" by position, proving the ordinal
  tracks array position, not rule identity, since it now sits with the
  swapped content).

**No data change** (as D59 confirms): order stays encoded solely as
`tracks.rules` array position, drag mechanics (`onDragStart`/`onDrop`)
untouched, no per-row Fluent key (a digit is locale-neutral data). The
ordinal re-renders reactively because the `v-for` re-runs on the drag
rebuild.

**Key placement / attribute-less set:** the new key carries no `.tooltip`
(it is not a registry label). It was placed in a new trailing
`## Rule grid ordinal (D59)` section in both catalogs, mirroring the
existing `## Save surface (D41)` single-key ADR-referenced section, rather
than inside the all-`.tooltip` `## TrackRule` registry-label section - so
the section's "every label carries a tooltip" uniformity is preserved and
the key sits with the file's other non-registry, attribute-less keys. The
attribute-less set is now exactly `{editor-save-note, editor-action-add,
editor-action-remove, editor-track-rule-order}` per the brief.

## Brief-vs-tree divergences

None material. The premises held against the tree:
- Grid was 4 columns (source, match-expr, optional, changes) at HEAD 9f4aa8a;
  T14's field-anchored diag marker lives inside the source cell, T15's
  curated-domain dropdowns live in the rule-detail section below the grid -
  neither added a grid column, so D59 makes it the 5th column as designed.
- The design's line anchors (`EditorView.vue:463-481` grid, `:358-372` drag)
  had shifted in the current tree (grid `<thead>` at ~534, drag handlers
  `onDragStart`/`onDrop` at 412-433); re-located by quoted structure. No
  behavioral premise refuted.

## Per-step

- Step 1 (failing assertion): done. Wrote the assertions, ran the filtered
  spec, watched it FAIL.
- Step 2 (add key en/de): done. Both catalogs 45 -> 46 ids (recounted, see
  below).
- Step 3 (add column): done.
- Step 4 (spec + gate): done, green.
- Step 5 (commit): done, `8f6400f`.

## Count recompute (not borrowed)

`grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/{en,de}/gui-editor.ftl`
(column-0 id lines, mirroring check-i18n's `MESSAGE_ID_RE`; indented
`.tooltip` lines excluded): **en 46, de 46**. Matches the brief's 45 -> 46.

## Test / gate evidence (all foreground)

Iteration used `playwright test --grep "drag-reorder swaps the rows"` with
`node_modules/.bin` prepended to PATH (the webServer shell otherwise cannot
resolve `vite`), rebuilding the mount bundle
(`vite build --config e2e/vite.mount.config.ts`) after each source/catalog
change.

Fire-verification (watched the checks fire, then pass):
1. **Step 1, key absent:** `1 failed` -
   `Error: e2e/i18n-en: no message "editor-track-rule-order" in the en catalog`
   (the `name()`/`en()` helper throws on the missing key).
2. **Step 2 intermediate, key present + column absent:** `1 failed` -
   `toHaveText` on `getByRole('columnheader').first()`: `Expected: "Order",
   Received: "Source"` (the assertion is wired to the real leading header
   and fires against the absent column).
3. **Cell-binding + 1-based offset fire:** temporarily changed
   `{{ index + 1 }}` to `{{ index }}`, rebuilt, ran: `1 failed` -
   `getByTestId('editor-rule-row').first().getByRole('cell').first()`:
   `Expected: "1", Received: "0"`. Restored to `index + 1`, reran: `1 passed`.
4. **Step 3, implementation complete:** `1 passed (566ms)`.

Full frontend gate (this diff touches zero Rust files, so the cargo half of
the nine-part gate - fmt/clippy/test/doc/deny - is unaffected; that half is
the controller's merge-time duty. The four frontend gates a Vue/TS/Fluent
change can influence were run in full):
- `pnpm lint` (eslint .): clean, no output.
- `pnpm build` (vue-tsc --noEmit && vite build): built, 165 modules, no
  type errors.
- `pnpm check:i18n`: `check-i18n: ok (41 source files scanned, 211 catalog
  ids, 17 unused warning(s), 1 other locale(s) checked for parity against 7
  en/ catalog(s))`. Cross-locale parity holds (en/de both carry the new key).
  The new key is NOT among the unused warnings (it is referenced via
  `$t("editor-track-rule-order")` in the template and in the spec); the 17
  warnings are the pre-existing, script-documented IpcError-code false
  positives (gui-common/gui-jobs), unchanged by this task.
- `pnpm test:e2e` (full suite): **52 passed**, including
  `smoke.spec.ts:935 ... a drag-reorder swaps the rows and updates the held
  model`, `editor-markers.spec.ts`, `editor-tooltips.spec.ts`,
  `editor-dropdowns` and all others.

## Surfaced items (patterns / decisions)

1. **New catalog section** `## Rule grid ordinal (D59)` in both gui-editor
   catalogs. Structural-conformance grant, zero outward effect (Fluent id
   resolves regardless of placement; no API/data/verification/user-visible
   change). Placed to keep the attribute-less key out of the all-tooltip
   `## TrackRule` section, matching the file's existing non-registry trailing
   sections. Surfaced because it establishes a new (single-key) section
   rather than extending an existing one; the alternative (inside
   `## TrackRule`) would have broken that section's tooltip uniformity.
2. The catalog header comment (en) enumerates "one labelKey per EditableField
   ... plus the save-surface note (D41)"; it already omits the generic
   list/map actions, so it is descriptive-of-primary-contents, not a strict
   inventory. The new key is not a registry label and "every label message
   carries its tooltip" stays true, so the header comment was left unchanged.
   Flagging in case the controller wants the header to mention the ordinal.
