<script setup lang="ts">
// D45 field widget: `{ kind: "directoryPath"; optional }`. A plain path
// textbox -- no file-picker dialog integration here: that needs a Tauri
// IPC round trip (`@tauri-apps/plugin-dialog`), and the mount-harness
// amendment is explicit that these widgets install no IPC mock because
// they are fed their model as a prop; wiring a real picker is Task 13's
// job alongside the rest of the IPC surface. `optional` is not enforced
// here either (zero semantic validation, spec 7): an empty string is a
// legal "unset" value core can diagnose if it isn't.
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
    >
  </div>
</template>
