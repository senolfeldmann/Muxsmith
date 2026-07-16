<script setup lang="ts">
// D45 field widget: `{ kind: "section"; of; optional }` -- a nested
// struct's own fields, driven by its registry (`of`), recursing through
// `FieldWidgetDispatcher`. `optional` is not a toggle here: an absent
// section (`modelValue` undefined) still renders its sub-fields against an
// empty object, so editing any one of them implicitly creates the section
// (no "create this section" button, no chrome text needed -- consistent
// with `zero semantic validation, holds the model as data`). `FixedField`
// entries (`"fixed" in fieldSpec`; currently only `Profile.profile_version`)
// render nothing: `FixedField.why` is a source comment, never user prose
// (`fieldSpec.ts`'s own doc on the type).
import { computed } from "vue";
import type { EditableField } from "../fieldSpec";
import type { EditableFieldOf } from "./shared";
import { registryByName } from "./shared";
import FieldWidgetDispatcher from "./FieldWidgetDispatcher.vue";

const props = defineProps<{ spec: EditableFieldOf<"section"> }>();
const model = defineModel<Record<string, unknown> | null>();

const registry = computed(() => registryByName[props.spec.widget.of]);

const fields = computed(() =>
  Object.entries(registry.value).filter(
    (entry): entry is [string, EditableField] => !("fixed" in entry[1]),
  ),
);

function fieldValue(key: string): unknown {
  return (model.value ?? {})[key];
}

function setFieldValue(key: string, value: unknown) {
  model.value = { ...(model.value ?? {}), [key]: value };
}
</script>

<template>
  <fieldset>
    <legend>{{ $t(spec.labelKey) }}</legend>
    <FieldWidgetDispatcher
      v-for="[key, fieldSpec] in fields"
      :key="key"
      :spec="fieldSpec"
      :model-value="fieldValue(key)"
      @update:model-value="setFieldValue(key, $event)"
    />
  </fieldset>
</template>
