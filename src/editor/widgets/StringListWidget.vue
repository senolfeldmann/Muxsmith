<script setup lang="ts">
// D45 field widget: `{ kind: "stringList" }` (e.g. Input.extensions,
// Locator.extensions -- a small set of short tokens). A single
// comma-separated textbox rather than per-item add/remove rows: no
// generic "add"/"remove" chrome text exists in gui-editor.ftl's 43 keys
// (D45 forbids growing it), and a flat scalar list is small and short
// enough that comma-separated editing is a common, idiomatic pattern for
// exactly this shape (tags/extensions-style fields). Parsing is
// structural (split/trim/filter empty), not semantic interpretation of
// the values -- consistent with spec 7's zero-semantic-validation rule.
import { computed, useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";

const props = defineProps<{ spec: EditableFieldOf<"stringList">; path?: string }>();
const model = defineModel<string[] | null>();

const id = useId();
const { diags, severity } = useDiagAnchor(() => props.path);

const text = computed<string>({
  get: () => (model.value ?? []).join(", "),
  set: (value: string) => {
    model.value = value
      .split(",")
      .map((item) => item.trim())
      .filter((item) => item.length > 0);
  },
});
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
      v-model="text"
      type="text"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
    >
  </div>
</template>
