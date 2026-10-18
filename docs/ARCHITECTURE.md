# Architektur

Anforderungen stehen in [SPECIFICATION.md](SPECIFICATION.md), begründete
Architekturentscheidungen in [DECISIONS.md](DECISIONS.md) und der aktuelle
Prüf- und Arbeitsstand in [NEXT_STEPS.md](NEXT_STEPS.md). Diese Dokumente sind
die dauerhafte Übergabe zwischen Codex-Sitzungen.

Aktueller Quellstand (Pi-Ausrollung und reale Clients separat prüfen):

```text
 iPhone-Web / Browser ── HTTP ── Pi: Axum / Rust-Fachcode ── SQLite
                                       ↕ /api/sync
 Desktop React ── Tauri IPC ── eingebetteter Rust-Fachcode ── lokale SQLite
                                       ↑
 Agent ── stdio ── Nextleaf --mcp ────────┘
```

## Gemeinsamer Fachcode und MCP

`backend/src/lib.rs` enthält die vorhandenen REST-/GTD-Operationen und
`api_router`, `create_pool`, `serve_stdio`; `main.rs` startet nur den Server.
`src-tauri/src/lib.rs` ruft den Router über Tower innerhalb des Prozesses auf.
Kein lokaler HTTP-Port und kein zweiter Sidecar sind für Desktop erforderlich.
React verwendet auf Desktop Tauri IPC, im Web weiterhin HTTP.

Der zentrale Streamable-HTTP-MCP-Adapter `/mcp` und `Nextleaf --mcp` verwenden
beide das offizielle Rust-SDK und dieselben 13 Fachoperationen. Der zentrale
Adapter benötigt Bearer-Schlüssel und Host-Allowlist, weist Browser-Origin ab
und ist ohne Schlüssel deaktiviert. Der lokale stdio-Prozess öffnet dieselbe
Desktop-Datenbank und startet denselben Hintergrundabgleich. Betriebssystem-
oder gerätelokale Darstellungssteuerung wird dadurch nicht automatisch angeboten.
Umfang und konkrete Agentengrenzen stehen in [MCP.md](MCP.md).

Der Nutzer bevorzugt Agent → lokale Nextleaf-Anbindung → privater Pi-Abgleich.
Eine iPhone-Web-App kann allein keine lokale Tool-Verbindung in ChatGPT oder
Claude herstellen. Agenten-App, Netzwerk und Sprachmodus müssen gesondert geprüft
werden; aktuell sind weder reale Claude-/ChatGPT-Clients noch iPhone-Sprache
konfiguriert. Kein öffentlicher Serverzugang oder Anbieter-Tunnel ist eingerichtet.

## Datenhaltung
- `areas`, `projects`, `tasks`, `tags`, `task_tags`, `project_tags`, `saved_views` (Perspectives als späterer Ausbau).
- UUIDs als TEXT und RFC3339-Zeitstempel als TEXT; Sortierreihenfolge explizit gespeichert.
- Kein direktes Schreiben von Clients in die Server-Datenbank.
- `available` und `blocked_by_sequence` werden als **abgeleitete** Informationen berechnet.
- Für sequenzielle Projekte ist das erste nicht abgeschlossene und nicht gelöschte Task-Element der Reihenfolge maßgeblich. Nur dieses darf (bei Status `next` und ohne aktive Zurückstellung) in die Next-Liste.
- Desktop speichert lokal mit bidirektionalem Abgleich; iPhone-Web bleibt online. Protokoll, Datenpfad und Konfliktbehandlung siehe unten.

## Importierte Daten und Anzeige

Migration `20261017000100_nirvana_import.sql` ergänzt die Task-Statuswerte
`later`, `recurring`, `trash`, Focus, Waiting-Kontakt und Zeit/Energie sowie
Projekt-Abschlussdaten. Bestehende Task-Tag-Verknüpfungen bleiben erhalten.
Alle importierten Projekte sind auf Nutzerwunsch Parallel. Reihenfolge vorläufig
nach MCP-Abruf; Tagesdaten bleiben `YYYY-MM-DD`, ohne erfundene Uhrzeiten.
Lokales `updated_at` ist der Importzeitpunkt; Quelländerungsalter bleibt im Rohobjekt.

