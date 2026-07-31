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
// never click Open or New, so the validate-on-edit watcher below is gated
// on `sessionActive`, which only this view's own two funnels set
// (`openPath` on a completed load, `createBlank` on a fresh seed -- D107):
// a bare-mounted `EditorView` edits its model plenty (drag-reorder, widget
// input) but never triggers `validate_profile_model`, keeping those specs
// green with no injected mock (mount-harness amendment's own review-
// check).
//
// No new `gui-editor.ftl`/`gui-common.ftl` keys: D45's design-doc catalog
// table enumerates every Fluent key the whole plan adds, and Task 13
// carries none (the brief's own Files list carries no `.ftl`). Later
// packages did add to it: `gui-editor.ftl` carries 54 ids today, eight
// of them this view's own affordances: profile creation
// (`editor-action-new`, `editor-empty`, `editor-unsaved`, D107),
// undo/redo (`editor-action-undo`, `editor-action-redo`, D108) and
// the discard confirmation (`editor-discard-title`,
// `editor-discard-message`, `editor-discard-confirm`,
// D109). The Open button, the
// currently-open-path line, and the file-dialog filter name reuse
// `batch-profile-pick`/`batch-profile-current`/`batch-profile-filter-name`
// (BatchView's own "choose + show a profile path" affordance -- their
// content is generic, not batch-specific, and cross-view key reuse already
// has two precedents:
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
// the machinery `ListWidget.vue` already uses for AttachmentRule items: it
// synthesizes a `{ kind: "section", of: "trackRule" }` spec, mounts
// `SectionWidget`, and writes the selected rule back immutably
// (`setRuleValue`). No new component, no new registry, no new catalog key:
// the panel is labelled via `aria-labelledby` pointing at the selected
// grid row's own id (`editor-rule-row-${index}`), and `SectionWidget`'s
// legend reuses `editor-tracks-rules` ("Rules"), already the grid
// heading/caption. Selection is cleared on reorder (`onDrop`) so a
// post-reorder edit can never land on a rule the user did not re-select.
import { computed, onMounted, provide, ref, useTemplateRef, watch } from "vue";
import { useFluent } from "fluent-vue";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import type { Profile, TrackRule } from "../bindings/profile";
import { SOURCE_KEYWORDS } from "../bindings/keywords";
import type { EditableField } from "../editor/fieldSpec";
import { profileFields, tracksFields } from "../editor/registries";
import FieldWidgetDispatcher from "../editor/widgets/FieldWidgetDispatcher.vue";
import SectionWidget from "../editor/widgets/SectionWidget.vue";
import type { EditableFieldOf } from "../editor/widgets/shared";
import ConfirmDialog from "../components/ConfirmDialog.vue";
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

// --- Task 4 (D108): undo/redo history, and the derived save state -------
//
// One history, driven by the single existing mutation funnel below
// (`watch(model)`): every profile write already passes through it
// (measured at plan authoring: seven whole-value assignments, no in-place
// mutation, no external writer), so one funnel can serialize the model
// into one undo history instead of each mutation site keeping its own. A
// history entry is the WHOLE serialized model (`JSON.stringify`), never a
// diff (D108 decision 1).
const history = ref<string[]>([]);
const position = ref(-1);
// The serialization of the profile last WRITTEN to disk -- not the live
// model and not the live history entry (D108 decision 3; Step 1c below,
// in `doSave`, is the only writer besides `resetHistory`). `null` before
// any profile has ever been opened or created, or after a load that
// failed to parse (decision 9).
const savedSnapshot = ref<string | null>(null);
// The coalescing boundary for "one entry per editing burst" (decision 2):
// while true, the push rule below REPLACES the entry at `position`
// instead of appending one. Cleared by a focus change inside the editor
// (`@focusout` on the root section, Step 2) and, explicitly, by every
// discrete grid operation (`addRule`, `removeSelectedRule`, `onDrop`)
// before it mutates -- focus alone is not enough, since two consecutive
// clicks of the same button never move focus.
let coalesce = false;
// D108 decision 5: on a push past this cap the oldest entry is dropped
// and `position` decremented. Measured in the form an entry actually
// takes (compact JSON of the serde-normalized model, defaults omitted per
// D48): the New seed is 101 bytes and the README's four-rule example 419
// bytes, so 100 entries of a realistic profile stays well under 50 KB and
// a pathological 20 KB profile still bounds the history at 2 MB.
// `savedSnapshot` is a value, not an index, so truncation cannot corrupt
// the dirty computation below -- dropping the saved snapshot out of the
// history only means undo cannot walk back that far.
const HISTORY_DEPTH = 100;

