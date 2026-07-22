import { shallowRef } from "vue";
import { createFluentVue } from "fluent-vue";
import { buildBundles, primarySubtag } from "./index";

/** The app's one fluent-vue instance and reactive locale (D56). A swap
 *  REPLACES the bundles array, never mutates it (the setter is
 *  shallowRef-backed; every $t/$ta/v-t tracks it). */
export const currentLocale = shallowRef("en");
export const fluent = createFluentVue({ bundles: buildBundles("en") });

/** Switches the whole app to `locale` in place: rebuilds the fallback
 *  chain into a fresh array (never a mutation, so the shallowRef fires),
 *  updates the reactive `currentLocale` topic re-render depends on, and
 *  sets `<html lang>` to the primary subtag. No reload, no remount -- every
 *  view is `v-show`-mounted, so live state survives (D56). */
export function applyLocale(locale: string): void {
  currentLocale.value = locale;
  fluent.bundles = buildBundles(locale); // fresh array, never mutation
  document.documentElement.lang = primarySubtag(locale);
}
