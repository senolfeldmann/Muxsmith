# Amendment report: test-mount harness for wave 3 (Plan 6, Tasks 10-12)

**Status: DONE.** One file edited: `docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`
(`git status`: `M` on that path only; `+121 / -26`). No product code, no test code, no
config touched - the plan document only. Not committed; left in the working tree for the
delta review.

## The mechanism chosen, and why it fits the existing harness pattern

A **second Vite lib/IIFE build** that bundles a page-side mount entry into
`e2e/.generated/mount-harness.js`, injected into a plain page via `page.setContent` +
`page.addScriptTag`, driven by a `window.__muxsmithMount__(spec)` global from a
Playwright-side helper. It is the structural twin of the established
`e2e/vite.harness.config.ts` -> `e2e/.generated/tauri-mock-harness.js` -> `mocks.ts`
chain, differing only in what it bundles (a Vue-mounting entry vs. the Tauri IPC mock).

Files it **adds** (all under `e2e/`, all output gitignored):
- `e2e/mount-entry.ts` - page-side bundle source. Component registry via
  `import.meta.glob(["../src/editor/widgets/*.vue", "../src/views/EditorView.vue"], { eager: true })`,
  keyed by basename; assigns `window.__muxsmithMount__` / `__muxsmithModel__` / `__muxsmithEmitted__`
  as side effects; wires Fluent through the app's own `buildBundles` from `../src/i18n`.
- `e2e/vite.mount.config.ts` - the second build (`plugins: [vue()]`, IIFE lib,
  `outDir: e2e/.generated`, `emptyOutDir: false`, `fileName: () => "mount-harness.js"`).
- `e2e/mount.ts` - Playwright helper (`mountComponent`, `readModel`, `readEmitted`).

Files it **modifies**:
- `e2e/global.d.ts` - three ambient `Window` globals for the mount API.
- `e2e/tsconfig.json` - add `"vite/client"` to `types`.
- `package.json` - insert the mount build into the `test:e2e` chain.

**Why this shape and not the alternatives:**

- **Reuse over reinvention (the load-bearing decision).** The one hard requirement is that
  the widgets render through real `gui-editor.ftl` messages, not stubs. `main.ts:24-27`
  builds the bundle chain via `buildBundles(locale)` from `src/i18n`, and
  `src/i18n/index.ts:17-20` loads the catalogs with
  `import.meta.glob(["../../locales/*/gui-*.ftl", ...], { query: "?raw", eager: true })`.
  Because the mount entry imports that same `buildBundles`, Vite inlines the real
  `locales/*/gui-editor.ftl` (created in Task 9) transitively - the harness cannot drift
  from the app's catalog loader, since it *is* the app's catalog loader. No stub, no
  hand-mirrored bundle.
