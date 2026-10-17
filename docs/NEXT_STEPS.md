# Arbeitsplan / Nächste Schritte

## 0. Gerüst auf dem Mac verifizieren
- [x] Lokalen Checkout unter `~/Developer/repo/TaskManager` und vorhandenes Git-Repository prüfen.
- [x] Rust, Node und Tauri-Systemvoraussetzungen prüfen; fehlendes Rust installieren.
- [x] Backend: `cargo fmt --check`, `cargo test`, `cargo run`.
- [x] Frontend: `npm --prefix frontend install`, `npm run web:build`, `npm run web:dev`.
- [x] Tauri: `npm install`, `npm run desktop:dev`.
- [x] SQLite-Datenbank und Web-Listendarstellungen mit Testaufgaben prüfen.
- [x] Testen: Sequential-Projekt A→B: A ist Next, B nicht; nach Abschluss von A wird B Next. Bei A=Waiting ist B weiter blockiert.
- [x] macOS-App mit eingebautem Frontend bauen (`npm run desktop:build -- --debug --bundles app`).
- [ ] Natives macOS-Fenster und dessen Backend-Verbindung visuell prüfen.
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
  durchgeführt. Laut Nutzer läuft die neueste Ubuntu-LTS-Version. Aktuell ist
  dies Ubuntu 26.04.1 LTS (Stand 2026-10-17); die tatsächlich installierte Version
  und Architektur vor Ort mit `cat /etc/os-release` und `uname -m` bestätigen.
  ARM64 als Build-Ziel vorgesehen; vorhandene Dienste und Speicher prüfen.
  Referenzen: [Ubuntu LTS](https://releases.ubuntu.com/26.04/),
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
- Lesen/Erstellen/Ändern/Abschließen von Aufgaben und Projekten.
- Zugriff auf Areas, Tags, eigene Perspectives.
- Sicherstellen, dass MCP dieselben Regeln wie die REST-API verwendet.
- Lokale Authentisierung/Berechtigungen klären, bevor externe Agenten zugreifen.

## 4. Stabilisierung und Desktop
- Einheitliche macOS-, Windows- und Linux-Builds und Tests.
- Optional eingebetteter Rust-Server/Sidecar für komfortablen lokalen Start.
- Backups, Datenbankmigrationen, automatisierte Tests.
- Vorbereitung für VPS-Zugriff und erst dann Authentisierung/TLS.

## Später (nicht Version 1)
- Native iPhone-App mit Local-First-Offline-Datenspeicher, Sync und Konfliktlösung.
- Umzug zu Hetzner/Tailscale nach Bedarf.
- Einmalige Übernahme vorhandener Nirvana-Daten, keine feste Importfunktion.
