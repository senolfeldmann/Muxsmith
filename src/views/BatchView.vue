<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useFluent } from "fluent-vue";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import DiagnosticsPanel from "../components/DiagnosticsPanel.vue";
import ResolutionTable from "../components/ResolutionTable.vue";
import SuggestionCard from "../components/SuggestionCard.vue";
import {
  applySuggestion,
  defaultAppSettings,
  dryRun,
  getSettings,
  loadProfile,
  saveProfile,
  setSettings,
  validateProfile,
} from "../ipc";
import type {
  AppSettings,
  Diagnostic,
  DirMemory,
  IpcError,
  ReportDocument,
  RunRequest,
} from "../ipc";
import type { StructuredEdit } from "../bindings/profile";

// Task 10 (spec 8.2 view 2, D22): profile pick + recents, source/output
// directory memory, dry-run report rendering, suggestions as show+copy
// cards, and the `start-run` handoff to App/JobsView (Plan 5 wave-5
// contract). Through Plan 5, no profile mutation anywhere in this view.
// Task 14 (D43, D49) adds exactly one: `onApplySuggestion` below, the
// narrow load/apply/save round trip behind a `SuggestionCard`'s `apply`
// emit -- everything else in this view (pick, dirs, dry-run, run handoff)
// stays read-only. Full profile editing remains Plan 6's editor.

const fluent = useFluent();

const settings = ref<AppSettings>(defaultAppSettings());
const selectedProfile = ref<string | null>(null);
const sourceDir = ref("");
const outputDir = ref("");
const report = ref<ReportDocument | null>(null);
const validating = ref(false);
const dryRunning = ref(false);
const ipcErrorCode = ref<string | null>(null);
const ipcErrorParams = ref<Record<string, string>>({});
// Task 14 (D43, D49, apply-wiring routing): which `report.suggestions`
// index is mid-round-trip, or `null` when none is -- drives the clicked
// card's own `aria-busy` (not a single shared boolean) and, folded into
// `busy` below, gates every other action while an apply is in flight the
// same way `validating`/`dryRunning` already do.
const applyingIndex = ref<number | null>(null);

const busy = computed(() => validating.value || dryRunning.value || applyingIndex.value !== null);

// Fix (D23): "the UI additionally disables Run while active" -- App
// forwards JobsView's own runActive state here (this view has no other
// way to know a run is active; App is the sole broker between the two
// views). Optional/defaulted so this view still type-checks and behaves
// correctly standalone, matching JobsView's own `pendingRun?:` precedent.
const props = withDefaults(defineProps<{ runActive?: boolean }>(), { runActive: false });

const emit = defineEmits<{ "start-run": [payload: RunRequest] }>();

onMounted(async () => {
  try {
    settings.value = await getSettings();
  } catch {
    // Non-fatal, mirrors FirstRun's identical tolerance (T9): recents and
    // directory memory just start empty, the view stays fully usable.
  }
});

/** Re-fetches settings immediately before every write and spreads the
 * result (mirrors SettingsDialog's baseline pattern): this view owns
 * `recent_profiles`/`dir_memory` but must never clobber
 * `mkvmerge_path`/`default_jobs`/`locale`, which SettingsDialog owns. */
async function updateSettings(mutate: (current: AppSettings) => AppSettings): Promise<void> {
  const current = await getSettings();
  const next = mutate(current);
  await setSettings(next);
  settings.value = next;
}

/** Mirrors `src-tauri/src/settings.rs::RECENT_PROFILES_CAP` (D27). The
 * Rust side truncates only inside `save()`, so without this client-side
 * cap the *rendered* MRU list would grow past the limit within one
 * session (self-healing only on restart); truncating in the mutation
 * keeps `settings.value` identical to what was actually persisted. */
const RECENT_PROFILES_CAP = 10;

async function rememberRecentProfile(path: string): Promise<void> {
  try {
    await updateSettings((current) => ({
      ...current,
      recent_profiles: [path, ...current.recent_profiles.filter((p) => p !== path)].slice(
        0,
        RECENT_PROFILES_CAP,
      ),
    }));
  } catch (e) {
    // Background bookkeeping only; a failed recents write never blocks
    // picking or validating the profile itself.
    console.warn("[batch] failed to persist recent profile:", e);
  }
}

