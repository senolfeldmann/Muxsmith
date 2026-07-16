<script setup lang="ts">
import type { Diagnostic } from "../ipc";
import { diagnosticFluentParams } from "../diagnosticFluentParams";

// Renders one flat list of diagnostics (spec 5.2, 8.4): severity icon
// (decorative, `aria-hidden`) plus the localized severity word and the
// `code`/`params` rendered through `diagnostics.ftl` -- the same catalog
// the CLI's `Renderer` reads (spec 7), so a diagnostic reads identically
// in both surfaces. Reused for the batch/config-level diagnostics
// (BatchView) and, once per file, for `ReportFile.diagnostics`
// (ResolutionTable): same shape, same rendering, no per-caller variant.
//
// Deliberately no "no diagnostics" empty state here: an empty list means
// there is nothing to say at this spot, and BatchView's own `role="status"`
// summary line already covers the batch-wide zero-diagnostics case: a
// second, repeated-per-instance empty message would be noise.
//
// Task 14 (D43, D49) added one-click apply: `SuggestionCard.vue` renders
// the apply control and emits it, `BatchView.vue` (their common parent --
// both are siblings there, not nested) handles the emit and owns the
// round trip. This component is not in that chain and left untouched by
// design: `core-109-two-required-no-fix`'s no-fix/partition diagnostic
// (`suggestion-partition`) renders here exactly as any other diagnostic,
// with no apply control, because no `Suggestion` exists for it to apply
// -- not a gap this panel needs to close. Giving a `Diagnostic` an apply
// affordance here (e.g. cross-rendering the `Suggestion` its
// `suggestion_ref` names) would need a second, `Suggestion`-typed prop
// threaded through every call site, reopening the "same shape, same
// rendering, no per-caller variant" genericity this component is built
// on for a feature the sibling/parent pair above already provides.
defineProps<{ diagnostics: Diagnostic[] }>();
</script>

<template>
  <ul v-if="diagnostics.length">
    <li
      v-for="(d, i) in diagnostics"
      :key="i"
    >
      <span
        class="severity-dot"
        :class="`severity-dot--${d.severity}`"
        aria-hidden="true"
      />
      <span>{{
        $t("batch-diagnostic-line", {
          severity: $t(`severity-${d.severity}`),
          message: $t(d.code, diagnosticFluentParams(d.code, d.params)),
        })
      }}</span>
    </li>
  </ul>
</template>

<style scoped>
.severity-dot {
  display: inline-block;
  width: 0.6em;
  height: 0.6em;
  border-radius: 50%;
  margin-inline-end: 0.35em;
}
.severity-dot--error {
  background: #c0392b;
}
.severity-dot--warning {
  background: #d68910;
}
.severity-dot--info {
  background: #2471a3;
}
</style>
