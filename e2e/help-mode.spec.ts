/**
 * Task 12 (D52): help mode - the sidebar, the always-visible toggle, the
 * three delegated capture listeners on <main> (hover/focus -> topic, click
 * -> pin with ALL activation suppressed per the E3 ruling), topic
 * resolution per active view, and Esc exiting help unless the settings
 * dialog is open. Drives the real served app with mocked IPC (`mocks.ts`),
 * exactly like `smoke.spec.ts`; annotations (`data-help-id`) arrive in Task
 * 13, so this spec exercises only what exists without them - which is
 * precisely the E3 global-suppression path (an unannotated control is
 * inert, no pin).
 *
 * The sidebar's rendered markup is compared against `marked` over the en
 * topic file (the `topicHtml` mechanism Task 11 landed), normalized once
 * through the page's own DOM so the check is serialization-identical to
 * what `v-html` produced - a raw-string compare would trip over the
 * browser's HTML re-serialization. The toggle's tooltip and the topic
 * files are the source of truth, never a hand-duplicated literal (the
 * binding test-discipline convention).
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";
import { marked } from "marked";
import type { Page } from "@playwright/test";
import { installTauriMocks, resolveWith } from "./mocks";
import { enAttr } from "./i18n-en";
import type { MkvmergeInfo } from "../src/ipc";

const MKVMERGE_INFO: MkvmergeInfo = { path: "/usr/bin/mkvmerge", version: "90.0.0" };

/** `marked` over the en topic file - the same string `topicHtml` produces
 *  and `v-html` assigns to the sidebar. */
function topicMarkup(helpId: string): string {
  return marked.parse(readFileSync(join("help", "en", `${helpId}.md`), "utf8"), { async: false });
}

/** Re-serializes `html` through the page DOM so it matches the sidebar's
 *  own `innerHTML` byte-for-byte (both pass through the browser's parser). */
function normalizeInPage(page: Page, html: string): Promise<string> {
  return page.evaluate((h) => {
    const div = document.createElement("div");
    div.innerHTML = h;
    return div.innerHTML;
  }, html);
}

test.describe("help mode (D52)", () => {
  test("toggle opens the sidebar on the active view's topic; the nav stays live and swaps it; a click in <main> is suppressed; Esc exits", async ({
    page,
  }) => {
    const recorded = await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });

    await page.goto("/");

    const toggle = page.getByTestId("help-toggle");
    const sidebar = page.getByTestId("help-sidebar");

    // The always-visible toggle carries aria-pressed and the Fluent tooltip.
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
    await expect(toggle).toHaveAttribute("title", enAttr("help-toggle-label", "tooltip"));

    // Toggling on shows the sidebar rendering the active (batch) view topic.
    await expect(sidebar).toBeHidden();
    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
    await expect(sidebar).toBeVisible();
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-batch")));

    // The nav stays live in help mode: switching to Jobs swaps the topic.
    await page.getByTestId("nav-jobs").click();
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-jobs")));

    // Back to batch to check activation suppression there.
    await page.getByTestId("nav-batch").click();
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-batch")));

    // A click inside <main> (the batch source-dir browse button) does NOT
    // fire its action: the mocked directory-pick IPC is never invoked, and
    // an unannotated target changes no topic (owner decision B / E3).
    await page.getByTestId("batch-source-browse").click();
    expect(recorded.some((r) => r.cmd === "plugin:dialog|open")).toBe(false);
    expect(await sidebar.innerHTML()).toBe(await normalizeInPage(page, topicMarkup("view-batch")));

    // Escape (no dialog open) exits help mode - the sidebar detaches.
    await page.keyboard.press("Escape");
    await expect(sidebar).toBeHidden();
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
  });

  test("with the settings dialog open, Escape closes the dialog and help mode stays active", async ({
    page,
  }) => {
    await installTauriMocks(page, {
      commands: {
        detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      },
    });

    await page.goto("/");

    const toggle = page.getByTestId("help-toggle");
    const sidebar = page.getByTestId("help-sidebar");

    await toggle.click();
    await expect(sidebar).toBeVisible();

    await page.getByTestId("open-settings").click();
    const dialog = page.getByTestId("settings-dialog");
    await expect(dialog).toBeVisible();

    // The native modal's own cancel semantics win: Escape closes the dialog
    // and help mode is untouched (it sits below the top layer).
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await expect(sidebar).toBeVisible();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
  });
});
