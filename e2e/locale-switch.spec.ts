/**
 * Task 7 (D56): the live locale switch. Selecting German in the settings
 * dialog and saving swaps the catalog IN PLACE -- the running app
 * re-renders in German with no reload and no lost view state, and
 * `<html lang>` follows. Drives the real served app with mocked IPC
 * (`mocks.ts`), exactly like `smoke.spec.ts`.
 *
 * The expected German text is derived from the real `locales/de` catalog
 * through `@fluent/bundle` (a local de-bundle helper mirroring
 * `i18n-en.ts`'s `buildEnBundle`), never a hardcoded string, so the
 * assertion is byte-identical to what the app itself renders. The
 * interaction strings (button/label names to click) come from the en
 * catalog via `i18n-en.ts`'s `en()`, the same test-discipline convention
 * `smoke.spec.ts` follows.
 *
 * Plan 12 W1 (D106) adds a second subject to this file: the SYSTEM-language
 * default, i.e. the state where nothing has ever been saved on the machine
 * (`locale: null`) and the interface must follow the operating system while
 * the control shows exactly that, and saving must not silently mint an
 * override. Those cases need a non-English system language, which the
 * suite does not otherwise have: `playwright.config.ts` pins `en-US` and
 * plan-5 D29 pins the test locale to English deliberately, so role names
 * match the en catalog. A describe-level `test.use({ locale: "de-DE" })`
 * is the additive route -- it applies to the cases inside that describe
 * only, leaves every sibling case (this file's own D56 case included) on
 * the config's `en-US`, and needs no edit to `playwright.config.ts` or to
 * `mocks.ts`'s default scenario. Every German string those cases assert is
 * one whose de value DIFFERS from its en value, because `buildBundles`
 * negotiates `[requested, en]` per message: an id whose two values are
 * identical would assert green even if the interface had fallen back to
 * English entirely.
 *
 * Inside that describe the en()-only line above does not apply to
 * INTERACTION LOCATORS, and the reason is that its premise is gone: a
 * locator for "Save" cannot find a button labelled "Speichern". What the
 * convention encodes is "resolve through the catalog, never hardcode a
 * literal", and that is preserved -- an interaction locator there resolves
 * through the catalog of the locale actually RENDERING at that point in the
 * case, `de(...)` while the interface is German and `en(...)` once it has
 * switched (controller ruling 1, Plan 12 Task 2). The rule for ASSERTIONS is
 * untouched and still binds exactly as stated above; an interaction locator
 * is not one of a case's assertions and is never counted as evidence that a
 * given bundle is live.
 *
 * "Not reloaded" is proven by a `document.body` dataset marker set before
 * the switch: a live bundle swap never touches `document.body`, but a full
 * `page.reload()` recreates the document and wipes it. The marker sits
 * outside `#app` deliberately -- Vue's in-place re-render only rewrites the
 * component tree under the mount point, so a marker on `body` is the clean
 * survives-a-swap / dies-on-reload witness the design's "views keep state"
 * clause needs.
 */
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import type { FluentVariable } from "@fluent/bundle";
import { installTauriMocks, resolveWith } from "./mocks";
import { en } from "./i18n-en";
import type { AppSettings, MkvmergeInfo } from "../src/ipc";

const LOCALES_DE = resolve(import.meta.dirname, "../locales/de");

/** Mirrors `i18n-en.ts`'s `buildEnBundle` for the de catalog: one
 *  `FluentBundle` over `locales/de/{gui-*,diagnostics}.ftl`, the exact set
 *  `src/i18n/index.ts` loads at runtime. Throws on an `addResource`
 *  collision -- the only thing that parser reports (see `i18n-en.ts`). */
function buildDeBundle(): FluentBundle {
  const bundle = new FluentBundle("de");
  const files = readdirSync(LOCALES_DE)
    .filter((f) => f.endsWith(".ftl") && (f.startsWith("gui-") || f === "diagnostics.ftl"))
    .sort()
    .map((f) => join(LOCALES_DE, f));
  for (const file of files) {
    const errors = bundle.addResource(new FluentResource(readFileSync(file, "utf8")));
    if (errors.length > 0) {
      throw new Error(`e2e/locale-switch: de catalog error: ${errors.map(String).join("; ")}`);
    }
  }
  return bundle;
}

const deBundle = buildDeBundle();

/** Renders one de message exactly as the app's own `$t(id, args)` would;
 *  throws on a missing id, a test-fixture bug to surface loudly. */
