<script setup lang="ts">
// D45 field widget: `{ kind: "optionalFlag" }`. NOT a native checkbox
// v-model: the off-state is absence (`undefined`), not `false`
// (`validate_locator` in profile/validate.rs rejects `Some(false)`;
// model.rs's own doc says "the only valid value is true" -- a tri-state
// control would offer a value the validator rejects), so `:checked`/
// `@change` are wired by hand instead of letting `v-model` write a bare
// `false`.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";

const props = defineProps<{ spec: EditableFieldOf<"optionalFlag">; path?: string }>();
const model = defineModel<true | undefined>();

const id = useId();
const { diags, severity } = useDiagAnchor(() => props.path);

function onChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  model.value = checked ? true : undefined;
}
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
      type="checkbox"
      :checked="model === true"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
      @change="onChange"
    >
  </div>
</template>
