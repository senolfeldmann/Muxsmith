/**
 * En-catalog loader for e2e assertions (Task 12 binding convention: "no
 * hardcoded user-facing assertions where a Fluent key exists -- assert
 * against the en catalog values"). Mirrors `src/i18n/index.ts`'s catalog
 * set exactly (`gui-*.ftl` + `diagnostics.ftl`; `cli.ftl` excluded -- the
 * frontend never renders it), using the same `@fluent/bundle` the app
 * itself renders through, so a smoke assertion is byte-identical to what a
 * user actually sees instead of a hand-duplicated string that can silently
 * drift from the catalog.
 *
 * This module is also the real-Fluent-parser completeness guard (T21
 * review) for every catalog in every `locales/<tag>/` directory: the
 * throwaway node script that proved T21's German catalogs parse cleanly
 * (task-21-report.md section 5) was never committed, so a future
 * indentation regression in a nested selector (`suggestion-partition` is
 * the deepest) would ship silently -- check-i18n.mjs's parity gate is a
 * column-0 id-line regex and cannot see a malformed multiline continuation
 * or a botched nested-selector indent (T20's recorded gap).
 *
 * IMPORTANT, verified against `@fluent/bundle`'s own runtime resource
 * parser (`node_modules/@fluent/bundle/esm/resource.js`): `addResource`'s
 * returned errors array reports ONLY duplicate-id collisions ("Attempt to
 * override an existing message/term"), never a malformed or Junk entry --
 * the lightweight runtime parser is deliberately lenient ("aims at parsing
 * valid Fluent messages with a success rate of 100%", its own source
 * comment) and silently DROPS an entry it cannot parse instead of
 * reporting it. A broken multiline continuation (e.g. an unbalanced brace)
 * was confirmed empirically to make `bundle.getMessage(id)` return
 * `undefined` while `addResource` still returns zero errors -- exactly the
 * "ships silently with per-message en fallback" failure mode T21's review
 * describes, and exactly what `addResource`-error-checking alone (as
 * `buildEnBundle` below already did, and as the review's own framing
 * assumed) cannot catch. `assertAllCatalogsParseCleanly` closes that gap
 * properly: for every id check-i18n.mjs's own column-0 scan finds in a
 * catalog's source text, it asserts the real parser actually produced a
 * message WITH a value for that id -- a Junk-dropped or truncated entry
 * fails this even when `addResource` stays silent. It walks every locale
 * directory in the same two groupings the app itself uses at runtime: the
 * frontend's combined gui-* + diagnostics bundle (mirrors
 * `src/i18n/index.ts`'s `buildBundle`) and `cli.ftl` standalone (mirrors
 * the Rust CLI's own separate bundle -- `catalog_completeness.rs` only
 * ever covers `locales/en/cli.ftl`, so a second locale's is otherwise
 * never real-parsed anywhere). `cli.ftl` is kept out of the combined
 * bundle deliberately: it shares ids with `gui-common.ftl`
 * (`identify-failed`, `mkvmerge-not-found`, `mkvmerge-query-failed`)
 * because the CLI and the frontend never load them into the same bundle
 * at runtime; merging them here would report that legitimate overlap as a
 * false "already defined" `addResource` error. Exported for `catalogs.spec.ts`'s
 * dedicated "all Fluent catalogs parse cleanly" test (moved out of a
 * module-import side effect so a catalog failure attributes to one named
 * red test instead of an opaque module-load error in every spec that
 * imports this file); `buildEnBundle` alone still runs for its own throw
 * side effect at import time, since the `en()` helper below needs its
 * memoized bundle regardless of which spec runs.
 */
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import type { FluentVariable } from "@fluent/bundle";

const LOCALES_ROOT = resolve(import.meta.dirname, "../locales");
const LOCALES_EN = join(LOCALES_ROOT, "en");

// Mirrors check-i18n.mjs's MESSAGE_ID_RE exactly (column-0 `id =` lines
// only -- see that script's PARSING CONSTRAINT comment for why this is
// deliberate and what it does and doesn't match).
const MESSAGE_ID_RE = /^([A-Za-z][A-Za-z0-9_-]*)\s*=/;

function scanIds(text: string): string[] {
  const ids: string[] = [];
  for (const line of text.split("\n")) {
    const m = MESSAGE_ID_RE.exec(line);
    if (m) {
      ids.push(m[1]);
    }
  }
  return ids;
}

/**
 * Parses `files` (full paths) into one `FluentBundle` for `locale`,
 * throwing on any `addResource` error (a real id collision -- see the
 * module doc for why that is the ONLY thing `addResource` ever reports)
 * or on any id `scanIds` found in the source text that the real parser
 * did not turn into a message with a value (a Junk-dropped or truncated
 * entry). `label` names the grouping being checked, for the error message.
 */
function parseOrThrow(locale: string, files: string[], label: string): FluentBundle {
  const bundle = new FluentBundle(locale);
  const expectedIds: string[] = [];
  for (const file of files) {
    const text = readFileSync(file, "utf8");
    expectedIds.push(...scanIds(text));
    const errors = bundle.addResource(new FluentResource(text));
    if (errors.length > 0) {
      throw new Error(`e2e/i18n-en: catalog error in ${label}: ${errors.map(String).join("; ")}`);
    }
  }
  const droppedIds = expectedIds.filter((id) => bundle.getMessage(id)?.value == null);
  if (droppedIds.length > 0) {
    throw new Error(
      `e2e/i18n-en: ${label} declares ${droppedIds.join(", ")} but the real Fluent parser ` +
        `produced no message value for ${droppedIds.length === 1 ? "it" : "them"} -- ` +
        "a malformed multiline continuation or nested-selector indent, most likely.",
    );
  }
  return bundle;
}

function buildEnBundle(): FluentBundle {
  const files = readdirSync(LOCALES_EN)
    .filter((f) => f.endsWith(".ftl") && (f.startsWith("gui-") || f === "diagnostics.ftl"))
    .sort()
    .map((f) => join(LOCALES_EN, f));
  return parseOrThrow("en", files, "locales/en/{gui-*,diagnostics}.ftl");
}

export function assertAllCatalogsParseCleanly(): void {
  const locales = readdirSync(LOCALES_ROOT, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();
  for (const locale of locales) {
    const dir = join(LOCALES_ROOT, locale);
    const allFiles = readdirSync(dir)
      .filter((f) => f.endsWith(".ftl"))
      .sort();
    const guiAndDiagnostics = allFiles
      .filter((f) => f.startsWith("gui-") || f === "diagnostics.ftl")
      .map((f) => join(dir, f));
    parseOrThrow(locale, guiAndDiagnostics, `locales/${locale}/{gui-*,diagnostics}.ftl`);
    if (allFiles.includes("cli.ftl")) {
      parseOrThrow(locale, [join(dir, "cli.ftl")], `locales/${locale}/cli.ftl`);
    }
  }
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
