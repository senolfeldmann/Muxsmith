app-title = Muxsmith

## D31: native close-confirmation dialog (window close with an active run).
## Consumed by the Rust shell at build time (include_str lookup in
## src-tauri/src/run.rs); keep these entries single-line simple messages
## (no attributes, no multiline continuations) -- the shell's lookup is a
## line parser, not a Fluent parser, and a unit test pins each key.
## Wording per D31's mkvtoolnix-gui reference (main_window.cpp,
## beforeCloseCheckRunningJobs).
close-abort-title = Abort running jobs
close-abort-message = There is currently a job running. Do you really want to abort all currently running jobs and quit?
close-abort-confirm = Abort jobs and quit
close-abort-dismiss = Cancel

## T9: app shell navigation (single window, three views + first-run +
## settings, spec 8.2). aria-current="page" on the active tab is set in
## the template, not encoded here. `nav-editor` added post-close (owner
## ruling 2026-07-17, plan-6 surface pass): a dedicated key for the editor
## tab instead of Task 13's original reuse of `batch-profile-heading`
## ("Profile"), which sat beside the other two tabs' activity labels
## while naming an object, and doubled a string already on screen as
## BatchView's own section heading.
nav-label = Main navigation
nav-batch = Batch
nav-jobs = Jobs
nav-editor = Editor
settings-open-label = Settings
settings-open-tooltip = Open application settings (mkvmerge path, default jobs, language).
browse-button = Browse...
browse-button-tooltip = Choose the file with a file picker.

## T9: shell-level IPC error codes (src-tauri/src/error.rs::IpcError).
## Keyed directly on IpcError.code, exactly like a core Diagnostic's `code`
## selects a diagnostics.ftl entry (spec 8.4): the frontend never shows a
## raw code when a message exists here, and falls back to the code string
## itself only when a key is genuinely missing.
mkvmerge-not-found = mkvmerge was not found.
mkvmerge-too-old = The mkvmerge found ({ $found }) is older than the required minimum version { $minimum }.
mkvmerge-spawn-failed = mkvmerge could not be started: { $detail }
# identify is registered IPC surface (src-tauri/src/lib.rs::identify) but
# had no gui-common.ftl entry, unlike every other IpcError code above and
# below it -- a missing key that fell back to the raw "identify-failed"
# code string. Wording adapted from cli.ftl's identical-purpose
# `identify-failed`, not copied verbatim: the CLI message interpolates
# `{ $file }`, but IpcError::from(IdentifyError) (error.rs) only ever
# attaches a `detail` param (the underlying JSON-parse/stat error text),
# never `file` -- a literal copy would reference an unpopulated variable.
identify-failed = Could not identify the file: { $detail }
mkvmerge-query-failed = Querying mkvmerge failed: { $detail }
settings-dir-unavailable = The application settings location could not be determined on this system.
settings-io-failed = Application settings could not be read or written: { $detail }
settings-parse-failed = The application settings file is corrupt: { $detail }
internal-task-failed = An internal error occurred: { $detail }

## D42/D43: the editor's IPC error codes (save_profile, apply_suggestion).
profile-save-io-failed = The profile could not be written: { $detail }
profile-save-failed = The profile could not be serialized for saving: { $detail }
apply-unparsable-config-path = The suggestion could not be applied: "{ $path }" does not name a rule.
apply-rule-index-out-of-range = The suggestion could not be applied: no rule at index { $index } (rule count: { $rules }).
apply-edit-changed-nothing = The suggestion changed nothing: rule { $index } already constrains "{ $property }".

## T9: first-run mkvmerge detection (D28).
firstrun-detecting = Looking for mkvmerge...
firstrun-missing-heading = mkvmerge was not found
firstrun-too-old-heading = mkvmerge is too old
firstrun-detect-failed-heading = mkvmerge detection failed
firstrun-guidance-windows = Install MKVToolNix from mkvtoolnix.download, then retry, or point Muxsmith directly at mkvmerge.exe below (typically under %ProgramFiles%\MKVToolNix\mkvmerge.exe).
firstrun-guidance-macos = Install MKVToolNix from mkvtoolnix.download into /Applications, then retry, or point Muxsmith directly at the mkvmerge binary below (typically /Applications/MKVToolNix.app/Contents/MacOS/mkvmerge or /usr/local/bin/mkvmerge).
firstrun-guidance-linux = Install the mkvtoolnix package from your distribution (e.g. apt, dnf, pacman), then retry, or point Muxsmith directly at the mkvmerge binary below (typically /usr/bin/mkvmerge or /usr/local/bin/mkvmerge).
firstrun-guidance-fallback = Install MKVToolNix from mkvtoolnix.download, then retry, or point Muxsmith directly at the mkvmerge binary below.
firstrun-picker-label = mkvmerge executable path
firstrun-picker-hint = Enter or browse to the mkvmerge executable if it is not installed in a standard location.
firstrun-use-path = Use this path
firstrun-use-path-tooltip = Save this mkvmerge path and detect it again.
firstrun-retry = Retry detection
firstrun-retry-tooltip = Detect mkvmerge again without changing the configured path.
