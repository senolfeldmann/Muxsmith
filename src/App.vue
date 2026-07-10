<script setup lang="ts">
import { onMounted, ref } from "vue";
import BatchView from "./views/BatchView.vue";
import JobsView from "./views/JobsView.vue";
import FirstRun from "./views/FirstRun.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import { detectMkvmerge } from "./ipc";
import type { IpcError } from "./ipc";

type View = "batch" | "jobs";

// Startup gate (D28, T9 brief): probe mkvmerge exactly once on mount. A
// clean result unlocks the shell; a missing/too-old one mounts FirstRun
// instead, which owns its own retry loop and clears `blockedError` itself
// once a fresh detect succeeds (see its `resolved` emit) -- checkMkvmerge
// never runs a second time.
const checking = ref(true);
const blockedError = ref<IpcError | null>(null);
const activeView = ref<View>("batch");
const settingsDialog = ref<InstanceType<typeof SettingsDialog> | null>(null);

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
      <button
        type="button"
        data-testid="open-settings"
        :title="$t('settings-open-tooltip')"
        @click="settingsDialog?.open()"
      >
        {{ $t("settings-open-label") }}
      </button>
    </nav>
    <main>
      <BatchView v-if="activeView === 'batch'" />
      <JobsView v-else />
    </main>
    <SettingsDialog ref="settingsDialog" />
  </template>
</template>
