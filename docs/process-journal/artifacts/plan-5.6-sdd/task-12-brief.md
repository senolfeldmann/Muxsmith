### Task 12: Catalogs + spec polish (Wave 2, after T5's rename merged)

**Files:**
- Modify: `locales/de/cli.ftl`, `locales/de/diagnostics.ftl`, `locales/de/gui-common.ftl`, `locales/de/gui-settings.ftl`, `locales/en/gui-settings.ftl`, `locales/en/gui-common.ftl`, `locales/en/gui-batch.ftl`, `locales/de/gui-batch.ftl`, `locales/en/gui-jobs.ftl` + `locales/de/gui-jobs.ftl` (headers only where listed), `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:286`

- [ ] **idiom (Fluent comment levels)** - mechanical level fix per the projectfluent.org comment spec: file-scope headers `#` -> `###` (de/cli.ftl, de/diagnostics.ftl, de/gui-common.ftl, de/gui-settings.ftl, en/gui-settings.ftl - where the `#` block adjacent to the first message currently attaches as that message's translator note); message-block section comments `#` -> `##` in en+de gui-common.ftl/gui-batch.ftl (gui-jobs.ftl already correct). Genuine single-message notes (e.g. en/gui-common.ftl identify-failed) stay `#`. No ids change; MESSAGE_ID_RE and run.rs's line parser unaffected - reviewer confirms check:i18n and the e2e real-parse stay green.
- [ ] **doc (seed whole-branch M1)** - the 5 overclaiming de headers scope their enforcement claim to keys: "keys mirror it (id parity enforced by scripts/check-i18n.mjs); placeables and selector structure mirror it by convention (reviewed manually, not machine-checked)". cli.ftl:2-3, diagnostics.ftl:2-3, gui-batch.ftl:2-3, gui-jobs.ftl:2-3, gui-common.ftl:2-3; gui-settings.ftl already claims only keys and stays.
- [ ] **doc (final-verification nit)** - spec §5.2:286 WorkerPanicked Severity cell: "info" -> "n/a (job-error token, not a rendered diagnostic)"; condition column unchanged.
- [ ] Full gate; commit `docs(i18n): Fluent comment levels + honest header claims; spec §5.2 WorkerPanicked cell`.

