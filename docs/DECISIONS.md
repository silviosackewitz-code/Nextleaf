# Architekturentscheidungen

Dieses Protokoll hält vereinbarte Entscheidungen fest. Neue Entscheidungen mit
Datum, Anlass, Begründung und Konsequenzen ergänzen. Überholte Entscheidungen
ausdrücklich als ersetzt markieren.

## 2026-10-18 – GitHub zunächst privat

**Nutzerentscheidung:** Das neue Repository Nextleaf zunächst privat anlegen.
Umgesetzt unter `silviosackewitz-code/Nextleaf`, als Remote `origin` verbunden.
Dies erlaubt noch keine öffentliche Veröffentlichung; Open-Source-Lizenz,
Release-Downloads und Container-Registry folgen getrennt. Lokale Testdaten,
Zugangsdaten und Buildpakete gehören nicht ins Quellcode-Repository.

## 2026-10-18 – Neuinstallation mit veröffentlichten Paketen prüfen

**Nutzerwunsch:** Nach der Mobilprüfung selbst alle Nextleaf-Teststände auf Pi,
Mac und iPhone entfernen und den üblichen Anwenderweg von Anfang an testen.
Installationsanleitung im Repository. Dies bestätigt den vorgesehenen Download-/
Compose-Weg für den Pi und die native Mac-Installation; kein Docker auf dem Mac.

**Umsetzung dieser Runde:** Anleitung und separate Release-Compose-/ENV-Vorlagen,
kleine mobile Touch-Korrekturen. Bestehende Installationen nicht entfernt.
**Noch erforderlich:** GitHub-Repository/Release-Ziel, veröffentlichte Images und
Mac-Installer, echte Geräteabnahme. Keine Hosting-Adresse oder Lizenz erfunden.

## 2026-10-18 – Automatische Verbindung gehört zur jeweiligen Desktop-Datenbank

**Anlass:** Nativer Mac-Test zeigte, dass eine frische, separate Datenbank beim
UI-Start eine alte WebView-Adresse übernahm und ungeplant zum Pi synchronisierte.

**Implementierungsentscheidung:** Startabgleich nutzt ausschließlich `remote_url`
aus `desktop_status`. Ohne gespeicherte Verbindung ist explizites Connect nötig;
eine gespeicherte Formularadresse allein löst keinen Abgleich aus. Bestehende
verbundene Datenbanken synchronisieren wie bisher automatisch.

**Begründung/Konsequenz:** Datenbankwechsel respektiert die dazugehörige Verbindung.
Kein zusätzlicher Schalter oder neuer Speicher. Mit frischer Datenbank und altem
WebView-Speicher nativ geprüft; anschließend vorhandene Mac-DB erfolgreich mit
pi7 abgeglichen. Dies dokumentiert die Fehlerkorrektur, keine neue vereinbarte
Produktfunktion. Der Pi benötigt dafür kein weiteres Update.

## 2026-10-18 – Eingebetteter Desktop-Fachcode, Offline-Sync und lokales MCP

**Anlass:** Der Nutzer möchte Desktop-Funktionen und Agentenzugriff ohne laufenden
Pi verwenden; Web auf dem iPhone bleibt online. Dies ersetzt die früheren
Entwurfsannahmen „nur Online-Desktop“, „lokaler Adapter noch offen“ und einen
zwingend ausschließlich zentralen MCP-Zugang.

**Entscheidung:** Rust-Fachcode als gemeinsame Library. Tauri ruft den vorhandenen
Axum-Router innerhalb des Prozesses über IPC/Tower auf; kein lokaler Port oder
Sidecar. `Nextleaf --mcp` bietet dieselben elf Fachoperationen über stdio und
öffnet dieselbe lokale SQLite-Datei. Datenpfad ist
`dirs::data_dir()/dev.taskmanager.desktop/nextleaf.db`. UI oder MCP-Prozess
versuchen bei konfigurierter Adresse alle 15 Sekunden den privaten Pi-Abgleich.

**Begründung:** Dieselben GTD-Regeln und Transaktionen gelten für Server, Desktop
und Agenten. Die lokale Datenbank ermöglicht Offline-Arbeit; bestehende
Entwicklungsdaten und produktive Pi-Dateien müssen nicht kopiert werden.

**Konsequenzen:** Areas, Tags, Projekte, Aufgaben und Beziehungen werden über
persistierte Baseline und feldweisen Dreiwege-Abgleich synchronisiert. Änderungen
an verschiedenen Feldern werden zusammengeführt; widersprüchliche Werte bleiben
sichtbar und erhalten lokale Daten. Eine ausdrückliche lokale/Server-Wahl betrifft
nur Konfliktfelder; neue Konfliktwerte verlangen erneute Prüfung. Serveridentität,
Client-Sequenz und letzte Request-Quittung verhindern falsches Zusammenführen und
Doppelverarbeitung. SQLite-Writer-Transaktionen schützen auch getrennte UI-/MCP-
Prozesse. `expected`-Vorbedingungen schützen zusätzlich offene Editorentwürfe vor
zwischenzeitlich eingetroffenen Änderungen.

