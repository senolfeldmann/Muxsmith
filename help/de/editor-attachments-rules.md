# Regeln für Anhänge

Eine geordnete Liste von Regeln über die Anhänge der Quelldatei. Jede Regel tut genau eines von drei Dingen:

- `Auswählen` behält die Anhänge, die ihr Ausdruck matcht.
- `Verwerfen` entfernt die Anhänge, die ihr Ausdruck matcht.
- `Hinzufügen` hängt eine externe Datei von der Platte an, gefunden über einen Verweis (Pfad, Erweiterungen, Zuordnung - derselbe Mechanismus wie bei externen Spurquellen).

## Die erste passende Regel gewinnt

Regeln werden in Listenreihenfolge ausgewertet, und für jeden Anhang entscheidet die erste Regel, deren Ausdruck matcht, über sein Schicksal. Anhänge, die keine Regel matcht, fallen auf die Einstellung `Nicht zugeordnet` zurück - siehe dieses Thema. Ziehe Zeilen zum Umordnen: setze eine spezifische `Verwerfen`-Regel über ein breites `Auswählen`, sonst greift das Auswählen zuerst.

## Mengen statt Einzeltreffer

Anders als Spurregeln sind Anhangsregeln nicht auf Eindeutigkeit beschränkt: eine Regel darf viele Anhänge matchen - Schriftarten kommen im Satz, und ein `Auswählen` kann sie alle behalten. Auch null Treffer sind kein Fehler; eine Regel, die nichts matcht, tut schlicht nichts.

## Matching

Ausdrücke nutzen dieselbe Match-Algebra wie Spurregeln (`exact`, `substring`, `regex`, `any`, `not` - siehe das Thema Match), über drei Anhangseigenschaften: `file_name`, `content_type`, `description`. Beispiel: alle Schriftarten behalten mit einem `Auswählen`, dessen Ausdruck `substring: { content_type: font }` ist.
