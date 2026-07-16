# Verdict round 1

All premises verified. I have everything needed for the verdict. Final report:

---

### Verdict
**Amendment:** NEEDS FIXES

Single blocking issue (Q1): the matchable-cell typing disposition is an owner decision of the same class Şenol was routed for on the settable side, but the plan presents it as an author-settled "follow-up." Route it before commit. The settable implementation, Task 12a, the budget sweep, falsifiability, and scope discipline are all commit-ready.

### Executability checks

**Type-map mechanism (holds).** `PropType` has exactly four variants: `String | Boolean | Integer | Float` (`crates/muxsmith-core/src/capability/mod.rs:23-31`). The prescribed exhaustive match `String->"string"`, `Boolean->"boolean"`, `Integer->"integer"`, `Float->"float"` (diff Task 12a Step 1) is total over the real variant set; a 5th variant fails the Rust build as claimed. `SETTABLE: &[(&str, PropType, &str)]` has 10 entries (`mod.rs:72-91`): 3 `String` (`language`, `track_name`, `sub_charset`), 7 `Boolean` (incl. `forced_track`, `default_track`) — so the Step 2 expected-content check (10 rows; those five spot-values) is accurate, and "no Integer/Float settable exists today" holds. `pub mod capability` confirmed (`lib.rs:8`); `PropType`/`SETTABLE` `pub`. The emitter pattern matches the committed `emit_keywords_ts` (`tests/ts_export.rs:18-36`): reads `TS_RS_EXPORT_DIR`, `// @generated` header, `std::fs::write(Path::new(&dir).join(...))`; the file's own doc (`:7-9`) confirms ts-rs exports types not values, so a hand-written value emitter is the right vehicle.

**Drift-check interaction (holds).** `ci.yml:134-138` runs `cargo test -p muxsmith-core --features ts` then `git diff --exit-code src/bindings/`; the D44 comment (`ci.yml:125-127`) states the diff "gates every tracked file under the directory," so a committed `settables.ts` is covered automatically. The amendment states the staging-before-proof precondition explicitly (Task 12a Step 3, "Stage first: ... against an untracked path it exits 0 regardless of content"), citing `git-diff-proof-needs-tracked-target` (`decision-ledger.yaml:~3533-3546`) correctly.

**Widget premises (holds).** On plan6-e: `PropertyMapWidget.vue` model is `Record<string, string> | null` (`:29`), value cell is a single `<input type="text">` for all values (`:69-74`), remove button uses `$t('editor-attachment-rule-drop')` (`:77,80`), add uses `$t('editor-attachment-rule-add')` (`:87`); its doc (`:19-24`) asserts "stays at 43 keys" and (`:9-15`) already names the scalar gap as an "accepted gap" — the pre-existing state the amendment's before-state describes. `ListWidget.vue` reuses the same keys (`:86,89,96`). `FieldWidgetDispatcher.vue:56` is exactly `const _exhaustive: never = kind;` — the never-arm shape the amendment tells Task 12 to mirror.

