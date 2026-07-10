### Task 10: Batch view (spec 8.2 view 2, minus apply-suggestion per D22)

**Files:**
- Create: `src/views/BatchView.vue`, `src/components/{ResolutionTable,DiagnosticsPanel,SuggestionCard}.vue`, `locales/en/gui-batch.ftl`
- Test: T12 smoke covers behavior; `pnpm lint`/`build` per commit

**Interfaces:**
- Consumes: T7 (`validate_profile`, `dry_run`, `get_settings`/`set_settings` for recents + dir_memory), T9 shell conventions.
- Produces: `startRun()` emit consumed by App to switch to Jobs view with run parameters (profile, source, output, jobs).

- [ ] **Step 1:** Profile picker (dialog plugin, `.yaml/.yml` filter) + recents list (MRU from settings); on pick -> `validate_profile` -> diagnostics render (code+params -> Fluent, severity icon + text, `role="status"` for the summary line).
- [ ] **Step 2:** Source/output dir pickers, prefilled from `dir_memory[profile]`, persisted back on change. Dry-run button (disabled while running, `aria-busy` during); render `batch_document`: per-file `<table>` (caption, `<th scope>`) rule -> resolved track; config + per-file diagnostics; suggestions as cards with the YAML fragment in `<pre><code>` and a copy button (clipboard-manager plugin) - copy is the ONLY suggestion action (D22).
- [ ] **Step 3:** Run button -> emits startRun with the current selection; disabled with a Fluent tooltip when errors exist or mkvmerge missing.
- [ ] **Step 4:** lint + build green. **Commit** `feat(gui): batch view - profile/dirs, dry-run report, suggestions show+copy`

---

