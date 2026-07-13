# Task 7 review verdict: Frontend + e2e (Stream E)

Base 3d58afe, head c740bf4. Commits: 612be9a (refactor(gui)), c740bf4 (refactor(e2e)).

## Spec Compliance

- ✅ Cross-task constraint: `meets_minimum: true,` deleted from the `MkvmergeInfo` literal, `e2e/smoke.spec.ts:80` (diff shows the line removed, trailing comma on the preceding `version` field still valid). Folded into the `refactor(e2e)` commit as instructed.
- ✅ Item 1 `defaultAppSettings()` dup: exported once from `src/ipc.ts` beside `AppSettings` (`src/ipc.ts:30-40`); `SettingsDialog.vue` and `BatchView.vue` both import it and their local copies are deleted. Field-for-field identical to the removed copies.
- ✅ Item 2 `RunRequest` dup: `JobsView.vue` now does `import type { ... RunRequest } from "../ipc"`, local `interface RunRequest {...}` (with its "parallel-task artifact" comment) deleted.
- ✅ Item 3 `runActive` idiom: `JobsView.vue` replaces `ref(false)` + `watch(...) => emit("update:runActive", ...)` + the `defineEmits` entry with `defineModel<boolean>("runActive", { default: false })`; `App.vue`'s `<JobsView>` binds `v-model:run-active="jobsRunActive"` in place of the old `@update:run-active` listener. App holds a real `ref(false)`, so the brief's default-desync caveat doesn't apply.
- ✅ Item 4 `useTemplateRef`: all four sites converted (`App.vue` settingsDialog, `SettingsDialog.vue` dialogEl, `LiveLog.vue` logEl, `JobsView.vue` runHistoryRef); manual `InstanceType<typeof X> | null` annotations removed at each. Template `ref="..."` attribute names were already identical to the old variable names, so no template edits were needed for the inference to work.
- ✅ Item 5 `resolveLocale`/`buildBundles` yagni: `main.ts:14` returns `Promise<string>`, `i18n/index.ts:79` narrows to `buildBundles(locale: string)`, the type-guard filter line is gone, the `Set` dedup is untouched.
- ✅ Item 6 `import.meta.dirname`: all three sites (`e2e/mocks.ts`, `e2e/i18n-en.ts`, `e2e/vite.harness.config.ts`) converted, `node:url`/unused `dirname` imports dropped at each.
- ✅ Item 7 mocks.ts JSON round-trip: `window.__muxsmithRecordInvoke__?.(cmd, args ?? null)` (no `JSON.stringify`); `page.exposeFunction`'s callback now `(cmd: string, args: unknown) => recorded.push({ cmd, args })` (no `JSON.parse`); `global.d.ts`'s `__muxsmithRecordInvoke__` signature narrowed to `(cmd: string, args: unknown) => void` to match.
- ✅ Item 8 `global.d.ts` typeof-import: `mockIPC`/`mockWindows`/`clearMocks`/`emit` type-only imported from `@tauri-apps/api/mocks` and `@tauri-apps/api/event`, `window.__muxsmithE2E__` fields become `typeof mockIPC` etc., replacing the hand-mirrored (and drifted) inline signatures.
- ✅ Item 9 `e2e/catalogs.spec.ts`: new file with one test (`"all Fluent catalogs parse cleanly"`), `assertAllCatalogsParseCleanly` exported from `i18n-en.ts` and no longer called at module scope; `buildEnBundle()` still runs at module scope as required (the `en()` helper needs it).
- ⚠️ Cannot verify from diff: unsigned commits, explicit staging (no commit metadata / staging record in a diff file; would require `git log`/`git show`, which this review does not run).
- ⚠️ Cannot verify from diff: the full 9-part gate's actual execution (only the report's narrative claims it; the diff itself has no test-run evidence). No reason to doubt it, but it's not something a diff can confirm.

## Strengths

