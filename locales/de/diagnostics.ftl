### German (de) translation of locales/en/diagnostics.ftl. The en catalog is
### the source of truth; keys mirror it (id parity enforced by
### scripts/check-i18n.mjs); placeables and selector structure mirror it by
### convention (reviewed manually, not machine-checked). Config-field names
### and keywords (profile_version, codec_kind, codec_id, exact,
### select/drop/add, any/not, raw:) stay literal. Straight ASCII quotes as
### in en.
severity-error = Fehler
severity-warning = Warnung
severity-info = Info

unsupported-profile-version = Nicht unterstützte profile_version { $found } (unterstützt: { $supported }).
parse-error = Das Profil konnte nicht geparst werden: { $detail }
no-track-rules = Das Profil definiert keine Spurregeln; mindestens eine ist erforderlich.
empty-match-expression = Dieser Match-Ausdruck ist leer und würde auf jede Spur passen.
empty-extensions = Die Erweiterungsliste darf nicht leer sein.
invalid-regex = Ungültiger regulärer Ausdruck: { $detail }
unknown-property = Unbekannte Eigenschaft "{ $property }". Sie ist nicht Teil des mkvmerge-Identifikationsmodells.
raw-property = Die Eigenschaft "{ $property }" wird mit einem raw:-Präfix verwendet; sie umgeht das Fähigkeitsmodell und wird untypisiert abgeglichen. Dies ist die Opt-in-Möglichkeit für Vorwärtskompatibilität mit einem neueren mkvmerge-Identifikationsschema.
raw-on-known-property = Die Eigenschaft "{ $property }" ist eine bekannte Eigenschaft mit besonderer Abgleichsemantik; das raw:-Präfix umgeht diese (Sprachnormalisierung, codec_kind-Aliasing) und gleicht stattdessen byte-literal ab.
not-string-property = Die Eigenschaft "{ $property }" hat den Typ { $actual_type }; { $condition }-Bedingungen erfordern eine Zeichenketten-Eigenschaft.
value-type-mismatch = Der Wert für "{ $property }" hat den Typ { $found }, erwartet wurde { $expected }.
unknown-settable-property = "{ $property }" ist keine setzbare Spureigenschaft.
invalid-keyword = Ungültiges Schlüsselwort "{ $found }". Erlaubt: { $allowed }.
locator-conflict = match_to_source und match_pattern schließen sich gegenseitig aus; setze nur eines.
invalid-template = Ungültige Vorlage: { $kind ->
    [unclosed-brace] nicht geschlossene Klammer an Position { $pos }
   *[empty-field] leeres Feld an Position { $pos }
}
unknown-template-field = Unbekanntes Vorlagenfeld "{ $field }". Verfügbare Felder: { $allowed }.
unknown-template-filter = Unbekannter Vorlagenfilter "{ $name }".
path-separator-in-template = Dateinamenvorlagen dürfen keine Pfadtrennzeichen enthalten.
attachment-rule-shape = Jede Dateianhang-Regel benötigt genau eines von select, drop, add (gefunden: { $found }).
provable-overlap = Die Regeln { $rule_a } und { $rule_b } überschneiden sich beweisbar: jede Spur, die auf die eine passt, passt auch auf die andere. Füge einer von beiden eine unterscheidende Bedingung hinzu.
ambiguous-rule = Die Regel passt auf { $count } Spuren; sie muss auf genau eine passen.
overlapping-rules = Die Regeln { $rules } beanspruchen alle die Spur { $track }.
missing-track = Keine Spur passt auf diese nicht-optionale Regel.
missing-external = Keine Datei passt auf diesen externen Verweis.
ambiguous-external = { $count } Dateien passen auf diesen externen Verweis; genau eine ist erforderlich.
output-collision = Der Ausgabepfad { $path } kollidiert mit einer vorhandenen Datei oder einer anderen geplanten Ausgabe.
source-overwrite = Der Ausgabepfad { $path } würde eine Quelldatei überschreiben. Das ist niemals erlaubt.
duplicate-identifier = Die Dateien { $file_a } und { $file_b } teilen sich die Kennung "{ $identifier }".
donor-is-primary = Die externe Spender-Datei { $donor } ist selbst eine Primärquelle.
ignored-file = Die Datei passt auf die Erweiterungsliste, aber nicht auf das Eingabemuster.
multiple-identifier-matches = Das Eingabemuster passt mehr als einmal in "{ $name }"; der erste Treffer wird verwendet.
unknown-property-skew = Die Eigenschaft "{ $property }" wurde über ein raw:-Opt-in untypisiert abgeglichen (unter Umgehung des Fähigkeitsmodells). Dieser Build fixiert die mkvmerge-Identifikationsschema-Version { $pinned }; diese Datei meldet Version { $found_version }.
schema-drift = Dieser Build fixiert die mkvmerge-Identifikationsschema-Version { $pinned }; mindestens eine identifizierte Datei in diesem Stapel meldet Schema-Version { $found_version }. Jede von der neueren Version hinzugefügte Eigenschaft liegt außerhalb des Fähigkeitsmodells; nutze ein raw:-Präfix, um sie untypisiert abzugleichen.
unknown-extension = Die Erweiterung "{ $extension }" ist keine der von mkvmerge unterstützten Erweiterungen ({ $known }). Ist dies ein Tippfehler, werden passende Dateien stillschweigend nie gefunden; ist es beabsichtigt, kann mkvmerge sie nicht verarbeiten.
codec-kind-exact-only = Die Eigenschaft "codec_kind" kann nur mit exact verwendet werden, nicht mit { $condition }. Gleiche stattdessen codec_id mit { $condition } ab.
invalid-property-value = Der Wert "{ $value }" ist für die Eigenschaft "{ $property }" nicht gültig. Zulässige Werte sind unter anderem: { $allowed }.
path-separator-in-rendered-name = Der erzeugte Ausgabedateiname "{ $name }" enthält ein Pfadtrennzeichen; Muxsmith legt niemals Unterverzeichnisse an.
empty-rendered-name = Der erzeugte Ausgabedateiname ist leer oder ungültig ("{ $name }").
non-utf8-path = { $role ->
    [output] Der Ausgabepfad
    [chapters] Die externe Kapiteldatei
    [attachment] Die angehängte Datei
    [donor] Die externe Spender-Datei
   *[primary] Die Quelldatei
} { $path } ist kein gültiges UTF-8 und kann nicht unverfälscht an mkvmerge übergeben werden.
empty-match-list = Eine "any"- oder "not"-Liste darf nicht leer sein; entferne sie oder füge mindestens einen Unterausdruck hinzu.
unidentifiable-source = Eine Quelldatei ist vorhanden, konnte aber nicht identifiziert werden: { $detail }.
unsupported-source = { $kind ->
    [donor] mkvmerge hat den externen Spender { $donor } identifiziert, aber sein Container ist keine unterstützte Multiplex-Quelle.
   *[primary] mkvmerge hat diese Datei identifiziert, aber ihr Container ist keine unterstützte Multiplex-Quelle.
}
empty-plan = Dieser Plan ergibt null Ausgabespuren; mkvmerge schreibt dennoch eine gültige, aber spurlose MKV.
suggestions-capped = { $dropped ->
    [one] 1 weiterer Vorschlag für diese Regel wurde bei 3 gedeckelt und nicht angezeigt.
   *[other] { $dropped } weitere Vorschläge für diese Regel wurden bei 3 gedeckelt und nicht angezeigt.
}
suggestion-partition = { $kind ->
    [overflow] { $dropped ->
        [one] 1 weitere Behebungsgruppe wurde bei 5 gedeckelt und nicht angezeigt.
       *[other] { $dropped } weitere Behebungsgruppen wurden bei 5 gedeckelt und nicht angezeigt.
    }
   *[group] { $count ->
        [one] Diese Datei benötigt ihre eigene Verfeinerung; wende an:
       *[other] Diese { $count } Dateien benötigen ihre eigene Verfeinerung; wende an:
    }
{ $fix }
    auf: { $files }
}
worker-panicked = Ein Worker-Thread ist beim Ausführen dieses Jobs abgestürzt. Dies ist ein Fehler in Muxsmith, kein mkvmerge-Fehler; Details stehen im Anwendungsprotokoll.
