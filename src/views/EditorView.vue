<script setup lang="ts">
// Task 11 (D45, spec 8.2 view 4): the profile editor's top-level rule
// grid and its drag-reorder, mounted standalone via the wave-3 harness (no
// nav entry exists until Task 13's App.vue wiring). Bespoke against
// `profile.ts` types directly, NOT through the field-widget dispatcher --
// see `ListWidget.vue`'s own doc on why the generic `list` widget is not
// this grid. Sections, the full per-rule detail editor, save and open IPC
// all follow in Tasks 12-13; this view holds only `tracks.rules` for now,
// but its `modelValue`/`update:modelValue` v-model contract is already the
// full `Profile`, so those tasks extend this file rather than reshaping
// its interface.
//
// Reorder is a SEMANTIC MODEL EDIT (binding note), not a DOM mutation: a
// drop rebuilds `tracks.rules` immutably and re-emits the whole profile,
// mirroring `ListWidget.vue`'s own reorder mechanics (a closure
// `dragIndex`, no `dataTransfer` read -- native HTML5 drag-and-drop needs
// no translated chrome for that reason, same rationale as that widget).
//
// Zero frontend semantic validation (spec 7): every row summary below is
// either a real profile-format token (`SOURCE_KEYWORDS`'s "primary"), an
// actual value already present in the model (an external path, a matchable
// property name and its scalar, a settable property name), or a plain
// count -- never app-authored prose, so no new `gui-editor.ftl` key is
// needed beyond the four `TrackRule` field labels the registry already
// carries (D45).
import { computed } from "vue";
import type { Profile, TrackRule } from "../bindings/profile";
import { SOURCE_KEYWORDS } from "../bindings/keywords";

const model = defineModel<Profile>();

const rules = computed<TrackRule[]>(() => model.value?.tracks.rules ?? []);

function sourceSummary(rule: TrackRule): string {
  if (typeof rule.source === "string") {
    return rule.source;
  }
  if (rule.source) {
    return rule.source.external.path;
  }
  return SOURCE_KEYWORDS[0];
}

function matchSummary(rule: TrackRule): string {
  const expr = rule.match;
  const parts: string[] = [];
  for (const [key, value] of Object.entries(expr.exact ?? {})) {
    parts.push(`${key}=${value}`);
  }
  for (const [key, value] of Object.entries(expr.substring ?? {})) {
    parts.push(`${key}~${value}`);
  }
  for (const [key, value] of Object.entries(expr.regex ?? {})) {
    parts.push(`${key}~/${value}/`);
  }
  if (expr.any && expr.any.length > 0) {
    parts.push(`any(${expr.any.length})`);
  }
  if (expr.not && expr.not.length > 0) {
    parts.push(`not(${expr.not.length})`);
  }
  return parts.join(", ");
}

function changesSummary(rule: TrackRule): string {
  return Object.keys(rule.changes ?? {}).join(", ");
}

let dragIndex: number | null = null;

function onDragStart(index: number) {
  dragIndex = index;
}

function onDrop(index: number) {
  if (dragIndex === null || dragIndex === index || !model.value) {
    dragIndex = null;
    return;
  }
  const nextRules = [...rules.value];
  const [moved] = nextRules.splice(dragIndex, 1);
  nextRules.splice(index, 0, moved);
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, rules: nextRules },
  };
  dragIndex = null;
}
</script>

<template>
  <section data-testid="view-editor">
    <h2>{{ $t("editor-tracks-rules") }}</h2>
    <table>
      <caption>
        {{ $t("editor-tracks-rules") }}
      </caption>
      <thead>
        <tr>
          <th scope="col">
            {{ $t("editor-track-rule-source") }}
          </th>
          <th scope="col">
            {{ $t("editor-track-rule-match-expr") }}
          </th>
          <th scope="col">
            {{ $t("editor-track-rule-optional") }}
          </th>
          <th scope="col">
            {{ $t("editor-track-rule-changes") }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(rule, index) in rules"
          :key="index"
          data-testid="editor-rule-row"
          draggable="true"
          @dragstart="onDragStart(index)"
          @dragover.prevent
          @drop="onDrop(index)"
        >
          <td>{{ sourceSummary(rule) }}</td>
          <td>{{ matchSummary(rule) }}</td>
          <td>
            <input
              type="checkbox"
              disabled
              :checked="rule.optional === true"
              :aria-label="$t('editor-track-rule-optional')"
            >
          </td>
          <td>{{ changesSummary(rule) }}</td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
