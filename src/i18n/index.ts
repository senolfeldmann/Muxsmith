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

/**
 * BCP-47 primary language subtag (everything before the first "-"),
 * lowercased. A saved setting or `navigator.language` is often
 * region-qualified ("de-DE", "de-AT", "en-US"), but catalogs live under a
 * primary-subtag directory ("locales/de", "locales/en"); matching the full
 * tag verbatim against the directory name would skip the catalog and fall
 * through to English (docs-tree S15 / Plan 5 task-9 review, latent until a
 * second locale landed with Task 21). German regional variants share one
 * CLDR plural-rule set, so collapsing the region is lossless for the
 * locale negotiation this loader does.
 */
function primarySubtag(locale: string): string {
  return locale.split("-")[0].toLowerCase();
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
 * As of the German locale (Task 21), a non-English primary subtag (e.g.
 * "de", normalized by `primarySubtag` from a "de-DE"/"de-AT" system tag)
 * resolves to its own catalog directory and this returns a two-bundle
 * chain (requested locale, then "en"); an unknown tag still resolves to a
 * single "en" bundle. A further locale stays pure content under a new
 * `locales/<tag>/` directory.
 */
export function buildBundles(locale: string | null | undefined): FluentBundle[] {
  const requested = [locale, "en"]
    .filter((tag): tag is string => typeof tag === "string" && tag.length > 0)
    .map(primarySubtag);
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
