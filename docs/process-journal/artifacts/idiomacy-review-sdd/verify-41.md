# Verify-41: JobsView hand-rolled v-model protocol (idiom, slice F5)

**Verdict: CONFIRMED**

## Finding under test

`src/views/JobsView.vue:210` - local `runActive` ref + watch-forwarder emitting
`update:runActive`, wired in `App.vue:114` as `@update:run-active="jobsRunActive = $event"`;
claimed to be a half-implemented v-model that `defineModel` (Vue 3.4+) replaces.

## (a) Cited code matches the claim

- `JobsView.vue:58`: `const runActive = ref(false)` - child-owned, mutated at
  lines 120, 182, 196.
- `JobsView.vue:49-50`: `defineProps<{ pendingRun?: RunRequest | null }>()` -
  **no `runActive` prop**; `defineEmits` declares `"update:runActive": [active: boolean]`.
- `JobsView.vue:205-210`: comment block + `watch(runActive, (value) => emit("update:runActive", value), { immediate: true })`.
- `App.vue:34`: `const jobsRunActive = ref(false)`; `App.vue:114`:
  `@update:run-active="jobsRunActive = $event"` on `<JobsView>`.
- `App.vue:107`: `:run-active="jobsRunActive"` is on **BatchView** (plain prop on a
  different component), untouched by the replacement.

The `update:x` event name without a matching `x` prop is exactly the half of the
v-model contract the finding describes. Accurate.

## (b) Replacement is current idiom for the pinned toolchain

- `package.json` pins `"vue": "3.5.39"` (>= 3.4).
- Official Vue docs (context7, `/vuejs/docs`, guide/components/v-model.md and
  api/sfc-script-setup.md): "In Vue 3.4+, the defineModel() macro is the
  recommended implementation method" for component v-model. Named form
  `defineModel('runActive', { default: false })` declares the `runActive` prop and
  emits `update:runActive` on every mutation of the returned ref - which subsumes
  the watcher's own rationale ("every mutation site is covered without having to
  remember to emit at each one") by construction, covering lines 120/182/196.
- TS type-argument form `defineModel<boolean>("runActive", { default: false })` is
  documented and valid.
- Parent side `v-model:run-active="jobsRunActive"` is the documented named-model
  binding. App binds a real `ref(false)`, so the parent-not-bound default-desync
  caveat does not apply; the `{ immediate: true }` initial emit of `false` is a
  no-op today (parent already `false`), so semantics are preserved exactly.

## (c) Duplication

No duplication claim in the finding. N/A.

## (d) YAGNI shape

Tag is `idiom`, not `yagni`; concrete construct and concrete replacement are named
regardless. N/A.

## Decision guard

- `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md` **D23**
  prescribes the *behavior* ("the UI additionally disables Run while active") and
  the shell topology, not the child->parent forwarding *mechanism*. The
  replacement preserves the behavior and App-as-broker topology unchanged.
- `docs/ROADMAP.md:40` (S12) tracks a *different* D23-related item: reset gated on
  `runActive` instead of "reset after resolve Ok" - reset-gating semantics, not the
  emit-vs-defineModel mechanism. Untouched by this refactor.
- No hits for `v-model` / `defineModel` / this construct anywhere in
  `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, or `docs/ROADMAP.md` (group K,
  test-hygiene, deferred entries included). Not tracked, no conflict.

## Conclusion

All four refutation legs fail; no recorded decision covers the mechanism.
The finding stands: complete the v-model contract with `defineModel<boolean>("runActive", { default: false })`
in JobsView (dropping the ref, the watch block, and the emits entry) and
`v-model:run-active="jobsRunActive"` in App.vue:114.
