# Editor-Ansicht

Der Editor bearbeitet ein Profil als strukturiertes Modell, nicht als YAML-Text. Öffne eine Profildatei oder eines der zuletzt verwendeten Profile; jedes Feld erscheint als Formularelement, die Spurregeln als Tabelle.

## Das Modell bearbeiten

Bearbeitet wird das aus der Datei geparste Modell im Speicher, nie der Dateitext selbst. Die Regel-Tabelle fasst jede Spurregel zusammen (Quelle, Match, Optional, Änderungen); die Reihenfolge der Regeln ist zugleich die Reihenfolge der Ausgabespuren, und per Ziehen einer Zeile ordnest du die Regeln um. Wählst du eine Zeile aus, öffnet sich unter der Tabelle ein Detailbereich mit allen Feldern dieser Regel. Die übrigen Abschnitte (Metadaten, Eingabe, Ausgabe, Anhänge, Kapitel, Tags, Titel) erscheinen als normale Formularabschnitte.

## Prüfung bei jeder Änderung

Jede Änderung prüft das ganze Profil im Hintergrund neu; der Meldungsbereich beschreibt daher immer den aktuellen Stand des Modells. Speichern ist gesperrt, solange eine Meldung mit Schweregrad Fehler besteht; das ist die einzige Sperre, die der Editor durchsetzt. Warnungen und Hinweise blockieren das Speichern nie.

## Speicherverhalten

Speichern schreibt die Datei komplett aus dem Modell neu: Kommentare, Schlüsselreihenfolge und Formatierung der Datei auf der Festplatte bleiben nicht erhalten, und Felder auf ihrem Standardwert werden weggelassen statt zurückgeschrieben. Das Format folgt der Dateiendung; ein YAML-Profil bleibt YAML. Ist dir ein von Hand kommentiertes Profil wichtig, lege vorher eine Kopie an oder stelle es unter Versionskontrolle.

Die Schaltfläche Anwenden auf einer Vorschlagskarte in der Stapel-Ansicht speichert auf dieselbe Weise: ein kanonisches Neuschreiben, dieselben Regeln (siehe das Thema zur Vorschlagskarte).
