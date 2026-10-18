# Nextleaf installieren: Raspberry Pi, Mac und iPhone

Stand: 18.10.2026. Diese Anleitung beschreibt die gewünschte Neuinstallation
mit leerem Datenbestand. Docker läuft auf dem **Pi**; auf dem **Mac** wird eine
native App installiert. Das **iPhone** öffnet die vom Pi bereitgestellte Web-App.

## Verfügbarkeit vor dem Start

**Der vollständige Download-Installationsweg ist noch nicht freigegeben.**
Der bestehende Testserver läuft auf pi7, die Mac-App ist ein lokal gebautes
Apple-Silicon-Debug-Paket. Es gibt noch keine veröffentlichte Release-Adresse,
kein abrufbares Nextleaf-Container-Image und kein signiertes/notarisiertes Mac-DMG.
Der Quellcode liegt im privaten [GitHub-Repository](https://github.com/silviosackewitz-code/Nextleaf).
Release-Dateien und Images sind dort noch nicht veröffentlicht. Die unten genannten
Download-Befehle erst verwenden, wenn diese Voraussetzungen erfüllt sind.

| Benötigt | Stand |
| --- | --- |
| `docker-compose.yml` und `example.env` derselben Version | Vorlagen unter [deploy/release](../deploy/release/) vorhanden |
| Versionierter HTTPS-Download mit Prüfsummen | Noch zu veröffentlichen |
| Fertiges Container-Image für Linux ARM64 (Pi), später auch AMD64 | Noch zu veröffentlichen und per Pull zu testen |
| Mac-DMG für die unterstützte Architektur mit Prüfsumme | Noch zu bauen, zu signieren/notarisieren und zu veröffentlichen |
| Installation auf leeren Geräten nach dieser Anleitung | Noch auszuführen |

Das private Repository verlangt Anmeldung; normale `wget`-Aufrufe können daraus
keine privaten Release-Dateien laden. Für den hier geplanten Endnutzerweg müssen
noch geeignete abrufbare Release-/Registry-Adressen bereitgestellt werden.

Die Root-Datei `compose.yaml` und `deploy/.env.example` gehören zum bisherigen
lokalen Build-/Updateweg. Für die Downloadinstallation ausschließlich die
Release-Dateien verwenden. Das pi7-Updatepaket ist kein Erstinstaller.

## 1. Voraussetzungen

Auf dem Pi: 64-Bit-Ubuntu, Docker Engine mit Compose-Plugin, `wget`, `curl`,
`sha256sum` sowie ein angemeldetes Tailscale. Der vorhandene Test-Pi verwendet
Ubuntu 24.04 LTS und ARM64. Prüfen:

```bash
uname -m
docker --version
docker compose version
tailscale status
```

Falls Docker fehlt, die [offizielle Ubuntu-Anleitung](https://docs.docker.com/engine/install/ubuntu/)
verwenden; das [Compose-Plugin](https://docs.docker.com/compose/install/) gehört dazu.
Auf Mac und iPhone ebenfalls Tailscale installieren und mit demselben privaten
Netz verbinden. Rust, Node und Docker sind für die fertige Mac-App nicht nötig.

Nextleaf hat derzeit keinen eigenen Web-Login. REST und Sync bleiben deshalb
innerhalb des berechtigten privaten Tailnets. Keinen Router-Port öffnen und
kein Tailscale Funnel aktivieren. Der optionale MCP-Schlüssel schützt nur MCP.

## 2. Pi: Release-Dateien herunterladen

Die veröffentlichte Release-Seite muss eine konkrete Version, ihre HTTPS-Adresse,
das zugehörige Image samt Architektur und die Prüfsummen nennen. Keine `latest`-
Version für diesen Test verwenden. Alle Dateien müssen zur gleichen Version gehören.

In einem neuen Downloadordner arbeiten. Die Abfrage erwartet die echte,
veröffentlichte HTTPS-Basisadresse des Releases; derzeit ist noch keine verfügbar.

```bash
mkdir nextleaf-install
cd nextleaf-install
read -r -p 'HTTPS-Basisadresse des veröffentlichten Releases: ' NEXTLEAF_RELEASE_BASE
case "$NEXTLEAF_RELEASE_BASE" in https://*) ;; *) echo 'HTTPS-Adresse erforderlich'; exit 1 ;; esac
wget --https-only -O docker-compose.yml "${NEXTLEAF_RELEASE_BASE%/}/docker-compose.yml"
wget --https-only -O example.env "${NEXTLEAF_RELEASE_BASE%/}/example.env"
wget --https-only -O SHA256SUMS "${NEXTLEAF_RELEASE_BASE%/}/SHA256SUMS"
sha256sum --check --ignore-missing SHA256SUMS
```

Nur fortfahren, wenn ausdrücklich **beide** Konfigurationsdateien mit `OK`
gemeldet werden. Bei fehlenden Dateien oder Prüfsummenfehlern abbrechen.

Konfiguration an ihren festen Platz kopieren. Das ist eine Erstinstallation;
eine vorhandene `.env` nicht durch diese Vorlage überschreiben.

```bash
sudo install -d -m 0750 /etc/nextleaf
sudo install -m 0640 docker-compose.yml /etc/nextleaf/docker-compose.yml
sudo install -m 0600 example.env /etc/nextleaf/.env
sudo nano /etc/nextleaf/.env
```

In `.env` einstellen:

- `TASKMANAGER_IMAGE`: vollständige Image-Adresse mit der veröffentlichten Version
  oder dem veröffentlichten Digest. Die Vorlage lässt dieses Pflichtfeld leer.
- `NEXTLEAF_DATA_DIR`: absoluter Datenordner auf dem Pi, Standard `/var/lib/nextleaf`.
- `NEXTLEAF_MCP_TOKEN`: für die erste App-Prüfung leer lassen (MCP deaktiviert).
  Zur späteren Agentenanbindung einen eigenen starken Schlüssel erzeugen; siehe [MCP.md](MCP.md).
- `NEXTLEAF_MCP_HOSTS`: bei MCP-Nutzung den exakten privaten HTTPS-Hostnamen ergänzen.

Datenordner anlegen; bei anderem Datenpfad den folgenden Pfad entsprechend ersetzen:

```bash
sudo install -d -o 10001 -g 10001 -m 0700 /var/lib/nextleaf
```

Dieser Ordner liegt außerhalb des Containers. Bei einem normalen Update bleibt
er erhalten. Für eine wirklich leere Erstinstallation muss er leer sein.

## 3. Pi: Image laden und starten

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml config --quiet
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml pull
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml up -d --wait
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml ps
curl --fail http://127.0.0.1:8787/api/health
```

`config --quiet` prüft die Konfiguration, ohne Schlüssel anzuzeigen. `pull` lädt
das fertige Image; auf dem Pi wird nichts kompiliert. `up` startet den Container
und wartet auf den Healthcheck. Erwartet: Dienst gesund, `status: ok`, Fähigkeiten
`tag-management` und `sync-v2`. Bei Fehlern nicht mit den Clients weitermachen:

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml logs --tail 100
```

## 4. Privaten HTTPS-Zugriff einrichten

Bestehende Tailscale-Serve-Konfiguration zuerst ansehen, damit andere Dienste
nicht versehentlich ersetzt werden. Ist der HTTPS-Port für Nextleaf vorgesehen:

```bash
sudo tailscale serve status
sudo tailscale serve --bg http://127.0.0.1:8787
sudo tailscale serve status
```

Die ausgegebene private HTTPS-Adresse aufbewahren. Dafür müssen HTTPS-Zertifikate
im Tailnet aktiviert sein; Tailscale führt bei Bedarf durch die Einrichtung.
[Referenz: Tailscale Serve](https://tailscale.com/docs/reference/tailscale-cli/serve).

Adresse auf dem Mac im Browser öffnen. Eine frische Installation zeigt leere
Listen. Eine Aufgabe `Installation check` anlegen und die Seite neu laden: Sie
muss erhalten bleiben. Noch keine alten Datenbanken oder Nirvana-Daten importieren.

## 5. Native Mac-App installieren

1. Das zur Serverversion gehörende Mac-DMG von der Release-Seite herunterladen
   und seine veröffentlichte Prüfsumme prüfen. Apple Silicon und Intel benötigen
   ein passendes Paket; bisher wurde nur Apple Silicon lokal getestet.
2. DMG öffnen, `Nextleaf.app` nach **Programme** ziehen, DMG auswerfen.
3. Nextleaf aus Programme starten. Bei einem vorgesehenen Endnutzer-Release
   darf keine Anleitung zum Entfernen der Quarantäne oder Abschalten von Gatekeeper
   nötig sein. Eine Blockierung gehört als fehlgeschlagener Release-Test gemeldet.
4. In **Settings → Local data and sync** die private HTTPS-Adresse des Pi
   eintragen und **Connect and sync** wählen. Tailscale muss verbunden sein.
5. Auf **Synced** beziehungsweise **Changes are synced.** warten. Die zuvor im
   Browser erstellte Aufgabe muss erscheinen.
6. Auf dem Mac eine zweite Aufgabe anlegen. Im Browser neu laden und ihren
   Empfang prüfen. Danach Mac-App beenden und aus Programme erneut öffnen.

Die Mac-App speichert lokal und arbeitet auch offline. Für den Offline-Test
Tailscale am Mac vorübergehend trennen, eine Testaufgabe ändern, die App neu
starten und den lokalen Erhalt prüfen. Tailscale wieder verbinden; danach müssen
Änderung und Sync-Status auf Mac/Pi übereinstimmen. Die Web-App bleibt online.

## 6. iPhone-Web-App einrichten

1. Tailscale verbinden und die private HTTPS-Adresse in Safari öffnen.
2. Die Testaufgaben müssen sichtbar sein. Über das Navigationsmenü die Ansichten
   wechseln; über **+** eine Aufgabe anlegen.
3. Im Safari-Menü **Teilen → Zum Home-Bildschirm** wählen. Falls angeboten,
   **Als Web-App öffnen** aktivieren; Namen `Nextleaf` bestätigen.
4. Vom Home-Bildschirm starten und nochmals Zugriff sowie Speicherung prüfen.

Die genaue Menüposition hängt von iOS ab. [Apple beschreibt den aktuellen
Ablauf hier](https://support.apple.com/en-lamr/guide/iphone/iphea86e5236/ios).
Das Home-Bildschirm-Symbol ist ein Zugang zur Web-App, keine native Offline-App.
Ohne erreichbaren Pi/Tailscale können dort keine Änderungen gespeichert werden.

## 7. Abnahme auf dem echten iPhone

Im Safari-Tab **und** vom Home-Bildschirm prüfen:

- Hoch-/Querformat, Scrollen und geöffnete Bildschirmtastatur: Felder, Save/Done
  und Cancel bleiben erreichbar; keine seitlich abgeschnittenen Dialoge.
- **Settings → Manage Tags**: Area, Person und Tag anlegen, langen Namen eingeben,
  umbenennen, archivieren und über **Show archived → Restore** wiederherstellen.
- Aufgabe öffnen: Tag-Vorschlag antippen, Tag/Area entfernen, Änderungen speichern.
- **More options → Waiting for**: Kontaktvorschlag und eigenen Freitext verwenden;
  alternativ **⋯ → Waiting for** nutzen. Auswahl muss nach erneutem Öffnen bestehen.
- Zugewiesenen Tag/Area archivieren: bestehende Zuordnung bleibt sichtbar,
  bei neuen Aufgaben wird der archivierte Eintrag nicht angeboten.
- Ungespeicherte Änderungen abbrechen: **Keep editing** erhält den Entwurf,
  **Discard changes** verwirft ihn.
- Änderungen vom iPhone nach Neuladen im Mac und umgekehrt sichtbar; keine Konflikte.

Die Browserprüfung in iPhone-Größen ersetzt diesen Geräte-/Safari-Test nicht.

## 8. Bestehenden Teststand für einen sauberen Neustart entfernen

Diesen Schritt führt der Nutzer erst vor dem geplanten Neuinstallationstest aus.
Alle Geräte und lokalen MCP-Prozesse vorher beenden: Ein alter Mac-Bestand darf
nicht beim ersten Abgleich den leeren Server wieder befüllen. Ein neuer Server
hat außerdem eine neue Identität und kann von einem alten Client abgewiesen werden.

- **Pi:** Mit der aktuell verwendeten Compose-Datei `docker compose down`
  ausführen. Beim bisherigen pi7 ist das `/etc/nextleaf/compose.yaml`, beim neuen
  Releaseweg `/etc/nextleaf/docker-compose.yml`; jeweils die dortige `.env` angeben.
  Danach ausschließlich Nextleaf-Konfiguration und den tatsächlich verwendeten
  Nextleaf-Datenordner entfernen oder weglegen. `down --volumes` allein leert
  den Host-Bind-Mount nicht. Falls auch alte Nextleaf-Images/Updateordner entfernt
  werden sollen, diese einzeln auswählen; kein globales Docker-Prune verwenden.
- **Mac:** Alle Nextleaf-Instanzen und Agentenprozesse mit `Nextleaf --mcp`
  beenden. App aus Programme sowie ggf. alte Testkopien entfernen. Im Finder
  über „Gehe zum Ordner“ `~/Library/Application Support/dev.taskmanager.desktop`
  öffnen und den gesamten Nextleaf-Datenordner entfernen oder weglegen, samt
  SQLite-Begleitdateien. Für vollständig zurückgesetzte Oberflächeneinstellungen
  zusätzlich die zugehörigen Ordner unter `~/Library/WebKit/dev.taskmanager.desktop`
  und `~/Library/Caches/dev.taskmanager.desktop` entfernen, soweit vorhanden.
  Bei abweichendem Datenordner gilt der zuvor in Settings angezeigte Speicherort.
- **iPhone:** Nextleaf-Web-App vom Home-Bildschirm entfernen, zugehörige Safari-Tabs
  schließen und in den Safari-Einstellungen nur die Website-Daten der privaten
  Nextleaf-Adresse löschen. Dadurch werden die Aufgaben auf dem Pi nicht gelöscht.

Docker, Tailscale, andere Apps und das Git-Repository bleiben bestehen. Danach
zuerst den leeren Pi einrichten, dann die frische Mac-App verbinden, zuletzt das
iPhone. Testdaten nicht aus einem alten Backup zurückspielen.

## Updates und Hilfe

Bei normalen Updates Daten und `.env` erhalten und vorher konsistent sichern.
Server zuerst, kompatible Clients danach aktualisieren. Sicherung/Wiederherstellung
und bisheriger Testbetrieb: [DEPLOYMENT.md](DEPLOYMENT.md).
Aktueller Freigabe-/Prüfstand: [NEXT_STEPS.md](NEXT_STEPS.md).
