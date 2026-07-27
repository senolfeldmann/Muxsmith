# Batch view

The Batch view takes a profile from selection to execution: choose a profile, review its validation, dry-run it against real files, then run.

- **Choose a profile.** Pick a Muxsmith profile YAML file, or reopen one from the recent-profiles list. Selecting a profile validates it immediately; the result appears under Diagnostics.
- **Directories.** The source directory is scanned for input files; the output directory receives the muxed files. Leave either empty to use the directory configured in the profile itself. Both entries are remembered per profile and restored the next time you select it.
- **Dry run** resolves every track rule against the files actually found and renders the full report below, without muxing anything.
- **Run** starts the batch; execution and progress move to the Jobs view.

## The resolution table

After a dry run, every matched file gets its own table: one row per track rule, in profile order, showing the track that rule resolved to (track id and kind). A `-` in the resolved-track column means the rule matched no track in this file; for a rule marked `optional` that is normal, for any other rule the report carries a diagnostic. A file listed without a table produced no plan at all; its diagnostics state why.

## Diagnostics

Findings come in three severities: errors, warnings and info notices. The summary line counts them across the profile, the batch and every file. Errors block Run; warnings and info notices do not. When the report proposes concrete fixes, they appear as suggestion cards below the file list (see the Suggestion card topic).

## When Run is disabled

Run stays disabled while no validated profile is selected, while any error remains, while mkvmerge is not available, or while another run is already active; the button's tooltip names the current reason. A dry run is not required first: Run plans internally from the current profile and directories.
