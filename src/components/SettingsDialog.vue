<script setup lang="ts">
import { reactive, ref } from "vue";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { getSettings, setSettings } from "../ipc";
import type { AppSettings, IpcError } from "../ipc";

// D27 settings dialog: native <dialog>, opened imperatively via
// `defineExpose`'s `open()` (App.vue holds a template ref and calls it
// from the shell's own Settings button). Deliberately not `method="dialog"`
// on the <form>: saving needs an async IPC round trip to complete (and the
// dialog to stay open on failure), which a plain `method="dialog"` submit
// cannot express -- it closes synchronously regardless of outcome.
const dialogEl = ref<HTMLDialogElement | null>(null);
const busy = ref(false);
const errorCode = ref<string | null>(null);
const errorParams = ref<Record<string, string>>({});

const form = reactive({
  mkvmergePath: "",
  defaultJobs: 1,
  locale: "en",
});

function defaultAppSettings(): AppSettings {
  return {
    mkvmerge_path: null,
    default_jobs: 1,
    locale: null,
    recent_profiles: [],
    dir_memory: {},
  };
}

let baseline: AppSettings = defaultAppSettings();

async function open() {
  errorCode.value = null;
  try {
    baseline = await getSettings();
  } catch (e) {
    const err = e as IpcError;
    errorCode.value = err.code;
    errorParams.value = err.params;
    baseline = defaultAppSettings();
  }
  form.mkvmergePath = baseline.mkvmerge_path ?? "";
  form.defaultJobs = baseline.default_jobs;
  form.locale = baseline.locale ?? "en";
  dialogEl.value?.showModal();
}

function close() {
  dialogEl.value?.close();
}

async function browse() {
  const picked = await openDialog({ multiple: false, directory: false });
  if (typeof picked === "string") {
    form.mkvmergePath = picked;
  }
}

async function save() {
  busy.value = true;
  errorCode.value = null;
  try {
    const next: AppSettings = {
      ...baseline,
      mkvmerge_path: form.mkvmergePath.trim() === "" ? null : form.mkvmergePath.trim(),
      default_jobs: Math.max(1, Math.trunc(form.defaultJobs) || 1),
      locale: form.locale,
    };
    await setSettings(next);
    baseline = next;
    dialogEl.value?.close();
  } catch (e) {
    const err = e as IpcError;
    errorCode.value = err.code;
    errorParams.value = err.params;
  } finally {
    busy.value = false;
  }
}

defineExpose({ open });
</script>

<template>
  <dialog
    ref="dialogEl"
    data-testid="settings-dialog"
    aria-labelledby="settings-title"
  >
    <form @submit.prevent="save">
      <h2 id="settings-title">
        {{ $t("settings-title") }}
      </h2>

      <p
        v-if="errorCode"
        role="alert"
      >
        {{ $t(errorCode, errorParams) }}
      </p>

      <div>
        <label for="settings-mkvmerge-path">{{ $t("settings-mkvmerge-path-label") }}</label>
        <input
          id="settings-mkvmerge-path"
          v-model="form.mkvmergePath"
          type="text"
          aria-describedby="settings-mkvmerge-path-hint"
        >
        <p id="settings-mkvmerge-path-hint">
          {{ $t("settings-mkvmerge-path-hint") }}
        </p>
        <button
          type="button"
          :title="$t('browse-button-tooltip')"
          @click="browse"
        >
          {{ $t("browse-button") }}
        </button>
      </div>

      <div>
        <label for="settings-default-jobs">{{ $t("settings-default-jobs-label") }}</label>
        <input
          id="settings-default-jobs"
          v-model.number="form.defaultJobs"
          type="number"
          min="1"
          step="1"
          aria-describedby="settings-default-jobs-hint"
        >
        <p id="settings-default-jobs-hint">
          {{ $t("settings-default-jobs-hint") }}
        </p>
      </div>

      <div>
        <label for="settings-locale">{{ $t("settings-locale-label") }}</label>
        <select
          id="settings-locale"
          v-model="form.locale"
          aria-describedby="settings-locale-hint"
        >
          <option value="en">
            {{ $t("settings-locale-option-en") }}
          </option>
          <option value="de">
            {{ $t("settings-locale-option-de") }}
          </option>
        </select>
        <p id="settings-locale-hint">
          {{ $t("settings-locale-hint") }}
        </p>
      </div>

      <button
        type="submit"
        :disabled="busy"
        :aria-busy="busy"
        :title="$t('settings-save-tooltip')"
      >
        {{ $t("settings-save") }}
      </button>
      <button
        type="button"
        :disabled="busy"
        :title="$t('settings-cancel-tooltip')"
        @click="close"
      >
        {{ $t("settings-cancel") }}
      </button>
    </form>
  </dialog>
</template>
