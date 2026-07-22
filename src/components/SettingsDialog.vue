<script setup lang="ts">
import { reactive, ref, useTemplateRef } from "vue";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { defaultAppSettings, getSettings, setSettings } from "../ipc";
import type { AppSettings, IpcError } from "../ipc";
import { applyLocale } from "../i18n/fluent";

// D27 settings dialog: native <dialog>, opened imperatively via
// `defineExpose`'s `open()` (App.vue holds a template ref and calls it
// from the shell's own Settings button). Deliberately not `method="dialog"`
// on the <form>: saving needs an async IPC round trip to complete (and the
// dialog to stay open on failure), which a plain `method="dialog"` submit
// cannot express -- it closes synchronously regardless of outcome.
const dialogEl = useTemplateRef("dialogEl");
const busy = ref(false);
const errorCode = ref<string | null>(null);
const errorParams = ref<Record<string, string | number>>({});

const form = reactive({
  mkvmergePath: "",
  defaultJobs: 1,
  locale: "en",
});

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
    // Live locale switch (D56): swap the catalog in place when the locale
    // actually changed, before `baseline` is reassigned below. Views are
    // v-show-mounted, so no state is lost. The `!== null` narrows
    // AppSettings.locale (string | null) to the string applyLocale takes;
    // next.locale is always set here (from form.locale), so the guard only
    // satisfies the type -- vue-tsc is the sole gate that catches it.
    if (next.locale !== baseline.locale && next.locale !== null) {
      applyLocale(next.locale);
    }
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

defineExpose({ open, isOpen: () => dialogEl.value?.open ?? false });
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
          {{ $ta("settings-mkvmerge-path-label").hint }}
        </p>
        <button
          type="button"
          :title="$ta('browse-button').tooltip"
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
          {{ $ta("settings-default-jobs-label").hint }}
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
          {{ $ta("settings-locale-label").hint }}
        </p>
      </div>

      <button
        type="submit"
        :disabled="busy"
        :aria-busy="busy"
        :title="$ta('settings-save').tooltip"
      >
        {{ $t("settings-save") }}
      </button>
      <button
        type="button"
        :disabled="busy"
        :title="$ta('settings-cancel').tooltip"
        @click="close"
      >
        {{ $t("settings-cancel") }}
      </button>
    </form>
  </dialog>
</template>
