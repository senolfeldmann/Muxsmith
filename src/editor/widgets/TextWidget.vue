<script setup lang="ts">
// D45 field widget: `{ kind: "text"; syntax; multiline }`. `syntax` (plain/
// regex/templateLiteral/templateRegex) is descriptive metadata only -- the
// frontend performs zero semantic validation (spec 7), so every syntax
// renders the same plain control, `multiline` picking `<textarea>` over
// `<input>`.
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";

const props = defineProps<{ spec: EditableFieldOf<"text">; path?: string }>();
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
    <textarea
      v-if="spec.widget.multiline"
      :id="id"
      v-model="model"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
    />
    <input
      v-else
      :id="id"
      v-model="model"
      type="text"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
    >
  </div>
</template>
