# Nextleaf auf dem Raspberry Pi

## Neuinstallation für Anwender

Die Anleitung für den geplanten vollständigen Neustart steht in
[INSTALLATION.md](INSTALLATION.md). Release-Vorlagen liegen unter
`deploy/release/`; sie sind mit Compose validiert, aber noch nicht veröffentlicht.
Die folgenden Abschnitte dokumentieren den bestehenden lokalen Build-/Updateweg.

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

## Künftiger Installationsweg: zwei Dateien und fertiges Image

Am 2026-10-17 als Ziel beschlossen, **noch nicht als Download verfügbar**:

1. Konfigurationsordner anlegen und hineinwechseln; Standard `/etc/nextleaf`,
   anderer Ordner möglich.
2. Zur selben Release-Version gehörende `docker-compose.yml` und `example.env`
   per `wget` herunterladen; ENV-Vorlage als `.env` ablegen.
3. `.env` anpassen (Image-Version, Datenpfad, gegebenenfalls MCP-Schlüssel),
   Datei vor fremdem Lesen schützen. Datenordner mit passenden Rechten anlegen;
   Standard bleibt `/var/lib/nextleaf` für Container-Benutzer 10001:10001.
4. Mit `docker compose pull` das fertige passende Image herunterladen und mit
   `docker compose up -d` die Anwendung im Hintergrund starten.
5. Health und Zugriff über die bestehende private Tailscale-Verbindung prüfen.

Voraussetzungen sind Docker Engine/Compose und eingerichteter privater Zugriff.
Images sollen für ARM64/Pi und AMD64 bereitstehen; kein Kompilieren auf dem Pi.
Release- und Registry-Adressen sind noch nicht eingerichtet, deshalb hier keine
scheinbar ausführbaren wget-Beispiele mit erfundenen URLs. Der jetzige lokale
Image-Tag und `pull_policy: never` unterstützen diesen Ablauf noch nicht.

Bei Updates `.env` mit eigenen Einstellungen/Secrets erhalten, Änderungen an
Vorlagen abgleichen, passende Version auswählen und vor Migration Daten sowie
Konfiguration konsistent sichern. Compose/Image aktualisieren, Container erneuern
und Ergebnis prüfen; Daten bleiben im externen Verzeichnis. Kein blindes
Überschreiben der `.env` durch erneuten Download. Die konkreten Installations- und
Update-Befehle werden nach Prüfung mit einer Erklärung je Schritt veröffentlicht.

