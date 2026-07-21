# Jobs view

The Jobs view shows the run currently executing and the history of past runs. Runs are started from the Batch view; this view takes over the moment one starts.

## Run lifecycle

Each job muxes one output file with mkvmerge. A job starts queued, then runs with live progress, and finishes in one of four states: done, done with warnings, failed, or cancelled. The header counts finished jobs against the total; after the run, the summary line counts ok, warning, failed and cancelled jobs.

## Cancel semantics

- **Cancel batch** stops the whole run: jobs still queued are marked cancelled, and every job currently muxing is terminated.
- **Cancel** on a single row cancels only that job: if it is running it is terminated immediately, if it is still queued it is skipped when its turn comes. The rest of the run continues.

Jobs that already finished keep their result either way.

## Live log

The live log streams mkvmerge's output as it arrives and can be filtered to a single job. The display keeps only the most recent lines; the complete log of every job is written to disk regardless.

## History and log export

Run history lists past runs from disk, with start time and per-job outcomes. For any job in any listed run you can view its full log, copy it to the clipboard, or save it to a file. If a note after a run says its log could not be written (or not completely), that run is missing from history or its job records are incomplete.
