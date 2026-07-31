import { defineConfig } from "eslint/config";
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
export default defineConfig(
  {
    // .worktrees/: SDD worktree checkouts (main checkout only) carry their
    // own target/ with rustdoc-generated JS that eslint must never see.
    // e2e/.generated/: the bundled, gitignored mock harness build (Task
    // 12) -- vendored @tauri-apps/api code via vite.harness.config.ts,
    // never hand-authored, never linted.
    ignores: [
      "dist/**",
      "src-tauri/**",
      "node_modules/**",
      "target/**",
      ".worktrees/**",
      "e2e/.generated/**",
      "playwright-report/**",
      "test-results/**",
    ],
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
      // D112 (owner ruling 2026-07-31): the pre-session state is ONE named
      // computed, `nothingOpenedOrCreated`, and a render gate that asks
      // `!model` directly is the defect that decision exists to remove --
      // that expression is also true after a load that failed to parse,
      // where the editor must NOT offer its pre-session surfaces. Scoped by
      // directive name to `v-if`/`v-else-if`, so the `:disabled="!model ||
      // !canUndo"` bindings D108 decision 10 requires stay legal: those gate
      // an ACTION on whether there is content, not a RENDER on whether
      // anything was ever opened or created.
      "vue/no-restricted-syntax": [
        "error",
        {
          selector:
            "VAttribute[directive=true][key.name.name=/^(if|else-if)$/] UnaryExpression[operator='!'] > Identifier[name='model']",
          message:
            "A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112).",
        },
      ],
      // `attributes` defaults to none checked (the rule only scans text
      // nodes out of the box); D29 requires accessible names/tooltips to
      // come from Fluent too, so a STATIC (non-`:`-bound) title/aria-label/
      // placeholder/alt is flagged like any other raw string. A `:title=`/
      // `:aria-label=` binding (the correct form, used throughout this
      // codebase) is a directive and outside this rule's reach regardless
      // -- this option only closes the "forgot to bind it" gap.
      "@intlify/vue-i18n/no-raw-text": [
        "error",
        {
          attributes: {
            "/.*/": ["title", "aria-label", "placeholder", "alt"],
          },
        },
      ],
    },
  },
);
