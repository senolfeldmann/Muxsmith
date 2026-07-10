# Task 12 report: Playwright smoke + i18n completeness gate + CI finish

## What was implemented

### Mechanics verified before writing anything (context7 + direct source reads, not memory)

- `@tauri-apps/api/mocks`' `mockIPC(cb, { shouldMockEvents: true })` (confirmed
  via context7's `v2.tauri.app/reference/javascript/api/namespacemocks` docs,
  then cross-checked against the actual shipped source,
  `node_modules/@tauri-apps/api/mocks.js`): it intercepts
  `plugin:event|listen`/`plugin:event|emit`/`plugin:event|unlisten`
  internally (a `Map<event, handlerId[]>`), so a plain call to the real
  `emit()` from `@tauri-apps/api/event` replays every `listen()` handler
  registered against the SAME mocked `window.__TAURI_INTERNALS__.invoke` --
  exactly the mechanism `src-tauri/src/run.rs` and `JobsView.vue` use in
  production. No hand-reimplementation of the wire contract was needed or
  written.
- `@tauri-apps/plugin-os`'s `platform()` is **not** an `invoke()` call at all
  (verified: `node_modules/@tauri-apps/plugin-os/dist-js/index.js`) -- it
  reads `window.__TAURI_OS_PLUGIN_INTERNALS__.platform` synchronously, a
  global normally injected by the real Rust plugin's init script. Neither
  `mockIPC` nor `mockWindows` touches it, so `e2e/mocks.ts`'s
  `installMockIPC` sets it explicitly. Missing this would have made
  `FirstRun.vue`'s `platform()` call at setup return `undefined` silently.