- **`import.meta.glob` component registry, no static `.vue` import.** This is not taste; it
  is forced by the type gate. `test:e2e` runs `tsc --noEmit -p e2e/tsconfig.json` (plain
  `tsc`, not `vue-tsc`). There is **no `declare module "*.vue"` shim** in the tree
  (`src/env.d.ts:1` is only `/// <reference types="vite/client" />`; the app build relies
  on `vue-tsc` understanding `.vue` natively). A static `import X from "./Foo.vue"` in an
  e2e file would fail that gate. Reaching components through `import.meta.glob` (whose
  result `vite/client` types) sidesteps the missing shim entirely; the only tsconfig change
  needed is adding `"vite/client"` to the e2e `types` (it is dropped there today - the e2e
  tsconfig overrides root's `types: ["vite/client"]` with `["node", "@playwright/test"]`).
- **A separate config with `emptyOutDir: false`, not a second entry in the existing one.**
  IIFE/UMD output forbids multiple entry points (Rollup restriction), so the mount build
  cannot be a second `lib.entry` in `vite.harness.config.ts`. A separate config, ordered
  after the tauri-mock build in the `test:e2e` chain and set `emptyOutDir: false`, lands
  `mount-harness.js` beside `tauri-mock-harness.js` (the tauri build cleans `.generated/`
  first with its `emptyOutDir: true`).
- **`page.setContent` + `addScriptTag`, no Tauri IPC mock.** Tasks 10-12 are pre-IPC (IPC
  wiring is Task 13), and the components are fed their model as a prop, so the mount page
  needs no `dist/`, no served app, and no Tauri mock. The harness round-trips the standard
  Vue `modelValue`/`update:modelValue` v-model and exposes the live model via
  `window.__muxsmithModel__()`, which covers the drag-reorder "held/emitted model" intent.

**Tree evidence checked (versions + config lines):**
- `package.json:36-37`: `vite 8.1.4`, `vue-tsc 3.3.7`; `:31` `@vitejs/plugin-vue 6.0.7`;
  `:23` `vue 3.5.39`; `:22` `fluent-vue 3.8.2`; `:27` `@playwright/test 1.61.1`. `:13`
  the exact `test:e2e` chain amended.
- `e2e/vite.harness.config.ts:16-28`: lib/IIFE into `.generated`, `emptyOutDir: true`,
  `minify: false` - the pattern the mount config mirrors. Its `e2e/tauri-mock-entry.ts:17-20`
  proves an IIFE bundles a dependency self-contained ("dependency-free IIFE" per the config
  doc) - so Vue + fluent-vue bundle the same way.
- `e2e/mocks.ts:147-148`: the `addInitScript({ path: HARNESS_PATH })` injection precedent.
- `playwright.config.ts:21-33`: `webServer` = `vite preview` over `dist/`; confirms
  `page.goto("/")` can only reach `index.html` -> `App.vue` (the executability gap).
- `src/main.ts:24-27` + `src/i18n/index.ts:17-20,79-94`: the `buildBundles` reuse anchor.
- `src/App.vue:10`: `type View = "batch" | "jobs"` (no editor member); `:105-115` views
  fed by props (`:pending-run`), confirming the prop-driven mount shape.
- `.gitignore:19`: `e2e/.generated/` already gitignored - `mount-harness.js` lands there,
  no `.gitignore` edit needed.

## Plan hunks changed

1. **New subsection `## Amendment 2026-07-16: test-mount harness for wave 3`** after the
   "How this plan cites the design" section. Five sentences: the executability gap, the
   controller's Option-A routing (Option B rejected), the chosen mechanism, what 10-12 vs.
   13 assert against, and "set up once in Task 10, reused by 11-12."
2. **Task 10** (widget components):
   - Files list: added the three created + three modified harness files.
   - New binding point: the wave-3 e2e RED/GREEN runs through the harness, not the served
     app; harness contract (props verbatim, `modelValue` round-trip, no IPC mock).
   - Steps restructured 5 -> 6: inserted **Step 1 "Set up the test-mount harness (one-time)"**
     with the full file list and verbatim contents of all six files. Old Step 1 (write
     assertions) -> Step 2, now mounting widgets via `e2e/mount.ts` instead of a mocked
     `load_profile`. Old Step 2 (run-fail) -> Step 3: RED reason changed from "no widgets
     exist" to `__muxsmithMount__` throwing `unknown mount component "TextWidget"` (empty
     glob). Old Step 3 (implement) -> Step 4: added "each widget exposes its value through
     `modelValue`". Old Step 4 (run suite) -> Step 5: GREEN via harness rebuild, note on why
     `pnpm build` still runs. Old Step 5 (commit) -> Step 6: staging extended to the six
     harness files, commit message updated.
3. **Task 11** (rule grid + drag-reorder): Step 1 now mounts `EditorView` via the harness
   with a two-rule `modelValue` and asserts DOM swap + `readModel().tracks.rules`; states
   EditorView takes the profile as `modelValue`. Step 2 RED = `unknown mount component
   "EditorView"`. Step 4 = GREEN via harness. Steps 3, 5 unchanged (staging unchanged).
4. **Task 12** (section composition + widget dispatch): Step 1 mounts `EditorView` with a
   full-profile `modelValue`; asserts sections + dispatched widget types. Step 2 RED =
   EditorView mounts (Task 11) but sections not composed. Step 4 = GREEN via harness. Steps
   3, 5 unchanged.
5. **Task 13**: one new binding-point sentence - the Tasks 10-12 mount-harness specs keep
   running and stay green, are neither deleted nor ported, so `EditorView` must stay
   mountable from an injected `modelValue` and `load_profile` feeds that model through the
   app's open flow (not an unconditional on-mount fetch). Steps unchanged.

Each amended step header carries the suffix `(amended 2026-07-16, mount-harness routing)`.

## Premises verified (file:line)

- Catalog loader is `import.meta.glob(..., { query: "?raw", eager: true })` and is reused
  via `buildBundles` - `src/i18n/index.ts:17-20`, `src/main.ts:26`.
