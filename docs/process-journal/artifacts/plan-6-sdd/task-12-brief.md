### Task 12a: the settable-property-type binding (new, 2026-07-16 owner-rulings routing)

**Stream E** (`.worktrees/plan6-e`), sequenced after Task 11 and before Task 12. It is the vehicle for owner Ruling 2's type source (Amendment 2): the typed value cells need a settable-property -> scalar-type map on the frontend, and hand-mirroring `capability::SETTABLE` in TypeScript is banned (the exact D44/D46 drift class). Per this plan's trigger 4 ("a second Muxsmith artifact needs TypeScript types -> extend the `ts` feature's export set rather than hand-mirroring again"), Task 5's ts-rs emitter is extended to emit one more committed binding. It is a separate step from Task 12 because it is a core-crate + generated-artifact + CI-drift concern (Task 5's layer, not the frontend's), and its output must be **tracked before** Task 12 imports it (`git-diff-proof-needs-tracked-target`, and the TypeScript build resolves the import only once the file exists).

**Files:**
- Modify: `crates/muxsmith-core/src/capability/mod.rs` (a documented public `matchable_properties()` accessor over the private `generated` table)
- Modify: `crates/muxsmith-core/tests/ts_export.rs` (a second value emitter beside `emit_keywords_ts`, emitting both property-type tables)
- Create (committed, generated): `src/bindings/settables.ts`

**Interfaces:**
- Consumes: `muxsmith_core::capability::{SETTABLE, PropType, matchable_type}` (`pub`, `crates/muxsmith-core/src/capability/mod.rs:22,72,40`) and the new `matchable_properties()` accessor this task adds.
- Produces, for Task 12: `src/bindings/settables.ts` exporting `SETTABLE_TYPES` (settable-name -> scalar-type-tag), `MATCHABLE_TYPES` (matchable-name -> scalar-type-tag), and the shared `PropScalarType` union type - one committed binding file carrying both tables (owner ruling: one file total).

**Read first:** design D44 (`:498-682`) for the emitter mechanism and the committed-binding + CI-drift contract; Task 5's `emit_keywords_ts` (`crates/muxsmith-core/tests/ts_export.rs`) for the exact emit-a-value-artifact shape this extends (ts-rs exports types, not values, so a value map needs a hand-written emitter); the house rule `git-diff-proof-needs-tracked-target` (`docs/decision-ledger.yaml`) for Step 3's staging precondition.

Binding points:
- **The type sources are `capability::SETTABLE` and the matchable table; hand-mirroring either list in TypeScript is banned** (the exact D44/D46 drift class that D46's generated keyword arrays and D44's generated bindings exist to prevent). `SETTABLE` is `&[(&str, PropType, &str)]` (10 rows). The matchable table is `generated::MATCHABLE_PROPERTIES` (`&[(&str, PropType)]`, 62 rows) **plus** the `codec_kind` virtual (`matchable_type("codec_kind") == String`, `mod.rs:41-43`) = **63** matchable props.
- **`generated` is a private module** (`mod.rs:9`, no `pub`), so `MATCHABLE_PROPERTIES` is not reachable from a `tests/` binary. This task adds a small documented public accessor `pub fn matchable_properties() -> &'static [(&'static str, PropType)] { generated::MATCHABLE_PROPERTIES }` to `capability/mod.rs`, keeping `generated` private while exposing the table - consistent with `SETTABLE`/`ATTACHMENT_PROPERTIES` already being `pub` and with the `settable()`/`matchable_type()` accessor style. The emitter reads it and appends `("codec_kind", matchable_type("codec_kind"))` so the virtual's tag is derived from its source arm, never re-asserted (the can-never-drift house principle).
- **`PropType -> tag` is an exhaustive `match`, so a new `PropType` variant fails the Rust build** (the house never-arm at the Rust layer, the CODEC-derivation shape D46 uses): `String -> "string"`, `Boolean -> "boolean"`, `Integer -> "integer"`, `Float -> "float"`. All four tags occur in `MATCHABLE_TYPES` (63 props: 22 String incl `codec_kind`, 9 Boolean, 27 Integer, 5 Float, tree-verified); `SETTABLE_TYPES` holds only `String`/`Boolean` today. The shared `PropScalarType` union names all four, so Task 12's widget switch is total over `PropType`'s whole domain (a later Integer settable, or a schema-regenerated new type, needs no widget change).
- **No `.github/workflows/ci.yml` change.** The Task-5 drift step (`ci.yml:134-138`) already runs `cargo test -p muxsmith-core --features ts` then `git diff --exit-code src/bindings/`, which covers `settables.ts` the moment it is committed. The untracked-first-generation hole D44 records still applies until then; Step 3 closes it in-task by staging before proving, and the commit closes it for CI.

