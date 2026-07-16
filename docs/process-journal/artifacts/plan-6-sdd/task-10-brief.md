### Task 10: D45 - the widget components

**Files:**
- Create: `src/editor/widgets/` - one component per `FieldWidget` variant (10)
- Create (test-mount harness, one-time - Step 1, see the amendment subsection): `e2e/mount-entry.ts`, `e2e/vite.mount.config.ts`, `e2e/mount.ts`
- Modify (test-mount harness, one-time - Step 1): `e2e/global.d.ts`, `e2e/tsconfig.json`, `package.json`
- Test: `e2e/smoke.spec.ts` (extend)

**Interfaces:**
- Consumes: Task 9's `FieldWidget`, `RegistryName`, option arrays.
- Produces, for Task 12: a widget dispatcher that renders any `FieldSpec`.

Binding points:
- **The frontend performs zero semantic validation** (spec 7). It holds the model as data, sends it, renders the returned diagnostics. Its only sanctioned local logic is the UX affordance spec 7 names: disabling Save while errors exist.
- **Sum types get an explicit `never` arm**: `const _exhaustive: never = x`. Both shapes fire, but only TS2322 **names** the unhandled variant, which is the same property that justifies the registry over the type. This is a deliberate, minimal improvement on the existing house shape (`src/jobRowState.ts:44-55`); `jobRowState.ts` is **not** required to change.
- **Cross-field constraints stay in core** (spec 7). Two exist in this surface and neither gets a widget: `AttachmentRule` requires exactly one of `select`/`drop`/`add`, and `Locator.match_to_source` is mutually exclusive with `match_pattern`. Both are already validated core-side and surface as diagnostics. A component **may** present the one-of as a mode selector - that is a UX affordance, not frontend semantic validation.
- The registry forces a label and widget to **exist** per field; it does **not** check the widget suits the field's type. That is accepted and recorded: a mismatched widget is a visible rendering bug caught the first time the panel opens, whereas a missing entry is silent absence. Do not add the mapped type `{ [K in keyof T]: FieldSpecFor<T[K]> }` - the brief settles the mechanism.
- **The wave-3 e2e RED/GREEN runs through the test-mount harness, not the served app** (amendment 2026-07-16, mount-harness routing). No editor mount point exists in the running app before Task 13 (`src/main.ts` mounts only `App.vue`, whose `View` union is `"batch" | "jobs"`; `EditorView.vue` is nav-wired only in Task 13; Playwright's `webServer` serves the single-entry `dist/`), so the step-1 assertions of Tasks 10-12 cannot render editor UI through `page.goto("/")`. Step 1 below builds a harness (reused verbatim by Tasks 11-12) that mounts the component under test in isolation. It passes the component's `props` verbatim and round-trips the standard Vue `modelValue`/`update:modelValue` v-model, exposing the live model via `window.__muxsmithModel__()`; it installs **no** Tauri IPC mock, because the widgets here and `EditorView` through Task 12 are fed their model as a prop - IPC wiring is Task 13.

- [ ] **Step 1: Set up the test-mount harness (one-time) (amended 2026-07-16, mount-harness routing)**

This harness is created here because Task 10 is the first task that needs a DOM-level render of editor UI, and it is reused verbatim by Tasks 11 and 12. It extends the established `e2e/vite.harness.config.ts` precedent (a pre-test Vite build into gitignored `e2e/.generated/`, injected into a plain page; it never touches `dist/` and never ships). Create three files and modify three; every file and its contents are named here, with no latitude to substitute another mechanism.

1. `e2e/mount-entry.ts` (create) - the page-side bundle source, pure `.ts` (runtime `h()`, no template), no static `.vue` import. Build a component registry with

   ```ts
   const modules = import.meta.glob<{ default: Component }>(
     ["../src/editor/widgets/*.vue", "../src/views/EditorView.vue"],
     { eager: true },
   );
   ```

   The `import.meta.glob` result is **path-keyed** (its keys are the full glob-relative paths, e.g. `"../src/editor/widgets/TextWidget.vue"`); do not build a separate basename-to-module map. The caller passes a bare basename (`TextWidget`, `FieldWidgetDispatcher`, `EditorView`, and the rest) and the mount driver reconstructs the path to index `modules` (see the `resolves` step below). `eager: true` is mandatory: an IIFE forbids code-splitting, which a lazy glob would introduce. Assign `window.__muxsmithMount__`, `window.__muxsmithModel__` and `window.__muxsmithEmitted__` as side effects (the window-global shape `e2e/tauri-mock-entry.ts` already uses). `__muxsmithMount__({ component, props, locale })`:
   - unmounts any previous app and resets `window.__muxsmithEmitted__ = []`;
   - resolves `modules["../src/editor/widgets/" + component + ".vue"]` (or the `EditorView` path), throwing `unknown mount component "<name>"` when absent - that throw is the Task-10/11/12 RED before the component exists;
   - creates a wrapper root that holds `const model = ref(props?.modelValue)`, renders `h(Comp, { ...props, modelValue: model.value, "onUpdate:modelValue": (v) => { model.value = v; window.__muxsmithEmitted__.push({ event: "update:modelValue", payload: v }); } })`, and sets `window.__muxsmithModel__ = () => model.value`;
   - `.use(createFluentVue({ bundles: buildBundles(locale ?? "en") }))`, importing `buildBundles` from `../src/i18n`. This is the load-bearing reuse: the real `locales/*/gui-editor.ftl` catalogs reach the page through the app's **own** `import.meta.glob` catalog loader (`src/i18n/index.ts`), so `$t` renders real messages, not stubs;
   - `.mount("#mount")`.
2. `e2e/vite.mount.config.ts` (create) - a second Vite build, parallel to `vite.harness.config.ts` but with the Vue plugin so `.vue` compiles and both `import.meta.glob` calls inline:

   ```ts
   import { resolve } from "node:path";
   import { defineConfig } from "vite";
   import vue from "@vitejs/plugin-vue";

   const here = import.meta.dirname;

   export default defineConfig({
     plugins: [vue()],
     build: {
       outDir: resolve(here, ".generated"),
       emptyOutDir: false, // must NOT wipe tauri-mock-harness.js, built by the step before
       minify: false,
       lib: {
         entry: resolve(here, "mount-entry.ts"),
         name: "MuxsmithMountHarness",
         formats: ["iife"],
         fileName: () => "mount-harness.js",
       },
     },
   });
   ```

   `emptyOutDir: false` is ordering-load-bearing: the `test:e2e` chain runs the tauri-mock build (which cleans `.generated/`) first, then this one, which must land `mount-harness.js` beside `tauri-mock-harness.js`, not replace it. The IIFE bundles Vue, fluent-vue and `@fluent/bundle` into one self-contained file exactly as `vite.harness.config.ts` bundles `@tauri-apps/api` today (that config's own doc calls its output a "dependency-free IIFE").
3. `e2e/mount.ts` (create) - the Playwright-side helper, parallel to `e2e/mocks.ts`:

   ```ts
   import { resolve } from "node:path";
   import type { Page } from "@playwright/test";

   const MOUNT_HARNESS_PATH = resolve(import.meta.dirname, ".generated/mount-harness.js");

   export interface MountSpec {
     component: string;
     props?: Record<string, unknown>;
     locale?: string;
   }

   export async function mountComponent(page: Page, spec: MountSpec): Promise<void> {
     await page.setContent('<!doctype html><div id="mount"></div>');
     await page.addScriptTag({ path: MOUNT_HARNESS_PATH });
     await page.evaluate((s) => window.__muxsmithMount__(s), spec);
   }

   export function readModel(page: Page): Promise<unknown> {
     return page.evaluate(() => window.__muxsmithModel__());
   }

   export function readEmitted(page: Page): Promise<Array<{ event: string; payload: unknown }>> {
     return page.evaluate(() => window.__muxsmithEmitted__);
   }
   ```
4. `e2e/global.d.ts` (modify) - add the three mount globals to the ambient `Window` interface, beside the existing `__muxsmithE2E__` block:

   ```ts
   __muxsmithMount__: (spec: { component: string; props?: Record<string, unknown>; locale?: string }) => void;
   __muxsmithModel__: () => unknown;
   __muxsmithEmitted__: Array<{ event: string; payload: unknown }>;
   ```
5. `e2e/tsconfig.json` (modify) - add `"vite/client"` to `compilerOptions.types`, making it `["node", "@playwright/test", "vite/client"]`, so the `tsc --noEmit -p e2e/tsconfig.json` gate types `import.meta.glob` in `mount-entry.ts` and in the transitively imported `src/i18n/index.ts`. **No `*.vue` module shim is added or needed**: components are reached only through `import.meta.glob` (whose result `vite/client` types), never through a static `import ... from "*.vue"` (which plain `tsc` cannot resolve in this tree - there is no `declare module "*.vue"` shim, and the app build relies on `vue-tsc` instead).
6. `package.json` (modify) - insert the mount build into the `test:e2e` chain, after the existing tauri-mock build and before `playwright test`:

   ```
   "test:e2e": "tsc --noEmit -p e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && vite build --config e2e/vite.mount.config.ts && playwright test"
   ```

- [ ] **Step 2: Write the failing mount-harness assertions (amended 2026-07-16, mount-harness routing)**

Extend `e2e/smoke.spec.ts` with per-widget rendering assertions that mount each widget through `e2e/mount.ts` (`mountComponent(page, { component: "TextWidget", props: { spec: <FieldSpec>, modelValue: <value> } })`), not the served app - `page.goto("/")` reaches no widget (there is no editor mount point until Task 13). Assert each widget renders its expected control with `getByRole` (e.g. `text` -> a textbox, `bool`/`optionalFlag` -> a checkbox, `select`/`keywordOrBlock` -> a combobox of its domain tokens), and that editing updates the held model via `readModel(page)`. Assert user-facing text against `e2e/i18n-en.ts` (the real en catalog), never a hand-duplicated literal, exactly as the existing smoke tests do.

- [ ] **Step 3: Run to confirm they fail (amended 2026-07-16, mount-harness routing)**

```bash
pnpm test:e2e
```
Expected: FAIL - `__muxsmithMount__` throws `unknown mount component "TextWidget"` because `src/editor/widgets/` is still empty, so the glob registry holds no widget. That throw is the genuine RED (the component does not exist yet).

- [ ] **Step 4: Implement the 10 widgets (amended 2026-07-16, mount-harness routing)**

One component per variant from `:806-819`. Follow the house component conventions - read two existing components first (`src/components/SuggestionCard.vue`, `src/components/JobRow.vue`) and match their prop/emit/`$t` style. Each widget exposes its editable value through the standard Vue `modelValue`/`update:modelValue` v-model, which is both the idiomatic shape and what the harness round-trips. Note the recorded `withDefaults` + `T | null` vue-tsc quirk in BUILDING.md's tooling section before fighting a type error.

- [ ] **Step 5: Run the e2e suite (amended 2026-07-16, mount-harness routing)**

```bash
pnpm build && pnpm test:e2e
```
Expected: PASS - `test:e2e` rebuilds `mount-harness.js` (the widgets are now in the glob) and the per-widget assertions render green. `pnpm build` still runs so Playwright's `vite preview` webServer has a `dist/` to boot, even though the mount assertions use `page.setContent`, not the served app.

- [ ] **Step 6: Full gate, then commit (amended 2026-07-16, mount-harness routing)**

```bash
git add src/editor/widgets e2e/mount-entry.ts e2e/vite.mount.config.ts e2e/mount.ts e2e/global.d.ts e2e/tsconfig.json package.json e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "gui: the ten field widgets, exhaustive by never-arm, plus the wave-3 test-mount harness (D45)"
```

---

