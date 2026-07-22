# Quelle (Spurregel)

Woher diese Regel ihre Spur nimmt. Der Standard `primary` matcht gegen die Spuren der Primärdatei selbst. Die Alternative ist ein *externer Verweis*: Die Regel holt ihre Spur aus einer Spender-Datei in der Nähe der Primärquelle - der Mechanismus hinter "nimm die türkischen Untertitel aus den `.srt`-Dateien neben den Episoden" oder "nimm das deutsche Audio aus einem zweiten Release".

Die Teile des Verweises:

- `path`: wo gesucht wird, relativ zum Verzeichnis der Primärdatei oder absolut. Sein eigenes `recursive`-Flag (standardmäßig aus) und seine eigene `extensions`-Liste bestimmen, welche Dateien Kandidaten sind - unabhängig von den Eingabe-Erweiterungen, die nur Primärquellen filtern.
- `match_to_source` oder `match_pattern`: wie ein Kandidat seiner Primärquelle zugeordnet wird. `match_to_source` verlangt die Kennung der Primärquelle im Namen des Spenders; ein Match-Muster ist eine Vorlage im Regex-Modus für Namen, die die Kennung anders kodieren. Beide schließen sich gegenseitig aus; jedes hat sein eigenes Thema.

Die Auswahl ist zweistufig: Der Verweis wählt Kandidaten-*Dateien*, dann wählt der Match-Ausdruck der Regel genau eine *Spur* innerhalb der gefundenen Datei. Spender-Dateien sind vollwertige Container - eine externe MKV mit einem Match auf Audio und Sprache `de` ist der vorgesehene Weg, passendes deutsches Audio aus einem parallelen Release zu ziehen.

Eindeutigkeit gilt auf beiden Stufen: Zwei passende Spender-Dateien ergeben `AmbiguousExternal`, zwei passende Spuren im Spender ergeben `AmbiguousRule`. Findet der Verweis keine Datei, scheitert die Regel mit `MissingExternal`, außer sie ist Optional. Ein Spender, der selbst Primärquelle des Stapels ist, wird mit `DonorIsPrimary` (Warnung) markiert: Er wird als eigene Ausgabe gemuxt *und* spendet Spuren.

Wann verwenden: immer dann, wenn eine gewünschte Spur außerhalb des Primärcontainers liegt. Für Spuren, die bereits in der Primärdatei stecken, bleibt die Quelle auf `primary`.
