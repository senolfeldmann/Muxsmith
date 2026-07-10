<script setup lang="ts">
// Jobs view (T11, spec 8.2 view 3, D30): live queue, per-job/batch cancel,
// run-finished summary, and run history + log export.
//
// Wave-5 contract (binding, controller-defined): receives `pending-run`
// from App.vue (owned by the parallel Batch-view task, T10) and, on every
// non-null value, invokes `start_run` then emits `consumed` so App clears
// its ref. `PendingRun` is declared here regardless of whether App.vue
// already passes it -- the two sides reconcile trivially since this file
// owns its whole contents.
//
// Event-ordering contract (`run.rs::start_run`'s own doc, binding): both
// `muxsmith://job-event` and `muxsmith://run-finished` listeners MUST be
// registered -- and, since `listen()` itself is async, actually confirmed
// registered -- before `start_run` is invoked. A soft outcome (profile
// load failure, missing mkvmerge, zero planned jobs) emits
// `muxsmith://run-finished` synchronously inside the Rust command, before
// its own promise resolves; `ensureListeners()` is awaited first on every
// dispatch to close that race.
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import JobRow from "../components/JobRow.vue";
import LiveLog from "../components/LiveLog.vue";
import RunHistory from "../components/RunHistory.vue";
import { cancelJob, cancelRun, startRun, JOB_EVENT_CHANNEL, RUN_FINISHED_CHANNEL } from "../ipc";
import type { IpcError, JobEvent, RunFinishedEvent } from "../ipc";
import { emptyRow } from "../jobRowState";
import type { JobRowData } from "../jobRowState";

/** Mirrors the Wave-5 contract's `RunRequest` shape (App.vue/BatchView.vue,
 * T10): the parameters a Batch-view "Run" click hands over. */
interface RunRequest {
  profile: string;
  source: string | null;
  output: string | null;
  jobs: number | null;
}

// Optional (`pendingRun?:`), not a bare required prop: App.vue is the
// PARALLEL task's file (T10) and does not pass `pending-run` yet on this
// branch's current placeholder wiring (`<JobsView v-else />`, no props) --
// this view must still type-check and behave correctly (no pending run) as
// a standalone unit until that wiring lands. The watcher below treats a
// missing prop exactly like an explicit `null` (`if (!req) return;`), so
// T10's eventual `:pending-run` binding needs no further change here.
const props = defineProps<{ pendingRun?: RunRequest | null }>();
const emit = defineEmits<{ consumed: [] }>();

// The DOM cap applies to the combined live-output feed LiveLog.vue filters
// locally (Step 2: "DOM-capped at 5000 lines - the full log is in the
// file"): the persisted job-<index>.json record has every line regardless.
const LOG_LINE_CAP = 5000;

const jobs = ref<JobRowData[]>([]);
const runActive = ref(false);
const startError = ref<IpcError | null>(null);
const actionError = ref<IpcError | null>(null);
const finishedSummary = ref<RunFinishedEvent | null>(null);
const logLines = ref<{ index: number; line: string }[]>([]);

const runHistoryRef = ref<InstanceType<typeof RunHistory> | null>(null);

function ensureJobsLength(n: number) {
  while (jobs.value.length < n) {
    jobs.value.push(emptyRow(jobs.value.length));
  }
}

function onJobEvent(event: JobEvent) {
  ensureJobsLength(event.index + 1);
  const row = jobs.value[event.index];
  switch (event.event) {
    case "started":
      row.output = event.output;
      row.state = { kind: "started" };
      break;
    case "progress":
      row.state = { kind: "progress", percent: event.percent };
      break;
    case "warning":
      row.warningCount += 1;
      break;
    case "error":
      // Surfaced via the eventual `finished` outcome's `errors` and via
      // LiveLog (every line, including tagged ones, also arrives as its
      // own verbatim `output` event, D24) -- no separate row field beyond
      // the warning-count badge the brief asks for.
      break;
    case "finished":
      row.state = { kind: "finished", outcome: event.outcome };
      row.warningCount = event.outcome.warnings.length;
      break;
    case "output":
      logLines.value.push({ index: event.index, line: event.line });
      if (logLines.value.length > LOG_LINE_CAP) {
        logLines.value.splice(0, logLines.value.length - LOG_LINE_CAP);
      }
      break;
  }
}

function onRunFinished(event: RunFinishedEvent) {
  // Authoritative reconciliation (Step 3): every index in the document is
  // written from it directly, regardless of what live events already said
  // -- the only source for a job that emitted nothing live at all (a
  // batch-cancelled job never dequeued, D16).
  ensureJobsLength(event.jobs.length);
  for (const entry of event.jobs) {
    jobs.value[entry.index] = {
      index: entry.index,
      output: entry.output,
      state: { kind: "finished", outcome: entry },
      warningCount: entry.warnings.length,
    };
  }
  finishedSummary.value = event;
  runActive.value = false;
  runHistoryRef.value?.refresh();
}

