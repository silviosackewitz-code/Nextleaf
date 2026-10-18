# Bedienreferenz: Nirvana

Direkt in der vorhandenen Nirvana-macOS-App am 2026-10-17 angesehen.
Keine Aufgaben angelegt, bearbeitet oder abgeschlossen. Beobachtungen sind
keine automatische Freigabe, jede Funktion sofort nachzubauen.

## Beobachtet

- Inbox steht getrennt oben, danach die alltäglichen GTD-Ansichten.
- Projects hat zwei Funktionen: der Titel öffnet eine Projektübersicht,
  der eigene Pfeil klappt die Sidebar-Liste ein/aus. Ein-/Ausklappen geprüft.
- Tags sind ein eigener einklappbarer Bereich. Logbook und Trash stehen unten.
  Areas werden über eine Auswahl oben gefiltert.
- Next kann gruppiert nach Projekt oder als gemeinsame Liste angezeigt werden.
  Das geöffnete Menü „Next View Options“ bietet außerdem „sequential next actions“
  und eine Begrenzung sichtbarer Projektaktionen (top 1 bis top 9 oder All).
  Die zugrundeliegenden Regeln wurden dabei nicht funktional getestet.
- Tags/Termine stehen kompakt an der Aufgabenzeile, Notizen haben einen Indikator.
  Oben sind Tag-Filter und drei Dichtestufen für die Liste sichtbar.
- Der vom Nutzer gezeigte große Detailbereich ist ein in der Liste aufgeklappter
  Editor: Titel/Tags/Notizen links, Zeit/Energie/Fälligkeit/Status/Projekt rechts;
  „Save Changes“ und „Cancel“ unten. Kein separates Betriebssystemfenster.
- Eine bereits erledigte Aufgabe blieb durchgestrichen in Next sichtbar.
  „Cleanup“ ist mit „Collect completed items to the Logbook“ beschrieben.
  Cleanup wurde nicht ausgeführt; automatischer Zeitpunkt nicht geprüft.

- Das Aufgabenmenü am rechten Zeilenpfeil bietet unter anderem Due Date,
  Areas/Contacts/Contexts, Time, Energy, Action State und Move. Time enthält
  5/10/15/30/45 Minuten und mehrere Stundenwerte; Energy low/med/high/None.
  Ebenfalls sichtbare Funktionen wie Convert/Copy/Email sind separat zu bewerten.
- Nutzer bestätigt einzeilige Aufgaben, bearbeitbare Inline-Details und dieses
  Schnellmenü. New Item mit Zielen für Aufgaben, Project und Tag gefällt ebenfalls
  und ist inzwischen umgesetzt. Weitere Nutzerbilder zeigen zweizeilige Titel/Tags
  in getrennten Bereichen und schmale Untermenüs links mit direkter Auswahl ohne Save.
  Diese Präzisierung ersetzt die frühere Einzeilenpflicht. Nutzer präzisiert: Kategorien per Maus öffnen,
  Inline-Editor rechts auf die fünf Referenzfelder beschränken; übrige Angaben
  unter More options. Änderungen und Prüfstand: NEXT_STEPS.md.

## Für Nextleaf bereits beauftragt

Kompakte Navigation, Inbox separat oben, seltene Ansichten unten, einklappbare
Projekte, Plus/Dialog zur Projektanlage, Areas in Settings, klare Tags,
Abschlussknopf am Titel, vorläufig sichtbare Erledigungen, reduzierte Hilfstexte.
Schlichte bestehende Icons und englische UI beibehalten.

## Nacheinander zu klären

- Details sind geklärt: Nutzer bestätigt ausdrücklich direktes Aufklappen in
  der Liste wie Nirvana. Der zwischenzeitliche separate Dialog wird ersetzt.

1. Cleanup ausdrücklich auslösen oder erledigte Zeilen bei Refresh/Ansichtswechsel
   ausblenden? Aktuell umgesetzt ist die zweite Variante; Backend-Abschluss wird
   in beiden Fällen sofort gespeichert und ist im Logbook erreichbar.
