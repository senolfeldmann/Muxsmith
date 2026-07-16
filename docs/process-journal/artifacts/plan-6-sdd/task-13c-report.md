# Task 13c report: shared recent-profiles memory, fed and rendered by the editor (spec 8.2 / whole-branch Finding 1)

Worktree `/home/senol/Git/Muxsmith/.worktrees/plan6-e`, branch `plan6-e`, starting HEAD `ed14ade`. Commit: `ef19fd9`.

## What was implemented

Closes whole-branch verdict Finding 1 (HIGH) and the owner's option-1a ruling
(`spec-clause-sweep-at-plan-close`, `docs/decision-ledger.yaml`): the editor
had only a pick button and never fed or rendered the shared
`AppSettings.recent_profiles` MRU memory BatchView maintains.

1. **`src/recentProfiles.ts` (new).** One exported function,
   `rememberRecentProfile(path: string): Promise<AppSettings | null>`,
   verbatim from the brief's binding point: re-fetches via `getSettings()`,
   moves `path` to the front of `recent_profiles` through a module-private
   `withRecentProfile` helper (de-duplicated, capped at a module-private
   `RECENT_PROFILES_CAP = 10`, mirroring `src-tauri/src/settings.rs`'s own
   constant, confirmed identical by reading that file), spreads the rest of
   `AppSettings` unchanged, persists via `setSettings(next)`, returns `next`.
   Swallows any IPC failure, `console.warn`s once (tag: `"[recents] failed
   to persist recent profile:"`), returns `null`.

2. **`src/views/BatchView.vue` (re-point, behavior-identical).** Deleted the
   local `RECENT_PROFILES_CAP` const and the local `rememberRecentProfile`
   function (including its try/catch swallow and MRU+cap expression); added
   `import { rememberRecentProfile } from "../recentProfiles";`; re-pointed
   the single call site inside `selectProfile` from `await
   rememberRecentProfile(path);` to `settings.value = (await
   rememberRecentProfile(path)) ?? settings.value;` -- the `?? settings.value`
   reproduces the old behavior exactly (success: freshly re-fetched+mutated
   settings; swallowed failure: unchanged). `updateSettings`, `persistDir`,
   the `settings` ref, and the recents template are untouched (verified by
   diff, see Review-check below). Only benign consequence: the swallow's
   console tag now reads `"[recents] ..."` instead of `"[batch] ..."` --
   message-only, no test asserts on console output.

