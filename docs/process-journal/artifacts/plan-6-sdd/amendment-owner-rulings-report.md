# Plan 6 - second mid-run amendment: owner rulings routing (2026-07-16)

Second mid-run amendment to `docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`.
Folds two owner rulings (Şenol, 2026-07-16) into Task 12 and their ripples into the plan.
Working-tree edit only (no commit), single file changed.

## Type-map mechanism chosen (with tree evidence)

**Extend Task 5's ts-rs emitter (`crates/muxsmith-core/tests/ts_export.rs`) to emit a
second committed value binding `src/bindings/settables.ts` from `capability::SETTABLE`**,
carried by a **new Task 12a** placed before Task 12; Task 12's `PropertyMapWidget`
consumes it.

- The authoritative source is `capability::SETTABLE: &[(&str, PropType, &str)]`
  (`crates/muxsmith-core/src/capability/mod.rs:72-91`), 10 entries, each carrying a
  `PropType` (`mod.rs:22-32`: `String | Boolean | Integer | Float`). Today all 10 are
  `String`/`Boolean`; no `Integer`/`Float` settable exists.
- `capability` is `pub` (`crates/muxsmith-core/src/lib.rs:8`), `SETTABLE`/`PropType`
  are `pub`, so a `tests/` integration test reaches them (the emitter already imports
  `muxsmith_core::profile::model::*` the same way).
- Emitter shape mirrors the existing `emit_keywords_ts` (`ts_export.rs:18-36`): a
  hand-written value emitter, because ts-rs exports **types, not values**. It emits
  `export type SettableScalarType = "boolean"|"integer"|"float"|"string";` plus
  `export const SETTABLE_TYPES = {...} as const satisfies Record<string, SettableScalarType>;`.
  The `PropType -> tag` map is an exhaustive Rust `match` (a 5th `PropType` variant
  fails the build - the house never-arm at the Rust layer, same shape as
  `CODEC_KIND_NAMES` derivation at `mod.rs:125-129`).
