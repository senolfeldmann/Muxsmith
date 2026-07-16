<script setup lang="ts">
// D45 field widget: `{ kind: "list"; item; reorderable }`. Each item
// renders through `SectionWidget` (synthesized against `item`'s own
// registry), recursing via `FieldWidgetDispatcher`. This generic widget is
// NOT the spec 8.2 top-level rule grid (`tracks.rules`/
// `attachments.rules`): that is Task 11's own bespoke grid with drag
// handles and per-rule summaries, built directly against `profile.ts`
// types rather than through this dispatcher. The registry's only current
// consumers of a generic `list` are `matchExpr.any`/`matchExpr.not`, both
// `reorderable: false`; `reorderable: true` is still handled here (native
// HTML5 drag-and-drop on the item itself, matching spec 8.2's own "drag to
// reorder" wording, needs no translated chrome) so the widget stays
// correct against the full `FieldWidget` contract, not just today's
// registry contents. Add/remove reuse `editor-attachment-rule-add`/`-drop`
// -- see `PropertyMapWidget.vue`'s doc for why that reuse is sound.
import { computed } from "vue";
import type { EditableFieldOf } from "./shared";
import SectionWidget from "./SectionWidget.vue";

const props = defineProps<{ spec: EditableFieldOf<"list"> }>();
const model = defineModel<unknown[] | null>();

const items = computed(() => model.value ?? []);

const itemSpec = computed<EditableFieldOf<"section">>(() => ({
  labelKey: props.spec.labelKey,
  widget: { kind: "section", of: props.spec.widget.item, optional: false },
}));

function itemValue(index: number): Record<string, unknown> | null {
  return (items.value[index] as Record<string, unknown> | null) ?? null;
}

function setItemValue(index: number, value: unknown) {
  const next = [...items.value];
  next[index] = value;
  model.value = next;
}

function addItem() {
  model.value = [...items.value, {}];
}

function removeItem(index: number) {
  const next = [...items.value];
  next.splice(index, 1);
  model.value = next;
}

let dragIndex: number | null = null;

function onDragStart(index: number) {
  dragIndex = index;
}

function onDrop(index: number) {
  if (dragIndex === null || dragIndex === index) {
    return;
  }
  const next = [...items.value];
  const [moved] = next.splice(dragIndex, 1);
  next.splice(index, 0, moved);
  model.value = next;
  dragIndex = null;
}
</script>

<template>
  <fieldset>
    <legend>{{ $t(spec.labelKey) }}</legend>
    <div
      v-for="(_, index) in items"
      :key="index"
      :draggable="spec.widget.reorderable"
      @dragstart="onDragStart(index)"
      @dragover.prevent
      @drop="onDrop(index)"
    >
      <SectionWidget
        :spec="itemSpec"
        :model-value="itemValue(index)"
        @update:model-value="setItemValue(index, $event)"
      />
      <button
        type="button"
        :aria-label="$t('editor-attachment-rule-drop')"
        @click="removeItem(index)"
      >
        {{ $t("editor-attachment-rule-drop") }}
      </button>
    </div>
    <button
      type="button"
      @click="addItem"
    >
      {{ $t("editor-attachment-rule-add") }}
    </button>
  </fieldset>
</template>
