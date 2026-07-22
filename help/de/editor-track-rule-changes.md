# Änderungen (Spurregel)

Änderungen listet die Eigenschafts-Bearbeitungen, die auf die eine von dieser Regel getroffene Spur angewendet werden. Nicht gelistete Eigenschaften laufen unverändert durch; eine Regel ohne Änderungen kopiert ihre Spur, wie sie ist.

Die setzbaren Eigenschaften sind eine kuratierte, geschlossene Menge, jede auf eine mkvmerge-Option abgebildet:

- `language`, `track_name`, `sub_charset` - String-wertig. `language` akzeptiert ISO-639-2- (`ger`) und BCP-47-Schreibweisen (`de`); `sub_charset` wird nachsichtig geprüft und an mkvmerge durchgereicht.
- `default_track`, `forced_track`, `enabled_track` - die booleschen Spur-Flags.
- `flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`, `flag_original` - die booleschen Barrierefreiheits- und Herkunfts-Flags.

Werte sind pro Eigenschaft typisiert: Die Flags nehmen `true` oder `false`, keine Strings. Ein unbekannter Schlüssel ist hier ein Konfigurationsfehler (`UnknownSettableProperty`). Das `raw:`-Präfix, das die Match-Seite akzeptiert, ist ein reines Matching-Opt-in; in Änderungen wird es nicht akzeptiert.

Beachte die Asymmetrie zum Matching: Die matchbare Menge ist deutlich größer. Du kannst auf `codec_id`, `audio_channels` oder `pixel_dimensions` *matchen*, aber nur die oben gelisteten Eigenschaften *setzen* - alles andere bestimmt die Quellspur.

Verhältnis zu Vorschlägen und Ein-Klick-Übernahme: Ist eine Regel mehrdeutig, schlägt die Vorschlags-Engine Verfeinerungen am *Match* der Regel vor, nie an ihren Änderungen (die Nur-Einschränken-Garantie). Die Übernahme eines Vorschlags engt ein, welche Spur die Regel wählt; die Änderungen bleiben unangetastet und greifen dann auf der einen verbleibenden Spur.

Typische Verwendung: Untertitel-Varianten benennen (`track_name` auf `English SDH` mit `flag_hearing_impaired` auf `true`), eine Audiospur per `default_track` hervorheben, oder einer Spender-Spur ihre echte `language` geben, wenn die Quelldatei sie leer gelassen hat.
