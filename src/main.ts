import { createApp } from "vue";
import { createFluentVue } from "fluent-vue";
import App from "./App.vue";
import { buildBundles } from "./i18n";
import { getSettings } from "./ipc";

/**
 * Resolves the locale before the app ever mounts (spec 8.4: "system locale
 * with manual override in app settings ... falls back to English per
 * message"), so the very first paint already uses the right catalog chain
 * instead of flashing English and re-rendering. `get_settings` failing
 * (e.g. no resolvable platform config dir) is not a startup blocker --
 * `buildBundles` already falls back to "en" for any locale it cannot
 * resolve, exactly as it would for an unset setting.
 */
async function resolveLocale(): Promise<string | null> {
  try {
    return (await getSettings()).locale ?? navigator.language;
  } catch {
    return navigator.language;
  }
}

async function bootstrap() {
  const locale = await resolveLocale();
  const fluent = createFluentVue({ bundles: buildBundles(locale) });
  createApp(App).use(fluent).mount("#app");
}

void bootstrap();
