# Match-Muster (Externer Verweis)

`Match-Muster` ordnet externe Spender-Dateien einer Quelldatei per Vorlage zu - für den Fall, dass die Spender den Bezeichner anders schreiben als die Quelldateien. Es ist die dritte Textsyntax des Profils: kein reiner Regex wie das Eingabemuster, keine wörtliche Vorlage wie der Ausgabedateiname, sondern eine Vorlage im Regex-Modus. Beides wird leicht verwechselt; die Render-Regel unten ist der Unterschied.

## Wie es gerendert wird

Vorlagenfelder setzen die aus dem Namen der Quelldatei eingefangenen Werte ein - und jeder Wert wird regex-escaped eingefügt, als Literal. Nur der Text, den du um die Felder herum schreibst, ist ein regulärer Ausdruck. Das gerenderte Ganze wird gegen jeden Kandidaten-Dateinamen gesucht; die Suche ignoriert Groß-/Kleinschreibung, solange `Groß-/Kleinschreibung beachten` nicht gesetzt ist.

- Felder: `{match}` (der ganze Bezeichner), benannte Gruppen wie `{season}`, nummerierte Gruppen `{g1}`, `{g2}`.
- Filter: `{season}` behält die eingefangene Schreibweise (`03`), `{season:int}` entfernt führende Nullen (`3`), `{season:pad2}` / `{season:pad3}` füllen mit Nullen auf.
- `{source_stem}` steht hier nicht zur Verfügung; es existiert nur im Literal-Modus.

## Beispiel

Für eine Quelldatei, die als `S03E01` mit den Gruppen `season` und `episode` gematcht wurde, matcht das Muster `staffel0*{season:int}episode0*{episode:int}` die Namen `staffel03episode01`, `staffel3episode01` und `Staffel3Episode1`.

## Kein reiner Regex

Die geschweiften Klammern sind Vorlagenfelder, keine Regex-Syntax, und eingesetzte Werte können nie als Regex-Fragmente wirken - ein eingefangenes `S03E01` wird escaped, nie interpretiert. Schreibe Regex-Konstrukte (`0*`, Alternativen, Zeichenklassen) nur in den umgebenden Text.

## Wechselwirkungen

- Schließt sich mit `Match zur Quelle` gegenseitig aus; das Flag ist exakt die Kurzform für ein Muster `{match}` - bevorzuge es, wenn Spender und Quelldateien dasselbe Namensschema teilen.
- Nach der Dateizuordnung wählt der Match-Ausdruck der Regel genau eine Spur innerhalb der Datei aus. Zwei passende Dateien sind ein Mehrdeutigkeitsfehler; keine ist ein Fehler wegen fehlender externer Datei - außer die Regel ist `Optional`. Diesen Ausweg gibt es nur bei Spurregeln (externe Kapitel und `Hinzufügen`-Verweise bei Anhängen haben keinen).
