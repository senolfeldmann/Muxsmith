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
//
// Task 13 (D45, D41, D42): open/save wiring, the save-surface standing
// note, and validate-on-edit. `EditorView` owns its own open/save/
// validate state (`currentPath`, `diagnostics`, the ipc-error pair)
// rather than App.vue, matching the brief's own "wire ... into
// EditorView.vue" step: App.vue's only change is the nav entry and mount
// block, no v-model, no shared editor state there. This keeps
// `EditorView` mountable from an injected `modelValue` alone -- the Tasks
// 10-12 mount-harness specs feed one directly, install no IPC mock, and
// never click Open, so the validate-on-edit watcher below is gated on
// `currentPath` (only Open's own IPC round trip ever sets it): a bare-
// mounted `EditorView` edits its model plenty (drag-reorder, widget
// input) but never triggers `validate_profile_model`, keeping those specs
// green with no injected mock (mount-harness amendment's own review-
// check).
//
// No new `gui-editor.ftl`/`gui-common.ftl` keys: D45's design-doc catalog
// table enumerates every Fluent key the whole plan adds, and Task 13
// carries none (`gui-editor.ftl` stays 45, the brief's own Files list
// carries no `.ftl`). The Open button, the currently-open-path line, and
// the file-dialog filter name reuse `batch-profile-pick`/`batch-profile-
// current`/`batch-profile-filter-name` (BatchView's own "choose + show a
// profile path" affordance -- their content is generic, not batch-
// specific, and cross-view key reuse already has two precedents:
// `browse-button`'s own documented reuse across BatchView/FirstRun/
// SettingsDialog, and `JobsView.vue`'s `<h2>` reusing `nav-jobs`
// internally rather than a bespoke `jobs-view-heading`). Save reuses
// `settings-save` ("Save", `SettingsDialog.vue`) the same way. The
// diagnostics heading reuses `batch-diagnostics-heading` ("Diagnostics").
// `editor-save-note` is Task 9's own key, exactly as the brief names it.
// Diagnostics render through the already-shared `DiagnosticsPanel.vue`
// (its own doc comment: "no per-caller variant"), a third consumer beside
// BatchView/ResolutionTable, contributing no new template `$t()` calls.
import { computed, ref, watch } from "vue";
import { useFluent } from "fluent-vue";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import type { Profile, TrackRule } from "../bindings/profile";
import { SOURCE_KEYWORDS } from "../bindings/keywords";
import type { EditableField } from "../editor/fieldSpec";
import { profileFields, tracksFields } from "../editor/registries";
import FieldWidgetDispatcher from "../editor/widgets/FieldWidgetDispatcher.vue";
import DiagnosticsPanel from "../components/DiagnosticsPanel.vue";
import { loadProfile, saveProfile, validateProfileModel } from "../ipc";
import type { Diagnostic, IpcError } from "../ipc";

const model = defineModel<Profile>();

const fluent = useFluent();

// --- Task 13: open/save/validate state ----------------------------------

const currentPath = ref<string | null>(null);
const diagnostics = ref<Diagnostic[]>([]);
const opening = ref(false);
const saving = ref(false);
const ipcErrorCode = ref<string | null>(null);
const ipcErrorParams = ref<Record<string, string>>({});

const hasErrors = computed(() => diagnostics.value.some((d) => d.severity === "error"));

// The one sanctioned frontend affordance (spec 7, D41 binding point):
// Save is disabled while any error-severity diagnostic exists.
const saveDisabled = computed(
  () => !model.value || !currentPath.value || hasErrors.value || saving.value || opening.value,
);

let validationGeneration = 0;

// Spec 7 ("every profile edit"): revalidates through `validate_profile_model`
// whenever the held model changes. Gated on `currentPath` -- see the doc
// comment above for why a bare mount-harness `EditorView` never reaches
// this. Every top-level `model.value =` assignment throughout the widget
// tree is a fresh object (`SectionWidget.vue`'s own `{ ...model.value,
// [key]: value }`, mirrored at every level up to this component's own
// `setFieldValue`/`setTracksUnmatched`/`onDrop` below), so a shallow watch
// (no `deep: true`) reliably fires on every leaf edit. `validationGeneration`
// discards a stale response: `validate_profile_model` runs on a Tauri
// blocking-task thread pool (D42's own doc on why it is `async` at all),
// so rapid edits can resolve out of order.
watch(model, async (value) => {
  if (!currentPath.value || !value) {
    return;
  }
  const generation = ++validationGeneration;
  try {
    const result = await validateProfileModel(value);
    if (generation === validationGeneration) {
      diagnostics.value = result.config_diagnostics;
    }
  } catch (e) {
    // Background, per-edit validation; a genuine failure here is an
    // internal-task panic (D42's own doc on `validate_profile_model`), not
    // a user action to react to -- mirrors BatchView's identical
    // tolerance for its own background `rememberRecentProfile` write.
    console.warn("[editor] background validation failed:", e);
  }
});

async function pickAndOpen(): Promise<void> {
  if (opening.value || saving.value) {
    return;
  }
  const picked = await openDialog({
    multiple: false,
    directory: false,
    filters: [{ name: fluent.$t("batch-profile-filter-name"), extensions: ["yaml", "yml"] }],
  });
  if (typeof picked !== "string") {
    return;
  }
  opening.value = true;
  ipcErrorCode.value = null;
  try {
    const doc = await loadProfile(picked);
    currentPath.value = picked;
    diagnostics.value = doc.config_diagnostics;
    model.value = doc.profile ?? undefined;
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    opening.value = false;
  }
}

async function doSave(): Promise<void> {
  if (saveDisabled.value || !model.value || !currentPath.value) {
    return;
  }
  saving.value = true;
  ipcErrorCode.value = null;
  try {
    await saveProfile(currentPath.value, model.value);
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    saving.value = false;
  }
}

// --- Tasks 11-12: section composition + the rule grid -------------------

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
    <button
      type="button"
      data-testid="editor-open"
      :disabled="opening || saving"
      :aria-busy="opening"
      @click="pickAndOpen"
    >
      {{ $t("batch-profile-pick") }}
    </button>
    <p v-if="currentPath">
      {{ $t("batch-profile-current", { path: currentPath }) }}
    </p>
    <p
      v-if="ipcErrorCode"
      role="alert"
    >
      {{ $t(ipcErrorCode, ipcErrorParams) }}
    </p>

    <section aria-labelledby="editor-diagnostics-heading">
      <h3 id="editor-diagnostics-heading">
        {{ $t("batch-diagnostics-heading") }}
      </h3>
      <DiagnosticsPanel :diagnostics="diagnostics" />
    </section>

    <template v-if="model">
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

      <p>{{ $t("editor-save-note") }}</p>
      <button
        type="button"
        data-testid="editor-save"
        :disabled="saveDisabled"
        :aria-busy="saving"
        @click="doSave"
      >
        {{ $t("settings-save") }}
      </button>
    </template>
  </section>
</template>
