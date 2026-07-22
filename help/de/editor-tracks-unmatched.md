# Nicht zugeordnete Spuren

`Nicht zugeordnet` entscheidet, was mit Spuren der Quelldatei geschieht, die keine Spurregel matcht: `keep` oder `drop`. Der Standard ist `drop`. Auf Spender-Spuren wirkt die Einstellung nie - externe Spuren gelangen nur über eine Regel in die Ausgabe.

## `drop`

Die Ausgabe enthält genau das, was die Regeln gematcht haben. Präzise, aber unnachgiebig: eine Spurart, für die du keine Regel geschrieben hast, verschwindet still aus der Ausgabe. Unter `drop` ist eine leere Regelliste ein Fehler, denn jede Spur würde verworfen.

## `keep`

Jede Spur der Quelldatei läuft durch; Regeln ändern darüber hinaus Eigenschaften oder ergänzen Spender-Spuren. Eine Regel, die eine Quellspur matcht, wendet ihre Änderungen weiterhin an. Unter `keep` ist eine leere Regelliste erlaubt: ein reiner Passthrough-Remux (Container normalisieren, nur Titel, Kapitel, Anhänge oder Tags anfassen); die Validierung meldet diesen Fall mit einem Info-Hinweis.

## Wechselwirkung mit der Reihenfolge

Die Regelreihenfolge definiert die Spurreihenfolge der Ausgabe - unter `keep` aber nur teilweise: die Ausgabe listet zuerst alle Spuren der Quelldatei in deren eigener Reihenfolge, danach die Spender-Spuren in Regelreihenfolge. Behaltene, aber nicht zugeordnete Quellspuren zählen für diese Reihenfolge als zugeordnet, und eine Regel, die eine Quellspur matcht, verschiebt sie nicht. Quellspuren umzuordnen erfordert deshalb `drop`; dort definiert allein die Regelliste die Reihenfolge. Siehe das Thema Spurregeln.

## Wahl

Nimm `drop`, wenn die Ausgabe eine definierte Spurbesetzung haben soll (das typische Serienprofil: Video, zwei Audiosprachen, ausgewählte Untertitel). Nimm `keep`, wenn die Quellstruktur erhalten bleiben soll und du nur Eigenschaften anpasst oder Spender-Spuren ergänzt.
