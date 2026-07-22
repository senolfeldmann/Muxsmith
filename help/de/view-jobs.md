# Jobs-Ansicht

Die Jobs-Ansicht zeigt den gerade laufenden Lauf und den Verlauf vergangener Läufe. Gestartet wird ein Lauf in der Stapel-Ansicht; ab dem Start übernimmt diese Ansicht.

## Lebenszyklus eines Laufs

Jeder Job multiplext eine Ausgabedatei mit mkvmerge. Ein Job beginnt in der Warteschlange, läuft dann mit Live-Fortschritt und endet in einem von vier Zuständen: fertig, fertig mit Warnungen, fehlgeschlagen oder abgebrochen. Die Kopfzeile zählt abgeschlossene Jobs gegen die Gesamtzahl; nach dem Lauf zählt die Zusammenfassungszeile ok, Warnung, fehlgeschlagen und abgebrochen.

## Abbrechen

- **Stapel abbrechen** stoppt den ganzen Lauf: Noch eingereihte Jobs werden als abgebrochen markiert, jeder gerade multiplexende Job wird beendet.
- **Abbrechen** in einer einzelnen Zeile bricht nur diesen Job ab: Läuft er, wird er sofort beendet; steht er noch in der Warteschlange, wird er übersprungen, wenn er an der Reihe wäre. Der Rest des Laufs geht weiter.

Bereits abgeschlossene Jobs behalten in beiden Fällen ihr Ergebnis.

## Live-Protokoll

Das Live-Protokoll zeigt die Ausgabe von mkvmerge, sobald sie eintrifft, und lässt sich auf einen einzelnen Job filtern. Die Anzeige behält nur die neuesten Zeilen; das vollständige Protokoll jedes Jobs wird unabhängig davon auf die Festplatte geschrieben.

## Verlauf und Protokoll-Export

Der Lauf-Verlauf listet vergangene Läufe von der Festplatte, mit Startzeit und den Ergebnissen pro Job. Zu jedem Job kannst du das vollständige Protokoll anzeigen, in die Zwischenablage kopieren oder in eine Datei speichern. Meldet ein Hinweis nach einem Lauf, dass sein Protokoll nicht oder nicht vollständig geschrieben werden konnte, fehlt dieser Lauf im Verlauf oder seine Job-Einträge sind unvollständig.
