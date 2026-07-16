<script setup lang="ts">
// D45 field widget: `{ kind: "propertyMap"; properties; values }` (e.g.
// TrackRule.changes, MatchExpr.exact/substring/regex -- a `Record<string,
// Scalar | string>`). `properties` (matchable/settable) names which
// capability domain the keys come from, but no such domain is exposed to
// the frontend (no ts-rs binding for `capability::matchable_type`/
// `capability::settable` exists); the frontend performs zero semantic
// validation (spec 7) and holds the model as data, so property NAMES are
// free text here -- core diagnoses an unknown/mistyped one. `values`
// ("scalar" vs "string") is likewise not enforced: every value is edited
// as a string (still a legal `Scalar`, since `Scalar = boolean | number |
// string`); a "scalar" field that actually needs a real boolean/number is
// the same class of accepted gap the registry itself already names ("a
// mismatched widget is a visible rendering bug caught the first time the
// panel opens" -- D45).
//
// Row `key`/`value` inputs use `data-testid` (no distinct accessible
// role/name exists for a free-text property name, matching the house
// fallback convention). Add/remove buttons reuse the existing
// `editor-attachment-rule-add`/`-drop` keys ("Add"/"Drop") verbatim
// rather than adding new ones: gui-editor.ftl stays at 43 keys (D45), and
// "Drop" is already this app's own established exclude-this-item
// vocabulary (KEEP_DROP), the same reuse pattern `browse-button` already
// has across FirstRun/SettingsDialog/BatchView.
import { computed } from "vue";
import type { EditableFieldOf } from "./shared";

defineProps<{ spec: EditableFieldOf<"propertyMap"> }>();
const model = defineModel<Record<string, string> | null>();

const rows = computed(() => Object.entries(model.value ?? {}));

function setKey(index: number, key: string) {
  const next = [...rows.value];
  next[index] = [key, next[index][1]];
  model.value = Object.fromEntries(next);
}

function setValue(index: number, value: string) {
  const next = [...rows.value];
  next[index] = [next[index][0], value];
  model.value = Object.fromEntries(next);
}

function addRow() {
  model.value = { ...(model.value ?? {}), "": "" };
}

function removeRow(index: number) {
  const next = [...rows.value];
  next.splice(index, 1);
  model.value = Object.fromEntries(next);
}
</script>

<template>
  <fieldset>
    <legend>{{ $t(spec.labelKey) }}</legend>
    <div
      v-for="([key, value], index) in rows"
      :key="index"
    >
      <input
        data-testid="property-map-key"
        type="text"
        :value="key"
        @input="setKey(index, ($event.target as HTMLInputElement).value)"
      >
      <input
        data-testid="property-map-value"
        type="text"
        :value="value"
        @input="setValue(index, ($event.target as HTMLInputElement).value)"
      >
      <button
        type="button"
        :aria-label="$t('editor-attachment-rule-drop')"
        @click="removeRow(index)"
      >
        {{ $t("editor-attachment-rule-drop") }}
      </button>
    </div>
    <button
      type="button"
      @click="addRow"
    >
      {{ $t("editor-attachment-rule-add") }}
    </button>
  </fieldset>
</template>
