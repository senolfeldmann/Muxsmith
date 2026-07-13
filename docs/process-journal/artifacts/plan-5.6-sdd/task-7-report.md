# Task 7 report: Frontend + e2e (Stream E)

Branch: `plan-5.6-e`, worktree `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-e`.
Commits: `612be9a` (refactor(gui)), `c740bf4` (refactor(e2e)).

## Cross-task constraint (done first)

Deleted `meets_minimum: true,` from the `MkvmergeInfo` mock literal in
`e2e/smoke.spec.ts:80`. The preceding task (T6, commit `3d58afe`) removed
`meets_minimum` from the Rust `MkvmergeInfo` and its `src/ipc.ts` mirror;
this literal was the sole remaining reference and the only reason
`pnpm test:e2e` was red on the branch. Included in the `refactor(e2e)`
commit since it lives in an e2e file exclusive to this task.

## Items implemented

1. **`defaultAppSettings()` dup** (`src/ipc.ts`, `SettingsDialog.vue:24-32`,
   `BatchView.vue:26-34`): exported one `defaultAppSettings()` from
   `ipc.ts` beside `AppSettings`; both components now import it, their
   identical local copies deleted. Verified the returned literal still
   matches `src-tauri/src/settings.rs::AppSettings::default()` field-for
   -field (`mkvmerge_path: None`, `default_jobs: default_jobs() == 1`,
   `locale: None`, `recent_profiles: Vec::new()`, `dir_memory:
   HashMap::new()`).
2. **`JobsView.vue` `RunRequest` dup**: replaced the local re-declared
   `interface RunRequest` (and its "parallel-task artifact" comment) with
   `import type { ... RunRequest } from "../ipc"`.
3. **`runActive` idiom** (`JobsView.vue` + `App.vue`): `runActive` is now
   `defineModel<boolean>("runActive", { default: false })`, replacing the
   local `ref(false)` + the `watch(runActive, ...) => emit("update:
   runActive", ...)` forwarder + the `update:runActive` defineEmits entry.
   `App.vue`'s `<JobsView>` now binds `v-model:run-active="jobsRunActive"`
   (attribute reordered ahead of `:pending-run` to satisfy
   `vue/attributes-order`'s TWO_WAY_BINDING-before-OTHER_ATTR rule, caught
   by `pnpm lint`). App already held a real `ref(false)`, so the
   default-desync caveat named in the brief doesn't apply.
4. **`useTemplateRef` idiom**, all four sites: `App.vue` (`settingsDialog`),
   `SettingsDialog.vue` (`dialogEl`), `LiveLog.vue` (`logEl`),
   `JobsView.vue` (`runHistoryRef`). Manual `InstanceType<typeof X> | null`
   annotations dropped; `vue-tsc 3.3.7` infers the element/component type
   from the matching `ref="..."` in the template (`pnpm build` confirms
   zero type errors). Grepped `src/` afterward for any remaining
   `InstanceType<typeof` template-ref pattern: none left.
5. **`resolveLocale`/`buildBundles` yagni** (`main.ts`, `i18n/index.ts`):
   `resolveLocale(): Promise<string>` (both branches already coalesce to
   `navigator.language`); `buildBundles(locale: string)`, dropped the
   `.filter((tag): tag is string => ...)` type-guard line; kept the `Set`
   dedup (still needed for `locale === "en"`). Grepped for other callers
   of `buildBundles`/`resolveLocale`: none, so the narrower signature has
   exactly one call site each.
6. **`import.meta.dirname` native**, three sites (`mocks.ts`, `i18n-en.ts`,
   `vite.harness.config.ts`): replaced `resolve(dirname(fileURLToPath(
   import.meta.url)), ...)` with `resolve(import.meta.dirname, ...)`,
   dropped the now-unused `node:url` `fileURLToPath` import and the
   `dirname` import from `node:path` at all three sites (confirmed each
   was the only usage via grep before editing). Verified typing support:
   `node_modules/@types/node/web-globals/importmeta.d.ts` declares
   `dirname: string` on the global `ImportMeta` interface (package version
   26.1.1, matching the pinned Node 26.5.0 runtime); `node --input-type
   =module -e "console.log(import.meta.dirname)"` confirmed the runtime
   value resolves correctly.
7. **`mocks.ts` JSON round-trip drop** (`:101`/`:148` in the pre-edit
   file): replaced `window.__muxsmithRecordInvoke__?.(cmd,
   JSON.stringify(args ?? null))` with `window.__muxsmithRecordInvoke__?.
   (cmd, args ?? null)`, and `page.exposeFunction`'s callback now takes
   `(cmd: string, args: unknown)` and pushes `args` directly (no
   `JSON.parse`). Empirically verified (not just trusted from the brief)
   that `page.exposeFunction` serializes the argument itself with
   richer-than-JSON fidelity: a throwaway script driving the real
   `playwright-core@1.61.1` (`chromium.launch()` + `exposeFunction`)
   showed `undefined` object-field values and a whole `undefined`
   argument both survive the Node<->page boundary intact (JSON would
   drop the former and cannot represent the latter as a function
   argument at all) — see scratch check, not committed.
