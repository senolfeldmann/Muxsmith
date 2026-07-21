# Vorschlagskarte

Ein Probelauf kann einen Konflikt finden, den das Einengen des Match einer Regel beheben würde: Eine Regel passt auf mehrere Spuren, obwohl sie auf genau eine passen muss, oder mehrere Regeln beanspruchen dieselbe Spur. Der Bericht schlägt die Korrektur dann als Vorschlagskarte vor. Die Karte benennt die betroffene Regel über ihren Konfigurationspfad (zum Beispiel `tracks[2].match`) und zeigt das vorgeschlagene Regel-Fragment als YAML, exakt der Text, der im Profil landen würde.

## Kopieren oder Anwenden

**Kopieren** legt das YAML-Fragment in die Zwischenablage, damit du es selbst in das Profil einfügen kannst.

**Anwenden** erledigt den ganzen Weg mit einem Klick: Es lädt das Profil frisch von der Festplatte, wendet die Einengung an und speichert die Profildatei sofort wieder auf die Festplatte. Es gibt keinen separaten Bestätigungs- oder Speicherschritt; nach Anwenden ist die Datei auf der Festplatte geändert. Gespeichert wird kanonisch, Kommentare und Formatierung der Datei bleiben also nicht erhalten (siehe das Editor-Thema).

## Was Anwenden nie tut

Ein angewendeter Vorschlag engt immer nur den Match der einen betroffenen Regel ein. Er ordnet nie Regeln um, fasst nie eine andere Regel an und lockert nie einen Match. Wiederholtes Anwenden läuft deshalb auf ein aufgelöstes Profil zu, statt zu pendeln.

## Nach dem Anwenden

Der Bericht auf dem Bildschirm wird nicht aktualisiert: Er zeigt weiter den Stand von vor dem Anwenden. Führe den Probelauf erneut aus, um die Wirkung zu sehen; die angewendete Änderung übersteht diesen nächsten Probelauf garantiert.
