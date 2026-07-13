# Idiomacy review - slice F5 (frontend: src/ + e2e/ + index.html + playwright.config.ts)

HEAD: 2f17880a956e05f833a3afdec2c650c176e391e5
Toolchain judged against: Vue 3.5.39, TypeScript 6.0.3, fluent-vue 3.8.2 / @fluent/bundle 0.19.1, @playwright/test 1.61.1, Vite 8.1.4, node 26.5.0, `"type": "module"`.

## Files read (complete)

src/App.vue, src/main.ts, src/env.d.ts, src/ipc.ts, src/i18n/index.ts, src/jobRowState.ts, src/diagnosticFluentParams.ts, src/components/{DiagnosticsPanel,JobRow,LiveLog,ResolutionTable,RunHistory,SettingsDialog,SuggestionCard}.vue, src/views/{BatchView,FirstRun,JobsView}.vue, e2e/{global.d.ts,i18n-en.ts,mocks.ts,smoke.spec.ts,tauri-mock-entry.ts,vite.harness.config.ts}, index.html, playwright.config.ts.

## Verification performed (not from training memory)

- `useTemplateRef`: Vue official docs (context7, /vuejs/docs) present it as the 3.5+ way; the `ref(null)` name-matching pattern is explicitly labeled "before 3.5".
- `defineModel`: Vue docs: "In Vue 3.4+, the defineModel() macro is the recommended implementation method" for component v-model; local-fallback-with-default behavior confirmed.
- Reactive props destructure defaults: Vue docs describe `withDefaults` as the mechanism "for Vue 3.4 and below"; native destructure defaults are the 3.5+ form.
- `import.meta.dirname`: node 26.5 native (stable since 20.11); empirically verified to work under Playwright 1.61's ESM loader (scratchpad spec run against the repo's own Playwright: passed); Vite 8's config loader shims it (`__vite_injected_original_dirname` present in `node_modules/vite/dist/node/chunks/node.js`).
- `@tauri-apps/api` type exports: `mocks.d.ts` exports `mockIPC`/`mockWindows`/`clearMocks`; `event.d.ts` declares/exports `emit<T>`.

## Findings

### 1. idiom - hand-rolled v-model protocol instead of `defineModel` (src/views/JobsView.vue:210, src/App.vue:114)

JobsView declares a local `runActive` ref, emits `"update:runActive"` via a dedicated `watch(runActive, (v) => emit("update:runActive", v), { immediate: true })` (lines 205-210 incl. comment block), and App wires `@update:run-active="jobsRunActive = $event"`. The event is even named in the v-model protocol's `update:x` form, but no `runActive` prop exists on JobsView, so it is a half-implemented v-model. Vue 3.4+ (pinned: 3.5.39) expresses exactly this - child-owned state mirrored to the parent - with `defineModel`:

- JobsView: `const runActive = defineModel<boolean>("runActive", { default: false });` replaces the `ref(false)` plus the whole watch-forwarder block; the `"update:runActive"` entry leaves `defineEmits`. All existing `runActive.value` assignments stay as-is.
- App: `v-model:run-active="jobsRunActive"` replaces the `:pending-run`-adjacent `@update:run-active` handler. App binds a real `ref(false)`, so the documented default-desync caveat does not apply.
- The watcher's own comment ("so every mutation is covered without having to remember to emit") describes precisely what `defineModel` does automatically.

lines_cut: 6, deps_cut: 0.

### 2. idiom - pre-3.5 template-ref pattern instead of `useTemplateRef` (4 sites; anchor src/App.vue:20)

`const x = ref<InstanceType<typeof C> | null>(null)` / `ref<HTMLElement | null>(null)` matched by template `ref="x"` is the documented pre-3.5 pattern. Vue 3.5 (pinned) recommends `useTemplateRef('x')`, which also infers the element/component type from the template (vue-tsc 3.3.7 supports the inference). Sites:

- src/App.vue:20 (`settingsDialog`)
- src/components/SettingsDialog.vue:13 (`dialogEl`)
- src/components/LiveLog.vue:40 (`logEl`)
- src/views/JobsView.vue:64 (`runHistoryRef`)

Pure replacement; the manual `InstanceType<...> | null` annotations disappear. lines_cut: 0, deps_cut: 0.

### 3. native - `dirname(fileURLToPath(import.meta.url))` instead of `import.meta.dirname` (3 sites; anchor e2e/mocks.ts:25)

Node 26.5 (pinned) provides `import.meta.dirname` natively. Verified to work in all three execution contexts involved: Playwright 1.61 ESM loader (empirical run), Vite 8 config loading (shim confirmed in vite dist), plain node ESM. Sites:

- e2e/mocks.ts:25-28 -> `const HARNESS_PATH = resolve(import.meta.dirname, ".generated/tauri-mock-harness.js");` (drops the `fileURLToPath` import and `dirname` from `node:path`)
- e2e/i18n-en.ts:58 -> `resolve(import.meta.dirname, "../locales")` (drops `fileURLToPath` import line, `dirname` from the path import)
- e2e/vite.harness.config.ts:15 -> delete `const here = ...` and lines 1-2's `dirname`/`fileURLToPath` imports; use `resolve(import.meta.dirname, ...)` at lines 19/23.

lines_cut: 6, deps_cut: 0.

### 4. idiom - hand-mirrored `@tauri-apps/api` signatures instead of `typeof` type-imports (e2e/global.d.ts:16)

`window.__muxsmithE2E__`'s member types re-declare the pinned API's function signatures by hand (lines 16-22), which can drift from the real `@tauri-apps/api` on a version bump (and already differs: the real `emit` is generic `<T>(event: string, payload?: T)`, the real `mockIPC` callback takes `payload?: InvokeArgs`). Type-only imports are erased at compile time, so the file's own constraint (code consumed inside `page.addInitScript`-serialized functions must not have runtime imports) does not apply:

