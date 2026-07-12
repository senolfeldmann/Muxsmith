### Task 21: German locale (#17 step 3)

**Files:**
- Create: `locales/de/cli.ftl`, `diagnostics.ftl`, `gui-batch.ftl`, `gui-common.ftl`, `gui-jobs.ftl`, `gui-settings.ftl`
- Modify: locale loader/scanner if de needs registration + primary-subtag normalization (docs-tree S15 - the loader half of this trigger; read the loader first)

- [ ] Step 1: Translate all six catalogs (agent draft). Terminology guardrails: Track/Spur, Datei, Profil, Regel, Vorschlag - keep mkvtoolnix-de's established vocabulary where it exists (SI-3: read mkvtoolnix's de.po for the domain terms, cite; that is terminology research on a GPL project's FACTS, no text copying beyond single domain terms).
- [ ] Step 2: check:i18n green (T20 now enforces parity). GUI smoke test with `de` locale in e2e (one Playwright case asserting a de string renders).
- [ ] Step 3: **Şenol reviews terminology before merge** (explicit gate - walkthrough #17). Commit after his pass: `i18n: German locale (#17)`.

