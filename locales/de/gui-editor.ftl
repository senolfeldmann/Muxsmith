# German (de) translation of locales/en/gui-editor.ftl. The en catalog is
# the source of truth; keys mirror it (id parity enforced by
# scripts/check-i18n.mjs). Match -> Match (Match-Ausdruck, Match-Muster,
# established loanword, not "Übereinstimmung"), external locator ->
# externer Verweis, template -> Vorlage, property -> Eigenschaft. Straight
# ASCII quotes as in en; declarative register throughout (the save note is
# a statement, not a command, so it stays declarative rather than
# du-imperative). The .tooltip attributes (Plan 7, D53/D55) follow the same
# declarative register; config keywords (keep, drop, error, skip,
# overwrite, primary, clear, match_to_source, settable property names)
# stay literal.

## Profile (top-level sections)

editor-profile-meta = Metadaten
    .tooltip = Optionale beschreibende Felder für dieses Profil: ein Anzeigename und Freitext-Notizen. Sie beeinflussen das Muxen nie.
editor-profile-input = Eingabe
    .tooltip = Wie Quelldateien gefunden werden: das Kennungsmuster, die zugelassenen Dateierweiterungen und ob Unterverzeichnisse durchsucht werden.
editor-profile-output = Ausgabe
    .tooltip = Wohin die gemuxten Dateien geschrieben werden, wie ihre Dateinamen entstehen und was passiert, wenn eine Ausgabedatei bereits existiert.
editor-profile-tracks = Spuren
    .tooltip = Welche Spuren jede Ausgabe enthält und in welcher Reihenfolge: die Richtlinie für nicht zugeordnete Spuren plus die geordneten Spurregeln.
editor-profile-attachments = Anhänge
    .tooltip = Was mit Anhängen wie Schriftarten passiert: die Richtlinie für nicht zugeordnete Anhänge plus select-, drop- und add-Regeln.
editor-profile-chapters = Kapitel
    .tooltip = Was mit den Kapiteln der Quelldatei passiert: keep übernimmt sie, drop entfernt sie, ein externer Verweis lädt Kapitel aus einer separaten Datei.
editor-profile-tags = Tags
    .tooltip = Ob die globalen und die spurbezogenen Tags der Quelldateien behalten oder verworfen werden.
editor-profile-title = Titel
    .tooltip = Containertitel jeder Ausgabe: keep behält den Quelltitel, clear entfernt ihn, eine Vorlage rendert einen neuen.

## Meta

editor-meta-name = Name
    .tooltip = Menschenlesbarer Name dieses Profils. Freitext; beeinflusst das Muxen nie.
editor-meta-description = Beschreibung
    .tooltip = Freitext-Notizen, wofür dieses Profil gedacht ist und wie es verwendet werden soll. Beeinflussen das Muxen nie.

## Input

editor-input-pattern = Muster
    .tooltip = Regulärer Ausdruck, der im Dateinamen jeder Quelldatei gesucht wird; der erste Treffer wird zur Kennung der Datei, und seine Capture-Gruppen werden zu Vorlagenfeldern für Dateinamen und Spender-Zuordnung.
editor-input-extensions = Erweiterungen
    .tooltip = Dateierweiterungen, die in den Stapel aufgenommen werden; Groß-/Kleinschreibung spielt keine Rolle, geprüft wird gegen das, was das lokale mkvmerge lesen kann.
editor-input-recursive = Rekursiv
    .tooltip = Auch die Unterverzeichnisse des Quellverzeichnisses durchsuchen, nicht nur die oberste Ebene.

## Output (OutputCfg)

editor-output-directory = Verzeichnis
    .tooltip = Standardverzeichnis für die gemuxten Dateien. Das in der Stapel-Ansicht gewählte Ausgabeverzeichnis übersteuert es pro Lauf.
editor-output-filename = Dateiname
    .tooltip = Wie jede Ausgabedatei benannt wird: keep übernimmt den Quell-Dateinamen mit der Erweiterung .mkv, eine Vorlage rendert einen neuen Namen aus den erfassten Feldern.
editor-output-on-collision = Bei Kollision
    .tooltip = Was passiert, wenn die Ausgabedatei bereits existiert: error lehnt sie ab, skip lässt sie mit einer Warnung aus, overwrite ersetzt sie.

## TemplateBlock

editor-template-block-template = Vorlage
    .tooltip = Vorlagentext, der pro Quelldatei gerendert wird. Die ganze Kennung, benannte und nummerierte Capture-Gruppen und der Quell-Dateiname stehen als Felder bereit; Filter entfernen führende Nullen oder füllen sie auf.

## ExternalBlock

editor-external-block-external = Externer Verweis
    .tooltip = Verweis auf Spender-Dateien: wo gesucht wird, welche Dateierweiterungen zählen und wie jede Spender-Datei ihrer Quelldatei zugeordnet wird.

## TrackRule

editor-track-rule-source = Quelle
    .tooltip = Woher die Spur stammt: primary liest die Quelldatei des Stapels selbst, ein externer Verweis zieht sie aus einer Spender-Datei.
editor-track-rule-match-expr = Match
    .tooltip = Bedingungen, die die Spur erfüllen muss; alle angegebenen Teile müssen zutreffen, und jede Regel muss genau eine Spur in ihrer Quelle treffen.