Trash und Vorlagen erscheinen nicht in Next. Scheduled zeigt auch Vorlagen,
mit Hinweis auf fehlende Wiederholungsregeln; keine automatische Ausführung.
Aufgabennotizen werden als Klartext in aufgeklappten Aufgabendetails angezeigt. Das Logbook wird nach
Abschlussdatum absteigend sortiert. Nach Abschluss bleibt die Zeile vorübergehend durchgestrichen; erneuter Klick
öffnet die Aufgabe im vorherigen Status, ohne Banner. Allgemeines Wiederöffnen im Logbook setzt weiterhin Next.

## Listen und Dialoge

Gemeinsame React-Oberfläche für Web und Desktop. Die Projektanlage verwendet
einen nativen HTML-Dialog mit Fokusbegrenzung und Escape. Aufgabendetails klappen
dagegen direkt in der Liste auf; jeweils eine Aufgabe ist geöffnet. Der Titel
öffnet per zugänglichem Button (`aria-expanded`) den Editor. Dort ist er als
Eingabefeld bearbeitbar; Save changes/Cancel und Escape führen zur kompakten
Zeile zurück und fokussieren wieder den Titel. Die Detailansicht zeigt Inhalt links und Eigenschaften rechts,
auf schmalen Fenstern untereinander. Der Abschlussknopf ist nur in geschlossenen Zeilen sichtbar; im Editor steht
der Focus-Stern neben dem Titel.
`TaskFields` und `TagEditor` bleiben im Inline-Editor; dort gilt Save changes/Cancel.
`TaskActions` verwendet ein schmales natives Popover und ein darin verschachteltes,
separat positioniertes Untermenü links. Das Untermenü verwendet manuelle Popover-
Sichtbarkeit, damit ein Klick auf eine Hauptkategorie nicht zuerst deren bereits
per Hover geöffnetes Untermenü durch Light-Dismiss schließt. Außenklick schließt
über das Hauptmenü beide. Pointer-Enter öffnet Kategorien; Klick/Enter/Pfeiltasten
unterstützen Tastatur und Touch. `quickMenu.ts` liefert direkte Auswahlwerte mit
minimalen PATCH-Daten und begrenzt die Untermenüposition auf das Fenster.
Auswahl speichert sofort über den vorhandenen Callback, schützt vor gleichzeitigen
Schreibvorgängen und schließt erst nach Erfolg. Fehler erhalten Menü und alten Wert.
Time-Werte sind mit dem Inline-Editor geteilt. Freie Texte werden mit Enter
oder beim Verlassen übernommen. Due/Starts öffnen den gemeinsamen Kalender;
Tageswahl im Schnellmenü speichert direkt.

Listen verwenden getrennte flexible Titel-/Metadatenspalten mit höchstens zwei
sichtbaren Zeilen. Checkbox und Menüknopf sind oben ausgerichtet. Kurze Zeilen sind
etwa 37px hoch; lange Titel/Tags etwa 57px. Kopfbereich und Sidebar sind kompakter.
Der vollständige Titel ist im mehrzeiligen Editor zugänglich; lange Notizen scrollen
innerhalb des begrenzten, vergrößerbaren Textbereichs. Kein Completed-Banner;
Wiederöffnen direkt am Häkchen. Abschluss-/Logbook-Regeln bleiben unverändert.
Der Inline-Editor zeigt fünf primäre Eigenschaften, weitere unter More options.

New Item verwendet ebenfalls ein natives Popover und vorhandene Dialoge.
GET/POST `/api/tags` nutzen die Tags-Tabelle; die neue Verwaltungsrunde ergänzt
Typ/Archivstatus über Migration `20261018010000_tag_management.sql`.
POST `/api/tasks` speichert optional `starred` direkt beim Anlegen, sodass
Focus-Aufgaben keine zweite, möglicherweise fehlschlagende Änderung benötigen.
Health-Fähigkeit `item-creation` schützt neue Clients vor alten Servern, die
Focus ignorieren würden. Tag-Vorschläge werden nur bei unterstütztem Server geladen.
`viewOptions.ts` validiert lokale Anzeigeeinstellungen und definiert New-Item-Ziele;
Filter/Verfügbarkeit bleiben im Backend. Display-Einstellungen liegen je Client
unter `nextleaf_display_options`, ohne Änderung an Aufgaben oder Serverdaten.

