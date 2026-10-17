# Nextleaf auf dem Raspberry Pi

Ziel: Raspberry Pi 5, Ubuntu Server LTS mit 64 Bit, Docker Engine mit Compose
und Tailscale auf dem Host. Der Container enthält Backend und Weboberfläche.
Tauri wird auf dem Server nicht benötigt.

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

Erwartet: `{"status":"ok"}`. Auf dem Pi ist die Weboberfläche zusätzlich unter
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
Docker-Image-Build, ARM64-Laufzeit, Persistenz nach Container-Neuerstellung,
Tailscale-Zugriff und Backup/Restore sind auf dem Pi noch auszuführen.
