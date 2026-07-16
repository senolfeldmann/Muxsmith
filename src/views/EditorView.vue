<script setup lang="ts">
// Task 11 (D45, spec 8.2 view 4): the profile editor's top-level rule
// grid and its drag-reorder, mounted standalone via the wave-3 harness (no
// nav entry exists until Task 13's App.vue wiring). Bespoke against
// `profile.ts` types directly, NOT through the field-widget dispatcher --
// see `ListWidget.vue`'s own doc on why the generic `list` widget is not
// this grid. `tracks.rules` stays this bespoke, read-only-summary grid
// unchanged by Task 12 (its own test is untouched); `tracks.unmatched`
// joins it as a normal registry-dispatched field, below.
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
//
// Task 12 (D45, amended 2026-07-16): SECTION COMPOSITION, driven by
// `profileFields` (Task 9's registry), not by a hand-listed field set --
// adding a field to `Profile` + its registry surfaces it here with no view
// edit. Every top-level `EditableField` dispatches generically through
// `FieldWidgetDispatcher`, with exactly ONE hand-built exception: `tracks`.
// `tracks.unmatched` still dispatches generically (below); `tracks.rules`
// keeps the bespoke grid above instead of the generic `list` widget, for
// the same reason `ListWidget.vue`'s own doc gives for why `attachments.
// rules` (unlike `tracks.rules`) DOES render generically -- TrackRule's
// shape lends itself to compact single-row summaries (source/match/
// changes as text, optional as a column), and Task 11 already built that
// grid with drag-reorder before this task existed; replacing it with the
// generic per-item `SectionWidget` form would drop the grid semantics
// (and would break Task 11's own `data-testid="editor-rule-row"` /
// `toContainText` assertions, which this task does not touch). `profile_
// version` (`FixedField`) is skipped the same way `SectionWidget.vue`
// skips one: filtered out, never dispatched, nothing rendered for it.
import { computed } from "vue";
import type { Profile, TrackRule } from "../bindings/profile";
import { SOURCE_KEYWORDS } from "../bindings/keywords";
import type { EditableField } from "../editor/fieldSpec";
import { profileFields, tracksFields } from "../editor/registries";
import FieldWidgetDispatcher from "../editor/widgets/FieldWidgetDispatcher.vue";

const model = defineModel<Profile>();

const rules = computed<TrackRule[]>(() => model.value?.tracks.rules ?? []);

// Every top-level `profileFields` entry except `tracks` (see the doc
// comment above) and `profile_version` (`FixedField`, never rendered).
const topLevelFields = computed(() =>
  Object.entries(profileFields).filter(
    (entry): entry is [string, EditableField] => entry[0] !== "tracks" && !("fixed" in entry[1]),
  ),
);

// `tracksFields.unmatched` is a real `EditableField` (registries.ts's own
// literal), but `tracksFields`'s declared type is `Record<keyof TracksCfg,
// FieldSpec>` (the `FixedField | EditableField` union), so a single
// property access does not narrow on its own -- same asymmetry `fields`
// in `SectionWidget.vue` closes with a runtime `"fixed" in entry[1]` guard
// over the whole registry; here there is exactly one known field, so a
// direct assertion is the equivalent for a single access.
const tracksUnmatchedSpec = tracksFields.unmatched as EditableField;

function fieldValue(key: string): unknown {
  return (model.value as unknown as Record<string, unknown> | undefined)?.[key];
}

function setFieldValue(key: string, value: unknown) {
  model.value = { ...(model.value ?? ({} as Profile)), [key]: value } as Profile;
}

function setTracksUnmatched(value: unknown) {
  if (!model.value) {
    return;
  }
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, unmatched: value as Profile["tracks"]["unmatched"] },
  };
}

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
    <FieldWidgetDispatcher
      v-for="[key, spec] in topLevelFields"
      :key="key"
      :spec="spec"
      :model-value="fieldValue(key)"
      @update:model-value="setFieldValue(key, $event)"
    />

    <fieldset>
      <legend>{{ $t("editor-profile-tracks") }}</legend>
      <FieldWidgetDispatcher
        :spec="tracksUnmatchedSpec"
        :model-value="model?.tracks.unmatched"
        @update:model-value="setTracksUnmatched"
      />
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
    </fieldset>
  </section>
</template>
