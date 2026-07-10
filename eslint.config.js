import pluginVue from "eslint-plugin-vue";
import vueI18n from "@intlify/eslint-plugin-vue-i18n";
import tseslint from "typescript-eslint";

// D27 (no hardcoded user-facing strings): Muxsmith uses Fluent (.ftl), not
// vue-i18n, so we take only `@intlify/eslint-plugin-vue-i18n`'s
// `no-raw-text` rule rather than its `recommended`/`base` presets, which
// pull in message-catalog rules (`no-unused-keys`, `no-missing-keys`, ...)
// that expect vue-i18n's JSON/YAML catalogs via `settings['vue-i18n']
// .localeDir`. `no-raw-text` itself only scans Vue template text nodes; it
// has no dependency on the vue-i18n runtime or catalog format, so it works
// standalone here (verified empirically: a raw template string trips this
// rule with no other vue-i18n config present).
export default tseslint.config(
  {
    ignores: ["dist/**", "src-tauri/**", "node_modules/**", "target/**"],
  },
  // typescript-eslint's base config sets `languageOptions.parser` with no
  // `files` restriction, so it must come before the Vue-specific blocks
  // below; otherwise it wins the flat-config merge for *.vue too and
  // vue-eslint-parser never sees the SFC (`<script setup>` then fails to
  // parse as if it were bare TS).
  ...tseslint.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  {
    files: ["**/*.vue"],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },
  {
    files: ["**/*.vue"],
    plugins: {
      "@intlify/vue-i18n": vueI18n,
    },
    rules: {
      "@intlify/vue-i18n/no-raw-text": "error",
    },
  },
);
