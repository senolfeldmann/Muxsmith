<script setup lang="ts">
// D45 field widget: `{ kind: "directoryPath"; optional }`. A plain path
// textbox -- no file-picker dialog: a picker is out of scope for Plan 6;
// the directory field is text-entry only (D45 widgets are prop-fed,
// zero-IPC). `optional` is not enforced here either (zero semantic
// validation, spec 7): an empty string is a legal "unset" value core can
// diagnose if it isn't.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";

defineProps<{ spec: EditableFieldOf<"directoryPath"> }>();
const model = defineModel<string | null>();

const id = useId();
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <input
      :id="id"
      v-model="model"
      type="text"
      :title="$ta(spec.labelKey).tooltip"
    >
  </div>
</template>
