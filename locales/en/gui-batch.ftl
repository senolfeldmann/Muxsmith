# T10: BatchView.vue and its components (spec 8.2 view 2, D22: suggestions
# are show-and-copy only, never applied). `browse-button` (gui-common.ftl,
# the generic "Browse..." label) is reused for the source/output directory
# pickers, but their tooltip is `batch-browse-dir-tooltip` below, not
# gui-common.ftl's `browse-button-tooltip` (fix: that one says "Choose the
# file with a file picker", wrong noun for a directory picker; it stays
# reserved for genuine file pickers, e.g. FirstRun's/SettingsDialog's
# mkvmerge-path browse button).

batch-view-heading = Batch

batch-profile-heading = Profile
batch-profile-pick = Choose profile...
batch-profile-pick-tooltip = Pick a Muxsmith profile YAML file to validate and run.
batch-profile-filter-name = Muxsmith profiles
batch-profile-none = No profile selected yet. Choose one below to validate it and start a batch.
batch-profile-current = Selected profile: { $path }

batch-recents-heading = Recent profiles
batch-recents-empty = No recent profiles yet.
batch-recents-select-tooltip = Open this profile again.

batch-dirs-heading = Directories
batch-browse-dir-tooltip = Choose the directory with a folder picker.
batch-source-label = Source directory
batch-source-hint = Directory scanned for input files. Leave empty to use the profile's own source directory.
batch-output-label = Output directory
batch-output-hint = Directory the muxed files are written to. Leave empty to use the profile's output directory.

batch-diagnostics-heading = Diagnostics
batch-diagnostics-summary = { $errors } error(s), { $warnings } warning(s), { $infos } info notice(s).
batch-diagnostic-line = { $severity }: { $message }

batch-dry-run = Dry run
batch-dry-run-tooltip = Resolve every track rule and render the report below without muxing anything.

batch-files-heading = Files
batch-resolution-rule-header = Rule
batch-resolution-track-header = Resolved track
batch-file-caption = { $source } (identifier: { $identifier }) -> { $output }
batch-file-no-plan = { $source } (identifier: { $identifier }): no plan produced; see diagnostics below.

batch-suggestions-heading = Suggestions
batch-suggestion-header = Suggestion for { $config_path }:
batch-suggestion-copy = Copy
batch-suggestion-copy-tooltip = Copy this YAML fragment to the clipboard.
batch-suggestion-copied = Copied to clipboard.

batch-run = Run
batch-run-tooltip = Start this batch: mux every resolved file with mkvmerge.
batch-run-tooltip-no-profile = Choose and validate a profile before running.
batch-run-tooltip-errors = Fix every error-severity diagnostic before running.
batch-run-tooltip-mkvmerge-missing = mkvmerge is not available; fix detection in Settings before running.
