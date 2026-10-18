# Arbeitsplan / Nächste Schritte

## Mobile Webansicht und Installationsweg — 2026-10-18

Beauftragt: neue Funktionen auf iPhone-Bedienbarkeit prüfen; danach möchte der
Nutzer selbst Pi, Mac und iPhone vollständig von Nextleaf-Testständen bereinigen
und den endgültigen Installationsweg mit leeren Daten testen. Noch nichts
zurückgesetzt. Installationsanleitung im Repository ausdrücklich gewünscht:
[INSTALLATION.md](INSTALLATION.md). Pi: Downloads plus fertiges Docker-Image und
Compose; Mac: native App/DMG; iPhone: private HTTPS-Web-App/Home-Bildschirm.

Geprüft im Codex-Browser, nicht auf einem echten iPhone: laufender pi7 bei
390×844 und 320×568. Person/Area/Tag anlegen, Person umbenennen/archivieren/
wiederherstellen, Kontaktauswahl im Schnellmenü nach geänderter Texteingabe,
Waiting-Freitext speichern, Tag-Vorschlag antippen, archivierte Zuordnungen
weiter anzeigen und archivierte Tags nicht neu vorschlagen. Abbrechen fragt nach
ungespeicherten Änderungen. Eigene Testaufgabe anschließend in Trash; eigene
Testperson, Testtag und Testarea archiviert. Kein vollständiger Reset.

Kleine Korrekturen im Quellstand: mobile Entfernen-Schaltflächen für Tags/Area
und Tag-Eingabe mindestens 44 px hoch, Entfernen/Edit mindestens 44 px breit;
Verwaltungs-Checkbox 20 px; Personen-/Typ-/Filterfelder 16 px Schrift.
Web-Verbindungshinweis beschreibt jetzt Online-Web statt Desktop.
Korrigierter Build zusätzlich mit separater lokaler DB bei 320×568, 390×844,
390×360 und Desktop 1280×800 geprüft. Keine seitlich überlaufenden geprüften
Dialoge; lange Namen umbrechen, Kopfaktionen bei geringer Höhe sichtbar.
DOM-Messungen bestätigen 44×44-Pixel-Aktionsflächen und 16-Pixel-Formularschrift.
Entwurferhalt über Keep editing und Persistenz von Tag-/Kontaktänderungen geprüft.
39 Frontend-Tests, TS/Vite-Build und git diff --check bestanden.

Diese Mobilkorrekturen sind noch nicht im laufenden pi7 oder im bereits gebauten
Mac-Paket enthalten; sie gehören in das nächste Release. Kein erneutes
Serverupdate in dieser Prüfrunde. Echte iPhone-Tastatur, Safari, Home-Bildschirm-
Start und Gerätewechsel bleiben die explizite Abnahme nach INSTALLATION.md.

Release-Vorlagen unter `deploy/release/`: Image verpflichtend, Pull erlaubt,
Datenpfad über ENV, sonst Schutz-/Loopback-Einstellungen wie bestehende Compose.
Auf dem Pi mit Compose nur validiert: Standard-/alternativer Datenpfad und
fehlendes Image; keine Containeränderung. Kein Image-Pull/Erstinstallationstest.
README von überholten Gerüst-/Sidecar-Angaben bereinigt.

GitHub-Repository auf ausdrückliche Bestätigung zunächst privat angelegt:
https://github.com/silviosackewitz-code/Nextleaf (Remote origin).
Der vollständige bisher lokal entwickelte Quellstand samt Anleitung wird in
Git übernommen; Datenbanken, lokale ENV-Dateien, Pakete/Builds und Python-Caches
sind ausgeschlossen. Noch keine Releases oder Registry-Images veröffentlicht.
Nächster Schritt: fertige Images und Mac-Installer bereitstellen und den
Downloadzugang klären (privates GitHub benötigt Anmeldung), danach Neuinstallation
gemeinsam testen. Signierung/Notarisierung und Open-Source-Lizenz bleiben offen.

## pi7 und Mac-App geprüft — 2026-10-18

Der Nutzer hat den Pi-Updater ausgeführt und `NEXTLEAF_UPDATE_OK` bestätigt.
Anschließend unabhängig geprüft: Docker/Tailscale aktiv, lokale und private
HTTPS-Health erfolgreich mit `tag-management` und `sync-v2`, Web-Dateien passend
zum unveränderten pi7-Paket, anonymer MCP-Zugriff korrekt mit HTTP 401 abgewiesen.
Root-Update-Log, Backup-Inhalte und authentifizierte MCP-Prüfung wurden danach
mangels sudo-Zugang nicht erneut gelesen; dafür liegt die Erfolgsmeldung des
Updaters vom Nutzer vor. Das vorbereitete pi7-Paket bleibt unverändert.

Mac-Debug-Bundle neu gebaut und mit der bestehenden lokalen Datenbank gestartet.
UI zeigt `Synced` / `Changes are synced.`; vollständiger Vergleich mit dem Pi
bestätigt identische Inhalte einschließlich Beziehungen und bestätigter Baseline,
Protokoll 2, keine Konflikte oder ausstehenden Änderungen, SQLite-Integrität/FKs ok.
Bestand: 120 Aufgaben, 11 Projekte, 3 Areas, 13 Tags einschließlich Archiv/Trash.
Die ursprünglich vorhandenen 119 Aufgaben, 11 Projekte, 3 Areas und 12 Tags sind
gegenüber der Momentaufnahme vor den Mac-Tests unverändert.

Beim ersten nativen Test übernahm die separate Testdatenbank eine alte Server-URL
aus dem WebView-Speicher und synchronisierte eine zusätzliche Testaufgabe und
Testperson. Nur diese Aufgabe wurde in Trash verschoben, nur diese Person
archiviert. Ursache in `frontend/src/main.tsx` behoben: automatischer Startabgleich
verwendet ausschließlich die in der jeweiligen Datenbank gespeicherte Verbindung.
Eine neue Datenbank erfordert explizites Connect. Anschließender nativer Test mit
frischer Datenbank blieb lokal trotz alter WebView-URL; Bearbeiten und Persistenz
funktionierten. Der Nutzer hat klargestellt: sämtliche Aufgaben sind Testdaten,
die Originale liegen weiterhin in Nirvana.

Weitere Nachweise: Migration einer Kopie der alten Mac-DB von v1 auf v2 ohne
Verlust bisheriger Tabellen/Beziehungen; lokaler stdio-MCP mit 13 Tools,
Person-Anlage/Umbenennen/Archivieren/Wiederherstellen und erhaltener Task-Verknüpfung;
39 Frontend-Tests, TS/Vite-Build und macOS-Debug-Build bestanden. Manage Tags
öffnet in der laufenden Mac-App. Sicherung vor Migration:
`data/mac-pi7-20261018/mac-before-pi7.db` (ignoriert, nicht einchecken).

Die laufende App liegt unter `src-tauri/target/debug/bundle/macos/Nextleaf.app`.
Zusätzlich wurde ein ZIP der Test-App im Ausgabeverzeichnis dieses Codex-Chats
bereitgestellt. Kein signierter/notarisierter Release. Noch offen: echter
iPhone-/Safari-Test, Windows/Linux, reale Agenten-Clients sowie Commit/Push.
Review/Perspectives bleiben zurückgestellt. Frühere Abschnitte beschreiben den
jeweiligen historischen Stand; pi7-Ausrollung und Mac-Aktivierung sind erledigt.