**Grenzen:** Snapshots bis 16 MiB/50.000 Datensätze; Harddelete außerhalb der App
stoppt den Abgleich, fachlicher Trash/Restore bleibt unterstützt. REST/Sync bleiben
privat ohne eigenen Login; MCP-Bearer schützt nur den zentralen MCP-Endpunkt.
Die iPhone-Web-App erhält keinen Offline-Bestand. Reale ChatGPT-/Claude- und
Sprachverbindungen folgen nicht automatisch aus dieser Implementierung.

**Status:** Quellstand umgesetzt, isolierte Rust-/HTTP-/Konflikttests und
Tauri-Rustprüfung/macOS-Debug-Bundle erfolgreich. Pi-Ausrollung, nativer
End-to-End-Test, stdio-Laufzeittest und produktiver initialer Mac-Pi-Abgleich
sind am 2026-10-18 geprüft; Details in NEXT_STEPS.md. Windows/Linux und reale Agenten-Clients nicht als geprüft behandeln.

## 2026-10-18 – Mobile Bedienung, Zähler und spätere Planung

**Entscheidung:** Eine gemeinsame React-App mit eigener mobiler Darstellung:
Drawer, mobiler Plus-Knopf, Touch-Menüs, Filter und einspaltige Details/Sheets.
Desktop behält kompakte Listen und Inline-Editoren. Inbox offen rot, Focus
überfällig rot plus übrige offene Focus-Aufgaben grau, Waiting grau; bei überfälligen Aufgaben nur
den Abschlusskreis rot umranden. Kein roter Rahmen um die gesamte Ansicht.

**Begründung:** Telefonbedienung benötigt ausreichend große Touchflächen und
andere Navigation, aber keine zweite fachliche Implementierung.

**Konsequenzen:** Mobile Darstellung ist im Quellstand umgesetzt; echte iPhone-,
Safari- und Tastaturprüfungen bleiben gesonderte Nachweise.
Perspectives sollen später den Area-Selektor ersetzen; der Perspektiveneditor
wählt aktive Areas/Tags und blendet den Rest aus. Review soll später schrittweise
Inbox und Projekte durchgehen. Historische OmniFocus-Referenz und Unterschiede
stehen in [REVIEW_RESEARCH.md](REVIEW_RESEARCH.md). Beide Funktionen bleiben
jetzt ausdrücklich außerhalb des Implementierungsumfangs.


## 2026-10-17 – Tags und feste untere Navigation

**Entscheidung:** Nach weiterer Nirvana-Referenz Tags in die Sidebar aufnehmen:
kompakte ein-/ausklappbare Filterliste, All und ein einzelner aktiver Tag. Projects
und Tags scrollen im oberen Bereich; Logbook/All tasks/Trash/Settings bleiben unten.

**Begründung:** Häufige Filter direkt erreichbar halten, ohne dass Archivnavigation
bei vielen Projekten oder Tags aus dem Fenster verschwindet.

**Konsequenz:** Bestehende Tagdaten und Sidebar-Flexaufteilung nutzen. Filter lokal
auf die bereits vom Server berechnete GTD-Liste anwenden; Backend-Verfügbarkeit
nicht neu berechnen. Keine Migration. Auf pi2 vorübergehend Tags aus vorhandenen
Aufgaben ableiten; vollständige Tagliste einschließlich unbenutzter Tags ab pi3.

## 2026-10-17 – Projektgruppen und reduzierte Navigation

**Anlass:** Nutzer präzisiert Nirvana-Referenz: Projekte gehören über ihre Aufgaben,
Capture-Zeile und Areas-Liste sollen entfallen; Settings-Icon ist versetzt/zu klein.

**Entscheidung:** Nichtleere Projektgruppen statt wiederholter Projektlabels pro
Aufgabe, projektlose Actions zuerst. Keine zweite Überschrift im Einzelprojekt.
New Item ersetzt permanente Erfassung. All Areas-Filter oben statt Sidebar-Liste;
Auswahl über GTD-Ansichtswechsel erhalten. Settings an unterer Navigation ausrichten.

**Konsequenz:** Ersetzt die frühere offene Gruppierungsentscheidung sowie permanente
Schnellerfassung. Nur Darstellung/Navigation ändern; Backend-Regeln unverändert.
In gruppierten Archivansichten bleibt die Abschlussreihenfolge innerhalb des Projekts.
Keine automatische Beschränkung auf erste Projektaufgaben oder weitere Nirvana-
Funktionen aus den Referenzbildern ableiten.

## 2026-10-17 – Direkte Schnellauswahl und höhere Listendichte

**Anlass:** Nutzer korrigiert anhand weiterer Nirvana-Bilder das bisherige zu große
Schnellmenü und die zu großzügigen Abstände. Ein seitlicher Formularbereich mit
Save trifft die gewünschte Interaktion nicht.

**Entscheidung:** Hauptmenü schmal rechts, einzelne Untermenüs links per Hover.
Werte sofort speichern, kein Save/Übernehmen/Cancel im Schnellmenü. Fehler bleiben
sichtbar und verhindern vorgetäuschten Erfolg. Inline-Editor behält Save/Cancel.
Titel und Tags bekommen getrennte Bereiche und bei Bedarf zwei Zeilen. Gesamte
Oberfläche verdichten. Completed-Banner entfernen; Häkchen erlaubt Wiederöffnen.

**Konsequenz:** Ersetzt die Einzeilenpflicht und Save-Regel für das Schnellmenü
in den früheren Entscheidungen. Keine Daten-/API-Änderung; bestehende PATCH-
Validierung nutzen. Keine zusätzlichen Nirvana-Funktionen aus Screenshots ableiten.
Review bleibt für später. Gemeinsames pi3-Paket vor Ausführung entsprechend ersetzen.

