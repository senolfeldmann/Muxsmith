# Bei Kollision (Ausgabe)

Diese Richtlinie bestimmt, was passiert, wenn ein gerenderter Ausgabepfad auf der Platte bereits als vorhandene Datei existiert. Die drei Werte verhalten sich deutlich unterschiedlich:

- **`error`** (Standard): Die betroffene Datei erhält keinen Plan, die Kollision wird als Fehler gemeldet. Nichts wird überschrieben, nichts still übersprungen.
- **`skip`**: Die betroffene Ausgabe entfällt mit einer Warnung; der Rest des Stapels läuft normal weiter.
- **`overwrite`**: Der Plan bleibt bestehen, die vorhandene Datei wird beim Lauf ersetzt; gemeldet als Info.

Was diese Richtlinie *nicht* regelt:

- Ein Ausgabepfad, der einem Eingabepfad entspricht - Primärquelle oder Spender-Datei -, ist unter jeder Richtlinie ein harter Fehler (`SourceOverwrite`). Muxsmith überschreibt nie seine eigenen Quellen.
- Rendern zwei geplante Ausgaben desselben Stapels auf denselben Pfad, ist das unabhängig von der Richtlinie immer ein Fehler (`OutputCollision`): Weder `skip` noch `overwrite` könnte festlegen, welcher der beiden Pläne gewinnt. Behebe stattdessen die Benennung - mach die Dateinamen-Vorlage oder das Eingabemuster eindeutig.

Wann welcher Wert:

- Lass `error` stehen, solange sich ein Profil noch einspielt: Es macht Benennungsfehler sichtbar, statt auf sie zu reagieren.
- Nutze `skip` für inkrementelle Wiederholungsläufe über ein wachsendes Verzeichnis: Bereits gemuxte Episoden bleiben unangetastet, nur neue Dateien erzeugen Ausgaben.
- Nutze `overwrite`, wenn du einen Stapel nach einer Profiländerung bewusst neu erzeugst und die vorhandenen Ausgaben per Definition veraltet sind.

Ein Probelauf zeigt die Kollisionsdiagnosen, ohne eine Datei anzufassen - so prüfst du vorab, was eine Richtlinie tun würde.
