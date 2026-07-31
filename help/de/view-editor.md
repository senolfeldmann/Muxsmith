# Editor-Ansicht

Der Editor bearbeitet ein Profil als strukturiertes Modell, nicht als YAML-Text. Lege ein neues Profil an, öffne eine Profildatei oder eines der zuletzt verwendeten Profile; jedes Feld erscheint als Formularelement, die Spurregeln als Tabelle.

## Ein Profil erstellen

Neues Profil legt ein Profil mit einer Kandidaten-Dateiendung und einer leeren Regel an; die Prüfung meldet die leere Regel als Warnung, nicht als Fehler - das Profil ist damit unvollständig, nicht falsch.

Der Editor hält jeweils ein Profil. Ersetzt du es, sei es durch Anlegen eines neuen oder durch Öffnen eines vorhandenen, warnt der Editor zuerst, solange das aktuelle Profil nicht gespeicherte Änderungen hat; lehnst du das ab, bleibt es unverändert. Ein Wechsel zu einer anderen Ansicht und zurück rührt es nie an. Auch beim Beenden der Anwendung wird bei nicht gespeicherten Änderungen gewarnt.

Jede Änderung lässt sich rückgängig machen. Rückgängig und Wiederholen stehen in der Aktionsleiste; dieselben Aktionen reagieren auch auf Tastenkürzel: Strg+Z (Cmd+Z unter macOS) für Rückgängig, Strg+Umschalt+Z oder Strg+Y (Cmd+Umschalt+Z oder Cmd+Y unter macOS) für Wiederholen.

## Das Modell bearbeiten

Bearbeitet wird das aus der Datei geparste Modell im Speicher, nie der Dateitext selbst. Die Regel-Tabelle fasst jede Spurregel zusammen (Quelle, Match, Optional, Änderungen); die Reihenfolge der Regeln ist zugleich die Reihenfolge der Ausgabespuren, und per Ziehen einer Zeile ordnest du die Regeln um. Wählst du eine Zeile aus, öffnet sich unter der Tabelle ein Detailbereich mit allen Feldern dieser Regel. Die übrigen Abschnitte (Metadaten, Eingabe, Ausgabe, Anhänge, Kapitel, Tags, Titel) erscheinen als normale Formularabschnitte.

## Prüfung bei jeder Änderung

Jede Änderung prüft das ganze Profil im Hintergrund neu; der Meldungsbereich beschreibt daher immer den aktuellen Stand des Modells. Speichern ist gesperrt, solange eine Meldung mit Schweregrad Fehler besteht; das ist die einzige Sperre, die der Editor durchsetzt. Warnungen und Hinweise blockieren das Speichern nie.

## Speicherverhalten

Bis zum Speichern wird nichts auf die Festplatte geschrieben. Hat das Profil noch keinen Pfad, öffnet der erste Speichervorgang einen Dialog, der nach dem Zielort fragt; jeder weitere Speichervorgang für dieses Profil schreibt danach direkt dorthin, ohne erneuten Dialog.

Speichern schreibt die Datei komplett aus dem Modell neu: Kommentare, Schlüsselreihenfolge und Formatierung der Datei auf der Festplatte bleiben nicht erhalten, und Felder auf ihrem Standardwert werden weggelassen statt zurückgeschrieben. Das Format folgt der Dateiendung; ein YAML-Profil bleibt YAML. Ist dir ein von Hand kommentiertes Profil wichtig, lege vorher eine Kopie an oder stelle es unter Versionskontrolle.

Die Schaltfläche Anwenden auf einer Vorschlagskarte in der Stapel-Ansicht speichert auf dieselbe Weise: ein kanonisches Neuschreiben, dieselben Regeln (siehe das Thema Vorschlagskarte).
