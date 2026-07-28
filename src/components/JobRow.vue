<script setup lang="ts">
// One job's queue row (T11 Step 1, spec 8.2 view 3): output filename,
// state chip, progress, per-row cancel. Purely presentational -- all state
// transitions happen in JobsView.vue's event handlers; this component only
// renders `job` and asks its parent to cancel via the `cancel` emit.
import { computed } from "vue";
import type { JobRowData } from "../jobRowState";
import { jobStateKey } from "../jobRowState";

const props = defineProps<{ job: JobRowData }>();
const emit = defineEmits<{ cancel: [index: number] }>();

const stateKey = computed(() => {
  const state = props.job.state;
  if (state.kind === "queued") {
    return "jobs-state-queued";
  }
  if (state.kind === "started" || state.kind === "progress") {
    return "jobs-state-running";
  }
  return jobStateKey(state.outcome.state);
});

// `undefined` renders a native indeterminate <progress> (no known percent
// yet: queued/started); a "finished" row is drawn full regardless of the
// terminal outcome (ok/warning/failed/cancelled -- the state chip already
// carries that distinction, D29's "role=progressbar ... or native
// <progress>").
const progressValue = computed<number | undefined>(() => {
  const state = props.job.state;
  if (state.kind === "progress") {
    return state.percent;
  }
  if (state.kind === "finished") {
    return 100;
  }
  return undefined;
});

// D100: the worker-panic payload a finished outcome carries (`JobOutcome.
// panic`, D98) is the one per-job error text this row surfaces; `null` for
// every non-terminal row and for every finished job that did not panic, so
// a failed row without a panic renders exactly as before.
const panicDetail = computed<string | null>(() => {
  const state = props.job.state;
  return state.kind === "finished" ? state.outcome.panic : null;
});

const isTerminal = computed(() => props.job.state.kind === "finished");
</script>

<template>
  <tr
    data-testid="job-row"
    :data-index="job.index"
  >
    <td>{{ job.output ?? $t("jobs-row-output-pending", { index: job.index }) }}</td>
    <td>
      <span>{{ $t(stateKey) }}</span>
      <span v-if="job.warningCount > 0">{{ $t("jobs-row-warning-count", { count: job.warningCount }) }}</span>
      <span
        v-if="panicDetail !== null"
        data-testid="job-panic"
      >{{ $t("worker-panicked", { detail: panicDetail }) }}</span>
    </td>
    <td>
      <progress
        :value="progressValue"
        max="100"
        data-testid="job-progress"
        :aria-label="$t('jobs-row-progress-label', { index: job.index })"
      />
    </td>
    <td>
      <button
        type="button"
        data-testid="job-cancel"
        :disabled="isTerminal"
        :title="$ta('jobs-row-cancel-label').tooltip"
        @click="emit('cancel', job.index)"
      >
        {{ $t("jobs-row-cancel-label") }}
      </button>
    </td>
  </tr>
</template>
