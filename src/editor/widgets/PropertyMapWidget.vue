<script setup lang="ts">
// D45 field widget: `{ kind: "propertyMap"; properties; values }` (e.g.
// TrackRule.changes, MatchExpr.exact/substring/regex -- a `Record<string,
// Scalar>`). `properties` (matchable/settable) names which capability
// domain the keys come from; property NAMES stay free text here (no
// closed-domain enforcement on the key itself -- core diagnoses an
// unknown/mistyped one), consistent with spec 7's zero-semantic-validation
// rule.
//
// VALUE cells ARE typed (owner Ruling 2, amended 2026-07-16, house
// principle `gui-typed-scalar-needs-typed-input`): whenever
// `spec.widget.values === "scalar"`, a row's value cell renders by the
// property's declared scalar type, looked up in Task 12a's committed
// `SETTABLE_TYPES` (`properties: "settable"`) or `MATCHABLE_TYPES`
// (`properties: "matchable"`) -- through one switch over `PropScalarType`
// in `cellKindFor`, exhaustive via the house `const _exhaustive: never =
// scalarType` arm (`FieldWidgetDispatcher.vue`'s own shape): `boolean` ->
// checkbox (a real `true`/`false`), `integer` -> `<input type="number">`,
// `float` -> `<input type="number" step="any">` (the one new input
// variant, decimal-accepting, enumerated inside this switch, not a new
// `FieldWidget` kind), `string` -> text. A property in NEITHER map
// (unknown, or user-mistyped) falls back to the text cell before the
// switch is ever reached -- core catches the unknown property anyway.
// `values === "string"` (`matchExpr.substring`/`regex`) stays a text cell
// unconditionally: a substring/regex pattern is `String` BY DEFINITION
// (never a Boolean/number), so it is gated out before any type lookup --
// a closed boundary on the `values` facet, not a fork. This is why `model`
// widens from `Record<string, string>` to `Record<string, Scalar>` (from
// `../../bindings/profile`): a Boolean settable/matchable must reload as a
// real `true`, not the string `"true"`.
//
// Row `key` inputs use `data-testid` (no distinct accessible role/name
// exists for a free-text property name, matching the house fallback
// convention); the value cell keeps the SAME `data-testid` across every
// typed variant so a row stays locatable regardless of its resolved type,
// while its accessible role (checkbox/spinbutton/textbox) carries the
// type-specific test assertions. Add/remove use the two generic
// `editor-action-add`/`-remove` keys (owner Ruling 1, amended 2026-07-16;
// catalog budget 45) -- not `editor-attachment-rule-add`/`-drop`, which
// now caption only the AttachmentRule fields they are the registry labels
// for (`registries.ts:185-189`).
import { computed } from "vue";
import type { EditableFieldOf } from "./shared";
import { MATCHABLE_TYPES, SETTABLE_TYPES } from "../../bindings/settables";
import type { PropScalarType } from "../../bindings/settables";
import type { Scalar } from "../../bindings/profile";

const props = defineProps<{ spec: EditableFieldOf<"propertyMap"> }>();
const model = defineModel<Record<string, Scalar> | null>();

const rows = computed(() => Object.entries(model.value ?? {}));

/** Which value-cell control a row's property name resolves to. Not one of
 *  the 10 `FieldWidget` kinds -- this is the typed switch INSIDE the
 *  `propertyMap` cell, per the binding point above. */
type ValueCellKind = "checkbox" | "integer" | "float" | "text";

function cellKindFor(key: string): ValueCellKind {
  if (props.spec.widget.values !== "scalar") {
    return "text";
  }
  const table =
    props.spec.widget.properties === "settable"
      ? (SETTABLE_TYPES as Record<string, PropScalarType>)
      : (MATCHABLE_TYPES as Record<string, PropScalarType>);
  const scalarType = table[key];
  if (scalarType === undefined) {
    return "text";
  }
  switch (scalarType) {
    case "boolean":
      return "checkbox";
    case "integer":
      return "integer";
    case "float":
      return "float";
    case "string":
      return "text";
    default: {
      const _exhaustive: never = scalarType;
      throw new Error(`unhandled PropScalarType: ${String(_exhaustive)}`);
    }
  }
}

function setKey(index: number, key: string) {
  const next = [...rows.value];
  next[index] = [key, next[index][1]];
  model.value = Object.fromEntries(next);
}

function setValue(index: number, value: Scalar) {
  const next = [...rows.value];
  next[index] = [next[index][0], value];
  model.value = Object.fromEntries(next);
}

function onCheckboxInput(index: number, event: Event) {
  setValue(index, (event.target as HTMLInputElement).checked);
}

function onNumberInput(index: number, event: Event) {
  setValue(index, Number((event.target as HTMLInputElement).value));
}

function onTextInput(index: number, event: Event) {
  setValue(index, (event.target as HTMLInputElement).value);
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
        v-if="cellKindFor(key) === 'checkbox'"
        data-testid="property-map-value"
        type="checkbox"
        :checked="value === true"
        @change="onCheckboxInput(index, $event)"
      >
      <input
        v-else-if="cellKindFor(key) === 'integer'"
        data-testid="property-map-value"
        type="number"
        :value="value"
        @input="onNumberInput(index, $event)"
      >
      <input
        v-else-if="cellKindFor(key) === 'float'"
        data-testid="property-map-value"
        type="number"
        step="any"
        :value="value"
        @input="onNumberInput(index, $event)"
      >
      <input
        v-else
        data-testid="property-map-value"
        type="text"
        :value="value"
        @input="onTextInput(index, $event)"
      >
      <button
        type="button"
        :aria-label="$t('editor-action-remove')"
        @click="removeRow(index)"
      >
        {{ $t("editor-action-remove") }}
      </button>
    </div>
    <button
      type="button"
      @click="addRow"
    >
      {{ $t("editor-action-add") }}
    </button>
  </fieldset>
</template>
