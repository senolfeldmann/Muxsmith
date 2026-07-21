# Vorlage

Der Vorlagentext rendert eine Zeichenkette aus den Feldern, die das Eingabemuster eingefangen hat. Eine Engine bedient alle Vorlagenflächen: hier im *Literalmodus* für Ausgabe-Dateiname und Titel, und im *Regex-Modus* für das Match-Muster eines Verweises (dieser Modus hat ein eigenes Thema; siehe das Thema Match-Muster).

Felder:

- `{match}`: die ganze Kennung, genau wie das Eingabemuster sie getroffen hat.
- `{season}`: eine benannte Capture-Gruppe, roh wie eingefangen (eine Gruppe, die `03` traf, rendert `03`).
- `{g1}`, `{g2}`: nummerierte Capture-Gruppen, für Muster ohne benannte Gruppen.
- `{source_stem}`: der Basisname der Primärdatei ohne Erweiterung. Nur im Literalmodus.

Filter, nach einem Doppelpunkt innerhalb der Klammern:

- `{season:int}` entfernt führende Nullen: `03` rendert als `3`.
- `{season:pad2}` und `{season:pad3}` füllen mit Nullen auf zwei bzw. drei Stellen auf: `3` rendert als `03` bzw. `003`.

Die Filter führen unterschiedliche Quellkonventionen auf eine kanonische Ausgabe zusammen: Ob eine Episode als `1` oder `01` eingefangen wurde, `{episode:pad2}` rendert immer `01`.

Im Literalmodus werden die Feldwerte als reiner Text eingesetzt, alles drumherum bleibt wörtlich erhalten. Beispiel: `Show - S{season:pad2}E{episode:pad2}.mkv` rendert `Show - S03E01.mkv` für eine Datei mit der Kennung `S03E01`.

Wechselwirkungen:

- Die verfügbaren Felder sind genau die Capture-Gruppen des Eingabemusters. Benennst du dort eine Gruppe um, ändern sich die Feldnamen für jede Vorlage; ein unbekanntes Feld in einer Vorlage ist ein Validierungsfehler und wird erkannt, bevor irgendeine Datei angefasst wird.
- Für Ausgabe-Dateinamen darf das gerenderte Ergebnis keinen Pfadtrenner enthalten und nicht leer sein; die Prüfungen und die Kollisionsfolgen stehen im Thema Dateiname.