- [ ] **Step 1: Add the accessor, then extend the emitter**

First, in `crates/muxsmith-core/src/capability/mod.rs`, add the documented public accessor `matchable_properties()` (binding point above) beside `matchable_type`; `generated` stays private. Then in `crates/muxsmith-core/tests/ts_export.rs`, add `use muxsmith_core::capability::{PropType, SETTABLE, matchable_type, matchable_properties};`, a private `fn scalar_tag(t: PropType) -> &'static str` with the exhaustive four-arm match above, and a second `#[test] fn emit_settables_ts()` mirroring `emit_keywords_ts`'s structure: read `TS_RS_EXPORT_DIR`, write a `// @generated by ...` header, then `export type PropScalarType = "boolean" | "integer" | "float" | "string";`, then two `export const ... = {` ... `} as const satisfies Record<string, PropScalarType>;` blocks - `SETTABLE_TYPES` (one `"<name>": "<scalar_tag>",` line per `SETTABLE` triple) and `MATCHABLE_TYPES` (one line per `matchable_properties()` row in its declaration order, then a final `"codec_kind": "<scalar_tag(matchable_type("codec_kind"))>"` line). Quote the keys, so a non-identifier property name stays valid TS. Write to `Path::new(&dir).join("settables.ts")` (one file, both tables).

- [ ] **Step 2: Generate and inspect (the first-generation check)**

