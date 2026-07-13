# Idiomacy verify-45 — BatchView.vue:53 `withDefaults` vs reactive props destructure

**Verdict: REFUTED**

Finding (tag=idiom): `withDefaults(defineProps<{ runActive?: boolean }>(), { runActive: false })`
is "the Vue-3.4-and-below form"; pinned Vue 3.5.39 should declare defaults via reactive props
destructure `const { runActive = false } = defineProps<...>()`, with `props.runActive` in
`runDisabledReason` becoming plain `runActive`.

## What checks out (not the reason for refutal)

- **Code quoted correctly (criterion a passes).** `src/views/BatchView.vue:53` is verbatim the
  cited `withDefaults(...)` call. `props` is referenced exactly once, at line 241
  (`if (props.runActive)`); nowhere else in script or template. The mechanical rewrite the
  finding proposes would be complete and correct.
- **Toolchain supports the replacement.** Vue pinned `3.5.39`, `@vitejs/plugin-vue` `6.0.7`,
  `vite.config.ts` has plain `plugins: [vue()]` with no `propsDestructure: false` override.
  Reactive props destructure is stabilized and default-on in 3.5. The finding's toolchain
  premise is accurate.

## Why it is refuted (criterion b, holistically applied)

The proposed replacement is a *valid* Vue 3.5 idiom in the abstract, but it is not "the" current
idiom for this pinned toolchain-and-codebase, and the finding's premise that the existing code is
non-idiomatic is false on two independent grounds:

1. **`withDefaults` is not deprecated or superseded in Vue 3.5.** Verified against the live docs
   (`https://vuejs.org/guide/typescript/composition-api.html`, "Props Default Values" — fetched,
   not from training memory; context7 `/vuejs/vue` is the Vue-2 mirror and not authoritative here).
   The docs present reactive props destructure first, then introduce `withDefaults` with: *"In 3.4
   and below, Reactive Props Destructure is not enabled by default. An alternative is to use the
   `withDefaults` compiler macro."* Both are presented as valid, fully-supported options;
   `withDefaults` is **nowhere** marked deprecated, legacy, or discouraged. The finding's label
   "the Vue-3.4-and-below form" reframes a co-equal, documented macro as a defect the docs do not
   assert. No idiomacy violation exists to fix.

2. **The codebase's uniform convention is `const props = defineProps<...>()` + `props.x`, and
   destructure is used nowhere.** All five other prop-taking components use the non-destructured
   form: `JobRow.vue:10`, `LiveLog.vue:16`, `FirstRun.vue:13`, `SuggestionCard.vue:14`,
   `JobsView.vue:49` (`pendingRun?: RunRequest | null`, an optional prop, kept non-destructured).
   `withDefaults` is the natural, codebase-consistent extension of that convention for the one prop
   that needs a default. BatchView's own comment (line 52) anchors the choice explicitly to
   *"matching JobsView's own `pendingRun?:` precedent"* — a deliberate style alignment. Adopting the
   replacement would make BatchView the sole component using reactive props destructure, *reducing*
   internal idiomatic consistency — the inverse of "solve it the way the codebase does." There is
   also a design-preference cost (the destructure form relies on the compiler rewriting `runActive`
   back to `props.runActive` for reactivity — implicit magic over the explicit `props.` access the
   rest of the codebase uses), but the consistency point alone carries the refutal.

Net: this is a stylistic preference dressed as an idiomacy finding. The current code is idiomatic
Vue 3.5 (documented alternative) and is the codebase-consistent form. Changing it neither fixes a
defect nor improves idiomacy; it introduces a lone stylistic outlier.

## Decision guard

Not tracked, no decision conflict. Specs D1-D35 (`docs/superpowers/specs/*.md`), `docs/IDEAS.md`,
and `docs/ROADMAP.md` contain no entry for `withDefaults`, reactive props destructure, or Vue prop
default declaration style. ROADMAP touches `BatchView.vue` only at S13 (settings-persistence errors,
line 290) and B11 (`RECENT_PROFILES_CAP` duplication, line 318) — both unrelated to this construct.

## Evidence

- `src/views/BatchView.vue:53` (cited), `:241` (`props.runActive`), `:52` (precedent comment)
- Siblings: `JobRow.vue:10`, `LiveLog.vue:16`, `FirstRun.vue:13`, `SuggestionCard.vue:14`, `JobsView.vue:49`
- `package.json`: `"vue": "3.5.39"`, `"@vitejs/plugin-vue": "6.0.7"`; `vite.config.ts`: no `propsDestructure` override
- Live docs: vuejs.org Composition-API TypeScript guide, "Props Default Values"
- HEAD: `2f17880a956e05f833a3afdec2c650c176e391e5`