// D107: a profile entered the editor through one of its own funnels,
// `openPath` or `createBlank`. It exists rather than the gates below
// reusing `currentPath` because a CREATED profile has no path at all --
// gating validation on `currentPath` would leave a new profile
// unvalidated until its first save. It is also not `model`: the bare
// mount-harness case (a spec injects `modelValue` and installs no IPC
// mock) must keep firing no IPC, which is a safeguard bought deliberately
// in plan 6 and preserved here rather than dropped.
//
// Task 4 (D108 decision 4): now DERIVED from `savedSnapshot` rather than a
// ref both funnels set directly. "A profile is in the editor" and "there
// is a clean baseline to compare it against" are the same fact,
// established at the same two moments -- `resetHistory`, below, called
// from `openPath` and `createBlank` -- by construction, so a second flag
// could only ever disagree with the first through a bug.
const sessionActive = computed(() => savedSnapshot.value !== null);

// D112 (owner ruling 2026-07-31): the pre-session state -- nothing has
// been opened or created at all -- and the ONE definition both surfaces
// that may appear only in that state read. Two terms, because two facts
// have to be absent at once: `model` carries "the editor holds
// something", `currentPath` carries "a file has been bound to the
// editor". A load that resolves but fails to parse leaves the second set
// and clears the first (`openPath` sets `currentPath`, then assigns
// `doc.profile ?? undefined`), and that state is the one the ruling is
// about: the path line names the failing file and the panel carries the
// parse error, so a second sentence saying no profile is open, plus a
// recents list offering a fresh start, contradict what is already on
// screen.
//
// NOT `sessionActive`: this task derives it from `savedSnapshot`, which
// the failed-load branch nulls (D108 decision 9), so `!sessionActive` is
// TRUE in exactly the state these two surfaces must stay hidden in. NOT
// `!model` alone: that is the gate Task 3 shipped, and it is what renders
// both surfaces over a failed load today.
const nothingOpenedOrCreated = computed(
  () => !model.value && currentPath.value === null,
);

// D108 decision 4: the save state is DERIVED and there is no second
// mechanism -- a value comparison, not an index comparison. Its failure
// direction is annoyance, never data loss: a spurious `dirty` warns where
// nothing was at risk, where a hand-set boolean a mutation path forgets
// would silently fail to warn where something was. That direction holds
// only because `savedSnapshot` marks the profile that was WRITTEN (Step
// 1c in `doSave`) rather than the live history entry -- marking the live
// entry would invert it exactly, reporting clean over content the file
// does not hold.
//
// Consumed below by this task's two discard guards (`pickAndOpen`,
// `createBlank`, D109 decisions 1-2) and, not yet built, by Task 6's
// close-app guard.
const dirty = computed(
  () => savedSnapshot.value !== null && history.value[position.value] !== savedSnapshot.value,
);

const canUndo = computed(() => position.value > 0);
const canRedo = computed(() => position.value < history.value.length - 1);

// D108 decisions 8-9, the reset sibling of the funnel below: given a
// profile, the history becomes a single clean entry holding it; given
// `undefined` -- the failed-load branch, where `doc.profile` is null and
// the diagnostic carries the parse error -- the history is CLEARED
// instead of left standing, so `sessionActive`/`dirty`/`canUndo`/
// `canRedo` are all false over a profile that never entered the editor,
// and undo cannot resurrect the previous one into a path that just failed
// to parse. `openPath` calls this with `doc.profile ?? undefined` on the
// exact value it assigns to the model, so the two can never disagree.
function resetHistory(profile: Profile | undefined): void {
  if (profile === undefined) {
    history.value = [];
    position.value = -1;
    savedSnapshot.value = null;
    coalesce = false;
    return;
  }
  const serialized = JSON.stringify(profile);
  history.value = [serialized];
  position.value = 0;
  savedSnapshot.value = serialized;
  coalesce = false;
}

const diagnostics = ref<Diagnostic[]>([]);
const opening = ref(false);
const saving = ref(false);
const ipcErrorCode = ref<string | null>(null);
const ipcErrorParams = ref<Record<string, string | number>>({});

// D109 decision 6: the one `ConfirmDialog` this view mounts, asked by both
// discard guards below (`pickAndOpen`, `createBlank`).
const confirmDialog = useTemplateRef("confirmDialog");

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
  () => !model.value || hasErrors.value || saving.value || opening.value,
);

let validationGeneration = 0;

