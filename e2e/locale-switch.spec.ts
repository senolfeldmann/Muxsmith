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
import { FluentBundle, FluentResource } from "@fluent/bundle";
import type { FluentVariable } from "@fluent/bundle";
import { installTauriMocks, resolveWith } from "./mocks";
import { en } from "./i18n-en";
import type { MkvmergeInfo } from "../src/ipc";

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
