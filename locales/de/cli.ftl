# German (de) translation of locales/en/cli.ftl. The en catalog is the
# source of truth; keys, placeables and selector structure mirror it and
# are parity-enforced by scripts/check-i18n.mjs. Terminology per Task 21
# (#17): Spur/Datei/Profil/Regel/Vorschlag, mkvtoolnix-de anchors where one
# exists. Sie-less imperative (du-Imperativ) for direct CLI instructions.
validate-ok = Das Profil ist gültig.
validate-summary = { $errors ->
    [one] 1 Fehler
   *[other] { $errors } Fehler
}, { $warnings ->
    [one] 1 Warnung
   *[other] { $warnings } Warnungen
}, { $infos ->
    [one] 1 Info
   *[other] { $infos } Infos
}.
diagnostic-line = [{ $severity }] { $config_path }: { $message }
diagnostic-line-file = [{ $severity }] { $file } { $config_path }: { $message }
mkvmerge-not-found = mkvmerge wurde nicht im PATH gefunden. Installiere MKVToolNix oder gib den Pfad zu mkvmerge an.
mkvmerge-query-failed = Die Abfrage von mkvmerge ist fehlgeschlagen.
identify-failed = { $file } konnte nicht identifiziert werden.
identify-not-media = { $file } ist keine erkannte Mediendatei.
identify-track-line = Spur { $id }: { $type } [{ $codec }] { $language }
dry-run-file = { $file } (Kennung: { $id })
dry-run-assignment =   Regel { $rule } -> Spur { $track }
dry-run-output =   Ausgabe: { $path }
dry-run-suggestion = Vorschlag für { $config_path }:
dry-run-summary = { $count ->
    [one] 1 passende Datei
   *[other] { $count } passende Dateien
} (durchsucht { $root }, Erweiterungen { $extensions })
run-job-start = [{ $index }/{ $total }] { $output } ... Start
run-job-progress = [{ $index }/{ $total }] { $output } ... { $percent }%
run-job-notice = [{ $index }/{ $total }] { $output } ... { $text }
run-job-ok = [{ $index }/{ $total }] { $output } ... ok ({ $seconds }s)
run-job-warning = [{ $index }/{ $total }] { $output } ... Warnung ({ $count ->
    [one] 1 Warnung
   *[other] { $count } Warnungen
}, { $seconds }s)
run-job-failed = [{ $index }/{ $total }] { $output } ... fehlgeschlagen (Exit-Code { $code })
run-job-cancelled = [{ $index }/{ $total }] { $output } ... abgebrochen
run-summary = { $ok } ok, { $warning } Warnung, { $failed } fehlgeschlagen, { $cancelled } abgebrochen
run-joblog-unavailable = Die Job-Protokolle für diesen Lauf konnten nicht geschrieben werden; es wird ohne gespeicherte Protokolle fortgefahren.
run-joblog-written = Die Job-Protokolle wurden nach { $dir } geschrieben.
run-joblog-incomplete = Die Job-Protokolle unter { $dir } sind unvollständig; einige Protokolldateien konnten nicht geschrieben werden.
