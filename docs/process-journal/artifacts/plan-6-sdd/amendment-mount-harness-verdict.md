# Verdict round 1 (extracted from the reviewer transcript)

### Verdict
**Amendment:** NEEDS FIXES

The mechanism itself is sound and fully executable - every load-bearing premise holds against the tree, coverage is complete, no design latitude is opened, scope is clean, and the RED steps are genuinely falsifiable. Two cheap, precisely-scoped hardening edits gate the commit: the Q1 reviewer-side handle and one missing amendment marker. Neither touches correctness.

### Executability checks

**1. e2e type gate is plain `tsc`, no `*.vue` shim, `import.meta.glob` the only type-safe access - HOLDS.**
- `package.json:13`: `test:e2e` runs `tsc --noEmit -p e2e/tsconfig.json` (plain `tsc`, not `vue-tsc`). Confirmed.
- `e2e/tsconfig.json:4`: `"types": ["node", "@playwright/test"]` - TS `types` fully replaces (does not merge) the parent's, so root `tsconfig.json:12`'s `["vite/client"]` is absent in the e2e gate today. `import.meta.glob` is therefore untyped there until the amendment adds `"vite/client"` (diff step 5). Correct.
- No `declare module "*.vue"` shim anywhere: `grep -rn 'declare module'` and a broadened `declare module .*vue` search both returned empty; I control-tested the grep against `declare global`, which fired at `e2e/global.d.ts:16`, so the empty result is trustworthy, not a malformed pattern. `src/env.d.ts:1` is only `/// <reference types="vite/client" />`. So a static `import X from "./Foo.vue"` would fail the plain-`tsc` gate, and the glob path (typed by `vite/client`) is the correct sidestep. The amendment's core justification is verified.
- Not in the diff but worth stating: adding `vite/client` alongside `node` does not conflict - both augment `ImportMeta` and TS merges the augmentations, so `import.meta.glob` (mount-entry) and `import.meta.dirname` (`vite.mount.config.ts`, `mount.ts`) both type. And `vite/client` does **not** declare `*.vue`, so it opens no static-`.vue` path that would contradict the glob-only claim. The amendment threads this correctly.

**2. `mount-entry.ts` reuses the app's own `buildBundles`, inlining the real `gui-editor.ftl` - HOLDS.**
- `src/i18n/index.ts:17-20`: `import.meta.glob([".../gui-*.ftl", ".../diagnostics.ftl"], { query: "?raw", import: "default", eager: true })`; `:79` exports `buildBundles(locale): FluentBundle[]`.
- `src/main.ts:4,26`: the app itself boots via `buildBundles(locale)` - importing the same function reuses the same catalog loader; the glob is anchored in `src/i18n/index.ts` and resolves `../../locales/` relative to that module regardless of the importer, so the harness cannot drift from the app's loader. Export shape (`FluentBundle[]`) matches the amendment's `createFluentVue({ bundles: buildBundles(locale ?? "en") })`.
- `gui-*.ftl` picks up `gui-editor.ftl`: it exists on Task 10's actual base (commit `57cc117`, the unmerged `plan6-e` stream) with **exactly 43 keys** (en). Not on `master` yet, which is correct - Task 9 precedes Task 10 in the serial stream-E sequence (`plan:1009`). The "real, not a stub" claim is accurate.

**3. Vite lib/IIFE shape vs installed vite + Vue plugin - HOLDS.**
- `package.json`: vite `8.1.4` (`:36`), `@vitejs/plugin-vue 6.0.7` (`:31`), vue `3.5.39` (`:23`), `fluent-vue 3.8.2` (`:22`), `@fluent/bundle 0.19.1` (`:16`) - all present.
- `e2e/vite.harness.config.ts:16-27` is the working IIFE-lib precedent (`emptyOutDir:true`, `minify:false`); its doc (`:4-13`) states it bundles `@tauri-apps/api` (a `dependencies` entry, `package.json:17`) into a "dependency-free IIFE." That empirically proves IIFE lib mode inlines `dependencies` in *this* tree - so Vue/fluent-vue/@fluent/bundle (all `dependencies`) inline identically, not merely by assertion. The mount config mirrors it plus `plugins:[vue()]`; `vite.config.ts:9` proves `.vue` compiles via that same plugin.
- Ordering: harness build (`emptyOutDir:true`) cleans `.generated/` first; mount build (`emptyOutDir:false`) lands `mount-harness.js` beside `tauri-mock-harness.js`. `package.json:13`'s chain, amended to insert the mount build between harness build and `playwright test` (diff plan `:1224` region), makes that order load-bearing and correct.