function de(id: string, args?: Record<string, FluentVariable>): string {
  const message = deBundle.getMessage(id);
  if (!message || !message.value) {
    throw new Error(`e2e/locale-switch: no de message "${id}"`);
  }
  const errors: Error[] = [];
  const text = deBundle.formatPattern(message.value, args, errors);
  if (errors.length > 0) {
    throw new Error(`e2e/locale-switch: de format error for "${id}": ${errors.map(String).join("; ")}`);
  }
  return text;
}

const MKVMERGE_INFO: MkvmergeInfo = { path: "/usr/bin/mkvmerge", version: "90.0.0" };

test.describe("live locale switch (D56)", () => {
  test("selecting German in settings swaps the catalog in place: no reload, view state survives, html lang follows", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    // Starts in English.
    await expect(batch.getByRole("heading", { name: en("batch-view-heading"), exact: true })).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.lang)).toBe("en");

    // A marker outside #app: survives a live catalog swap, wiped by a reload.
    await page.evaluate(() => {
      document.body.dataset.localeSwitchAlive = "yes";
    });

    // Open settings, pick German, save.
    await page.getByTestId("open-settings").click();
    const dialog = page.getByTestId("settings-dialog");
    await expect(dialog).toBeVisible();
    await dialog.locator("select#settings-locale").selectOption("de");
    await dialog.getByRole("button", { name: en("settings-save"), exact: true }).click();
    await expect(dialog).toBeHidden();

    // (a) The visible heading re-rendered to its de value in place; the en
    // value is gone (a real re-render, not an addition).
    await expect(batch.getByRole("heading", { name: de("batch-view-heading"), exact: true })).toBeVisible();
    await expect(
      batch.getByRole("heading", { name: en("batch-view-heading"), exact: true }),
    ).toHaveCount(0);

    // (b) No reload happened: the pre-switch body marker is still there.
    expect(await page.evaluate(() => document.body.dataset.localeSwitchAlive)).toBe("yes");

    // (c) The document language followed the switch.
    expect(await page.evaluate(() => document.documentElement.lang)).toBe("de");
  });
});

/** Nothing ever saved on this machine: the stored override is absent, which
 *  is what "follow the system language" IS (spec 8.2). Supplied per
 *  scenario rather than by changing `mocks.ts`'s default `locale: "en"`,
 *  which every other scenario in the suite inherits (D106 decision 6). */
const SYSTEM_SETTINGS: AppSettings = {
  mkvmerge_path: null,
  default_jobs: 1,
  locale: null,
  recent_profiles: [],
  dir_memory: {},
};

/** The state the "leaving the system language" case below persists: an
 *  explicit English override on a machine whose system language is German.
 *  It is the starting scenario of the opposite direction rather than a
 *  continuation of one page, because this mock is stateless -- `set_settings`
 *  does not feed `get_settings`, and `open()` re-reads the baseline, so a
 *  second save inside one page would compare `null` against `null`, the
 *  live-switch guard would stay false, and the merged case would go RED
 *  against a correct implementation: its `set_settings` assertion still
 *  passes (`null` is persisted), while both rendering assertions fail --
 *  the German heading never comes back and `<html lang>` stays `en`. So the
 *  split is here because that direction cannot be made green inside one page
 *  without a reload or a second fixture state, NOT because a merged form
 *  would be a false green (controller ruling 1, Plan 12 Task 2). */
const EN_OVERRIDE_SETTINGS: AppSettings = { ...SYSTEM_SETTINGS, locale: "en" };

