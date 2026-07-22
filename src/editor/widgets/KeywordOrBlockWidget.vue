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
import { useDiagAnchor } from "../diagAnchor";
import { diagnosticFluentParams } from "../../diagnosticFluentParams";
import SectionWidget from "./SectionWidget.vue";

const props = defineProps<{ spec: EditableFieldOf<"keywordOrBlock">; path?: string }>();
const model = defineModel<string | Record<string, unknown> | null>();

const id = useId();

// The keyword-or-block widget is the sole anchor for its own path (core's
// grammar has no segment for the union root beyond the field, e.g.
// `chapters`, `tracks[i].source`, `output.filename`, `title`); its nested
// block section receives the same path for child construction but does not
// re-anchor it (`anchor-self=false` below), so there is one marker, on the
// user-facing control, not a redundant second one on the block wrapper.
const { diags, severity } = useDiagAnchor(() => props.path);

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
      :value="isKeyword ? model : ''"
      :title="$ta(spec.labelKey).tooltip"
      :class="severity !== null ? `diag-anchored--${severity}` : undefined"
      :aria-invalid="severity === 'error' ? 'true' : undefined"
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
      :path="path"
      suppress-self-anchor
    />
  </div>
</template>
