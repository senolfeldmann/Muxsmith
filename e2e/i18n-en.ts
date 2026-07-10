/**
 * En-catalog loader for e2e assertions (Task 12 binding convention: "no
 * hardcoded user-facing assertions where a Fluent key exists -- assert
 * against the en catalog values"). Mirrors `src/i18n/index.ts`'s catalog
 * set exactly (`gui-*.ftl` + `diagnostics.ftl`; `cli.ftl` excluded -- the
 * frontend never renders it), using the same `@fluent/bundle` the app
 * itself renders through, so a smoke assertion is byte-identical to what a
 * user actually sees instead of a hand-duplicated string that can silently
 * drift from the catalog.
 */
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import type { FluentVariable } from "@fluent/bundle";

const LOCALES_EN = resolve(dirname(fileURLToPath(import.meta.url)), "../locales/en");

function buildEnBundle(): FluentBundle {
  const bundle = new FluentBundle("en");
  const files = readdirSync(LOCALES_EN)
    .filter((f) => f.endsWith(".ftl") && (f.startsWith("gui-") || f === "diagnostics.ftl"))
    .sort();
  for (const file of files) {
    const errors = bundle.addResource(
      new FluentResource(readFileSync(join(LOCALES_EN, file), "utf8")),
    );
    if (errors.length > 0) {
      throw new Error(`e2e/i18n-en: catalog error in ${file}: ${errors.map(String).join("; ")}`);
    }
  }
  return bundle;
}

const bundle = buildEnBundle();

/**
 * Renders one message id through the real en catalog, exactly as the
 * app's own `$t(id, params)` would (fluent-vue delegates to the same
 * `FluentBundle.formatPattern`). Throws on a missing id or a formatting
 * error rather than returning the id itself -- a test fixture referencing
 * a nonexistent key is a test bug to fail loudly on, not a value to
 * silently degrade.
 */
export function en(id: string, args?: Record<string, FluentVariable>): string {
  const message = bundle.getMessage(id);
  if (!message || !message.value) {
    throw new Error(`e2e/i18n-en: no message "${id}" in the en catalog`);
  }
  const errors: Error[] = [];
  const text = bundle.formatPattern(message.value, args, errors);
  if (errors.length > 0) {
    throw new Error(`e2e/i18n-en: format error for "${id}": ${errors.map(String).join("; ")}`);
  }
  return text;
}
