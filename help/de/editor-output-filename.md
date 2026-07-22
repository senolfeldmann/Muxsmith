# Dateiname (Ausgabe)

Dieses Feld bestimmt, wie jede Ausgabedatei heißt. Zwei Modi:

- **`keep`**: Die Ausgabe behält den Basisnamen der Quelldatei; als Erweiterung wird `.mkv` erzwungen.
- **Vorlage**: Eine Vorlage im Literalmodus rendert den Namen aus den Kennungsfeldern der Datei, zum Beispiel `Show - S{season}E{episode}.mkv`. Eine fehlende `.mkv`-Erweiterung wird automatisch angehängt. Felder und Filter sind die der Vorlagen-Engine; siehe das Thema Vorlage.

Zwei Regeln werden auf dem *gerenderten* Namen geprüft (nicht nur auf dem Vorlagentext), auf jeder Plattform gleich:

- Ein Pfadtrenner (`/` oder `\`) im gerenderten Namen ist ein Fehler (`PathSeparatorInRenderedName`); der Dateiname kann in v1 keine Unterverzeichnisse anlegen.
- Ein leerer Stamm, `.` oder `..` ist ein Fehler (`EmptyRenderedName`) - typischerweise eine Vorlage, deren Felder alle leer gerendert wurden.

Kollisionsfolgen:

- Rendern zwei geplante Ausgaben auf denselben Pfad, ist das immer ein Fehler (`OutputCollision`), unabhängig von der Kollisionsrichtlinie: Der Stapel ist in sich widersprüchlich, und keine Richtlinie könnte festlegen, welcher Plan gewinnt. Mach die Vorlage oder das Eingabemuster eindeutig.
- Ein gerenderter Pfad, der auf der Platte bereits als vorhandene Datei existiert, unterliegt der Richtlinie Bei Kollision; siehe dieses Thema für die drei Verhaltensweisen.
- Ein gerenderter Pfad, der einem Eingabepfad entspricht, ist immer ein harter Fehler (`SourceOverwrite`).

Wann welcher Modus: `keep`, wenn die Quellnamen bereits stimmen und sich nur der Inhalt ändert - der übliche Fall beim Aufräumen einer Bibliothek in ein eigenes Ausgabeverzeichnis. Eine Vorlage, wenn du einen ganzen Stapel auf ein einheitliches Namensschema normalisierst; genau dafür gibt es die Capture-Gruppen des Eingabemusters.
