## German (de) translation of locales/en/gui-editor.ftl. The en catalog is
## the source of truth; keys mirror it (id parity enforced by
## scripts/check-i18n.mjs). Match -> Match (Match-Ausdruck, Match-Muster,
## established loanword, not "Übereinstimmung"), external locator ->
## externer Verweis, template -> Vorlage, property -> Eigenschaft. Straight
## ASCII quotes as in en; declarative register throughout (the save note is
## a statement, not a command, so it stays declarative rather than
## du-imperative).

## Profile (top-level sections)

editor-profile-meta = Metadaten
editor-profile-input = Eingabe
editor-profile-output = Ausgabe
editor-profile-tracks = Spuren
editor-profile-attachments = Anhänge
editor-profile-chapters = Kapitel
editor-profile-tags = Tags
editor-profile-title = Titel

## Meta

editor-meta-name = Name
editor-meta-description = Beschreibung

## Input

editor-input-pattern = Muster
editor-input-extensions = Erweiterungen
editor-input-recursive = Rekursiv

## Output (OutputCfg)

editor-output-directory = Verzeichnis
editor-output-filename = Dateiname
editor-output-on-collision = Bei Kollision

## TemplateBlock

editor-template-block-template = Vorlage

## ExternalBlock

editor-external-block-external = Externer Verweis

## TrackRule

editor-track-rule-source = Quelle
editor-track-rule-match-expr = Match
editor-track-rule-optional = Optional
editor-track-rule-changes = Änderungen

## Locator

editor-locator-path = Pfad
editor-locator-recursive = Rekursiv
editor-locator-extensions = Erweiterungen
editor-locator-match-to-source = Match zur Quelle
editor-locator-match-pattern = Match-Muster
editor-locator-case-sensitive = Groß-/Kleinschreibung beachten

## Attachments (AttachmentsCfg)

editor-attachments-unmatched = Nicht zugeordnet
editor-attachments-rules = Regeln

## Tracks (TracksCfg)

editor-tracks-unmatched = Nicht zugeordnet
editor-tracks-rules = Regeln

## AttachmentRule

editor-attachment-rule-select = Auswählen
editor-attachment-rule-drop = Verwerfen
editor-attachment-rule-add = Hinzufügen

## Tags (TagsCfg)

editor-tags-global = Global
editor-tags-track = Spur

## MatchExpr

editor-match-expr-exact = Exakt
editor-match-expr-substring = Substring
editor-match-expr-regex = Regex
editor-match-expr-any = Beliebig
editor-match-expr-not = Nicht

## Save surface (D41)

editor-save-note = Speichern schreibt die Datei komplett aus dem Modell neu: Kommentare, Schlüsselreihenfolge und Formatierung bleiben dabei nicht erhalten, und Felder, die auf ihrem Standardwert stehen, werden nicht zurückgeschrieben.
