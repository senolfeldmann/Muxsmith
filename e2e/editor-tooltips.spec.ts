import { expect, test } from "@playwright/test";
import { enAttr } from "./i18n-en";
import { mountWidget } from "./mount";
import * as registries from "../src/editor/registries";

const SAMPLE_VALUE: Record<string, unknown> = {
  text: "",
  bool: false,
  optionalFlag: undefined,
  select: undefined, // SelectWidget renders its options from the spec
  keywordOrBlock: "keep",
  directoryPath: "",
  stringList: [],
  propertyMap: {},
  list: [],
  section: {},
};

const REGISTRIES = [
  registries.profileFields, registries.metaFields, registries.inputFields,
  registries.outputFields, registries.templateBlockFields,
  registries.externalBlockFields, registries.trackRuleFields,
  registries.locatorFields, registries.attachmentsFields,
  registries.tracksFields, registries.attachmentRuleFields,
  registries.tagsFields, registries.matchExprFields,
] as const;

test("every editable field renders its label message's .tooltip as title", async ({ page }) => {
  for (const registry of REGISTRIES) {
    for (const spec of Object.values(registry)) {
      if ("fixed" in spec) continue; // the sole FixedField, profile_version
      await mountWidget(page, spec, SAMPLE_VALUE[spec.widget.kind]);
      const expected = enAttr(spec.labelKey, "tooltip");
      const titles = await page
        .locator("[title]")
        .evaluateAll((els) => els.map((e) => e.getAttribute("title")));
      expect(titles, `labelKey ${spec.labelKey}`).toContain(expected);
    }
  }
});
