### Task 9: D45 - the registry data layer, catalogs, and the i18n gate

**Files:**
- Create: `src/editor/fieldSpec.ts` (the `FieldSpec`/`FieldWidget`/`RegistryName` types)
- Create: `src/editor/registries.ts` (the 13 registries + the option arrays + the completeness guards)
- Create: `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl`
- Modify: `scripts/check-i18n.mjs` (the `LABEL_KEY_RE` scan)

`e2e/catalogs.spec.ts` is **not** a deliverable of this task and needs no work: its single test delegates to `assertAllCatalogsParseCleanly`, which globs `locales/<tag>/*.ftl` (`e2e/i18n-en.ts`) and so picks up the new `gui-editor.ftl` automatically with zero change to either file (design `:1728-1731`). It is listed here only to record that the guard already covers the new catalog for free; do not stage it, do not edit it.

**Interfaces:**
- Consumes: Task 5's `src/bindings/profile.ts` and `src/bindings/keywords.ts`.
- Produces, for Tasks 10-13: the 13 exported registries, `FieldSpec`, `FieldWidget`, `RegistryName`, `COLLISION_POLICIES`, `KEEP_DROP`.

**Read first:** design D45 (`:685-1099`) in full. It carries the type definitions (`:694-706`, `:806-819`), the 13-struct table (`:768-782`), the **complete 43-field widget table** (`:848-894`), the option-array guard shape (`:905-909`), and the `check-i18n` fix (`:1082-1091`).

Binding points:
- **All 13 structs get a registry and all 7 enums get a `never` arm.** No subset: the registry's entire value is total coverage, and registering only a subset reintroduces the exact silent-absence failure the mechanism exists to close.
- **42 of 43 fields are `EditableField`; the one `FixedField` is `Profile.profile_version`**, which spec 4 pins at 1. Do **not** reach for `Omit<Profile, "profile_version">` - that silently disables the forcing function for that key forever.
- `FieldWidget` has **10 variants**, closed. `fixed` is **not** one of them: it is the other half of the `FieldSpec` union.
- **The four keyword arrays are imported from `src/bindings/keywords.ts`, never hand-written.** `COLLISION_POLICIES` and `KEEP_DROP` are declared here with the `satisfies` completeness guard, because TS can see their unions; the keyword domains are not in the TS type at all (the untagged enums project to `Block | string`), which is why they are generated instead.
- `gui-editor.ftl` carries exactly **43** keys: 42 labels + 1 save-surface note (Global Constraints ruling 2). **No tooltip budget** (ruling 4: the editor ships without tooltips; spec 8.3's editor baseline is Plan 7).
- Widget facets add **no** keys: `select` and `keywordOrBlock` render their options from the domain arrays, and those are profile-format tokens (`keep`, `drop`, `error`, `primary`), not prose - the same call D39 made for the `allowed` param.
- Three widget choices are settled by evidence, not by the Rust type, and are the ones most likely to be got wrong: `optionalFlag` is a checkbox whose off-state is **absence** (not a tri-state - `validate.rs:466-472` rejects `Some(false)`); `TextSyntax` has **four** values because `Locator.match_pattern` is a template in *regex* mode, a genuinely third thing from `Input.pattern`'s regex and `TemplateBlock.template`'s literal-mode template; `propertyMap.properties` is `matchable | settable` because `exact` and `changes` offer **different** domains (`codec_kind` is matchable-only).

- [ ] **Step 1: Write the types and the registries**

`src/editor/fieldSpec.ts` gets `EditableField`, `FixedField`, `FieldSpec`, `TextSyntax`, `FieldWidget`, `RegistryName` per `:694-706` and `:806-819`. `src/editor/registries.ts` gets the 13 registries, filled from the **43-row table at `:848-894`** - work it row by row; it is complete and it is the contract.

`reorderable` is semantic, not taste: `tracks.rules` is output track order and `attachments.rules` resolves first-match-wins in list order, so both reorder; `any` (logical OR) and `not` (logical NOR) carry no order, so neither does.

- [ ] **Step 2: Prove the registry-completeness proof fires (deliberate break)**

The registry's whole value is that it fails the **build**, not a test. Now that step 1 has created `registries.ts`, prove it:

```bash
# delete one entry from outputFields and run:
pnpm build
# Expected: error TS2741: Property 'on_collision' is missing in type '{...}'
#           but required in type 'Record<keyof OutputCfg, FieldSpec>'
# Restore it and confirm green. Record the observed error in your report.
```
Note: two or more missing keys report **TS2739** listing them all. Same check, two messages; the design does not depend on which fires.

- [ ] **Step 3: Add the option arrays with their completeness guards**

Per `:905-909`, for `COLLISION_POLICIES` and `KEEP_DROP`. This is D45's own `never`-arm principle applied to a value list, so it is the house rule of this ADR rather than a new idea.

- [ ] **Step 4: Write the catalogs**

`locales/en/gui-editor.ftl` and `locales/de/gui-editor.ftl`, 43 keys each: one `labelKey` per `EditableField` plus `editor-save-note`. The note's content is fixed by D41 and must name the **whole** behaviour, not just comments - comments are the smaller half:

```
editor-save-note = Saving rewrites the file from the model: comments, key order and formatting are not preserved, and fields left at their default are not written back.
```

A note naming only comments would understate what the user is about to see and would be read as a defect report the first time someone diffs their profile. Write the de counterpart per the de header's register rules.

- [ ] **Step 5: Close check 1 of the i18n gate**

Per design `:1076-1091`, add a second scanning regex alongside `CALL_RE` in `scripts/check-i18n.mjs`:

```js
const LABEL_KEY_RE = /labelKey:\s*(['"])([^'"]*)\1/g;
```

applied to the same `src/**/*.{vue,ts}` sweep, with every match added to `literalCallIds` and pushed to `missing` when it is not a known catalog id. Match the script's own deliberate line-based approach - it is **not** a Fluent parser (`:102-120`), and this is not the place to make it one.

**Check 2 needs no change** and check 3 is untouched: check 2 already counts a key as used when it appears anywhere in `src/` as a quoted literal, single- or double-quoted (`:191-198`, the test at `:193`), which is precisely the registry's `labelKey: "editor-..."` shape - the same mechanism that already passes `src/jobRowState.ts:44-55`'s identical map-to-Fluent-key pattern. Net effect: registry label keys become **hard-gated**, which is a net gain over today, not a trade.

- [ ] **Step 6: Prove the new scan fires**

```bash
pnpm check:i18n
# Expected: green.
# Now break it deliberately:
#   change one labelKey to "editor-does-not-exist"
pnpm check:i18n
# Expected: FAILS naming editor-does-not-exist. Revert and confirm green.
```

- [ ] **Step 7: Full gate, then commit**

```bash
git add src/editor/fieldSpec.ts src/editor/registries.ts locales/en/gui-editor.ftl locales/de/gui-editor.ftl scripts/check-i18n.mjs
git -c commit.gpgsign=false commit -m "gui: the field registry, its catalogs, and the label-key i18n gate (D45)"
```

---

