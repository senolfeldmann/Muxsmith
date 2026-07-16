# Task 12 report: D45 - the editor view, part b: section composition and widget dispatch, plus the generic action keys and typed value cells

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan6-e`, branch `plan6-e`.
Starting HEAD: `94a7d3d` (Task 12a's `src/bindings/settables.ts` committed).
Final HEAD: `0ba894a`.

## What was implemented

### Step 1: the two generic action keys (owner Ruling 1)

`locales/en/gui-editor.ftl` and `locales/de/gui-editor.ftl` each gained a new
`## Generic list/map actions` section:

```
editor-action-add = Add
editor-action-remove = Remove
```

de: `editor-action-add = Hinzufügen`, `editor-action-remove = Entfernen`.
`editor-attachment-rule-add`/`-drop` were left untouched (they remain the
`attachmentRuleFields.add`/`.drop` registry labels).

### Step 2/3: failing assertions, then RED

Extended `e2e/smoke.spec.ts`:
- Rewrote the Task-10 describe-block comment (previously claiming "43 keys" /
  reuse of `editor-attachment-rule-add`/`-drop`) to describe the 45-key
  budget and the two dedicated action keys.
- Repointed the two existing Task-10 mount specs (`propertyMap widget:
  add/remove rows...`, `list widget: add/remove nested items...`) from
  `name("editor-attachment-rule-add"/"-drop")` to
  `name("editor-action-add"/"-remove")`.
- Added a new `test.describe("editor view: section composition and typed
  value cells (Task 12, D45)")` block with three tests: full-profile
  section composition, the settable Boolean/String anti-vacuity round trip,
  and the matchable Boolean/Float/Integer anti-vacuity round trip.

RED run (`pnpm test:e2e`), 5 failed for the named reasons:

```
1) propertyMap widget: add/remove rows edit a key-value map
   waiting for getByRole('button', { name: 'Remove', exact: true })  -- still renders "Drop"
2) list widget: add/remove nested items (matchExpr.any, item: matchExpr)
   waiting for getByRole('button', { name: 'Remove', exact: true })  -- still renders "Drop"
3) EditorView composes every profile section...
   waiting for getByRole('group', { name: 'Metadata', exact: true })  -- EditorView mounts but is uncomposed
4) propertyMap typed value cells: the settable Boolean/String anti-vacuity round trip
   Expect "not toBeChecked": element(s) not found  -- no checkbox exists; forced_track is still a textbox
5) propertyMap typed value cells: the matchable Boolean/Float/Integer anti-vacuity round trip
   Expect "not toBeChecked": element(s) not found  -- same, matchable side
```
18 passed (untouched Task 9-11 specs), confirming isolation.

### Step 4: re-point captions, add typed value cells

- `src/editor/widgets/ListWidget.vue`: add/remove buttons now call
  `$t("editor-action-add")`/`$t("editor-action-remove")`; doc comment
  rewritten (drops the stale "43 keys" claim, explains why
  `attachments.rules` renders through this generic widget while
  `tracks.rules` does not).
