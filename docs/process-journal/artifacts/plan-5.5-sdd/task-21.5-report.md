# Task 21.5 report: German selectable in settings

Branch `plan55-t215`, worktree `.worktrees/t215`, based on post-T21 master.

## Endonym call

**Not endonym-labeled.** The field's own precedent settles it: `settings-locale-option-en` is `English` in `locales/en/gui-settings.ftl` but `Englisch` in `locales/de/gui-settings.ftl` — the German catalog uses the German *word* for English, not the English endonym. So each option label is translated into the CURRENT UI locale, same as every other string in the catalog; it is not a fixed endonym list.

Followed that precedent exactly: `settings-locale-option-de = German` (en catalog), `settings-locale-option-de = Deutsch` (de catalog). "Deutsch" happens to coincide with German's own endonym, but that is incidental to the pattern (a translation of the string "German" into German), not a deliberate endonym choice — a future third locale's `-de` label would follow the same current-locale-translation rule, not "always show the native name."

## settings-locale-hint wording

- en: `English and German ship in this version; more languages can be added later without changing this setting's meaning.`
- de: `In dieser Version sind Englisch und Deutsch enthalten; weitere Sprachen können später hinzugefügt werden, ohne die Bedeutung dieser Einstellung zu ändern.`

Minimal edit of the existing sentence (`Only English` -> `English and German`) to match the real two-locale state; second clause ("more languages can be added later...") unchanged, already correctly future-proofed. Also dropped the de catalog's stale header comment flagging the hint/dropdown as out-of-scope-for-T21 (both are now fixed, so the note is obsolete).

## SettingsDialog.vue change

Added the German `<option>` mirroring the existing English one:

```html
<option value="en">{{ $t("settings-locale-option-en") }}</option>
<option value="de">{{ $t("settings-locale-option-de") }}</option>
```

## e2e evidence

New case in `e2e/smoke.spec.ts` (`german locale` describe block, alongside the existing de-AT seeding case): *"selecting German in the settings dialog saves it, and it renders the German catalog on the next start."*

Real architectural constraint discovered while designing this: `main.ts` resolves the locale exactly once, before mount (`resolveLocale()` -> `buildBundles()` -> `createFluentVue`), and nothing anywhere swaps the live `FluentBundle`s afterwards — no `useFluent()` call site reassigns `bundles`. A locale change saved through the settings dialog therefore only takes effect on the app's next start, same as most desktop apps. The test reflects that honestly rather than inventing live-reload behavior out of scope for this task:

1. Opens the app (English, default mock), clicks `open-settings`, confirms the select currently reads `"en"`.
2. Selects `de` via `selectOption`, clicks Save.
3. Asserts real invocation evidence: exactly one `set_settings` call recorded with `args.settings.locale === "de"` (not a UI echo).
4. Simulates the resulting restart: registers a second, layered `page.addInitScript(installMockIPC, { get_settings: [de settings], ... })` and calls `page.reload()`. `@tauri-apps/api/mocks`' `mockIPC` reassigns `window.__TAURI_INTERNALS__.invoke` outright, so the later-registered scenario governs the reloaded page (verified in `node_modules/@tauri-apps/api/mocks.js`).
5. Re-opens the settings dialog and asserts a literal German string (`"Einstellungen"` heading), not `en(id)`/a `de()` helper — same rationale as the existing de-AT test: proving the de bundle is genuinely active, not that a translation function was called correctly. Also asserts the select now reads `"de"`.

Infrastructure change to support step 4: exported `installMockIPC` from `e2e/mocks.ts` (was file-private). `installTauriMocks` cannot be called a second time on the same page — its `page.exposeFunction("__muxsmithRecordInvoke__", ...)` throws on a second registration for the same name — so the export lets a test add a second, later-winning mock scenario without hitting that.

Ran red first (before the SettingsDialog/ftl changes): `locator.selectOption` timed out, "did not find some options" — confirmed the `de` option genuinely did not exist yet. Green after implementation, including the rest of the smoke suite (6/6 pass, all pre-existing cases untouched).

## Gate results (all run in foreground)

