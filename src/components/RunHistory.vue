<script setup lang="ts">
// Run history + log export (T11 Step 4, D30 gap closure: mkvtoolnix-gui
// can open a finished job's log as text; this is Muxsmith's equivalent).
// `list_runs` renders newest-first (already sorted server-side, D26);
// selecting a run shows its persisted `summary.json` jobs; selecting a job
// fetches its full `job-<index>.json` via `get_job_log` and offers
// copy-to-clipboard and save-as-file.
//
// `RunMeta.summary` cannot express `joblog_status: "incomplete"` (that
// nuance exists only in the live `run-finished` event JobsView.vue itself
// renders) -- this view shows exactly what `summary.json` has, per the
// binding contract, and invents nothing beyond it.
import { computed, onMounted, ref } from "vue";
import { useFluent } from "fluent-vue";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import { getJobLog, listRuns } from "../ipc";
import type { IpcError, JobLogRecord, RunMeta } from "../ipc";
import { jobStateKey } from "../jobRowState";

const fluent = useFluent();

const runs = ref<RunMeta[]>([]);
const loading = ref(false);
const loadError = ref<IpcError | null>(null);

const selectedRunId = ref<string | null>(null);
const selectedJobIndex = ref<number | null>(null);
const jobLog = ref<JobLogRecord | null>(null);
const jobLogLoading = ref(false);
const jobLogError = ref<IpcError | null>(null);
// One busy flag per export action: save's native dialog can stay open for
// as long as the user likes and must not disable the unrelated copy
// button, and copy's own async gap needs its own double-click guard.
const copyBusy = ref(false);
const saveBusy = ref(false);
const exportFailed = ref(false);

async function refresh() {
  loading.value = true;
  loadError.value = null;
  try {
    runs.value = await listRuns();
  } catch (e) {
    loadError.value = e as IpcError;
  } finally {
    loading.value = false;
  }
}

onMounted(refresh);

const selectedRun = computed(
  () => runs.value.find((r) => r.run_id === selectedRunId.value) ?? null,
);

function selectRun(runId: string) {
  selectedRunId.value = runId;
  selectedJobIndex.value = null;
  jobLog.value = null;
  jobLogError.value = null;
  exportFailed.value = false;
}

async function selectJob(index: number) {
  const runId = selectedRunId.value;
  if (!runId) {
    return;
  }
  selectedJobIndex.value = index;
  jobLog.value = null;
  jobLogError.value = null;
  exportFailed.value = false;
  jobLogLoading.value = true;
  try {
    jobLog.value = await getJobLog(runId, index);
  } catch (e) {
    jobLogError.value = e as IpcError;
  } finally {
    jobLogLoading.value = false;
  }
}

function logText(record: JobLogRecord): string {
  return record.lines.join("\n");
}

async function copyLog() {
  if (!jobLog.value || copyBusy.value) {
    return;
  }
  exportFailed.value = false;
  copyBusy.value = true;
  try {
    await writeText(logText(jobLog.value));
  } catch {
    exportFailed.value = true;
  } finally {
    copyBusy.value = false;
  }
}

async function saveLog() {
  // Captured before the dialog gap: the native save dialog can stay open
  // indefinitely, and the user may select a different job meanwhile --
  // what gets written must be the log that was shown when Save was
  // clicked, matching the suggested filename built from the same ids.
  const record = jobLog.value;
  const runId = selectedRunId.value;
  const jobIndex = selectedJobIndex.value;
  if (!record || runId === null || jobIndex === null || saveBusy.value) {
    return;
  }
  exportFailed.value = false;
  saveBusy.value = true;
  try {
    const path = await save({
      defaultPath: `${runId}-job-${jobIndex}.log`,
      filters: [
        { name: fluent.$t("jobs-history-export-filter-name"), extensions: ["log", "txt"] },
      ],
    });
    if (path) {
      await writeTextFile(path, logText(record));
    }
  } catch {
    exportFailed.value = true;
  } finally {
    saveBusy.value = false;
  }
}

defineExpose({ refresh });
</script>

<template>
  <section data-testid="jobs-history">
    <h3>{{ $t("jobs-history-heading") }}</h3>
    <button
      type="button"
      data-testid="jobs-history-refresh"
      :disabled="loading"
      :aria-busy="loading"
      :title="$ta('jobs-history-refresh').tooltip"
      @click="refresh"
    >
      {{ $t("jobs-history-refresh") }}
    </button>

    <p
      v-if="loadError"
      role="alert"
    >
      {{ $t(loadError.code, loadError.params) }}
    </p>
    <p v-else-if="!loading && runs.length === 0">
      {{ $t("jobs-history-empty") }}
    </p>
    <ul
      v-else
      data-testid="jobs-history-list"
    >
      <li
        v-for="run in runs"
        :key="run.run_id"
      >
        <button
          type="button"
          data-testid="jobs-history-run"
          :data-run-id="run.run_id"
          :aria-current="run.run_id === selectedRunId ? 'true' : undefined"
          @click="selectRun(run.run_id)"
        >
          {{
            $t("jobs-history-run-label", {
              startedAt: run.started_at,
              ok: run.summary.summary.ok,
              warning: run.summary.summary.warning,
              failed: run.summary.summary.failed,
              cancelled: run.summary.summary.cancelled,
            })
          }}
        </button>
      </li>
    </ul>

    <div
      v-if="selectedRun"
      data-testid="jobs-history-detail"
    >
      <table>
        <caption>
          {{ $t("jobs-history-jobs-caption") }}
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
              {{ $t("jobs-col-actions") }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="job in selectedRun.summary.jobs"
            :key="job.index"
          >
            <td>{{ job.output }}</td>
            <td>{{ $t(jobStateKey(job.state)) }}</td>
            <td>
              <button
                type="button"
                data-testid="jobs-history-job"
                :data-index="job.index"
                @click="selectJob(job.index)"
              >
                {{ $t("jobs-history-view-log") }}
              </button>
            </td>
          </tr>
        </tbody>
      </table>

      <div
        v-if="selectedJobIndex !== null"
        data-testid="jobs-history-log"
      >
        <p v-if="jobLogLoading">
          {{ $t("jobs-history-log-loading") }}
        </p>
        <p
          v-else-if="jobLogError"
          role="alert"
        >
          {{ $t(jobLogError.code, jobLogError.params) }}
        </p>
        <template v-else-if="jobLog">
          <h4>{{ $t("jobs-history-log-region-label") }}</h4>
          <pre data-testid="jobs-history-log-text">{{ logText(jobLog) }}</pre>
          <button
            type="button"
            data-testid="jobs-history-copy"
            :disabled="copyBusy"
            :aria-busy="copyBusy"
            :title="$ta('jobs-history-copy-log').tooltip"
            @click="copyLog"
          >
            {{ $t("jobs-history-copy-log") }}
          </button>
          <button
            type="button"
            data-testid="jobs-history-save"
            :disabled="saveBusy"
            :aria-busy="saveBusy"
            :title="$ta('jobs-history-save-log').tooltip"
            @click="saveLog"
          >
            {{ $t("jobs-history-save-log") }}
          </button>
          <p
            v-if="exportFailed"
            role="alert"
          >
            {{ $t("jobs-history-export-failed") }}
          </p>
        </template>
      </div>
    </div>
  </section>
</template>
