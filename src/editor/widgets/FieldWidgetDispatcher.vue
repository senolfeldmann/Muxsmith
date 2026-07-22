<script setup lang="ts">
// D45: the widget dispatcher -- given one `EditableField`, resolves and
// renders the component matching its `widget.kind`. `widgetComponentFor`'s
// switch carries an explicit `const _exhaustive: never = kind` default
// arm (D45's own deliberate, minimal improvement over the existing house
// shape at `src/jobRowState.ts:44-55`, which relies on plain control-flow
// exhaustiveness with no such arm): a `FieldWidget` variant added without
// a case here fails the *build* with TS2322 naming the unhandled kind,
// not just an unreachable `default` at runtime.
//
// The registry forces label+widget EXISTENCE per field, not
// type-suitability (D45); this dispatcher mirrors that by passing `spec`
// straight through `<component :is>` rather than re-deriving a narrower
// per-kind prop type at the call site -- a mismatched widget-to-field
// pairing is a visible rendering bug caught the first time the panel
// opens, the same accepted gap the registry itself already names.
import type { Component } from "vue";
import type { EditableField, FieldWidget } from "../fieldSpec";
import TextWidget from "./TextWidget.vue";
import BoolWidget from "./BoolWidget.vue";
import OptionalFlagWidget from "./OptionalFlagWidget.vue";
import SelectWidget from "./SelectWidget.vue";
import KeywordOrBlockWidget from "./KeywordOrBlockWidget.vue";
import DirectoryPathWidget from "./DirectoryPathWidget.vue";
import StringListWidget from "./StringListWidget.vue";
import PropertyMapWidget from "./PropertyMapWidget.vue";
import ListWidget from "./ListWidget.vue";
import SectionWidget from "./SectionWidget.vue";

// `path` (D57): the config_path this field renders at, threaded through
// unchanged to the resolved widget so it can anchor its diagnostic marker
// by exact-string equality. Absent when the editor mounts a widget outside
// the diagnostics-providing EditorView (the standalone mount harness).
defineProps<{ spec: EditableField; path?: string }>();
const model = defineModel<unknown>();

function widgetComponentFor(kind: FieldWidget["kind"]): Component {
  switch (kind) {
    case "text":
      return TextWidget;
    case "bool":
      return BoolWidget;
    case "optionalFlag":
      return OptionalFlagWidget;
    case "select":
      return SelectWidget;
    case "keywordOrBlock":
      return KeywordOrBlockWidget;
    case "directoryPath":
      return DirectoryPathWidget;
    case "stringList":
      return StringListWidget;
    case "propertyMap":
      return PropertyMapWidget;
    case "list":
      return ListWidget;
    case "section":
      return SectionWidget;
    default: {
      const _exhaustive: never = kind;
      throw new Error(`unhandled FieldWidget kind: ${String(_exhaustive)}`);
    }
  }
}
</script>

<template>
  <component
    :is="widgetComponentFor(spec.widget.kind)"
    v-model="model"
    :spec="spec"
    :path="path"
    :data-help-id="spec.helpId"
  />
</template>
