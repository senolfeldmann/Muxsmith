# Muster (Eingabe)

Das Eingabemuster ist ein regulärer Ausdruck, den Muxsmith unverankert im Basisnamen jeder Kandidatendatei sucht. Der erste Treffer wird die *Kennung* der Datei: die Zeichenkette, die benennt, welche Episode oder Einheit diese Datei im Stapel darstellt.

Die Kennung steuert zwei Dinge:

- **Spender-Zuordnung.** Ein externer Verweis mit aktiviertem `match_to_source` verlangt die Kennung der Primärquelle im Namen der Spender-Datei, und das `match_pattern` eines Verweises baut seine Suche auf den Capture-Feldern der Kennung auf. So findet eine externe Untertitel- oder Audiodatei die Primärquelle, zu der sie gehört; siehe das Thema Quelle.
- **Vorlagenfelder.** Benannte Capture-Gruppen (zum Beispiel `(?<season>\d{2})`) und nummerierte Gruppen werden zu Feldern wie `{season}` oder `{g1}`, verfügbar in der Dateinamen-Vorlage, der Titel-Vorlage und im Match-Muster eines Verweises; `{match}` trägt die ganze Kennung. Siehe das Thema Vorlage.

Diagnosen, die du kennen solltest:

- Eine Datei, deren Erweiterung passt, deren Basisname aber nicht auf das Muster passt, wird übersprungen und als `IgnoredFile` (Info) gemeldet. Ein zu enges Muster bleibt so sichtbar, statt den Stapel still zu verkleinern.
- Passt das Muster mehr als einmal in einen Basisnamen, wird der erste Treffer verwendet und `MultipleIdentifierMatches` (Info) gemeldet.
- Zwei Primärquellen mit derselben Kennung (eine 720p- und eine 1080p-Kopie derselben Episode) werden beide gemuxt und ziehen beide dieselben externen Dateien an; `DuplicateIdentifier` (Warnung) weist darauf hin, weil Ausgabe-Vorlagen dann kollidieren können.

Beim Schreiben: Erfasse genau den Teil des Namens, der die Einheit identifiziert, zum Beispiel `S(?<season>\d{2})E(?<episode>\d{2})` für eine Serie. Benenne die Gruppen, die du in Vorlagen verwenden willst; alles außerhalb des Treffers spielt für die Identität keine Rolle.