**4. Injection path vs Playwright API - HOLDS.**
- `@playwright/test 1.61.1` (`package.json:28`). `page.setContent` / `page.addScriptTag({ path })` / `page.evaluate` are all long-stable APIs. `e2e/mocks.ts:24,147,160` is the sibling precedent (`resolve(import.meta.dirname, …)`, `addInitScript({ path })`, `evaluate`). `addScriptTag` (not `addInitScript`) is the *correct* variant here: the harness page is built by `setContent` with no navigation, so `addInitScript` (fires on navigation) would never run. `mount.ts`'s `resolve(import.meta.dirname, ".generated/mount-harness.js")` copies the proven `mocks.ts:24` pattern verbatim.

**5. `e2e/.generated/` gitignored - HOLDS.** `.gitignore:19`: `e2e/.generated/`. No `.gitignore` edit needed; `git status` shows only the plan doc dirty.

**RED-accuracy premise (empty-glob build succeeds):** at Step 3 the widgets dir and `EditorView.vue` are absent (verified: `src/editor/` is empty on master; `src/editor/widgets/` and `src/views/EditorView.vue` do not exist), so the glob matches zero files. Vite returns `{}` for a zero-match glob without erroring (documented behavior), so the mount build succeeds and the RED is the runtime `unknown mount component "TextWidget"` throw as described - not a build error. The run-fail step is itself the empirical confirmation; the author's report Concern 3 flags this correctly.

### Coverage

- **Task 10** - complete. Files list adds the 3 created + 3 modified harness files (plan `:1104-1105`); new binding point routes wave-3 RED/GREEN through the harness with the full contract (plan `:1118`); Step 1 sets up all six files verbatim (`:1120`); old Step 1→Step 2 mounts via `mount.ts` (`:1207`, per-widget `getByRole` intent preserved); old Step 2→Step 3 RED = the throw (`:1211`); old Step 3→Step 4 adds the `modelValue` widget contract (`:1218`); old Step 4→Step 5 GREEN via harness rebuild (`:1222`); old Step 5→Step 6 stages all seven files + updated message (`:1229`).
- **Task 11** - complete. Step 1 mounts `EditorView` with a two-rule `modelValue`, asserts grid order + `readModel().tracks.rules` after drag-reorder (`:1254`); Step 2 RED = `unknown mount component "EditorView"` (`:1258`); Step 4 GREEN via harness (`:1266`). Steps 3/5 unchanged and consistent (harness committed in Task 10, so no re-staging). Drag-reorder intent preserved.
- **Task 12** - complete. Step 1 mounts `EditorView` with a full `modelValue`, asserts section composition + widget dispatch (`:1300`); Step 2 RED correctly *distinct* (component mounts, sections not composed - `:1304`); Step 4 GREEN (`:1312`). Section-composition intent preserved.
- **Task 13** - binding note present (`:1348`), see Q1.
- **Architecture / Wave-3 intro - no contradiction.** The Wave-3 intro (`:1009-1011`) describes only task sequencing, not the e2e mechanism. `grep` for `opening the editor|page.goto|served app` returns hits *only* inside the amended text - the original "opening the editor renders …" step bodies in Tasks 11/12 were fully rewritten, leaving no stale served-app assertion anywhere in the plan.

### Adjudications

**Q1:** The binding note at `plan:1348` *does* read as a constraint, not advice, and is correctly placed in Task 13's Binding-points block: it is bolded, carries the marker, and uses imperative constraint language ("`EditorView` **must** stay mountable from an injected `modelValue`", "neither deleted nor ported"). For the **implementer** it is adequately enforced: the Tasks 10-12 mount specs live in `e2e/smoke.spec.ts` and run inside Task 13's own `pnpm test:e2e` gate, so an unconditional `load_profile` in `onMounted` throws (bare mount, no IPC mock) and turns the gate red - the violation cannot ship silently.

It **needs strengthening on the reviewer side.** The residual failure mode is not silent breakage but *gaming the red*: the implementer makes the gate green the wrong way - deleting, porting, or guarding a mount spec, or injecting an IPC mock into the bare mount. The note forbids exactly this ("neither deleted nor ported"), but that clause is a statement a reviewer must *notice*, with no step they *execute* - the very shape that reliably under-fires. Task 13's steps (`:1405`+) carry no review-check for it. Add an explicit handle. Exact text, as a new bullet in Task 13's final gate/commit step:

> - **Review-check (mount-harness coverage survives, amendment 2026-07-16, mount-harness routing):** confirm `git diff <task-12-commit> -- e2e/smoke.spec.ts` shows no mount-harness spec deleted, ported to the served app, or guarded/skipped, and that they pass in this task's `pnpm test:e2e`. Confirm `EditorView` mounts from `modelValue` alone (no unconditional `load_profile` in `onMounted`; `load_profile` feeds the model through the app's open flow). A mount spec made green by an on-mount fetch or an injected IPC mock is a wave-3 coverage regression, not a passing gate.

That converts the constraint from "notice it" to "run this diff, confirm these two facts."

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
1. **Q1 reviewer handle missing** (`plan:1348` constraint, `plan:1405`+ steps carry no check). The forward-constraint has implementer-side executable enforcement but no reviewer-side handle for the "neither deleted nor ported" clause. Add the review-check bullet above to Task 13's final step.
2. **Task 10 Step 4 body was amended but the header lacks the marker** (`plan:1218` header; `:1220` body). The body gained the load-bearing sentence "Each widget exposes its editable value through the standard Vue `modelValue`/`update:modelValue` v-model … what the harness round-trips" - the widget-side half of the harness contract - yet Steps 1/2/3/5/6 (lines 1120, 1207, 1211, 1222, 1229) all carry `(amended 2026-07-16, mount-harness routing)` and Step 4 does not. Dimension 4 requires the marker on every changed step; a marker-based diff of the plan would miss that this step was amended. Append the marker to the `Step 4` header.

#### Minor (Nice to Have)
1. **Registry-keying wording is internally inconsistent** (diff `mount-entry.ts` spec, plan `:1120` block). It says the registry is "keyed by file basename (`TextWidget`, …)" but the resolve expression accesses `modules["../src/editor/widgets/" + component + ".vue"]` - and `import.meta.glob` returns a **path-keyed** object, not basename-keyed. The lookup expression is correct and unambiguous, so there is no functional latitude; only the "keyed by file basename" phrase is loose (it describes the caller-facing param, not the object's keys). Reword to "the `import.meta.glob` result is path-keyed; the caller passes a basename that the mount driver reconstructs into the path" to avoid an implementer building a redundant basename map.
2. **Empty-glob-build-succeeds is documented-behavior, self-confirmed at the run-fail step.** No action needed - the amendment already frames Step 3 as the empirical confirmation - noting it because it is the single premise resting on Vite semantics rather than a tree artifact.

### HARVEST
- **Good pattern worth a doctrine note: "reuse the app's own loader so the test path cannot drift from production."** The amendment's strongest move is importing `buildBundles` rather than mirroring a Fluent bundle - the harness *is* the app's catalog loader, so the "real catalog vs stub" risk is structurally eliminated, not policed. This is the same principle as the existing `tauri-mock-entry.ts` re-exporting the real `@tauri-apps/api` instead of reimplementing the wire contract. Generalizes: a test double that *imports* the production seam beats one that *reconstructs* it.
- **Recurring enforcement gap: a forward cross-task constraint stated as a binding sentence has no reviewer handle.** Q1 is an instance of the `regel-braucht-ausloeser-und-handgriff` pattern at plan-doctrine altitude: a constraint that spans tasks ("stay green after Task 13") needs an *executed* check in the downstream task's steps, not just a *stated* one in its binding points. Candidate house rule: "a binding point that constrains a later task's design must be paired with a review-check line in that task's steps." This is the second time (per the branch's harvest log) a cross-task constraint landed as prose-only; worth promoting.

---

# Re-review after the fix round

**Amendment: APPROVED**

- Important 1 (Q1 review-check bullet): resolved. New hunk at diff `@@ -1276,6 +1371,8 @@`, plan line ~1374, places my prescribed text verbatim as a bullet in Task 13 Step 5, after "Full gate, then commit" and before the `git add`/commit block (diff lines 242 vs 244-246).
- Important 2 (marker on Task 10 "Implement the 10 widgets"): resolved. Header now reads `Step 4: Implement the 10 widgets (amended 2026-07-16, mount-harness routing)` (diff line 137), consistent with the other five amended steps.
- Minor 1 (path-keyed wording): resolved. Diff line 50 now states the `import.meta.glob` result is path-keyed with an example key, "do not build a separate basename-to-module map," and that the caller passes a basename the driver reconstructs into the path; no longer implies a basename-keyed object.
- Nothing beyond the three edits changed: the three prior hunks (amendment subsection, Task 10 Files, Tasks 11/12/13-binding) are byte-identical to my prior read; the only new content is the reworded line 50, the added marker on line 137, and the new review-check hunk. The blob hash and ~1.3k-byte growth are fully accounted for by these three.
