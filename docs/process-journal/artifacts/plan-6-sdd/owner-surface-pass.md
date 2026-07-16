# Plan 6 - Owner Rendered-Surface Pass: alle nutzersichtbaren Strings des Profil-Editors

Kompilat fuer den Owner-Review am Plan-Ende (master `962005b`, "whole-branch READY"-Merge).
Zeigt jeden String, der durch Plan 6 im Profil-Editor sichtbar wurde: die 45 neuen
`gui-editor.ftl`-Keys, die 2 neuen `gui-batch.ftl`-Keys (One-Click-Apply), die 7
wiederverwendeten Keys an neuen Render-Orten, die app-eigene Grid-Notation der
Regel-Tabelle sowie die eine noch offene Owner-Frage. Verbatim aus den Quelldateien,
keine Paraphrase der zitierten Strings.

## 1. Die 45 gui-editor.ftl-Keys

Quelle: `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl`, Datei-Reihenfolge.
Typ-Spalte: **Label** (42 Feld-Beschriftungen), **Aktion** (2 generische Listen-/Map-Buttons),
**Save-Hinweis** (1 Standing Note zum Speicherverhalten, D41).

| # | Key | EN | DE | Typ |
|---|---|---|---|---|
| 1 | `editor-profile-meta` | Metadata | Metadaten | Label |
| 2 | `editor-profile-input` | Input | Eingabe | Label |
| 3 | `editor-profile-output` | Output | Ausgabe | Label |
| 4 | `editor-profile-tracks` | Tracks | Spuren | Label |
| 5 | `editor-profile-attachments` | Attachments | Anhänge | Label |
| 6 | `editor-profile-chapters` | Chapters | Kapitel | Label |
| 7 | `editor-profile-tags` | Tags | Tags | Label |
| 8 | `editor-profile-title` | Title | Titel | Label |
| 9 | `editor-meta-name` | Name | Name | Label |
| 10 | `editor-meta-description` | Description | Beschreibung | Label |
| 11 | `editor-input-pattern` | Pattern | Muster | Label |
| 12 | `editor-input-extensions` | Extensions | Erweiterungen | Label |
| 13 | `editor-input-recursive` | Recursive | Rekursiv | Label |
| 14 | `editor-output-directory` | Directory | Verzeichnis | Label |
| 15 | `editor-output-filename` | Filename | Dateiname | Label |
| 16 | `editor-output-on-collision` | On collision | Bei Kollision | Label |
| 17 | `editor-template-block-template` | Template | Vorlage | Label |
| 18 | `editor-external-block-external` | External locator | Externer Verweis | Label |
| 19 | `editor-track-rule-source` | Source | Quelle | Label |
| 20 | `editor-track-rule-match-expr` | Match | Match | Label |
| 21 | `editor-track-rule-optional` | Optional | Optional | Label |
| 22 | `editor-track-rule-changes` | Changes | Änderungen | Label |
| 23 | `editor-locator-path` | Path | Pfad | Label |
| 24 | `editor-locator-recursive` | Recursive | Rekursiv | Label |
| 25 | `editor-locator-extensions` | Extensions | Erweiterungen | Label |
| 26 | `editor-locator-match-to-source` | Match to source | Match zur Quelle | Label |
| 27 | `editor-locator-match-pattern` | Match pattern | Match-Muster | Label |
| 28 | `editor-locator-case-sensitive` | Case-sensitive | Groß-/Kleinschreibung beachten | Label |
| 29 | `editor-attachments-unmatched` | Unmatched | Nicht zugeordnet | Label |
| 30 | `editor-attachments-rules` | Rules | Regeln | Label |
| 31 | `editor-tracks-unmatched` | Unmatched | Nicht zugeordnet | Label |
| 32 | `editor-tracks-rules` | Rules | Regeln | Label |
| 33 | `editor-attachment-rule-select` | Select | Auswählen | Label |
| 34 | `editor-attachment-rule-drop` | Drop | Verwerfen | Label |
| 35 | `editor-attachment-rule-add` | Add | Hinzufügen | Label |
| 36 | `editor-tags-global` | Global | Global | Label |
| 37 | `editor-tags-track` | Track | Spur | Label |
| 38 | `editor-match-expr-exact` | Exact | Exakt | Label |
| 39 | `editor-match-expr-substring` | Substring | Substring | Label |
| 40 | `editor-match-expr-regex` | Regex | Regex | Label |
| 41 | `editor-match-expr-any` | Any | Beliebig | Label |
| 42 | `editor-match-expr-not` | Not | Nicht | Label |
| 43 | `editor-save-note` | Saving rewrites the file from the model: comments, key order and formatting are not preserved, and fields left at their default are not written back. | Speichern schreibt die Datei komplett aus dem Modell neu: Kommentare, Schlüsselreihenfolge und Formatierung bleiben dabei nicht erhalten, und Felder, die auf ihrem Standardwert stehen, werden nicht zurückgeschrieben. | **Save-Hinweis** |
| 44 | `editor-action-add` | Add | Hinzufügen | **Aktion** |
| 45 | `editor-action-remove` | Remove | Entfernen | **Aktion** |

## 2. Die 2 neuen gui-batch-Keys (One-Click-Apply, Task 14 / D43, D49)

Quelle: `locales/en/gui-batch.ftl:59-60`, `locales/de/gui-batch.ftl:54-55`.

| Key | EN | DE |
|---|---|---|
| `batch-suggestion-apply` | Apply | Anwenden |
| `batch-suggestion-apply-tooltip` | Apply this fix to the profile and save it. | Diese Korrektur auf das Profil anwenden und speichern. |

## 3. Die 7 wiederverwendeten Keys an neuen Orten