2. Gruppierung ist bestätigt und umgesetzt: nichtleere Projektgruppen, projektlose
   Actions zuerst, keine wiederholten Projektlabels. Area-Hinweise, Tags und Termine
   bleiben per View options ein-/ausblendbar. Capture-Zeile und Areas-Sidebar-Liste
   entfernt; New Item und All Areas-Filter oben. Settings unten bündig ausgerichtet.

Vollständige Bearbeitung, Projektreihenfolge, Tags/Focus sowie Perspectives und
MCP bleiben im vorhandenen Funktionsplan. Review wird erst in der nächsten
Spezifikationsrunde anhand OmniFocus/iPad geplant, jetzt nicht implementiert.

## iPhone-Referenz (2026-10-17): sechs Nutzer-Screenshots

Alle sechs aktuellen Nirvana-iPhone-Screenshots in der macOS-Fotos-App visuell
angesehen (Aufgabenübersicht, Navigation, Eingabe mit Tastatur, Aufgabendetails,
Time-Filter, Ansichtsmenü). Keine Fotos geändert oder private Inhalte ins
Repository kopiert. Zusätzlich öffentliche iPhone-App-Store-Screens angesehen:
https://apps.apple.com/us/app/nirvana-for-gtd/id1022913190
Die Nutzer-Screenshots sind für diesen Entwurf maßgeblich.

Beobachtet:
- Aufgabenliste über nahezu volle Breite; Checkbox links, Focus-Stern rechts,
  Titel mehrzeilig, optionale Metadaten darunter. Erledigte Aufgabe bleibt blass
  und durchgestrichen in der Liste. Runder Plus-Knopf unten rechts.
- Kopf mit Menüknopf, All Areas und Drei-Punkte-Menü. Darunter Ansichtstitel und
  horizontale Filterreihe mit All/Due/Time/Energy/Tags; am Rand abgeschnittene
  weitere Filter sichtbar (Scrollgeste selbst nicht geprüft).
- Seitliches überlagerndes Navigationspanel mit Inbox, GTD-Ansichten und
  einklappbarem Projects-Bereich; Hauptansicht rechts teilweise sichtbar.
- Details als hoher einspaltiger Sheet mit Done oben, Titel/Notizen, Focus,
  Tags/Time/Energy/Due, Projektzuordnung und Move/Trash/Mehr unten.
- Eingabescreenshot zeigt Cancel/Save oben und Bildschirmtastatur unten;
  Aktionen bleiben darüber sichtbar. Ob Done automatisch speichert und wann
  die App zwischen diesen Zuständen wechselt, ist aus Bildern nicht belegbar.
- Time-Filter öffnet ein einzelnes kompaktes Auswahlmenü. Ansichtsmenü bietet
  View Options, Theme & Font Size, Log completed, Paste und Share.

Für Nextleaf übernehmen: mobile Navigation, volle Listenbreite, Touchmenüs,
separate mobile Details und tastaturverträgliche Bedienung. Die Desktop-Regel
„Abschlusskreis im Editor ausblenden“ nicht allein aufgrund des fremden Screens
ändern. Farben/zusätzliche Überschriften/Projektlabels und neue Funktionen sind
nicht automatisch übernommen. Noch offen: genaue mobile Speicher-/Zurück-Regel,
Filterumfang und Gesten. Bei Unklarheiten Nutzer gezielt fragen; weitere
Screenshots nach Nutzerangebot bei Bedarf, aktuell kein zusätzlicher Bedarf.

Quellcodeprüfung durch Agent: Sidebar wird unter 720px nur oberhalb des Inhalts
angeordnet; kleine Desktop-Klickziele bleiben, Taskmetadaten in schmaler Spalte,
seitliche Untermenüs können überlappen, Safe-Area-Regeln fehlen. Kein tatsächlicher
iPhone-Lauf dieser Prüfung. Nächster Schritt ist ein konkreter mobiler Entwurf.
