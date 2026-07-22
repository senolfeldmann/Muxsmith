<script setup lang="ts">
import { onMounted, ref, useTemplateRef } from "vue";
import BatchView from "./views/BatchView.vue";
import JobsView from "./views/JobsView.vue";
import EditorView from "./views/EditorView.vue";
import FirstRun from "./views/FirstRun.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import { detectMkvmerge } from "./ipc";
import type { IpcError, RunRequest } from "./ipc";

type View = "batch" | "jobs" | "editor";

// Startup gate (D28, T9 brief): probe mkvmerge exactly once on mount. A
// clean result unlocks the shell; a missing/too-old one mounts FirstRun
// instead, which owns its own retry loop and clears `blockedError` itself
// once a fresh detect succeeds (see its `resolved` emit) -- checkMkvmerge
// never runs a second time.
const checking = ref(true);
const blockedError = ref<IpcError | null>(null);
const activeView = ref<View>("batch");
const settingsDialog = useTemplateRef("settingsDialog");

// Plan 5 wave-5 shell contract (T10 brief): BatchView's `start-run` emit
// hands over the picked profile/dirs/jobs; App just stores it and switches
// to Jobs, which owns actually calling `startRun` (T11) and clears this
// via `consumed` once it has (`pendingRun` is a one-shot handoff, not
// shared state either view reads back).
const pendingRun = ref<RunRequest | null>(null);

// Fix (D23): forwarded from JobsView's `update:runActive` emit so
// BatchView's Run gate can disable Run while a run is active ("the UI
// additionally disables Run while active", D23's own sentence) -- App is
// the natural owner since it already brokers pendingRun between the two
// views and neither view can see the other directly.
const jobsRunActive = ref(false);

function onStartRun(request: RunRequest) {
  pendingRun.value = request;
  activeView.value = "jobs";
}

async function checkMkvmerge() {
  try {
    await detectMkvmerge();
    blockedError.value = null;
  } catch (e) {
    blockedError.value = e as IpcError;
  } finally {
    checking.value = false;
  }
}

onMounted(checkMkvmerge);
</script>

<template>
  <p
    v-if="checking"
    aria-live="polite"
  >
    {{ $t("firstrun-detecting") }}
  </p>
  <FirstRun
    v-else-if="blockedError"
    :error="blockedError"
    @resolved="blockedError = null"
  />
  <template v-else>
    <header>
      <h1>{{ $t("app-title") }}</h1>
    </header>
    <nav :aria-label="$t('nav-label')">
      <button
        type="button"
        data-testid="nav-batch"
        :aria-current="activeView === 'batch' ? 'page' : undefined"
        @click="activeView = 'batch'"
      >
        {{ $t("nav-batch") }}
      </button>
      <button
        type="button"
        data-testid="nav-jobs"
        :aria-current="activeView === 'jobs' ? 'page' : undefined"
        @click="activeView = 'jobs'"
      >
        {{ $t("nav-jobs") }}
      </button>
      <!-- Post-close fix (owner ruling 2026-07-17, plan-6 surface pass):
           dedicated `nav-editor` key ("Editor"/"Editor"). Task 13's
           original reuse of `batch-profile-heading` ("Profile"/"Profil")
           sat beside the other two tabs' activity labels ("Batch"/"Jobs")
           while naming an object instead, and doubled a string already on
           screen as BatchView's own profile-picking section heading (see
           `gui-common.ftl`'s `nav-*` comment for the fuller rationale).
           `batch-profile-heading` itself is untouched -- it still captions
           BatchView's own heading, per `EditorView.vue`'s doc comment on
           the editor's other, still-current key reuses. -->
      <button
        type="button"
        data-testid="nav-editor"
        :aria-current="activeView === 'editor' ? 'page' : undefined"
        @click="activeView = 'editor'"
      >
        {{ $t("nav-editor") }}
      </button>
      <button
        type="button"
        data-testid="open-settings"
        :title="$ta('settings-open-label').tooltip"
        @click="settingsDialog?.open()"
      >
        {{ $t("settings-open-label") }}
      </button>
    </nav>
    <main>
      <!-- v-show, not v-if: all three views stay mounted across tab
           switches, so JobsView's live run listeners (registered in its
           onMounted, torn down in onUnmounted) survive navigating away
           mid-run, and EditorView's open profile/diagnostics/currentPath
           state (Task 13) survives a switch to Jobs and back. The hidden
           view is display:none -- out of the a11y tree, cannot trap focus.
           Only the first-run gate above (v-if/v-else-if) unmounts the
           shell; eager-mounting all three views at startup costs nothing
           at this scale. -->
      <BatchView
        v-show="activeView === 'batch'"
        :run-active="jobsRunActive"
        @start-run="onStartRun"
      />
      <JobsView
        v-show="activeView === 'jobs'"
        v-model:run-active="jobsRunActive"
        :pending-run="pendingRun"
        @consumed="pendingRun = null"
      />
      <EditorView v-show="activeView === 'editor'" />
    </main>
    <SettingsDialog ref="settingsDialog" />
  </template>
</template>
