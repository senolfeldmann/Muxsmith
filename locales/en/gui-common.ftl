app-title = Muxsmith

# D31: native close-confirmation dialog (window close with an active run).
# Consumed by the Rust shell at build time (include_str lookup in
# src-tauri/src/run.rs); keep these entries single-line simple messages
# (no attributes, no multiline continuations) -- the shell's lookup is a
# line parser, not a Fluent parser, and a unit test pins each key.
# Wording per D31's mkvtoolnix-gui reference (main_window.cpp,
# beforeCloseCheckRunningJobs).
close-abort-title = Abort running jobs
close-abort-message = There is currently a job running. Do you really want to abort all currently running jobs and quit?
close-abort-confirm = Abort jobs and quit
close-abort-dismiss = Cancel
