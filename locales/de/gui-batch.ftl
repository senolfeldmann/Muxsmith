## German (de) translation of locales/en/gui-batch.ftl. The en catalog is
## the source of truth; keys mirror it (id parity enforced by
## scripts/check-i18n.mjs); placeables and selector structure mirror it by
## convention (reviewed manually, not machine-checked). Batch -> Stapel,
## Regel, Vorschlag, Probelauf (dry run); GUI imperative in infinitive form.

batch-view-heading = Stapel

batch-profile-heading = Profil
batch-profile-pick = Profil auswählen...
    .tooltip = Eine Muxsmith-Profil-YAML-Datei zum Prüfen und Starten auswählen.
batch-profile-filter-name = Muxsmith-Profile
batch-profile-none = Noch kein Profil ausgewählt. Wähle unten eines aus, um es zu prüfen und einen Stapel zu starten.
batch-profile-current = Ausgewähltes Profil: { $path }

batch-recents-heading = Zuletzt verwendete Profile
batch-recents-empty = Noch keine zuletzt verwendeten Profile.
batch-recents-select =
    .tooltip = Dieses Profil erneut öffnen.

batch-dirs-heading = Verzeichnisse
batch-source-label = Quellverzeichnis
    .hint = Verzeichnis, das nach Eingabedateien durchsucht wird. Leer lassen, um das im Profil hinterlegte Quellverzeichnis zu verwenden.
batch-output-label = Ausgabeverzeichnis
    .hint = Verzeichnis, in das die gemultiplexten Dateien geschrieben werden. Leer lassen, um das Ausgabeverzeichnis des Profils zu verwenden.

batch-diagnostics-heading = Meldungen
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
    .tooltip = Jede Spurregel auflösen und den Bericht unten erzeugen, ohne etwas zu multiplexen.

batch-files-heading = Dateien
batch-resolution-rule-header = Regel
batch-resolution-track-header = Aufgelöste Spur
batch-resolved-track = { $id } ({ $kind })
batch-file-caption = { $source } (Kennung: { $identifier }) -> { $output }
batch-file-no-plan = { $source } (Kennung: { $identifier }): kein Plan erzeugt; siehe Meldungen unten.

batch-suggestions-heading = Vorschläge
batch-suggestion-header = Vorschlag für { $config_path }:
batch-suggestion-copy = Kopieren
    .tooltip = Dieses YAML-Fragment in die Zwischenablage kopieren.
batch-suggestion-copied = In die Zwischenablage kopiert.
batch-suggestion-apply = Anwenden
    .tooltip = Diese Korrektur auf das Profil anwenden und speichern.

batch-run = Starten
    .tooltip = Diesen Stapel starten: jede aufgelöste Datei mit mkvmerge multiplexen.
    .tooltip-no-profile = Vor dem Starten ein Profil auswählen und prüfen.
    .tooltip-errors = Vor dem Starten jede Meldung mit Schweregrad Fehler beheben.
    .tooltip-mkvmerge-missing = mkvmerge ist nicht verfügbar; korrigiere die Erkennung in den Einstellungen, bevor du startest.
    .tooltip-run-active = Es läuft bereits ein Stapel. Warte, bis er fertig ist, oder wechsle zur Jobs-Ansicht, um ihn abzubrechen.
