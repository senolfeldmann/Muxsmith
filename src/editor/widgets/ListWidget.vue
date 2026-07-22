<script setup lang="ts">
// D45 field widget: `{ kind: "list"; item; reorderable }`. Each item
// renders through `SectionWidget` (synthesized against `item`'s own
// registry), recursing via `FieldWidgetDispatcher`. This generic widget is
// NOT the spec 8.2 top-level rule grid (`tracks.rules`): that is Task 11's
// own bespoke grid with drag handles and per-rule summaries, built
// directly against `profile.ts` types rather than through this
// dispatcher. `attachments.rules` (`reorderable: true`) DOES render
// through this generic widget -- its rows are not scalar summaries the
// way TrackRule's are (two of its three fields are themselves nested
// match expressions), so there is no compact grid to build, and native
// HTML5 drag-and-drop on the item itself (matching spec 8.2's own "drag to
// reorder" wording, needs no translated chrome) is already handled here.
// Add/remove use the two generic `editor-action-add`/`-remove` keys
// (owner Ruling 1, amended 2026-07-16; the closed editor catalog budget,
// `editor-generic-action-keys`) -- not `editor-attachment-rule-add`/`-drop`,
// which now caption only the AttachmentRule fields they are the registry
// labels for.
import { computed } from "vue";
import type { EditableFieldOf } from "./shared";
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";
import SectionWidget from "./SectionWidget.vue";

const props = defineProps<{ spec: EditableFieldOf<"list">; path?: string }>();
const model = defineModel<unknown[] | null>();

const items = computed(() => model.value ?? []);

const itemSpec = computed<EditableFieldOf<"section">>(() => ({
  labelKey: props.spec.labelKey,
  widget: { kind: "section", of: props.spec.widget.item, optional: false },
}));

// The list's own path anchors the bare `{p}.any` / `{p}.not` diagnostics
// (EmptyMatchList) at the list root; each item's `[{i}]` path is threaded
// down to its section, which anchors its own marker (D57).
const { diags, severity } = useDiagAnchor(() => props.path);

function itemPath(index: number): string | undefined {
  return props.path === undefined ? undefined : `${props.path}[${index}]`;
}

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
    dragIndex = null;
    return;
  }
  const next = [...items.value];
  const [moved] = next.splice(dragIndex, 1);
  next.splice(index, 0, moved);
  model.value = next;
  dragIndex = null;
}

// A drag that leaves the list (or is cancelled with Escape) fires no `drop`
// at all, leaving `dragIndex` stale for the NEXT unrelated drag-and-drop --
// a stray drop could then pair with a stale index instead of the drag that
// actually produced it. `dragend` fires unconditionally on every drag,
// dropped or not, so it is the one event that always resets it.
function onDragEnd() {
  dragIndex = null;
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
    <div
      v-for="(_, index) in items"
      :key="index"
      :draggable="spec.widget.reorderable"
      @dragstart="onDragStart(index)"
      @dragover.prevent
      @drop="onDrop(index)"
      @dragend="onDragEnd"
    >
      <SectionWidget
        :spec="itemSpec"
        :path="itemPath(index)"
        :model-value="itemValue(index)"
        @update:model-value="setItemValue(index, $event)"
      />
      <button
        type="button"
        :aria-label="$t('editor-action-remove')"
        @click="removeItem(index)"
      >
        {{ $t("editor-action-remove") }}
      </button>
    </div>
    <button
      type="button"
      @click="addItem"
    >
      {{ $t("editor-action-add") }}
    </button>
  </fieldset>
</template>
