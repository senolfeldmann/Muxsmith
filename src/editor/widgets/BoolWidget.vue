<script setup lang="ts">
// D45 field widget: `{ kind: "bool" }` -- a plain two-state checkbox.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";

defineProps<{ spec: EditableFieldOf<"bool"> }>();
const model = defineModel<boolean>();

const id = useId();
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <input
      :id="id"
      v-model="model"
      type="checkbox"
      :title="$ta(spec.labelKey).tooltip"
    >
  </div>
</template>