1. `cargo fmt --all --check` — clean.
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean.
3. `cargo test --workspace` — 78+32+... all crates, 0 failed.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — clean.
5. `cargo deny check` — advisories/bans/licenses/sources all ok.
6. `pnpm lint` — clean.
7. `pnpm build` — vue-tsc + vite build, clean.
8. `pnpm check:i18n` — exit 0; parity check passes for `locales/de/` against `locales/en/` (the new `settings-locale-option-de` key is present and matched in both, `settings-locale-hint` values checked for id parity only, not content — the pre-existing warning-only "unused key" list is unchanged, unrelated to this change).
9. `pnpm test:e2e` (`tsc --noEmit -p e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && playwright test`) — 6/6 pass, including the new case. `e2e/i18n-en.ts`'s `assertAllCatalogsParseCleanly()` (the all-locales parse guard) runs at import time as part of this and passed, so both edited `.ftl` files parse cleanly through the real Fluent parser, not just the line-regex parity scanner.

`pnpm install --frozen-lockfile` ran once at the start (fresh worktree, matched pinned Node 26.5.0 / pnpm 11.10.0, no resolution changes).

## Self-review

- Diff is minimal and scoped exactly to the brief's Modify list plus the one necessary test-infra export (`installMockIPC`); no unrelated refactor.
- Did not add any live-reload/bundle-swap mechanism to make the setting "just work" mid-session — that would be new production behavior outside this task's brief, and the existing architecture (resolve-once-at-bootstrap) is a legitimate, common pattern for a desktop app's language setting. Flagged below as worth a conscious decision, not silently left as a surprise.
- Reused the existing de-AT test's conventions throughout (literal German assertion strings, `visibleText`/BiDi awareness not actually needed here since no placeable is involved, `installTauriMocks`'s recorded-call pattern for real evidence) rather than inventing new idioms.
- Verified the `mockIPC`-reassignment behavior empirically against the installed `@tauri-apps/api` package source before relying on it, rather than assuming Playwright/mocks semantics.

## Concerns

- **No in-session locale switch exists anywhere in the app.** This was true before this task and stays true after it; T21.5's brief scoped the fix to reachability (the option exists, the hint is honest), not to live-apply behavior. Worth a conscious product call for Plan 6+: either document "restart to apply" in the settings-locale-hint text itself, or wire a real live bundle swap (`fluent.bundles = buildBundles(locale)` on save) if instant switching is expected. Neither is in this brief's scope; flagging so it is a decision, not a silent gap.
- `check:i18n`'s check 3 (cross-locale parity) verifies key sets only, not content — a de value could technically diverge in meaning from its en counterpart without failing any gate. Pre-existing property of the tool, not introduced here.

## Addendum: T21.5 review fixes (commit `b833f2a`)

Owner review overturned the endonym call above: the picker convention is that a user stuck in a foreign UI locale must still find their own language, so `settings-locale-option-en`/`-de` are now **identical strings in both catalogs** (`English` / `Deutsch`), not translated per current locale. Also fixed: the hint enumerated the shipped languages (re-stales at a third locale) and didn't mention that the resolve-once-at-bootstrap architecture (flagged above) means a changed locale needs a restart. New wording (second review pass dropped the carried-over "more can be added later..." middle clause as convoluted filler):

- en: `Available languages are listed in the dropdown. A changed language takes effect after restarting Muxsmith.`
- de: `Die verfügbaren Sprachen stehen in der Auswahlliste. Eine geänderte Sprache wird erst nach einem Neustart von Muxsmith wirksam.`

Minor: the post-reload e2e combobox locator (`e2e/smoke.spec.ts`) was `getByRole("combobox")` with no name, unlike the pre-reload one which binds to the catalog label; aligned it to `getByRole("combobox", { name: "Sprache", exact: true })`, matching the literal-German-string pattern the same test already uses for the post-reload heading assertion (no `de()` catalog helper exists, only `en()`).

Gates rerun in foreground: `pnpm check:i18n` (ok, same pre-existing unused-key warnings), `pnpm test:e2e` (6/6 pass), `pnpm lint` (clean).
