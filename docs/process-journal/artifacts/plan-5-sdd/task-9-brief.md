### Task 9: Frontend app shell, Fluent, first-run, settings dialog

**Files:**
- Create: `src/app/` (nav + view switch), `src/i18n/` (fluent-vue setup, catalog loader), `src/views/FirstRun.vue`, `src/components/SettingsDialog.vue`, `locales/en/gui-settings.ftl`; extend `locales/en/gui-common.ftl`
- Modify: `src/App.vue`, `src/main.ts`
- Test: `vue-tsc` (in `pnpm build`) + eslint; behavior covered by T12 smoke

**Interfaces:**
- Produces: single-window layout - `<nav>` (Batch | Jobs, `aria-current` on active) + `<main>`; view switch is a `ref<'batch'|'jobs'>` (no router at two views); `t()` from fluent-vue everywhere; first-run flow: on mount call `detect_mkvmerge` -> found: proceed; missing/too old: `FirstRun.vue` full-screen guidance per OS (`platform()` from @tauri-apps/api) with manual path picker (dialog plugin) writing `set_settings` then re-detect; SettingsDialog: native `<dialog>`, labeled form fields (`label for`/`id`) for mkvmerge path, default jobs, locale.
- Consumed by: T10/T11 mount their views into the switch; conventions (semantic HTML, Fluent-only strings, `data-testid` on structural nodes) are the template the view tasks copy.

- [ ] **Step 1:** Implement shell + Fluent loader (catalogs via `?raw` imports, one FluentBundle, diagnostics.ftl included - diagnostic rendering reuses the SAME message templates as the CLI, spec 8.4).
- [ ] **Step 2:** First-run + settings against T7 commands; every control labeled; `pnpm lint` (no-raw-text) + `pnpm build` green.
- [ ] **Step 3: Commit** `feat(gui): app shell, Fluent wiring, first-run detection, settings dialog`

---

