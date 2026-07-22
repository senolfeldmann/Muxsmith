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
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";
import FieldWidgetDispatcher from "./FieldWidgetDispatcher.vue";

// `suppressSelfAnchor` (D57): a section rendered through the dispatcher
// (locator root, matchExpr root, attachment-rule item) anchors a marker at
// its own path -- the default (absent boolean prop is `false` in Vue).
// `KeywordOrBlockWidget` renders its nested block section at the SAME path
// it owns (core's grammar adds no segment for the block), so it passes
// `suppress-self-anchor` to keep the keyword-or-block widget the sole
// marker for that shared path; the section still uses its path to build
// child paths. Negative sense deliberately: Vue coerces an absent
// Boolean-typed prop to `false`, so the common case (anchor) must be the
// `false` state.
const props = defineProps<{
  spec: EditableFieldOf<"section">;
  path?: string;
  suppressSelfAnchor?: boolean;
}>();
const model = defineModel<Record<string, unknown> | null>();

const registry = computed(() => registryByName[props.spec.widget.of]);

const fields = computed(() =>
  Object.entries(registry.value).filter(
    (entry): entry is [string, EditableField] => !("fixed" in entry[1]),
  ),
);

const { diags, severity } = useDiagAnchor(() =>
  props.suppressSelfAnchor ? undefined : props.path,
);

function childPath(key: string): string | undefined {
  return props.path === undefined ? undefined : `${props.path}.${key}`;
}

function fieldValue(key: string): unknown {
  return (model.value ?? {})[key];
}

function setFieldValue(key: string, value: unknown) {
  model.value = { ...(model.value ?? {}), [key]: value };
}
</script>

<template>
  <fieldset :title="$ta(spec.labelKey).tooltip">
    <legend>
      {{ $t(spec.labelKey) }}
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
    </legend>
    <FieldWidgetDispatcher
      v-for="[key, fieldSpec] in fields"
      :key="key"
      :spec="fieldSpec"
      :path="childPath(key)"
      :model-value="fieldValue(key)"
      @update:model-value="setFieldValue(key, $event)"
    />
  </fieldset>
</template>