Referenz: [Immich – Docker Compose](https://docs.immich.app/install/docker-compose/).
Die folgenden Abschnitte dokumentieren den bisherigen Betrieb und bleiben bis
zur geprüften Umstellung maßgeblich.

Ziel: Raspberry Pi 5, Ubuntu Server 24.04 LTS (Noble) mit 64 Bit, Docker Engine mit Compose
und Tailscale auf dem Host. Der Container enthält Backend und Weboberfläche.
Tauri wird auf dem Server nicht benötigt.

Am 2026-10-17 anhand der vom Nutzer übermittelten Pi-Terminalausgaben bestätigt:
Ubuntu 24.04.5 LTS, Release 24.04, Codename Noble und `arm64`.
Docker CE 29.9.1 und Compose 5.6.0 installiert; `sudo docker run --rm hello-world`
hat das ARM64-Image heruntergeladen und erfolgreich ausgeführt. Kein direkter
SSH-Zugriff durch Codex; die Befehle wurden vom Nutzer ausgeführt.

Tailscale anschließend auf dem Host installiert und angemeldet. Die vom Nutzer
gelieferte Ausgabe von `tailscale status` zeigt Pi, Mac und iPhone im selben
Tailnet; `tailscale ip -4` liefert eine Adresse. Geräteadressen und Kontoangaben
bleiben außerhalb der versionierten Dokumentation. SSH über Tailscale vom Nutzer
und anschließend durch Codex erfolgreich geprüft. Nextleaf über Tailscale Serve vom Mac aus erfolgreich per HTTPS geprüft.

### Erste Installation erfolgreich (2026-10-17)

Auftrag: Nextleaf auf dem Pi installieren und den aktuellen Mac-Datenbestand
übernehmen. Konsistente SQLite-Online-Sicherung mit 118 Aufgaben, 11 Projekten,
3 Areas, 12 Tags und 128 Importzuordnungen erstellt. Integrität/Fremdschlüssel
geprüft; kein erneuter Nirvana-Import. Der bisherige Mac-Stand bleibt erhalten,
ist aber nach dem Umzug keine synchronisierte zweite Datenbank.

Aktueller Quellstand einschließlich noch uncommitteter Änderungen, Sicherung
und einmaliges Installationsskript liegen im privaten Benutzerverzeichnis
`~/nextleaf-setup-20261017/` auf dem Pi. Übertragung per SSH, SHA-256-Prüfung
aller Dateien und Datenvergleich auf dem Pi erfolgreich. Lokale private Kopie:
`data/deploy-pi-20261017/`; weder Daten noch Installationsarchiv einchecken.
Pi: etwa 437 GiB frei, 7,8 GiB RAM; keine bestehende Nextleaf-Installation.

Vom Nutzer erfolgreich ausgeführt: `sudo bash ~/nextleaf-setup-20261017/install.sh`.
Codex kann per SSH zugreifen, aber sudo erfordert die Passworteingabe des
Nutzers in dessen Terminal. Das Skript baut das Image samt Rust-Tests, kopiert
die Sicherung nach `/var/lib/nextleaf/taskmanager.db`, startet Compose und
vergleicht alle Anwendungsdaten vor/nach Container-Neuerstellung. Bei bereits
vorhandener DB oder Konfiguration bricht es ab, statt etwas zu überschreiben.
Protokoll: `~/nextleaf-setup-20261017/install.log`. Shell-Syntax und Datenprüfer
(einschließlich absichtlich veränderter Testkopie) geprüft. ARM64-Docker-Build,
sieben Rust-Tests, Frontend-Build, Healthcheck und Web-Auslieferung bestanden.
Alle Zeilen der sieben Anwendungstabellen einschließlich 118 Aufgaben und
11 Projekten vor und nach Container-Neuerstellung identisch zur Sicherung;
SQLite-Integrität und Fremdschlüssel gültig. Codex hat Installationsprotokoll
und aktuellen Healthcheck über SSH geprüft. Image: `nextleaf:0.1.0-pi1`,
Container `nextleaf-nextleaf-1`, Port ausschließlich `127.0.0.1:8787`.
Tailscale Serve anschließend vom Nutzer mit HTTPS aktiviert. Codex hat die
Konfiguration geprüft: HTTPS-Port 443 leitet an `http://127.0.0.1:8787` weiter,
kein Funnel konfiguriert. Vom Mac aus mit regulärer TLS-Zertifikatsprüfung:
Healthcheck, HTML, CSS/JavaScript, Projekte und Aufgaben jeweils HTTP 200.
Browser-Sichtprüfung und iPhone-Aufruf stehen noch aus. Die private Adresse
mit `tailscale serve status` auf dem Pi ablesen.

### Vorbereitetes pi4-Update: Projekteinstellungen und Kalender

Neuer Quellstand: Projektübersicht/Inline-Einstellungen, Fälligkeit/Focus/Tags für
Projekte, kompakte Task-Eigenschaften, Tag-Eingabe per Enter/Vorschlag, Area-Pfeil,
gelber Focus-Stern und Kalender mit abgedunkeltem Hintergrund. Additive Migration
20261017000200; API-Fähigkeit project-editing verhindert Speichern auf pi3.

Paket `data/update-projects-20261017/`, vorgesehenes Pi-Verzeichnis
`~/nextleaf-update-projects-20261017/`, Image `nextleaf:0.1.0-pi4`.
Update baut zuerst, sichert vor dem Wechsel alle Daten/Konfiguration und Quellen
unter `/var/backups/nextleaf/<UTC>-pi4/` und prüft anschließend Health/SQLite.
Nach möglichem Migrationsstart wird bei Fehlern die neue App gestoppt und die DB
beibehalten; kein automatisches Downgrade oder Zurückspielen eines alten Datenstands.
Die Sicherung bleibt für eine gezielte Wiederherstellung erhalten.
Vier simulierte Updater-Szenarien bestanden: Erfolg, Build-/Start-Fehler und
fehlende Fähigkeit. In allen Fällen Testdaten erhalten; tatsächlicher Pi-Lauf offen.
Übertragungsstatus und native Sichtprüfung: NEXT_STEPS.md.

Wenn das Paket übertragen ist, im Pi-Terminal:

```bash
sudo bash ~/nextleaf-update-projects-20261017/update.sh
```

`sudo` gewährt Administratorrechte; `bash` führt Sicherung, Build, Wechsel und
Prüfung aus. Erfolg endet mit NEXTLEAF_UPDATE_OK. Nicht das Erstinstallationsskript
oder ein früheres Datenpaket erneut verwenden.

### Gesamtupdate pi3: kompakte Bedienrunde

Quellstand enthält kompakten Inline-Editor, direkte Hover-Schnellauswahl mit
linken Untermenüs ohne Save, dichtere zweizeilige Projektgruppen ohne Completed-Banner,
New Item statt Capture-Zeile, All Areas-Filter statt Sidebar-Liste, Tags-Filter
und feste bündige untere Navigation,
Tag-Anlage/-Vorschläge und Anzeigeoptionen. Backend-Fähigkeit `item-creation`
kennzeichnet die neue Tag-/Focus-Anlage. Keine Schemaänderung erforderlich.
Shell-Syntax und vier isolierte Update-Selbsttests bestanden: Erfolg, Build-Fehler,
Start-Fehler und fehlende Server-Fähigkeit; Rückfall und Datenerhalt geprüft.
Docker ist in diesen Selbsttests simuliert; der tatsächliche Lauf wurde danach
vom Nutzer mit NEXTLEAF_UPDATE_OK bestätigt.

Paket lokal unter `data/update-compact-20261017/`, auf dem Pi unter
`~/nextleaf-update-compact-20261017/`. Übertragung abgeschlossen; alle
Quell-/Skriptdateien per SHA-256 und Shell-Syntax auf Pi erfolgreich geprüft.
Ausführung am 2026-10-17 durch Nutzer bestätigt. Anschließend unabhängig geprüft:
Docker/Tailscale aktiv, Health lokal und per HTTPS ok mit task-editing und
item-creation. Ausgeliefertes JavaScript/CSS stimmt per SHA-256 mit dem aktuellen
Mac-Frontend überein. Tags/Projekte/Areas/Next/Logbook erfolgreich gelesen;
Mac-App lädt nach Refresh alle zwölf Tags vom Pi. Privilegiertes Update-Protokoll
und Docker-Details ohne sudo-Passwort nicht direkt geprüft; kein produktiver
Schreibtest oder vollständiger Restore-Test in dieser Nachprüfung.
Verwendet denselben Sicherungs-/Rückfallablauf wie pi2; aktuelle Pi-Daten bleiben
maßgeblich. Neue Version `nextleaf:0.1.0-pi3`, Backup unter
`/var/backups/nextleaf/<UTC-Zeit>-pi3/`. Keine alte Mac-DB im Paket.
Bereits vom Nutzer im Pi-Terminal ausgeführt:

```bash
sudo bash ~/nextleaf-update-compact-20261017/update.sh
```

`sudo` stellt Administratorrechte bereit; `bash` startet das vorbereitete Skript.
Es baut die gesamte Version, sichert Daten/Konfiguration, wechselt den Container
und prüft die neue Server-Fähigkeit. Erfolg: `NEXTLEAF_UPDATE_OK`.

Pi-Neustart nach pi2 inzwischen geprüft: Nutzer führte Neustart aus; SSH-Laufzeit,
aktive Docker-/Tailscale-Dienste und Nextleaf-Health lokal sowie über HTTPS
bestätigt. Vollständige Backup-Wiederherstellung weiterhin nicht getestet.

### Update auf pi2 ausgeführt und Anwendung geprüft (2026-10-17)

Die Mac-App enthält jetzt den Inline-Editor und das Aufgaben-Schnellmenü.
Erweiterte Änderungen benötigen `task-editing` im Health-Endpunkt des Servers;
pi1 meldet diese Fähigkeit noch nicht. Der aktualisierte Pi meldet sie inzwischen
korrekt; der Schutz bleibt für Clients an älteren Servern bestehen.

Einmaliges Update-Paket lokal unter `data/update-pi-20261017/`, ohne Datenbank,
Secrets oder erneuten Nirvana-Import. `update.sh` baut zuerst das Image
`nextleaf:0.1.0-pi2`, während die alte App weiterläuft. Danach stoppt es die App
kurz, sichert das vollständige Datenverzeichnis und die Konfiguration unter
`/var/backups/nextleaf/<UTC-Zeit>-pi2/` und startet die neue Version. Das Backup
enthält auch Quellstand und Prüfsummen. Bestehende Pi-Daten bleiben maßgeblich.
Es gibt in dieser Änderung keine Schemaänderung. Bei fehlgeschlagenem Start/
Healthcheck setzt das Skript die vorherige Image-Auswahl zurück und startet
sie wieder; es überschreibt dabei keine Datenbank mit einem älteren Snapshot.

Shell-Syntax und isolierter Selbsttest bestanden: erfolgreicher Wechsel,
Build-Fehler, Start-Fehler und fehlende Server-Fähigkeit; Daten unverändert,
Sicherungsarchiv korrekt, vorherige Konfiguration im Fehlerfall wiederhergestellt.
Docker-Aufrufe sind in diesem Selbsttest simuliert. Der Nutzer hat den tatsächlichen
ARM64-Build und Versionswechsel anschließend mit `NEXTLEAF_UPDATE_OK` bestätigt.
Kein vollständiger Restore-Test.

Übertragungsstand: Paket nach `~/nextleaf-update-20261017/` übertragen und
auf dem Pi entpackt. SHA-256 aller 58 Dateien und Shell-Syntax dort geprüft.
Vom Nutzer erfolgreich ausgeführt; zur Dokumentation der verwendete Befehl:

```bash
sudo bash ~/nextleaf-update-20261017/update.sh
```

`sudo` führt das Skript mit Administratorrechten aus, die Docker und die
Systemverzeichnisse benötigen. `bash` startet das vorbereitete Skript; `~`
steht für das Benutzerverzeichnis. Es baut, sichert, wechselt die Version und
prüft den Dienst. Während des kurzen Wechsels die App nicht bearbeiten.
Erfolgsmeldung: `NEXTLEAF_UPDATE_OK`. Protokoll: `update.log` neben dem Skript.
Danach Web-App neu laden; die bereits aktualisierte Mac-App kann den Server
unverändert weiterverwenden. Tailscale-Konfiguration bleibt erhalten.

Nachprüfung durch Codex: Healthcheck direkt auf Pi und vom Mac über HTTPS
meldet `status: ok` und `task-editing`. HTML und referenzierte JS-/CSS-Dateien
abrufbar; Datei-Hashes entsprechen dem aktuellen lokalen Frontend-Build.
Native Mac-App: eigene Testaufgabe angelegt, mehrzeilige Notizen/Time im
Inline-Editor und Energy über das Schnellmenü gespeichert. Nach Refresh erneut
angezeigt und über die Pi-API unabhängig bestätigt. Testaufgabe danach in den
wiederherstellbaren Trash verschoben; bestehende Benutzeraufgaben unverändert.
Update-Protokoll und Backup-Inhalt sind root-geschützt und wurden in dieser
Nachprüfung nicht direkt gelesen. Der Skriptabschluss ist vom Nutzer bestätigt;
Pi-Neustart und vollständige Backup-Wiederherstellung bleiben ungeprüft.

### Behobenes Startproblem bei der Einrichtung

Die falsche APT-Adresse `docker.com` durch die signierte offizielle Quelle
`https://download.docker.com/linux/ubuntu` ersetzt (Suite `noble`, Architektur
`arm64`, Schlüssel `/etc/apt/keyrings/docker.asc`). Danach scheiterte Docker mit
`no sockets found via socket activation`, während containerd bereits lief.
Folgende Schritte haben den Start wiederhergestellt:

```bash
sudo systemctl stop docker.service docker.socket
sudo systemctl daemon-reload
sudo systemctl reset-failed docker.service docker.socket
sudo systemctl start docker.socket
sudo systemctl start docker.service
sudo docker run --rm hello-world
```

Die ursprüngliche Ursache der fehlenden Socket-Übergabe ist nicht abschließend
geklärt. Keine Dienstdefinitionen oder Containerdaten dafür verändert.
Docker ist laut systemd aktiviert; ein Start nach Pi-Neustart bleibt ungeprüft.

## Vorbereitung

Auf dem Pi `cat /etc/os-release` und `uname -m` prüfen (ARM64: `aarch64`).
Docker nach der [offiziellen Ubuntu-Anleitung](https://docs.docker.com/engine/install/ubuntu/)
installieren; `sudo docker version` und `sudo docker compose version` prüfen.
Tailscale nach der [Linux-Anleitung](https://tailscale.com/docs/install/linux)
auf dem Host installieren und mit dem eigenen Tailnet verbinden.

Die folgenden Befehle im Repository auf dem Pi ausführen. Das Image wird dort
nativ für ARM64 gebaut. Rust und Node sind nur in den Build-Stufen enthalten;
die Installation auf dem Host ist nicht nötig. Der erste Build benötigt Zeit,
freien Speicher und Internetzugriff. `cargo test --locked` läuft beim Image-Bau.

```bash
sudo docker build -t nextleaf:0.1.0-pi1 .
sudo install -d -m 0750 /etc/nextleaf
sudo install -d -o 10001 -g 10001 -m 0750 /var/lib/nextleaf
sudo install -m 0640 compose.yaml /etc/nextleaf/compose.yaml
sudo test -e /etc/nextleaf/.env || sudo install -m 0600 deploy/.env.example /etc/nextleaf/.env
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml config
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml up -d --wait
curl --fail http://127.0.0.1:8787/api/health
```

Erwartet: `status` ist `ok`; ab pi2 enthält `capabilities` zusätzlich `task-editing`. Auf dem Pi ist die Weboberfläche zusätzlich unter
`http://127.0.0.1:8787` erreichbar. Für jeden neuen Build einen neuen Image-Tag
verwenden und in `/etc/nextleaf/.env` unter `TASKMANAGER_IMAGE` eintragen.
Es gibt noch kein veröffentlichtes Registry-Image; Compose verwendet mit
`pull_policy: never` ausschließlich das zuvor lokal gebaute Image.

## Daten und Zugriff

| Host | Verwendung |
| --- | --- |
| `/etc/nextleaf/compose.yaml` | Containerkonfiguration |
| `/etc/nextleaf/.env` | Ausgewählte Image-Version und lokale Einstellungen |
| `/var/lib/nextleaf/` | Dauerhafte Daten, nach `/data` eingebunden |

Die App läuft mit UID/GID `10001:10001`. Der Datenordner muss diesem numerischen
Benutzer gehören; dafür wird kein Host-Benutzer angelegt. Fehlende Host-Ordner
werden von Compose absichtlich nicht automatisch angelegt. Das Root-Dateisystem
ist schreibgeschützt, `/tmp` flüchtig. Logs gehen an Docker und werden rotiert.
Die SQLite-Datei heißt aus Kompatibilitätsgründen weiterhin `taskmanager.db`;
auch die Umgebungsvariablen behalten das Präfix `TASKMANAGER_`.

Der Host veröffentlicht ausschließlich `127.0.0.1:8787`. Für andere Geräte
Tailscale Serve auf dem Pi einrichten:

```bash
sudo tailscale serve --bg http://127.0.0.1:8787
tailscale serve status
```

Die ausgegebene HTTPS-Adresse im Browser eines verbundenen Geräts öffnen.
Gegebenenfalls HTTPS im Tailnet aktivieren. Zugriff per Tailnet-Regeln auf die
berechtigten Nutzer/Geräte beschränken: Die App besitzt noch keinen eigenen
Login. Keine Router-Portfreigabe und kein Tailscale Funnel einrichten.
Siehe [Tailscale Serve](https://tailscale.com/docs/features/tailscale-serve).

## Start- und Persistenzprüfung

Vor echten Daten eine Testaufgabe über die HTTPS-Weboberfläche anlegen.
Dann den Container neu erstellen:

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml up -d --force-recreate --wait
curl --fail http://127.0.0.1:8787/api/health
curl --fail http://127.0.0.1:8787/api/tasks
```

Die Testaufgabe muss nach Neuladen der Oberfläche mit derselben ID weiterhin
vorhanden sein. Danach auch einen Pi-Neustart und den Tailscale-Zugriff prüfen.
`restart: unless-stopped` startet einen zuvor laufenden Dienst wieder.
Diagnose:

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml ps
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml logs --tail 100
```

## Sicherung, Wiederherstellung und Updates

Für eine einfache konsistente Sicherung die App stoppen:

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml stop
```

Während sie gestoppt bleibt, **das gesamte** `/var/lib/nextleaf` und
`/etc/nextleaf` mit dem Server-Backup auf ein separates Ziel sichern. Auch
Image-Tag und zugehörigen Git-Commit aufbewahren; der Quellstand muss gesichert
sein, solange kein Registry-Image verfügbar ist. Danach wieder starten:

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/compose.yaml start
curl --fail http://127.0.0.1:8787/api/health
```

Nicht nur die einzelne DB-Datei aus dem laufenden Betrieb kopieren. Für spätere
Sicherungen ohne Unterbrechung die SQLite Online Backup API einsetzen.
Backup-Ziel und Zeitplan sind noch festzulegen.

Wiederherstellung zuerst auf einer getrennten Instanz testen: dieselbe
App-Version bauen, bei gestoppter App die gesicherten Verzeichnisse
wiederherstellen, Datenbesitz auf `10001:10001` setzen und mit `up -d --wait`
starten. Inhalte und SQLite-Integrität prüfen. Vor Updates immer sichern;
Migrationen laufen beim Start. Ein älteres Image allein ist nach einer
Schemaänderung kein verlässlicher Rollback: gegebenenfalls auch das passende
Datenbackup wiederherstellen.

## Prüfstand

Compose-Konfiguration auf macOS mit dem offiziellen Compose-Programm geprüft.
Docker-Grundbetrieb auf ARM64 per Nutzer-Terminalausgabe bestätigt (siehe oben).
Nextleaf-ARM64-Image-Build, Tests, Laufzeit und Persistenz nach
Container-Neuerstellung und HTTPS-Erreichbarkeit über Tailscale bestanden.
Pi-Neustart, Browser-/iPhone-Sichtprüfung und vollständiger Backup/Restore-Test
sind noch auszuführen.

## Gebündeltes pi5-Update: Projekte, MCP und Editor (2026-10-17)

Das Paket `nextleaf-update-mcp-20261017` ersetzt das noch nicht ausgeführte
pi4-Paket; pi4 muss nicht vorher installiert werden. Lokal vorbereitet,
produktiver Build und Wechsel noch ausstehend.

Auf dem Pi ausführen:

```bash
sudo bash ~/nextleaf-update-mcp-20261017/update.sh
```

`sudo` erlaubt die Änderung der geschützten App-Konfiguration. Das Skript prüft
Prüfsummen, baut `nextleaf:0.1.0-pi5` vor der Unterbrechung, sichert Quellstand,
Daten und Konfiguration, installiert die additive Projektmigration, übernimmt die
neue Compose-Datei und startet die App mit unverändertem Daten-Bind-Mount.
Es erzeugt einen zufälligen MCP-Schlüssel nur, wenn noch keiner konfiguriert ist,
und ergänzt den lokalen sowie privaten Tailscale-Hostnamen. Der Schlüssel wird
nicht ausgegeben; die Konfiguration bleibt 0600. Vorhandene MCP-Werte bleiben erhalten.

Health, Weboberfläche, SQLite und der authentifizierte MCP-Zugang werden geprüft.
Erfolgsmeldungen: NEXTLEAF_MCP_OK und NEXTLEAF_UPDATE_OK. Nach möglichem
Migrationsstart wird bei Fehlern gestoppt und kein altes Image automatisch gegen
die neue Datenbank gestartet. Sicherungspfad steht im Protokoll.

Der Zugang bleibt im Tailnet; das Skript richtet weder einen öffentlichen Port
noch Cloud-Connectoren ein. ChatGPT/Claude benötigen anschließend ihren
Verbindungsweg; siehe [MCP.md](MCP.md). Der Mac-Build verwendet dieselben Pi-Daten.

## Desktop-Installation: beschlossener Release-Weg (2026-10-17)

Vom Nutzer bestätigt, noch kein veröffentlichter Installationsweg:
Fertige native Pakete als Release-Downloads neben dem Git-Quellcode anbieten.
Falls GitHub als Hosting gewählt wird, GitHub Releases nutzen; Hosting und
Build-Automation sind noch festzulegen. Nutzer benötigen keinen Quellcode-Build.

- macOS: DMG herunterladen, öffnen und Nextleaf nach Programme ziehen.
- Windows: Setup-EXE herunterladen und installieren.
- Linux: zunächst AppImage, zusätzlich DEB für Debian/Ubuntu prüfen.

Tauri unterstützt diese Formate. `bundle.targets = "all"` aktiviert bereits
plattformabhängige Bundles, ist aber kein Nachweis fertiger Installer für alle
Systeme. Bisher macOS-Debug-App geprüft; Release-Signierung/Notarisierung sowie
Windows-/Linux-Installer ungeprüft. Für reguläre macOS-Veröffentlichung Signierung
und Notarisierung vorbereiten; Schlüssel bleiben außerhalb des Repositories.

Geplanter Offline-Desktop muss lokale Rust-Komponenten und Datenbankinitialisierung
mitliefern. Beim ersten Verbinden Pi-Adresse konfigurieren und Daten übernehmen.
Benutzerdaten liegen außerhalb des App-Pakets im Anwendungsdatenordner und müssen
beim Update erhalten bleiben. Diese Offline-Funktion ist noch nicht implementiert.
Zunächst Updates durch Installation einer neueren Version; integrierte signierte
Updates via Tauri-Updater sind eine spätere Option, nicht bereits beauftragt.

Quellen: [Tauri Distribution](https://v2.tauri.app/distribute/),
[Tauri Updater](https://v2.tauri.app/plugin/updater/).

### Pi5-Nachprüfung nach Nutzermeldung (2026-10-17)

Nutzer meldet NEXTLEAF_UPDATE_OK für das MCP-Gesamtupdate. Anschließend lesend
geprüft: Docker/tailscaled aktiv, Web-Root HTTP 200, `/api/health` lokal und über
privates Tailscale-HTTPS mit status ok, project-editing und mcp; MCP enabled true.
Unauthentifizierte POST-Anfrage an `/mcp` liefert 401 wie vorgesehen.
Authentifizierter Tool-Test in dieser Nachprüfung nicht ausgeführt, da sudo ein
Passwort verlangt; keine Secrets ausgelesen und keine produktiven Daten verändert.
Reale ChatGPT-/Claude-Verbindung weiterhin offen.


## pi6: mobile Oberfläche und Desktop-Abgleich (2026-10-18)

Lokales Paket: `data/update-offline-20261018/`, Archiv
`data/nextleaf-update-offline-20261018.tar.gz`. Enthält Quellcode, Update-/Prüfskript
und Prüfsummen, keine Datenbank oder produktiven Schlüssel. Pi5 bleibt bis zur
Ausführung aktiv. Mac-Debug-Bundle wurde lokal gebaut und mit getrennten
Testdaten erfolgreich per stdio und nativer Oberfläche geprüft.

Nach Übertragung auf dem Pi ausführen:

```bash
sudo bash ~/nextleaf-update-offline-20261018/update.sh
```

`sudo` gibt die nötigen Administratorrechte; `bash` führt das Update-Skript aus.
Es prüft alle Paketdateien, baut zuerst `nextleaf:0.1.0-pi6`, stoppt erst danach
kurz den Dienst und sichert Daten/Konfiguration unter
`/var/backups/nextleaf/<UTC-Zeit>-pi6/`. Bestehende Secrets bleiben erhalten.
Dann startet es die neue Version und prüft Datenbankintegrität/Fremdschlüssel,
Health-Fähigkeiten `sync`/`conditional-updates`, Web und authentifiziertes MCP.
Nach möglichem Migrationsbeginn erfolgt kein automatisches Downgrade.
Die SQLite-Prüfung bestätigt Integrität, keinen separaten Vorher-/Nachher-Inhaltsvergleich.
Erfolgsmeldung: `NEXTLEAF_UPDATE_OK`.

Danach iPhone-Web neu laden und das neue lokale Mac-Bundle starten. Der Mac nutzt
die bisher gespeicherte Pi-Adresse für den ersten Abgleich. Er erhält eine eigene
Datenbank im OS-Anwendungsdatenverzeichnis; keine alte lokale DB über den Pi kopieren.
Unter Settings stehen Sync-Status, Konflikte und das konkrete lokale MCP-Kommando.
Die App kann offline gestartet werden; Änderungen warten dauerhaft auf Verbindung.
Privater Tailscale-Zugang bleibt unverändert, keine öffentliche Freigabe.

Noch offen: echtes iPhone, reale Agenten-Clients und Backup-Restore.

Paket am 2026-10-18 übertragen und Prüfsummen/Shell-Syntax auf dem Pi geprüft
(`NEXTLEAF_PACKAGE_VERIFIED`). Installation vom Nutzer am 2026-10-18 abgeschlossen.
Unabhängig nachgeprüft: Docker/Tailscale aktiv, lokales Health/Web und privates
HTTPS erreichbar, sync/conditional-updates vorhanden. Ausgelieferte Web-Assets
entsprachen beim Test dem lokalen pi6-Build. Mac-App geöffnet und Synced;
vollständiger Datenvergleich mit Pi identisch: 119 Aufgaben, 11 Projekte,
3 Areas, 12 Tags. Sync-Baseline identisch, keine Konflikte/ausstehenden Änderungen,
SQLite-Integrität und Fremdschlüssel in Ordnung. Anonymes MCP wird mit 401 abgelehnt.
Keine produktiven Aufgaben durch diesen Test geändert. Rootgeschütztes Update-Log
und Backup nicht erneut gelesen; echter Restore weiterhin ungeprüft.
Nachfolgende UI-Änderungen (Untagged und weitere Bedienrunde) sind nicht in pi6 enthalten.
