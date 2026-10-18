# Review: OmniFocus auf dem iPad als Referenz

Recherche: 2026-10-17. Nur Grundlage für eine spätere Spezifikationsrunde;
**kein Implementierungsauftrag für Review oder Perspectives**. Der Nutzer
wünscht einen geführten Durchgang in kleinen Schritten, einschließlich Inbox
und einzelner Projekte. Aktuell haben mobile Bedienung, Desktop-Offline und MCP
Vorrang.

## Historisch belegt

Der offizielle **OmniFocus-for-iPad-Guide aus der ersten iPad-Generation (2010)**
beschreibt genau den wesentlichen Ablauf der Nutzererinnerung: Ein Projekt nach
dem anderen erscheint im Hauptbereich. Nach Prüfung bestätigt man ausdrücklich
mit „Mark Reviewed“ und gelangt zum nächsten Projekt. Projektstatus und
Review-Intervall lassen sich dabei ändern. Die bebilderte Review-Seite ist die
gedruckte Seite 6, PDF-Seite 7. Das Dokument nennt zwei Wochen als damaligen
Standard; diese historische Vorgabe ist keine Nextleaf-Anforderung.
[Originalguide, PDF](https://www.omnigroup.com/documents/review_guides/OmniFocus-iPad-Review-Guide.pdf#page=7)

Der Herstellerbeitrag **„OmniFocus at School: Reviews“ vom 7. September 2012**
beschreibt eine Sidebar mit den zur Prüfung anstehenden Projekten und daneben
genau ein ausgewähltes Projekt. Unten stehen letzte Prüfung, Intervall,
Projektstatus und die Bestätigung. Der Artikel empfiehlt bewusst kleine
Review-Portionen und passende Intervalle je Projekt. Inhaltlich prüft man, ob
das Projekt noch relevant oder bereits fertig ist und ob eine klare nächste
Aktion existiert. Pausierte Projekte kommen weiterhin zur Prüfung.
[Beitrag mit historischem Bildausschnitt](https://www.omnigroup.com/blog/omnifocus-at-school-reviews),
[Datumsbeleg im Herstellerarchiv](https://www.omnigroup.com/blog/archives/)

Auch **OmniFocus 2 für iOS, Handbuch 2.5**, dokumentiert die Prüfung je Projekt
und eine sichtbare Bestätigung unten rechts. Auf dem iPad befindet sich das
Intervall unten, auf dem iPhone oben. Der dort genannte Standard ist bereits
eine Woche. Nicht alle Details oder Standardwerte waren also über die Versionen
identisch. Ein langer Druck auf einen Projektnamen erlaubt eine vorgezogene
Prüfung. Das Handbuch enthält einen Screenshot der iPad-Review-Ansicht.
[OmniFocus 2: Review](https://support.omnigroup.com/documentation/omnifocus/ios/2.5/en/review/)

## Heute dokumentiert

Das aktuell abgerufene **OmniFocus-4-Handbuch 4.9.3** behält den Kern bei:
Eine flache Liste fälliger Reviews führt durch jeweils ein Projekt. „Previous“
und „Next“ navigieren; „Mark Reviewed“ aktualisiert dagegen das Datum der letzten
Prüfung, berechnet den nächsten Termin anhand des Intervalls und zeigt dann das
nächste Projekt. Navigation und Bestätigung sind getrennt. Die Anzahl anstehender
Projekte, letzte Prüfung und Intervall bleiben sichtbar. Sortierung und Anzeige
pausierter/zurückgestellter Projekte sind einstellbar; diese Zusatzoptionen sind
keine automatisch übernommenen Anforderungen für Nextleaf.
[OmniFocus 4: Review](https://support.omnigroup.com/documentation/omnifocus/universal/4.9.3/en/perspectives/#review)

## Inbox ist ein eigener Teil des gesamten Reviews

Die belegte eingebaute **Review-Perspektive prüft Projekte**. Die Quellen belegen
keinen allgemeinen eingebauten Assistenten, der automatisch zuerst die Inbox,
dann alle Listen und Projekte abarbeitet. Das umfassendere GTD Weekly Review
umfasst zusätzlich unter anderem Eingangsklärung, Kalender, Waiting und Someday.
Omnis GTD-Whitepaper unterscheidet diese Routine von der Projektprüfung und
beschreibt eigene wiederkehrende Checklisten als Review-Vorlagen.
[GTD-Whitepaper: Reviewing, Review Templates und Weekly Review](https://inside.omnifocus.com/gtd-whitepaper)

Ein späterer Nextleaf-Durchgang mit Inbox **und** Projekten ist daher eine
bewusste Umsetzung des Nutzerwunschs, keine nachgewiesene 1:1-Kopie einer
früheren OmniFocus-Funktion. Welche alte Version der Nutzer verwendet hat,
ist nicht bekannt.

## Warum das zum Nutzerwunsch passt — eigene Ableitung

- Ein Gegenstand im Mittelpunkt begrenzt Ablenkung; die übrige Arbeit bleibt
  als Warteliste erreichbar.
- Eine ausdrückliche Bestätigung gibt einen klaren Abschluss. Bloßes Öffnen
  oder Weiterblättern behauptet nicht, dass etwas geprüft wurde.
- Sichtbarer Restumfang und regelmäßige Wiedervorlage entlasten das Gedächtnis.
- Aufgaben und Projektstatus lassen sich im Prüfungskontext korrigieren.
- Kleine Portionen vermeiden den Zwang, immer den gesamten Bestand abzuarbeiten.

Dies sind Gestaltungsableitungen aus dem beschriebenen Ablauf und den
Nutzerpräferenzen, keine gemessenen Aussagen über Produktivität oder Usability.

## Für die spätere Planung offen

1. Konkrete Schritte und Reihenfolge: Inbox, Projekte, gegebenenfalls Waiting
   und Someday; welchen Umfang soll eine Sitzung haben?
2. Standardintervall, Ausnahmen je Projekt und Behandlung leerer/pausierter
   Projekte. Review-Datum von einer Aufgabenfälligkeit getrennt halten.
3. Pausieren/Fortsetzen und Überspringen ohne falsche Review-Bestätigung.
4. Einfluss der späteren Perspektiven: kein unbemerkter Ausschluss versteckter
   Areas oder Tags aus einem als vollständig dargestellten Review.
5. Einheitliche fachliche Regeln für UI, Offline-Abgleich und spätere MCP-Tools;
   ein Agent soll Reviews nicht ohne Auftrag als durchgeführt bestätigen.

Geprüft wurden offizielle Dokumenttexte und historische Quellenverweise.
Keine OmniFocus-Version wurde live bedient. Der Screenshotabruf der PDF-Seite
scheiterte im Recherchewerkzeug; die visuellen Einzelheiten sind daher nicht
als eigene Bildprüfung dokumentiert. Keine Review-Felder, Migrationen oder
Oberflächen implementiert.
