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
// Row `key` inputs use `data-testid` for TEST LOCATION (no distinct
// accessible role/name exists for a free-text property name to assert
// against, matching the house fallback convention for locators); the value
// cell keeps the SAME `data-testid` across every typed variant so a row
// stays locatable regardless of its resolved type, while its accessible
// role (checkbox/spinbutton/textbox) carries the type-specific test
// assertions. Accessible NAMING (a different concern from test location)
// is wired below via `useId`/`aria-labelledby`, the same primitive
// `TextWidget.vue`'s `useId`+`<label :for>` pattern is built on, adapted
// for a one-legend-many-rows widget: a single `<label for>` can't serve
// every row, so both inputs reference IDs by `aria-labelledby` instead of
// owning a `<label>` each. Add/remove use the two generic
// `editor-action-add`/`-remove` keys (owner Ruling 1, amended 2026-07-16;
// the closed editor catalog budget, `editor-generic-action-keys`) -- not
// `editor-attachment-rule-add`/`-drop`, which
// now caption only the AttachmentRule fields they are the registry labels
// for (`attachmentRuleFields` in `registries.ts`).
import { computed, inject, useId } from "vue";
import type { EditableFieldOf } from "./shared";
import { editorDiagnosticsByPath, worstSeverity } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";
import {
  CODEC_KIND_NAMES,
  MATCHABLE_TYPES,
  SETTABLE_TYPES,
  TYPE_VALUES,
} from "../../bindings/settables";
import type { PropScalarType } from "../../bindings/settables";
import type { Scalar } from "../../bindings/profile";
import type { Diagnostic } from "../../ipc";

const props = defineProps<{ spec: EditableFieldOf<"propertyMap">; path?: string }>();
const model = defineModel<Record<string, Scalar> | null>();

const rows = computed(() => Object.entries(model.value ?? {}));

// Per-row diagnostic anchors (D57): a `changes`/`exact` row anchors at
// `{widget path}.{rowKey}` (e.g. `tracks[0].changes.language`). The map is
// injected ONCE here and looked up per row, since the rows are inline (not
// child components) -- `useDiagAnchor`'s getter form is for a per-instance
// child, which a row is not.
const byPath = inject(editorDiagnosticsByPath, undefined);
const rowAnchors = computed(() =>
  rows.value.map(([key, value]) => {
    const rowPath = props.path === undefined ? undefined : `${props.path}.${key}`;
    const diags: Diagnostic[] =
      rowPath === undefined || byPath === undefined ? [] : (byPath.value.get(rowPath) ?? []);
    return { key, value, path: rowPath, diags, severity: worstSeverity(diags) };
  }),
);

// Wave item 9 (whole-branch a11y finding, `PropertyMapWidget.vue`'s
// key/value inputs had no accessible name at all -- axe `label`/critical):
// zero new catalog keys, zero new strings. `legendId` labels the KEY input
// by the widget's own EXISTING legend text (`$t(spec.labelKey)`, e.g.
// "Changes"/"Match expression") -- the same text sighted users already
// read as this widget's heading, now also wired as the accessible name
// source via `aria-labelledby` instead of a 1:1 `<label for>` (the
// fieldset can only be legended once, but `aria-labelledby` lets every
// row's input point at that one id). The VALUE input's name additionally
// references the row's OWN key input id: the accessible-name computation
// (WAI-ARIA `aria-labelledby`) includes a referenced textbox's live VALUE,
// so a row with key "forced" reads as "Changes forced" for its value
// control -- distinguishing rows by data the user already typed, not by
// any new copy. `useId()` (not a hand-rolled prefix) matches the
// established per-instance-unique-id primitive `TextWidget.vue` already
// uses; `-${index}` extends it per row within this one widget instance.
const legendId = useId();
const keyIdBase = useId();

function keyInputId(index: number): string {
  return `${keyIdBase}-${index}`;
}

// D58 (`gui-closed-domain-dropdowns`): the two curated matchable domains
// `type` (4 values) and `codec_kind` (17 aliases) render as a `<select>` in
// the EXACT-match value cells, resolved BEFORE the scalar-type switch. The
// domain arrays are the emitter's committed output (`TYPE_VALUES`/
// `CODEC_KIND_NAMES`, never hand-written in TS), keyed by the property name.
const DOMAINS: Record<string, readonly string[]> = {
  type: TYPE_VALUES,
  codec_kind: CODEC_KIND_NAMES,
};

function domainFor(key: string): readonly string[] {
  return DOMAINS[key] ?? [];
}

/** Which value-cell control a row's property name resolves to. Not one of
 *  the 10 `FieldWidget` kinds -- this is the typed switch INSIDE the
 *  `propertyMap` cell, per the binding point above. `select` (D58) is
 *  resolved ahead of the scalar switch. */