```ts
import type { clearMocks, mockIPC, mockWindows } from "@tauri-apps/api/mocks";
import type { emit } from "@tauri-apps/api/event";
// ...
__muxsmithE2E__: {
  mockIPC: typeof mockIPC;
  mockWindows: typeof mockWindows;
  clearMocks: typeof clearMocks;
  emit: typeof emit;
};
```

This is the exact lockstep-with-the-pinned-version property the harness's own docs argue for. lines_cut: 0, deps_cut: 0.

### 5. idiom - `withDefaults` on Vue 3.5 instead of reactive props destructure (src/views/BatchView.vue:53)

`const props = withDefaults(defineProps<{ runActive?: boolean }>(), { runActive: false });` is the 3.4-and-below form. Pinned Vue 3.5.39 (destructure enabled by default; no `propsDestructure` override in vite.config.ts):

```ts
const { runActive = false } = defineProps<{ runActive?: boolean }>();
```

`props.runActive` in `runDisabledReason` becomes plain `runActive`. lines_cut: 0, deps_cut: 0.

### 6. idiom - catalog-completeness guard as an import side effect instead of a test (e2e/i18n-en.ts:136)

`assertAllCatalogsParseCleanly()` runs at module import time "for its throw side effect". Playwright's idiom for a run-gating invariant is a named test (or `globalSetup`): as an import side effect, a broken catalog fails every test in every importing spec with an opaque module-load error instead of one clearly named failing test, and the full multi-locale parse re-runs in every parallel worker that imports the module. Concrete replacement: keep `buildEnBundle()` at module scope (the `en()` helper legitimately needs the memoized bundle) and move the all-locales guard into a dedicated spec, e.g. `e2e/catalogs.spec.ts` with `test("all Fluent catalogs parse cleanly", () => assertAllCatalogsParseCleanly());`. The guard's intent ("fails the whole e2e run") is preserved - the suite still goes red - with correct attribution. lines_cut: 0, deps_cut: 0.

### 7. native - manual JSON round-trip over `page.exposeFunction` (e2e/mocks.ts:101, 148)

The page side calls `window.__muxsmithRecordInvoke__?.(cmd, JSON.stringify(args ?? null))` and the node side does `JSON.parse(argsJson)`. Playwright's `exposeFunction` already serializes arguments itself (at call time, in the page context - the same snapshot semantics), and Tauri invoke args are JSON-shaped by construction. Pass `args ?? null` directly and drop the parse; `global.d.ts`'s `__muxsmithRecordInvoke__` signature becomes `(cmd: string, args: unknown) => void`. lines_cut: 0, deps_cut: 0.

### 8. yagni - dead null tolerance across the locale bootstrap (src/i18n/index.ts:79, src/main.ts:16)

`resolveLocale()` is typed `Promise<string | null>` but both branches coalesce to `navigator.language` (a string), so `null` is unproducible; `buildBundles(locale: string | null | undefined)` then defends with a type-guard filter (`.filter((tag): tag is string => typeof tag === "string" && tag.length > 0)`) against exactly the inputs its only caller has already normalized away. Even the residual empty-string case degrades identically without the filter (`primarySubtag("") === ""` matches no catalog directory, `buildBundle` returns null, chain falls to "en"). Concrete replacement: `resolveLocale(): Promise<string>`, `buildBundles(locale: string)`, delete the filter line (keep the `Set` dedup, which still handles `locale === "en"`). Null-handling stays at the one boundary that has a real null: the `?? navigator.language` in main.ts. lines_cut: 1, deps_cut: 0.

## Routed (not findings; incidental observations for the correctness owner)

- src/diagnosticFluentParams.ts:26 - the doc comment claims usize-parity strictness ("rejects negative numbers, floats, empty strings, and scientific notation"), but `Number()`-based parsing accepts `"1e3"` (-> 1000), `"0x10"` (-> 16), and whitespace-padded `" 42 "`, all of which pass `Number.isInteger(n) && n >= 0` and get promoted. Behaviorally harmless today (params arrive as canonical Rust `usize::to_string()` output), but the comment overclaims; `/^\d+$/.test(raw)` would deliver the documented strictness.

## Explicitly considered and not flagged

- `primarySubtag` hand-roll vs `Intl.Locale`/`@fluent/langneg`: `new Intl.Locale()` throws on malformed input (a hand-edited settings value must not crash the bootstrap), and `@fluent/langneg` is not an installed dependency - the 2-line split is the dependency-light, more robust choice here.
- `visibleText()` + `textContent()` sync assertions in smoke.spec.ts instead of web-first `toHaveText`: justified - Fluent's U+2066-2069 isolates make `toHaveText` unusable against plain expected strings, and each site is preceded by an auto-retrying visibility/containText assertion.
- LiveLog's JS sticky-bottom scroll: no CSS-native equivalent covers "release when the user scrolls up"; the JS pattern is the ecosystem norm for `role="log"` panes.
- `RunRequest` re-declared in JobsView.vue, `defaultAppSettings()` in two files, `scanIds`/`MESSAGE_ID_RE` mirroring check-i18n.mjs: cross-file duplication, excluded per brief (separate sweep).
- Known non-findings honored: version pins, TS 6.0.3 ceiling, RECENT_PROFILES_CAP TS/Rust duplication (B11), fake-mkvmerge helpers, MUXSMITH_RUNS_ROOT.
- Playwright config judged idiomatic: `forbidOnly` on CI, `reuseExistingServer`, webServer host pin (documented reason), single-project Chromium (documented scope), `list` reporter.
