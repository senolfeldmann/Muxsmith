<script setup lang="ts">
// D45 field widget: `{ kind: "keywordOrBlock"; keywords; block }`. The
// underlying wire type is an untagged `Block | string` (e.g. FilenameCfg,
// ChaptersCfg, SourceCfg): the value is EITHER one of the fixed `keywords`
// tokens OR a `block`-shaped nested struct. No mode toggle switches
// between them (that needs UI-only translated text with no existing
// catalog key, and spec 7 keeps this class of either-or resolution out of
// the frontend already -- AttachmentRule's one-of is the named precedent:
// three independent optional sections, no toggle, core diagnoses an
// over-set model). The keyword combobox and the nested block section are
// therefore both always visible; whichever the user actually populates is
// what gets sent.
import { computed, useId } from "vue";
import type { EditableFieldOf } from "./shared";
import SectionWidget from "./SectionWidget.vue";

const props = defineProps<{ spec: EditableFieldOf<"keywordOrBlock"> }>();
const model = defineModel<string | Record<string, unknown> | null>();

const id = useId();

const isKeyword = computed(() => typeof model.value === "string");

const blockSpec = computed<EditableFieldOf<"section">>(() => ({
  labelKey: props.spec.labelKey,
  widget: { kind: "section", of: props.spec.widget.block, optional: true },
}));

const blockModel = computed<Record<string, unknown> | null | undefined>({
  get: () => (isKeyword.value ? undefined : (model.value as Record<string, unknown> | null | undefined)),
  set: (value) => {
    model.value = value;
  },
});
</script>

<template>
  <div>
    <label :for="id">{{ $t(spec.labelKey) }}</label>
    <select
      :id="id"
      :value="isKeyword ? model : ''"
      :title="$ta(spec.labelKey).tooltip"
      @change="model = ($event.target as HTMLSelectElement).value"
    >
      <option
        v-for="keyword in spec.widget.keywords"
        :key="keyword"
        :value="keyword"
      >
        {{ keyword }}
      </option>
    </select>
    <SectionWidget
      v-model="blockModel"
      :spec="blockSpec"
    />
  </div>
</template>