PATCH `/api/tasks/{id}` unterstützt Titel, Notizen, Status, Focus, Tags,
Projekt/Area, Waiting for, Start/Fälligkeit sowie Time/Energy. Fehlend bedeutet
unverändert, explizites null löscht optionale Angaben. Feldvalidierung vor der
Änderung; Aufgaben- und Tag-Verknüpfungen werden in einer Transaktion gespeichert.
Projektwechsel hängt die Aufgabe am Ende des Zielprojekts ein. Keine neue Migration.
Der Health-Endpunkt meldet `task-editing`; neue Clients prüfen dies vor erweiterten
Änderungen, damit alte Server keine unbekannten Felder stillschweigend ignorieren.
Titel-/Status-Änderungen bleiben mit dem vorherigen Pi-Stand kompatibel.

Frisch abgeschlossene Aufgaben werden ausschließlich im aktuellen Client und
Ansichtskontext an ihrer bisherigen Position mitgeführt. Der gespeicherte Status
ist sofort Completed; die Next-Verfügbarkeit bleibt vollständig Backend-Aufgabe.
`taskList.ts` führt aktuelle Serverantworten und kurzzeitig beibehaltene Zeilen
ohne Duplikate zusammen. Expliziter Refresh und Navigation löschen diese lokale
Darstellung. Automatisches Nachladen bei Fensterfokus behält sie bei und gleicht
extern wieder geöffnete Aufgaben ab. Keine Schemaänderung oder dauerhafte Kopie.

## Entwicklungsbetrieb

Auswahlfelder bleiben native HTML-Selects mit gemeinsamem CSS für Web und
Desktop: flache Darstellung und eigener Pfeil, native Menü-/Tastaturbedienung.
Erzwungene Kontrastfarben verwenden wieder die Systemdarstellung.