Kein einziger neuer Key fuer Nav-Tab, Open/Save-Buttons, Datei-Dialog-Filter,
Diagnostics- und Recents-Ueberschrift im Editor: alle 7 Render-Stellen ziehen einen
bereits existierenden `gui-batch.ftl`- oder `gui-settings.ftl`-Key. Render-Orte gegen
`src/views/EditorView.vue` (HEAD `962005b`) und `src/App.vue` verifiziert.

| Key | EN | DE | Render-Ort | Fit-Anmerkung |
|---|---|---|---|---|
| `batch-profile-heading` | Profile | Profil | Nav-Tab, `src/App.vue:101` | siehe unten |
| `batch-profile-pick` | Choose profile... | Profil auswählen... | Open-Button, `src/views/EditorView.vue:401` | - |
| `batch-profile-current` | Selected profile: { $path } | Ausgewähltes Profil: { $path } | Current-Path-Zeile, `src/views/EditorView.vue:404` | siehe unten |
| `batch-profile-filter-name` | Muxsmith profiles | Muxsmith-Profile | Datei-Dialog-Filter, `src/views/EditorView.vue:218` | - |
| `settings-save` | Save | Speichern | Save-Button, `src/views/EditorView.vue:540` | - |
| `batch-diagnostics-heading` | Diagnostics | Meldungen | Diagnostics-Ueberschrift (eigene Element-id `editor-diagnostics-heading`), `src/views/EditorView.vue:441` | - |
| `batch-recents-heading` | Recent profiles | Zuletzt verwendete Profile | Recents-Ueberschrift (eigene Element-id `editor-recents-heading`), `src/views/EditorView.vue:413` | - |

Zwei der sieben Wiederverwendungen wurden in `task-13-verdict.md` als grenzwertig
eingestuft (die anderen fuenf als generisch/passend):

- **Nav-Tab ("Profile"/"Profil"):** steht neben den Aktivitaets-Labeln der anderen
  beiden Tabs ("Batch"/"Stapel", "Jobs") - "Profile" benennt aber ein Objekt, keine
  Aktivitaet, und derselbe String labelt bereits BatchViews eigene Abschnittsueberschrift
  fuer die Profilwahl (zwei Render-Stellen fuer einen String im selben Screen).
- **Current-Path-Zeile ("Selected profile: ..."):** milder Batch-Einschlag im Tonfall
  ("ausgewaehlt" statt "geoeffnet"/"in Bearbeitung"), von der Review aber als tragbar
  eingestuft.

## 4. Die Grid-Notation der Regel-Tabelle (app-authored, vom Owner bisher nicht gesehen)

Quelle: `src/views/EditorView.vue`, drei Summary-Funktionen ueber `TrackRule`. Kein
`$t()`-Aufruf, keine Fluent-Keys - reine app-eigene Notation aus rohen Profil-Werten,
gerendert in der Regel-Tabelle (`:463-517`) und im Zeilen-Button (`sourceSummary`).

**`sourceSummary` (`:282-290`)** - Quelle-Spalte (Zeilen-Button-Text):
- String-Quelle wird durchgereicht: `rule.source = "primary"` -> `primary`
- Externe Quelle liefert den rohen Pfad: `rule.source = { external: { path: "/media/commentary.ac3" } }` -> `/media/commentary.ac3`
- Fehlt `rule.source`, Fallback auf `SOURCE_KEYWORDS[0]`: -> `primary`

**`matchSummary` (`:292-311`)** - Match-Spalte, Teile kommagetrennt (`", "`):
- Exact: `key=value` - `match.exact = { codec: "AC3" }` -> `codec=AC3`
- Substring: `key~value` - `match.substring = { name: "commentary" }` -> `name~commentary`
- Regex: `key~/value/` - `match.regex = { name: "^Director" }` -> `name~/^Director/`
- Any: `any(n)` - `match.any` mit 3 Eintraegen -> `any(3)`
- Not: `not(n)` - `match.not` mit 2 Eintraegen -> `not(2)`
- Kombiniert (mehrere Teile gleichzeitig gesetzt): `codec=AC3, name~commentary, any(3)`
- Passthrough/leer: keines der fuenf Felder gesetzt -> leerer String, Zelle rendert leer

**`changesSummary` (`:313-315`)** - Changes-Spalte, Keys von `rule.changes` kommagetrennt:
- `rule.changes = { forced: true, default_track: false }` -> `forced, default_track`
- Passthrough/leer: `rule.changes` leer/undefiniert -> leerer String, Zelle rendert leer

## 5. Offene Owner-Frage

Wie in `.superpowers/sdd/plan-6/progress.md:178-181` festgehalten:

> OPEN OWNER QUESTION: dedicated nav-editor key ("Editor", budget 45->46) vs keeping
> "Profil" as the tab label - recommendation: dedicated key; can ride amendment 3 or
> plan close. The plan-close pass now covers the RENDERED editor surface (six reused
> strings, esp. nav tab + "Selected profile:").

Hintergrund dazu aus `task-13-verdict.md:69` (Design-Latitude-Gap, Q2a):

> Design latitude gap (Q2a): the plan's "no new user-facing string" + nav mandate left
> the which-key set unenumerated. Route to the controller as a plan-coverage finding;
> the owner pass may conclude a dedicated nav-editor key is warranted (a plan
> amendment).

Zur Entscheidung: entweder ein dedizierter Key `nav-editor` = "Editor"/"Editor" wird
eingefuehrt (Budget 45 -> 46, eigener Plan-6-Amendment-Schritt), oder der Nav-Tab
behaelt die Wiederverwendung von `batch-profile-heading` ("Profile"/"Profil") bei. Die
Controller-Empfehlung in `progress.md` lautet: dedizierter Key.
