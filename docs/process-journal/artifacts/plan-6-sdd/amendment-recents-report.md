# Amendment 5 report: Task 13c - the shared recents memory, fed and rendered by the editor

**Status:** DONE. One file changed: `docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md`.
**Mechanism in one sentence:** extract BatchView's `rememberRecentProfile` round trip into a shared `src/recentProfiles.ts`, re-point BatchView to it behavior-identically, and have the editor remember a profile on Open and render a recents list reusing the existing `batch-recents-heading` Fluent key (zero new catalog keys).

Authoring anchored against the plan in the main checkout (master `42893c2`) and the code in worktree `plan6-e`. `plan6-e` HEAD is still `bf46932`, but its working tree carries the uncommitted eight-item fix wave (all eight fix-wave files show `M`); anchors are therefore written as quoted content, marked re-verify-at-dispatch.

## Module design (tree evidence, incl. the real settings mechanism)

`src/recentProfiles.ts` (flat in `src/`, matching the house's existing flat shared modules `jobRowState.ts` and `ipc.ts` - verified `ls src/`). Public surface: one function.

```ts
export async function rememberRecentProfile(path: string): Promise<AppSettings | null>
```

It re-fetches via `getSettings()`, moves `path` to the front of `recent_profiles` (dedup + cap at a module-private `RECENT_PROFILES_CAP = 10`), spreads the rest of `AppSettings` unchanged (never clobbers `mkvmerge_path`/`default_jobs`/`locale`/`dir_memory`), persists via `setSettings(next)`, returns `next`; on any IPC failure it swallows, `console.warn`s once, and returns `null`. The MRU computation is a module-private `withRecentProfile(recents, path)` (not exported - single internal caller, scale-appropriate).

**The real settings mechanism (verified in the tree, `src/ipc.ts:249-254`):**
- `getSettings()` -> `invoke<AppSettings>("get_settings")`
- `setSettings(settings)` -> `invoke<void>("set_settings", { settings })`

This is exactly what BatchView already persists recents through today (its `updateSettings` helper wraps the same two calls). `AppSettings.recent_profiles: string[]` is declared in `src/ipc.ts`. `RECENT_PROFILES_CAP = 10` mirrors `src-tauri/src/settings.rs::RECENT_PROFILES_CAP` (D27), carried verbatim from BatchView's own mirror comment.

**Why return `AppSettings | null` (not `string[]` or `void`):** BatchView renders `settings.recent_profiles` AND uses `settings.dir_memory`, so it needs the full re-fetched settings back to keep its `settings.value` ref coherent (byte-for-byte what its old `updateSettings` did via `settings.value = next`). The editor takes `.recent_profiles` off the return. `null` on failure preserves both callers' "a failed recents write never blocks the pick/open" tolerance without forcing each caller to wrap a throwing call.

## Every plan hunk

1. **Architecture line (line 7), ripple:** appended a clause stating the post-verdict Task 13c closes spec 8.2's recents clause after the fix wave, is a delta not a fifth build wave, leaves Task 14 the last wave-dependent task, and does not alter the wave structure. Closes the wave-structure fork the new phase would otherwise open.

2. **Amendment 5 block (after the Amendment 4 block, before Wave 1):** 4 sentences (within the five-sentence limit). States Finding 1, the option-1a owner ruling, the shared-module + editor-render + `batch-recents-heading` reuse (zero new keys), the post-verdict sequencing, and the `(amended 2026-07-16, recents routing)` marker.

3. **Task 13c section (after Task 14, before "Triggers this plan creates"):** a `## Post-verdict amendment` framing paragraph plus the `### Task 13c` task. Full contents: re-verify-at-dispatch note; Files (create `src/recentProfiles.ts`, modify `BatchView.vue`/`EditorView.vue`, extend `e2e/smoke.spec.ts`); Interfaces; Read-first; seven binding points; seven checkbox steps (RED test, run-red, module, BatchView re-point, editor wiring, run suite, gate+commit). The full module body and the editor recents `<section>` HTML are written out; the BatchView refactor is enumerated (moves/stays/changes); the e2e assertions are specified against the real `installTauriMocks`/`recorded`/`resolveWith` harness.

## Premises verified against the trees

| Premise | Result |
|---|---|
| plan6-e HEAD vs fix wave | HEAD `bf46932`; fix wave uncommitted in working tree (8 files `M`, matching the verdict's fix-wave list) - anchors written as content, re-verify-at-dispatch |
| BatchView `rememberRecentProfile` + `RECENT_PROFILES_CAP` + `updateSettings` | present; `rememberRecentProfile` uses `updateSettings`; `updateSettings` also used by `persistDir` (so it stays) |
| BatchView single call site | `selectProfile`: `await rememberRecentProfile(path);` directly before `await runValidate();`; `pickProfile` funnels through `selectProfile` |
| settings IPC mechanism | `getSettings`->`get_settings`, `setSettings`->`set_settings` (`ipc.ts:249-254`) |
| EditorView `pickAndOpen` | dialog -> `loadProfile(picked)` -> set `currentPath`/`diagnostics`/`model`; no recents write, no onMounted |
| EditorView pre-Open surface | Open button + `<p v-if="currentPath">` + diagnostics section; `<template v-if="model">` holds the form |
| reuse-key-with-distinct-id precedent | `<h3 id="editor-diagnostics-heading">{{ $t("batch-diagnostics-heading") }}</h3>` - the exact pattern the recents heading copies |
| App.vue keeps views mounted | `v-show` (App.vue:98-104 rationale, confirmed in Task 13 binding point) -> BatchView's `id="batch-recents-heading"` is always live -> editor MUST use a distinct id |
| `batch-recents-heading` exists bilingual | en `gui-batch.ftl:21`, de `gui-batch.ftl:16` -> reuse adds zero new keys |
| no recents key in gui-editor | confirmed absent |
| e2e mock harness | `installTauriMocks(page,{commands:{cmd:[resolveWith(...)]}})` returns `recorded` (`{cmd,args}[]`); dialog = `plugin:dialog|open`; `set_settings` args = `{settings}`; `recorded.filter(r=>r.cmd===...)` + `args as {...}` is the established assertion shape (T14 test) |
| Task-14 "last" claim | Task 14 says only "Task 13b is the new last wave-3 task" (wave-3 ordering); no absolute-last claim -> no edit needed; 13c is post-verdict, not wave-3 |
| ASCII / AI-tell glyphs | my additions clean; the lone em-dash grep hit (line 741) is pre-existing Task-7 typography-check text |

## The reused-key enumeration (seven; `batch-recents-heading` is the seventh)

Grounded against the tree, not borrowed:
1. `batch-profile-pick` - editor Open button (`EditorView.vue:366`)
2. `batch-profile-current` - open-path line (`:369`)
3. `batch-profile-filter-name` - dialog filter (`:170`)
4. `settings-save` - Save button (`:479`)
5. `batch-diagnostics-heading` - diagnostics heading, distinct id `editor-diagnostics-heading` (`:380`)
6. `batch-profile-heading` - nav tab (`App.vue:101`)
7. `batch-recents-heading` - recents heading, distinct id `editor-recents-heading` - **added by Task 13c**

Matches the verdict's §c enumeration ("six reused keys ... `batch-recents-heading` becomes a seventh reused key"). Zero NEW keys.

## Forks closed

- **Duplicate-id fork:** reuse only the Fluent text key, use a distinct element id (`editor-recents-heading`), because App.vue's `v-show` keeps BatchView's `batch-recents-heading` id live. A naive "reuse the id too" is an axe violation and ambiguous `aria-labelledby`.
- **Reused-key-scope fork:** the editor reuses exactly `batch-recents-heading`; it deliberately does NOT reuse `batch-recents-empty` or `batch-recents-select-tooltip` (that would expand the rendered-surface delta past the one key the owner scoped). Empty state is handled by `v-if="recents.length"` (nothing renders when empty); recent entries carry a model-derived `{{ path }}` label (no tooltip). If an empty-state string or tooltip is later wanted -> NEEDS_CONTEXT, not a keyboard decision.
- **Wave-structure fork:** the architecture line and the Amendment 5 block both state 13c is a post-verdict delta, not a fifth build wave, and does not disturb Task 14's last-wave-dependent standing.
- **Cross-view staleness fork:** editor recents writes are visible to BatchView on its next settings re-fetch (its existing pattern); the rendered-list live-refresh rides the already-deferred Task-14 auto-refresh / apply-vs-editor-concurrency items. Not a new fork.
- **Regression fork:** EditorView's new `onMounted` recents read is tolerant, so existing served tests that do not mock `get_settings` render recents empty and hide the section - the Task-13 served test is unaffected.

## Falsifiability

- **Absence check paired control:** assertion 1 asserts `getByTestId("editor-recent-profile")` present with seeded recents AND count 0 with empty recents, on the same selector; the RED run (Step 2) exercises the present branch before the list exists, so neither half passes vacuously.
- **Echo-mock distinct values (`echo-mock-distinct-fixture-values`):** `RECENT_PATH` != `OPENED_PATH`; the write assertion checks `set_settings`'s recorded `recent_profiles[0] === OPENED_PATH` and the load assertion checks `load_profile`'s recorded `path === RECENT_PATH` - using the wrong value fails an assertion in either direction. Asserted against recorded IPC calls, not a UI echo.

## Concerns / re-verify-at-dispatch

- **Line-number drift:** the fix wave is landing uncommitted in `plan6-e`; every code anchor in Task 13c is quoted content, and the section carries an explicit re-verify-at-dispatch banner. The implementer matches text, not numbers.
- **`get_settings` queue depth:** the editor calls `get_settings` twice per open scenario (mount read + `rememberRecentProfile` re-fetch). The test note says to queue one response per expected call and re-verify the harness's queue-vs-reuse semantics at dispatch (I did not read `installTauriMocks`'s internals; the T14 test shows multi-entry queues are consumed in order).
- **Behavior-identity of the BatchView re-point:** verified by reading the current `rememberRecentProfile`/`updateSettings`/`selectProfile` - the `?? settings.value` reconstruction preserves the success-and-failure settings.value semantics and the `dir_memory`-read ordering. Step 7's review-check pins this with a `git diff` gate.
- **Console warn tag change** ("[batch]" -> "[recents]") is message-only; no test asserts on console.

## Source

- Originating session: `session_01Ue3Jyb9LRnf6ZYLNEbmeCz`
- Created: 2026-07-16
- Series: Plan 6 amendment 5 (Task 13c, recents routing)
