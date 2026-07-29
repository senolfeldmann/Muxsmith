<script setup lang="ts">
// Task 11 (D45, spec 8.2 view 4): the profile editor's top-level rule
// grid and its drag-reorder, mounted standalone via the wave-3 harness (no
// nav entry exists until Task 13's App.vue wiring). Bespoke against
// `profile.ts` types directly, NOT through the field-widget dispatcher --
// see `ListWidget.vue`'s own doc on why the generic `list` widget is not
// this grid. `tracks.rules` stays this bespoke, read-only summary OF THE
// ROW VALUES, unchanged by Task 12 (its own test is untouched); but the
// grid now also carries row SELECTION, and a detail panel below it (Task
// 13b) edits the selected rule through `SectionWidget` over `trackRule` --
// the same registry path `attachments.rules` uses through `ListWidget`,
// closing the spec-8.2 "detail editor per rule" gap
// (`registry-slot-capability-delta`, `docs/decision-ledger.yaml`).
// `tracks.unmatched` joins it as a normal registry-dispatched field, below.
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
//
// Task 13b (D45, spec 8.2, amended 2026-07-16, detail-editor routing): row
// selection on the grid (a native `<button data-testid="editor-rule-
// select">` with `:aria-current`, the `jobs-history-run` button in
// `RunHistory.vue` being the house precedent -- keyboard-reachable for
// free, no hand-rolled interactive `<tr>`) plus the per-rule detail panel
// beneath the grid. The panel is pure registry composition, byte-for-byte
// the machinery `ListWidget.vue`
// already uses for AttachmentRule items: it synthesizes a `{ kind:
// "section", of: "trackRule" }` spec, mounts `SectionWidget`, and writes
// the selected rule back immutably (`setRuleValue`). No new component, no
// new registry, no new catalog key: the panel is labelled via
// `aria-labelledby` pointing at the selected grid row's own id
// (`editor-rule-row-${index}`), and `SectionWidget`'s legend reuses
// `editor-tracks-rules` ("Rules"), already the grid heading/caption.
// Selection is cleared on reorder (`onDrop`) so a post-reorder edit can
// never land on a rule the user did not re-select.
import { computed, onMounted, provide, ref, watch } from "vue";
import { useFluent } from "fluent-vue";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import type { Profile, TrackRule } from "../bindings/profile";
import { SOURCE_KEYWORDS } from "../bindings/keywords";
import type { EditableField } from "../editor/fieldSpec";
import { profileFields, tracksFields } from "../editor/registries";
import FieldWidgetDispatcher from "../editor/widgets/FieldWidgetDispatcher.vue";
import SectionWidget from "../editor/widgets/SectionWidget.vue";
import type { EditableFieldOf } from "../editor/widgets/shared";
import { editorDiagnosticsByPath, worstSeverity } from "../editor/diagAnchor";
import { diagnosticFluentParams } from "../diagnosticFluentParams";
import DiagnosticsPanel from "../components/DiagnosticsPanel.vue";
import { getSettings, loadProfile, saveProfile, validateProfileModel } from "../ipc";
import type { Diagnostic, IpcError } from "../ipc";
import { rememberRecentProfile } from "../recentProfiles";

const model = defineModel<Profile>();

const fluent = useFluent();

// --- Task 13: open/save/validate state ----------------------------------

const currentPath = ref<string | null>(null);
const diagnostics = ref<Diagnostic[]>([]);
const opening = ref(false);
const saving = ref(false);
const ipcErrorCode = ref<string | null>(null);
const ipcErrorParams = ref<Record<string, string | number>>({});

// Task 13c (spec 8.2, whole-branch Finding 1): the shared recents memory
// (`src/recentProfiles.ts`), fed on every open below and rendered in the
// pre-Open empty state (template). The read is tolerant, mirroring
// BatchView's own onMounted tolerance (T9/T10): a failed fetch just leaves
// the list empty, the editor stays fully usable.
const recents = ref<string[]>([]);

onMounted(async () => {
  try {
    recents.value = (await getSettings()).recent_profiles;
  } catch {
    // Tolerant, mirrors BatchView's onMounted: recents start empty, the
    // editor stays usable.
  }
});

const hasErrors = computed(() => diagnostics.value.some((d) => d.severity === "error"));

// --- Task 14 (D57): field-anchored diagnostic markers -------------------
//
// Group the diagnostics by `config_path` and provide the map; each widget
// anchors its marker by EXACT string equality against the paths the widget
// tree constructs while rendering (the composition layer mirrors core's
// emission grammar verbatim, never parsing or normalizing). A path with no
// rendered control (`profile_version`, or any future core path the tree
// does not know) is silently panel-only -- the marker layer is strictly
// additive and the diagnostics panel below is never filtered.
const diagnosticsByPath = computed(() => {
  const map = new Map<string, Diagnostic[]>();
  for (const d of diagnostics.value) {
    const list = map.get(d.config_path) ?? [];
    list.push(d);
    map.set(d.config_path, list);
  }
  return map;
});
provide(editorDiagnosticsByPath, diagnosticsByPath);

