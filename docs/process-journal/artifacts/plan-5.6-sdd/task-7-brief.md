### Task 7: Frontend + e2e (Stream E, after T6 - shares src/ipc.ts)

**Files:**
- Modify: `src/ipc.ts`, `src/components/SettingsDialog.vue`, `src/views/BatchView.vue`, `src/views/JobsView.vue`, `src/App.vue`, `src/components/LiveLog.vue`, `src/main.ts`, `src/i18n/index.ts`, `e2e/mocks.ts`, `e2e/i18n-en.ts`, `e2e/vite.harness.config.ts`, `e2e/global.d.ts`
- Create: `e2e/catalogs.spec.ts`

**Interfaces:** exports `defaultAppSettings()` from src/ipc.ts.

- [ ] `SettingsDialog.vue:24-32` + `BatchView.vue:26-34` **dup** - export defaultAppSettings() from src/ipc.ts beside the AppSettings interface; import in both components (the TS-vs-Rust default mirror itself is boundary-necessary; only the two TS copies were the finding).
- [ ] `JobsView.vue:35-40` **dup** - `import type { RunRequest } from "../ipc";`, delete the local re-declaration (the in-file comment marks it a parallel-task artifact).
- [ ] `JobsView.vue:205-210` + `App.vue:114` **idiom** - `const runActive = defineModel<boolean>("runActive", { default: false })` replaces the local ref + watch-forwarder + update:runActive emit entry; App.vue binds `v-model:run-active="jobsRunActive"` (App binds a real ref(false), so the default-desync caveat does not apply).
- [ ] `App.vue:20`, `SettingsDialog.vue:13`, `LiveLog.vue:40`, `JobsView.vue:64` **idiom** - `useTemplateRef('...')` at all four sites; the manual `InstanceType<...> | null` annotations disappear via template-based inference (vue-tsc 3.3.7 supports it).
- [ ] `src/main.ts:16` + `src/i18n/index.ts:79-81` **yagni** - resolveLocale(): `Promise<string>` (both branches already coalesce to navigator.language); buildBundles(locale: string); delete the type-guard filter line; keep the Set dedup (still handles locale === "en"); null-handling stays only at the real boundary (`?? navigator.language` in main.ts).
- [ ] `e2e/mocks.ts:25-28`, `e2e/i18n-en.ts:58`, `e2e/vite.harness.config.ts:15` **native** - `resolve(import.meta.dirname, ...)`; drop the node:url fileURLToPath imports and dirname where unused (stable since Node 20.11; verified under Playwright 1.61's ESM loader and Vite 8's config shim).
- [ ] `e2e/mocks.ts:101/:148` **native** - drop the manual JSON.stringify/parse round-trip through page.exposeFunction (it serializes arguments itself with the same snapshot semantics): `window.__muxsmithRecordInvoke__?.(cmd, args ?? null)`, push the received value directly; global.d.ts signature becomes `(cmd: string, args: unknown) => void`.
- [ ] `e2e/global.d.ts:16-22` **idiom** - typeof-import the pinned signatures: `import type { clearMocks, mockIPC, mockWindows } from "@tauri-apps/api/mocks"; import type { emit } from "@tauri-apps/api/event";` then `mockIPC: typeof mockIPC;` etc. - lockstep by construction (the hand-mirror had already drifted; type-only imports are erased, so the addInitScript-serialization constraint does not apply).
- [ ] `e2e/i18n-en.ts:136` **idiom** - move the all-locales parse guard out of module-import side effect into new `e2e/catalogs.spec.ts`: `test("all Fluent catalogs parse cleanly", () => assertAllCatalogsParseCleanly())`; keep buildEnBundle() at module scope (the en() helper needs the memoized bundle). Failure attribution becomes one named red test instead of opaque module-load errors in every spec.
- [ ] Full gate (pnpm lint / build / check:i18n / test:e2e are the load-bearing parts here); commits `refactor(gui): ...` / `refactor(e2e): ...`.

