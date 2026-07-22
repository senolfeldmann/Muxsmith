# Exakter Match

Der `exact`-Teil eines Match-Ausdrucks vergleicht Spureigenschaften auf Gleichheit. Er ist eine Zuordnung von Eigenschaftsnamen zu Werten; eine Spur kommt infrage, wenn jeder Eintrag zutrifft. Alle Einträge sind UND-verknüpft, zusammen mit allen weiteren Teilen des Ausdrucks - die vollständige Algebra beschreibt das Thema Match.

## Typisierte Gleichheit

`exact` vergleicht jeden Wert in der Domäne seiner Eigenschaft, nicht als rohen Text:

- Zahlen vergleichen numerisch: `6` ist gleich `6.0`.
- Sprachen vergleichen als Sprachen: ISO-639-Schreibweisen und BCP-47-Tags werden auf eine kanonische Form reduziert; `de` ist gleich `ger`, `pt-Latn-BR` gleich `pt-BR`, während echte Unterschiede erhalten bleiben (`pt-BR` ist nicht `pt-PT`).
- Zeichenketten vergleichen unter Beachtung der Groß-/Kleinschreibung. Für Enthaltensein ohne Groß-/Kleinschreibung nimm `substring`, für byte-genaue Muster `regex`.

## Kuratierte Domänen

Eigenschaften mit geschlossenem Wertebereich werden geprüft, statt still nie zu matchen: ein `type`- oder `codec_kind`-Wert außerhalb seiner Domäne ist ein Konfigurationsfehler, eine unbekannte `language` ein Planungsfehler, geprüft gegen deine mkvmerge-Installation. `codec_kind` ist ein sprechender Alias (`srt`, `ass`, `pgs`, ...) für Mengen von `codec_id`-Werten und nur unter `exact` erlaubt - brauchst du `substring` oder `regex`, matche stattdessen `codec_id`.

## Boolesche Flags: fehlend heißt false

mkvmerge meldet Flags wie `flag_hearing_impaired` nur, wenn sie gesetzt sind. `exact` spiegelt das: eine boolesche Eigenschaft, die in der Identifikation einer Spur fehlt, gilt als `false`; `exact: { flag_hearing_impaired: false }` matcht also auch Spuren, die das Flag nie gesetzt haben.

## Der `raw:`-Bypass

Ein Eigenschaftsname mit dem Präfix `raw:` (etwa `raw:dolby_complexity_index`) matcht eine Eigenschaft, die das Schema dieses Builds noch nicht kennt: keine Existenz-, Typ- oder Domänenprüfung, keine Sprachnormalisierung, kein Fehlend-heißt-false - nur byte-genaue Wertgleichheit gegen die wörtlich benannte Eigenschaft. Die Diagnosen markieren jeden Bypass. Nutze ihn nur für Eigenschaften, die ein neueres mkvmerge meldet; ein Tippfehler in einem `raw:`-Namen matcht still nie.
