<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { platform } from "@tauri-apps/plugin-os";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { detectMkvmerge, getSettings, setSettings } from "../ipc";
import type { IpcError } from "../ipc";

// D28: mkvmerge missing/too-old full-screen guidance. `error` is the
// failure App.vue's own startup probe hit; this component owns every
// retry after that (re-detect, and re-detect-after-setting-a-path) and
// only reports back up once ITS OWN detect succeeds (`resolved`), so the
// parent never needs to re-probe redundantly.
const props = defineProps<{ error: IpcError }>();
const emit = defineEmits<{ resolved: [] }>();

const currentError = ref<IpcError>(props.error);
const manualPath = ref("");
const busy = ref(false);

function guidanceKeyFor(p: string): string {
  switch (p) {
    case "windows":
      return "firstrun-guidance-windows";
    case "macos":
      return "firstrun-guidance-macos";
    case "linux":
      return "firstrun-guidance-linux";
    default:
      return "firstrun-guidance-fallback";
  }
}

// platform() is fixed at compile time (Tauri's own doc), so this needs no
// reactivity of its own.
const guidanceKey = guidanceKeyFor(platform());

const headingKey = computed(() => {
  switch (currentError.value.code) {
    case "mkvmerge-not-found":
      return "firstrun-missing-heading";
    case "mkvmerge-too-old":
      return "firstrun-too-old-heading";
    default:
      return "firstrun-detect-failed-heading";
  }
});

onMounted(async () => {
  try {
    const settings = await getSettings();
    manualPath.value = settings.mkvmerge_path ?? "";
  } catch {
    // Settings unreadable: the picker just starts empty. This screen's
    // job is mkvmerge detection, not settings diagnostics.
  }
});

async function browse() {
  const picked = await openDialog({ multiple: false, directory: false });
  if (typeof picked === "string") {
    manualPath.value = picked;
  }
}

/** `save`: write `manualPath` as the settings override, then re-detect
 * (D28's "manual path picker ... writing set_settings then re-detect").
 * `!save` ("Retry detection"): re-detect only, against whatever override
 * (if any) is already persisted -- covers the "I just installed
 * mkvtoolnix into a standard location" case with no path to set. */
async function attempt(save: boolean) {
  busy.value = true;
  try {
    if (save) {
      const settings = await getSettings();
      await setSettings({
        ...settings,
        mkvmerge_path: manualPath.value.trim() === "" ? null : manualPath.value.trim(),
      });
    }
    await detectMkvmerge();
    emit("resolved");
  } catch (e) {
    currentError.value = e as IpcError;
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <main data-testid="first-run">
    <div aria-live="polite">
      <h1>{{ $t(headingKey) }}</h1>
      <p>{{ $t(currentError.code, currentError.params) }}</p>
    </div>

    <p>{{ $t(guidanceKey) }}</p>

    <form @submit.prevent="attempt(true)">
      <div>
        <label for="firstrun-path">{{ $t("firstrun-picker-label") }}</label>
        <input
          id="firstrun-path"
          v-model="manualPath"
          type="text"
          aria-describedby="firstrun-path-hint"
        >
        <p id="firstrun-path-hint">
          {{ $t("firstrun-picker-hint") }}
        </p>
        <button
          type="button"
          :title="$t('browse-button-tooltip')"
          @click="browse"
        >
          {{ $t("browse-button") }}
        </button>
      </div>

      <button
        type="submit"
        :disabled="busy || manualPath.trim() === ''"
        :aria-busy="busy"
        :title="$t('firstrun-use-path-tooltip')"
      >
        {{ $t("firstrun-use-path") }}
      </button>
      <button
        type="button"
        :disabled="busy"
        :aria-busy="busy"
        :title="$t('firstrun-retry-tooltip')"
        @click="attempt(false)"
      >
        {{ $t("firstrun-retry") }}
      </button>
    </form>
  </main>
</template>
