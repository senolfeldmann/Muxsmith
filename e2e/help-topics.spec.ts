import { readFileSync } from "node:fs";
import { join } from "node:path";
import { expect, test } from "@playwright/test";
import { marked } from "marked";
import { loadHarness } from "./mount"; // added by this task's Step 4

test("topicHtml renders locale topic, falls back per topic to en, then to the raw id", async ({ page }) => {
  await loadHarness(page);
  const html = (id: string, locale: string) =>
    page.evaluate(([i, l]) => window.__muxsmithTopicHtml__(i, l), [id, locale]);
  const en = marked.parse(readFileSync(join("help", "en", "view-batch.md"), "utf8"), { async: false });
  const de = marked.parse(readFileSync(join("help", "de", "view-batch.md"), "utf8"), { async: false });
  expect(await html("view-batch", "en")).toBe(en);
  expect(await html("view-batch", "de")).toBe(de);
  expect(await html("view-batch", "de-AT")).toBe(de);     // primary-subtag collapse
  expect(await html("view-batch", "fr")).toBe(en);        // unknown locale -> en topic
  expect(await html("no-such-topic", "en")).toBe("no-such-topic"); // raw id, never blank
});