// Spec 7 ("every profile edit"): revalidates through `validate_profile_model`
// whenever the held model changes. Gated on `sessionActive`, which both of
// this view's funnels set (`openPath` and `createBlank`) -- see the doc
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
  if (!sessionActive.value || !value) {
    return;
  }
  // D108 decision 1: a COMPARISON, not a flag. An undo-driven assignment
  // sets `model.value` to `history[position]` itself (`undo`/`redo`
  // below), so this comparison matches and the write cannot push -- no
  // "applying history" latch exists anywhere in this file to forget to
  // set or clear. The same comparison incidentally dedupes a widget that
  // re-emits an identical value.
  const serialized = JSON.stringify(value);
  if (serialized !== history.value[position.value]) {
    if (coalesce) {
      history.value = [...history.value.slice(0, position.value), serialized];
    } else {
      history.value = [...history.value.slice(0, position.value + 1), serialized];
      position.value = history.value.length - 1;
      if (history.value.length > HISTORY_DEPTH) {
        history.value = history.value.slice(1);
        position.value -= 1;
      }
      coalesce = true;
    }
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
    resetHistory(doc.profile ?? undefined);
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
  // D109 decision 1: confirm, then the file dialog, then replace. Declining
  // returns without touching anything; a cancelled file dialog after a
  // confirmed discard leaves the model untouched too, because nothing is
  // discarded until `openPath` below actually succeeds.
  if (dirty.value && !(await confirmDialog.value?.ask())) {
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

// D107 decision 1, spec 8.2: the seed New puts in the editor. A FUNCTION
// rather than a shared constant, so every New gets a fresh object no
// earlier session's edits can reach -- the same immutable-rebuild
// discipline every other write in this view follows.
//
// `input.extensions` carries a value because the validator forces one, not
// for convenience: an empty list is `empty-extensions` at ERROR severity
// (measured), and an error disables Save, so a bare schema-minimum seed
// would greet the user with a dead Save button. What the seed does produce
// is exactly one diagnostic, `empty-match-expression` at WARNING severity
// on `tracks[0].match` -- incomplete-until-filled and announced, the same
// idiom spec 8.2 already blesses for Add's empty rule, one level up.
// `pattern: ".*"` over `""`: both are diagnostic-free, and `.*` makes the
// identifier the whole basename rather than the empty string for every
// file, so the seed is immediately usable in a dry run.
function blankProfile(): Profile {
  return {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: { rules: [{ match: {} }] },
  };
}

// The creation funnel (D107), sibling of `openPath`: it touches no file,
// so it holds no `opening`/`saving` flag of its own -- it only refuses to
// run while one of those round trips is in flight, the same guard
// `pickAndOpen` uses. D109 makes it `async` (the discard guard's own
// `await`, below), but that await is gated on `dirty` and short-circuits
// entirely while the editor is clean, so the common case runs synchronously
// to completion exactly as before.
//
// While the body stays fully synchronous, the statement order inside it
// does not matter. `watch(model)` above runs at Vue's default
// `flush: "pre"` -- the callback is QUEUED, not run at the assignment -- so
// by the time it reads `sessionActive` this entire body has finished and
// every write is visible to it whichever order they were made in. The same
// fact covers `diagnostics`: no render happens between two synchronous ref
// writes, so a previous profile's findings cannot paint against the new
// model from any position. Measured: swapping `resetHistory`'s call and
// `model`'s assignment, and moving the `diagnostics` clear after the model
// assignment, each leave the whole suite green.
//
// What IS load-bearing is the RELATIVE ORDER of those two, and it becomes
// load-bearing exactly when the synchronicity above stops holding:
// `resetHistory` runs BEFORE `model` is assigned. Measured, all three
// configurations -- gate first with an `await` between gate and model: 79
// passed. Gate first with the `await` after both: 79 passed. Model first
// with an `await` between them: 3 failed (the first three cases of the New
// describe in `e2e/smoke.spec.ts`). Setting the gate first makes this
// function await-proof: at the moment `model` is written, `resetHistory`
// has already run and `sessionActive` (now derived from `savedSnapshot`) is
// already true, so the callback that write queues cannot observe anything
// else, wherever it later flushes. The reverse order breaks as soon as an
// `await` lands between the two -- the watcher runs at that microtask
// boundary, reads a still-false gate, returns early, and the seed is never
// validated. `resetHistory` takes the place of the plain `sessionActive.
// value = true` assignment Task 3 put here; it must stay in that same
// position for the reasons above, plus one of its own -- `history[0]` must
// already equal the serialized model, or the push rule sees a difference
// and appends a second entry, so a freshly created profile would start one
// step deep and dirty.
//
// Which is the case this funnel walks into: the discard guard (D109) makes
// it `async` and puts an `await` in front of the seed. The gate stays
// above the model assignment there, as required. This comment has been
// wrong about this function twice, in opposite directions -- first that
// every statement's order was load-bearing, then that none of it was and
// that the constraint was gate and model sharing one synchronous block --
// so do not reconstruct either claim from the code's shape.
//
// The seeded rule is selected (index 0) so the detail panel opens on the one
// field the warning is about, mirroring Add's own behaviour (D67, D107
// decision 9).
async function createBlank(): Promise<void> {
  if (opening.value || saving.value) {
    return;
  }
  // D109 decision 2: the same guard `pickAndOpen` uses, ahead of the seed
  // this time rather than a file dialog -- New discards exactly as much as
  // Open. This is also the `await` the comment above warns about: the gate
  // (`resetHistory`) must stay ahead of the model assignment below it,
  // which it already is.
  if (dirty.value && !(await confirmDialog.value?.ask())) {
    return;
  }
  ipcErrorCode.value = null;
  currentPath.value = null;
  diagnostics.value = [];
  const profile = blankProfile();
  resetHistory(profile);
  model.value = profile;
  selectedIndex.value = 0;
}

async function doSave(): Promise<void> {
  if (saveDisabled.value || !model.value) {
    return;
  }
  // Captured before the dialog gap: the native save dialog can stay open
  // indefinitely, and the model may change underneath it -- what gets
  // written must be the profile that was in the editor when Save was
  // clicked (same rule as the job-log export in `RunHistory`).
  const profile = model.value;
  let path = currentPath.value;
  const needsPath = path === null;
  saving.value = true;
  ipcErrorCode.value = null;
  try {
    // Tested directly rather than through `needsPath`, and the two are
    // deliberately NOT unified: `path` is reassigned just below, so the
    // alias narrows nothing and only a direct null test makes `path` a
    // `string` at `saveProfile`; the second `needsPath` further down must
    // keep asking the ORIGINAL question, since by then `path === null` is
    // false and the recents write is gated on the path being NEWLY
    // established (D107 decision 5).
    if (path === null) {
      const picked = await saveDialog({
        defaultPath: "profile.yaml",
        filters: [{ name: fluent.$t("batch-profile-filter-name"), extensions: ["yaml", "yml"] }],
      });
      if (typeof picked !== "string") {
        return;
      }
      path = picked;
    }
    await saveProfile(path, profile);
    // D108 decision 3: mark the profile that was WRITTEN -- the captured
    // `profile`, never `history.value[position.value]` and never
    // `model.value`. Two awaits sit between the `profile` capture above
    // and this point (the save dialog on the needs-path branch, and the
    // write itself), and the editing surface stays live through both
    // (`saving.value` disables only `editor-save`/`editor-new`/
    // `editor-open`, not the widgets), so `model`/history can move inside
    // either window -- marking anything read HERE could name a state that
    // was never written. The mkvtoolnix-gui precedent this shape borrows
    // (`updateConfigFromControlValues(); p.config.save(); p.savedState =
    // currentState();`) is fully synchronous, so `currentState()` there IS
    // what was written; carrying that shape across an `await` means
    // carrying the captured value forward explicitly instead of re-reading
    // "the current one" at the far end.
    savedSnapshot.value = JSON.stringify(profile);
    currentPath.value = path;
    if (needsPath) {
      // A profile that just acquired a path is exactly what the recents
      // memory is for; an already-pathed save leaves it alone, as before.
      const persisted = await rememberRecentProfile(path);
      if (persisted) {
        recents.value = persisted.recent_profiles;
      }
    }
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

// D108 decision 2: every discrete grid operation (this function, `addRule`,
// `removeSelectedRule`) clears the coalescing boundary itself, as its first
// statement, rather than relying on the `@focusout` listener (Step 2) alone
// -- two consecutive clicks of the same button never move focus, so
// `@focusout` alone would silently merge them into one undo step.
function onDrop(index: number) {
  coalesce = false;
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
  // D108 decision 2: same reasoning as `onDrop` above.
  coalesce = false;
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
  // D108 decision 2: same reasoning as `onDrop` above.
  coalesce = false;
  if (selectedIndex.value === null || !model.value) return;
  const next = [...rules.value];
  next.splice(selectedIndex.value, 1);
  model.value = {
    ...model.value,
    tracks: { ...model.value.tracks, rules: next },
  };
  selectedIndex.value = null;
}

// --- Task 4 (D108): undo, redo, and the keyboard binding -----------------

// D108 decision 10: gated on `model.value` in the function itself, not only
// in the buttons' `:disabled` -- both the action row and the keyboard
// handler below sit outside `<template v-if="model">`, so either could
// otherwise apply a history entry while the editor holds nothing. Clearing
// `selectedIndex` follows the same rule `onDrop`/`removeSelectedRule`
// already do: a selection maps to a rule identity, not a position, and the
// applied entry may not even have a rule at that index. `coalesce = false`
// bounds the residual named in the plan's own behavioural-gap note: any
// edit right after an undo/redo starts a fresh burst rather than silently
// merging into whatever burst was in progress before it.
function undo(): void {
  if (!canUndo.value || !model.value) {
    return;
  }
  position.value -= 1;
  model.value = JSON.parse(history.value[position.value]) as Profile;
  selectedIndex.value = null;
  coalesce = false;
}

function redo(): void {
  if (!canRedo.value || !model.value) {
    return;
  }
  position.value += 1;
  model.value = JSON.parse(history.value[position.value]) as Profile;
  selectedIndex.value = null;
  coalesce = false;
}

// D108 decision 6: the exact set of `<input>` types the browser itself
// treats as free-text entry, so a text-entry control keeps its native
// character-level undo while typing (model-level undo takes over once
// focus leaves it, via `@focusout` above clearing the coalescing flag).
const TEXT_ENTRY_INPUT_TYPES = new Set([
  "text",
  "search",
  "url",
  "tel",
  "password",
  "email",
  "number",
]);

function isTextEntryTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) {
    return false;
  }
  if (target.tagName === "TEXTAREA") {
    return true;
  }
  return target.tagName === "INPUT" && TEXT_ENTRY_INPUT_TYPES.has((target as HTMLInputElement).type);
}

// D108 decision 6: one binding set for all three platforms, accepting both
// modifiers -- no per-OS branch, at the stated cost that the shortcut needs
// focus inside the editor (the visible Undo/Redo buttons, Step 4, cover
// that). A document-level listener is deliberately not used: the root
// section's own `@keydown` cannot fire while another view is active and
// needs no lifecycle teardown.
function onEditorKeydown(event: KeyboardEvent): void {
  if (isTextEntryTarget(event.target)) {
    return;
  }
  const mod = event.ctrlKey || event.metaKey;
  if (!mod) {
    return;
  }
  const key = event.key.toLowerCase();
  if (key === "z" && !event.shiftKey) {
    event.preventDefault();
    undo();
  } else if ((key === "z" && event.shiftKey) || key === "y") {
    event.preventDefault();
    redo();
  }
}
</script>

<template>
  <!-- D108 decisions 2 and 6: `@focusout` (not `@blur`) is the coalescing
       boundary because it bubbles to this ancestor, measured, where `blur`
       does not; `@keydown` is the undo/redo keyboard channel -- see
       `onEditorKeydown`'s own doc for the no-per-OS-branch reasoning and
       the text-entry exemption. -->
  <section
    data-testid="view-editor"
    data-help-id="view-editor"
    @focusout="coalesce = false"
    @keydown="onEditorKeydown"
  >
    <!-- D109 decision 6: the one confirm surface both discard guards ask
         (`pickAndOpen`, `createBlank`). -->
    <ConfirmDialog
      ref="confirmDialog"
      :title="$t('editor-discard-title')"
      :message="$t('editor-discard-message')"
      :confirm-label="$t('editor-discard-confirm')"
      :cancel-label="$t('settings-cancel')"
    />
    <button
      type="button"
      data-testid="editor-new"
      :disabled="opening || saving"
      @click="createBlank"
    >
      {{ $t("editor-action-new") }}
    </button>
    <button
      type="button"
      data-testid="editor-open"
      :disabled="opening || saving"
      :aria-busy="opening"
      @click="pickAndOpen"
    >
      {{ $t("batch-profile-pick") }}
    </button>
    <button
      type="button"
      data-testid="editor-undo"
      :disabled="!model || !canUndo"
      @click="undo"
    >
      {{ $t("editor-action-undo") }}
    </button>
    <button
      type="button"
      data-testid="editor-redo"
      :disabled="!model || !canRedo"
      @click="redo"
    >
      {{ $t("editor-action-redo") }}
    </button>
    <p v-if="currentPath">
      {{ $t("batch-profile-current", { path: currentPath }) }}
    </p>
    <p
      v-else-if="sessionActive"
      data-testid="editor-unsaved"
    >
      {{ $t("editor-unsaved") }}
    </p>
    <p
      v-if="nothingOpenedOrCreated"
      data-testid="editor-empty"
    >
      {{ $t("editor-empty") }}
    </p>

    <section
      v-if="nothingOpenedOrCreated && recents.length"
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

    <section
      v-if="diagnostics.length"
      aria-labelledby="editor-diagnostics-heading"
    >
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