- No `*.vue` module shim exists; only `src/env.d.ts:1` (`vite/client` ref). Root
  `tsconfig.json` includes `src/**` + `vite.config.ts`, `types: ["vite/client"]`; e2e
  `tsconfig.json:4` overrides `types` to `["node", "@playwright/test"]` (no `vite/client`)
  and includes only e2e files - hence the two required tsconfig facts (add `vite/client`;
  reach `.vue` only through glob).
- `e2e/.generated/` gitignored - `.gitignore:19`.
- IIFE bundles a dependency self-contained - `e2e/vite.harness.config.ts` doc + it bundling
  `@tauri-apps/api`.
- Ambient-global injection shape - `e2e/global.d.ts:16-38`, `e2e/tauri-mock-entry.ts:20`.
- `src/editor/` does not exist yet (Task 9 creates it), `src/views/EditorView.vue` does not
  exist yet (Task 11) - so the glob is empty at Task 10 Step 3, giving a genuine RED.

## Concerns

1. **EditorView's model source (latent, routed to Task 13).** The harness feeds the model as
   a `modelValue` prop. If Task 13's implementer wires `load_profile` into EditorView's
   `onMounted` unconditionally, the bare harness mount (no Tauri mock) would throw and the
   Tasks 11-12 specs would go red. The added Task 13 binding-point sentence closes this by
   requiring EditorView to stay mountable from an injected `modelValue`, with the load path
   in the app's open flow. This is a real constraint on Task 13's product design, stated but
   not otherwise enforced by a test until Task 13 runs.
2. **Widget v-model contract.** The harness round-trips only the standard
   `modelValue`/`update:modelValue`. Task 10's Step 4 now states widgets expose their value
   that way. A widget that chose a non-standard emit name for its value would not be observed
   by `readModel`/`readEmitted` (its DOM would still be assertable). The idiomatic Vue choice
   is `modelValue`, so this is a named contract, not open latitude.
3. **`import.meta.glob` array-pattern + eager on a not-yet-existing path.** The glob lists
   `../src/views/EditorView.vue` before that file exists (Tasks 10-11). `import.meta.glob`
   matches present files only and yields an empty entry for an absent path, so the Task 10
   build does not error - it simply has no `EditorView` key yet, which is the intended
   Task 11 RED. Verified conceptually against Vite glob semantics; the SDD implementer will
   confirm empirically at Step 3 (the run-fail step is exactly that confirmation).
4. **Spec-file placement.** The mount assertions stay in `e2e/smoke.spec.ts` (minimal change,
   per the existing task structure) rather than a new spec file. They coexist with the
   served-app smoke tests using `page.setContent` instead of `page.goto`; Playwright runs
   both fine. If the reviewer prefers a dedicated `e2e/editor.mount.spec.ts`, that is a
   one-line redirect in each Step 1 and the Task 10 staging line - flagged, not taken, to
   keep the amendment minimal.
5. **Architecture paragraph + wave-3 intro left unchanged.** Checked both (plan `:7` and the
   Wave 3 intro at the top of that section): neither describes the e2e mechanism, so neither
   contradicts the amendment. Per the brief's "only if they contradict," untouched.

## Fix round (delta review NEEDS FIXES, 2026-07-16)

Applied exactly the three coordinator-relayed edits; nothing else changed; still uncommitted; ASCII typography verified.

- **Important 1 - Task 13 review-check bullet (plan line 1374).** Added the verbatim `Review-check (mount-harness coverage survives ...)` bullet inside Task 13's Step 5 ("Full gate, then commit"), before the `git add`/commit block. Gives the "neither deleted nor ported" clause an executable reviewer-side handle: diff `e2e/smoke.spec.ts` since the Task-12 commit for deleted/ported/skipped mount specs, and confirm `EditorView` mounts from `modelValue` alone (no on-mount `load_profile`, no injected IPC mock making a spec green).
- **Important 2 - Task 10 Step 4 marker (plan line 1218).** Appended `(amended 2026-07-16, mount-harness routing)` to the "Implement the 10 widgets" header; its body already carried the amended `modelValue`/`update:modelValue` contract sentence, so it now matches Steps 1/2/3/5/6.
- **Minor 1 - registry-keying wording (plan line 1133, in the `mount-entry.ts` spec block).** Replaced "keyed by file basename (...)" with an explicit statement that the `import.meta.glob` result is path-keyed and the mount driver reconstructs the path from a bare basename, forbidding a redundant basename-to-module map.