type ValueCellKind = "select" | "checkbox" | "integer" | "float" | "text";

function cellKindFor(key: string, value: Scalar): ValueCellKind {
  // D58 dropdown: only in the exact-match value cells (matchable+scalar),
  // only inside a track context (the D57 `path` -- the attachment maps share
  // `matchExprFields` but their property universe has no `type`, ground-truth
  // flaw), only for the byte-exact keys `type`/`codec_kind` (a `raw:type` key
  // fails this and keeps its free-text cell, preserving the `raw:` bypass),
  // and only when the value is empty (a fresh row) or already a domain member
  // -- an out-of-domain value stays a text input so the dropdown never eats
  // data it cannot represent.
  if (
    props.spec.widget.properties === "matchable" &&
    props.spec.widget.values === "scalar" &&
    props.path !== undefined &&
    props.path.startsWith("tracks[") &&
    (key === "type" || key === "codec_kind") &&
    (value === "" || (typeof value === "string" && domainFor(key).includes(value)))
  ) {
    return "select";
  }
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

// `Object.fromEntries` collapses a duplicate key to its LAST occurrence
// (last-write-wins): if `setKey` renames a row's key to match another
// existing row, or two rows already share a key from a loaded profile, the
// earlier row's value silently disappears here rather than erroring. Left
// uncaught deliberately (spec 7, zero frontend semantic validation) --
// core's own validator diagnoses a duplicate/missing property against the
// saved YAML, the same way it catches an unknown property name.
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

function onSelectInput(index: number, event: Event) {
  setValue(index, (event.target as HTMLSelectElement).value);
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
  <fieldset :title="$ta(spec.labelKey).tooltip">
    <legend :id="legendId">
      {{ $t(spec.labelKey) }}
    </legend>
    <div
      v-for="(row, index) in rowAnchors"
      :key="index"
    >
      <input
        :id="keyInputId(index)"
        data-testid="property-map-key"
        type="text"
        :aria-labelledby="legendId"
        :value="row.key"
        @input="setKey(index, ($event.target as HTMLInputElement).value)"
      >
      <span
        v-if="row.severity !== null"
        role="img"
        class="diag-marker"
        :class="`diag-marker--${row.severity}`"
        data-testid="diag-marker"
        :data-diag-path="row.path"
        :aria-label="$t(`severity-${row.severity}`)"
        :title="row.diags.map((d) => $t(d.code, diagnosticFluentParams(d.code, d.params))).join('\n')"
      />
      <select
        v-if="cellKindFor(row.key, row.value) === 'select'"
        data-testid="property-map-value"
        :aria-labelledby="`${legendId} ${keyInputId(index)}`"
        :class="row.severity !== null ? `diag-anchored--${row.severity}` : undefined"
        :aria-invalid="row.severity === 'error' ? 'true' : undefined"
        :value="row.value"
        @change="onSelectInput(index, $event)"
      >
        <option
          v-if="row.value === ''"
          value=""
        />
        <option
          v-for="opt in domainFor(row.key)"
          :key="opt"
          :value="opt"
        >
          {{ opt }}
        </option>
      </select>
      <input
        v-else-if="cellKindFor(row.key, row.value) === 'checkbox'"
        data-testid="property-map-value"
        type="checkbox"
        :aria-labelledby="`${legendId} ${keyInputId(index)}`"
        :class="row.severity !== null ? `diag-anchored--${row.severity}` : undefined"
        :aria-invalid="row.severity === 'error' ? 'true' : undefined"
        :checked="row.value === true"
        @change="onCheckboxInput(index, $event)"
      >
      <input
        v-else-if="cellKindFor(row.key, row.value) === 'integer'"
        data-testid="property-map-value"
        type="number"
        :aria-labelledby="`${legendId} ${keyInputId(index)}`"
        :class="row.severity !== null ? `diag-anchored--${row.severity}` : undefined"
        :aria-invalid="row.severity === 'error' ? 'true' : undefined"
        :value="row.value"
        @input="onNumberInput(index, $event)"
      >
      <input
        v-else-if="cellKindFor(row.key, row.value) === 'float'"
        data-testid="property-map-value"
        type="number"
        step="any"
        :aria-labelledby="`${legendId} ${keyInputId(index)}`"
        :class="row.severity !== null ? `diag-anchored--${row.severity}` : undefined"
        :aria-invalid="row.severity === 'error' ? 'true' : undefined"
        :value="row.value"
        @input="onNumberInput(index, $event)"
      >
      <input
        v-else
        data-testid="property-map-value"
        type="text"
        :aria-labelledby="`${legendId} ${keyInputId(index)}`"
        :class="row.severity !== null ? `diag-anchored--${row.severity}` : undefined"
        :aria-invalid="row.severity === 'error' ? 'true' : undefined"
        :value="row.value"
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
