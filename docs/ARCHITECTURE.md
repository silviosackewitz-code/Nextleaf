# Architektur

Anforderungen stehen in [SPECIFICATION.md](SPECIFICATION.md), begründete
Architekturentscheidungen in [DECISIONS.md](DECISIONS.md) und der aktuelle
Prüf- und Arbeitsstand in [NEXT_STEPS.md](NEXT_STEPS.md). Diese Dokumente sind
die dauerhafte Übergabe zwischen Codex-Sitzungen.

```text
                     React + TypeScript (Vite)
                       /                 \
           Webbrowser / mobile        Tauri-2-Desktop-Shell
                       \                 /
                    HTTP JSON API (Axum)
                            |
              Rust: GTD- und Sequential-Regeln
                            |
                        SQLite (SQLx)

              MCP-Adapter (später, dieselbe Logik)
```

## Datenhaltung
- `areas`, `projects`, `tasks`, `tags`, `task_tags`, `saved_views` (Perspectives als späterer Ausbau).
- UUIDs als TEXT und RFC3339-Zeitstempel als TEXT; Sortierreihenfolge explizit gespeichert.
- Kein direktes Schreiben von Clients in die Server-Datenbank.
- `available` und `blocked_by_sequence` werden als **abgeleitete** Informationen berechnet.
- Für sequenzielle Projekte ist das erste nicht abgeschlossene und nicht gelöschte Task-Element der Reihenfolge maßgeblich. Nur dieses darf (bei Status `next` und ohne aktive Zurückstellung) in die Next-Liste.
- Noch **kein** Offline-Datenspeicher je Client; spätere Synchronisation benötigt Change Tracking, Geräte-IDs, Tombstones und Konfliktpolitik. Dafür erst einen gesonderten Entwurf freigeben.

## Entwicklungsbetrieb
1. Rust-Backend lauscht standardmäßig nur auf `127.0.0.1:8787`.
2. Vite auf `http://localhost:1420`; `/api` wird in der Entwicklung an Axum weitergeleitet.
3. Tauri lädt in der Entwicklung die Vite-Weboberfläche; im Paket das gebaute Frontend.
4. In der Desktop-App wird die Server-URL in lokalen Einstellungen gespeichert (Default `http://127.0.0.1:8787`). Bei Web-Nutzung wird standardmäßig dieselbe Origin verwendet.
5. Der Desktop-Client startet den Server **noch nicht automatisch**; beide Prozesse starten in dieser Etappe getrennt.

## Sicherheit / Remote-Zugriff
Aktuell ist die API ein **lokaler Entwicklungsdienst ohne Login**. Nicht öffentlich routen oder ins Internet exponieren. Vor späterem VPS-Betrieb Authentifizierung, Autorisierung, CORS-Allowlist, TLS/gesicherten Tunnel, Backup/Restore und Migrationsstrategie ergänzen.

## Vorbereiteter Docker-Betrieb auf dem Raspberry Pi 5

- Docker Engine und Compose auf Ubuntu Server (ARM64).
- Ein App-Container enthält das Rust-Backend und das gebaute React-Frontend;
  Axum liefert API und Weboberfläche aus. Tauri bleibt ein Desktop-Client.
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
Compose-Konfiguration validiert; Docker-/ARM64-Build und Betrieb stehen aus.
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
