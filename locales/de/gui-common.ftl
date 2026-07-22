### German (de) translation of locales/en/gui-common.ftl. The en catalog is
### the source of truth; keys mirror it (id parity enforced by
### scripts/check-i18n.mjs); placeables and selector structure mirror it by
### convention (reviewed manually, not machine-checked). NOTE: the native
### close-confirmation dialog reads the EN catalog directly
### (src-tauri/src/run.rs include_str!s locales/en/gui-common.ftl), so the
### close-abort-* strings below are not yet shown to a de user; kept
### single-line and translated for parity and a later shell i18n. The
### IpcError-code keys mirror their en wording.
app-title = Muxsmith

close-abort-title = Laufende Jobs abbrechen
close-abort-message = Derzeit läuft ein Job. Möchtest du wirklich alle laufenden Jobs abbrechen und beenden?
close-abort-confirm = Jobs abbrechen und beenden
close-abort-dismiss = Abbrechen

nav-label = Hauptnavigation
nav-batch = Stapel
nav-jobs = Jobs
nav-editor = Editor
settings-open-label = Einstellungen
    .tooltip = Anwendungseinstellungen öffnen (mkvmerge-Pfad, Standard-Jobs, Sprache).
browse-button = Durchsuchen...
    .tooltip = Die Datei über einen Dateidialog auswählen.
    .tooltip-directory = Das Verzeichnis über einen Ordnerdialog auswählen.

help-toggle-label = Hilfe
    .tooltip = Hilfemodus umschalten: Fahre über ein Bedienelement oder klicke es an, um zu lesen, was es tut.
help-sidebar-label = Hilfe-Seitenleiste

mkvmerge-not-found = mkvmerge wurde nicht gefunden.
mkvmerge-too-old = Das gefundene mkvmerge ({ $found }) ist älter als die erforderliche Mindestversion { $minimum }.
mkvmerge-spawn-failed = mkvmerge konnte nicht gestartet werden: { $detail }
identify-failed = Die Datei konnte nicht identifiziert werden: { $detail }
mkvmerge-query-failed = Die Abfrage von mkvmerge ist fehlgeschlagen: { $detail }
settings-dir-unavailable = Der Speicherort der Anwendungseinstellungen konnte auf diesem System nicht ermittelt werden.
settings-io-failed = Die Anwendungseinstellungen konnten nicht gelesen oder geschrieben werden: { $detail }
settings-parse-failed = Die Datei mit den Anwendungseinstellungen ist beschädigt: { $detail }
internal-task-failed = Ein interner Fehler ist aufgetreten: { $detail }

profile-save-io-failed = Das Profil konnte nicht geschrieben werden: { $detail }
profile-save-failed = Das Profil konnte für das Speichern nicht serialisiert werden: { $detail }
apply-unparsable-config-path = Der Vorschlag konnte nicht angewendet werden: "{ $path }" benennt keine Regel.
apply-rule-index-out-of-range = Der Vorschlag konnte nicht angewendet werden: keine Regel an Index { $index } (Regelanzahl: { $rules ->
        [one] 1 Regel
       *[other] { $rules } Regeln
    }).
apply-edit-changed-nothing = Der Vorschlag hat nichts geändert: Regel { $index } schränkt "{ $property }" bereits ein.

firstrun-detecting = mkvmerge wird gesucht...
firstrun-missing-heading = mkvmerge wurde nicht gefunden
firstrun-too-old-heading = mkvmerge ist zu alt
firstrun-detect-failed-heading = mkvmerge-Erkennung fehlgeschlagen
firstrun-guidance-windows = Installiere MKVToolNix von mkvtoolnix.download und versuche es erneut, oder verweise Muxsmith unten direkt auf mkvmerge.exe (üblicherweise unter %ProgramFiles%\MKVToolNix\mkvmerge.exe).
firstrun-guidance-macos = Installiere MKVToolNix von mkvtoolnix.download nach /Applications und versuche es erneut, oder verweise Muxsmith unten direkt auf die mkvmerge-Binärdatei (üblicherweise /Applications/MKVToolNix.app/Contents/MacOS/mkvmerge oder /usr/local/bin/mkvmerge).
firstrun-guidance-linux = Installiere das Paket mkvtoolnix aus deiner Distribution (z. B. apt, dnf, pacman) und versuche es erneut, oder verweise Muxsmith unten direkt auf die mkvmerge-Binärdatei (üblicherweise /usr/bin/mkvmerge oder /usr/local/bin/mkvmerge).
firstrun-guidance-fallback = Installiere MKVToolNix von mkvtoolnix.download und versuche es erneut, oder verweise Muxsmith unten direkt auf die mkvmerge-Binärdatei.
firstrun-picker-label = Pfad zur mkvmerge-Programmdatei
    .hint = Gib den Pfad zur mkvmerge-Programmdatei ein oder wähle sie aus, falls sie nicht an einem Standardort installiert ist.
firstrun-use-path = Diesen Pfad verwenden
    .tooltip = Diesen mkvmerge-Pfad speichern und erneut erkennen.
firstrun-retry = Erkennung wiederholen
    .tooltip = mkvmerge erneut erkennen, ohne den konfigurierten Pfad zu ändern.
