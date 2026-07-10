/**
 * Shared job-row model for the Jobs view (T11, spec 8.2 view 3): the local
 * per-index state `JobsView.vue` accumulates from the live `job-event`
 * stream and reconciles from `run-finished`, plus the terminal-state ->
 * Fluent-key mapping both `JobRow.vue` (live rows) and `RunHistory.vue`
 * (persisted `RunJobEntry`s from a past run's `summary.json`) need.
 */
import type { JobOutcome, JobState } from "./ipc";

/**
 * One row's live state. Starts `"queued"` (no event yet for this index --
 * covers both "not dequeued yet" and a batch-cancelled job that is skipped
 * silently and never emits anything live, D16/D25); `"started"`/
 * `"progress"` track the live stream; `"finished"` is set either from a
 * live `finished` event or, for a row that received no live events at all,
 * reconciled directly from the `run-finished` document (the only source
 * for that case).
 */
export type JobRowLiveState =
  | { kind: "queued" }
  | { kind: "started" }
  | { kind: "progress"; percent: number }
  | { kind: "finished"; outcome: JobOutcome };

export interface JobRowData {
  index: number;
  /** The job's rendered output path; unknown until its `started` event (or
   * the run's finished document) reports it. */
  output: string | null;
  state: JobRowLiveState;
  /** Live-updated from `warning` events as they arrive; overwritten with
   * the authoritative count once the row reaches `"finished"`. */
  warningCount: number;
}

export function emptyRow(index: number): JobRowData {
  return { index, output: null, state: { kind: "queued" }, warningCount: 0 };
}

/** Maps a terminal `JobState` (the Rust enum's four values) to its
 * gui-jobs.ftl state-chip key. Shared between `JobRow.vue` (live rows,
 * once `state.kind === "finished"`) and `RunHistory.vue` (a persisted
 * `RunJobEntry`, always terminal). */
export function jobStateKey(state: JobState): string {
  switch (state) {
    case "ok":
      return "jobs-state-ok";
    case "warning":
      return "jobs-state-warning";
    case "failed":
      return "jobs-state-failed";
    case "cancelled":
      return "jobs-state-cancelled";
  }
}
