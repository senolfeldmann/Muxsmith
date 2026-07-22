# Erweiterungen (Eingabe)

Die Erweiterungsliste entscheidet, was überhaupt in den Stapel gelangt: Nur Dateien, deren Erweiterung hier steht, kommen als Primärquellen in Betracht. Der Abgleich ignoriert Groß- und Kleinschreibung, `mkv` deckt also auch `MKV` ab.

- Die Liste ist nicht auf MKV beschränkt. Jeder Container, den mkvmerge lesen kann, ist eine zulässige Quelle (`mp4`, `avi` und weitere); der Ausgabecontainer ist immer Matroska.
- Jeder Eintrag wird zur Laufzeit gegen die Ausgabe von `--list-types` des lokalen mkvmerge geprüft. Ein Eintrag, den mkvmerge nicht kennt, wird weiterhin zum Abgleich verwendet, aber als `UnknownExtension` (Warnung) gemeldet - ein Tippfehler würde sonst still Kandidaten aus dem Stapel ausschließen.
- Die Checkbox Rekursiv neben diesem Feld bestimmt, ob auch Unterverzeichnisse des Quellverzeichnisses durchsucht werden (standardmäßig an).

Wechselwirkungen mit anderen Einstellungen:

- Diese Liste filtert nur *Primärdateien*. Jeder externe Verweis trägt seine eigene `extensions`-Liste und sein eigenes `recursive`-Flag für die Spender-Suche; siehe das Thema Quelle. Externe `.srt`-Untertitel gehören in die Liste des Verweises, nicht hierher - stünde `srt` hier, würde jede Untertiteldatei zur Primärquelle mit eigener Ausgabe.
- Eine Datei, die den Erweiterungsfilter passiert, aber nicht auf das Eingabemuster passt, wird als `IgnoredFile` (Info) gemeldet. Die beiden Filter bleiben so unterscheidbar: Die Erweiterungsliste entscheidet, was betrachtet wird, das Muster entscheidet, was identifiziert wird.

Wann anpassen: Trage genau die Quellcontainer ein, die gemuxt werden sollen, typischerweise `mkv` allein oder `mkv` plus `mp4`. Eine breitere Liste vergrößert den Stapel; eine engere ist der einfachste Weg, fremde Dateien im selben Verzeichnis aus einem Lauf herauszuhalten.