async function persistDir(kind: "source" | "output", value: string): Promise<void> {
  if (!selectedProfile.value) {
    return;
  }
  const profile = selectedProfile.value;
  const trimmed = value.trim();
  try {
    await updateSettings((current) => {
      const existing: DirMemory = { ...current.dir_memory[profile] };
      if (trimmed === "") {
        delete existing[kind];
      } else {
        existing[kind] = trimmed;
      }
      return { ...current, dir_memory: { ...current.dir_memory, [profile]: existing } };
    });
  } catch (e) {
    console.warn("[batch] failed to persist directory memory:", e);
  }
}

async function runValidate(): Promise<void> {
  if (!selectedProfile.value) {
    return;
  }
  validating.value = true;
  ipcErrorCode.value = null;
  try {
    report.value = await validateProfile(selectedProfile.value);
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    validating.value = false;
  }
}

async function selectProfile(path: string): Promise<void> {
  if (busy.value) {
    return;
  }
  selectedProfile.value = path;
  report.value = null;
  ipcErrorCode.value = null;
  const memory = settings.value.dir_memory[path];
  sourceDir.value = memory?.source ?? "";
  outputDir.value = memory?.output ?? "";
  await rememberRecentProfile(path);
  await runValidate();
}

async function pickProfile(): Promise<void> {
  if (busy.value) {
    return;
  }
  const picked = await openDialog({
    multiple: false,
    directory: false,
    filters: [{ name: fluent.$t("batch-profile-filter-name"), extensions: ["yaml", "yml"] }],
  });
  if (typeof picked === "string") {
    await selectProfile(picked);
  }
}

async function pickDir(kind: "source" | "output"): Promise<void> {
  if (busy.value) {
    return;
  }
  const picked = await openDialog({ multiple: false, directory: true });
  if (typeof picked !== "string") {
    return;
  }
  if (kind === "source") {
    sourceDir.value = picked;
  } else {
    outputDir.value = picked;
  }
  await persistDir(kind, picked);
}

function onDirInputChange(kind: "source" | "output"): void {
  void persistDir(kind, kind === "source" ? sourceDir.value : outputDir.value);
}

async function runDryRun(): Promise<void> {
  if (!selectedProfile.value || busy.value) {
    return;
  }
  dryRunning.value = true;
  ipcErrorCode.value = null;
  try {
    report.value = await dryRun(
      selectedProfile.value,
      sourceDir.value.trim() === "" ? undefined : sourceDir.value.trim(),
      outputDir.value.trim() === "" ? undefined : outputDir.value.trim(),
    );
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    dryRunning.value = false;
  }
}

/** Task 14 (D43, D49, apply-wiring routing): handles a `SuggestionCard`'s
 * `apply` emit. `payload.config_path` is a config-field LOCATOR
 * (`tracks[<N>].match`, parsed core-side by `rule_index_of`), never a
 * file path -- it and `payload.edit` are forwarded to `apply_suggestion`
 * exactly as received, never parsed or interpreted here. The load/save
 * path is always `selectedProfile`, the profile FILE this view already
 * has open; apply does not validate or re-plan (D43: no compound
 * apply-and-validate command), and this view does not auto-refresh the
 * report afterwards (design D43's post-apply validation is the editor's
 * `validate_profile_model` round trip, which this view does not have;
 * `core-03`'s guarantee is that the applied edit survives the *next*
 * dry run, not that apply triggers one -- auto-refresh is a deferred,
 * controller-routed ROADMAP candidate, not built here). */
async function onApplySuggestion(
  payload: { config_path: string; edit: unknown },
  index: number,
): Promise<void> {
  if (!selectedProfile.value || busy.value) {
    return;
  }
  applyingIndex.value = index;
  ipcErrorCode.value = null;
  try {
    const doc = await loadProfile(selectedProfile.value);
    if (!doc.profile) {
      // A `ParseError` since the suggestion was computed: `load_profile`
      // folds it into a single explanatory diagnostic rather than
      // throwing (D42's own doc on `load_profile`'s envelope), so that
      // diagnostic's own code/params are the correct thing to surface
      // through this view's existing shared alert line -- reusing it
      // needs no bespoke fallback code.
      const parseDiagnostic = doc.config_diagnostics[0];
      if (parseDiagnostic) {
        ipcErrorCode.value = parseDiagnostic.code;
        ipcErrorParams.value = parseDiagnostic.params;
      } else {
        // Contract violation (D42's `load_profile` envelope): `profile:
        // null` is documented to always pair with a lead diagnostic
        // explaining why. An empty `config_diagnostics` here means core
        // broke that contract -- there is no diagnostic to surface through
        // the shared alert line, so at minimum this stops being a silent
        // no-op.
        console.error(
          "[batch] load_profile returned profile: null with no diagnostics",
          selectedProfile.value,
        );
      }
      return;
    }
    const updated = await applySuggestion(
      doc.profile,
      payload.config_path,
      payload.edit as StructuredEdit,
    );
    await saveProfile(selectedProfile.value, updated);
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    applyingIndex.value = null;
  }
}