## Aktueller Stand 2026-10-18 — Frontend-Integration abgeschlossen

Fortsetzung anhand der Repository-Spezifikation und SESSION_HANDOFF.md. Keine
neuen Produktanforderungen beschlossen. Die unterbrochene Tags-/People-Runde
ist jetzt mit der API verbunden und vollständig gebaut.

- Tag/Area-Typen samt Archivstatus, Legacy-Defaults, typed addTag und
  updateTag/updateArea mit expected und tag-management-Fähigkeitsprüfung ergänzt.
- TaskDetails/TaskFields/TaskActions erhalten aktive People; Waiting for bietet
  Kontakte und behält Freitext. Kontaktklick und Tastaturwahl werden nicht durch
  vorheriges Blur-Speichern des Texteingabefelds überholt.
- Archivierte Areas nur bei vorhandener Zuordnung weiter anzeigen. Neue Aufgaben/
  Projekte übernehmen keinen archivierten Area-Filter. Bereits zugewiesene Tags
  bleiben sichtbar, aktive Vorschläge kommen aus der nicht archivierten Liste;
  manuell eingegebene archivierte Namen weist das Backend atomar zurück.
- Manage Tags: Konflikt erhält Entwurf; nach Cancel werden aktuelle Einträge neu
  geladen, damit erneutes Öffnen nicht wieder veraltete Vorbedingungen verwendet.
- Neu geprüft: 39 Frontend-Tests, vollständiger TypeScript-/Vite-Build,
  28 reguläre Backend-Tests plus separater echter HTTP-/Offline-Sync-Test,
  Backend-Clippy all targets ohne Warnungen und macOS-Debug-App-Build.
- Isolierter Browser-Test mit erfundenen Daten, Desktop 1280×800 und Mobil
  390×844: Kontaktauswahl trotz verändertem Freitext, freie Texte im Editor,
  archivierte Zuordnungen sichtbar, archivierte Kontakte nicht neu angeboten,
  Personen umbenennen/archivieren/wiederherstellen, Konfliktentwurf erhalten und
  aktuelle Werte nach Cancel geladen. Kein echter iPhone-/Safari-Test.

**Nächster Schritt: Server-Update vorbereiten und ausrollen.** Nutzer bestätigt
im aktuellen Chat ausdrücklich die Reihenfolge Server vor Mac. Neue Mac-App ist
nur gebaut, nicht mit produktiver DB gestartet. Produktive Daten und Pi wurden
in dieser Fortsetzung nicht verändert. Der letzte dokumentierte Produktivstand
bleibt pi6; kein neuer Live-Abgleich behauptet.

Vor Ausrollung: alten Mac-Prozess beenden, konsistente Sicherungen, neues pi7-Paket
mit tag-management/13 MCP-Tools und Inhaltsvergleich vorbereiten/prüfen, Pi zuerst
aktualisieren; erst danach neues Mac-Bundle auf produktivem Bestand starten.
Sync v2 ist inkompatibel mit v1; kein ungeprüftes Downgrade nach Migration.
Pi7-Paket, Ausrollung, native Laufzeitprüfung dieser Runde sowie Commit/Push bleiben
offen. Review/Perspectives bleiben später; Windows/Linux/Signierung/Restore und
reale Agenten-/Sprachverbindungen bleiben gesonderte Aufgaben.

## Historische Session-Übergabe 2026-10-18 — Integration inzwischen abgeschlossen

**Nutzer hat die Arbeit für einen Sessionwechsel angehalten.** Zuerst
[SESSION_HANDOFF.md](SESSION_HANDOFF.md) lesen. Danach Integration fertigstellen,
prüfen, Mac/Pi-Paket vorbereiten und den ausdrücklich beauftragten Commit/Push
abschließen. Noch kein Commit/Push dieser Änderungen erfolgt. Keine Agentenarbeit
mehr erforderlich; die Ergebnisse liegen bereits im gemeinsamen Arbeitsbaum.

### Neue Bedienrunde: Tags und Listenoptionen

- Backend umgesetzt: Tags/People-Typen, Umbenennen, Archiv/Restore für Tags und
  Areas, `expected`-Konfliktschutz, Health-Fähigkeit `tag-management`, 13 MCP-Tools.
  Sync v2 verwendet stabile Tag-IDs und migriert gespeicherten v1-Clientzustand.
- Frontend-Komponenten umgesetzt: Manage Tags, kontextabhängige Filter,
  Waiting-Kontakt vor Titel, drei Notizmodi, Untagged, View Options nur Next/Focus,
  Project actions Top 1–9/All, Hover-Untermenü und mobile Auswahl.
- **Integration noch offen:** api.ts hat noch alte Tag-/Area-Typen und keine
  updateTag/updateArea-Methoden; TaskDetails/TaskActions/TaskFields fehlen
  people-Props und Kontaktauswahl. Vollständiger TypeScript-Build daher noch
  nicht bestanden; bisheriger Vite-Vorschaubuild ersetzt diese Prüfung nicht.
- Agenten-Prüfergebnisse: 28 reguläre Backend-Tests plus separater echter
  HTTP-/Offline-Test, Backend-Clippy/Formatprüfung; 33 Frontend-Tests bestanden.
  Isolierte Browserprüfung mit erfundenen Daten: Next-Limit/Notizmodi, Waiting,
  Desktop-Hover und mobile 390×844-Menüs. Kein Produktivbestand verändert.
- Neue Bedienrunde noch nicht gebaut/ausgerollt als Mac-Bundle oder Pi-Paket.
  Produktiv gilt weiterhin der geprüfte pi6-Stand. Nutzer bestätigt inzwischen,
  dass die bisherige iPhone-App funktioniert; kein eigener umfassender Safari-Test.
- **Update-Reihenfolge:** alten Mac-Prozess beenden, Sicherungen erstellen,
  Pi zuerst aktualisieren, dann neue Mac-App öffnen. Sync v2 lehnt v1 ab;
  kein automatisches Downgrade nach möglicher Datenmigration.

## Aktueller Stand: Mobile, Desktop offline und lokales MCP (2026-10-18)

Implementiert: adaptive iPhone-Web-Oberfläche (Drawer, New-Item-FAB, mobile
Details, Filter und Touchmenüs), ausgerichtete Projects/Tags-Überschriften,
Überfälligkeit nur am Abschlusskreis, rote Inbox-/Focus-Zähler. Focus zeigt
überfällige Aufgaben rot und die übrigen offenen grau (beide zusammen = Gesamt).
Mac-App verwendet eingebettete Rust-Fachlogik und eigene SQLite; stdio-MCP
(`--mcp`) greift auf dieselbe DB zu. Abgleich alle 15 Sekunden während App oder
MCP-Prozess laufen, dauerhafte ausstehende Änderungen und explizite Konfliktwahl.
Veraltete Task-/Projektentwürfe überschreiben dank atomarer `expected`-Prüfung
keine neueren Feldwerte. Details in ARCHITECTURE.md und MCP.md.

