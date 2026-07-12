### Task 21.5: German reachable from the UI (added 2026-07-12, Şenol decision at the T21 gate)

T21 shipped the de locale, but the settings dropdown lists only English
and the settings-locale-hint claims only English is included (now false).
Şenol: fix immediately, not with Plan 6.

**Files:**
- Modify: `src/components/SettingsDialog.vue` (German option), `locales/en/gui-settings.ftl` + `locales/de/gui-settings.ftl` (option label key, hint corrected in both)
- Test: e2e settings case (select de via the UI, assert a German string renders after switch)

- [ ] Failing e2e first, implement, parity green, full nine-part gate; commit `feat(gui): German selectable in settings (T21 follow-up)`.

