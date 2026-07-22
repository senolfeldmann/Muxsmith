# Nicht zugeordnet (Anhänge)

`Nicht zugeordnet` entscheidet, was mit Anhängen geschieht, die keine Anhangsregel matcht: `keep` oder `drop`. Der Standard ist `keep` - bewusst das Gegenteil des Spuren-Standards.

## Warum der Standard behält

Anhänge sind Begleitmaterial, meist Schriftarten - und verworfene Schriftarten machen die Darstellung von ASS/SSA-Untertiteln beim Abspielen still kaputt, ohne Fehler an irgendeiner Stelle der Kette. `keep` macht das sichere Verhalten zum Standard: Anhänge laufen durch, solange du sie nicht ausdrücklich behandelst.

## `drop`

Die Ausgabe enthält nur die Anhänge, die eine `Auswählen`-Regel gematcht hat, plus Dateien aus `Hinzufügen`-Regeln. Nimm `drop`, um Cover oder anderes unerwünschtes Beiwerk gezielt zu entfernen - und schreibe dann Auswahlregeln für die Schriftarten, die deine Untertitel brauchen, sonst sind sie weg.

## Wechselwirkungen

- Zuerst laufen die Regeln, in Listenreihenfolge, die erste passende Regel gewinnt pro Anhang; nur Anhänge, die keine Regel berührt hat, fallen auf diese Einstellung zurück. Siehe das Thema Regeln für Anhänge.
- Dasselbe Bedienelement wie `Nicht zugeordnet` bei den Spuren, aber andere Domänensemantik: Spuren stehen standardmäßig auf `drop`, weil die Ausgabebesetzung meist definiert sein soll; Anhänge auf `keep`, weil sie Begleitmaterial sind und ihr Verlust still scheitert.
