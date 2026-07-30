import { createApp } from "vue";
import App from "./App.vue";
import { effectiveLocale } from "./i18n";
import { applyLocale, fluent } from "./i18n/fluent";
import { getSettings } from "./ipc";

/**
 * Resolves the locale before the app ever mounts (spec 8.4: "system locale
 * with manual override in app settings ... falls back to English per
 * message"), so the very first paint already uses the right catalog chain
 * instead of flashing English and re-rendering. The "absent means the
 * system language" rule itself is not restated here: `effectiveLocale`
 * owns it, and the settings dialog's live switch calls the same seam
 * (D106). `get_settings` failing (e.g. no resolvable platform config dir)
 * is not a startup blocker -- an unreadable settings file carries no
 * override either, so the catch resolves exactly as an unset setting does,
 * and `buildBundles` still falls back to "en" for any locale it cannot
 * resolve.
 */
async function resolveLocale(): Promise<string> {
  try {
    return effectiveLocale((await getSettings()).locale);
  } catch {
    return effectiveLocale(null);
  }
}

async function bootstrap() {
  const locale = await resolveLocale();
  applyLocale(locale);
  createApp(App).use(fluent).mount("#app");
}

void bootstrap();