editor-track-rule-optional = Optional
    .tooltip = Erlaubt der Regel, keine Spur zu treffen: sie wird dann übersprungen, statt die Datei scheitern zu lassen. Zwei passende Kandidaten bleiben ein Fehler.
editor-track-rule-changes = Änderungen
    .tooltip = Eigenschaftsänderungen, die auf die getroffene Spur angewendet werden, zum Beispiel language, track_name oder default_track.

## Locator

editor-locator-path = Pfad
    .tooltip = Verzeichnis, das nach Spender-Dateien durchsucht wird: relativ zum Verzeichnis der Quelldatei oder absolut.
editor-locator-recursive = Rekursiv
    .tooltip = Auch die Unterverzeichnisse des Verweispfads durchsuchen, nicht nur das Verzeichnis selbst.
editor-locator-extensions = Erweiterungen
    .tooltip = Dateierweiterungen, die bei der Suche nach Spender-Dateien berücksichtigt werden, geprüft gegen das, was das lokale mkvmerge lesen kann.
editor-locator-match-to-source = Match zur Quelle
    .tooltip = Ordnet Spender-Dateien ihren Quelldateien zu, indem der Dateiname des Spenders die aus der Quelle erfasste Kennung enthalten muss. Kurzform für ein Match-Muster aus genau dieser Kennung.
editor-locator-match-pattern = Match-Muster
    .tooltip = Vorlage, die als regulärer Ausdruck gegen Spender-Dateinamen läuft; eingesetzte Feldwerte werden wörtlich verglichen. Schließt sich mit match_to_source gegenseitig aus.
editor-locator-case-sensitive = Groß-/Kleinschreibung beachten
    .tooltip = Spender-Dateinamen unter Beachtung der Groß-/Kleinschreibung abgleichen; standardmäßig wird sie ignoriert.

## Attachments (AttachmentsCfg)

editor-attachments-unmatched = Nicht zugeordnet
    .tooltip = Was mit Anhängen passiert, die keine Regel trifft: keep übernimmt sie in die Ausgabe, drop verwirft sie. Werden Schriftarten verworfen, werden ASS-Untertitel stillschweigend falsch dargestellt.
editor-attachments-rules = Regeln
    .tooltip = Geordnete select-, drop- und add-Regeln für Anhänge. Regeln greifen in Listenreihenfolge, der erste Treffer gewinnt, und eine Regel darf mehrere Anhänge treffen.

## Tracks (TracksCfg)

editor-tracks-unmatched = Nicht zugeordnet
    .tooltip = Was mit Spuren der Quelldatei passiert, die keine Regel trifft: keep übernimmt sie in die Ausgabe, drop verwirft sie.
editor-tracks-rules = Regeln
    .tooltip = Geordnete Spurregeln; jede muss genau eine Spur treffen, und die Listenreihenfolge bestimmt die Spurreihenfolge der Ausgabe.

## AttachmentRule

editor-attachment-rule-select = Auswählen
    .tooltip = Match-Ausdruck für Anhänge, die behalten werden; jeder passende Anhang wird in die Ausgabe übernommen.
editor-attachment-rule-drop = Verwerfen
    .tooltip = Match-Ausdruck für Anhänge, die verworfen werden; jeder passende Anhang wird weggelassen.
editor-attachment-rule-add = Hinzufügen
    .tooltip = Verweis auf externe Dateien, die jeder Ausgabe als Anhang beigefügt werden; gesucht wird wie bei Spender-Dateien.

## Tags (TagsCfg)

editor-tags-global = Global
    .tooltip = Ob die containerweiten Tags der Quelldateien behalten oder verworfen werden.
editor-tags-track = Spur
    .tooltip = Ob die spurbezogenen Tags der Quelldateien behalten oder verworfen werden.

## MatchExpr

editor-match-expr-exact = Exakt
    .tooltip = Eigenschaften werden als typisierte Gleichheit verglichen: Zahlen numerisch, Sprachen über gleichwertige Schreibweisen hinweg. Mehrere Einträge müssen alle zutreffen.
editor-match-expr-substring = Substring
    .tooltip = Prüft, ob String-Eigenschaften den Wert enthalten; Groß-/Kleinschreibung spielt keine Rolle. Mehrere Einträge müssen alle zutreffen.
editor-match-expr-regex = Regex
    .tooltip = Regulärer Ausdruck, der in String-Eigenschaften gesucht wird, Groß-/Kleinschreibung wie geschrieben. Mehrere Einträge müssen alle zutreffen.
editor-match-expr-any = Beliebig
    .tooltip = Teilausdrücke, von denen mindestens einer zutreffen muss: der ODER-Teil eines Match-Ausdrucks.
editor-match-expr-not = Nicht
    .tooltip = Teilausdrücke, von denen keiner zutreffen darf: schließt Spuren aus, die sonst passen würden.

## Save surface (D41)

editor-save-note = Speichern schreibt die Datei komplett aus dem Modell neu: Kommentare, Schlüsselreihenfolge und Formatierung bleiben dabei nicht erhalten, und Felder, die auf ihrem Standardwert stehen, werden nicht zurückgeschrieben.

## Generic list/map actions

editor-action-add = Hinzufügen
editor-action-remove = Entfernen

## Rule grid ordinal (D59)

# Presentation-only 1-based index column; not a registry label, so no
# .tooltip. The digit itself is locale-neutral data rendered in the cell.
editor-track-rule-order = Reihenfolge