- Hand-mirroring the settable list in TS is the banned D44/D46 drift class; trigger 4
  ("a second Muxsmith artifact needs TypeScript types -> extend the `ts` feature's
  export set") routes it to the emitter. The CI drift step already globs
  `src/bindings/` (`.github/workflows/ci.yml:134-138`), so **no `ci.yml` change** - it
  covers `settables.ts` once committed.
- `PropertyMapWidget` (settable path only) switches on the row property's
  `SETTABLE_TYPES` tag: `boolean -> checkbox` (real `true`/`false`), `integer`/`float`
  -> number input, `string` -> text; exhaustive `const _exhaustive: never = t` arm
  (same as `FieldWidgetDispatcher.vue:56`), text fallback for an unknown property.
  `model` widens `Record<string, string>` -> `Record<string, Scalar>`.

**Why Task 12a and not folded into Task 12's commit:** the binding is a core-crate +
generated-artifact + CI-drift concern (Task 5's layer), it must be **tracked before**
Task 12 imports it (the TS build resolves the import only once the file exists, and
`git-diff-proof-needs-tracked-target` requires the drift target staged), and it mirrors
Task 5's isolated-generation precedent. The prompt allows "a Task-12-adjacent new step
where strictly necessary"; the layer boundary + ordering dependency + separate proof
obligation make it so. Task 12 stays a frontend task consuming an existing binding.

## Scope decision: settable only, matchable surfaced

`PropertyMapWidget` renders **both** the settable `changes` map
(`trackRuleFields.changes`, `properties: "settable"`, `registries.ts:149-152`) **and**
the matchable `exact` map (`matchExprFields.exact`, `properties: "matchable"`,
`values: "scalar"`, `registries.ts:198-201`). The matchable-scalar `exact` cell has the
**identical** `Scalar::Str`-where-Boolean gap. Owner Ruling 2
(`gui-typed-scalar-needs-typed-input`) is scoped to **settable** property type, and the
T10 review traced the `changes`/settable path only. Decision: type the settable cells
(keyed on `properties === "settable"`), keep matchable cells as text (unchanged), and
**surface** the matchable gap explicitly in a Task 12 binding point + this report rather
than silently resolving it (its type source would be the far larger generated
`matchable_type` table, out of the ruling's `SETTABLE`-scoped vehicle). This is an
enumerated branch on a known facet, not an open fork.

## Plan hunks (step, before -> after gist)

1. **Global Constraints ruling 2 (`:18`)** - budget statement. `carries **43** keys:
   42 labels + 1 note` -> `carries **45** keys: 42 labels + 1 note + 2 generic action
   keys`, with the `43 -> 45` revision, the `editor-generic-action-keys` ruling
   reference, and the design `:1736`/`:1749` supersession note.
2. **Global Constraints ruling 4 (`:20`)** - `stays exactly 43 keys` -> `stays exactly
   45 keys`; added that the 2 action buttons' Plan-7 tooltip treatment is out of scope
   (the `42 tooltip keys` count is untouched).
3. **Amendment 2 block (new, `:37`)** - second amendment subsection (4 sentences):
   states Ruling 1 (generic action keys, exact en/de strings, budget 43->45), Ruling 2
   (typed value cells, the `ValueTypeMismatch` reason), their vehicles (Task 12a +
   Task 12), and the one-sentence design-doc-supersession note the prompt asked for.
4. **Task 9 binding point (`:1036`)** - `carries exactly **43** keys ... (ruling 2)`
   -> keeps 43 as Task 9's as-built count + annotates final budget 45 (the 2 action
   keys land in Task 12, are not `EditableField` labels, so 42-label count is untouched).
5. **Task 9 Step 4 (`:1065`)** - `43 keys each: one labelKey per EditableField plus
   editor-save-note` kept (arithmetically 42+1); appended a note that the 2 action keys
   arrive in Task 12.
6. **Task 12a (new task, `:1287-1340`)** - the settable-type binding: Files, Interfaces,
   Binding points (SETTABLE source, exhaustive PropType match, no ci.yml change), and
   4 steps (extend emitter; generate+inspect first-gen check; stage-then-drift-proof;
   gate+commit).
7. **Task 12 header (`:1344`)** - retitled + `(amended 2026-07-16, owner-rulings
   routing)`.
8. **Task 12 Files (`:1348`)** - added `locales/{en,de}/gui-editor.ftl`,
   `ListWidget.vue`, `PropertyMapWidget.vue`.
9. **Task 12 Interfaces (`:1352-1353`)** - Consumes Task 12a's `settables.ts`; Produces
   `gui-editor.ftl` at final 45 keys.
10. **Task 12 Binding points (`:1359-1362`)** - added: generic-action-keys re-pointing
    (with the `check:i18n`-stays-green reasoning: `editor-attachment-rule-add`/`-drop`
    remain registry labels); typed value cells (mechanism, model widening, never-arm);
    matchable-scope surfaced boundary.
11. **Task 12 Steps (`:1368-1408`)** - restructured from 5 to 7 steps: S1 add catalog
    keys (check:i18n deferred); S2 write failing composition + caption + typed-cell
    assertions and update the T10 `propertyMap`/`list` specs (`smoke.spec.ts:724-756`
    + comment `:589-606`); S3 run RED (naming each failure); S4 re-point captions +
    typed cells; S5 section composition; S6 GREEN suite; S7 gate + commit (updated
    `git add` + message).
12. **Task 13 binding point (`:1430`)** - `stays 43 keys` -> `stays 45 keys` with the
    revision reference; added "adds no editor catalog keys".

## Premises verified against the tree

- `editor-generic-action-keys` ruling: `docs/product-boundaries.yaml:404-418` (budget
  43->45, generic keys over reuse, de "Entfernen").
- `gui-typed-scalar-needs-typed-input` ruling: `docs/decision-ledger.yaml:3617-3631`
  (typed cells land in Plan 6 via T12).
- `git-diff-proof-needs-tracked-target`: `docs/decision-ledger.yaml:3533-3546`
  (staging precondition for a `git diff --exit-code` proof).
- `SETTABLE` / `PropType`: `crates/muxsmith-core/src/capability/mod.rs:72-91` / `:22-32`;
  `pub mod capability` at `crates/muxsmith-core/src/lib.rs:8`.
- Emitter pattern: `crates/muxsmith-core/tests/ts_export.rs:18-36` (emit_keywords_ts).
- CI drift step globs `src/bindings/`: `.github/workflows/ci.yml:134-138`; bindings on
  master are `keywords.ts`, `profile.ts` (settables.ts new).
- plan6-e catalog is exactly 43 keys (en + de), `locales/{en,de}/gui-editor.ftl`;
  `editor-attachment-rule-add`="Add"/"Hinzufügen", `-drop`="Drop"/"Verwerfen"
  (`en:73-74`, `de:74-75`).
- Both widgets reuse the attachment-rule keys as generic buttons:
  `PropertyMapWidget.vue:77,80,87`, `ListWidget.vue:86,89,96` (plan6-e).
- propertyMap facets `{ properties: "matchable"|"settable"; values: "scalar"|"string" }`:
  `src/editor/fieldSpec.ts:68`; settable use = `trackRuleFields.changes`
  (`registries.ts:149-152`), matchable-scalar = `matchExprFields.exact` (`:198-201`).
- Existing T10 specs asserting old captions + string values:
  `e2e/smoke.spec.ts:724-756`, comment `:589-606`; `en(id)` catalog resolution via
  `name()` at `:58`, import `:24`; dispatcher never-arm at
  `FieldWidgetDispatcher.vue:56`.

## Budget-count sweep result (every "43" site and its disposition)

Budget (key-count) sites, all set to 45 as the authoritative/final budget:
- `:18` ruling 2 -> **45** (authoritative constraint, revised with ref). CHANGED.
- `:20` ruling 4 -> **45**. CHANGED.
- `:1430` Task 13 -> **45**. CHANGED.

As-built Task 9 counts (kept 43, arithmetically 42 labels + 1 note; +2 land in Task 12):
- `:1036` Task 9 binding -> 43 kept + final-45 annotation. ANNOTATED.
- `:1065` Task 9 Step 4 -> 43 kept + Task-12 forward note. ANNOTATED.

NOT the gui-editor.ftl key budget (left unchanged):
- `:7,:14,:127,:130,:592,:605,:608,:612,:678,:870,:1002,:1471,:1489` etc - all `D43`
  (ADR number). Not a count.
- `:31,:1029,:1042` (was 1025/1029/1038) - "43-field widget table" / "42 of 43 fields"
  / "43-row table": these count **model fields** (42 EditableField + 1 FixedField), not
  catalog keys. The 2 action keys are not fields, so field count stays 43. Left.
- `:117,:179,:193,:228` - inside Task 1, which folds the ORIGINAL four rulings into the
  **design document**. These reproduce/verify the design's own 43-statements. Per the
  prompt the design doc is superseded-by-note, NOT edited; leaving them keeps the plan's
  record of Task 1 faithful to the (unedited) design. The Amendment 2 note + ruling 2
  record the supersession. Left.
- `:395` (`:430-437`), `:624` (`:943`), `:988` (`:643-659`), `:1029` (`:848-894`),
  `:1454` (`:243-252`) - line-range citations containing "43"/"45". Not counts. Left.

New sites (my additions) referencing "43" as the superseded/old value in context:
`:39` (Amendment 2), `:1359,:1376,:1391` (Task 12) - all correctly framed as the old
claim being corrected. Consistent.

## Concerns

1. **Matchable-scalar `exact` cell stays untyped** (surfaced, not resolved). Same
   `ValueTypeMismatch` class as the settable cell, out of Ruling 2's scope. Core still
   catches it at error severity and the save-disable affordance blocks the run path, so
   no silent GUI mux path. If the owner wants it typed it is a follow-up (bigger
   generated `matchable_type` type source). Flagged in a Task 12 binding point.
2. **Integer/float widget arms are not e2e-drivable** today (no `Integer` settable in
   `SETTABLE`). They are covered by the compile-time `never`-arm exhaustiveness only;
   stated in Task 12 Step 2 so it is not read as a coverage gap. The anti-vacuity e2e
   assertion is the Boolean round-trip (`forced_track` -> real `true`, not `"true"`).
3. **The add-caption re-point is textually invisible** (both `editor-action-add` and
   `editor-attachment-rule-add` render "Add"/"Hinzufügen"). Only the remove caption
   ("Drop" -> "Remove") is a falsifiable e2e observable; the add re-point is verified by
   the widget source referencing `editor-action-add`. Stated in Task 12 Step 2 to avoid
   a vacuous assertion.
4. **Task 12's commit is heterogeneous** (composition + 2 widgets + catalog + spec).
   Task 12a isolates the core/generated piece; the rest is one coherent frontend commit
   for the two rulings. Acceptable given the ruling spans catalog + widgets, but a
   reviewer should read it as one feature landing, not a single-concern commit.
5. **"Amendment 2" heading vs the earlier unlabeled "Amendment 2026-07-16: test-mount
   harness"** - the first amendment is not labeled "Amendment 1". "Amendment 2" is now a
   stable anchor referenced ~7x; kept for reference integrity.
6. **Task 14 checked, no ripple**: it modifies `gui-batch.ftl` + batch components, not
   the editor catalog or `PropertyMapWidget`; carries no 43-key/action-key statement.

---

# Fix round 2 (2026-07-16): delta review NEEDS FIXES applied

Delta review returned one Important (routed to owner, ruled) + two Minors. All applied to
the same single file; still uncommitted.

## Important - matchable exact cells typed in Plan 6 too (owner ruling)

Owner ruling recorded in `docs/decision-ledger.yaml` `gui-typed-scalar-needs-typed-input`
(new 2nd occurrence) and `docs/product-boundaries.yaml` `gui-closed-domain-dropdowns`
(new, tier 2): the matchable `exact` map cells are typed in Plan 6, scalar form; the
curated `type`/`codec_kind` dropdowns are explicitly Plan 7 (ROADMAP). My previous
author-deferral framing of the matchable path was **replaced entirely** by the owner's
decision.

New premises verified against the tree before writing:
- `MATCHABLE_PROPERTIES` (`crates/muxsmith-core/src/capability/generated.rs:7`) = **62**
  rows: `grep -cE "PropType::$t\b"` yields String 21, Boolean 9, Integer 27, Float 5.
  Plus the `codec_kind` virtual (String, `mod.rs:41-43`, `matchable_type("codec_kind")`)
  = **63** props: 22 String, 9 Boolean, 27 Integer, 5 Float - matches the ruling's numbers.
- `generated` is a **private** module (`mod.rs:9`, no `pub`); no public re-export, no
  public enumerator (only name-keyed `matchable_type`/`matchable_domain`). So the emitter
  cannot read the full table. **Task 12a therefore adds a documented public accessor**
  `matchable_properties() -> &'static [(&'static str, PropType)]` in `capability/mod.rs`,
  keeping `generated` private - house-conforming (`SETTABLE`/`ATTACHMENT_PROPERTIES` are
  already `pub`; `settable()`/`matchable_type()` are the accessor style). This is a real
  new premise the ruling's "source is MATCHABLE_PROPERTIES" glossed over; verified, not
  assumed.
- Real property names baked into the tests/inspection: `min_luminance`/`max_luminance`
  (Float), `audio_channels` (Integer), `forced_track`/`default_track` (Boolean, matchable
  per `mod.rs:157` test).

Edits (all in Task 12a and Task 12):
- **Task 12a Files/Interfaces/binding points/Step 1**: added the `matchable_properties()`
  accessor to `capability/mod.rs`; the emitter now emits **both** `SETTABLE_TYPES` (10)
  and `MATCHABLE_TYPES` (63, generated rows in declaration order + `codec_kind` appended
  via `matchable_type("codec_kind")` so the virtual's tag is derived not re-asserted) into
  the one `settables.ts`; the shared union renamed `SettableScalarType` -> **`PropScalarType`**
  (7 sites) since it now types matchable too.
- **Task 12a Step 2**: the first-generation inspection check baked with the tree-verified
  numbers (SETTABLE 10; MATCHABLE 63 = 22/9/27/5) plus a `grep -cE` cross-check command.
- **Task 12a Step 4** commit: `git add` includes `capability/mod.rs`; message covers both
  tables.
- **Task 12 typed-cell binding point**: rewritten to type **both** maps - cell is typed
  when `values === "scalar"`, map chosen by the `properties` facet
  (`settable`->SETTABLE_TYPES, `matchable`->MATCHABLE_TYPES). Enumerated the **Float input
  variant** `<input type="number" step="any">` as a new branch **inside** the widget
  switch, explicitly NOT a new `FieldWidget` kind (Boolean->checkbox, Integer->number,
  Float->number+decimal, String->text; exhaustive `never` arm).
- **New closed-boundary binding point**: `values: "string"` maps (substring/regex,
  `registries.ts:202-209`) stay text cells - String-by-definition (a pattern is text),
  stated with its reason, enumerated on the `values` facet.
- **New Plan-7 binding point**: `type`/`codec_kind` dropdowns are the owner's Plan-7
  decision (`gui-closed-domain-dropdowns`, ROADMAP); in Plan 6 both are String -> text.
  Replaces the deferral framing entirely.
- **Task 12 Step 2**: added the matchable Boolean anti-vacuity round-trip (`forced_track`
  via `matchExprFields.exact`, real `true`) and the Float case (`min_luminance` -> number
  input, `1.5` as an actual number); corrected the stale "no Integer settable, arms not
  drivable" note (matchable now drives Boolean/Float; Integer shares the number branch).
- **Task 12 Step 3**: RED naming extended to the matchable checkbox + spinbutton failures.
- **Task 12 Step 4**: widget implementation extended to both maps + the Float variant.
- **Amendment 2 block**: ruling-2 sentence broadened to both maps + the Plan-7 dropdowns.

## Minor 2 - de register characterization

Task 12 Step 1: "declarative-verb register" -> corrected to "infinitives / nominalized
verb forms, the same button-label form the de catalog already uses at
`editor-attachment-rule-add = Hinzufügen`, not an imperative". Strings unchanged
(owner-mandated).

## Minor 3 - marker on every changed step

Added "(amended 2026-07-16, owner-rulings routing)" to Task 12's renumbered **Step 5**
header and to the Task 12 Files **Test:** line.

## Fix-round-2 concerns

1. **New core public API**: `matchable_properties()` accessor is a genuine surface
   addition the ruling did not name (it assumed the table was reachable). It is minimal,
   documented, `pub`-consistent with `SETTABLE`, and `generated` stays private. Flagged so
   the reviewer sees it is an intended new binding-point, not an incidental leak.
2. **`settables.ts` filename now carries the matchable table too** (owner allowed "one
   file total, sibling constant your call"). The name is slightly narrow for its contents;
   kept per the ruling rather than renamed to avoid a second churn. The file doc-comment
   should say it holds both tables.
3. **Integer arm has no runtime fixture** (only Boolean + Float driven, per the ruling's
   ask). It is the number branch minus `step="any"`, covered by the exhaustive `never`
   arm; an optional `audio_channels` Integer assertion is noted as cheap-if-wanted, not
   required.
4. **Float precision on the wire**: `step="any"` number inputs emit JS numbers; `Scalar`
   is `boolean | number | string`, so a float rides the `number` arm - consistent with
   `TS_RS_LARGE_INT = "number"` already forcing `i64` to `number` (D44). No bigint/precision
   trap introduced beyond the one D44 already owns.