- `@tauri-apps/plugin-fs`'s `writeTextFile` sends the target path via the
  invoke call's *third* argument (`options.headers.path`), which
  `mockIPC`'s wrapper drops before calling the mock callback (verified in
  `node_modules/@tauri-apps/plugin-fs/dist-js/index.js`) -- the mock cannot
  see the path at all, only the raw byte payload as `args`. Not exercised by
  the three named scenarios (`RunHistory.vue`'s save-as flow), but the
  generic fallback resolves it to `null` so an incidental invocation never
  hangs a mounted `RunHistory`.
- `queue.rs`'s `JobEvent` and `run.rs`'s `start_run`/`emit_run_finished`
  were read directly (not assumed from `ipc.ts`) to confirm the soft-outcome
  synchronous-emit contract and the exact payload shapes; fixtures in
  `smoke.spec.ts` are additionally `satisfies JobEvent`/`RunFinishedEvent`/
  etc. (type-only imports from `../src/ipc`). CORRECTED (review fix wave,
  see the appended section): as originally committed (`102f159`), NOTHING
  actually ran that type-check -- `e2e/tsconfig.json` existed but was
  unwired (`test:e2e` was `vite build` + `playwright test`, both
  type-erasing; `pnpm build`'s `vue-tsc` scopes to `src/`; lint is
  non-type-aware), so the `satisfies` annotations were inert and the drift
  claim was false. Fixed by prepending `tsc --noEmit -p e2e/tsconfig.json`
  to `test:e2e`; a RED/GREEN probe (below) confirms the drift protection is
  now real.

### Mock harness (`e2e/`)

A plain browser cannot resolve bare npm imports, and the app under test is
served as a static `dist/` build (no bundler in the loop at test time). To
use the *real* `@tauri-apps/api/mocks`/`event` code rather than
hand-rolling the IPC wire format: `e2e/tauri-mock-entry.ts` re-exports
`mockIPC`/`mockWindows`/`clearMocks`/`emit` onto `window.__muxsmithE2E__`;
`e2e/vite.harness.config.ts` bundles it (Vite lib mode, IIFE, zero external
imports -- verified by grepping the output) into
`e2e/.generated/tauri-mock-harness.js`, gitignored, rebuilt on every
`test:e2e` run so it can never drift from the pinned `@tauri-apps/api`
version `src/` itself uses. `e2e/mocks.ts` injects it via
`page.addInitScript({ path })` (reads straight off disk, never touches
`dist/`/`public/` -- this file can never leak into the shipped Tauri
binary), plus a second `addInitScript(installMockIPC, scenario)` wiring a
per-command scripted response queue and a `page.exposeFunction`-backed
Node-side call log.

`e2e/i18n-en.ts` loads the real `locales/en/{gui-*,diagnostics}.ftl`
catalogs through `@fluent/bundle` (same library, same catalog set as
`src/i18n/index.ts`) and exposes `en(id, args)` for assertions -- every
user-facing string the smoke checks against is rendered through the real
catalog, never hand-duplicated.

### Smoke scenarios (`e2e/smoke.spec.ts`)

1. **First-run gate**: `detect_mkvmerge` rejects once (`mkvmerge-not-found`)
   then resolves; asserts the guidance heading/detail/platform-guidance
   text, an axe scan, then drives the manual-path recovery (mocked
   `plugin:dialog|open` returns a path, submit re-detects) through to the
   main shell. Asserts real invocation evidence: `detect_mkvmerge` called
   exactly twice, `set_settings` called with the manual path -- not just a
   UI-state flip.
2. **Batch dry-run**: mocked `validate_profile` (empty) then `dry_run`
   (full fixture: a diagnostic with a param, a resolved file plan, a
   suggestion) renders the resolution table (`getByRole("table", ...)`
   matched via the `<caption>`-derived accessible name), the diagnostic
   line (composed through the real `batch-diagnostic-line`/`severity-*`/
   `unknown-property` catalog entries), and the suggestion card; clicking
   Copy is asserted against the recorded `plugin:clipboard-manager|write_text`
   call's actual `text` argument, plus the UI's "Copied" feedback.
3. **Jobs live run**: mocked `start_run` returns `total_jobs: 2`; the test
   then drives `started`/`progress`/`output`/`finished` events by hand via
   the real `emit()`, asserting the `<progress>` DOM property mid-run
   (before any `finished` event -- this specifically exercises the live
   event path, not reconciliation), the live-log pane's text, a per-row
   cancel click's real `cancel_job({index:1})` invocation, then a
   `run-finished` document whose `aria-live` summary text is asserted
   against the real `jobs-summary-line` catalog rendering.

One a11y scan (`@axe-core/playwright`, fail on `serious`/`critical`
impact) per scenario, at a materially different view state (first-run
guidance / dry-run results / mid-run jobs view). Verified the scan is real
(not vacuously passing) with a throwaway debug spec: 28 axe passes, 0
violations, 0 incomplete on the batch view's default state.

### `scripts/check-i18n.mjs`

Two independent checks, no dependencies:

- **Hard failure** (exit 1): every literal `t('id')`/`$t('id')` call in
  `src/**/*.{vue,ts}` must resolve to a real catalog id. The call-detection
  regex needed a negative lookbehind (`(?<![\w$])\$?t\(`) after an early
  version produced false "missing" hits -- `emit(`, `writeText(`,
  `useFluent(`, `.mount(`, `attempt(` all end in `t(` and would otherwise
  match (verified empirically against the actual codebase before settling
  on the fix; `BatchView.vue`'s `emit("start-run", ...)` was the concrete
  case that would have broken). Verified both directions live: a seeded
  typo correctly fails with exit 1 and a precise file:line; the real tree
  passes with exit 0.
- **Soft warning** (always exit 0): catalog ids never referenced anywhere
  in `src/`. Per the brief, `diagnostics.ftl`'s id set and `gui-common.ftl`'s
  four D31 `close-abort-*` ids (Rust-only, consumed via `include_str!`) are
  explicitly allowlisted as known-used. Beyond the brief's literal
  instruction, "used" also counts any catalog id appearing anywhere in
  `src/` as a quoted string literal (not just inside a `t()`/`$t()` call) --
  this catches the `case "jobs-state-ok": return "jobs-state-ok"`-style
  mapping-function pattern used throughout `jobRowState.ts`/`JobRow.vue`/
  `FirstRun.vue`, which a literal-call-only scan would otherwise flag as
  unused despite being genuinely reachable.

**Known, accepted limitation**: 11 shell `IpcError` codes (`gui-common.ftl`'s
`mkvmerge-spawn-failed`/`mkvmerge-query-failed`/`settings-dir-unavailable`/
`settings-io-failed`/`settings-parse-failed`/`internal-task-failed`,
`gui-jobs.ftl`'s `run-already-active`/`no-active-run`/`invalid-run-id`/
`job-log-unavailable`/`job-log-not-found`) are reached only via the generic
`$t(err.code, err.params)` pattern and never spelled out literally anywhere
in `src/`, so they surface as "unused" even though genuinely rendered
whenever that IPC error occurs. These follow the *identical* dynamic-dispatch
shape as `diagnostics.ftl` (both `.ftl` files' own header comments say
"keyed directly on IpcError.code"), so allowlisting them the same way would
be a defensible follow-up -- deliberately not done here since the brief
scoped the dynamic-id accommodation to `diagnostics.ftl` specifically and
this half of the gate is warning-only (zero effect on `check:i18n`'s exit
code). Current run: 11 warnings, all in this category, `exit 0`.

### CI (`.github/workflows/ci.yml`)

Appended three Linux-only steps (`if: runner.os == 'Linux'`, matching the
existing mkvtoolnix/webkit2gtk guard style) after the existing `pnpm build`
step: `pnpm check:i18n`, `pnpm exec playwright install --with-deps chromium`,
`pnpm test:e2e`. SHA-pin + comment conventions untouched; no other job
changed.

## One real bug found and fixed along the way

`playwright.config.ts`'s `webServer` originally used `127.0.0.1` for both
`baseURL` and the health-check `url`, with `vite preview --port 4173
--strictPort` left to bind its default `localhost` host. On this machine
`vite preview` resolved `localhost` to `::1` only (`ss -tlnp` confirmed
`[::1]:4173` listening, `curl http://127.0.0.1:4173` connection-refused),
so Playwright's IPv4 health check timed out against a server that was, in
fact, already up. Fixed by forcing `vite preview --host 127.0.0.1
--port 4173 --strictPort` explicitly, removing the ambiguity rather than
hoping a given runner's `localhost` resolution order happens to match.

## New dependency pins (`save-exact`, already project policy)

- `@playwright/test` 1.61.1
- `@axe-core/playwright` 4.12.1 (pulls `axe-core` 4.12.1, `playwright-core`
  peer satisfied by 1.61.1)
- `@types/node` 26.1.1 (dev-only, for `e2e/`'s `node:fs`/`node:path`/
  `node:url` imports; not consumed by `pnpm build`'s `vue-tsc` pass, which
  stays scoped to `src/`)

## Gate (foreground, final tree, all green)

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo test --workspace` (72 passed, unchanged -- this task
touched no Rust), `cargo deny check` (advisories/bans/licenses/sources ok),
`pnpm lint`, `pnpm build`, `pnpm check:i18n` (0 missing, 11 known-limitation
warnings, exit 0), `pnpm test:e2e` (3 passed, chromium, ~0.9s).

## Self-review: real behavior, not mock echoes

- Every mocked-IPC assertion that matters checks the **recorded invocation**
  (command name + actual argument value via `page.exposeFunction`), not
  just a resulting UI flag: clipboard copy asserts the exact `text` sent to
  `plugin:clipboard-manager|write_text`; per-row cancel asserts
  `cancel_job`'s `index` argument; first-run recovery asserts both the
  second `detect_mkvmerge` call and `set_settings`'s persisted path.
- The progress-bar assertion (`toHaveJSProperty("value", 42)`) happens
  strictly before any `finished` event is emitted, so it specifically
  exercises the live `job-event` -> DOM path, not `run-finished`'s
  authoritative reconciliation (which the same test also exercises
  afterward, for both a normal and a per-job-cancelled row).
- Fixtures for `JobEvent`/`RunFinishedEvent`/`ReportDocument`/etc. are
  `satisfies`-checked against `../src/ipc`'s real types (type-only import,
  zero runtime cost); since the review fix wave wired `tsc --noEmit -p
  e2e/tsconfig.json` into `test:e2e` (it was unwired in `102f159` -- the
  original version of this bullet overclaimed), a future contract drift
  between `run.rs`/`queue.rs` and `ipc.ts` genuinely fails `test:e2e` at
  its type step (RED/GREEN-probed) rather than silently testing a stale
  shape.
- No test asserts against a hand-duplicated string; every user-facing
  assertion goes through `e2e/i18n-en.ts`'s real Fluent rendering.
- One real bug caught during setup (not a placeholder pass): the `getByRole`
  locators originally used default fuzzy substring name matching, which
  made "Run" (the batch-run button) ambiguously match "Dry run" and a
  fixture path containing "run-demo.yaml" -- caught by an actual 3-way
  strict-mode-violation failure on first run, fixed by a `name()` helper
  that forces `exact: true` everywhere, not by narrowing the fixture text.

## Concerns

- The i18n "unused" check's 11-warning residual noise (documented above);
  a defensible, low-risk follow-up if Şenol wants it cleaned up.
- `RunHistory.vue`'s save-as-file path (`plugin:fs|write_text_file`) is
  covered only by a generic fallback mock (returns success unconditionally,
  since the invoke call's path lives in an argument `mockIPC` cannot see),
  not exercised by name in any of the three named scenarios -- consistent
  with the brief ("mock each where a scenario needs it"), just noting it's
  the one plugin command with zero direct assertion coverage.

## Files changed

- New: `playwright.config.ts`, `e2e/{smoke.spec.ts,mocks.ts,
  tauri-mock-entry.ts,vite.harness.config.ts,i18n-en.ts,global.d.ts,
  tsconfig.json}`, `scripts/check-i18n.mjs`
- Modified: `package.json` (+2 scripts, +3 devDependencies), `pnpm-lock.yaml`,
  `.github/workflows/ci.yml` (+3 Linux-only steps), `.gitignore`
  (`e2e/.generated/`, `playwright-report/`, `test-results/`),
  `eslint.config.js` (ignore the generated harness bundle + Playwright
  report/result dirs)

Commit: `102f159` -- `test(gui): playwright smoke + axe a11y + i18n
completeness gate`.

---

## Fix wave (coordinator review verdict: Needs fixes -- 1 Important, 1 Minor)

Commit `945ee96` -- `fix(gui-e2e): wire e2e type-check into test:e2e; document i18n parser constraint`, on top of `102f159`. Two files changed
(`package.json`, `scripts/check-i18n.mjs`); JS-only, so the four cargo
gates are untouched by construction (no Rust file, no Cargo manifest, no
dependency change) and were not re-run.

1. **Important -- e2e type-check was unwired; the drift-protection claim
   was false.** `e2e/tsconfig.json` existed but nothing invoked it:
   `test:e2e` ran `vite build` (type-erasing) + `playwright test` (babel
   strip-types), `pnpm build`'s `vue-tsc` scopes to `src/`, and lint is
   non-type-aware -- so the `satisfies JobEvent`/`RunFinishedEvent` fixture
   annotations compiled against nothing and the report's "contract drift
   fails the smoke's type-check" claim did not hold. Fix: `test:e2e` is now
   `tsc --noEmit -p e2e/tsconfig.json && vite build --config
   e2e/vite.harness.config.ts && playwright test`. Plain `tsc` from the
   already-pinned `typescript` 6.0.3 (no new dependency; `vue-tsc` would
   also work but nothing under `e2e/` imports a `.vue` file, so the
   Vue-aware wrapper buys nothing here).

   **Drift probe (RED/GREEN), run for real, not asserted:**
   - RED: renamed one fixture field (`duration_ms` -> `durationMs` in
     scenario (c)'s first `finished` outcome); `mise exec -- pnpm test:e2e`
     failed at the type step, exit 2, before the harness build or
     Playwright ever ran:
     `e2e/smoke.spec.ts(307,71): error TS2561: Object literal may only
     specify known properties, but 'durationMs' does not exist in type
     'JobOutcome'. Did you mean to write 'duration_ms'?`
   - GREEN: reverted the probe (verified `git diff` on the spec is empty);
     full `test:e2e` passes: type step clean, harness built, 3/3 smoke
     tests green.

   Both places in this report that made the false claim (the
   "Mechanics verified" bullet and the "Self-review" bullet) are corrected
   in place above, each stating the fix.

2. **Minor -- check-i18n.mjs parsing constraint documented.** The
   per-line `MESSAGE_ID_RE` now carries a header comment naming the
   deliberate single-line, non-Fluent-parser constraint (mirroring
   `run.rs::ftl_message`'s identically documented line-lookup constraint):
   multiline values/selectors are safe for id extraction (id on the first
   line, continuation lines indented and never matched); attributes
   (`.attr =`) are NOT registered -- if the frontend ever addresses a
   `$t("msg.attr")` form, the scanner will flag it as missing and
   `parseCatalogIds` must be extended, not worked around; terms
   (`-term =`) are deliberately not registered (catalog-internal, never a
   `$t()` argument).

## Fix-wave gate (foreground, final tree, all green)

`mise exec -- pnpm lint`, `mise exec -- pnpm build`, `mise exec -- pnpm
check:i18n` (0 missing, same 11 known-limitation warnings, exit 0),
`mise exec -- pnpm test:e2e` (type step + harness build + 3/3 smoke
passed, ~0.9s). Cargo gates untouched by this JS-only change, as stated
above.
