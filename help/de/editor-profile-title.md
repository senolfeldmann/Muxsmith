# Titel

`Titel` steuert den Containertitel jeder Ausgabe - den Namen, den Player für die Datei anzeigen, verschieden vom Namen einzelner Spuren. Das Feld nimmt ein Schlüsselwort oder eine Vorlage.

## Schlüsselwörter

- `keep` behält den Containertitel der Quelldatei.
- `clear` leert ihn.

## Vorlage

Schalte das Feld auf eine Vorlage um, um pro Datei einen neuen Titel zu bauen. Vorlagen rendern im Literal-Modus: Felder werden als einfacher Text eingesetzt, nichts wird escaped oder als Muster interpretiert.

- Felder: `{match}` (der ganze gematchte Bezeichner), benannte Capture-Gruppen aus dem Eingabemuster wie `{season}`, nummerierte Gruppen `{g1}` und `{source_stem}` (der Dateiname der Quelldatei ohne Erweiterung).
- Filter: `{season}` behält die eingefangene Schreibweise (`03`), `{season:int}` entfernt führende Nullen, `{season:pad2}` / `{season:pad3}` füllen mit Nullen auf.

Beispiel: `Show S{season}E{episode}` ergibt `Show S03E01` für eine Quelldatei, die als `S03E01` gematcht wurde.

## Wechselwirkungen

- Die Vorlagenfelder stammen aus den Capture-Gruppen des Eingabemusters - benenne dort Gruppen, um sie hier zu verwenden. Siehe das Thema Muster.
- Quelltitel tragen häufig Release-Group- oder Encoder-Reste; `clear` gibt jeder Ausgabe einen sauberen Stand, `keep` passt nur bei gepflegten Quellen, und eine Vorlage vereinheitlicht die Titel über den ganzen Stapel.