// `report.value` covers both shapes T7 returns: `validate_profile`'s
// config-only document (files/batch_diagnostics/suggestions always empty)
// and `dry_run`'s full `batch_document`. Grouping config+batch-level
// diagnostics here (as opposed to per-file, rendered inside each
// ResolutionTable) covers cross-file/runtime facts like DuplicateIdentifier
// that the brief's "config + per-file" wording does not name explicitly
// but that can carry error severity -- hiding them here while still
// counting them in the summary/Run-gate below would make an error block
// Run with no visible explanation.
const generalDiagnostics = computed<Diagnostic[]>(() =>
  report.value ? [...report.value.config_diagnostics, ...report.value.batch_diagnostics] : [],
);
const allDiagnostics = computed<Diagnostic[]>(() =>
  report.value
    ? [...generalDiagnostics.value, ...report.value.files.flatMap((f) => f.diagnostics)]
    : [],
);
const diagnosticCounts = computed(() => {
  const counts = { error: 0, warning: 0, info: 0 };
  for (const d of allDiagnostics.value) {
    counts[d.severity] += 1;
  }
  return counts;
});
const hasErrors = computed(() => diagnosticCounts.value.error > 0);

/** The Fluent key explaining why Run is disabled, or `null` when it isn't.
 * The two conditions the T10 brief names (errors, mkvmerge missing), the
 * functional precondition of having a validated report to run at all --
 * `start_run` re-plans and dry-runs internally regardless (spec 5.5), so
 * this view never requires an explicit dry-run click first -- plus the D23
 * fix: a run already active elsewhere in the app (Jobs view, via
 * `runActive`) checked first, since it overrides every other reason. */
const runDisabledReason = computed<string | null>(() => {
  if (props.runActive) {
    return "batch-run-tooltip-run-active";
  }
  if (!selectedProfile.value || !report.value) {
    return "batch-run-tooltip-no-profile";
  }
  if (report.value.mkvmerge_found === false) {
    return "batch-run-tooltip-mkvmerge-missing";
  }
  if (hasErrors.value) {
    return "batch-run-tooltip-errors";
  }
  return null;
});
const runDisabled = computed(() => runDisabledReason.value !== null || busy.value);
const runTooltip = computed(() => fluent.$t(runDisabledReason.value ?? "batch-run-tooltip"));

function emitStartRun(): void {
  if (runDisabled.value || !selectedProfile.value) {
    return;
  }
  emit("start-run", {
    profile: selectedProfile.value,
    source: sourceDir.value.trim() === "" ? null : sourceDir.value.trim(),
    output: outputDir.value.trim() === "" ? null : outputDir.value.trim(),
    // No jobs-count control in this view (outside the T10 brief's scope);
    // pass the app's own configured default rather than `null` so Run
    // does not silently downgrade to sequential -- `start_run`'s own
    // Rust-side default for `jobs: None` is 1, independent of
    // AppSettings.default_jobs (src-tauri/src/run.rs).
    jobs: settings.value.default_jobs,
  });
}
</script>

