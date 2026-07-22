# Task 14 (D57) - Independent Reviewer Verdict

**Verdict: NEEDS FIXES**

One reachable double-marker defect that violates D57's single-anchor invariant for the `tracks[{i}]` path; the fixture masks it by construction. Everything else - the 15-path mapping contract, the three surfaced items (Q1-Q3), the semantic/presentation split, and the quality checks - is approved-quality. The fix is one line plus a fixture extension.

Commit 18a9801 on `plan7-f`, parent f8e7d5d. Full gate re-run green under my own build: **42 passed** (incl. `editor-markers` + axe), `pnpm build` (vue-tsc) exit 0, `eslint` on the task files exit 0. 15-file diff exact, no `.ftl` touched, commit message and `git add` list match the plan verbatim.

---

## Findings

### F1 (BLOCKER) - detail-panel `SectionWidget` root re-anchors `tracks[{i}]`, doubling the grid-row marker

The bespoke grid row anchors `tracks[{i}]` (design's named anchor for lint `ProvableOverlap`, `lint.rs:34`). The per-rule detail panel mounts `<SectionWidget :path="selectedPath">` with `selectedPath = tracks[{i}]` (EditorView.vue:373-375, 597-602) and **no `suppress-self-anchor`**. So `SectionWidget`'s own `useDiagAnchor(() => suppressSelfAnchor ? undefined : path)` anchors a second marker at the same path, on the panel's "Rules" legend.

This is the exact same-path collision `suppressSelfAnchor` was created to prevent for keywordOrBlock->block - applied to that collision, not to this structurally identical one. It is the invariant D57 states as closed ("One marker element per anchored control", D57:936-940) and the e2e's own stated contract ("a redundant second marker for a path - fails a count", spec header:13-16).

**Reachable in production:** `ProvableOverlap` emits at `tracks[{i}]` for the pair's second rule; the natural user action to fix an overlap is to select that rule, which opens its detail panel -> double marker.

**Fire-verified** (isolated probe spec, single bare `tracks[0]` diagnostic, then removed byte-clean): `data-diag-path="tracks[0]"` count = **1 before** opening rule 0's panel, **2 after**. The second span is the detail-panel legend, class `diag-marker--error`, identical `title`.

The task fixture does not catch it: its only bare `tracks[{i}]` diagnostic is on `tracks[1]` (fixture #5) while the test opens **rule 0**'s panel, so the flagged rule's own panel is never opened. This is a fixture-coverage hole, not a design ambiguity - the design assigns the grid row as the anchor and uses the detail root's `tracks[{i}]` only as a child-path prefix.

**Fix (small, direction unambiguous):**
1. `EditorView.vue` detail panel (line ~597): add `suppress-self-anchor` to `<SectionWidget>` - the grid row is the design-named anchor; the detail root is a path prefix only. (`suppressSelfAnchor` already suppresses the self-anchor while still using `path` for child construction - exactly the keywordOrBlock precedent.)
2. `editor-markers.spec.ts`: extend the fixture so a bare `tracks[{i}]` diagnostic sits on the rule whose panel is opened, and assert `data-diag-path="tracks[i]"` count stays 1 with the panel open. This locks the invariant the current fixture leaves open.

No other finding blocks. The remainder below is verified-good.

---

## The mapping contract (dimension 1) - PASS (one exception = F1)

Walked the grammar table against the threaded paths; all 15 fixture paths anchor at exactly one control, confirmed by the passing spec's exact per-path + total counts, and hardened by my two double-marker fire-verifications (both caught by the count/`data-diag-path` guards):

- Roots threaded correctly: top-level serialized keys in `EditorView.topLevelFields` (`input`, `output`, `attachments`, `chapters`, `title`, ...); `tracks` bespoke; `profile_version` filtered (FixedField) in both `EditorView` and `SectionWidget` -> #1 panel-only, marker count 0, panel row present.
- `SectionWidget` `.{fieldKey}` (serialized), `ListWidget` `[{i}]`, `PropertyMapWidget` `.{rowKey}`, `KeywordOrBlockWidget` passes its own path unchanged to the block. Asymmetries mirrored, not normalized: `tracks[{i}]` (never `tracks.rules[{i}]`), `attachments.rules[{i}]` (with `.rules`), `{p}.match`/`.select`/`.drop`.
- Three explicit placements verified against fixtures #8/#9/#12/#13/#15: bare `{p}.any` at the ListWidget root (#9), bare locator root at the locator SectionWidget (#8, #12), template diagnostics at the TextWidget bare template root (#13, #15).
- Unmapped path stays panel-only (#1); panel lists ALL 16 diagnostics unfiltered before and after selection (completeness asserted twice).
- Full suite 42 (`smoke` 42 incl. mount-harness + `editor-markers` + `editor-tooltips` + help + locale + catalogs). Green.

---

## Adjudications

### Q1 - `suppressSelfAnchor`: GRANT-COVERED mechanical realization (not an invented set member)

The uniform render rule + KeywordOrBlockWidget's same-path block section does produce a double marker; fixture #14 mandates the singular keyword-or-block anchor. `suppressSelfAnchor` is the minimal internal mechanism realizing that enumerated behavior, with zero outward surface (an internal prop; markers behave exactly as the fixture enumerates).

- **"No core path lands on the block wrapper alone" - verified against the grammar.** KeywordOrBlock paths are `chapters` (:151), `output.filename` (:131), `title` (:168), `tracks[{i}].source` (:107) - all anchor at the keyword-or-block widget; the block's children have strictly deeper paths (`output.filename.template`, `chapters.external`, ...). No core diagnostic anchors at the block section's own (= shared) path other than the one the keyword control already owns. So making the keyword widget the sole anchor is correct.
- **The double is real - reproduced.** Removed `suppress-self-anchor` in scratch, rebuilt `dist`, ran the spec: `[data-diag-path="chapters"]` resolved to 2 elements (strict-mode violation) and the total-count guard would also fail. Restored byte-identically (`command cp -f` + `cmp`, git clean), rebuilt green.

Verdict: legitimate, grant-covered. It is not the prop that is wrong - it is that the **same decided mechanism is applied incompletely** (F1 is the second instance of the identical same-path collision, left unsuppressed). That is a `thorough_separation` gap, not a grant-boundary overreach.

### Q2 - negative-sense naming: SOUND (coercion claim verified in this codebase)

- **Coercion claim verified empirically (Vue 3.5.39).** The type-only `defineProps<{ suppressSelfAnchor?: boolean }>()` compiles to runtime `suppressSelfAnchor: { type: Boolean, required: false }` (checked via `vue/compiler-sfc` on the actual SFC). An SSR mount with the prop absent yields `false` (typeof `boolean`), not `undefined` - Vue's Boolean casting applies to type-based declarations too.
- **Ruling: sound engineering.** With positive `anchorSelf?: boolean`, absent coerces to `false`, so every normally-rendered section (which must anchor) would silently stop anchoring unless wrapped in `withDefaults(..., { anchorSelf: true })`. The negative sense leverages absent=false=common-case-anchors, matching native HTML/Vue boolean-attribute semantics (cf. `disabled`) - the ecosystem-idiomatic choice, not a workaround. `withDefaults` with a positive name is an equally clean alternative; picking the negative sense is a legitimate taste call. Minor note: the read site `suppressSelfAnchor ? undefined : path` treats `false`/`undefined` identically, so the coercion is load-bearing for the *naming* rationale, not for correctness.

### Q3 - `role="img"` + `data-testid` + `data-diag-path` beyond the shown markup: JUSTIFIED

- **`role="img"` is an a11y necessity - verified.** The plan mandates the marker carry an accessible name (`aria-label = severity`) and that axe passes (Step 6). Stripped `role="img"` from TextWidget's marker in scratch, rebuilt, ran: axe raised **`aria-prohibited-attr` at serious impact** on 3 marker spans - "aria-label attribute cannot be used on a span with no valid role attribute." A generic span prohibits `aria-label`; `role="img"` (a graphical status dot) is the minimal role that permits it. The plan's illustrative snippet (bare span + `aria-label`) would have failed its own axe gate; the implementer correctly closed that latent defect. Restored byte-identically, rebuilt green.
- **`data-testid` + `data-diag-path` are house convention + required test infra.** `data-testid`-for-test-location is the established house pattern (cited in PropertyMapWidget's own doc). `data-diag-path` is the mechanism the plan's Step 1 explicitly requires ("a redundant second marker for a path - fails a count") - and it is exactly what caught both the Q1 and the F1 double-markers. Not unauthorized surface growth.

---

## Semantic mappings vs presentation tokens (dimension 3) - PASS

- Severity->class (`diag-marker--{severity}`, `diag-anchored--{severity}`) matches D57 exactly.
- `outline` (not `border`) + `outline-offset` - the design's enumerated no-layout-shift semantic; verified in `style.css:71-82` (outline draws outside the box, no reflow).
- Colors are carve-out presentation tokens and **mirror DiagnosticsPanel exactly** (`#c0392b`/`#d68910`/`#2471a3` == `severity-dot--{error,warning,info}`, `DiagnosticsPanel.vue:64-70`). No semantic drift - three distinct hues, error/warning/info distinguishable. The marker's `var(--diag-*-color, <fallback>)` indirection (panel hardcodes) is benign: no such vars are defined, so effective colors are identical.

---

## Quality (dimension 4) - PASS

- **`worstSeverity` extraction:** behavior-identical to the plan snippet (same `SEVERITY_ORDER = [error,warning,info]`, same `<` comparison, first-worst-wins). No ordering change. Extracted to a shared helper precisely because the grid rows and propertyMap rows compute severity *without* `useDiagAnchor` (they inject the map once and look up per row/index - exactly the plan's own per-row guidance). A justified DRY move over three inlined loops, not a deviation.
- **First `provide`/`inject` site:** confirmed - only Task 14's three files use it (`EditorView` provides, `diagAnchor.ts` declares the typed `InjectionKey` Symbol, `PropertyMapWidget` injects). Clean house-establishing template (see HARVEST).
- **15-file diff exact;** no new Fluent keys anywhere (no `.ftl` in the diff - fire-verified, see dimension 5); commit message + `git add` list verbatim to the plan.
- **`aria-invalid` ONLY on error-anchored form controls:** verified - present (conditional `severity==='error'`) on text/textarea/checkbox/select/number inputs across the 8 form widgets + all 4 PropertyMap cell variants; **absent** on the marker span itself and on section/list/grid-caption/grid-row anchors. Matches D57:956-958 exactly.

---

## Dimension 5 - my own absence check, fire-verified against a control

Claim "no new Fluent keys / no `.ftl` changed" rests on `git diff --name-only f8e7d5d HEAD | grep -i ftl` returning empty. Fired it against a known-present control: `git diff --name-only 906260b~1 906260b | grep -i ftl` -> `locales/de/gui-common.ftl`, `locales/en/gui-common.ftl`. The grep produces output when `.ftl` files change, so the empty result on the task commit is real, not a malformed pattern.

(The two double-marker probes above are themselves control-fired absence/count checks: each broken variant made the count guard fire, confirming the guards are load-bearing.)

---

## HARVEST

- **`provide`/`inject` house pattern (first site).** Template for future Vue injection in this app: a typed `InjectionKey<ComputedRef<...>>` Symbol in a dedicated module, provider computes+`provide()`s, consumers `inject(key, undefined)` with an **`undefined` default so a standalone-mounted widget degrades to no-op** (the mount-harness tolerance). Reusable shape; note it where the next injection lands.
- **Q1 = grant-boundary calibration data (over-restriction watch, both directions).**
  - *Do not over-restrict:* an internal mechanical prop that realizes enumerated fixture behavior with zero outward surface (`suppressSelfAnchor`) is **grant-covered**, not an invented set member owing NEEDS_CONTEXT. Verifying "no core path lands on the wrapper alone" against the grammar is the right test, and it passed.
  - *Do not under-apply:* the same-path-collision class had **two** instances (keywordOrBlock->block, grid-row->detail-root). Suppressing one and not the other (F1) is the `thorough_separation` failure mode - a decided dedup mechanism must be applied to every instance of its class, not the first one noticed. The trigger is readable: "two controls can construct the same path."
- **Illustrative plan snippets can carry latent defects.** The plan's marker markup (bare span + `aria-label`) would have failed its own axe gate; a11y-correctness (`role` permitting `aria-label`) has to be verified against the tool, not copied from the plan's shorthand. The implementer caught this; reviewers should treat plan markup as intent, not as axe-verified.
- **Fixture-coverage vs contract.** The e2e states the "no redundant second marker for a path" contract in its header but the fixture only exercises it for keywordOrBlock. A stated invariant needs a fixture path that actually reaches every structural site that can violate it - here, opening the detail panel of a rule that carries a bare `tracks[{i}]` diagnostic.

---

*Probes (2 src edits, 1 spec edit, 1 throwaway spec) all restored/removed byte-identically; `git status` clean; `dist` rebuilt to the committed state; verdict file is the only lasting write.*

---

## Fix-round delta verdict (2026-07-22)

**Verdict: APPROVED**

F1 is resolved to the original verdict's own standards, the fix introduces no new defect, and it conforms to the D57 single-anchor invariant. Delta scope only; Q1-Q3 and dimensions 1/3/4/5 of the original verdict are not re-litigated.

Fix commit `ff49658` on `plan7-f`, parent `18a9801`. Two files, +29/-6 (`git show --numstat`): `src/views/EditorView.vue` (+7/-1), `e2e/editor-markers.spec.ts` (+22/-5). No `.ftl`, no other file. Working tree clean after my runs (`dist`, `test-results` gitignored).

**My own gate run (all foreground, own build):**
- `pnpm build` (vue-tsc --noEmit && vite build) -> exit 0, built.
- `npx playwright test --grep "field-anchored markers"` -> **1 passed** (GREEN).
- `pnpm test:e2e` (e2e tsc + harness + mount builds + full suite) -> **42 passed** (incl. editor-markers, editor-tooltips, all mount-harness smoke, axe).

### Dimension 1 - F1 resolution: PASS

The diff implements exactly the verdict's two-part fix.

- **Suppress-self-anchor on the rule-detail SectionWidget.** `EditorView.vue:605` adds `suppress-self-anchor` to the detail-panel `<SectionWidget :path="selectedPath">`. Mechanism verified at source: `SectionWidget` reads `useDiagAnchor(() => suppressSelfAnchor ? undefined : path)` (`SectionWidget.vue:45-47`) -> no legend marker; `childPath(key)` still uses `props.path` unconditionally (`:49-51`), so children still anchor. This is the keywordOrBlock->block precedent (`KeywordOrBlockWidget.vue:80`) applied verbatim. The grid row (`EditorView.vue:571-580`, `data-diag-path="tracks[${index}]"`) remains the design-named anchor.
- **Fixture reaches the collision + count-stays-1 asserted with the panel open.** Fixture #17 (`overlapping-rules`, error, `config_path: "tracks[0]"`) sits on rule 0, the rule whose panel the test opens; `TOP_MARKERS` gains `{tracks[0], error}`; line 222 asserts `marker(page,"tracks[0]").toHaveCount(1)` as the first panel-open check.
- **The new assertion is load-bearing (spot-checked from source; RED not re-reproduced).** With the panel open, the grid row anchors `tracks[0]` once; without the suppression the detail `SectionWidget` would call `useDiagAnchor(() => "tracks[0]")`, find fixture #17 in the injected map, and render a second `data-diag-path="tracks[0]"` marker on the legend -> `toHaveCount(1)` fails with Received 2. That is exactly the report's captured RED output. `OverlappingRules` is the runtime-correct code to reach `tracks[0]` (`ProvableOverlap` anchors only at a pair's second rule, never `tracks[0]`); the `overlapping-rules` Fluent key resolves in both `locales/{en,de}/diagnostics.ftl` with matching params, so the marker `title` renders. Nothing looked off, so per the brief I did not reproduce the full red run.

### Dimension 2 - no new defects: PASS

- **Exact-count guards recomputed against the fixture enumeration.** 17 diagnostics (16 distinct paths; `input.pattern` doubled) -> panel `li` count 17 (lines 172, 240), the two `16->17` bumps are the additive delta of #17. Closed-panel markers = 10 = `TOP_MARKERS.length` (line 185): input.pattern, input.extensions, tracks.rules, tracks[0], tracks[1], attachments.rules[0], attachments.rules[0].add, output.filename.template, chapters, title.template. Open-panel total = 10 + 5 `DETAIL_MARKERS` = 15 (line 227). All guards compute off `.length`, so no literal edit there; all reconcile with the GREEN run.
- **Appended-at-end fixture claim verified.** #17 is inserted after #16 (`title.template`), last in the `diagnostics` array; every existing 1-based index (#5/#6 tracks paths, #8/#9/#12/#13/#15 referenced by the original verdict) stays valid. `TOP_MARKERS` order is iteration-only, not index-referenced.
- **EditorView change limited to the one mount plus comment** (`:370-377` comment, `:605` attribute). No other logic touched.
- **No weakened/deleted/skipped/reworded existing assertion.** `grep` for `.skip/.only/.fixme` empty (control-fired against the real `test(`/`toHaveCount` lines, which matched). The two count bumps are the computable delta, kept exact. No user-visible change beyond the marker dedup.

### Dimension 3 - house: PASS

The diff realizes the D57 single-anchor invariant as stated: the non-designated control (detail-panel `SectionWidget`) carries the suppression, the path is still used for child construction (`childPath` ignores `suppressSelfAnchor`), and the designated control (grid row) keeps the sole marker. The extended `selectedPath` comment matches the file's documented style (standalone block above the statement). No other subject of `conventions.yaml` / `process-conventions.yaml` is touched by the diff.

### HARVEST

- **Textbook `thorough_separation` closure, no over-restriction.** The fix applies the already-decided dedup mechanism (`suppressSelfAnchor`) to the second instance of its same-path-collision class, reusing the keywordOrBlock pattern verbatim - zero new prop, zero outward surface, additive pattern-conforming test. Cleanly inside the structural-conformance grant; the grant forced no stop it should have covered.
- **Fixture author picked the structurally-correct lint code, not the convenient one.** Reaching the `tracks[0]` open-panel site required a code that can anchor there; `OverlappingRules` (anchors at `tracks[0]`) was chosen over reusing `ProvableOverlap` (anchors only at the pair's second rule). This is the same "a fixture must reach every structural site that can violate a stated invariant" lesson the original verdict harvested, now applied correctly on the fix side.

*Delta review: read-only except this appended section. No source or spec edits; `pnpm build` + e2e runs only touched gitignored `dist`/`test-results`. `git status` clean; original verdict text above untouched.*