Geprüft auf macOS:
- 20 reguläre Backend-Tests und zusätzlicher echter HTTP-/Offline-Test;
  Backend- und Desktop-Clippy ohne Warnungen, Rust-Formatprüfung.
- 27 Frontend-Tests, TypeScript/Vite und gepackte Tauri-Debug-App erfolgreich.
- Mobile Browserprüfung mit erfundenen Daten bei 390×844: Navigation mit roten
  Zählern, Task-Editor ohne Abschlusskreis, Kalender mit Abdunklung, Speichern
  eines Datums und Entfernen der Überfälligkeitsmarkierung. Kein echter iPhone-Test.
- Gepacktes Mac-Programm per stdio: 11 Tools, Task anlegen/ändern, Prozessneustart,
  erneutes Lesen und Abschluss, SQLite-Integrität. Eigene temporäre Test-DB.
- Native Mac-App liest denselben Testbestand, öffnet abgeschlossene Aufgabe offline
  erneut. Erfolgreicher Initialabgleich mit separatem lokalen Rust-Testserver;
  gleichzeitiger MCP-Prozess ändert Notizen, Änderung erreicht Testserver.
  Titeländerung am Testserver erscheint automatisch in Mac-App. Keine Pi-Aufgaben
  oder produktiven Daten für diese Schreibtests verwendet.
- Native Konfliktanzeige mit voneinander abweichenden Task-Titeln geprüft;
  beide Werte bleiben bis zur ausdrücklichen Auswahl erhalten. Serverfassung
  ausdrücklich bestätigt, anschließend Synced und korrekter Titel.
- Native App-Dialoge statt window.confirm: Keep editing erhält den Entwurf;
  Discard changes verwirft ihn und führt die gewählte Navigation aus.
  Erneuter finaler Mac-Build erfolgreich.
- Gezielte unabhängige Codeprüfung: korrigierbare fehlgeschlagene Erstverbindung,
  erneute Prüfung vor Konfliktauflösung, veraltete Editorwerte und parallele
  GUI-/MCP-Abgleiche ohne weitere gefundene Blocker.

