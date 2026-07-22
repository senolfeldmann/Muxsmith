<script setup lang="ts">
// D45 field widget: `{ kind: "optionalFlag" }`. NOT a native checkbox
// v-model: the off-state is absence (`undefined`), not `false`
// (validate.rs:466-472 rejects `Some(false)`; model.rs's own doc says "the
// only valid value is true" -- a tri-state control would offer a value the
// validator rejects), so `:checked`/`@change` are wired by hand instead of
// letting `v-model` write a bare `false`.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";

defineProps<{ spec: EditableFieldOf<"optionalFlag"> }>();
const model = defineModel<true | undefined>();

const id = useId();

function onChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  model.value = checked ? true : undefined;
}
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <input
      :id="id"
      type="checkbox"
      :checked="model === true"
      :title="$ta(spec.labelKey).tooltip"
      @change="onChange"
    >
  </div>
</template>
