# T11: Jobs view (spec 8.2 view 3, D30) - live queue, cancel, history, log
# export. Keyed by component area (jobs-batch-*/jobs-row-*/jobs-log-*/
# jobs-history-*), mirroring gui-settings.ftl's per-view prefix convention.

## Batch header + run summary

jobs-batch-progress = { $finished } / { $total } jobs finished
jobs-cancel-batch-label = Cancel batch
    .tooltip = Cancel every queued and in-flight job in this run.
jobs-summary-line = { $ok } ok, { $warning } warning, { $failed } failed, { $cancelled } cancelled
jobs-joblog-incomplete = The run's log could not be fully written to disk; some job records may be missing.
jobs-joblog-unavailable = The run's log could not be written to disk; this run will not appear in history.
jobs-no-run = No run is active. Start a run from the Batch view.

## Job table (JobRow.vue)

jobs-table-caption = Jobs in the current run
jobs-col-output = Output
jobs-col-state = State
jobs-col-progress = Progress
jobs-col-actions = Actions
jobs-row-output-pending = Job { $index }
jobs-row-progress-label = Progress for job { $index }
jobs-row-cancel-label = Cancel
    .tooltip = Cancel this job.
jobs-row-warning-count = { $count ->
    [one] 1 warning
   *[other] { $count } warnings
}
jobs-state-queued = Queued
jobs-state-running = Running
jobs-state-ok = Done
jobs-state-warning = Done with warnings
jobs-state-failed = Failed
jobs-state-cancelled = Cancelled

## Live log (LiveLog.vue)

jobs-log-region-label = Live job output
jobs-log-filter-label = Show output for
jobs-log-filter-all = All jobs

## Run history + log export (RunHistory.vue, D30 gap closure)

jobs-history-heading = Run history
jobs-history-refresh = Refresh
    .tooltip = Reload the list of past runs from disk.
jobs-history-empty = No past runs found.
jobs-history-run-label = { $startedAt } - { $ok } ok, { $warning } warning, { $failed } failed, { $cancelled } cancelled
jobs-history-jobs-caption = Jobs in this run
jobs-history-view-log = View log
jobs-history-log-region-label = Job log
jobs-history-log-loading = Loading job log...
jobs-history-copy-log = Copy log
    .tooltip = Copy this job's full log to the clipboard.
jobs-history-save-log = Save as...
    .tooltip = Save this job's full log to a file.
jobs-history-export-failed = The log could not be copied or saved.
jobs-history-export-filter-name = Log files

## Shell-level IPC error codes (src-tauri/src/run.rs::IpcError codes; keyed
## directly on IpcError.code, same convention as gui-common.ftl's
## mkvmerge-*/settings-* block).

run-already-active = A run is already active.
no-active-run = No run is currently active.
invalid-run-id = "{ $run_id }" is not a valid run id.
job-log-unavailable = The run log location could not be determined on this system.
job-log-not-found = No log was found for job { $index } of run { $run_id }.
