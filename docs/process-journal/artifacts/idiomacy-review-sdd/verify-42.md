# Verify-42: pre-3.5 template-ref pattern -> useTemplateRef (F5, idiom)

**Verdict: CONFIRMED**

Finding: four sites use the pre-3.5 name-matching template-ref pattern
(`ref<InstanceType<typeof C> | null>(null)` / `ref<HTMLElement | null>(null)`); Vue 3.5
replaces it with `useTemplateRef`, and the manual annotations disappear via
template-based inference.

## (a) Cited code says what the finding claims — YES

All four declarations exist verbatim and each is a genuine template ref (static string
key, bound exactly once in the template):

| Site | Declaration | Template binding | Kind |
|------|-------------|------------------|------|
| `src/App.vue:20` | `ref<InstanceType<typeof SettingsDialog> \| null>(null)` | `:117 <SettingsDialog ref="settingsDialog" />` | component |
| `src/components/SettingsDialog.vue:13` | `ref<HTMLDialogElement \| null>(null)` | `:90 <dialog ref="dialogEl">` | element |
| `src/components/LiveLog.vue:40` | `ref<HTMLElement \| null>(null)` | `:87 <div ref="logEl" role="log">` | element |
| `src/views/JobsView.vue:64` | `ref<InstanceType<typeof RunHistory> \| null>(null)` | `:338 <RunHistory ref="runHistoryRef" />` | component |

`useTemplateRef` is not yet used anywhere in `src/` (grep clean).

Non-material imprecision: the finding's opening generic form files `dialogEl` under
`ref<HTMLElement | null>`, but the actual annotation is the tighter `HTMLDialogElement`.
The finding lists `dialogEl` explicitly and the replacement is unaffected.

## (b) Replacement is current idiom for the pinned toolchain — YES

Toolchain (package.json): `vue 3.5.39`, `vue-tsc 3.3.7`, `typescript 6.0.3`,
`@vitejs/plugin-vue 6.0.7`.

Verified against current official docs (context7 `/vuejs/docs`,
`guide/typescript/composition-api.md`), not training memory:

- `useTemplateRef()` is the Vue 3.5+ recommended way; the ref-name-matching `ref()` form
  is the "before 3.5" pattern. `useTemplateRef` shipped in 3.5.0, so 3.5.39 has it.
- Automatic type inference for **static** refs works "With Vue 3.5 and
  `@vue/language-tools` 2.1 (powering both the IDE language service and `vue-tsc`)".
  `vue-tsc 3.3.7` is language-tools 3.x, far past the 2.1 floor, so it supports the
  inference. The finding's toolchain claim holds.
- All four are static single refs — exactly the case the docs say auto-inference covers.
  The explicit-generic escape hatch (`useTemplateRef<T>('x')`) the docs show is only for
  the auto-inference-*fails* cases (dynamic `<component :is>`, generic components); none
  apply here.

Exposed-member and behavioral soundness (the rewrite must not break the call sites):

- `SettingsDialog` does `defineExpose({ open })`; App calls `settingsDialog?.open()`.
  `RunHistory` does `defineExpose({ refresh })`; JobsView calls
  `runHistoryRef.value?.refresh()`. `useTemplateRef` auto-inference resolves to the
  component public instance type, which includes `defineExpose`'d members, so both
  call sites keep type-checking.
- Element sites infer `HTMLDialogElement` / `HTMLDivElement` (the latter a subtype of the
  current `HTMLElement`; `scrollHeight`/`scrollTop`/`clientHeight` remain available).
- `useTemplateRef` returns a readonly shallow ref; every site only reads the ref
  (`?.` calls, template unwrap), none reassign it. No behavioral change. The manual
  annotations become redundant, exactly as the finding states.

## (c) No load-bearing difference between the four sites

They split into two component refs and two element refs, but the pattern and the
replacement are uniform; the finding lists both forms explicitly, so the split does not
undercut the grouping.

## (d) N/A — tag is `idiom`, not `yagni`; concrete construct + concrete replacement both named.

## Decision guard — no conflict, not tracked

Grepped `docs/superpowers/specs/*.md` (D-memos), `docs/IDEAS.md`, `docs/ROADMAP.md`:

- No spec/IDEAS hit for `useTemplateRef` / `template ref` / `InstanceType`. D27 pins the
  frontend stack (Vue 3 + native `<dialog>` opened via a template ref) but records no
  decision to keep the pre-3.5 `ref()` declaration style; `useTemplateRef` is vue core, no
  new dependency, so no conflict.
- ROADMAP "Cosmetic cleanup, one pass (sweep group K)" (`:260`) enumerates specific
  unrelated items (dead `at` param, `*[empty-field]` mislabel, TracksCfg placement, stale
  module doc, plan-1 archive remnants, eager chapters/attachments resolve); template refs
  are not among them. Deferral/restraint entries elsewhere are product-feature scope
  (IDEAS) or unrelated tooling (coverage, test-hygiene). Nothing tracks this GUI idiom.

## Conclusion

Code matches; the replacement is the documented current idiom for the exact pinned
toolchain; it is a mechanical, type-safe drop-in at all four static-ref sites (exposed
members and element interfaces preserved); no recorded decision covers or contradicts it.
CONFIRMED.
