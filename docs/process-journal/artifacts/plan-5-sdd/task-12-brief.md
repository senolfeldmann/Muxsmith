### Task 12: Playwright smoke + i18n completeness gate + CI finish

**Files:**
- Create: `playwright.config.ts`, `e2e/{smoke.spec.ts,mocks.ts}`, `scripts/check-i18n.mjs`
- Modify: `package.json` (scripts `test:e2e`, `check:i18n`), `.github/workflows/ci.yml`
- Test: this task IS the test layer

**Interfaces:**
- Consumes: everything T9-T11 rendered; `mockIPC`/`mockWindows` from `@tauri-apps/api/mocks` (the smoke runs the Vite build in a plain browser; no tauri-driver - spec 10 keeps GUI tests thin).

- [ ] **Step 1: Smoke scenarios** (all locators `getByRole` first, `data-testid` fallback; locale pinned `en`): (a) detect fails -> first-run guidance visible, manual path via mocked dialog fixes it; (b) mocked `dry_run` document -> resolution table + diagnostics + suggestion copy calls the mocked clipboard; (c) mocked run: emit scripted job-events -> rows progress, log fills, cancel_job invoked by row button, run-finished summary announced.
- [ ] **Step 2: a11y assertion:** `@axe-core/playwright` scan on each view in the smoke; fail on serious/critical violations.
- [ ] **Step 3: `check-i18n.mjs`:** parse `locales/en/gui-*.ftl` + `diagnostics.ftl` message ids; scan `src/**/*.{vue,ts}` for `t('...')`/`$t('...')` ids; unknown id -> exit 1 (missing key); report unused gui-* keys as warnings. Wire as `pnpm check:i18n`.
- [ ] **Step 4: CI:** append `pnpm check:i18n`, `pnpm exec playwright install --with-deps chromium` + `pnpm test:e2e` to the Linux leg. Full gate + all pnpm gates green, CI verified on a push. **Commit** `test(gui): playwright smoke + axe a11y + i18n completeness gate`

---