// The bespoke rule grid is not a registry-dispatched widget, so its caption
// (`tracks.rules`) and each row (`tracks[{i}]`, which also anchors lint's
// ProvableOverlap) look up their anchors against the map here directly.
function pathDiags(path: string): Diagnostic[] {
  return diagnosticsByPath.value.get(path) ?? [];
}
function markerTitle(diags: Diagnostic[]): string {
  return diags.map((d) => fluent.$t(d.code, diagnosticFluentParams(d.code, d.params))).join("\n");
}
const rulesCaptionDiags = computed(() => pathDiags("tracks.rules"));
const rulesCaptionSeverity = computed(() => worstSeverity(rulesCaptionDiags.value));
function ruleRowDiags(index: number): Diagnostic[] {
  return pathDiags(`tracks[${index}]`);
}
function ruleRowSeverity(index: number) {
  return worstSeverity(ruleRowDiags(index));
}

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

// Task 13c: one funnel. Both the Open dialog and a recents-list click route
// through `openPath` -- mirrors BatchView routing both its pick and its
// recents-click through `selectProfile` (T10).
async function openPath(path: string): Promise<void> {
  if (opening.value || saving.value) {
    return;
  }
  opening.value = true;
  ipcErrorCode.value = null;
  try {
    const doc = await loadProfile(path);
    currentPath.value = path;
    diagnostics.value = doc.config_diagnostics;
    model.value = doc.profile ?? undefined;
    // Background bookkeeping only (mirrors BatchView's identical
    // tolerance): `rememberRecentProfile` swallows and returns `null` on
    // failure, so a recents-write failure never reaches this `catch` and
    // never surfaces as an open error.
    const persisted = await rememberRecentProfile(path);
    if (persisted) {
      recents.value = persisted.recent_profiles;
    }
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    opening.value = false;
  }
}

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
  await openPath(picked);
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

// --- Task 13b: row selection + the per-rule detail panel -----------------
//
// The grid stays Task 11's read-only summary of the row VALUES, but a row
// is now selectable, and the panel below the grid edits the selected rule
// through the existing registry composition: `SectionWidget` over the
// `trackRule` registry, byte-for-byte the machinery `ListWidget.vue` already
// uses for AttachmentRule items (`attachments.rules`) -- the same code path,
// closing the spec-8.2 "detail editor per rule" gap
// (`registry-slot-capability-delta`) with zero new components and zero new
// catalog keys.

const selectedIndex = ref<number | null>(null);

// The detail panel edits rule `selectedIndex` through `SectionWidget` over
// `trackRule`; its path root is `tracks[{i}]` (D57), so every field it
// dispatches anchors at the same paths core emits for that rule. The root
// itself is a child-path prefix only: the bespoke grid row already anchors
// `tracks[{i}]` (lint's ProvableOverlap, the design-named anchor), so the
// mount passes `suppress-self-anchor` to avoid a redundant second marker at
// that path -- mirroring the same-path collision `KeywordOrBlockWidget`
// suppresses for its nested block.
const selectedPath = computed(() =>
  selectedIndex.value === null ? undefined : `tracks[${selectedIndex.value}]`,
);

function selectRule(index: number) {
  selectedIndex.value = index;
}

const ruleDetailSpec: EditableFieldOf<"section"> = {
  labelKey: "editor-tracks-rules",
  widget: { kind: "section", of: "trackRule", optional: false },
};

// Mirrors `ListWidget.vue`'s `itemValue`/`setItemValue` and this file's own
// `onDrop` immutable rebuild -- the `Record<string, unknown>` cast on the
// way in and the `TrackRule` cast on the way out are the same asymmetry
// `ListWidget` closes.
const selectedRule = computed<Record<string, unknown> | null>(() =>
  selectedIndex.value === null
    ? null
    : ((rules.value[selectedIndex.value] as Record<string, unknown> | undefined) ?? null),
);

function setRuleValue(value: unknown) {
  if (selectedIndex.value === null || !model.value) {
    return;
  }
  const next = [...rules.value];
  next[selectedIndex.value] = value as TrackRule;
  model.value = { ...model.value, tracks: { ...model.value.tracks, rules: next } };
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
  // A reorder invalidates any prior selection index mapping to a rule
  // identity, not a position -- clearing it forces a post-reorder edit to
  // go through a fresh, explicit re-selection.
  selectedIndex.value = null;
}

// Same rationale as `ListWidget.vue`'s own `onDragEnd`: a drag that leaves
// the grid or is cancelled fires no `drop`, and `dragend` is the one event
// that always fires regardless, so it is the reliable place to clear a
// stale `dragIndex` before it can pair with an unrelated later drop.
function onDragEnd() {
  dragIndex = null;
}