- `src/editor/widgets/PropertyMapWidget.vue`: same repoint, plus the typed
  value cells:
  - Imports `SETTABLE_TYPES`, `MATCHABLE_TYPES`, `PropScalarType` from
    `../../bindings/settables` (Task 12a) and `Scalar` from
    `../../bindings/profile`.
  - `model` widened from `Record<string, string> | null` to
    `Record<string, Scalar> | null`.
  - New `cellKindFor(key): "checkbox" | "integer" | "float" | "text"`:
    returns `"text"` immediately when `spec.widget.values !== "scalar"`
    (the closed substring/regex boundary), returns `"text"` when the key
    is in neither `SETTABLE_TYPES` nor `MATCHABLE_TYPES` (unknown-property
    fallback), otherwise switches exhaustively over `PropScalarType` with a
    `const _exhaustive: never = scalarType` default arm (mirrors
    `FieldWidgetDispatcher.vue`'s `widgetComponentFor` shape).
  - Template: `v-if`/`v-else-if` chain over `cellKindFor(key)` renders
    `<input type="checkbox">` (real boolean via `.checked`), `<input
    type="number">` (integer), `<input type="number" step="any">` (float,
    the one new input variant), or `<input type="text">` (string/fallback)
    - all sharing `data-testid="property-map-value"`.

### Step 5: EditorView.vue section composition

- Iterates `profileFields` (Task 9's registry) via a new `topLevelFields`
  computed, filtering out `profile_version` (`FixedField`, same convention
  `SectionWidget.vue` uses) and `tracks` (the one hand-built exception).
  Each remaining `EditableField` dispatches through
  `FieldWidgetDispatcher` - `meta`, `input`, `output`, `attachments`,
  `tags` render as `section`-kind fieldsets (recursing into their own
  registries, so `attachments.rules` renders through the generic
  `ListWidget`); `chapters`/`title` render as `keywordOrBlock`.
- `tracks` is special-cased: `tracksFields.unmatched` dispatches
  generically (a new `tracksUnmatchedSpec` constant, cast to
  `EditableField` since a single `Record<keyof TracksCfg, FieldSpec>`
  property access does not narrow away `FixedField` on its own); Task 11's
  bespoke rule grid (table, `data-testid="editor-rule-row"`, drag-reorder)
  is kept byte-for-byte, now wrapped in a `<fieldset><legend>{{ $t(
  "editor-profile-tracks") }}</legend>` alongside the `unmatched` select.
- Task 11's own rule-grid test (`editor view: rule grid + drag-reorder`) is
  untouched and still green (verified below) - the grid's markup did not
  change, only its wrapping container.

### Step 6/7: GREEN, then the full gate, then commit

All 23 e2e tests pass (`pnpm test:e2e`); `pnpm build`, `pnpm lint`,
`pnpm check:i18n` all green. Full nine-part gate run foreground (below).
Commit `0ba894a`, unsigned, `Co-Authored-By: Claude Sonnet
<noreply@anthropic.com>` trailer, explicit file staging (no `git add -A`).

## The 45-key counts

```
$ grep -cE "^[A-Za-z][A-Za-z0-9_-]*\s*=" locales/en/gui-editor.ftl
45
$ grep -cE "^[A-Za-z][A-Za-z0-9_-]*\s*=" locales/de/gui-editor.ftl
45
```
Both catalogs: 42 field labels + 1 save-surface note (`editor-save-note`) +
2 generic action keys (`editor-action-add`/`editor-action-remove`) = 45.

## GREEN run (final)

```
Running 23 tests using 16 workers
...
23 passed (1.2s)
```
Includes the untouched `editor view: rule grid + drag-reorder (Task 11,
D45)` test, still passing unmodified.

## Gate results (nine parts, all run foreground from the worktree root)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | pass (exit 0) |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass (Finished, no warnings) |
| 3 | `cargo test --workspace` | pass - every `test result: ok` line shows `0 failed` across all 39 test binaries (grepped) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | pass (Finished, generated docs, no warnings) |
| 5 | `cargo deny check` | pass - "advisories ok, bans ok, licenses ok, sources ok" (exit 0, verified directly) |
| 6 | `pnpm lint` | pass (no output = clean) |
| 7 | `pnpm build` | pass (vue-tsc + vite build clean) |
| 8 | `pnpm check:i18n` | pass - "ok (35 source files scanned, 231 catalog ids, 18 unused warning(s), 1 other locale(s) checked for parity...)"; the 18 unused-warning ids are pre-existing (IPC-error-code keys and `editor-save-note`, reached only at runtime / in Task 13), none new |
| 9 | `pnpm test:e2e` | pass - 23/23 |

No Rust files were touched in this task (Task 12a already landed the
binding), so parts 1-5 are a clean re-verification of the inherited state,
run in full per the "no subsets" instruction rather than skipped.

## Files changed

```
e2e/smoke.spec.ts                        | 158 ++++++++++++++++++++++++++--
locales/de/gui-editor.ftl                |   5 +
locales/en/gui-editor.ftl                |   5 +
src/editor/widgets/ListWidget.vue        |  30 +++---
src/editor/widgets/PropertyMapWidget.vue | 142 +++++++++++++++++++++-----
src/views/EditorView.vue                 | 170 ++++++++++++++++++++++---------
6 files changed, 408 insertions(+), 102 deletions(-)
```
Matches the brief's Files list exactly; `git status` before staging showed
no other modified files.

## Self-review

1. **13 registries drive the sections (no hand-listing).** `EditorView.vue`
   iterates `Object.entries(profileFields)` and dispatches every
   `EditableField` generically via `FieldWidgetDispatcher`, with one
   documented, structural exception (`tracks`, see Concerns). Adding a new
   field to `Profile` + its registry entry surfaces it in the composed view
   with no `EditorView.vue` edit, for every field except `tracks.rules`.
2. **45/45 keys both locales**, verified by direct `grep -c` count above
   (not inferred from `check:i18n`'s summary line).
3. **Typed switch total over the four tags**, with unknown-name text
   fallback. `cellKindFor` in `PropertyMapWidget.vue` switches
   exhaustively over `PropScalarType` (`boolean`/`integer`/`float`/
   `string`) with a `const _exhaustive: never` default arm; a property
   absent from both `SETTABLE_TYPES` and `MATCHABLE_TYPES` (or a
   `values: "string"` map) returns `"text"` before the switch is ever
   reached.
4. **Anti-vacuity assertions in place and green**: settable Boolean
   (`forced_track`) round-trips a real `true` (`=== true`, not `"true"`);
   matchable Boolean (`forced_track`) likewise; matchable Float
   (`min_luminance`, 1 -> 1.5) round-trips a real `number`. An Integer
   round trip (`audio_channels`, matchable) was added too (brief marks it
   optional, "cheap" - it directly exercises the switch's `integer` arm,
   which is otherwise only reachable structurally, never asserted).
5. **No dropdowns built.** `type` and `codec_kind` are both `String`-typed
   in `MATCHABLE_TYPES`, so they resolve to the `string` arm of
   `cellKindFor` (plain text cell) like every other string property; no
   `<select>` was added for either. Grepped: no new `SelectWidget` usage,
   no new closed-domain option array.

## Concerns

**The "tracks" composition is the one field EditorView.vue special-cases,
rather than iterating `profileFields` with zero exceptions.** The brief's
"Produces" line says EditorView "renders every field of the loaded profile
through the right widget," and its own Step 5 says "do not hand-list
fields" - taken fully literally, that could be read as requiring
`tracks.rules` to render through the generic `list` widget like
`attachments.rules` does. I did not do that, for a concrete, verifiable
reason: `ListWidget.vue`'s own pre-existing doc comment (predating this
task) explicitly names the generic widget as NOT the spec 8.2 top-level
rule grid, and Task 11's rule-grid test (`data-testid="editor-rule-row"`,
`toContainText("video"/"audio")` on the summary cells) is not in this
task's brief's list of specs to update - meaning it is a protected,
unmodifiable assertion under the structural-conformance grant. Routing
`tracks.rules` through the generic `ListWidget` would replace the
plain-text summary cells with live sub-widgets (a `<select>`'s
`textContent` includes every `<option>`, but an `<input>`'s current value
is never part of `textContent`), which would break that protected test
outright. I resolved this by treating `tracks` as the one documented
exception (matching the asymmetry `ListWidget.vue` and the design doc
(D45's own opening line, "components stay hand-built... the registry is a
forcing function, not the type") already establish for `tracks.rules`
specifically), rather than opening a NEEDS_CONTEXT round-trip - the
brief's own concrete Step 2 test enumeration (composition + `optionalFlag`
-> checkbox + `select` -> combobox) does not require `tracks.rules` to be
generically dispatched, and both are satisfied without it. Flagging this
explicitly since it is the one place this task's implementation diverges
from a fully literal reading of "every field, no exceptions" - happy to
revisit if the controller reads it differently.

**No per-rule "detail editor" was built.** Spec 8.2 names a "detail editor
per rule" as a distinct element from the rule grid; no task in the
approved Plan 6 task list (1-14) is titled or scoped for it, Task 12's
brief never mentions it in its binding points or its four enumerated test
assertions, and its Files list (declared exhaustive) has no new component
file for it. `TrackRule`'s own four fields (`source`/`match`/`optional`/
`changes`) therefore stay reachable only as read-only summaries in the
grid, never through a live widget - the same gap that already existed
after Task 11 and is unmentioned as something Task 12 must close. Read the
brief's concrete TDD steps as authoritative over the one-line "Produces"
summary and did not build this; noting it since a genuinely editable
profile arguably wants it eventually (perhaps a controller-level tracking
item, not silently absorbed into a task that never asked for it).
