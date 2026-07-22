# German (de) translation of locales/en/gui-jobs.ftl. The en catalog is the
# source of truth; keys mirror it (id parity enforced by
# scripts/check-i18n.mjs); placeables and selector structure mirror it by
# convention (reviewed manually, not machine-checked). run -> Lauf, Job(s),
# Status, Verlauf, Protokoll. The IpcError-code keys mirror their en
# wording.

## Batch header + run summary

jobs-batch-progress = { $finished } / { $total } Jobs abgeschlossen
jobs-cancel-batch-label = Stapel abbrechen
    .tooltip = Jeden eingereihten und laufenden Job in diesem Lauf abbrechen.
jobs-summary-line = { $ok } ok, { $warning } Warnung, { $failed } fehlgeschlagen, { $cancelled } abgebrochen
jobs-joblog-incomplete = Das Protokoll des Laufs konnte nicht vollständig auf die Festplatte geschrieben werden; einige Job-Einträge fehlen möglicherweise.
jobs-joblog-unavailable = Das Protokoll des Laufs konnte nicht auf die Festplatte geschrieben werden; dieser Lauf erscheint nicht im Verlauf.
jobs-no-run = Es ist kein Lauf aktiv. Starte einen Lauf in der Stapel-Ansicht.

## Job table (JobRow.vue)

jobs-table-caption = Jobs im aktuellen Lauf
jobs-col-output = Ausgabe
jobs-col-state = Status
jobs-col-progress = Fortschritt
jobs-col-actions = Aktionen
jobs-row-output-pending = Job { $index }
jobs-row-progress-label = Fortschritt für Job { $index }
jobs-row-cancel-label = Abbrechen
    .tooltip = Diesen Job abbrechen.
jobs-row-warning-count = { $count ->
    [one] 1 Warnung
   *[other] { $count } Warnungen
}
jobs-state-queued = In Warteschlange
jobs-state-running = Läuft
jobs-state-ok = Fertig
jobs-state-warning = Fertig mit Warnungen
jobs-state-failed = Fehlgeschlagen
jobs-state-cancelled = Abgebrochen

## Live log (LiveLog.vue)

jobs-log-region-label = Live-Job-Ausgabe
jobs-log-filter-label = Ausgabe anzeigen für
jobs-log-filter-all = Alle Jobs

## Run history + log export (RunHistory.vue, D30 gap closure)

jobs-history-heading = Lauf-Verlauf
jobs-history-refresh = Aktualisieren
    .tooltip = Die Liste vergangener Läufe von der Festplatte neu laden.
jobs-history-empty = Keine vergangenen Läufe gefunden.
jobs-history-run-label = { $startedAt } - { $ok } ok, { $warning } Warnung, { $failed } fehlgeschlagen, { $cancelled } abgebrochen
jobs-history-jobs-caption = Jobs in diesem Lauf
jobs-history-view-log = Protokoll anzeigen
jobs-history-log-region-label = Job-Protokoll
jobs-history-log-loading = Job-Protokoll wird geladen...
jobs-history-copy-log = Protokoll kopieren
    .tooltip = Das vollständige Protokoll dieses Jobs in die Zwischenablage kopieren.
jobs-history-save-log = Speichern unter...
    .tooltip = Das vollständige Protokoll dieses Jobs in eine Datei speichern.
jobs-history-export-failed = Das Protokoll konnte nicht kopiert oder gespeichert werden.
jobs-history-export-filter-name = Protokolldateien

## Shell-level IPC error codes (src-tauri/src/run.rs::IpcError codes; keyed
## directly on IpcError.code, same convention as gui-common.ftl's
## mkvmerge-*/settings-* block).

run-already-active = Es ist bereits ein Lauf aktiv.
no-active-run = Derzeit ist kein Lauf aktiv.
invalid-run-id = "{ $run_id }" ist keine gültige Lauf-ID.
job-log-unavailable = Der Speicherort des Lauf-Protokolls konnte auf diesem System nicht ermittelt werden.
job-log-not-found = Für Job { $index } von Lauf { $run_id } wurde kein Protokoll gefunden.