// --- Plan 7.5 (D65-D70, D72): Add/Remove for track rules -----------------
//
// Add appends the EMPTY SKELETON `{ match: {} }` (D65) -- invalid-until-
// filled, guided by the existing diagnostics plumbing rather than by a
// prefilled guess; it auto-selects the new rule, so the detail panel opens
// purely reactively (`v-if="selectedRule"`), with no focus call anywhere
// (D67). Both mutations are the same immutable whole-model rebuild every
// other mutation in this view performs.
function addRule() {
  if (!model.value) return;
  const next = [...rules.value, { match: {} }];
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, rules: next },
  };
  selectedIndex.value = next.length - 1;
}

// Selection clears after removal, same rationale as `onDrop` above: indices
// after the removed rule shift by one, so a retained index would silently
// point at a different rule -- selection maps to an identity, not a
// position. Remove works down to zero rules; the zero-rule state is legal
// and surfaced by core's own diagnostics (D69), so the editor holds no
// guard against it.
function removeSelectedRule() {
  if (selectedIndex.value === null || !model.value) return;
  const next = [...rules.value];
  next.splice(selectedIndex.value, 1);
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, rules: next },
  };
  selectedIndex.value = null;
}
</script>

<template>
  <section
    data-testid="view-editor"
    data-help-id="view-editor"
  >
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

    <section
      v-if="!currentPath && recents.length"
      aria-labelledby="editor-recents-heading"
      data-testid="editor-recents"
    >
      <h4 id="editor-recents-heading">
        {{ $t("batch-recents-heading") }}
      </h4>
      <ul>
        <li
          v-for="path in recents"
          :key="path"
        >
          <button
            type="button"
            data-testid="editor-recent-profile"
            :disabled="opening || saving"
            @click="openPath(path)"
          >
            {{ path }}
          </button>
        </li>
      </ul>
    </section>

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
        :path="key"
        :model-value="fieldValue(key)"
        @update:model-value="setFieldValue(key, $event)"
      />

      <fieldset>
        <legend>{{ $t("editor-profile-tracks") }}</legend>
        <FieldWidgetDispatcher
          :spec="tracksUnmatchedSpec"
          path="tracks.unmatched"
          :model-value="model?.tracks.unmatched"
          @update:model-value="setTracksUnmatched"
        />
        <h2>{{ $t("editor-tracks-rules") }}</h2>
        <table>
          <caption data-help-id="editor-tracks-rules">
            {{ $t("editor-tracks-rules") }}
            <span
              v-if="rulesCaptionSeverity !== null"
              role="img"
              class="diag-marker"
              :class="`diag-marker--${rulesCaptionSeverity}`"
              data-testid="diag-marker"
              data-diag-path="tracks.rules"
              :aria-label="$t(`severity-${rulesCaptionSeverity}`)"
              :title="markerTitle(rulesCaptionDiags)"
            />
          </caption>
          <thead>
            <tr>
              <th scope="col">
                {{ $t("editor-track-rule-order") }}
              </th>
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
              :id="`editor-rule-row-${index}`"
              :key="index"
              data-testid="editor-rule-row"
              draggable="true"
              @dragstart="onDragStart(index)"
              @dragover.prevent
              @drop="onDrop(index)"
              @dragend="onDragEnd"
            >
              <td>{{ index + 1 }}</td>
              <td>
                <button
                  type="button"
                  data-testid="editor-rule-select"
                  :aria-current="selectedIndex === index ? 'true' : undefined"
                  @click="selectRule(index)"
                >
                  {{ sourceSummary(rule) }}
                </button>
                <span
                  v-if="ruleRowSeverity(index) !== null"
                  role="img"
                  class="diag-marker"
                  :class="`diag-marker--${ruleRowSeverity(index)}`"
                  data-testid="diag-marker"
                  :data-diag-path="`tracks[${index}]`"
                  :aria-label="$t(`severity-${ruleRowSeverity(index)}`)"
                  :title="markerTitle(ruleRowDiags(index))"
                />
              </td>
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
        <button
          type="button"
          data-testid="editor-rule-add"
          @click="addRule"
        >
          {{ $t("editor-action-add") }}
        </button>
        <button
          type="button"
          data-testid="editor-rule-remove"
          :disabled="selectedIndex === null"
          @click="removeSelectedRule"
        >
          {{ $t("editor-action-remove") }}
        </button>
      </fieldset>

      <section
        v-if="selectedRule"
        data-testid="editor-rule-detail"
        :aria-labelledby="`editor-rule-row-${selectedIndex}`"
      >
        <SectionWidget
          :spec="ruleDetailSpec"
          :path="selectedPath"
          suppress-self-anchor
          :model-value="selectedRule"
          @update:model-value="setRuleValue"
        />
      </section>

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
