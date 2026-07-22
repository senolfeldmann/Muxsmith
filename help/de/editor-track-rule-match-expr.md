# Match (Spurregel)

Der Match-Ausdruck wählt aus der Quelle der Regel die eine Spur aus, die diese Regel behandelt. Er ist eine Konjunktion aus bis zu fünf Teilen; jeder vorhandene Teil muss zutreffen:

- **`exact`**: Gleichheit von Eigenschaften, typisiert - Zahlen vergleichen numerisch, Sprachen als Sprachen (`de` ist gleich `ger`). Typisierte Gleichheit, die kuratierten Wertedomänen und der `raw:`-Bypass haben ein eigenes Thema; siehe das Thema Exact.
- **`substring`**: Enthaltensein ohne Groß-/Kleinschreibung, nur String-Eigenschaften - der übliche Weg, `SDH` oder `Commentary` im Spurnamen zu fangen.
- **`regex`**: Regex-Suche, genau wie geschrieben; nutze `(?i)` für Groß-/Kleinschreibungs-Unabhängigkeit. Nur String-Eigenschaften.
- **`any`**: eine Liste von Teilausdrücken, von denen mindestens einer zutreffen muss (ODER).
- **`not`**: eine Liste von Teilausdrücken, von denen keiner zutreffen darf (Ausschluss).

Regeln der Algebra:

- Mehrere Eigenschaften in einem Teil sind UND: `exact` mit `type: audio` und `language: en` verlangt beides auf derselben Spur.
- `any` und `not` enthalten vollständige Ausdrücke und schachteln beliebig tief; typische Profile bleiben flach.
- Eine vorhandene, aber leere `any`- oder `not`-Liste ist ein Konfigurationsfehler (`EmptyMatchList`) - eine leere ODER- oder Ausschlussgruppe ist immer eine unfertige Bearbeitung, nie ein sinnvolles "keine Bedingung". Lass den Schlüssel stattdessen weg.

Der Eindeutigkeitsvertrag: Jede Regel muss genau eine Spur ihrer Quelle treffen. Null Treffer bei einer Pflichtregel ist `MissingTrack` (der Hinweis listet Beinahe-Treffer: Spuren gleichen Typs oder gleicher Sprache und die jeweils verfehlte Bedingung); zwei oder mehr sind `AmbiguousRule`; eine von zwei Regeln beanspruchte Spur ist `OverlappingRules`. Der Schalter Optional deckt den legitimen Null-Treffer-Fall ab. Bei Mehrdeutigkeit schlägt die Vorschlags-Engine einschränkende Verfeinerungen vor, die du mit einem Klick übernehmen kannst - siehe die Vorschlagskarte in der Stapel-Ansicht.