Die beauftragte einmalige Nirvana-Datenübernahme wird anhand eines vollständigen
Quelldatenbestands vorbereitet. Auf Nutzerwunsch dient Nirvanas offizieller
MCP-Server als bevorzugte Lesequelle; bei Umfangslücken den JSON-Export ergänzen.
Die Verbindung gehört zur lokalen Codex-Konfiguration, nicht zum Nextleaf-Server
oder Docker-Image. Quelldaten und DB-Sicherungen bleiben außerhalb
von Git. Zuordnung und Import zunächst in einer separaten SQLite-Datenbank
prüfen; noch nicht unterstützte Inhalte ausdrücklich ausweisen. Das einmalige
Rust-Wartungsprogramm `backend/src/bin/import_nirvana.rs` importiert den lokalen
Snapshot transaktional. `nirvana_imports` ordnet Quell-IDs lokalen UUIDs zu und
bewahrt die Originalobjekte. Gleiche Objekte werden übersprungen, geänderte
Quellen abgelehnt; lokale Bearbeitungen nicht überschrieben. Backup vor Migration
und Import erforderlich. Kein permanenter Nirvana-Zugang im App-Server.
Prüf- und Umsetzungsplan: [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

1. Rust-Backend lauscht standardmäßig nur auf `127.0.0.1:8787`.
2. Vite auf `http://localhost:1420`; `/api` wird in der Entwicklung an Axum weitergeleitet.
3. Tauri lädt in der Entwicklung die Vite-Weboberfläche; im Paket das gebaute Frontend.
4. Desktop startet den eingebetteten lokalen Fachcode automatisch. Die konfigurierte
   Server-URL dient dem Sync; lokale API-Aufrufe benötigen keinen laufenden Pi.
5. Web verwendet standardmäßig dieselbe Origin; der lokale Vite-Proxy bleibt für
   Browserentwicklung verfügbar. Desktop-Tests können den Datenordner über
   `NEXTLEAF_DESKTOP_DATA_DIR` isolieren.

## Sicherheit / Remote-Zugriff
Die Server-REST-API einschließlich `/api/sync` besitzt **keinen eigenen Login** und bleibt auf Loopback bzw. privates Tailnet begrenzt. Nicht öffentlich routen oder ins Internet exponieren. Vor späterem VPS-Betrieb Authentifizierung, Autorisierung, CORS-Allowlist, TLS/gesicherten Tunnel, Backup/Restore und Migrationsstrategie ergänzen.

## Vorbereiteter Docker-Betrieb auf dem Raspberry Pi 5

- Docker Engine und Compose auf Ubuntu Server 24.04.5 LTS (Noble, ARM64);
  Betriebssystem, Architektur und hello-world anhand der vom Nutzer gelieferten
  Pi-Terminalausgaben bestätigt. Nextleaf-ARM64-Betrieb und Datenpersistenz
  nach Container-Neuerstellung sowie HTTPS-Erreichbarkeit über Tailscale geprüft.
- Ein App-Container enthält das Rust-Backend und das gebaute React-Frontend;
  Axum liefert API und Weboberfläche aus. Tauri verwendet denselben Fachcode lokal und synchronisiert mit diesem Server.
- SQLite liegt in `/var/lib/nextleaf` auf dem Pi, das als Bind-Mount nach
  `/data` eingebunden wird. Compose und lokale Einstellungen liegen unter
  `/etc/nextleaf` (siehe Verzeichnis- und Sicherungskonzept unten).
  Das gesamte Datenverzeichnis einschließlich SQLite-Begleitdateien liegt damit
  außerhalb der beschreibbaren Container-Schicht. Das Image enthält nur die App.
  Container-Updates oder Neuerstellung behalten dieses Verzeichnis bei.
- Im Container bindet Axum an `0.0.0.0:8787`; Compose veröffentlicht den Port
  ausschließlich als `127.0.0.1:8787:8787` auf dem Host.
- Tailscale läuft auf dem Host; Tailscale Serve leitet HTTPS aus dem privaten
  Tailnet an diesen lokalen Port weiter. Zugriff auf berechtigte Nutzer begrenzen.
- Compose übernimmt Start und Neustarts. Backups als konsistente SQLite-Sicherung
  (Online Backup API) erstellen oder die App vor einer Dateisicherung sauber
  stoppen. Eine laufende Datenbank nicht einfach als einzelne Datei kopieren.
  Sicherungen zusätzlich auf einem anderen Datenträger/System aufbewahren und
  Wiederherstellung testen. Host-Verzeichnis und Schreibrechte vor Start einrichten.

Dockerfile und Compose sind vorbereitet. Mehrstufiger Build für Frontend und
Backend; Laufzeit als UID/GID `10001:10001`, Root-Dateisystem schreibgeschützt.
Ein expliziter lokal gebauter Image-Tag wird über `/etc/nextleaf/.env` gewählt.
Compose-Konfiguration, Docker-/ARM64-Build, sieben Rust-Tests, Healthcheck und
Datenpersistenz nach Container-Neuerstellung auf dem Pi erfolgreich geprüft.
Siehe [DEPLOYMENT.md](DEPLOYMENT.md) für Einrichtung, Prüfung und Wiederherstellung.
Referenz: [Docker Engine auf Ubuntu](https://docs.docker.com/engine/install/ubuntu/).

## Linux-Verzeichnisse und Wiederherstellung

Recherche vom 2026-10-17: Es gibt keinen gemeinsamen vorgeschriebenen
Docker-App-Ordner. Der Filesystem Hierarchy Standard ordnet `/opt` zusätzlicher
Software zu, `/etc` der Host-Konfiguration und `/var/lib` dauerhaftem
Anwendungszustand. `/srv` betrifft bereitgestellte Dienstdaten; für interne
Datenbankdateien passt `/var/lib` genauer. Bei einer klassischen Installation
unter `/opt` gehören veränderliche Daten nach `/var/opt` und Konfiguration nach
`/etc/opt`. Unsere App wird als Container-Image bereitgestellt.
Quelle: [FHS 3.0](https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.html).

Vergleich mit bestehenden Apps:

- Paperless-ngx dokumentiert Docker-Volumes für Dokumente, Hilfsdaten und
  Datenbank sowie einen Exporter für Sicherung/Wiederherstellung. Ein fester
  `/opt`-Pfad wird dafür nicht verlangt.
  [Paperless-Administration](https://docs.paperless-ngx.com/administration/)
- Immich lässt den Compose-Ordner frei wählen und legt Host-Pfade über
  `UPLOAD_LOCATION` und `DB_DATA_LOCATION` fest. Backups benötigen sowohl
  Datenbank als auch Mediendateien.
  [Immich Compose](https://docs.immich.app/install/docker-compose/),
  [Immich Backup/Restore](https://docs.immich.app/administration/backup-and-restore/)

Daraus abgeleitete Struktur für Nextleaf auf dem Pi:

| Host-Pfad | Inhalt | Sicherung |
| --- | --- | --- |
| `/etc/nextleaf/compose.yaml` | Containerdefinition mit festgelegter Image-Version | Ja |
| `/etc/nextleaf/.env` | Lokale Einstellungen, gegebenenfalls Secrets | Ja, geschützt |
| `/var/lib/nextleaf/` | SQLite-Datei und Begleitdateien; nach `/data` gemountet | Ja, konsistent |
| Container-Image und Container-Schicht | App, Laufzeit und gebautes Frontend | Keine laufende Datensicherung |

Alle dauerhaften App-Daten müssen im eingebundenen Verzeichnis liegen. Aktuell
liegen Aufgaben, Projekte, Areas und weitere Datensätze in SQLite. Es gibt
keine separate Dateiablage für Anhänge. Reproduzierbare Caches und Logs sind
für die Wiederherstellung des App-Zustands nicht erforderlich.

Zur ersten Sicherung die App kontrolliert stoppen und Datenverzeichnis sowie
Konfiguration kopieren; danach wieder starten. Alternativ eine konsistente
SQLite-Online-Sicherung erstellen. Die laufende Datenbank nicht mit einer
einfachen Dateikopie sichern. Backups samt Versionsangabe auf ein separates
Ziel übertragen und lokale Secrets schützen.
[SQLite Backup API](https://sqlite.org/backup.html)

Wiederherstellung: Docker/Compose einrichten, Konfiguration und Daten bei
gestoppter App zurückspielen, Eigentümer/Schreibrechte herstellen und dieselbe
App-Version mit demselben Bind-Mount starten. Danach Datenintegrität und Inhalte
prüfen; Versionsupdates erst anschließend. Container-Neuerstellung bei intaktem
Host-Datenordner benötigt kein Zurückspielen der Daten. Das passende Image muss
abrufbar bleiben oder aus einem gesicherten Quellstand gebaut werden können;
Quellcode und Builddateien deshalb unabhängig im Git-Repository erhalten.

Diese Festlegung ersetzt den früheren Vorschlag `/srv/taskmanager/data`.
Nach Wahl des Produktnamens Nextleaf wurden auch die bisher geplanten Pfade
`/etc/taskmanager` und `/var/lib/taskmanager` durch die oben genannten ersetzt.
Backup-Ziel, Zeitplan und Wiederherstellungstest stehen noch aus.

Projektgruppen entstehen in `groupTasks` ohne Veränderung der Serverdaten: Actions
zuerst, dann vorhandene Projekte in Sidebar-Reihenfolge, nur nichtleere Gruppen.
Innerhalb jeder Gruppe bleibt die API-Reihenfolge erhalten. Im Einzelprojekt wird
die zusätzliche Gruppenüberschrift unterdrückt. Fehlende Projektmetadaten dürfen
keine Aufgaben ausblenden. Projektlabels in Aufgaben entfallen; der bisherige lokale
Anzeige-Schlüssel `project` steuert jetzt nur Area-Hinweise (UI-Label Area).
Die permanente Erfassungszeile entfällt; New Item bleibt der zentrale Einstieg.
Area-Auswahl oben erhält den GTD-View und begrenzt Projekte/Aufgaben. Navigation
zwischen GTD-Ansichten setzt sie nicht zurück. Settings verwendet dieselbe navitem/
navsymbol-Struktur wie die übrige untere Navigation.

Tags verwenden die globale Tagliste und filtern die vom Backend bereits nach GTD,
Area und Projekt berechnete Liste zusätzlich lokal; keine eigene Verfügbarkeitslogik.
Auf pi2 ohne Tag-Endpunkt werden vorhandene Namen aus All tasks abgeleitet, bis pi3
auch unbenutzte Tags liefert. Tagfilter behält Reihenfolge und wird vor Projektgruppen
angewendet. Der obere Sidebar-Flexbereich scrollt mit min-height:0/overflow-y:auto;
der separate Footer schrumpft nicht und bleibt im sticky Sidebar-Viewport sichtbar.

## Projekteinstellungen und Datumsauswahl

Migration `20261017000200_project_details.sql` ergänzt Projekt-Fälligkeit,
Focus-Markierung und project_tags, ohne vorhandene Inhalte zu ändern. PATCH
`/api/projects/{id}` validiert Namen, Status, Modus, Area, Datum und Tags; Updates
und Verknüpfungen laufen atomar. Keine kaskadierende Änderung der Aufgaben bei
Projektabschluss. Bestehende Backend-Verfügbarkeitsregeln lesen den neuen Modus
beim nächsten Abruf. Health meldet project-editing; Clients blockieren Speichern
auf älteren Servern. Fehlende neue Felder alter Server erhalten im Client Defaults.

Projects ist eine Frontend-Ansicht, kein neuer Task-Status. `projectRows` zählt
bereits geladene offene Aufgaben ohne GTD-Verfügbarkeit neu zu berechnen.
`ProjectDetails` nutzt TagEditor/DateField und die vorhandenen Inline-Editor-Stile.
Projektansichten kombinieren offene und abgeschlossene API-Ergebnisse in ihrer
Sortierreihenfolge; Trash bleibt separat. Projekt-Tags und Focus gelten für das
Projekt, werden nicht auf seine Aufgaben übertragen.

DateField verwendet einen nativen modalen HTML-Dialog (Portal im body), am Feld
positioniert, mit eigenem englischen Monatsraster und abgedunkeltem Backdrop.
Keine neue Bibliothek; Kalenderberechnung in lokalen Tagen, keine UTC-Konvertierung.
Schnellmenüs schließen vor dem Kalender, damit der Dialog nicht zusammen mit
seinem Popover verschwindet. Tastaturfokus kehrt zum Auslöser zurück, sofern sichtbar.
TagEditor zeigt passende bekannte Tags als Combobox-Vorschläge. Unbestätigter
Eingabetext bleibt rein lokal, nur Enter/Vorschlagsklick ändert die Entwurfs-Tags.

Nach dieser Migration startet der alte SQLx-Server ggf. nicht mehr. Der pi4-Updater
sichert Daten und Konfiguration vor dem Wechsel; nach möglichem Migrationsstart
kein automatisches Downgrade oder Überschreiben der DB. Fehler benötigen Diagnose
mit vorhandener Sicherung; vor Migrationsstart kann die alte Version wieder starten.

## Implementierter Desktop-Offline-Abgleich (2026-10-18)

Desktop-Daten: `dirs::data_dir()/dev.taskmanager.desktop/nextleaf.db`; auf macOS
also `~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
SQLite verwendet WAL und Foreign Keys. Alte Entwicklungsdatenbanken werden nicht
kopiert. UI und stdio-MCP verwenden denselben Bestand, auch in getrennten Prozessen.

Migration `20261018000000_sync.sql` ergänzt ausschließlich Metadaten:
`sync_identity`, `sync_client`, `sync_receipts`. `backend/src/sync.rs` überträgt
konsistente Snapshots von Areas, Tags, Projekten, Aufgaben und Tagbeziehungen.
Import-Audit und noch ungenutzte Saved Views bleiben lokal; keine SQLite-Dateien
werden übertragen. Sync v2 verwendet stabile Tag-IDs als Identität, damit
Umbenennungen und Typänderungen Beziehungen erhalten. Tagmetadaten liegen als
ID-indizierte Records, Aufgaben-/Projektbeziehungen als ID-Listen im Snapshot;
die REST-API verwendet für Beziehungen weiterhin Namen. Gleichnamige parallele
Neuanlagen werden über explizite kanonische ID-Zuordnungen zusammengeführt,
auch bei lokalen Änderungen während einer laufenden Anfrage.

Migration `20261018010000_tag_management.sql` ergänzt `kind` (tag/person) und
Archivstatus für Tags sowie Archivstatus für Areas. Archivierung erhält bestehende
Zuordnungen; neue Zuordnungen zu archivierten Einträgen werden atomar abgewiesen.
GET liefert auch archivierte Einträge, aktive Auswahlfilter sind Client-Aufgabe.
PATCH für Tags/Areas unterstützt `expected`; Health meldet `tag-management`.
Bestehende Tags bleiben normale Tags; Kontakte werden nicht anhand von Namen
erraten. `waiting_for` bleibt als vorhandener Freitext erhalten.

`migrate_client_state` konvertiert gespeicherte v1-Baselines und ausstehende
Anfragen beim Öffnen der DB. Protokoll v1 wird anschließend abgewiesen. Deshalb
Pi zuerst aktualisieren und den alten Mac-Prozess beenden, bevor das neue
Mac-Bundle auf die produktive lokale Datenbank zugreift. pi7 und das neue Mac-Bundle wurden am 2026-10-18 mit dem vorhandenen
Testbestand abgeglichen; Prüfdetails stehen in NEXT_STEPS.md.

- Baseline, laufende Request-ID und ausstehende lokale Unterschiede sind dauerhaft.
  Ein Dreiwege-Vergleich führt Änderungen verschiedener Felder zusammen.
- Konflikte behalten die lokalen Daten. Explizite lokale/Server-Wahl betrifft nur
  konfliktäre Felder; zwischenzeitlich veränderte Konfliktwerte verlangen erneute
  Prüfung. Keine globale Last-write-wins-Regel.
- `BEGIN IMMEDIATE` schützt Lesen/Vergleichen/Schreiben über UI-, MCP- und
  Sync-Prozesse hinweg. Eine nach Beginn der Netzübertragung erfolgte lokale
  Änderung wird beim Abschluss erneut gegen den gesendeten Stand abgeglichen.
- Server speichert die letzte Quittung samt Sequenz pro Client. Wiederholungen
  derselben Anfrage liefern dieselbe Antwort; alte oder anders wiederverwendete
  Request-IDs werden abgewiesen.
- Automatischer Startabgleich verwendet nur `remote_url` aus der aktuellen
  Datenbank; eine alte WebView-/localStorage-Adresse darf keine neue Datenbank
  verbinden. Die erste Verbindung erfolgt explizit über Settings.
- URL und Serveridentität werden dauerhaft gebunden. Vor erster bestätigter
  Identität ist eine falsche URL korrigierbar; danach schützt die Bindung vor
  versehentlicher Verbindung mit einem anderen Bestand.
- HTTPS erforderlich, außer Loopback für Entwicklung; keine HTTP-Weiterleitungen.
  `/api/sync` weist Browser-Origin-Anfragen ab. Die bestehende private Netzgrenze
  schützt REST und Sync; MCP-Bearer schützt nur `/mcp`.
- Snapshots sind auf 16 MiB und 50.000 Datensätze begrenzt. Physische Löschungen
  außerhalb der App stoppen den Abgleich; fachlicher Trash/Restore funktioniert.
  Für wesentlich größere Bestände später inkrementelles Protokoll entwerfen.

Tauri und stdio-MCP versuchen bei gespeicherter URL alle 15 Sekunden den Abgleich,
solange der jeweilige Prozess läuft. Die UI erhält ein Änderungsereignis; offene
Entwürfe bleiben erhalten. Settings zeigt Status, Konflikte, Adresse, Datenordner
und MCP-Befehl. Die neue Serverfunktion muss vor produktiver Erstübernahme auf
dem Pi ausgerollt sein; der bisherige pi5-Stand besitzt sie noch nicht.

Task-/Project-PATCH unterstützt optional `expected` mit ursprünglichen Werten
geänderter Felder. Vergleich und Änderung liegen in derselben Writer-Transaktion;
Abweichungen liefern 409. Die UI verlangt dafür `conditional-updates`, damit ein
älterer Server den Schutz nicht still weglässt. Sequential-/Verfügbarkeitsregeln
bleiben vollständig im gemeinsamen Fachcode.

Isolierte Tests decken Initialabgleich, Offline-Neustart, parallele Geräte,
Tag-Duplikate, Beziehungen, Abschluss/Trash/Restore, Retry, Identitätswechsel,
Konfliktentscheidung, veränderte Konflikte und veraltete Editorentwürfe ab.
Ein echter lokaler HTTP-Test prüft zusätzlich Erreichbarkeit und Offline-Fehler.
Rust-/Tauri-Prüfung und macOS-Debug-Bundle erfolgreich; native Bedienung, produktiver
Abgleich, reale Agenten-Clients und Windows/Linux sind gesonderte Prüfungen.

## Geplanter Release-Vertrieb für Server (2026-10-17)

Künftige Releases liefern zusammenpassende `docker-compose.yml`, `example.env`
(zur lokalen Speicherung als `.env`) und vorgebaute versionierte Images für
`linux/arm64` und `linux/amd64`. Repository-/Registry-Adresse und Release-Pipeline
sind noch festzulegen; keine erfundenen Download-URLs. Installation benötigt nur
Konfigurationsdateien, Datenverzeichnis mit passenden Rechten und Docker Compose.

Konfigurationsordner darf frei gewählt werden; Vorgabe `/etc/nextleaf` bleibt.
Daten-Bind-Mount erhält künftig seinen Pfad aus `.env`, Standard `/var/lib/nextleaf`.
Loopback-Port und Tailscale-Zugriff bleiben erhalten. Aktuell verlangt Compose
noch ein lokal gebautes Image (`pull_policy: never`); das muss für den
Release-Vertrieb angepasst und mit beiden Architekturen geprüft werden.
Bestehende Pi-Updater und deren Backups bleiben bis zur geprüften Umstellung gültig.

## Desktop-Pakete und mobile Oberfläche (2026-10-17)

Native Tauri-Releasepakete sind als Installationsweg bestätigt; Zielplattformen
getrennt bauen/testen, Signierung und macOS-Notarisierung für Veröffentlichung
vorbereiten. Versionierte Pakete zunächst manuell aktualisieren, Benutzerdaten
außerhalb des Programmpakets erhalten. Hosting/Build-Automation noch festzulegen.

Die iPhone-Bedienung verwendet eine gemeinsame React-Web-App mit adaptiver
Desktop-/Mobil-Darstellung: Drawer-Navigation, mobiler Plus-Knopf, Touch-Menüs,
Filter sowie einspaltige Details/Sheets sind im Quellstand umgesetzt. Gemeinsame Typen, API-Aufrufe, Datenoperationen,
Validierung und Feldkomponenten; Navigation, Detailpräsentation und Menüs dürfen
mobile Varianten erhalten. Kein zweites unabhängiges Web-Projekt oder Backend
nötig. Bildschirmbreite und Eingabemöglichkeiten statt allein User-Agent verwenden.
Touchziele, Safe Areas, Tastatur und Safari-Verhalten gesondert prüfen.
Echte iPhone-/Safari-/Bildschirmtastatur-Prüfung steht separat aus; kein Offline-Speicher für die mobile Web-App.

## Spätere Perspectives und Review

Perspectives sollen den heutigen Area-Selektor ersetzen; der Editor wählt aktive
Areas und Tags und blendet übrige Inhalte aus. Review soll einen geführten
Durchgang durch Inbox und einzelne Projekte ermöglichen. Recherche und Abgrenzung
zur historischen OmniFocus-Projektprüfung: [REVIEW_RESEARCH.md](REVIEW_RESEARCH.md).
Beide Funktionen bleiben ausdrücklich außerhalb der aktuellen Implementierung.

## Frontend-Integration der Tagverwaltung (2026-10-18)

api.ts normalisiert ältere Tag-/Area-Antworten und verlangt tag-management vor
explizit typisierter Anlage und Verwaltungs-PATCH. expected bleibt als Objekt im
Body erhalten. Einfache Taganlage mit nur einem Namen bleibt item-creation-kompatibel.
Aktive People werden durch App an TaskDetails/TaskFields und TaskActions gereicht;
der Editor verwendet native Vorschläge mit unverändertem Freitext, das Schnellmenü
direkte Kontaktwahl. Pointer-/Fokusbehandlung verhindert, dass Blur vor dem
Kontaktklick einen anderen Wert speichert. Archivierte Areas bleiben nur für die
aktuelle Zuordnung auswählbar; bei Neuanlage wird kein archivierter Area-Filter
übernommen. Manage Tags lädt nach Cancel eines fehlgeschlagenen Entwurfs aktuelle
Einträge nach. Keine neue Migration oder neue Fachregel in dieser Integration.
Prüfstand und unveränderte produktive Ausrollung stehen in NEXT_STEPS.md.

## Release-Dateien für die Erstinstallation

`deploy/release/docker-compose.yml` und `example.env` bilden den geplanten
Downloadweg ab: versioniertes veröffentlichtes Image als Pflichtwert,
`pull_policy: missing`, Datenordner aus `NEXTLEAF_DATA_DIR` (Standard
`/var/lib/nextleaf`), Loopback-Port und Benutzer 10001:10001 wie bisher.
Root-`compose.yaml` bleibt beim lokalen Build-/Updateweg mit `pull_policy: never`.
Noch keine Registry/Release-Adresse; Konfiguration geprüft, Pull und Installation
noch offen. Ablauf und Grenzen siehe INSTALLATION.md.