## 2026-10-17 – Vollständige kompakte Bedienrunde

**Entscheidung:** Nutzer fordert die bisher besprochenen Bedienpunkte zusammen
statt weiterer Teilstände. Inline-Editor rechts auf Time, Energy, Due, Status,
Project reduzieren; übrige Felder unter More options erhalten. Schnellmenü
öffnet Kategorien per Mausbewegung mit festen Zeilen und seitlichem Feldbereich.
New Item für Aufgaben/Projekte/Tags und Anzeigeoptionen je Ansicht ergänzen.

**Begründung:** Häufige Aktionen sollen flüssig erreichbar sein, ohne lange
Formulare oder beim Öffnen springende Kategorien. English UI und schlichte
Icons bleiben. Review ist weiterhin ausdrücklich für spätere Planung reserviert;
Perspectives und MCP bleiben die getrennten Funktionsetappen der Spezifikation.

**Konsequenz:** Vorhandene Komponenten/API wiederverwenden. Nur Tags-Anlage und
Focus bei Aufgabenerstellung benötigen Backend-Ergänzungen, keine Migration.
Anzeigepreferenzen lokal validieren/speichern. Ein gemeinsames Update auf pi3
vorbereiten; bestehende Pi-Daten erhalten, vor Wechsel sichern. Neuer Mac-Client
bleibt für vorhandene Funktionen mit pi2 kompatibel; neue Tag-/Focus-Anlage
fordert pi3 vor dem Schreiben an. Gruppierung und Cleanup-Änderung bleiben
unentschieden; aktuelle bestätigte Regeln nicht stillschweigend ersetzen.

## 2026-10-17 – Einzeilige Aufgaben, Inline-Editor und Schnellmenü

**Entscheidung:** Nach Nirvana-Sichtprüfung und Nutzer-Screenshots Aufgaben
kompakt in einer Zeile zeigen. Aufklappen öffnet den Editor mit Titel/Tags/Notizen
links und Eigenschaften rechts. Kleiner Pfeil am Zeilenende öffnet schnelle
Eigenschaftsänderungen. Time und Energy ausdrücklich bearbeitbar übernehmen.

**Begründung:** Die Liste bleibt lesbar; Einstellungen sind bei Bedarf zugänglich.
Beide Bedienwege verwenden dieselben Felder und Backend-Regeln.

**Konsequenz:** Bestehenden PATCH-Endpunkt um die übrigen vorhandenen Aufgabenfelder
erweitern, fehlend/null sauber unterscheiden, Tags und Aufgabe atomar ändern.
Keine neue Datenmigration. Save changes/Cancel statt Speichern jeder Texteingabe;
bei alten Servern erweiterte Änderungen mit verständlichem Update-Hinweis ablehnen.
Nirvana-Konvertierung, Kopieren, E-Mail und Wiederholungseditor sind nicht Teil
dieser Änderung. New Item-Menü ist als nächster UI-Schritt dokumentiert.

## 2026-10-17 – Review für spätere Spezifikation vorgemerkt

**Entscheidung:** Review als eigenes Thema der nächsten Spezifikationsrunde
vorsehen, vorerst keine Implementierung. Referenz ist OmniFocus, insbesondere
die iPad-Bedienung.

**Begründung:** Der Nutzer möchte den Ablauf bewusst planen, während die
aktuelle Etappe die alltägliche Aufgabenbedienung vereinfacht.

**Konsequenz:** Noch keine Review-Felder, Hintergrundabläufe oder UI hinzufügen.
Projektumfang, Intervalle und Sitzungsschritte sind gemeinsam festzulegen.

## 2026-10-17 – Kompakte GTD-Oberfläche und gemeinsame Daten

**Entscheidung:** Bestehende Nirvana-orientierte Produktplanung konkretisieren:
aufklappbare Aufgabendetails in der Liste mit großem Notizbereich und seitlichen Eigenschaften,
erkennbare Tag-Chips, fest am Titel ausgerichtete Abschlussknöpfe, vorübergehend
sichtbare erledigte Aufgaben. Projektanlage über Plus/Dialog, Areas in Settings,
kontextbezogenes Erfassen im Projekt. Inbox separat oben, seltene Ansichten unten;
redundante Hilfstexte einschließlich „GTD / …“ entfernen und einfache Icons
beibehalten. Projects ist einklappbar; untere Navigation bleibt auf Desktop
sichtbar. Der direkte Nirvana-Vergleich und offene Unterschiede stehen in
[UX_REFERENCE.md](UX_REFERENCE.md).

**Begründung:** Mehr Übersicht ohne zusätzliche dauerhafte Formulare oder
unnötige Einstellmöglichkeiten. Der Nutzer möchte die einfache Nirvana-Bedienung
mit den bereits geplanten Erweiterungen, keine neue Funktionsplanung.

**Konsequenz:** Abschluss sofort per vorhandener API speichern; sichtbare
Erledigungen nur als lokale Listenanzeige bis Refresh/Ansichtswechsel behalten.
Backend-GTD-Regeln und Datenmodell bleiben unverändert. Der zunächst vertagte
Aufgaben-Editor wurde anschließend beauftragt (siehe Entscheidung oben);
konfigurierbare Metadaten, Perspectives und MCP bleiben nächste Schritte.
Die Mac-App wird auf den Pi verbunden; lokale Alt-Daten nicht erneut importieren.

