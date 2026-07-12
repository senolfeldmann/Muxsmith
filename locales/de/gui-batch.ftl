# German (de) translation of locales/en/gui-batch.ftl. The en catalog is
# the source of truth; keys, placeables and selector structure mirror it
# (parity-enforced by scripts/check-i18n.mjs). Batch -> Stapel, Regel,
# Vorschlag, Probelauf (dry run); GUI imperative in infinitive form.

batch-view-heading = Stapel

batch-profile-heading = Profil
batch-profile-pick = Profil auswählen...
batch-profile-pick-tooltip = Eine Muxsmith-Profil-YAML-Datei zum Prüfen und Ausführen auswählen.
batch-profile-filter-name = Muxsmith-Profile
batch-profile-none = Noch kein Profil ausgewählt. Wähle unten eines aus, um es zu prüfen und einen Stapel zu starten.
batch-profile-current = Ausgewähltes Profil: { $path }

batch-recents-heading = Zuletzt verwendete Profile
batch-recents-empty = Noch keine zuletzt verwendeten Profile.
batch-recents-select-tooltip = Dieses Profil erneut öffnen.

batch-dirs-heading = Verzeichnisse
batch-browse-dir-tooltip = Das Verzeichnis über einen Ordnerdialog auswählen.
batch-source-label = Quellverzeichnis
batch-source-hint = Verzeichnis, das nach Eingabedateien durchsucht wird. Leer lassen, um das im Profil hinterlegte Quellverzeichnis zu verwenden.
batch-output-label = Ausgabeverzeichnis
batch-output-hint = Verzeichnis, in das die gemultiplexten Dateien geschrieben werden. Leer lassen, um das Ausgabeverzeichnis des Profils zu verwenden.

batch-diagnostics-heading = Diagnosen
batch-diagnostics-summary = { $errors ->
    [one] 1 Fehler
   *[other] { $errors } Fehler
}, { $warnings ->
    [one] 1 Warnung
   *[other] { $warnings } Warnungen
}, { $infos ->
    [one] 1 Hinweis
   *[other] { $infos } Hinweise
}.
batch-diagnostic-line = { $severity }: { $message }

batch-dry-run = Probelauf
batch-dry-run-tooltip = Jede Spurregel auflösen und den Bericht unten erzeugen, ohne etwas zu multiplexen.

batch-files-heading = Dateien
batch-resolution-rule-header = Regel
batch-resolution-track-header = Aufgelöste Spur
batch-file-caption = { $source } (Kennung: { $identifier }) -> { $output }
batch-file-no-plan = { $source } (Kennung: { $identifier }): kein Plan erzeugt; siehe Diagnosen unten.

batch-suggestions-heading = Vorschläge
batch-suggestion-header = Vorschlag für { $config_path }:
batch-suggestion-copy = Kopieren
batch-suggestion-copy-tooltip = Dieses YAML-Fragment in die Zwischenablage kopieren.
batch-suggestion-copied = In die Zwischenablage kopiert.

batch-run = Ausführen
batch-run-tooltip = Diesen Stapel starten: jede aufgelöste Datei mit mkvmerge multiplexen.
batch-run-tooltip-no-profile = Vor dem Ausführen ein Profil auswählen und prüfen.
batch-run-tooltip-errors = Vor dem Ausführen jede Diagnose mit Schweregrad Fehler beheben.
batch-run-tooltip-mkvmerge-missing = mkvmerge ist nicht verfügbar; korrigiere die Erkennung in den Einstellungen, bevor du ausführst.
batch-run-tooltip-run-active = Es läuft bereits ein Stapel. Warte, bis er fertig ist, oder wechsle zur Jobs-Ansicht, um ihn abzubrechen.
