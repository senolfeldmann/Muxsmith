import { FluentBundle, FluentResource } from "@fluent/bundle";

/**
 * Catalog source of truth (spec 8.4): `locales/<locale>/{gui-*,diagnostics}.ftl`,
 * the same tree the CLI embeds at build time
 * (`crates/muxsmith-cli/src/i18n.rs`). `cli.ftl` is deliberately excluded:
 * it is CLI-only vocabulary the frontend never renders.
 *
 * Globbed by locale directory rather than a hardcoded "en" import list, so
 * a v1.x locale addition is pure `.ftl` content under a new
 * `locales/<tag>/` directory; this loader needs no change (spec 11: "the
 * mechanism ships complete; adding a locale is content work, not a
 * refactor"). It also means a later view task's own `gui-<view>.ftl`
 * (T10's `gui-batch.ftl`, T11's `gui-jobs.ftl`) is picked up automatically,
 * with no edit here either.
 */
const catalogSources = import.meta.glob(
  ["../../locales/*/gui-*.ftl", "../../locales/*/diagnostics.ftl"],
  { query: "?raw", import: "default", eager: true },
);

const LOCALE_DIR = /\/locales\/([^/]+)\//;

function catalogsForLocale(locale: string): string[] {
  return Object.keys(catalogSources)
    .filter((path) => LOCALE_DIR.exec(path)?.[1] === locale)
    .sort()
    .map((path) => catalogSources[path]);
}

function buildBundle(locale: string): FluentBundle | null {
  const sources = catalogsForLocale(locale);
  if (sources.length === 0) {
    return null;
  }
  const bundle = new FluentBundle(locale);
  for (const source of sources) {
    const errors = bundle.addResource(new FluentResource(source));
    for (const error of errors) {
      // A catalog parse/collision problem is a build-time bug, not a
      // reason to crash the app; surfaced for visibility, mirroring the
      // CLI's own missing-key fallback (never silently hide a problem).
      console.warn(`[i18n] catalog error in locale "${locale}":`, error);
    }
  }
  return bundle;
}

/**
 * Builds the fallback chain fluent-vue negotiates every message against
 * (spec 8.4: "falls back to English per message"): the requested/system
 * locale first, then "en" unconditionally. fluent-vue documents `bundles`
 * as "current negotiated fallback chain of languages" -- a message missing
 * from the first bundle falls through to the next, so this is real
 * per-message fallback, not just a startup default.
 *
 * v1 ships English content only (non-goal 11), so `catalogsForLocale`
 * currently only ever resolves "en" and this always returns a single
 * bundle; the chain shape is what makes a later locale addition pure
 * content instead of a loader change.
 */
export function buildBundles(locale: string | null | undefined): FluentBundle[] {
  const requested = [locale, "en"].filter(
    (tag): tag is string => typeof tag === "string" && tag.length > 0,
  );
  const seen = new Set<string>();
  const bundles: FluentBundle[] = [];
  for (const tag of requested) {
    if (seen.has(tag)) {
      continue;
    }
    seen.add(tag);
    const bundle = buildBundle(tag);
    if (bundle) {
      bundles.push(bundle);
    }
  }
  return bundles;
}