Ausrollung pi6 am 2026-10-18 vom Nutzer abgeschlossen und unabhängig geprüft:
Docker/Tailscale aktiv, Health lokal und über privates HTTPS erfolgreich,
Fähigkeiten sync/conditional-updates vorhanden. Web-Assets entsprachen zum
Prüfzeitpunkt bytegenau dem damaligen lokalen Build. Mac-App gestartet und
**Synced** bestätigt. Vollständiger Snapshotvergleich Pi ↔ Mac: 119 Aufgaben,
11 Projekte, 3 Areas, 12 Tags identisch; Baseline identisch, keine Konflikte oder
ausstehenden Änderungen. SQLite-Integrität/Fremdschlüssel erfolgreich geprüft.
Keine produktiven Aufgaben für Schreibtests verändert. Anonymes MCP liefert 401.
Mac-DB: `~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
Der danach ergänzte Untagged-Filter ist noch nicht ausgerollt.

Offen: echtes iPhone/Safari samt
Tastatur/Safe Areas/Gesten, konkrete ChatGPT-/Claude-Konfiguration und Sprachtest,
Windows/Linux-Desktop, signierte öffentliche Release-Pakete, Backup-Restore.
Lokales stdio-MCP stellt keine automatische lokale Verbindung zu iPhone-Agenten her.
Perspectives und Review ausdrücklich später; Recherche in REVIEW_RESEARCH.md.
Perspectives sollen den Area-Selektor ersetzen und mehrere Areas/Tags bündeln.

Die folgenden Abschnitte dokumentieren den vorherigen Verlauf; bei Widersprüchen
gilt dieser aktuelle Stand.

## Neuester Stand: iPhone-Referenz und Pi5 (2026-10-17)

Sechs vom Nutzer bereitgestellte iPhone-Screenshots in Fotos angesehen und unter
UX_REFERENCE.md ausgewertet. Eigene mobile Bedienung erforderlich; eine gemeinsame
React-App mit getrennten Layout-/Interaktionsvarianten ist der technische Ansatz.
Agent hat bestehende mobile CSS/Komponenten nur lesend geprüft. Noch kein mobiler
Umbau, keine echte iPhone-/Safari-/Gestenprüfung. Details in SPECIFICATION.md.
Desktop-Release-Installationsweg vom Nutzer inzwischen bestätigt, nicht nur Vorschlag.

Pi5: Nutzer meldet NEXTLEAF_UPDATE_OK. Danach Docker/tailscaled aktiv, Web HTTP 200,
Health lokal und über Tailscale-HTTPS ok mit project-editing und mcp enabled true;
MCP ohne Anmeldung korrekt 401. Authentifizierten MCP-Test nicht erneut ausgeführt
(sudo verlangt Passwort). Keine Produktivdaten geändert. Frühere Hinweise unten
auf ausstehende pi5-Ausführung sind damit überholt; Agentenverbindung bleibt offen.

## Geplante einfache Serverinstallation (2026-10-17)

Nutzerwunsch für künftige Releases: Ordner → Compose/ENV per wget → konfigurieren
→ Compose starten, wie Immich. Anforderung dokumentiert, **noch nicht verfügbar**.
Dafür Release-/Registry-Ziel festlegen, ARM64-/AMD64-Images bauen und veröffentlichen,
passende Compose/ENV-Dateien liefern, lokale-only Image-Regel ersetzen und
Datenpfad über ENV konfigurierbar machen. Datenrechte und bestehende private
Netzgrenze beibehalten. Neuinstallation, Update und Datenerhalt auf isolierten
Instanzen prüfen; Anleitung mit erklärten Befehlen. Bestehende Pi-Installation
und vorbereitete Update-Pakete in dieser Runde unverändert.

## Neueste Anforderung: Desktop offline, iPhone-Web online (2026-10-17)

Beschlossen und dokumentiert, **noch nicht implementiert**: lokale SQLite-DB
für Desktop, offline lesen und ändern, später bidirektional mit dem Pi abgleichen.
Lokale MCP-Operationen sollen denselben Bestand und dieselben Rust-Regeln nutzen.
Web-App auf iPhone bleibt online und braucht einen erreichbaren Pi.

Nächster Entwurf: gemeinsame Rust-Fachlogik für Desktop/Server, lokaler Dienst,
Initialabgleich, dauerhafte ausstehende Änderungen, Versions-/Konfliktregeln,
Löschung/Wiederherstellung und sichere Wiederaufnahme bei Verbindungsabbrüchen.
Danach isolierte Tests für Offline-Neustart, Änderungen von zwei Geräten,
Reconnect, wiederholte Zustellung und Konflikte; erst dann Datenübernahme.
Alte Mac-DB ist weiterhin nur Rückfallstand. Der aktuelle Client schreibt noch
direkt auf den Pi. Keine Laufzeit- oder Produktivdatenänderung in dieser
Planungsrunde; Dokumentationskonsistenz und Diff-Format geprüft.

## Vorherige Präzisierung: lokaler Agentenzugriff (2026-10-17)

Gewünscht: Agent auf Mac/iPhone → lokale Nextleaf-Anbindung → bestehende private
Pi-Verbindung. Keine direkte Anbieter-Cloud-Erreichbarkeit des Pi voraussetzen.
Lokaler Adapter noch nicht gebaut. Claude Desktop unterstützt lokale Erweiterungen,
Claude Mobile diesen Weg nicht; ChatGPT dokumentiert URL/Tunnel, keine lokale
Brücke zu unserer iPhone-Web-App. Zunächst diese mobile Einschränkung klären;
keine Cloud-Freigabe oder eigene Agenten-App ohne passenden Auftrag einrichten.
Der vorhandene Server-MCP und das private pi5-Paket bleiben bestehen.

## MCP und hervorgehobener Editor: lokaler pi5-Stand (2026-10-17)

Auf ausdrücklichen Auftrag jetzt implementiert: zentraler `/mcp`-Endpoint mit
elf Tools für alle vorhandenen fachlichen API-Operationen; offizielles Rust-SDK,
Bearer-Schlüssel, Host-Allowlist, Origin-Abweisung, ohne Schlüssel deaktiviert.
Keine duplizierten GTD-Regeln. Umfang, lokale UI-Ausnahmen und künftige Funktionen
in [MCP.md](MCP.md); Review bleibt nächste Spezifikationsrunde.

Geprüft: 13 Rust-Tests (inklusive aller elf MCP-Tools über den HTTP-Service,
Initialisierung, Authentifizierung, Host/Origin, Partial Updates, Null-Löschung,
Fehler ohne Teiländerung, Sequential/Parallel, Abschluss und Wiederöffnung).
20 Frontend-Tests bestanden. Zusätzlich echter lokaler Prozess mit isolierter DB: Read-only-Prüfprogramm meldet
NEXTLEAF_MCP_OK. Mac-Debug-Build und TypeScript/Vite erfolgreich; native Mac-App
neu geladen, Pi-Daten lesbar, Projects-Übersicht/Editor und Kalender samt Backdrop
geprüft. Task-Editor jetzt mit dunklerem Hintergrund, deutlicherem Rand und Schatten;
kein Abschlusskreis im offenen Editor. Keine produktiven Datensätze verändert.

Pi5-Update vorbereitet: enthält pi4-Projektfunktionen, MCP und Editor-Kontrast.
Sechs isolierte Updater-Szenarien bestanden (Erfolg, bestehender Schlüssel,
Build-/Start-/Health-/MCP-Fehler); Daten bleiben erhalten, Schlüssel bleibt geheim,
vorhandene Konfiguration bleibt erhalten, kein Downgrade nach möglicher Migration.
Pi läuft bei letzter Live-Prüfung noch pi3; sudo benötigt Nutzereingabe.
Pi5-Paket übertragen: 72 Dateien per SHA-256 und Shell-Syntax auf dem Pi
bestätigt (NEXTLEAF_PACKAGE_VERIFIED). Tatsächliche Ausführung steht aus;
Nutzerbefehl: `sudo bash ~/nextleaf-update-mcp-20261017/update.sh`.

Gewünschte Clients: ChatGPT und Claude, auch per Sprache auf iPhone und Mac.
MCP-Unterstützung allein macht den privaten Pi nicht aus der Anbieter-Cloud
zugänglich. OpenAI Secure MCP Tunnel und Claude-Remote-Zugang dokumentiert;
Cloud-Anbindung, Konto-Anmeldung und reale Sprachtests noch **nicht eingerichtet**.
Keine öffentliche Freigabe. Pi5 bleibt ausführbar; für den anschließenden
Agentenzugang gilt die neueste Präzisierung oben statt einer vorgegebenen
Cloud-Verbindung. Windows/Linux-Desktop und Restore weiterhin offen.

## Projekte und Editor: lokaler pi4-Stand (2026-10-17)

Umgesetzt: Projects-Übersicht mit Action-Zählern und aufklappbaren Einstellungen;
Name/Notizen/Area/Tags/Fälligkeit/Focus/Status/Parallel-Sequential speicherbar.
Plus entfernt, New-Item-Pfeil identisch zu All Areas. Eigenschaftswerte ohne
zusätzliche Labels, Tagzeile mit Enter/Vorschlagsklick und Area-Pfeil, gelber Stern,
kein Abschlusskreis im offenen Editor. Gemeinsamer Kalender mit Backdrop auch
für Due/Starts im Schnellmenü. Additive Migration und project-editing-API.

Geprüft: 20 Frontend-Tests, 11 Rust-Tests inkl. atomarem Projekt-Update, Rollback,
Feldlöschung, Erhalt der Aufgabenreihenfolge und Next-Regeln bei Moduswechsel.
Browser mit separater DB: Projektübersicht/0 Actions, Editor speichern/wiederöffnen,
Enter für neue Tags, Klick auf Vorschlag, unbestätigten Text nicht speichern,
Focus/Sequential, Kalenderwahl im Editor und direkte Speicherung im Schnellmenü.
TypeScript/Vite und macOS-Debug-Bundle erfolgreich; gemeinsames Pfeil-SVG und
vertikale Zentrierung im Browser bestätigt. Vier simulierte pi4-Updater-Szenarien
bestanden; nach möglichem Migrationsstart kein unsicheres Downgrade.
Native Prüfung nach Entsperren abgeschlossen: neuer Mac-Build geladen, Projects,
Inline-Editor und Kalender mit Backdrop geprüft; ohne Speichern am Produktivbestand.
Pi4-Paket übertragen: 69 Dateien auf Pi per SHA-256 und Skript-Syntax geprüft.
Tatsächlicher Lauf noch offen; Pi bleibt pi3. Kein produktiver Datenbestand verändert.


## Pi-Update bestätigt und nächster Änderungsauftrag (2026-10-17)

- Nutzer bestätigt NEXTLEAF_UPDATE_OK. Danach unabhängig geprüft: Docker und
  Tailscale aktiv; Health lokal und über HTTPS ok mit task-editing/item-creation.
  JavaScript und CSS vom Pi stimmen per SHA-256 mit dem aktuellen Mac-Frontend
  überein. API liefert 12 Tags, 11 Projekte, 3 Areas, 16 Next-Aufgaben und
  23 Logbook-Einträge; All liefert 80 Einträge (kein Gesamtbestand aller Status).
- Native Mac-App nutzt bestätigte Pi-HTTPS-Adresse. Refresh lädt jetzt alle
  zwölf Tags; New Item enthält Focus/Project/Tag, Tag-Dialog öffnet korrekt.
  Keine produktiven Aufgaben oder Tags für diese Prüfung verändert.
- Grenzen: privilegiertes Update-Protokoll/Docker-Details ohne sudo-Passwort
  nicht lesbar; kein erneuter Schreibtest am Produktivbestand, kein Restore-Test.
- **Nächster Änderungsdurchlauf:** Plus neben Projects entfernen. Projektanlage
  ausschließlich über New Item oben. Im folgenden lokalen pi4-Stand umgesetzt.
  Pfeil bei New Item vertikal zentrieren und Form/Größe mit All Areas vereinheitlichen.

## Aktueller Gesamtstand: kompakte Bedienrunde (2026-10-17)

Zusätzlich Tags ergänzt: einklappbare umbrechende Filterliste, All/Einzelauswahl.
Test mit 16 zusätzlichen Projekten ausschließlich in lokaler Test-DB: 1011px
Sidebar-Inhalt in 515px Scrollbereich, Footer vor/nach 496px Scrollweg unverändert
bei y=575px und 133px Höhe. Tag Laptop zeigt eine passende Aufgabe, All wieder fünf.
Aktuelle Mac-App danach erneut gebaut und gestartet: Tags, gruppierte Pi-Aufgaben
und ausgerichtete feste untere Navigation nativ visuell bestätigt. Der aktuelle
Pi-Server liefert über den Kompatibilitätsweg bereits verwendete Tags; die volle
Tag-Liste einschließlich unbenutzter Tags folgt mit dem vorbereiteten pi3-Update.

Letzte Präzisierung umgesetzt: Projektgruppen (Actions für projektlose Aufgaben)
statt Projektlabel pro Task; Einzelprojekt ohne doppelte Überschrift. Capture-Zeile
entfernt. Areas nur als oberer Filter und Verwaltung in Settings; Filter bleibt beim
GTD-Ansichtswechsel erhalten. Settings nutzt gleiche Zeilen-/Icon-Ausrichtung unten.
Geprüft: 16 Frontend-Tests inklusive Gruppenreihenfolge, leerer/fehlender Projekte
und Datenerhalt. Browser mit Test-DB: keine Projektlabels in Zeilen, keine doppelte
Überschrift im Einzelprojekt, New Item erstellt Aufgabe im geöffneten Projekt;
Area-bezogene Anlage und Wechsel Next → Waiting → Next behalten Filter/Daten.
Abschluss/Wiederöffnen innerhalb einer Projektgruppe verschiebt keine Zeile.


Nach Nutzerkorrektur: Schnellmenü durch direkte Schnellauswahl ersetzt. Schmales
Hauptmenü rechts, separates Hover-Untermenü links, kein Save/Cancel. Tastatur und
Klick ebenfalls möglich. Titel/Tags umbrechen unabhängig auf bis zu zwei Zeilen;
Kopfbereich, Sidebar und Zeilen verdichtet. Completed-Banner vollständig entfernt.
Browser-Test mit isolierter DB: Auswahl sofort gespeichert und beim Wiederöffnen
bestätigt; Hover ohne Kategorie-Klick, Tastatur links/rechts, Fehlerfall bei gestopptem
Testserver ohne falsche Bestätigung, erfolgreicher erneuter Versuch. Lange Titel und
vier Tags umbrechen; 30 Notizzeilen bleiben in 130px Textbereich scrollbar. Abschluss
und Wiederöffnen geprüft; Zeilenpositionen und Höhen unverändert, kein Banner.
Datumswahl mit Enter geprüft: tatsächlich gewähltes Datum gespeichert und in der
Liste zurückgelesen (native Eingabewerte direkt übernehmen).
Mac-App neu gebaut und gestartet: Pi-Daten, dichtere Liste und linkes Untermenü
nativ visuell geprüft; Escape schließt beide Menüs. Pi-Paket durch korrigierten
Stand ersetzt, 62 Dateien per SHA-256 auf Pi verifiziert; Nutzer-Ausführung offen.
Der folgende ursprüngliche Prüfstand gilt soweit durch diese Korrektur nicht ersetzt.


- Besprochene UI-Punkte gemeinsam umgesetzt: fünf Standardfelder im Inline-Editor,
  More options für Starts/Area/Waiting/Focus, kürzere Notizen und engere Abstände.
  Schnellmenü öffnet per Mausbewegung; Kategorien bleiben stehen, eine Eigenschaft
  erscheint daneben. Klick und Tastatur bleiben unterstützt.
- New Item für alle Erfassungsziele, Project und Tag; Tag-Vorschläge im Editor.
  Focus wird atomar mit der Aufgabe angelegt. Schnelle Erfassung folgt der Ansicht,
  im Projekt immer Next; permanenter Statuswähler entfällt. Projektkontext bleibt.
- View options: Projekt-/Area-Hinweis, Tags und Fälligkeit je Ansicht ein-/ausblenden,
  lokal gespeichert. Die bereits umgesetzte Sidebar, Projektanlage, einzeiligen
  Aufgaben und vorübergehend sichtbaren Abschlüsse bleiben Bestandteil des Stands.
- Geprüft: zehn Rust-Tests, 16 Frontend-Tests, Formatprüfung, TypeScript/Vite und
  macOS-Debug-Build. Testdatenbank im Browser: fünf primäre Felder, zusätzliche
  Felder erreichbar, Editor bei 1280px Fensterbreite etwa 348px hoch. Pointer-Wechsel
  ohne Kategorie-Klick per Mausbewegung geprüft; Zeilen bleiben unverändert,
  Energy-Änderung bleibt beim Wechsel zu Time erhalten und wird gespeichert.
  New Item: Tag erstellt und als Vorschlag vorhanden, Focus-Aufgabe angelegt und
  in Focus angezeigt. Projekt-Schnellerfassung in Next sichtbar. Tags ausgeblendet
  und Einstellung nach Neuladen erhalten. Keine produktiven Aufgaben verändert.
- Pi-Neustart vom Nutzer durchgeführt: per SSH kurze Laufzeit bestätigt,
  docker/tailscaled aktiv, Nextleaf lokal und über HTTPS erfolgreich erreichbar.
  Neu gestartete native Mac-App lädt die Pi-Daten; kompakter Editor mit fünf
  primären Feldern und More options visuell bestätigt.
  Auf Pi läuft weiterhin pi2 mit `task-editing`. Gemeinsames pi3-Update für die
  neue Bedienrunde auf Pi übertragen, SHA-256 und Shell-Syntax dort geprüft.
  Ausführung durch Nutzer steht aus; nicht als ausgerollt behandeln.
- Review bleibt nächste Spezifikationsrunde. Perspectives/MCP und vollständige
  Projekt-/Area-Bearbeitung bleiben spätere Funktionsetappen. Cleanup-Zeitpunkt
  unverändert bis zur Klärung; keine zusätzlichen Nirvana-Funktionen allein
  aufgrund der Screenshots übernommen.
- Offen: tatsächlicher pi3-Build/Wechsel auf Pi, Windows/Linux-Desktop und
  vollständiger Backup/Restore-Test. Aktueller Paketstatus: DEPLOYMENT.md.

## Vorheriger Prüfstand: pi2 und erste UI-Runde (2026-10-17)


- Mac-App auf den gemeinsamen Pi-Server verbunden; Zugriff im nativen Fenster
  mit 19 Next-Aufgaben und 11 Projekten bestätigt. URL bleibt lokal gespeichert,
  keine Serveradresse im Repository. Alte Mac-DB nicht überschrieben/synchronisiert.
- Umgesetzt: erledigte Aufgaben vorübergehend durchgestrichen behalten,
  Checkbox am Titel, aufklappbare Details in der Liste, Tag-Chips, gleichmäßige Projektliste,
  Areas in Settings, Plus/Dialog für Projekte, direkte Erfassung im Projekt,
  kurze Leeransicht, entfernte Hilfstexte. Inbox separat oben; Logbook/All tasks/
  Trash unten; doppelte Focus-Überschrift und „GTD / …“ entfernt. Projects
  ein-/ausklappbar; Plus bleibt sichtbar.
- Geschlossene Aufgaben einzeilig; Inline-Editor für Titel, Notizen, Tags,
  Time/Energy, Termine, Status, Projekt/Area, Waiting for und Focus. Save changes/
  Cancel; rechter Zeilenpfeil öffnet ein Schnellmenü mit denselben Feldern.
  Backend validiert Änderungen und speichert Aufgabe/Tag-Verknüpfungen atomar.
  Fehlend bleibt unverändert, null löscht optionale Angaben. Keine neue Migration.
- Geprüft: neun Rust-Tests, Rust-Formatprüfung, sieben Frontend-Tests,
  TypeScript/Vite und macOS-Debug-Bundle. Tests prüfen unter anderem Feldlöschung,
  Erhalt unveränderter Inhalte, Fehler-Rollback, Projektwechsel/Reihenfolge sowie
  Schutz vor alten Servern, die neue Felder still ignorieren würden.
- Browserprüfung mit eigener Test-DB: Projektanlage Sequential, Erfassung im
  Projekt, Abschluss bleibt sichtbar/Folgeaufgabe verfügbar, Undo nach Waiting,
  Refresh/Logbook, Area-Anlage, Dialog/Escape/Fokusrückkehr, lange Projektzeilen.
  Inline-Editor: Titel, mehrzeilige Notizen, Tags, Time/Energy und Datum gespeichert
  und erneut geöffnet; Cancel verwirft ungespeicherte Notizen. Schnellmenü:
  Energy gelöscht und Status geändert; übrige Inhalte erhalten. Menü bleibt im
  Fenster und scrollt. Navigations-Rückfrage bei ungespeicherten Änderungen ist
  implementiert, ihr Browser-Dialogtest war nicht eindeutig; separat nachprüfen.
- Neu gestartete native Mac-App lädt Pi-Daten; einzeilige Liste, Inline-Editor,
  flache Auswahlfelder und Schnellmenü visuell geprüft. Escape im Editor führt
  zurück zur Titel-Schaltfläche. Keine produktiven Aufgaben für Tests verändert.
  Projektliste per Tastatur ein-/ausklappbar; Plus bleibt erreichbar. Untere
  Navigation bleibt auf Desktop sichtbar, mittlere Sidebar scrollt.
- Pi-Update auf pi2 vom Nutzer erfolgreich ausgeführt (`NEXTLEAF_UPDATE_OK`).
  Danach unabhängig geprüft: Health lokal auf Pi und über Tailscale-HTTPS meldet
  `task-editing`; ausgelieferte HTML-/JS-/CSS-Dateien entsprechen dem aktuellen
  Mac-Build. Tailscale Serve bleibt auf privaten Tailnet-Zugriff begrenzt.
- End-to-End nach Update: eigene Testaufgabe in der nativen Mac-App erstellt,
  mehrzeilige Notizen und Time im Inline-Editor gespeichert, Energy über das
  Schnellmenü geändert. Nach Refresh wieder geöffnet und zusätzlich über die
  Pi-API zurückgelesen: alle Werte erhalten. Testaufgabe anschließend ausschließlich
  in wiederherstellbaren Trash verschoben; vorhandene Aufgaben nicht geändert.
  Mac-App kann die neuen Felder jetzt nutzen. Kein erneuter Mac-DB-Import.
- Root-geschütztes Update-Protokoll und Backup-Inhalt in dieser Nachprüfung
  nicht direkt lesbar; erfolgreicher Skriptabschluss vom Nutzer bestätigt.
  Vollständiger Restore- und Pi-Neustart-Test bleiben offen. Details: DEPLOYMENT.md.
- Nirvana direkt angesehen und Verhalten in [UX_REFERENCE.md](UX_REFERENCE.md)
  festgehalten. Als Nächstes das gewünschte zentrale New Item-Menü, danach
  Cleanup-Verhalten und Ansichtsoptionen einzeln besprechen. New Item ist
  vorgemerkt, noch nicht implementiert.
- Geplant: Bearbeitung von Projekten/Areas, Reihenfolge, konfigurierbare
  Zusatzinformationen in Listen; danach weiterer GTD-Ausbau, Perspectives
  und vollständige MCP-Schnittstelle. Nicht als umgesetzt behandeln.
- Nächste Spezifikationsrunde: Review anhand OmniFocus (besonders iPad) planen;
  Umfang und Intervalle gemeinsam klären. Ausdrücklich noch nicht implementieren.
- Offen: Windows/Linux-Desktop, Pi-Neustart, gesonderter Backup/Restore-Test.

Die folgenden Abschnitte enthalten auch frühere Prüfschritte; bei widersprüchlichen
Statusangaben gilt der aktuelle Arbeitsstand oben.

## Aktuelle Etappe: echte Daten und nächste Funktionen

- Nirvana-MCP nach App-Neustart verfügbar; OAuth und Lesezugriff bestätigt.
  Alle 128 gelieferten Objekte mit Notizen abgerufen (`has_more=false`),
  Logbook/Trash separat abgeglichen. Kein Schreiben nach Nirvana.
- Auf ausdrücklichen Nutzerwunsch alle 11 Projekte Parallel. 117 Aufgaben,
  3 Areas und 12 Kontext-Tags übernommen; vorhandene lokale Aufgabe erhalten.
  Quelle/Backups unter ignoriertem `data/`; niemals einchecken.
- Schema/API/Anzeige um Later, Trash, Wiederholungsvorlagen, Focus, Notizen,
  Tags und Datums-/Waiting-/Zeit-/Energieangaben ergänzt. Keine automatischen
  Wiederholungen: Regel fehlt im MCP-Abruf. Manuelle Reihenfolge und genaue
  Uhrzeiten ebenfalls nicht geliefert; vollständiger Kontoexport ungeprüft.
- Auswahlfelder auf Nutzerwunsch flach im App-Design gestaltet. Browser-
  Sichtprüfung und native Auswahlbedienung erfolgreich; macOS-Bundle gebaut.
- Nutzer findet abgehakte Aufgaben nicht wieder: Einträge sind vorhanden.
  Logbook nach Abschlussdatum absteigend sortieren, direkte Rückmeldung mit
  View Logbook und Undo. Browser-Prüfung mit separater Testdatenbank bestanden:
  Abhaken → Rückmeldung → Logbook → Undo → Aufgabe wieder in Next.
- Sieben Rust-Tests bestanden: GTD-Regeln, Trash/Vorlagen, Logbook-Sortierung,
  Migration bestehender Links, Import-Rollback, Wiederholung ohne Duplikate
  oder Überschreiben lokaler Änderungen. Probelauf: alle 128 Objekte und
  Beziehungen gegen Quelle verglichen, lokale Daten erhalten; erneuter Lauf
  überspringt alle 128. Live-API, SQLite-Integrität und Fremdschlüssel geprüft.
- TypeScript/Vite und Tauri-Debug-App gebaut. Backend und Desktop-App gestartet.
  Eigene native Sichtprüfung sowie Windows/Linux-Desktop noch offen.
- Pi-Terminalausgaben vom Nutzer geprüft: Ubuntu 24.04.5 LTS Noble und ARM64.
  Docker CE 29.9.1 und Compose 5.6.0 installiert; ARM64-hello-world erfolgreich.
  Socket-Startproblem durch Neuladen von systemd und geordneten Neustart behoben
  (siehe DEPLOYMENT.md). Docker-Autostart nach Pi-Neustart noch ungeprüft.
  Tailscale installiert und angemeldet: Nutzer-Ausgabe zeigt Pi, Mac und iPhone
  im selben Tailnet. SSH vom Mac und durch Codex erfolgreich; sudo erfordert
  Nutzer-Passwort im Terminal, keine Passwörter im Chat.
- Nextleaf-Installation mit aktuellem Mac-Datenbestand beauftragt. Konsistente
  Sicherung (118 Aufgaben/11 Projekte) und Quellstand auf den Pi übertragen,
  SHA-256 und Datenvergleich bestanden. Einmaliges Skript unter
  `~/nextleaf-setup-20261017/install.sh` vom Nutzer erfolgreich ausgeführt:
  ARM64-Build, sieben Rust-Tests, Health/Web-Auslieferung und vollständiger
  Datenvergleich vor/nach Container-Neuerstellung bestanden. Protokoll und
  Healthcheck von Codex per SSH geprüft. Tailscale Serve mit HTTPS vom Nutzer
  aktiviert; Konfiguration und Zertifikat, API/HTML/CSS/JS vom Mac aus geprüft.
  Browser-/iPhone-Sichtprüfung, Pi-Neustart und Backup/Restore noch offen.
  Installationsdetails und Wiederaufnahme in DEPLOYMENT.md. Während des Umzugs
  lokale App nicht weiter bearbeiten; keine Synchronisation zwischen den DBs.
- Nächste Funktionen gemeinsam planen: Bearbeitung von Titel/Notizen/Zuordnung,
  Projekten/Areas und Reihenfolge, danach weiterer GTD-Ausbau. Importdetails
  und Grenzen: [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

## 0. Gerüst auf dem Mac verifizieren
- [x] Lokalen Checkout unter `~/Developer/repo/TaskManager` und vorhandenes Git-Repository prüfen.
- [x] Rust, Node und Tauri-Systemvoraussetzungen prüfen; fehlendes Rust installieren.
- [x] Backend: `cargo fmt --check`, `cargo test`, `cargo run`.
- [x] Frontend: `npm --prefix frontend install`, `npm run web:build`, `npm run web:dev`.
- [x] Tauri: `npm install`, `npm run desktop:dev`.
- [x] SQLite-Datenbank und Web-Listendarstellungen mit Testaufgaben prüfen.
- [x] Testen: Sequential-Projekt A→B: A ist Next, B nicht; nach Abschluss von A wird B Next. Bei A=Waiting ist B weiter blockiert.
- [x] macOS-App mit eingebautem Frontend bauen (`npm run desktop:build -- --debug --bundles app`).
- [x] Natives macOS-Fenster und dessen Backend-Verbindung visuell prüfen (aktueller Stand mit Pi-Verbindung).
- [ ] Windows- und Linux-Builds auf den jeweiligen Systemen prüfen.

### Prüfstand vom 2026-10-17

- Umgebung: macOS auf Apple Silicon, Node 22.23.2, npm 10.9.8,
  Xcode 26.3; Rust 1.99.1 über den offiziellen rustup-Installer eingerichtet.
  Falls nötig vor Cargo-Befehlen `source "$HOME/.cargo/env"` ausführen.
- Fehlenden Axum-Import `patch` ergänzt; Rust-Quellcode mit `cargo fmt` formatiert.
  Keine UI-Umgestaltung vorgenommen.
- Backend: drei Unit-Tests bestanden. HTTP-Test gegen einen laufenden Server
  mit eigener temporärer SQLite-Datei bestanden: Health, Areas/Projekte/Aufgaben
  anlegen, Titel/Status ändern, Abschluss und Wiederöffnung, Parallel/Sequential,
  Waiting-Blockade, Area-/Projektfilter, Logbook, ungültige Eingaben, 404 und CORS.
- SQLx-Migration erfolgreich angewandt. Persistierte Testdaten, SQLite-Integrität
  und Fremdschlüssel geprüft; keine Testaufgaben in die normale Entwicklungs-DB geschrieben.
- TypeScript-Prüfung und Vite-Produktionsbuild erfolgreich; Web-App im Browser
  mit Testdaten und funktionierender API-Verbindung geprüft. Statische Auslieferung
  des Builds über Axum ebenfalls geprüft.
- Tauri-2-Konfiguration, Capabilities, Icons und Frontend-Pfade geprüft.
  macOS-Entwicklungsbuild erfolgreich, Desktop-Prozess gestartet.
- Gepacktes macOS-Debug-Bundle erfolgreich erzeugt unter
  `src-tauri/target/debug/bundle/macos/TaskManager.app`. Die native Sichtprüfung
  war durch fehlende macOS-Berechtigungen für Computer Use blockiert; Fenster
  und API-Verbindung im gepackten Client sind noch nicht visuell bestätigt.
- npm- und Cargo-Lockfiles für beide Teilprojekte erzeugt. TypeScript-Buildcache
  wird ignoriert; wirkungslose `VITE_API_BASE_URL`-Beispielvariable entfernt.
- Dokumentation und Sitzungsübergabe: `AGENTS.md`, `SPECIFICATION.md`,
  `ARCHITECTURE.md`, `DECISIONS.md` und dieser Arbeitsstand sind eingerichtet.
- Noch nicht geprüft: Windows/Linux, signierte Release-Pakete und Installer.
  Backend und Desktop bleiben getrennte Prozesse; automatischer Serverstart ist geplant.
- Zum Abschluss Backend mit Standardkonfiguration gestartet: Web-App und Health
  unter `http://127.0.0.1:8787` erreichbar. `data/taskmanager.db` wurde automatisch
  angelegt und enthält keine Testaufgaben. Für einen Neustart die README verwenden.
