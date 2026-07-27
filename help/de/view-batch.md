# Stapel-Ansicht

Die Stapel-Ansicht führt ein Profil von der Auswahl bis zur Ausführung: Profil wählen, Prüfung ansehen, Probelauf gegen echte Dateien, dann starten.

- **Profil wählen.** Wähle eine Muxsmith-Profil-YAML-Datei aus oder öffne eines aus der Liste der zuletzt verwendeten Profile erneut. Beim Auswählen wird das Profil sofort geprüft; das Ergebnis erscheint unter Meldungen.
- **Verzeichnisse.** Das Quellverzeichnis wird nach Eingabedateien durchsucht; in das Ausgabeverzeichnis werden die gemultiplexten Dateien geschrieben. Lass ein Feld leer, um das im Profil hinterlegte Verzeichnis zu verwenden. Beide Angaben werden pro Profil gemerkt und beim nächsten Auswählen wiederhergestellt.
- **Probelauf** löst jede Spurregel gegen die tatsächlich gefundenen Dateien auf und erzeugt den vollständigen Bericht unten, ohne etwas zu multiplexen.
- **Starten** beginnt den Stapel; Ausführung und Fortschritt übernimmt die Jobs-Ansicht.

## Die Auflösungstabelle

Nach einem Probelauf erhält jede passende Datei eine eigene Tabelle: eine Zeile pro Spurregel, in Profil-Reihenfolge, mit der Spur, auf die die Regel aufgelöst wurde (Spur-ID und Typ). Ein `-` in der Spalte "Aufgelöste Spur" bedeutet: Die Regel passt auf keine Spur dieser Datei. Für eine mit `optional` markierte Regel ist das normal, für jede andere trägt der Bericht eine Meldung. Eine Datei ohne Tabelle hat gar keinen Plan erhalten; die Meldungen nennen den Grund.

## Meldungen

Meldungen haben drei Schweregrade: Fehler, Warnungen und Hinweise. Die Zusammenfassungszeile zählt sie über Profil, Stapel und alle Dateien. Fehler blockieren das Starten; Warnungen und Hinweise nicht. Schlägt der Bericht konkrete Korrekturen vor, erscheinen sie als Vorschlagskarten unter der Dateiliste (siehe das Thema Vorschlagskarte).

## Wann Starten gesperrt ist

Starten bleibt gesperrt, solange kein geprüftes Profil ausgewählt ist, solange ein Fehler besteht, solange mkvmerge nicht verfügbar ist oder solange bereits ein anderer Lauf aktiv ist; der Tooltip der Schaltfläche nennt den aktuellen Grund. Ein Probelauf vorab ist nicht nötig: Starten plant intern mit dem aktuellen Profil und den aktuellen Verzeichnissen.
