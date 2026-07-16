<script setup lang="ts">
// D45 field widget: `{ kind: "text"; syntax; multiline }`. `syntax` (plain/
// regex/templateLiteral/templateRegex) is descriptive metadata only -- the
// frontend performs zero semantic validation (spec 7), so every syntax
// renders the same plain control, `multiline` picking `<textarea>` over
// `<input>`.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";

defineProps<{ spec: EditableFieldOf<"text"> }>();
const model = defineModel<string | null>();

const id = useId();
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <textarea
      v-if="spec.widget.multiline"
      :id="id"
      v-model="model"
    />
    <input
      v-else
      :id="id"
      v-model="model"
      type="text"
    >
  </div>
</template>
