## Task 2: the settings language control becomes three-state (W1)

Read first: this plan's D106 in full; `docs/ROADMAP.md`'s "OWNER QA PASS, round 3" finding 1 through its ruling and its known-cost block; `src/main.ts`; `src/i18n/index.ts`; `src/components/SettingsDialog.vue`; `locales/en/gui-settings.ftl` and `locales/de/gui-settings.ftl`; `e2e/locale-switch.spec.ts` in full (its `de()`/`buildDeBundle` helpers are reused); `e2e/mocks.ts`'s `installMockIPC` for the `get_settings` fallback and the second-registration mechanism; the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src/i18n/index.ts` (add the exported `effectiveLocale` seam; nothing else)
- Modify: `src/main.ts` (`resolveLocale` routes through the seam; its doc comment names it)
- Modify: `src/components/SettingsDialog.vue` (the sentinel constant, the form init, the save mapping, the live switch and its comment, the third `<option>`)
- Modify: `locales/en/gui-settings.ftl` (the new option id and the reworded `.hint`)
- Modify: `locales/de/gui-settings.ftl` (the same two)
- Modify: `e2e/locale-switch.spec.ts` (a new describe with three cases; the file's header doc gains the new subject)

`playwright.config.ts` and `e2e/mocks.ts` are NOT edited (D106 decisions 5 and 6). `e2e/smoke.spec.ts` is NOT edited by this task.

**Interfaces:**
- Consumes: nothing.
- Produces: `effectiveLocale`, whose second consumer is the dialog.

- [ ] **Step 1: the seam.** Add to `src/i18n/index.ts`, exported, with a doc comment that states the rule and names both callers by symbol:

```ts
export function effectiveLocale(saved: string | null): string {
  return saved ?? navigator.language;
}
```

- [ ] **Step 2: `src/main.ts`.** `resolveLocale` becomes `effectiveLocale((await getSettings()).locale)` in the try and `effectiveLocale(null)` in the catch, with the import added. The existing doc comment keeps both facts it carries (why the locale resolves before mount, and why a `get_settings` failure is not a startup blocker) and gains the seam's name in place of the inline `navigator.language` description.

- [ ] **Step 3: `src/components/SettingsDialog.vue`.** Four edits:
  - A module-level constant with a comment stating why the empty string is the sentinel and naming the sibling field that already uses it: `const SYSTEM_LOCALE = "";`
  - `form`'s initial `locale` becomes `SYSTEM_LOCALE`, and `open()`'s init becomes `form.locale = baseline.locale ?? SYSTEM_LOCALE;`
  - `save()`'s `next` carries `locale: form.locale === SYSTEM_LOCALE ? null : form.locale,` and the live switch becomes exactly

```ts
    if (next.locale !== baseline.locale) {
      applyLocale(effectiveLocale(next.locale));
    }
```

    The existing comment's `!== null` narrowing explanation is replaced by one naming the seam; the D56 live-switch rationale and the v-show-keeps-state sentence stay.
  - The `<select>` gains, as its FIRST option, `<option :value="SYSTEM_LOCALE">{{ $t("settings-locale-option-system") }}</option>`; the `en` and `de` options keep their explicit values and their order.

- [ ] **Step 4: the catalogs, both locales, fenced.** In `locales/en/gui-settings.ftl` replace exactly

```
settings-locale-label = Language
    .hint = Which language the Muxsmith interface uses.
```

  with exactly

```
settings-locale-label = Language
    .hint = Which language the Muxsmith interface uses. System language follows your operating system and falls back to English where a translation is missing.
settings-locale-option-system = System language
```

  and in `locales/de/gui-settings.ftl` replace exactly

```
settings-locale-label = Sprache
    .hint = In welcher Sprache Muxsmith seine Oberfläche anzeigt.
```

  with exactly

```
settings-locale-label = Sprache
    .hint = In welcher Sprache Muxsmith seine Oberfläche anzeigt. Systemsprache folgt der Sprache deines Betriebssystems und fällt auf Englisch zurück, wo eine Übersetzung fehlt.
settings-locale-option-system = Systemsprache
```

  Both files keep `settings-locale-option-en`/`-de` unchanged and in place. Neither value carries a placeable, so the pattern-structure parity check (D55 rule 5) is satisfied by construction, and the attribute-name sets stay equal.

- [ ] **Step 5: the tests, in `e2e/locale-switch.spec.ts`.** A new `test.describe("system-locale default (D106)")` carrying `test.use({ locale: "de-DE" })` as its first statement, three cases. Every interaction string comes from `en()`. **Every asserted German string must be one whose German value DIFFERS from its English value** - either a literal that exists only in de, or `de(id)` for an id whose two values differ - because `buildBundles` negotiates `[requested, en]` per message, so an assertion on an id whose values are identical passes even when the interface fell back to English entirely. That is the frontend instance of the fallback handle in the Global Constraints, and the set it excludes is measured rather than guessed: **15 gui-* ids carry identical en/de values, and both language option labels (`settings-locale-option-en` = `English`, `settings-locale-option-de` = `Deutsch`) are among them**, in the very dialog these cases drive. **The method decides the figure, so it is stated in full: full multi-line value comparison, attributes excluded, over messages that carry a value of their own.** Two coarser readings of the same tree give two other numbers and both reconcile under that one rule - counting value-less messages as well gives 16 (the single extra member is `batch-recents-select`, which carries only a `.tooltip`), and comparing first lines only gives 18 (the two further members are the selector messages whose `{ $n ->` opening matches while their German branches differ). The three cases below are already clear of that set - they assert the batch heading (`Batch` / `Stapel`) and the new `settings-locale-option-system` (`System language` / `Systemsprache`) - so this scopes the permission to its safe set rather than changing any prescribed assertion.
  - **Case 1, first run.** Mock `get_settings` with `locale: null` (and `detect_mkvmerge` as the file already does). `page.goto("/")`. Assert (a) the batch heading renders its de value, and `documentElement.lang` is `"de"`; (b) open settings and `select#settings-locale` has value `""`, and the selected option's text equals the de `settings-locale-option-system`.
  - **Case 2, saving without touching the language.** Same scenario. Open settings, change `settings-default-jobs` to a different number, save. Assert exactly one `set_settings` call and that its `settings.locale` is `null`. **This is the defect's core - the first Save creating an override the user never requested - and it is the persisted half of W1-c.**
  - **Case 3, the round trip.** Same scenario. Open settings, select `"en"`, save: assert the recorded `set_settings.locale === "en"`, that the de heading is gone and the en heading present, and `documentElement.lang === "en"`. Then open settings again, select the system option, save: assert the recorded `set_settings.locale === null`, the de heading back, and `documentElement.lang === "de"`. No reload anywhere in this case: the live path is what it measures.
  - The file's header doc gains a paragraph naming the new subject and stating why the describe-level locale override is safe (it does not disturb the suite-wide English pinning of plan-5 D29, and the config is untouched).

- [ ] **Step 6: verification.**
  - **Absence check L1, the single resolution rule.** `grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts"`. **RED, run FIRST on the pre-state: exactly 2 lines**, both in `src/main.ts` - `resolveLocale`'s try branch and its catch branch, which are the two places the rule is written today. **GREEN on the end state: 0.** **Soundness control, because an empty grep and a broken grep look identical:** the same expression WITHOUT the filter must return exactly **2** lines from `src/i18n/index.ts` on the end state - the pre-existing occurrence in `primarySubtag`'s doc comment plus Step 1's `return saved ?? navigator.language;` - proving the pattern still matches where the token survives. **Both figures are measured on the plan's baseline commit, and the comment occurrence is the reason the control is 2 rather than 1**; an implementer whose recount disagrees returns NEEDS_CONTEXT with both runs pasted rather than adjusting the fence. **Reachable green state, argued member by member:** both pre-state occurrences sit inside `resolveLocale`, which Step 2 rewrites whole, and the replacement contains no such token.
  - The full gate as `BUILDING.md` enumerates it, foreground, green. `pnpm check:i18n` is the hard cross-locale parity gate for the new id, `pnpm lint` the D27 no-raw-text check for the new option.
  - `git diff --stat` covers exactly the six files in the Files list; anything else is a defect signal -> NEEDS_CONTEXT.
  - Report **both** existing `smoke.spec.ts` locale-control assertions by name - `toHaveValue("en")` on `localeSelect` and `toHaveValue("de")` on `reloadedLocaleSelect`, both inside the German-locale describe's settings-save case - together with the measurement showing why each stays valid: the first runs under the mock default's concrete `"en"`, the second under the case's own `DE_SETTINGS`, and shape A changes the display of neither.

- [ ] **Step 7: commit.**

```bash
git add src/i18n/index.ts src/main.ts src/components/SettingsDialog.vue locales/en/gui-settings.ftl locales/de/gui-settings.ftl e2e/locale-switch.spec.ts
git -c commit.gpgsign=false commit -m "settings: a third system-language option, so the effective locale and the shown value agree and the override stays removable" -- src/i18n/index.ts src/main.ts src/components/SettingsDialog.vue locales/en/gui-settings.ftl locales/de/gui-settings.ftl e2e/locale-switch.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the sentinel and its two mappings; the seam's name, signature and home; that the option label does not name the resolved language; the four fenced catalog strings; that `playwright.config.ts` and `e2e/mocks.ts` are untouched; that both existing locale-control assertions stay (the `"en"` one and the `"de"` one); that an out-of-band stored locale gets no handling.

---