**Catalog premises (holds).** plan6-e `gui-editor.ftl` is 43 keys en and de. `editor-attachment-rule-drop = Drop`/`Verwerfen`, `editor-attachment-rule-add = Add`/`Hinzufügen` (en:73-74, de:75-76 — the report's "de:74-75" is off by one, immaterial). No `editor-action-*` keys yet. `registries.ts`: `trackRuleFields.changes` is `propertyMap, properties:"settable", values:"scalar"` (:149-152); `matchExprFields.exact` is `properties:"matchable", values:"scalar"` (:198-201); `attachmentRuleFields.add/.drop` carry the `editor-attachment-rule-add/-drop` labels on `section` widgets (:185-189) — this is the second, surviving use that keeps `check:i18n` green after the widgets re-point, verified. The new de strings `Hinzufügen`/`Entfernen` match the ruling verbatim and the existing de register (`editor-attachment-rule-add` is already `Hinzufügen`). Smoke-spec anchors confirmed: `name()` at `:58`, `en` import `:24`, the "43 keys" describe-comment at `:605` (block `:589-606`), the `propertyMap`/`list` T10 specs at `:724-757` using `{ forced: "true" }` (string) and the old caption keys.

### Coverage

**Ruling 1 (generic action keys): carried.** Bilingual keys added (diff Task 12 Step 1); widgets re-pointed and their reuse doc-comments rewritten (Step 4); budget stated 45 at all three authoritative sites.

**Ruling 2 (typed value cells): carried.** Task 12a emits the type binding; Task 12 adds the exhaustive typed switch with never-arm + text fallback, widens the model to `Record<string, Scalar>`, with a real-boolean round-trip anti-vacuity assertion (Step 2 point 3).

**"43" sweep (correct).** Every remaining `43` in the amended plan judged individually:
- Three authoritative budget sites -> **45**: `:18` (ruling 2), `:20` (ruling 4), `:1430` (Task 13). ✓
- Task 9 as-built 43 **annotated, not rewritten**: `:1036`, `:1065` (kept 43 = 42 labels + 1 note, with forward note that +2 land in Task 12). ✓
- Model field-count 43s **correctly left**: `:31`, `:1029`, `:1033`, `:1042` ("43-field/row widget table", "42 of 43 fields" — these count model fields, not catalog keys; the 2 action keys are not fields). ✓
- Task 1 design-fold 43s **correctly left superseded-by-note**: `:117`, `:179`, `:193`, `:228` — all inside Task 1, which folds the original rulings into the (unedited) design; supersession recorded via Global Constraints ruling 2 + Amendment 2. ✓
- Old-value references in changed text: `:39`, `:1359`, `:1376`, `:1391` — all framed as the superseded/old claim. ✓
- `D43` ADR refs not matched by `\b43\b` (no word boundary in `D43`), correctly untouched.

No operative budget statement was missed. Task 12a sequenced correctly (after Task 11, before Task 12; its `settables.ts` consumed by Task 12) with its own gate+commit (Step 4); Task 12 restructured 5->7 steps with its own gate+commit (Step 7); T10 mount-spec updates enumerated with RED (Step 3 names each failure) / GREEN (Step 6).

### Adjudications

**Q1(a): faithful to the recorded ruling, but the matchable disposition is owner-territory — route it.** The ledger entry `gui-typed-scalar-needs-typed-input` (`decision-ledger.yaml`, decided occurrence) reads "type-appropriate input **per settable property type**"; the originating T10 trace named the `changes`/settable path (`validate_changes/scalar_fits`). Scoping the typed cells to `properties:"settable"` is faithful to that recorded text — the author is not entitled to expand an owner ruling, and typing matchable was not ruled.

But the matchable `exact` cell carries a **verified-identical, user-reachable** defect, not a theoretical one: the matchable domain includes `forced_track` (Boolean) and `audio_channels` (Integer) — proven by the crate's own tests at `mod.rs:157-158` (`matchable_type("forced_track") == Some(Boolean)`, `matchable_type("audio_channels") == Some(Integer)`). A user matching `exact: { forced_track: true }` hits the same permanent `ValueTypeMismatch` a string-only cell forces. Whether Plan 6 also types that cell is a user-visible product-scope decision of exactly the class that was owner-routed twice the same day (the generic-keys decision, T10 Q2: "user-visible wording, proc-latitude routing rule"; the settable-typing decision itself). By parity it is Şenol's call, not the author's.

The amendment closes it as "if the owner wants it typed, it is a follow-up" (diff Task 12 binding point, plan `:1362`) with an effort-economy rationale ("the far larger generated `matchable_type` table") — the precise coloring `feedback_spiegelung_offene_dimensionen` warns against, and a "follow-up" note is the "die as noted" failure the prompt names. So (a) **resolves to: needs the owner.** The controller should route a NEEDS_CONTEXT decision — type matchable in Plan 6 (source: `matchable_type`/`MATCHABLE_PROPERTIES`, not `SETTABLE`), or owner-defer it to Plan 7 explicitly — and the plan's framing should reflect his ruling rather than presume deferral.

**Q1(b): yes** — for the *implementer*, the exclusion is a closed, unmisreadable boundary: the widget branches on the `properties` facet, matchable/`values:"string"` stay text cells, settable/scalar gets typed cells. No implementer latitude. The defect is not that an implementer could misread it; it is that the author, not the owner, made the product call.

**Q2: correct — no ci.yml change needed, and no other gate needs extension.** `ci.yml:134-138`'s `git diff --exit-code src/bindings/` gates every tracked file in the directory, so the committed `settables.ts` is covered from its first commit (D44 comment `:125-127` states this). The emitter test is a `#[test]` in the `#![cfg(feature = "ts")]` file, so `cargo test -p muxsmith-core --features ts` already runs it with no workflow edit; the default `cargo test --workspace` skips it (feature off) as the amendment notes; `pnpm build` typechecks the standalone `settables.ts` via the same `src/bindings/` include that already covers `keywords.ts`/`profile.ts`. The untracked-first-generation hole is closed in-task (Step 3 stages before proving) and by the commit. Verified against the committed workflow.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
1. **Matchable-cell typing is an owner decision closed by the author (Q1).** Plan `:1362` (diff Task 12 binding point "Scope of the typed cells is `properties:"settable"` only") presents the matchable exclusion as a settled author "follow-up." Given the verified-identical Boolean/Integer defect on the same widget (`mod.rs:157-158`) and the parity with the two same-day owner-routed decisions of this class, route it to Şenol as NEEDS_CONTEXT (type matchable in Plan 6 via `matchable_type`, or explicitly owner-defer to Plan 7); reframe the binding point to record his ruling rather than presume deferral. Lightweight fix — the settable work does not change.

#### Minor (Nice to Have)
2. **de register wording.** The amendment calls `Hinzufügen`/`Entfernen` a "declarative-verb register" (diff Task 12 Step 1); they are infinitives/nominalized, and the de header rule nominally says du-imperative. The strings are owner-mandated and catalog-consistent (existing `editor-attachment-rule-add = Hinzufügen`), so no change to the strings — only the loose characterization.
3. **Marker completeness.** Task 12 Step 5 (section composition, content unchanged, only renumbered from old Step 3) and the Task 12 `Test:` line carry no "(amended ...)" marker while the surrounding block does. Content is unchanged, so defensible; flagging for the strict reading of "marker on every changed step."

### HARVEST
- **Parity-routing rule (doctrine candidate):** when the owner rules on one instance of a defect class on a shared component, the identical instance on the same component is owner-territory *by parity* and not author-closeable; surfacing it as a plan "follow-up" is the die-as-noted failure, not a discharge of the surface-don't-resolve duty. This sharpens `feedback_spiegelung_offene_dimensionen` for the plan-amendment-authoring context (design-authoring is allowed to decide design, but user-visible product scope stays owner-routed).
- **Verified-absence discipline done right:** Task 12a Step 3's drift proof is a model fire-once — it stages, confirms exit=0, deliberately appends drift, watches exit=1, restores, and instructs "read the fire from the OUTPUT, not from its absence." Good instance to cite when teaching the negative-check rule.
- **Effort-economy tell:** the author's justification for the matchable exclusion ("the far larger generated `matchable_type` table") is a clean example of effort-economy coloring a scope reading — the exact signal to escalate rather than close.

---

# Re-review after fix round 2 (matchable ruling folded in)

All delta checks pass against the tree. Final re-review:

**Amendment: APPROVED**

1. **Matchable-extension premises + accessor — holds.** `generated.rs` counts verified exactly: 62 rows = 21 String / 9 Boolean / 27 Integer / 5 Float; +`codec_kind` virtual (String) = 63 / 22 String (`generated.rs:7`; type grep 21/9/27/5). `generated` is private (`mod.rs:9` `mod generated;`, no `pub`), so the table is genuinely unreachable from a `tests/` crate — the accessor need is real. `pub fn matchable_properties() -> &'static [(&'static str, PropType)]` is the minimal correct exposure: keeps `generated` private, returns the one table read-only, matches the existing `pub fn matchable_type`/`settable` + `pub static SETTABLE`/`ATTACHMENT_PROPERTIES` style, and `matchable_type` already reads the same static from `mod.rs:44` so the path compiles. In-mandate: listed in Task 12a Files, Step 1, and the commit `git add` (diff `:106`) — a core-crate concern correctly placed in Task 12a, not Task 12.

2. **Float variant + string-map boundary — holds.** `float -> <input type="number" step="any">` is enumerated inside the `PropScalarType` switch, explicitly "the one new input variant ... not a new `FieldWidget` variant" (plan `:1364`, `:1397`). substring/regex-stay-text is a closed boundary with reason ("String-by-definition"), keyed on the `values` facet; `registries.ts:202-209` confirms both are `properties:"matchable", values:"string"`.

3. **Dropdowns record the owner's ruling; deferral framing gone.** Plan `:1366` states the `type`/`codec_kind` dropdowns as "the owner's recorded Plan-7 decision, not a presumed follow-up." The only `follow-up` occurrence in the plan is that negation. All three referenced house records exist and cross-check: `gui-closed-domain-dropdowns` (`product-boundaries.yaml:419`, Şenol decree, type/codec_kind → Plan 7), the matchable occurrence on `gui-typed-scalar-needs-typed-input` (`decision-ledger.yaml:3632`, same 63/22/9/27/5 counts), and `ROADMAP.md:89-91`.

4. **Both Minors resolved.** Register: plan `:1391` now reads "owner-mandated strings; they are infinitives / nominalized verb forms ... not an imperative" (accurate, catalog-consistent). Markers: Step 5 (`:1400`) and the Test line (`:1352`) both now carry "(amended 2026-07-16, owner-rulings routing)".

5. **Rename clean.** Zero `SettableScalarType` remain in the plan; `PropScalarType` at every site (10 mentions across Task 12a Interfaces/binding/Step 1-2 and Task 12 Interfaces/binding/Step 4). No dangling old name; the name is semantically right now that both settable and matchable feed it.

6. **Matchable anti-vacuity present with named RED.** Step 2.4 (`:1381`): matchable Boolean `{forced_track:false}` → checkbox asserting `=== true`; Float `{min_luminance:1}` → spinbutton `step="any"`, set 1.5, `readModel` returns real number `1.5`. Both fixtures real (`generated.rs:41` Boolean, `:48` Float). RED named at `:1387`: both settable+matchable `forced_track` textbox-not-checkbox / string-not-boolean, and `min_luminance` textbox-not-spinbutton.

7. **In-scope; no re-litigation.** The round-1→round-2 delta maps entirely to the folded matchable ruling + dropdowns Plan-7 recording + the two Minors. Unchanged: Global Constraints ruling 2/4, Task 9 as-built annotations, Task 13, the mount-harness bullet, the "43" sweep dispositions, and Task 12a's fire-once drift proof (Step 3 verbatim). The settable path is intact; the matchable path is added alongside it in the same switch. No settled round-1 non-finding reopened.

The Important-1 governance defect is fully cured: the owner ruled, the ruling is recorded in the house files, and the plan now carries it as a closed owner decision rather than an author-presumed deferral. Executability, coverage, latitude, scope, and falsifiability all hold. Cleared to commit.