<template>
  <section data-testid="view-batch">
    <h2>{{ $t("batch-view-heading") }}</h2>

    <section
      aria-labelledby="batch-profile-heading"
      :aria-busy="validating"
    >
      <h3 id="batch-profile-heading">
        {{ $t("batch-profile-heading") }}
      </h3>
      <p v-if="!selectedProfile">
        {{ $t("batch-profile-none") }}
      </p>
      <p v-else>
        {{ $t("batch-profile-current", { path: selectedProfile }) }}
      </p>
      <button
        type="button"
        data-testid="batch-profile-pick"
        :disabled="busy"
        :title="$t('batch-profile-pick-tooltip')"
        @click="pickProfile"
      >
        {{ $t("batch-profile-pick") }}
      </button>

      <h4 id="batch-recents-heading">
        {{ $t("batch-recents-heading") }}
      </h4>
      <ul
        v-if="settings.recent_profiles.length"
        aria-labelledby="batch-recents-heading"
      >
        <li
          v-for="(path, i) in settings.recent_profiles"
          :key="path"
        >
          <button
            type="button"
            data-testid="batch-recent-profile"
            :data-index="i"
            :disabled="busy"
            :title="$t('batch-recents-select-tooltip')"
            @click="selectProfile(path)"
          >
            {{ path }}
          </button>
        </li>
      </ul>
      <p v-else>
        {{ $t("batch-recents-empty") }}
      </p>
    </section>

    <section aria-labelledby="batch-dirs-heading">
      <h3 id="batch-dirs-heading">
        {{ $t("batch-dirs-heading") }}
      </h3>
      <div>
        <label for="batch-source-dir">{{ $t("batch-source-label") }}</label>
        <input
          id="batch-source-dir"
          v-model="sourceDir"
          type="text"
          aria-describedby="batch-source-dir-hint"
          :disabled="busy"
          @change="onDirInputChange('source')"
        >
        <p id="batch-source-dir-hint">
          {{ $t("batch-source-hint") }}
        </p>
        <button
          type="button"
          data-testid="batch-source-browse"
          :disabled="busy"
          :title="$t('batch-browse-dir-tooltip')"
          @click="pickDir('source')"
        >
          {{ $t("browse-button") }}
        </button>
      </div>
      <div>
        <label for="batch-output-dir">{{ $t("batch-output-label") }}</label>
        <input
          id="batch-output-dir"
          v-model="outputDir"
          type="text"
          aria-describedby="batch-output-dir-hint"
          :disabled="busy"
          @change="onDirInputChange('output')"
        >
        <p id="batch-output-dir-hint">
          {{ $t("batch-output-hint") }}
        </p>
        <button
          type="button"
          data-testid="batch-output-browse"
          :disabled="busy"
          :title="$t('batch-browse-dir-tooltip')"
          @click="pickDir('output')"
        >
          {{ $t("browse-button") }}
        </button>
      </div>
    </section>

    <p
      v-if="ipcErrorCode"
      role="alert"
    >
      {{ $t(ipcErrorCode, ipcErrorParams) }}
    </p>

    <button
      type="button"
      data-testid="batch-dry-run"
      :disabled="!selectedProfile || busy"
      :aria-busy="dryRunning"
      :title="$t('batch-dry-run-tooltip')"
      @click="runDryRun"
    >
      {{ $t("batch-dry-run") }}
    </button>

    <template v-if="report">
      <section aria-labelledby="batch-diagnostics-heading">
        <h3 id="batch-diagnostics-heading">
          {{ $t("batch-diagnostics-heading") }}
        </h3>
        <p role="status">
          {{
            $t("batch-diagnostics-summary", {
              errors: diagnosticCounts.error,
              warnings: diagnosticCounts.warning,
              infos: diagnosticCounts.info,
            })
          }}
        </p>
        <DiagnosticsPanel :diagnostics="generalDiagnostics" />
      </section>

      <section
        v-if="report.files.length"
        aria-labelledby="batch-files-heading"
      >
        <h3 id="batch-files-heading">
          {{ $t("batch-files-heading") }}
        </h3>
        <ResolutionTable
          v-for="f in report.files"
          :key="f.source"
          :file="f"
        />
      </section>

      <section
        v-if="report.suggestions.length"
        aria-labelledby="batch-suggestions-heading"
      >
        <h3 id="batch-suggestions-heading">
          {{ $t("batch-suggestions-heading") }}
        </h3>
        <SuggestionCard
          v-for="(s, i) in report.suggestions"
          :key="i"
          :data-index="i"
          :suggestion="s"
          :applying="applyingIndex === i"
          :busy="busy"
          @apply="onApplySuggestion($event, i)"
        />
      </section>
    </template>

    <button
      type="button"
      data-testid="batch-run"
      :disabled="runDisabled"
      :aria-busy="busy"
      :title="runTooltip"
      @click="emitStartRun"
    >
      {{ $t("batch-run") }}
    </button>
  </section>
</template>