**Präzisierung:** Nach direkter Sichtprüfung der Nirvana-App bestätigt der Nutzer
„Direkt in der Liste wie Nirvana“. Dies ersetzt den vorläufigen separaten
Detaildialog. Projektanlage bleibt ein eigener kleiner Dialog.

## 2026-10-17 – Pi-Installation mit aktuellem Datenbestand

**Entscheidung:** Nutzer beauftragt die Installation und bestätigt ausdrücklich
die Übernahme der aktuellen Mac-Daten einschließlich Nirvana-Import und lokaler
Änderungen. Dafür eine konsistente SQLite-Online-Sicherung übertragen, statt
Nirvana erneut zu importieren.

**Begründung:** Bestehende IDs, Beziehungen und zwischenzeitliche Abschlüsse
bleiben erhalten. Quellcode und Daten werden getrennt übertragen; das Image
enthält keine Benutzerdaten.

**Konsequenz:** Nach erfolgreichem Umzug den Pi als gemeinsamen Datenbestand
verwenden und Clients dorthin verbinden. Die bisherige Mac-DB bleibt als
Rückfallstand erhalten, wird aber nicht synchronisiert. Laufzeit und Persistenz
vor Abschluss prüfen. Vorbereitung und offene sudo-Ausführung: DEPLOYMENT.md.

## 2026-10-17 – Nirvana-Testimport und auffindbare Abschlüsse

**Entscheidung:** Nutzer bestätigt alle gelieferten Inhalte für den Testbetrieb,
sämtliche Projekte ausdrücklich Parallel. 128 MCP-Objekte (117 Aufgaben,
11 Projekte) und alle gelieferten Areas/Kontexte übernommen. Eine vorhandene
lokale Aufgabe erhalten. Kein Schreiben nach Nirvana.

**Umsetzung:** Konsistente Sicherung, Probelauf auf einer Kopie, transaktionaler
Rust-Importer mit Quell-ID-Zuordnung und unveränderten Originalobjekten.
Erledigte Einträge, Trash und Wiederholungsvorlagen bleiben unterscheidbar.