8. **`global.d.ts` typeof-import idiom**: `mockIPC`/`mockWindows`/
   `clearMocks`/`emit` on `window.__muxsmithE2E__` are now `typeof
   mockIPC` etc., type-only-imported from `@tauri-apps/api/mocks` and
   `@tauri-apps/api/event`. Verified against the shipped `.d.ts`: the real
   `mockIPC` signature is `(cb: (cmd: string, payload?: InvokeArgs) =>
   unknown, options?: MockIPCOptions) => void` — the hand-mirror's
   `payload` had drifted to `args?: unknown` and the inline options type
   had drifted from the named `MockIPCOptions`, confirming the brief's
   "hand-mirror had already drifted" claim. `mocks.ts`'s existing call
   sites still type-check (parameter names are irrelevant structurally;
   confirmed via `pnpm build`/`tsc --noEmit -p e2e/tsconfig.json`).
9. **`e2e/catalogs.spec.ts`** (new file): moved the all-locales parse
   guard out of `i18n-en.ts`'s module-import side effect.
   `assertAllCatalogsParseCleanly` is now `export`ed and no longer called
   at module scope; `buildEnBundle()` stays at module scope (the `en()`
   helper needs its memoized bundle regardless of which spec runs). The
   new spec has one test: `test("all Fluent catalogs parse cleanly", () =>
   assertAllCatalogsParseCleanly())`. Updated the stale doc-comment
   sentence in `i18n-en.ts` that claimed the guard "runs for its throw
   side effect at import time" (no longer true for the guard, still true
   for `buildEnBundle`).

## Gate results (all nine parts, run in the worktree before committing)

- `cargo fmt --all --check`: clean (no output).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo test --workspace`: all suites green (78 unit tests in
  `muxsmith-gui`, plus core/cli/xtask suites), no failures. Untouched by
  this task; ran for completeness per BUILDING.md.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `pnpm lint`: clean.
- `pnpm build` (vue-tsc --noEmit && vite build): clean, 82 modules,
  built in ~130ms.
- `pnpm check:i18n`: ok (179 catalog ids, 12 pre-existing "unused"
  warnings — dynamically-rendered diagnostic codes, unrelated to this
  task's edits — 1 other locale checked for parity).
- `pnpm test:e2e` (foreground): **7/7 passed**, including the new
  `catalogs.spec.ts` test. This was RED before the cross-task constraint
  fix (stray `meets_minimum` field failing the TS build step of
  `test:e2e`); now green.

Re-ran `pnpm lint` / `pnpm build` / `pnpm check:i18n` / `pnpm test:e2e`
a second time after both commits landed, working tree clean — identical
results.

## Files changed

- `src/ipc.ts` — `defaultAppSettings()` export.
- `src/components/SettingsDialog.vue` — dedup default settings, template
  ref.
- `src/views/BatchView.vue` — dedup default settings.
- `src/views/JobsView.vue` — `RunRequest` import, `defineModel`,
  template ref.
- `src/App.vue` — `v-model:run-active`, template ref.
- `src/components/LiveLog.vue` — template ref.
- `src/main.ts` — `resolveLocale` return type.
- `src/i18n/index.ts` — `buildBundles` parameter type + dead filter.
- `e2e/smoke.spec.ts` — cross-task constraint (`meets_minimum` deletion).
- `e2e/mocks.ts` — `import.meta.dirname`, drop JSON round-trip.
- `e2e/i18n-en.ts` — `import.meta.dirname`, export+relocate parse guard.
- `e2e/vite.harness.config.ts` — `import.meta.dirname`.
- `e2e/global.d.ts` — typeof-imported mock signatures, narrowed
  `__muxsmithRecordInvoke__`.
- `e2e/catalogs.spec.ts` — new file, one test.

## Self-review

- **Completeness**: all 9 checklist items plus the cross-task constraint
  implemented; grepped for stray leftovers of each removed pattern
  (`InstanceType<typeof`, local `RunRequest`/`defaultAppSettings`
  duplicates, `dirname`/`fileURLToPath` usages) — none found.
- **Quality**: two claims that looked like "trust the brief" spots were
  independently re-verified against source/runtime rather than taken on
  faith (per house convention proc-07): `import.meta.dirname` typing
  (`@types/node` 26.1.1's `web-globals/importmeta.d.ts`) and
  `page.exposeFunction`'s serialization fidelity (empirical script
  against the real `playwright-core@1.61.1`, not committed — deleted
  after use).
- **Discipline**: no restructuring beyond the brief. The one
  editorial call was updating two stale doc-comment sentences that the
  moved/changed code left factually wrong (`i18n-en.ts`'s "runs for its
  throw side effect at import time" claim, and `JobsView.vue`'s top
  banner comment describing the old watch-forwarder) — both are
  direct consequences of the item's own change, not scope creep, and are
  called out here rather than silently folded in.
- **Test output**: `pnpm test:e2e` output is pristine — 7/7 passed, no
  warnings, no flaky retries observed across two full runs.

## Surfaced patterns / deviations for the house ledger

- No new pattern, restraint, or deviation to escalate. All nine items
  were mechanical fixes matching an already-verified brief; the two
  empirical spot-checks (import.meta.dirname typings,
  page.exposeFunction fidelity) confirmed the brief's claims rather than
  overturning them, so nothing here rises to a new `agent-emergent`
  entry under `docs/conventions.yaml`'s promotion rule.
- One attribute-order fix not explicitly named in the brief:
  `App.vue`'s `<JobsView>` needed `v-model:run-active` placed before
  `:pending-run` to satisfy `eslint-plugin-vue`'s `vue/attributes-order`
  rule (TWO_WAY_BINDING before OTHER_ATTR). Purely mechanical
  consequence of introducing `v-model`, caught by `pnpm lint` itself, not
  a design decision.
