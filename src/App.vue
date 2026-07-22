<script setup lang="ts">
import { computed, onMounted, ref, useTemplateRef, watch } from "vue";
import BatchView from "./views/BatchView.vue";
import JobsView from "./views/JobsView.vue";
import EditorView from "./views/EditorView.vue";
import FirstRun from "./views/FirstRun.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import HelpSidebar from "./components/HelpSidebar.vue";
import { helpMode, hoverId, pinnedId, VIEW_TOPICS } from "./help/state";
import { detectMkvmerge } from "./ipc";
import type { IpcError, RunRequest } from "./ipc";
import "./style.css";

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

// Help mode (D52): app-shell state lives in ./help/state; App owns the
// delegation, the topic resolution, and the highlight bookkeeping.
const mainEl = useTemplateRef("mainEl");

const helpTopicId = computed(
  () => pinnedId.value ?? hoverId.value ?? VIEW_TOPICS[activeView.value],
);

function helpTarget(event: Event): string | null {
  const el = (event.target as Element | null)?.closest?.("[data-help-id]") ?? null;
  return el?.getAttribute("data-help-id") ?? null;
}
function onHelpHover(event: Event) {
  hoverId.value = helpTarget(event);
}
function onHelpClick(event: Event) {
  // E3 ruling: ALL activation inside <main> is suppressed in help mode;
  // an annotated target pins its topic instead of activating.
  event.preventDefault();
  event.stopPropagation();
  const id = helpTarget(event);
  if (id !== null) {
    pinnedId.value = id;
  }
}
function onHelpKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    // The native modal's own cancel semantics win while the dialog is
    // open (it sits in the top layer above help mode anyway).
    if (settingsDialog.value?.isOpen()) {
      return;
    }
    helpMode.value = false;
    return;
  }
  if ((event.key === "Enter" || event.key === " ") && helpTarget(event) !== null) {
    event.preventDefault();
    event.stopPropagation();
    pinnedId.value = helpTarget(event);
  }
}

watch(helpMode, (on) => {
  const main = mainEl.value;
  if (!main) return;
  if (on) {
    main.addEventListener("mouseover", onHelpHover, true);
    main.addEventListener("focusin", onHelpHover, true);
    main.addEventListener("click", onHelpClick, true);
    document.addEventListener("keydown", onHelpKeydown, true);
  } else {
    main.removeEventListener("mouseover", onHelpHover, true);
    main.removeEventListener("focusin", onHelpHover, true);
    main.removeEventListener("click", onHelpClick, true);
    document.removeEventListener("keydown", onHelpKeydown, true);
    pinnedId.value = null;
    hoverId.value = null;
  }
});

// A view switch clears BOTH pinnedId and hoverId (D52, round-6 amendment):
// the pin because a hidden v-show view's pin would highlight nothing
// visible; the hover because the nav sits outside <main>, so no hover event
// fires en route to the tab and a stale hoverId would otherwise keep the old
// view's topic. Deliberately narrow: hoverId is cleared ONLY here, never
// eagerly elsewhere (the delegation's normal null-on-unannotated-hover stays
// as-is).
watch(activeView, () => {
  pinnedId.value = null;
  hoverId.value = null;
});

// Highlight classes: help-hover (faint) on the hovered element,
// help-pinned (prominent) on the pinned one. Semantic mapping is closed;
// exact colors/widths are presentation tokens
// (latitude-carveout-presentation-tokens).
function setHelpClass(id: string | null, cls: string, prev: Element | null): Element | null {
  prev?.classList.remove(cls);
  if (id === null || !helpMode.value) return null;
  const el = mainEl.value?.querySelector(`[data-help-id="${id}"]`) ?? null;
  el?.classList.add(cls);
  return el;
}
let hoverEl: Element | null = null;
let pinnedEl: Element | null = null;
watch([hoverId, helpMode], () => {
  hoverEl = setHelpClass(hoverId.value, "help-hover", hoverEl);
});
watch([pinnedId, helpMode], () => {
  pinnedEl = setHelpClass(pinnedId.value, "help-pinned", pinnedEl);
});

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
      <button
        type="button"
        data-testid="help-toggle"
        :title="$ta('help-toggle-label').tooltip"
        :aria-pressed="helpMode ? 'true' : 'false'"
        @click="helpMode = !helpMode"
      >
        {{ $t("help-toggle-label") }}
      </button>
    </nav>
    <div class="app-body">
      <main ref="mainEl">
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
      <HelpSidebar
        v-if="helpMode"
        :topic-id="helpTopicId"
      />
    </div>
    <SettingsDialog ref="settingsDialog" />
  </template>
</template>