test.describe("system-locale default (D106)", () => {
  test.use({ locale: "de-DE" });

  /** Installs one stored-settings state under the describe's German system
   *  language. Returns the Node-side call log, so a save assertion reads the
   *  real `set_settings` invocation rather than a UI echo. */
  function installStored(page: Page, settings: AppSettings) {
    return installTauriMocks(page, {
      commands: {
        get_settings: [resolveWith(settings)],
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });
  }

  const saved = (recorded: { cmd: string; args: unknown }[]) =>
    recorded
      .filter((r) => r.cmd === "set_settings")
      .map((r) => (r.args as { settings: AppSettings }).settings);

  test("first run: the interface follows the system language and the control says so", async ({
    page,
  }) => {
    await installStored(page, SYSTEM_SETTINGS);

    await page.goto("/");

    // (a) batch-view-heading is "Batch" (en) vs "Stapel" (de) -- a value the
    // en fallback cannot produce, so this fails if the de bundle is not the
    // one actually rendering.
    const batch = page.getByTestId("view-batch");
    await expect(batch.getByRole("heading", { name: de("batch-view-heading"), exact: true })).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.lang)).toBe("de");

    // (b) The control agrees with what the user sees: the sentinel is
    // selected, and its label is the de one.
    await page.getByTestId("open-settings").click();
    const dialog = page.getByTestId("settings-dialog");
    await expect(dialog).toBeVisible();
    const localeSelect = dialog.locator("select#settings-locale");
    await expect(localeSelect).toHaveValue("");
    await expect(localeSelect.locator("option:checked")).toHaveText(
      de("settings-locale-option-system"),
    );
  });

  test("saving without touching the language writes no override", async ({ page }) => {
    const recorded = await installStored(page, SYSTEM_SETTINGS);

    await page.goto("/");

    await page.getByTestId("open-settings").click();
    const dialog = page.getByTestId("settings-dialog");
    await expect(dialog).toBeVisible();
    await dialog.locator("input#settings-default-jobs").fill("3");
    // The interface is German here, so the Save control resolves through the
    // de catalog (see this file's header).
    await dialog.getByRole("button", { name: de("settings-save"), exact: true }).click();
    await expect(dialog).toBeHidden();

    // The defect's core: the first Save used to mint an override the user
    // never requested, permanently removing system-following from the UI.
    const writes = saved(recorded);
    expect(writes).toHaveLength(1);
    expect(writes[0].locale).toBeNull();
    // Control: the save really carried the edit, so the `locale` assertion
    // above is not green merely because nothing was saved at all.
    expect(writes[0].default_jobs).toBe(3);
  });

  // The two directions across the sentinel run as two cases, one scenario
  // each, both on the LIVE path with no reload anywhere -- see
  // EN_OVERRIDE_SETTINGS for why one page cannot carry both.

  test("leaving the system language: picking English stores the override and switches live", async ({
    page,
  }) => {
    const recorded = await installStored(page, SYSTEM_SETTINGS);

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    const dialog = page.getByTestId("settings-dialog");

    await page.getByTestId("open-settings").click();
    await expect(dialog).toBeVisible();
    await dialog.locator("select#settings-locale").selectOption("en");
    // Still German at the moment of the click: the switch happens on save.
    await dialog.getByRole("button", { name: de("settings-save"), exact: true }).click();
    await expect(dialog).toBeHidden();

    expect(saved(recorded)).toHaveLength(1);
    expect(saved(recorded)[0].locale).toBe("en");
    await expect(batch.getByRole("heading", { name: en("batch-view-heading"), exact: true })).toBeVisible();
    await expect(
      batch.getByRole("heading", { name: de("batch-view-heading"), exact: true }),
    ).toHaveCount(0);
    expect(await page.evaluate(() => document.documentElement.lang)).toBe("en");
  });

  test("returning to the system language: removing the override stores null and switches live", async ({
    page,
  }) => {
    const recorded = await installStored(page, EN_OVERRIDE_SETTINGS);

    await page.goto("/");

    const batch = page.getByTestId("view-batch");
    const dialog = page.getByTestId("settings-dialog");

    // Red-state control for the German assertions below: the stored override
    // really is in force first, so "the German heading is back" cannot be
    // green on a page that was German all along.
    await expect(batch.getByRole("heading", { name: en("batch-view-heading"), exact: true })).toBeVisible();
    await expect(
      batch.getByRole("heading", { name: de("batch-view-heading"), exact: true }),
    ).toHaveCount(0);
    expect(await page.evaluate(() => document.documentElement.lang)).toBe("en");

    await page.getByTestId("open-settings").click();
    await expect(dialog).toBeVisible();
    const localeSelect = dialog.locator("select#settings-locale");
    await expect(localeSelect).toHaveValue("en");
    await localeSelect.selectOption("");
    // English at the moment of the click, so the Save control resolves
    // through the en catalog here.
    await dialog.getByRole("button", { name: en("settings-save"), exact: true }).click();
    await expect(dialog).toBeHidden();

    // The override is removable -- the two-option control made this state
    // unreachable forever once anything had been saved.
    expect(saved(recorded)).toHaveLength(1);
    expect(saved(recorded)[0].locale).toBeNull();
    await expect(batch.getByRole("heading", { name: de("batch-view-heading"), exact: true })).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.lang)).toBe("de");
  });
});