```bash
cargo test -p muxsmith-core --features ts
ls -1 src/bindings/
```
Expected: `keywords.ts`, `profile.ts`, `settables.ts`. Open `settables.ts` and confirm from its content (numbers tree-verified against `generated.rs`): `SETTABLE_TYPES` has **10** rows (one per `SETTABLE`), with `forced_track`/`default_track` -> `"boolean"` and `language`/`track_name`/`sub_charset` -> `"string"`; `MATCHABLE_TYPES` has **63** rows - 22 `"string"` (incl. `codec_kind`), 9 `"boolean"`, 27 `"integer"`, 5 `"float"` - with the spot-checks `codec_kind` -> `"string"`, `forced_track`/`default_track` -> `"boolean"`, `audio_channels` -> `"integer"`, `min_luminance`/`max_luminance` -> `"float"`; and `PropScalarType` names all four tags. Because nothing imports the file until Task 12, this inspection **is** the first-generation correctness check (the CI drift step cannot see an untracked file - D44's recorded hole).

For the type-count spot-check you can cross-check against the tree directly: `for t in String Boolean Integer Float; do printf "%s " "$t"; grep -cE "PropType::$t\b" crates/muxsmith-core/src/capability/generated.rs; done` yields 21/9/27/5 (String is 22 in `MATCHABLE_TYPES` after the `codec_kind` virtual is appended).

- [ ] **Step 3: Stage, then prove the drift check catches drift**

Stage first: the proof is a `git diff --exit-code`, and against an **untracked** path it exits 0 regardless of content, silently inverting the expected red (`git-diff-proof-needs-tracked-target`).

```bash
git add src/bindings/settables.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"     # Expected: exit=0 (index == working tree)
printf '\n// drift\n' >> src/bindings/settables.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"     # Expected: exit=1 (the gate fires)
git checkout -- src/bindings/settables.ts
cargo test -p muxsmith-core --features ts               # regenerate a clean tree
git diff --exit-code src/bindings/ ; echo "exit=$?"     # Expected: exit=0
```
Read the fire from the OUTPUT (`exit=1` on the appended file), not from its absence, and record it in the report.

- [ ] **Step 4: Full gate, then commit**

Run the nine-part gate (the default `cargo test --workspace` does not run the `ts` feature; `pnpm build` typechecks the new `settables.ts` standalone even though Task 12 is its first importer). Then:

```bash
git add crates/muxsmith-core/src/capability/mod.rs crates/muxsmith-core/tests/ts_export.rs src/bindings/settables.ts
git -c commit.gpgsign=false commit -m "core: emit the settable + matchable property-type bindings for the editor's typed value cells (D44 trigger 4, owner rulings 2026-07-16)"
```

---

### Task 12: D45 - the editor view, part b: section composition and widget dispatch, plus the generic action keys and typed value cells (amended 2026-07-16, owner-rulings routing)

**Files:**
- Modify: `src/views/EditorView.vue` (section composition + widget dispatch over the 13 registries)
- Modify (amended 2026-07-16, owner-rulings routing): `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl` (the 2 generic action keys), `src/editor/widgets/ListWidget.vue` and `src/editor/widgets/PropertyMapWidget.vue` (re-point to the action keys; PropertyMapWidget also gains the typed value cells)
- Test (amended 2026-07-16, owner-rulings routing): `e2e/smoke.spec.ts` (extend, and update the Task-10 `propertyMap`/`list` mount specs)

**Interfaces:**
- Consumes: Task 9's 13 registries, Task 10's widget dispatcher, Task 11's `EditorView.vue` scaffold, and (amended) **Task 12a's `src/bindings/settables.ts`** (`SETTABLE_TYPES`, `MATCHABLE_TYPES`, `PropScalarType`).
- Produces, for Task 13: an `EditorView.vue` that renders every field of the loaded profile through the right widget; `gui-editor.ftl` at its final **45** keys.

Binding points:
- **The section composition is driven by the 13 registries, not by hand-listed fields.** Each registry maps `keyof <Cfg>` to a `FieldSpec`; the view iterates the registry and dispatches Task 10's widget for each field's `FieldWidget`. Adding a field to the model + registry surfaces it here with no view edit - that is the registry's whole point.
- **The frontend performs zero semantic validation** (spec 7). It renders the field, holds the value, and (in Task 13) sends the model. No per-field validity logic here. (The typed value cells below are a rendering affordance keyed on the property's declared type, not semantic validation: core still validates every model, and the save-disable affordance still gates the run path.)
- The `FixedField` (`Profile.profile_version`) renders read-only; it has no `labelKey` and no widget (`FieldWidget` has no `fixed` variant).
- **Generic action keys (owner Ruling 1, amended 2026-07-16).** `gui-editor.ftl` gains `editor-action-add` (en "Add", de "Hinzufügen") and `editor-action-remove` (en "Remove", de "Entfernen"); `ListWidget.vue` and `PropertyMapWidget.vue` re-point their generic add/remove-row buttons from `editor-attachment-rule-add`/`-drop` to these two. `editor-attachment-rule-add`/`-drop` stay in the catalog and stay used - they remain the `attachmentRuleFields.add`/`.drop` registry labels (`src/editor/registries.ts:185-189`), so `check:i18n` keeps counting them - but they now caption **only** the AttachmentRule fields. Budget: **45** (42 labels + 1 note + 2 action keys). The two widgets' and `smoke.spec.ts`'s doc comments that assert "gui-editor.ftl stays at 43 keys" via this reuse are now false and are rewritten.
- **Typed value cells on both scalar property maps (owner Ruling 2, amended 2026-07-16).** A `PropertyMapWidget.vue` value cell is type-appropriate whenever `spec.widget.values === "scalar"`, for **both** the settable `changes` map and the matchable `exact` map. The property's scalar type comes from the map selected by the `properties` facet: `properties === "settable"` -> `SETTABLE_TYPES`, `properties === "matchable"` -> `MATCHABLE_TYPES` (both from Task 12a's `settables.ts`). One internal switch over `PropScalarType` renders the tag: `boolean -> <input type="checkbox">` (round-trips a real `true`/`false`); `integer -> <input type="number">` (integer step); `float -> <input type="number" step="any">` - the **one new input variant**, a number input that accepts decimals, enumerated **inside** this switch, **not** as a new `FieldWidget` variant; `string -> <input type="text">`. The switch is exhaustive over `PropScalarType` with the house `const _exhaustive: never = t` arm (the same shape `FieldWidgetDispatcher.vue:56` uses); a property name in neither map falls back to the text cell (core catches an unknown property anyway). The widget `model` widens from `Record<string, string>` to `Record<string, Scalar>` (from `../../bindings/profile`) - a Boolean property must reload as `true`, not `"true"` (`gui-typed-scalar-needs-typed-input`).
- **`values: "string"` maps stay text cells, as a closed boundary.** `matchExpr.substring` and `matchExpr.regex` are `values: "string"` (`src/editor/registries.ts:202-209`): their target `MatchExpr.substring`/`regex` hold a `String` **by definition** (a substring or a regex pattern is text, never a Boolean/number), so their cells are always `<input type="text">` regardless of the property named. This is not the typed path deciding to skip them; it is that a string-pattern cell has no scalar type to look up. Enumerated on the `values` facet, not a fork.
- **The curated closed-domain DROPDOWNS for `type` and `codec_kind` are Plan 7, by owner ruling.** `gui-closed-domain-dropdowns` (`docs/product-boundaries.yaml`) decrees a selection control wherever a value has a closed set; the two curated matchable domains `type` (4 values) and `codec_kind` (17 aliases) get their dropdowns in **Plan 7** with the editor comfort layer (the ROADMAP already carries the item). In Plan 6 both are `String`-typed (`type` is String; `codec_kind` is the String virtual), so this amendment renders them as the `string` -> text cell above; the dropdown upgrade is the owner's recorded Plan-7 decision, not a presumed follow-up. `language` and other open/runtime-domain values stay free-entry with core validation.

- [ ] **Step 1: Add the two generic action keys to both catalogs (amended 2026-07-16, owner-rulings routing)**

Add to `locales/en/gui-editor.ftl` and `locales/de/gui-editor.ftl`, under a new `## Generic list/map actions` section:

```
editor-action-add = Add
editor-action-remove = Remove
```

de: `editor-action-add = Hinzufügen`, `editor-action-remove = Entfernen` (owner-mandated strings; they are infinitives / nominalized verb forms, the same button-label form the de catalog already uses at `editor-attachment-rule-add = Hinzufügen`, not an imperative; straight ASCII quotes). Leave `editor-attachment-rule-add`/`-drop` in place: they remain the AttachmentRule field labels. Catalog is now **45** keys. Do **not** run `pnpm check:i18n` yet - the two keys are unused until Step 4 re-points the widgets, and check 2 would flag them; the i18n gate runs at Step 6 once they are used.

- [ ] **Step 2: Write the failing assertions - composition, action captions, and typed cells (amended 2026-07-16, owner-rulings routing)**

Extend `e2e/smoke.spec.ts` and **update the Task-10 mount specs** (`smoke.spec.ts:724-756`, the `propertyMap` and `list` specs, plus their describe-block comment at `:589-606` asserting "43 keys"), through the Task-10 harness (`e2e/mount.ts`), not the served app:

1. **Section composition** (the original Task 12 assertion): `mountComponent(page, { component: "EditorView", props: { modelValue: <full profile> } })`, asserting each section (input, tracks, output, attachments, tags, and the rest) renders with its fields dispatched to the expected widget types (`optionalFlag` -> checkbox, `select` -> combobox of its domain tokens).
2. **Action captions** (Ruling 1): the generic `list` and `propertyMap` widgets' **remove** button is `name("editor-action-remove")` ("Remove"), not the old `editor-attachment-rule-drop` ("Drop"); the **add** button is `name("editor-action-add")`. The falsifiable observable is the remove caption: en "Drop" -> "Remove" (the add caption is textually "Add" under both keys, so a text assertion on it is vacuous and the add re-point is verified by the widget source referencing `editor-action-add`, per Step 4). Repoint the two existing specs' `name("editor-attachment-rule-add")`/`-drop` lookups accordingly.
3. **Typed value cell - settable map** (Ruling 2, the anti-vacuity assertion): mount `PropertyMapWidget` with `{ spec: trackRuleFields.changes, modelValue: { forced_track: false } }` (a real Boolean settable). Assert the `forced_track` value cell is a **checkbox** (`getByRole("checkbox")`), check it, and assert `readModel(page)` returns `{ forced_track: true }` with `true` a real boolean (`=== true`), not the string `"true"`. Add a String-settable row (`{ language: "eng" }`) and assert its value cell is a **textbox** that round-trips the string.
4. **Typed value cell - matchable map** (Ruling 2, the owner's matchable extension): mount `PropertyMapWidget` with `{ spec: matchExprFields.exact, modelValue: { forced_track: false } }` (a real Boolean **matchable** property, `matchable_type("forced_track") == Boolean`). Assert a **checkbox** that round-trips an actual `true` (`=== true`) - the matchable-Boolean anti-vacuity case. Then the **Float** case: mount with `{ spec: matchExprFields.exact, modelValue: { min_luminance: 1 } }` (`min_luminance` is `Float`), assert the value cell is a **number input** (`getByRole("spinbutton")`, `step="any"`), set it to `1.5`, and assert `readModel(page)` returns `{ min_luminance: 1.5 }` with `1.5` an actual number, not `"1.5"`. (Boolean and Float are the driven fixtures the ruling names; the `integer` arm is the same `<input type="number">` branch minus `step="any"` and rides the exhaustive `never`-arm switch rather than its own fixture. Add an `audio_channels` Integer round-trip too if cheap, but it is not required.)

- [ ] **Step 3: Run to confirm they fail (amended 2026-07-16, owner-rulings routing)**

```bash
pnpm test:e2e
```
Expected: FAIL, and name each: the composition assertions find no composed sections (`EditorView` mounts but is uncomposed); the remove button `name("editor-action-remove")` matches no button (the widgets still render "Drop" from `editor-attachment-rule-drop`); in **both** the settable (`trackRuleFields.changes`) and matchable (`matchExprFields.exact`) mounts the `forced_track` cell is a `textbox`, not a `checkbox`, and `readModel` returns the string `"true"`, not the boolean `true`; the `min_luminance` matchable cell is a `textbox`, not a `spinbutton`. All are genuine RED (the widgets and composition are unchanged from Tasks 10/11).

- [ ] **Step 4: Re-point the action captions and add the typed value cells (amended 2026-07-16, owner-rulings routing)**

In `src/editor/widgets/ListWidget.vue` and `src/editor/widgets/PropertyMapWidget.vue`, change the generic add/remove-row buttons' `$t("editor-attachment-rule-add")`/`$t("editor-attachment-rule-drop")` to `$t("editor-action-add")`/`$t("editor-action-remove")`, and rewrite the doc comments that justified the old reuse and the "43 keys" claim (now Ruling 1 / 45 keys). In `PropertyMapWidget.vue`: import `SETTABLE_TYPES`, `MATCHABLE_TYPES` and `PropScalarType` from `../../bindings/settables` and `Scalar` from `../../bindings/profile`; widen `model` to `Record<string, Scalar> | null`; and when `spec.widget.values === "scalar"`, render the value cell by the row property's type - looked up in `SETTABLE_TYPES` when `spec.widget.properties === "settable"`, else `MATCHABLE_TYPES` - through one internal switch over `PropScalarType`: `boolean` -> checkbox (real boolean), `integer` -> `<input type="number">`, `float` -> `<input type="number" step="any">` (the new decimal-accepting variant, inside this switch, not a new `FieldWidget` kind), `string` -> text input; a `const _exhaustive: never = t` default and a text-cell fallback for a property in neither map. When `spec.widget.values === "string"` (substring/regex), keep the text cell unconditionally (String-by-definition, closed-boundary binding point above). Match the house component conventions already in the two files.

- [ ] **Step 5: Implement section composition and widget dispatch (amended 2026-07-16, owner-rulings routing)**

Drive the sections from the 13 registries and dispatch each field through Task 10's widget dispatcher. Do not hand-list fields.

- [ ] **Step 6: Run the suite (amended 2026-07-16, owner-rulings routing)**

```bash
pnpm build && pnpm lint && pnpm check:i18n && pnpm test:e2e
```
Expected: PASS - the composed sections, the "Remove"/"Add" captions, and the typed cells render green; `check:i18n` is green because both new keys are now used by the widgets and `editor-attachment-rule-add`/`-drop` are still used by the registry.

- [ ] **Step 7: Full gate, then commit (amended 2026-07-16, owner-rulings routing)**

```bash
git add src/views/EditorView.vue src/editor/widgets/ListWidget.vue src/editor/widgets/PropertyMapWidget.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: editor section composition and widget dispatch, the generic action keys, and the settable + matchable typed value cells (D45, owner rulings 2026-07-16)"
```

---