- Complete and faithful to all 9 brief items plus the cross-task constraint; no item skipped, none re-interpreted.
- Two claims independently re-verified against real source/runtime rather than trusted from the brief (`import.meta.dirname` typings against the installed `@types/node`, `page.exposeFunction` serialization fidelity against a real Playwright run) — matches `proc-07-verify-against-source`.
- Clean two-commit split mirroring the file-ownership boundary (`refactor(gui)` for `src/`, `refactor(e2e)` for `e2e/`), matching item 18's commit-naming instruction.
- No scope creep: every touched file is inside the task's owned set (`src/ipc.ts`, `SettingsDialog.vue`, `BatchView.vue`, `JobsView.vue`, `App.vue`, `LiveLog.vue`, `main.ts`, `i18n/index.ts`, `e2e/*`); `src-tauri/` and `locales/` untouched.
- Doc-comment drift caught and fixed as a direct consequence of the code changes it describes (`i18n-en.ts`'s "runs for its throw side effect" sentence, `JobsView.vue`'s banner comment about the old watch-forwarder) rather than left stale or silently rewritten beyond what the change required.
- The disclosed "mechanical extra" (`v-model:run-active` moved ahead of `:pending-run` in `App.vue`) is judged legitimate: it is the unavoidable, lint-forced consequence of the `defineModel`/`v-model` conversion item 3 explicitly mandated (`vue/attributes-order`'s TWO_WAY_BINDING-before-OTHER_ATTR rule), not an independent editorial choice. Source-attribute order has no runtime effect.

## Issues

### Critical (Must Fix)

None.

### Important (Should Fix)

None.

### Minor (Nice to Have)

- `src/i18n/index.ts:79` / `src/main.ts:14`: removing the type-guard filter (item 5) eliminates the `null`/`undefined` cases but not the empty-string case — `(await getSettings()).locale ?? navigator.language` only substitutes on nullish, not on `""`, so a persisted empty-string locale would reach `buildBundles(["", "en"])` where the old filter would have dropped it to `["en"]` only. `buildBundle("")` (unchanged, not in this diff) most likely resolves to no directory and returns `null`, so the net bundle chain is probably still just `["en"]` — but this isn't verifiable from the diff, and the failure mode if `buildBundle` doesn't handle an empty path gracefully is unverified. Low practical risk: nothing in the diff shows a code path that persists `locale` as `""` rather than `null`. This gap is inherited from the brief's own scoping (the brief's rationale addresses only null/undefined), not an independent implementer choice.

## House dimension

No deviations found against `docs/product-boundaries.yaml`, `docs/conventions.yaml`, or `docs/process-conventions.yaml` — this task is a pure technical-code idiomacy pass (executing the pre-1.0 idiomacy-review gate, `proc-09-idiomacy-review`), touches no product-scope surface, and the two self-disclosed empirical spot-checks reinforce (not violate) `proc-07-verify-against-source`.

Harvested candidates for the ledger (first occurrence each, not yet promotable at agent-emergent's count-3 threshold, worth watching for recurrence):

- **v-model attribute reordering is a mechanical, forced consequence of adopting `defineModel`.** Any future `ref`+emit-forwarder → `defineModel` conversion on a component with other bound attributes will likely need the same `vue/attributes-order` reorder (TWO_WAY_BINDING before OTHER_ATTR/EVENTS). Worth a one-line note in the Vue idiomacy playbook if it recurs on the next such conversion.
- **Import-time side-effect assertions belong in a named test, not a module top-level call.** `e2e/catalogs.spec.ts` converts `i18n-en.ts`'s eager `assertAllCatalogsParseCleanly()` into an explicit `test(...)` so a catalog regression fails attributably. If another module-scope "throws to fail the suite" pattern turns up elsewhere in `e2e/`, this is the precedent to point at.

## Assessment

**Task quality:** Approved
**Reasoning:** All 9 brief items and the cross-task constraint are implemented exactly as specified, verifiably from the diff; the one disclosed mechanical extra is a forced, non-discretionary consequence of an in-scope item; the single Minor finding is a low-probability, brief-inherited edge case with no evidence of a triggering code path.
