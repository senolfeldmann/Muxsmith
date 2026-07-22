<script setup lang="ts">
// D45 field widget: `{ kind: "select"; options }`. `options` are
// profile-format tokens (e.g. COLLISION_POLICIES, KEEP_DROP) rendered raw,
// never through `$t` -- they are config vocabulary, not UI prose, and
// gui-editor.ftl stays at its 43 label keys (D45's own constraint).
import { useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";

const props = defineProps<{ spec: EditableFieldOf<"select">; path?: string }>();
const model = defineModel<string>();

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
    <select
      :id="id"
      v-model="model"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
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
