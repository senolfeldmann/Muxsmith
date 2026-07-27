# Regeln (Spuren)

Das Regelraster ist das Herz eines Profils: eine geordnete Liste von Regeln, von denen jede genau eine Spur pro Quelldatei auswählt und optional deren Eigenschaften ändert. Jede Rasterzeile fasst eine Regel zusammen - Quelle, Match-Ausdruck, Optional-Flag, Änderungen.

## Reihenfolge ist Ausgabereihenfolge

Die Zeilenreihenfolge definiert die Spurreihenfolge der Ausgabe. Ziehe eine Zeile, um sie umzuordnen; die neue Rasterreihenfolge ist die neue Ausgabereihenfolge. Eine Einschränkung: unter `Nicht zugeordnet: keep` behalten die Quellspuren ihre Quellreihenfolge am Anfang der Ausgabe, nur Spender-Spuren folgen in Regelreihenfolge - siehe das Thema Nicht zugeordnet (Spuren).

## Eine Regel bearbeiten

Klicke eine Zeile an, um sie auszuwählen; das Detailpanel unter dem Raster bearbeitet die ausgewählte Regel - ihre Quelle (die Quelldatei oder ein externer Spender), ihren Match-Ausdruck, das Optional-Flag und die Eigenschaftsänderungen.

Die Schaltfläche Hinzufügen hängt eine neue, leere Regel am Ende der Liste an, wählt sie aus und öffnet ihr Detailpanel. Eine Warnung im Detailpanel markiert die neue Regel, bis du ihren Match-Ausdruck ausgefüllt hast.

Die Schaltfläche Entfernen löscht die ausgewählte Regel; die Schaltfläche bleibt gesperrt, solange keine Zeile ausgewählt ist. Entfernen verlangt keine Bestätigung - wie jede andere Änderung im Editor betrifft es nur das Modell, und die Datei auf der Festplatte ändert sich erst, wenn du speicherst (siehe das Thema Editor-Ansicht).

## Genau eine Spur pro Regel

Jede Regel muss pro Quelldatei genau eine Spur ergeben. Zwei Kandidaten sind ein Mehrdeutigkeitsfehler - verenge den Match-Ausdruck (die Vorschlagskarten in der Stapel-Ansicht können dir das Verengen abnehmen). Null Kandidaten sind ebenfalls ein Fehler, außer die Regel ist `Optional` - siehe das Thema Optional.

## Wann die Liste leer sein darf

Eine leere Regelliste ist nur unter `Nicht zugeordnet: keep` erlaubt - ein reiner Passthrough-Remux. Unter `drop` ist mindestens eine Regel erforderlich, sonst würde jede Spur verworfen. Auch die letzte Regel darf entfernt werden; die dann leere Liste ist genau der hier beschriebene Zustand.