3. **`src/views/EditorView.vue` (editor: one funnel, remember on open,
   render pre-Open).**
   - Split `pickAndOpen` into `openPath(path: string)` (the load/diagnostics/
     model body, keeping the `opening.value || saving.value` guard,
     `opening`/`ipcErrorCode`/try/finally exactly as before) plus a thin
     `pickAndOpen` that keeps its own guard, opens the dialog, and calls
     `openPath(picked)` when the dialog returns a string.
   - Inside `openPath`'s `try`, after `model.value = doc.profile ??
     undefined;`: `const persisted = await rememberRecentProfile(path); if
     (persisted) { recents.value = persisted.recent_profiles; }` -- a
     recents-write failure is swallowed inside the shared function and
     never reaches `openPath`'s own `catch`.
   - Added `const recents = ref<string[]>([])` and a tolerant `onMounted`
     that reads `(await getSettings()).recent_profiles`, catching and
     leaving `recents` empty on failure (mirrors BatchView's own `onMounted`
     tolerance).
   - Added imports: `onMounted` (vue), `getSettings` (`../ipc`),
     `rememberRecentProfile` (`../recentProfiles`).
   - Added the recents `<section>` verbatim from the brief's binding point,
     directly after the `<p v-if="currentPath">` open-path line and before
     the `ipcErrorCode` alert paragraph / diagnostics section: `v-if=
     "!currentPath && recents.length"`, `aria-labelledby="editor-recents-
     heading"`, `data-testid="editor-recents"`, heading `<h4
     id="editor-recents-heading">{{ $t("batch-recents-heading") }}</h4>`
     (distinct element id, reused text key -- same pattern as the existing
     `editor-diagnostics-heading`/`batch-diagnostics-heading` precedent),
     each entry a native `<button data-testid="editor-recent-profile"
     :disabled="opening || saving" @click="openPath(path)">{{ path }}
     </button>`.

4. **`e2e/smoke.spec.ts` (additive).** One new `test.describe("editor view:
   recent profiles (Task 13c, spec 8.2 / recents routing)")` block, four
   tests, appended after the existing Task 13b describe block. Nothing else
   in the file touched.

Zero new Fluent catalog keys anywhere: `batch-recents-heading` (already
bilingual, `locales/en/gui-batch.ftl:21` / `locales/de/gui-batch.ftl:16`) is
reused as-is, the seventh reused rendered-surface key per the whole-branch
verdict's pending owner set.

## TDD evidence

**RED** (`pnpm test:e2e`, before any implementation code, after only the
test file was extended): 3 of 4 new tests failed, named:
- `the pre-Open surface renders a seeded recent profile` -- `toHaveCount(1)`
  on `editor-recent-profile` received 0 (no recents list exists yet).
- `clicking a recent opens through the same load_profile funnel as pick` --
  30s timeout waiting for `editor-recent-profile` to exist to click it.
- `opening a profile writes it to the front of the shared recents memory
  (echo, distinct values)` -- `set_settings` recorded-call filter had length
  0 instead of 1 (the editor never calls `rememberRecentProfile`).

The 4th test, the paired absence control (`recent_profiles: []` renders
count 0), passed trivially at RED -- expected and by design: it is
non-vacuous only paired with the presence test above, which the RED run
demonstrably exercises (per the brief's own falsifiability framing). All 28
pre-existing tests stayed green during this RED run.

**GREEN** (`pnpm test:e2e`, after Steps 3-5): 31/31 passed, including all 4
new Task 13c tests and every pre-existing test (widget mount-harness, batch
dry-run, jobs live-run, german locale, Task 11/12/13/13b editor specs)
unmodified.

## Review-check outputs (Step 7, before the full gate)

**BatchView re-point diff -- behavior-identical, confirmed by `git diff`:**

```
diff --git a/src/views/BatchView.vue b/src/views/BatchView.vue
@@ import block @@
+import { rememberRecentProfile } from "../recentProfiles";
@@ (24 lines deleted: the RECENT_PROFILES_CAP const + local rememberRecentProfile fn) @@
@@ selectProfile @@
-  await rememberRecentProfile(path);
+  settings.value = (await rememberRecentProfile(path)) ?? settings.value;
```

`updateSettings`, `persistDir`, the `settings` ref, and the recents template
(`<h4 id="batch-recents-heading">` / the `settings.recent_profiles` `<ul>`)
do not appear anywhere in the diff -- untouched, as required. Full stat: `1
file changed, 2 insertions(+), 24 deletions(-)`.

**`e2e/smoke.spec.ts` -- purely additive:** `git diff --stat` shows `139
insertions(+)`, 0 deletions (the single `-` the raw diff contains is the
`--- a/e2e/smoke.spec.ts` file header, not a content deletion). Grepped the
diff's added lines for `.only(`/`.skip(`/`.fixme(`: none found. No
mount-harness or Task-13 served spec was touched.

**EditorView still mounts from `modelValue` alone:** the file's one
`onMounted` hook calls only `getSettings()` (tolerant try/catch, feeds
`recents`); `loadProfile` has exactly one call site in the whole file,
inside `openPath`, reached only via a user action (`pickAndOpen` after the
dialog resolves, or a recents-button click) -- never from mount. Confirmed
by grep (`loadProfile(` -> 1 hit; `onMounted` -> the recents-read block
only). The mount-harness specs (Tasks 10-13b), which install no Tauri IPC
mock, stayed green in the same `pnpm test:e2e` run -- the tolerant
`onMounted` swallows the mock-less `getSettings()` failure exactly as
designed.

## Gate results (all foreground, all green)

1. `cargo fmt --all --check` -- clean, exit 0.
2. `cargo clippy --workspace --all-targets -- -D warnings` -- clean, exit 0.
3. `cargo test --workspace` -- all passed (81 `muxsmith-core` lib tests +
   `muxsmith-cli`/`muxsmith-gui`/`xtask` suites + doc-tests), 0 failed.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- clean,
   exit 0.
5. `cargo deny check` -- `advisories ok, bans ok, licenses ok, sources ok`.
6. `pnpm lint` (`eslint .`) -- clean, exit 0 (D27 `no-raw-text` included;
   every added string is a `$t(...)` call or the model-derived `path`).
7. `pnpm build` (`vue-tsc --noEmit && vite build`) -- clean, exit 0.
8. `pnpm check:i18n` -- `ok (36 source files scanned, 233 catalog ids, 17
   unused warning(s), 1 other locale(s) checked for parity against 7 en/
   catalog(s))`. All 17 warnings are pre-existing dynamic-`$t()` IpcError
   codes, unrelated to this task, which adds no catalog key.
9. `pnpm test:e2e` -- **31/31 passed**, 1.8s.

## Files changed

- `src/recentProfiles.ts` (new, 45 lines): the shared module.
- `src/views/BatchView.vue`: 2 insertions, 24 deletions (re-point only).
- `src/views/EditorView.vue`: 87 insertions, 13 deletions (funnel split,
  recents state/mount-read, template section).
- `e2e/smoke.spec.ts`: 139 insertions, 0 deletions (one additive describe
  block, 4 tests).

Commit `ef19fd9`, unsigned (`commit.gpgsign=false`), trailer
`Co-Authored-By: Claude Sonnet <noreply@anthropic.com>`, explicit staging of
exactly these four files (no `git add -A`).

## Self-review against the brief's binding points

- **Zero new catalog keys:** confirmed -- no `locales/` file appears in
  `git status`; `check:i18n`'s unused-key list is unchanged from the
  pre-existing 17. `batch-recents-heading` is reused, not minted.
- **Distinct element id, reused text key:** confirmed by grep --
  `editor-recents-heading` (EditorView's element id) and
  `batch-recents-heading` (BatchView's element id) never collide;
  `$t("batch-recents-heading")` is the shared text key both `<h4>`s render.
- **One funnel:** confirmed -- `openPath` has exactly two call sites in
  EditorView.vue (`pickAndOpen`'s `await openPath(picked);` and the recents
  button's `@click="openPath(path)"`), and `loadProfile` is called nowhere
  else.
- **Behavior-identity diff clean:** confirmed above (Review-check section);
  `updateSettings`/`persistDir`/BatchView's recents template are absent from
  the diff, and the `?? settings.value` reconstruction reproduces the old
  success/failure branches exactly, with `dir_memory` read ordering in
  `selectProfile` unchanged (that line precedes the re-pointed call site,
  untouched).
- **TDD named RED:** confirmed above, three distinct named failures matching
  the brief's Step 2 expectation, plus the deliberately-vacuous-until-paired
  4th test passing for the documented reason.

## Concerns

None outstanding. One judgment call, flagged for visibility rather than
silently absorbed: the brief's Step 1 said "queue one `get_settings`
response per expected call"; I queued a single response per test instead
(mirroring the existing "german locale" describe block's own pattern one
section above). `mocks.ts`'s own documented queue semantics ("once
exhausted, the last entry repeats for any further call") make a
single-entry queue behaviorally identical to an N-entry queue of the same
value here, since nothing mutates the seeded `recent_profiles` between the
editor's mount read and `rememberRecentProfile`'s re-fetch in any of these
four tests -- confirmed by the GREEN run exercising exactly that path
(assertion 3's echo test reads the re-fetched value and still produces the
correct `[OPENED_PATH, RECENT_PATH]` write). No behavior gap; noted only
because the brief's wording suggested a literal per-call queue and I
verified the discrepancy is only cosmetic before proceeding rather than
assuming.