**Grenzen:** MCP liefert weder Wiederholungsregeln noch manuelle Reihenfolge
oder genaue Uhrzeiten. Vorlage sichtbar, aber keine automatische Wiederholung;
Reihenfolge vorläufig nach Abruf. Keine vollständige Kontoexport-Prüfung.
Details und Prüfungen in [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

**Nachtrag zum Abhaken:** Nutzer findet abgehakte Aufgaben nicht wieder.
Einträge sind vorhanden; bisherige Sortierung folgt der Aufgabenreihenfolge.
Logbook künftig nach Abschlusszeitpunkt absteigend. Sofortige Rückmeldung
mit View Logbook und Undo ergänzt. Daten werden beim Abhaken nicht gelöscht.

## 2026-10-17 – Auswahlfelder an das App-Design angleichen

**Anlass:** Nutzer markiert die nativen Auswahlfelder für Projektart und
Aufgabenstatus als unpassend zur Oberfläche.

**Entscheidung:** Bestehende HTML-Selects mit CSS gestalten (`appearance: none`,
SVG-Pfeil, sichtbarer Tastaturfokus), einschließlich der Statusfelder in
Aufgabenzeilen. Keine zusätzliche Komponente oder Bibliothek. Das geöffnete
Auswahlmenü bleibt nativ; im erzwungenen Kontrastmodus auch das geschlossene Feld.

**Konsequenz:** Keine Änderung an Auswahlwerten, Daten oder Geschäftslogik.
Gemeinsames Styling gilt für Web und Tauri.

## 2026-10-17 – Nirvana über offiziellen MCP-Server lesen

**Entscheidung:** Auf Nutzerwunsch den offiziellen Server
`https://mcp.nirvanahq.com/mcp` in Codex verbinden und für die einmalige
Datenübernahme verwenden. Nirvana-Inhalte dabei nur lesen. Der JSON-Export
bleibt eine Ergänzung, falls MCP nicht alle benötigten Inhalte bereitstellt.

**Begründung:** Der Nutzer kennt die Anbindung aus Claude und bevorzugt den
direkten Kontozugriff. Eine eigene Integrationsimplementierung ist nicht nötig.

**Status:** Globalen Codex-MCP-Eintrag `nirvana` angelegt und OAuth-Anmeldung
erfolgreich abgeschlossen. Nach App-Neustart sind die Werkzeuge verfügbar;
`whoami` und `get_task_counts` erfolgreich ausgeführt. Lesezugriff bestätigt.
Inzwischen 128 Objekte mit Notizen abgerufen; Logbook/Trash zusätzlich separat
abgeglichen und Testimport durchgeführt (siehe neuere Entscheidung oben).
Nicht gelieferte Metadaten bleiben dokumentierte Grenzen. Eintrag laut Nutzer in der UI nicht sichtbar,
Ursache ungeprüft; kein technischer Zugriffsblocker mehr.
Keine Zugangsdaten im Repository. Siehe [offizielle Nirvana-Anleitung](https://www.nirvanahq.com/help/ai/mcp-setup).

## 2026-10-17 – Ubuntu-Version des Pi konkretisiert

**Feststellung:** Nutzer meldet Ubuntu 24.04.5 LTS, Release 24.04, Codename
Noble auf dem Raspberry Pi 5. Dies ersetzt die frühere Annahme „neueste LTS“
und die daraus abgeleitete Zuordnung zu 26.04.1.

**Konsequenz:** Anleitung auf Ubuntu 24.04 LTS beziehen. Docker, Host-Datenpfade
und Tailscale-Konzept bleiben bestehen. Direkte Prüfung von Betriebssystem,
Architektur und Containerbetrieb steht aus.

## 2026-10-17 – Vollständige einmalige Nirvana-Datenübernahme beauftragt

**Anforderung:** Der Nutzer möchte alle vorhandenen Daten seines Nirvana-HQ-
Kontos in Nextleaf übernehmen und die nächste Funktionsetappe daran planen.

**Vorgehen:** Den offiziellen JSON-Export auswerten und die Feldzuordnung gegen
unser Datenmodell prüfen. Das ersetzt die frühere zeitliche Einordnung als
späteren optionalen Schritt; eine dauerhafte Importoberfläche oder Synchronisation
ist weiterhin nicht vorgesehen. Bestehende Nextleaf-Daten sichern, zuerst in
einer getrennten Datenbank importieren und Vollständigkeit prüfen.

**Ergänzung:** Nutzer erlaubt ausdrücklich auch direktes Auslesen aus Nirvana,
da er den Export nicht findet. Nach Anmeldung zuerst den offiziellen Export
selbst suchen; alternativ zugängliche UI-Inhalte lesen und Umfangsgrenzen
offenlegen. Kein Zugriff auf versteckte Sitzungsdaten erforderlich.

**Status:** Export noch nicht bereitgestellt. Der Browser zeigte die Anmeldung;
inzwischen MCP-Verbindung eingerichtet und OAuth erfolgreich (siehe oben).
Testimport inzwischen durchgeführt. Vorgeschlagene Reihenfolge der Funktionen
und Prüfgrenzen stehen in [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

## 2026-10-17 – Nextleaf nach Recherche beibehalten

**Entscheidung:** Der Nutzer bleibt nach Kenntnis der Rechercheergebnisse bei
**Nextleaf**. Eine eigene Domain ist vorerst nicht vorgesehen. Falls später
eine Domain benötigt wird, darf deren Name vom Produktnamen abweichen.
Donebrook und Readyfern werden nicht übernommen; die Namenssuche ist beendet.

**Konsequenzen:** App, Pakete und vorbereitete Deployment-Pfade bleiben bei
Nextleaf. Keine weitere Umbenennung oder Domainregistrierung. Die bisherigen
Namensüberschneidungen und Prüfgrenzen bleiben in [NAMING.md](NAMING.md)
dokumentiert; die Produktentscheidung ist keine rechtliche Markenfreigabe.

## 2026-10-17 – Namensentscheidung wieder geöffnet (ersetzt)

Ersetzt durch „Nextleaf nach Recherche beibehalten“ oben.

**Entscheidung:** Nach den Recherchetreffern zu Nextleaf und Clear Trail sucht
der Nutzer neue, vorab geprüfte Namen. Die frühere Entscheidung für Nextleaf
gilt damit nicht mehr als endgültiger Veröffentlichungsname.

**Konsequenzen:** Vor weiteren Umbenennungen Kandidaten prüfen und dem Nutzer
zur Auswahl vorlegen. Aktuell Donebrook und Readyfern; Befunde, Quellen und
Prüfgrenzen in [NAMING.md](NAMING.md). Bis zur Auswahl bleiben Code und
Deployment-Pfade bei Nextleaf. Keine Domain oder GitHub-Organisation reserviert.

## 2026-10-17 – Projektname Nextleaf

**Entscheidung:** Die Anwendung heißt **Nextleaf**, mit „leaf“ (englisch für
„Blatt“). Die zuvor übernommene Schreibweise „Nextlief“ wurde auf Hinweis des
Nutzers korrigiert. Eine Open-Source-Veröffentlichung
ist beabsichtigt; die Lizenz ist noch auszuwählen.

**Konsequenzen:** Produktname, Fenster- und Seitentitel sowie npm-/Cargo-Pakete
tragen den neuen Namen. Der bestehende Checkout-Ordner bleibt `TaskManager`.
Bereits verwendete Speicherkennungen bleiben kompatibel: `taskmanager.db`,
`TASKMANAGER_*`, `taskmanager_api_url` und Tauri-ID `dev.taskmanager.desktop`.
So zeigt die Umbenennung nicht auf eine neue leere Datenbank oder neue lokale
Einstellungen. Der noch nicht eingerichtete Pi verwendet `/etc/nextleaf` und
`/var/lib/nextleaf`; dies ersetzt die zuvor geplanten Pfade mit `taskmanager`.
Die anschließende [Namensprüfung](NAMING.md) ergab bestehende gleichnamige
Softwareprojekte, eine belegte GitHub-Organisation und registrierte `.com`- und
`.org`-Domains. Markenhinweise bestehen ebenfalls; rechtliche Freigabe offen.
Vor öffentlicher Veröffentlichung den Namen deshalb erneut bewerten.

## 2026-10-17 – Serverdaten in einem festen Host-Verzeichnis

**Entscheidung:** Im geplanten Docker-Betrieb liegen die persistenten Daten in
einem festen Ordner auf dem Raspberry Pi und werden per Bind-Mount eingebunden.
Zuordnung nach Recherche der Linux-Standards: `/var/lib/nextleaf` auf dem
Host nach `/data` im Container. Compose und lokale Einstellungen liegen unter
`/etc/nextleaf`. Der ursprüngliche Vorschlag `/srv/taskmanager/data` ist
damit ersetzt.

**Begründung:** Der Nutzer möchte App und Daten getrennt halten und den
Datenordner unmittelbar in die Serversicherung aufnehmen können. Für die
interne Datenbank verwenden wir die FHS-Trennung von Anwendungszustand und
Host-Konfiguration. Recherche und App-Vergleich stehen in `ARCHITECTURE.md`.

**Konsequenzen:** Keine Daten im Image oder in der beschreibbaren Container-Schicht.
Das gesamte SQLite-Verzeichnis einbinden; Berechtigungen passend zum App-Prozess
setzen. Container-Neuerstellung darf Daten nicht entfernen. Für Backups die
SQLite Online Backup API verwenden oder die App für eine konsistente
Dateisicherung stoppen. Auch Compose-Konfiguration, lokale Einstellungen und
App-Version sichern. Das Container-Image wird erneut bezogen oder aus
gesichertem Quellcode gebaut. Wiederherstellung gesondert testen.

Status: Dockerfile, `.dockerignore`, Compose und Konfigurationsbeispiel erstellt.
Compose validiert; Image-Build und Betrieb auf dem Pi noch ungeprüft.
Die App läuft im Container als UID/GID `10001:10001`, mit schreibgeschütztem
Root-Dateisystem und beschreibbarem Daten-Bind-Mount. Das Image wird zunächst
auf dem Pi aus dem Repository gebaut und mit einem expliziten lokalen Tag
gestartet; eine Registry-Veröffentlichung ist nicht eingerichtet.
Einrichtung und Prüfablauf: [DEPLOYMENT.md](DEPLOYMENT.md).
Referenzen: [Docker Bind-Mounts](https://docs.docker.com/engine/storage/bind-mounts/),
[SQLite Backup API](https://sqlite.org/backup.html).

## 2026-10-17 – Einheitlich englische Benutzeroberfläche

**Entscheidung:** Alle von der Anwendung bereitgestellten UI-Texte sind auf
Englisch, einschließlich Fehlermeldungen und barrierefreier Beschriftungen.
Das HTML-Dokument verwendet `lang="en"`.

**Begründung:** Der Nutzer wünscht eine einheitliche englische Oberfläche
anstelle der bisherigen Mischung aus Deutsch und Englisch.

**Konsequenzen:** Neue UI-Texte ebenfalls auf Englisch schreiben. Keine
Sprachauswahl oder Übersetzungsbibliothek erforderlich. Benutzerinhalte werden
nicht übersetzt; die deutschsprachige Projektdokumentation bleibt bestehen.

## 2026-10-17 – Dokumentation und Planung im Repository

**Entscheidung:** Entwicklung und Planung erfolgen in Codex. `docs/` ist die
dauerhafte Quelle für Spezifikation, Anforderungen, Architektur und Arbeitsstand.
`AGENTS.md` verweist neue Sitzungen auf diese Dokumente.

**Begründung:** Neue Codex-Sitzungen sollen ohne Zugriff auf ältere Chats
weiterarbeiten können.

**Konsequenzen:** Anforderungen in `SPECIFICATION.md`, Systemaufbau in
`ARCHITECTURE.md`, begründete Entscheidungen hier und offene Arbeit in
`NEXT_STEPS.md` bei relevanten Änderungen gemeinsam aktualisieren.

## 2026-10-17 – Bestehender Stack und gemeinsame Geschäftslogik

**Entscheidung:** Rust/Axum stellt die HTTP-API bereit, SQLx verwaltet SQLite.
React/TypeScript/Vite liefert dieselbe Oberfläche für Browser und Tauri 2.
GTD-Verfügbarkeit und Sequential-Regeln werden ausschließlich im Backend berechnet.

**Begründung:** Web, Desktop und ein späterer MCP-Adapter sollen dieselben Regeln
verwenden. Desktop-Zielplattformen sind macOS, Windows und Linux.

**Konsequenzen:** Keine direkte Datenbankanbindung aus dem Frontend und keine
zweite GTD-Implementierung im Client. Jede Zielplattform separat bauen und testen.

## 2026-10-17 – Zuerst das Gerüst verifizieren

**Entscheidung:** Zunächst Installation, Builds, SQLite, API und lokalen Start
prüfen. Größere Änderungen an der Benutzeroberfläche folgen erst nach Auftrag.
Der Backend-Dienst wird in dieser Etappe separat von Tauri gestartet.

**Begründung:** Ein verifiziertes Gerüst ist die Grundlage für die weitere
Produktentwicklung; automatische Serververwaltung ist ein späterer Schritt.

**Konsequenzen:** Aktuell nur Startprobleme und kleine Konfigurationsfehler
beheben. Kein Sidecar, keine Synchronisation und kein Remote-Betrieb in dieser Etappe.


## 2026-10-17 – Projektübersicht, Inline-Einstellungen und gemeinsame Kalender

**Entscheidung:** Projects öffnet eine Übersicht; getrennte Pfeilbedienung erhält
Collapse. Name öffnet Inline-Einstellungen, Actions-Zähler die Aufgaben. Wiederverwendung
von Task-Editor-Stilen, TagEditor und einem gemeinsamen DateField für Projekte/Tasks.

**Begründung:** Nutzer hat diese Abläufe anhand der Nirvana-Screenshots präzisiert.
Eigenschaften tragen ihre Werte direkt im Feld, ohne doppelte sichtbare Beschriftung.
Tags nur per Enter oder Vorschlagsklick bestätigen; Area rechts in derselben Zeile.
Focus als gelber Stern am Titel, kein Abschlusskreis im geöffneten Editor.

**Konsequenzen:** Kleine additive Projektmigration für Due/Focus/Tags, atomare
Projekt-PATCH-API mit project-editing-Kompatibilitätsprüfung. Projektabschluss
ändert keine Task-Statuswerte; Moduswechsel nutzt vorhandene Backend-Regeln.
Kalender benötigt keine Abhängigkeit, nutzt modalen Dialog samt Fokus/Backdrop.
Pi-Update sichert vorher und versucht nach Migration kein ungesichertes Downgrade.

## 2026-10-17 – Vollständige MCP-Abdeckung über das zentrale Backend

**Entscheidung:** Nextleaf soll einen eigenen zentralen MCP-Zugang am Pi-Backend
erhalten. Alle fachlichen App-Funktionen müssen für autorisierte Agenten verfügbar
sein; neue Funktionen werden in diese Abdeckung aufgenommen. Mac- und Web-App
arbeiten weiterhin auf demselben Serverbestand.

**Begründung:** Der Nutzer wünscht vollständige Agentenbedienung auf allen genutzten
Plattformen. Getrennte MCP-Server pro Oberfläche würden keine zusätzliche
fachliche Fähigkeit liefern und die Zugänge unnötig vervielfachen.

**Konsequenzen:** Noch nicht implementiert. Vor Umsetzung Funktionsinventar,
Transport/Client-Kompatibilität und Authentifizierung/Berechtigungen festlegen.
Gemeinsame Geschäftslogik statt nachgebauter GTD-Regeln. Lokale UI-Einstellungen
als eigene Abdeckungslücke dokumentieren und bei Bedarf Client-Anbindung entwerfen.
Nirvanas vorhandener MCP-Connector bleibt unabhängig. Keine vorgezogene Umsetzung
des noch zu spezifizierenden Reviews.

## 2026-10-17 – MCP-Implementierung und visuelle Editor-Abgrenzung

**Entscheidung:** Nutzer beauftragt MCP jetzt. Offizielles Rust-SDK statt eigenem
Protokoll, Streamable HTTP unter `/mcp`, Bearer-Authentifizierung und explizite
Host-Allowlist; Browser-Origin-Anfragen ablehnen. Adapter ruft vorhandene
REST-Operationen direkt auf. Ohne Schlüssel bleibt MCP deaktiviert.

**Begründung:** Alle aktuell verfügbaren fachlichen Funktionen sollen Agenten
zugänglich sein, mit identischer Validierung und Datenhaltung. Ein gemeinsamer
Server unterstützt Mac und iPhone, sofern der jeweilige Client ihn erreicht.

**Konsequenzen:** Client- und Sprachintegration separat prüfen; kein impliziter
Cloud-Zugriff ins Tailnet, kein OAuth und keine öffentliche Freigabe der
weiterhin unauthentifizierten REST-API. Künftige App-Funktionen müssen MCP
mitführen. Vollständige Funktionszuordnung und Einschränkungen in MCP.md.
Der Task-Editor bleibt inline und kompakt; dunklerer Hintergrund und klarer Rand
heben ihn auf ausdrücklichen Nutzerwunsch hervor.

## 2026-10-17 – Präzisierung: Agentenzugriff bevorzugt über das Endgerät

**Entscheidung/Ziel:** Der Nutzer möchte ChatGPT/Claude lokal mit Nextleaf auf
Mac/iPhone verbinden. Nextleaf soll seine vorhandene private Serververbindung
für die Speicherung nutzen. Direkte Erreichbarkeit des Pi aus der Anbieter-Cloud
ist keine gewünschte Voraussetzung. Dies präzisiert die vorherige Annahme eines
ausschließlich zentralen MCP-Zugangs.

**Begründung:** Geräte haben bereits Zugriff auf den Pi; dieser soll privat bleiben.

**Konsequenzen:** Lokalen Mac-Adapter prüfen. Mobile Umsetzung nicht als machbar
oder fertig zusagen: Claude unterscheidet lokale Desktop-Erweiterungen von
Remote-Connectors; die lokalen Erweiterungen sind auf Mobile nicht verfügbar.
ChatGPT dokumentiert Server-URL/Tunnel statt lokaler iPhone-Web-App-Anbindung.
Kein unaufgeforderter Cloud-Zugang, keine neue lokale Datenbank/Offline-Sync und
keine eigene Sprachagenten-App aus dieser Präzisierung ableiten. Implementierter
Backend-Adapter bleibt bestehen; lokale Client-Anbindung ist noch offen.

## 2026-10-17 – Desktop-Offline-Nutzung ausdrücklich erforderlich

**Entscheidung:** Desktop erhält eine lokale SQLite-Datenbank und bidirektionalen
Abgleich mit dem Pi. Offline arbeiten und Änderungen nach Wiederverbindung
übertragen. Die iPhone-Web-App bleibt zunächst ein Online-Client. Diese spätere
Entscheidung ersetzt die vorherige Einschränkung „keine neue lokale Datenbank/
Offline-Sync“ und die reine Weiterleitung als Desktop-Ziel.

**Begründung:** Desktop soll auch ohne Serverzugriff verwendbar sein. Der Nutzer
akzeptiert für die iPhone-Web-App vorerst die erforderliche Pi-Verbindung.

**Konsequenzen:** Gemeinsame Rust-Geschäftslogik auf Desktop und Server, lokale
UI-/MCP-Schreibvorgänge über denselben Datenpfad. Dauerhafte Änderungsverfolgung,
wiederaufnehmbarer Abgleich und Konfliktbehandlung sind erforderlich. Deren
konkreter Entwurf steht aus; keine stillen Überschreibungen oder Dateikopien als
Sync. Bestehende Daten schützen und aktuelle Pi-Daten als initialen Bestand
verwenden. Kein automatischer Offline-Modus für ChatGPT/Claude selbst.
Anforderung dokumentiert, Laufzeitfunktion noch nicht implementiert.

## 2026-10-17 – Installation mit Compose/ENV-Downloads wie Immich

**Entscheidung:** Für zukünftige Serverinstallationen Ordner anlegen, zwei
Release-Dateien per `wget` herunterladen, `.env` konfigurieren und Docker Compose
starten. Vorgebaute ARM64-/AMD64-Images ersetzen den Build auf dem Zielserver.

**Begründung:** Nutzer möchte den vertrauten, übersichtlichen Immich-Ablauf,
auch für Neuinstallation auf einem anderen Rechner oder Pi.

**Konsequenzen:** Release-Dateien und Images gemeinsam versionieren, Registry
und Veröffentlichung einrichten, Download-/Update-Anleitung mit Erklärungen
bereitstellen. Standardpfade `/etc/nextleaf` und `/var/lib/nextleaf` bleiben,
Konfigurationsordner frei wählbar und Datenpfad künftig über `.env` einstellbar.
Secrets/Daten erhalten, Sicherung vor Migration, kein automatisches Downgrade
nach Schemaänderungen. Nur Ziel festgehalten; keine Veröffentlichung oder
Umstellung der produktiven Installation in dieser Planungsrunde.

## 2026-10-17 – Native Desktop-Pakete bestätigt

**Entscheidung:** Der Nutzer bestätigt DMG/Setup-EXE/AppImage bzw. DEB als
Release-Downloads und zunächst manuelle Updates durch neue Installationspakete.
**Begründung:** Normale Installation ohne Entwicklerwerkzeuge auf dem Zielgerät.
**Konsequenzen:** Release-Builds, Plattformtests und Signierung vorbereiten,
Benutzerdaten erhalten. Kein integrierter Updater in dieser Etappe. Noch keine
Pakete veröffentlicht; Hosting weiterhin festzulegen.

## 2026-10-17 – Eigene Telefonbedienung in gemeinsamer Web-App

**Anforderung:** iPhone-Web soll sich an Nirvanas iPhone-Bedienung orientieren;
Nutzer stellt sechs Screenshots bereit. Desktop und Telefon benötigen eigene
Darstellung und Interaktion.
**Technischer Ansatz:** Gemeinsame React-Codebasis/API mit adaptiven Oberflächen,
mobile Navigation und Details/Touchmenüs getrennt wo nötig.
**Begründung:** Unterschiedliche Bildschirm- und Eingabeanforderungen ohne
Duplizieren von Fachlogik und Pflege zweier unabhängiger Apps.
**Konsequenzen:** Aktuelle CSS-Verkleinerung reicht nicht. Mobilentwurf und
Safari-/Tastatur-/Touchprüfung erforderlich, Desktop-Layout erhalten. Beobachtete
Nirvana-Screens nicht als Beleg für nicht getestete Gesten/Speicherregeln werten.


## 2026-10-18 – Getrennte People/Tags und stabile IDs im Abgleich

**Entscheidung:** People und normale Tags teilen das Tagmodell mit explizitem
Typ; Areas bleiben eine eigene Entität. Umbenennen und Archiv/Restore erhalten
Zuordnungen. Sync v2 verwendet Tag-IDs statt Namen als Identität, REST-Beziehungen
bleiben zunächst namensbasiert. MCP übernimmt die neuen Verwaltungsoperationen.

**Begründung:** Der Nutzer verlangt getrennte Verwaltung und Kontaktanzeige bei
Waiting. Namen dürfen bei Umbenennung keine Beziehungen verlieren. Archivierung
ist reversibel und erhält den vorhandenen Bestand.

**Konsequenzen:** Additive Migration, keine automatische Umklassifizierung alter
Tags, Erhalt von waiting_for-Freitext, atomarer Schutz vor neuen Zuordnungen zu
archivierten Einträgen. ID-Kanonisierung und Migration gespeicherter v1-Baselines/
ausstehender Anfragen erforderlich. Alte Sync-Protokolle werden abgewiesen;
Pi vor neuem Mac-Bundle aktualisieren. Backend isoliert geprüft, gemeinsame
Frontend-Integration und produktive Ausrollung noch offen.