- Dieser geprüfte Stand bildet den ersten Git-Commit einschließlich Quellcode,
  Dokumentation und Lockfiles. Nächster gemeinsamer Schritt: Anwendung ansehen
  und anschließend das nächste Inkrement planen.

## 1. GTD-Grundfunktionen vervollständigen
- Areas, Projekte und Tasks vollständig bearbeiten, sortieren, archivieren/löschen (sichere Soft-Delete-Strategie).
- Tags und Verknüpfung zu Aufgaben; Focus/Markierungen; Start-, Fälligkeitsdaten; Filter.
- Aktuelle Aufgabenansichten und Datenmodell fachlich gegen Nirvana-HQ-Verhalten prüfen.
- Tastaturbedienung und responsives Layout verbessern.

### Ergänzungen vom 2026-10-17

- Oberfläche vollständig auf Englisch umgestellt, einschließlich Tooltips,
  barrierefreier Beschriftungen und HTML-Sprache. TypeScript/Vite-Build und
  erneuter macOS-Debug-App-Build erfolgreich. Native Sichtprüfung weiterhin offen.
- Als mögliche erste Serverumgebung den vorhandenen Raspberry Pi 5 mit Ubuntu
  Server und Tailscale besprochen; noch keine Installation beauftragt oder
  durchgeführt. Nutzer hat die Angabe präzisiert: Ubuntu 24.04.5 LTS,
  Release 24.04, Codename Noble. Die frühere Annahme 26.04.1 ist ersetzt.
  Betriebssystem und Architektur vor Ort mit `cat /etc/os-release` und `uname -m` bestätigen.
  ARM64 als Build-Ziel vorgesehen; vorhandene Dienste und Speicher prüfen.
  Referenzen: [Ubuntu LTS](https://releases.ubuntu.com/24.04/),
  [Ubuntu auf Raspberry Pi](https://ubuntu.com/hardware/docs/boards/how-to/ubuntu_supported/raspberry-pi/).
- Vorgeschlagener Betrieb: Axum mit gebautem Frontend auf `127.0.0.1:8787`,
  Zugriff über Tailscale Serve mit HTTPS; Tailscale auch auf den zugreifenden
  Geräten. Wegen fehlendem App-Login den Zugriff auf berechtigte eigene Geräte/
  Nutzer begrenzen. Für Dauerbetrieb Docker Compose, persistente DB-Pfade und
  SQLite-Backups vorbereiten; ARM/Linux-Build und Zugriff separat testen.
  Referenzen: [Tailscale unter Linux](https://tailscale.com/docs/install/linux),
  [Tailscale Serve](https://tailscale.com/docs/features/tailscale-serve).
- Docker als gewünschte Betriebsform besprochen: ARM64-Image mit Backend und
  gebautem Frontend, SQLite-Datenordner auf dem Host per Bind-Mount,
  Veröffentlichung nur auf Host-Loopback und
  Tailscale direkt auf Ubuntu. Dockerfile, `.dockerignore`, Compose-Konfiguration
  und [Betriebsanleitung](DEPLOYMENT.md) sind erstellt; danach Build, API und
  Datenpersistenz beim Neuerstellen des Containers sowie Backup/Restore prüfen.
  Nach FHS-Recherche festgelegter Datenpfad: `/var/lib/nextleaf` auf dem Pi,
  im Container `/data`; Konfiguration unter `/etc/nextleaf`. Dies ersetzt den
  vorherigen `/srv`-Vorschlag. Daten bleiben außerhalb des Containers;
  konsistente SQLite-Sicherungen oder Dateisicherung bei gestoppter App vorsehen.
- Paperless-ngx, Immich und FHS-Verzeichnisstandard recherchiert; Ergebnis und
  Sicherungsumfang in `ARCHITECTURE.md` dokumentiert. Zusätzlich zu den Daten
  Compose, lokale Einstellungen und App-Version sichern. Vor Betrieb Backup-Ziel
  und Zeitplan festlegen; Wiederherstellung in einer frischen Instanz testen.

### Docker-Vorbereitung und Name

- Produktname vom Nutzer bestätigt: **Nextleaf**. UI, Titel und Paketnamen
  angepasst. Bestehende DB-Dateinamen, Umgebungsvariablen, lokale Einstellungen
  und Tauri-App-ID aus Kompatibilitätsgründen beibehalten (siehe DECISIONS.md).
- Schreibweise von Nextlief auf **Nextleaf** korrigiert. Backend-Tests,
  Frontend-/macOS-App-Build und Compose-Prüfung danach erneut erfolgreich.
- Open Source beabsichtigt; Lizenzwahl steht aus. [Namensprüfung](NAMING.md)
  ergab gleichnamige Softwareprojekte, eine GitHub-Organisation sowie belegte
  `.com`-/`.org`-Domains. `.app`/`.dev` ohne RDAP-Eintrag; Registrierbarkeit nicht
  zugesichert. Nutzer bleibt nach Recherche bei **Nextleaf**. Keine eigene
  Domain vorgesehen; eine spätere Domain darf anders heißen. Namenssuche
  abgeschlossen, vollständige Markenprüfung weiterhin nicht durchgeführt.
- Nutzer schlug als Alternative Clear Trail vor. Nachprüfung ebenfalls mit
  Software-/IT-Namensüberschneidungen und belegten `.com`-/`.org`-/`.app`-Domains.
  Auf Nutzerwunsch anschließend **Donebrook** und **Readyfern** vorgeprüft.
  Beide Alternativen zugunsten von Nextleaf verworfen; keine weitere
  Umbenennung vorgenommen. Details und Quellen in NAMING.md.
- Nach Umbenennung geprüft: drei Backend-Tests und Rust-Formatprüfung bestanden;
  TypeScript/Vite-Build und Tauri-Debug-Bundle erfolgreich. Neues Paket:
  `src-tauri/target/debug/bundle/macos/Nextleaf.app`. Native Sichtprüfung offen.
- Docker: Mehrstufen-Build, nicht privilegierter App-Benutzer, Healthcheck,
  schreibgeschütztes Root-Dateisystem, Daten-Bind-Mount und Loopback-Port.
  Neue Pi-Pfade: `/etc/nextleaf` und `/var/lib/nextleaf`.
- Compose mit dem offiziellen Compose-Programm v5.6.0 auf macOS validiert;
  Image-Auswahl, Loopback-Port, Datenpfad, fehlende automatische Ordneranlage,
  Benutzerrechte und Fehler bei fehlender Image-Angabe geprüft.
- Kein Docker-Daemon auf diesem Mac vorhanden. Image-Build, ARM64-Betrieb,
  Persistenztest, Pi-Neustart, Tailscale und Backup/Restore bleiben offen.

## 2. Perspectives
- Filter für Areas, Tags, Status, Projektart, Projekte/Aufgaben, Termine, Verfügbarkeit.
- Gespeicherte Filter mit editierbarem Namen; dieselbe Abfragelogik im Backend.
- Ergebnisse in der Sidebar verfügbar machen.

## 3. MCP-Schnittstelle
- Server für derzeit vorhandene Funktionen implementiert und lokal geprüft; siehe aktuellen pi5-Stand oben und MCP.md.
- Pi-Ausrollung und ChatGPT-/Claude-Verbindungen samt Sprachtests abschließen.
- Neue fachliche Funktionen einschließlich späterer Perspectives/Review ebenfalls über MCP anbieten.
- Gerätelokale Anzeige-/Verbindungseinstellungen benötigen gegebenenfalls eigene Client-Anbindung.

## 4. Stabilisierung und Desktop
- Bestätigter Installationsweg: native Pakete als Release-Downloads (Mac DMG, Windows EXE, Linux AppImage/DEB); Details und ungeprüfte Schritte in DEPLOYMENT.md. Hosting, Signierung und Build-Automation noch festlegen; kein automatischer Updater eingerichtet.
- Einheitliche macOS-, Windows- und Linux-Builds und Tests.
- Lokaler Rust-Dienst und SQLite für Offline-Desktop mit Pi-Abgleich erforderlich; eingebettet/Sidecar im Entwurf festlegen.
- Backups, Datenbankmigrationen, automatisierte Tests.
- Vorbereitung für VPS-Zugriff und erst dann Authentisierung/TLS.

## Später (nicht Version 1)
- Native iPhone-App mit Local-First-Offline-Datenspeicher, Sync und Konfliktlösung.
- Umzug zu Hetzner/Tailscale nach Bedarf.
- Einmalige Nirvana-Übernahme ist in die aktuelle Etappe vorgezogen; keine feste Importfunktion.
