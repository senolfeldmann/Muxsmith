# Optional (Spurregel)

Optional deckt genau einen Fall ab: Die Regel darf *null* Spuren treffen, ohne dass die Datei scheitert. Sonst ändert sich nichts - zwei oder mehr passende Spuren bleiben auch bei einer optionalen Regel ein `AmbiguousRule`-Fehler. Optional heißt "darf fehlen", nie "locker gematcht".

- **Aus** (Standard): Die Regel ist Pflicht. Null Treffer sind ein `MissingTrack`-Fehler, und sein Hinweis listet Beinahe-Treffer - Spuren gleichen Typs oder gleicher Sprache samt der jeweils verfehlten Bedingung -, sodass du siehst, ob die Regel falsch ist oder die Spur wirklich fehlt.
- **An**: Null Treffer lassen diese Spur in der Ausgabe der Datei einfach weg, ohne Diagnose. Bei einer externen Quelle wird ein Verweis, der keine Spender-Datei findet, genauso toleriert (eine Pflichtregel würde `MissingExternal` melden).

Wann verwenden: für Spuren, die legitim nur in einem Teil der Dateien existieren - Forced-Untertitel, die nur manche Episoden tragen, eine Kommentarspur auf ausgewählten Releases. Lass eine Regel überall dort Pflicht, wo Fehlen ein Defekt wäre, den du sehen willst: Eine fehlende Hauptaudiospur soll die Datei laut scheitern lassen, nicht still eine dünnere Ausgabe erzeugen.

Wechselwirkungen:

- Optional lockert die Eindeutigkeit nicht und ist darum nie eine Lösung für Mehrdeutigkeit. Trifft eine Regel zwei Spuren, schlägt die Vorschlags-Engine stattdessen einschränkende Verfeinerungen vor; und wenn keine einzelne Verfeinerung den Konflikt für den ganzen Stapel löst, gruppiert der Bericht die betroffenen Dateien nach der Korrektur, die jede Gruppe braucht (`SuggestionPartition`). Der Schalter Optional ändert daran nichts.
- Die Änderungen einer optionalen Regel greifen nur, wenn tatsächlich eine Spur getroffen wurde; bei null Treffern wird nichts angewendet, und die Ausgabe-Spurreihenfolge schließt die Lücke einfach.