let listenersReady: Promise<[UnlistenFn, UnlistenFn]> | null = null;
let unlistenJobEvent: UnlistenFn | null = null;
let unlistenRunFinished: UnlistenFn | null = null;

function ensureListeners(): Promise<[UnlistenFn, UnlistenFn]> {
  if (!listenersReady) {
    listenersReady = Promise.all([
      listen<JobEvent>(JOB_EVENT_CHANNEL, (e) => onJobEvent(e.payload)),
      listen<RunFinishedEvent>(RUN_FINISHED_CHANNEL, (e) => onRunFinished(e.payload)),
    ]);
    void listenersReady.then(([un1, un2]) => {
      unlistenJobEvent = un1;
      unlistenRunFinished = un2;
    });
  }
  return listenersReady;
}

onMounted(() => {
  void ensureListeners();
});

onUnmounted(() => {
  unlistenJobEvent?.();
  unlistenRunFinished?.();
});

watch(
  () => props.pendingRun,
  async (req) => {
    if (!req) {
      return;
    }
    await ensureListeners();
    jobs.value = [];
    logLines.value = [];
    finishedSummary.value = null;
    startError.value = null;
    actionError.value = null;
    runActive.value = true;
    try {
      const started = await startRun({
        profile: req.profile,
        source: req.source ?? undefined,
        output: req.output ?? undefined,
        jobs: req.jobs ?? undefined,
      });
      ensureJobsLength(started.total_jobs);
    } catch (e) {
      startError.value = e as IpcError;
      runActive.value = false;
    } finally {
      emit("consumed");
    }
  },
  { immediate: true },
);

async function onCancelBatch() {
  actionError.value = null;
  try {
    await cancelRun();
  } catch (e) {
    actionError.value = e as IpcError;
  }
}

async function onCancelJob(index: number) {
  actionError.value = null;
  try {
    await cancelJob(index);
  } catch (e) {
    actionError.value = e as IpcError;
  }
}

const finishedCount = computed(
  () => jobs.value.filter((j) => j.state.kind === "finished").length,
);

const joblogNoteKey = computed(() => {
  switch (finishedSummary.value?.joblog_status) {
    case "incomplete":
      return "jobs-joblog-incomplete";
    case "unavailable":
      return "jobs-joblog-unavailable";
    default:
      return null;
  }
});
</script>

<template>
  <section data-testid="view-jobs">
    <h2>{{ $t("nav-jobs") }}</h2>

    <p
      v-if="startError"
      role="alert"
    >
      {{ $t(startError.code, startError.params) }}
    </p>
    <p
      v-if="actionError"
      role="alert"
    >
      {{ $t(actionError.code, actionError.params) }}
    </p>

    <template v-if="jobs.length > 0 || runActive || finishedSummary">
      <section data-testid="jobs-batch-header">
        <p>{{ $t("jobs-batch-progress", { finished: finishedCount, total: jobs.length }) }}</p>
        <button
          type="button"
          data-testid="cancel-batch"
          :disabled="!runActive"
          :title="$t('jobs-cancel-batch-tooltip')"
          @click="onCancelBatch"
        >
          {{ $t("jobs-cancel-batch-label") }}
        </button>
      </section>

      <table data-testid="jobs-table">
        <caption>
          {{ $t("jobs-table-caption") }}
        </caption>
        <thead>
          <tr>
            <th scope="col">
              {{ $t("jobs-col-output") }}
            </th>
            <th scope="col">
              {{ $t("jobs-col-state") }}
            </th>
            <th scope="col">
              {{ $t("jobs-col-progress") }}
            </th>
            <th scope="col">
              {{ $t("jobs-col-actions") }}
            </th>
          </tr>
        </thead>
        <tbody>
          <JobRow
            v-for="job in jobs"
            :key="job.index"
            :job="job"
            @cancel="onCancelJob"
          />
        </tbody>
      </table>

      <LiveLog
        :jobs="jobs"
        :lines="logLines"
      />

      <p
        aria-live="polite"
        data-testid="jobs-run-summary"
      >
        <template v-if="finishedSummary">
          {{
            $t("jobs-summary-line", {
              ok: finishedSummary.summary.ok,
              warning: finishedSummary.summary.warning,
              failed: finishedSummary.summary.failed,
              cancelled: finishedSummary.summary.cancelled,
            })
          }}
          <template v-if="joblogNoteKey">
            {{ $t(joblogNoteKey) }}
          </template>
        </template>
      </p>
    </template>
    <p
      v-else
      data-testid="jobs-empty"
    >
      {{ $t("jobs-no-run") }}
    </p>

    <RunHistory ref="runHistoryRef" />
  </section>
</template>
