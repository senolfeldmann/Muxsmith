# Kapitel

`Kapitel` steuert die Kapitelspur jeder Ausgabe: ein Schlüsselwort oder eine externe Spender-Datei.

## Schlüsselwörter

- `keep` übernimmt die Kapitel der Quelldatei unverändert.
- `drop` entfernt sie aus der Ausgabe.

## Externe Kapitel

Statt eines Schlüsselworts schaltet das Feld auf einen externen Verweis um: die Kapitel kommen dann aus einer eigenen Datei auf der Platte, eine pro Quelldatei. Der Verweis funktioniert genau wie eine externe Spurquelle - ein Pfad (relativ zum Verzeichnis der Quelldatei oder absolut), eine Erweiterungsliste und die Zuordnung über `Match zur Quelle` oder ein `Match-Muster` (siehe diese Themen).

- Der Verweis muss pro Quelldatei genau eine Kapiteldatei ergeben: keine ist ein Fehler wegen fehlender externer Datei, zwei oder mehr ein Mehrdeutigkeitsfehler.
- Der Inhalt der Datei ist alles, was mkvmerge als Kapitel akzeptiert: Matroska-Kapitel-XML oder das einfache Kapitelformat.

## Wann was

`keep`, wenn die Quellen korrekte Kapitel tragen; `drop`, wenn ihre Kapitel falsch oder unerwünscht sind - das Profil bearbeitet Kapitelinhalte nicht, es leitet sie nur. Der Spender-Weg deckt den Fall ab, dass Kapitel außerhalb des Videos liegen (heruntergeladen, generiert oder pro Episode erstellt) und im selben Durchgang eingemuxt werden sollen.
