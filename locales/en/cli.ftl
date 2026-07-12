validate-ok = Profile is valid.
validate-summary = { $errors ->
    [one] 1 error
   *[other] { $errors } errors
}, { $warnings ->
    [one] 1 warning
   *[other] { $warnings } warnings
}, { $infos ->
    [one] 1 info
   *[other] { $infos } infos
}.
diagnostic-line = [{ $severity }] { $config_path }: { $message }
diagnostic-line-file = [{ $severity }] { $file } { $config_path }: { $message }
mkvmerge-not-found = mkvmerge was not found on PATH. Install MKVToolNix or set the mkvmerge path.
mkvmerge-query-failed = Querying mkvmerge failed.
identify-failed = Could not identify { $file }.
identify-not-media = { $file } is not a recognized media file.
identify-track-line = Track { $id }: { $type } [{ $codec }] { $language }
dry-run-file = { $file } (identifier: { $id })
dry-run-assignment =   rule { $rule } -> track { $track }
dry-run-output =   output: { $path }
dry-run-suggestion = Suggestion for { $config_path }:
dry-run-summary = { $count ->
    [one] 1 file matched
   *[other] { $count } files matched
} (searched { $root }, extensions { $extensions })
run-job-start = [{ $index }/{ $total }] { $output } ... start
run-job-progress = [{ $index }/{ $total }] { $output } ... { $percent }%
run-job-notice = [{ $index }/{ $total }] { $output } ... { $text }
run-job-ok = [{ $index }/{ $total }] { $output } ... ok ({ $seconds }s)
run-job-warning = [{ $index }/{ $total }] { $output } ... warning ({ $count ->
    [one] 1 warning
   *[other] { $count } warnings
}, { $seconds }s)
run-job-failed = [{ $index }/{ $total }] { $output } ... failed (exit { $code })
run-job-cancelled = [{ $index }/{ $total }] { $output } ... cancelled
run-summary = { $ok } ok, { $warning } warning, { $failed } failed, { $cancelled } cancelled
run-joblog-unavailable = Job logs could not be written for this run; continuing without persisted logs.
run-joblog-written = Job logs written to { $dir }
run-joblog-incomplete = Job logs under { $dir } are incomplete; some log files could not be written.
