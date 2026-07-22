/**
 * Task 15 (D58): curated-domain dropdowns in exact-match value cells.
 * Mounts `PropertyMapWidget` in isolation through the wave-3 mount harness
 * (`mount.ts`), feeding it a field `spec`, the D57 `path` prop, and an
 * initial model. The value cell resolves to a `<select>` iff all four D58
 * conditions hold (matchable+scalar widget; `tracks[` path; key exactly
 * `type`/`codec_kind`; value `""` or a domain member); every other
 * combination keeps its existing text/typed cell. The closed condition
 * matrix (plan Task 15 step 3) is one `test` per row, plus the two
 * write-path behaviours.
 *
 * The value cell keeps `data-testid="property-map-value"` across every
 * resolved variant (the widget's own convention), so a case is identified
 * by the cell's TAG (`SELECT` vs `INPUT`) and, for a select, its option
 * set -- not by a variant-specific locator.
 */
import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";
import { mountComponent, readEmitted, readModel } from "./mount";
import * as registries from "../src/editor/registries";
import { CODEC_KIND_NAMES, TYPE_VALUES } from "../src/bindings/settables";
import type { FieldSpec } from "../src/editor/fieldSpec";
import type { Scalar } from "../src/bindings/profile";

const EXACT = registries.matchExprFields.exact; // matchable + scalar
const SUBSTRING = registries.matchExprFields.substring; // matchable + string
const CHANGES = registries.trackRuleFields.changes; // settable + scalar

async function mountMap(
  page: Page,
  spec: FieldSpec,
  path: string,
  model: Record<string, Scalar>,
): Promise<void> {
  await mountComponent(page, {
    component: "PropertyMapWidget",
    props: { spec, path, modelValue: model },
  });
}

function valueCell(page: Page) {
  return page.getByTestId("property-map-value");
}

function cellTag(page: Page): Promise<string> {
  return valueCell(page).evaluate((el) => el.tagName);
}

test("case 1: fresh type row on a track exact-match cell renders a select with the 4 domain options plus an empty placeholder", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { type: "" });
  expect(await cellTag(page)).toBe("SELECT");
  await expect(valueCell(page).locator("option")).toHaveCount(TYPE_VALUES.length + 1);
  await expect(valueCell(page).locator('option[value=""]')).toHaveCount(1);
  for (const v of TYPE_VALUES) {
    await expect(valueCell(page).locator(`option[value="${v}"]`)).toHaveCount(1);
  }
});

test("case 2: an in-domain type value renders a select with that value selected and no placeholder", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { type: "audio" });
  expect(await cellTag(page)).toBe("SELECT");
  await expect(valueCell(page)).toHaveValue("audio");
  await expect(valueCell(page).locator("option")).toHaveCount(TYPE_VALUES.length);
  await expect(valueCell(page).locator('option[value=""]')).toHaveCount(0);
});

test("case 3: a fresh codec_kind row renders a select with all 17 alias options plus a placeholder", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { codec_kind: "" });
  expect(await cellTag(page)).toBe("SELECT");
  await expect(valueCell(page).locator("option")).toHaveCount(CODEC_KIND_NAMES.length + 1);
  await expect(valueCell(page).locator('option[value=""]')).toHaveCount(1);
});

test("case 4: a raw:type key keeps its free-text cell (byte equality; raw: bypass preserved)", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { "raw:type": "" });
  expect(await cellTag(page)).toBe("INPUT");
  await expect(valueCell(page)).toHaveValue("");
});

test("case 5: an out-of-domain type value stays a text input with the value intact (never eaten)", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { type: "vido" });
  expect(await cellTag(page)).toBe("INPUT");
  await expect(valueCell(page)).toHaveValue("vido");
});

test("case 6: the same type row outside a track context (attachment path) stays a text input", async ({
  page,
}) => {
  await mountMap(page, EXACT, "attachments.rules[0].select", { type: "" });
  expect(await cellTag(page)).toBe("INPUT");
});

test("case 7: a substring (values:string) cell is outside the decree and stays a text input", async ({
  page,
}) => {
  await mountMap(page, SUBSTRING, "tracks[0].match.substring", { type: "" });
  expect(await cellTag(page)).toBe("INPUT");
});

test("case 8: a changes (settable) cell is unchanged (settables carry no type/codec_kind)", async ({
  page,
}) => {
  await mountMap(page, CHANGES, "tracks[0].changes", { language: "" });
  expect(await cellTag(page)).toBe("INPUT");
});

test("selecting a domain member writes the string value exactly as the text cell does", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { type: "" });
  await valueCell(page).selectOption("video");
  expect(await readModel(page)).toEqual({ type: "video" });
  const emitted = await readEmitted(page);
  expect(emitted.at(-1)?.payload).toEqual({ type: "video" });
});

test("correcting an out-of-domain value to a domain member re-resolves the cell to a select", async ({
  page,
}) => {
  await mountMap(page, EXACT, "tracks[0].match.exact", { type: "vido" });
  expect(await cellTag(page)).toBe("INPUT");
  await valueCell(page).fill("video");
  expect(await readModel(page)).toEqual({ type: "video" });
  expect(await cellTag(page)).toBe("SELECT");
  await expect(valueCell(page)).toHaveValue("video");
});
