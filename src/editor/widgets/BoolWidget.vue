<script setup lang="ts">
// D45 field widget: `{ kind: "bool" }` -- a plain two-state checkbox.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";

const props = defineProps<{ spec: EditableFieldOf<"bool">; path?: string }>();
const model = defineModel<boolean>();

const id = useId();
const { diags, severity } = useDiagAnchor(() => props.path);
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <span
      v-if="severity !== null"
      role="img"
      class="diag-marker"
      :class="`diag-marker--${severity}`"
      data-testid="diag-marker"
      :data-diag-path="path"
      :aria-label="$t(`severity-${severity}`)"
      :title="diags.map((d) => $t(d.code, diagnosticFluentParams(d.code, d.params))).join('\n')"
    />
    <input
      :id="id"
      v-model="model"
      type="checkbox"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
    >
  </div>
</template>
