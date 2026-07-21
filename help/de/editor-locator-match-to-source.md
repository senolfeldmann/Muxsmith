# Match zur Quelle

`Match zur Quelle` ordnet externe Spender-Dateien der Quelldatei zu, zu der sie gehören. Das Bedienelement ist ein Flag, das entweder gesetzt ist oder im Profil fehlt - ein explizites `false` gibt es nicht; abwählen entfernt es.

## Was es tut

Ist das Flag gesetzt, kommt eine Spender-Datei nur infrage, wenn ihr Dateiname den gematchten Bezeichner der Quelldatei enthält - den Text, den das Eingabemuster eingefangen hat, etwa `S03E01`. Es ist exakt die Kurzform für ein `Match-Muster` mit dem Inhalt `{match}`.

Beispiel: die Quelldatei `Show S03E01.mkv` wurde als `S03E01` gematcht; in einem Verzeichnis voller Untertiteldateien passen nur Namen, die `S03E01` enthalten (etwa `Show.S03E01.tr.srt`). Der Vergleich ignoriert Groß-/Kleinschreibung, solange `Groß-/Kleinschreibung beachten` nicht gesetzt ist.

## Wechselwirkungen

- Schließt sich mit `Match-Muster` gegenseitig aus: setze das eine oder das andere. Schreiben die Spender den Bezeichner anders als die Quelldateien (etwa `staffel3episode1` statt `S03E01`), wähle das Flag ab und schreibe stattdessen ein Muster - siehe das Thema Match-Muster.
- Die Dateizuordnung ist nur die erste Stufe: der Match-Ausdruck der Regel wählt danach genau eine Spur innerhalb der zugeordneten Spender-Datei aus.
- Eindeutigkeit gilt auf Dateiebene: passen zwei Spender-Dateien zu einer Quelldatei, ist das ein Mehrdeutigkeitsfehler; passt keine, ein Fehler wegen fehlender externer Datei - außer die Regel ist `Optional`.

## Wann einsetzen

Immer dann, wenn die Spender demselben Namensschema folgen wie die Quelldateien - der Normalfall bei Sidecar-Untertiteln oder einem zweiten Release derselben Serie. Ohne das Flag (und ohne Match-Muster) ist jede vom Verweis gefundene Datei ein Kandidat; das funktioniert nur, wenn der Verweis genau eine Datei findet.
