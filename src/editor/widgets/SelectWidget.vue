<script setup lang="ts">
// D45 field widget: `{ kind: "select"; options }`. `options` are
// profile-format tokens (e.g. COLLISION_POLICIES, KEEP_DROP) rendered raw,
// never through `$t` -- they are config vocabulary, not UI prose, and
// gui-editor.ftl stays at its 43 label keys (D45's own constraint).
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";

defineProps<{ spec: EditableFieldOf<"select"> }>();
const model = defineModel<string>();

const id = useId();
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <select
      :id="id"
      v-model="model"
      :title="$ta(spec.labelKey).tooltip"
    >
      <option
        v-for="option in spec.widget.options"
        :key="option"
        :value="option"
      >
        {{ option }}
      </option>
    </select>
  </div>
</template>
