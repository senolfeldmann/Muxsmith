<script setup lang="ts">
// D45 field widget: `{ kind: "directoryPath"; optional }`. A plain path
// textbox -- no file-picker dialog: a picker is out of scope for Plan 6;
// the directory field is text-entry only (D45 widgets are prop-fed,
// zero-IPC). `optional` is not enforced here either (zero semantic
// validation, spec 7): an empty string is a legal "unset" value core can
// diagnose if it isn't.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";

const props = defineProps<{ spec: EditableFieldOf<"directoryPath">; path?: string }>();
const model = defineModel<string | null>();

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
      type="text"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
    >
  </div>
</template>
