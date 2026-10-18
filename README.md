# Nextleaf

Projektname: **Nextleaf**. Der lokale Checkout-Ordner heißt derzeit weiterhin `TaskManager`.

Ein GTD-Task-Manager nach dem **einfachen Bedienprinzip von Nirvana HQ** mit OmniFocus-inspirierter Sequential-Logik, später eigenen Perspectives und MCP. Eigenständiges Open-Source-fähiges Projekt; kein Code von Nirvana oder OmniFocus enthalten.

## Installation für Anwender

[Installationsanleitung für Pi, Mac und iPhone](docs/INSTALLATION.md).
Sie beschreibt den Download-/Compose-Weg, die native Mac-App, die iPhone-Web-App
und den geplanten Neustart mit leeren Daten. **Release-Downloads, Container-Registry
und signiertes Mac-DMG sind noch nicht veröffentlicht.** Der aktuelle Testbetrieb
und Prüfstand stehen in [NEXT_STEPS.md](docs/NEXT_STEPS.md).

Die folgenden Befehle richten eine **Entwicklungsumgebung** ein.

## Voraussetzungen für Entwicklung auf dem Mac
- Rust (`rustup`, inklusive Cargo)
- Node.js (aktuelle LTS-Version) und npm
- Xcode Command Line Tools (`xcode-select --install`), für Tauri auf macOS außerdem die üblichen Tauri-Systemvoraussetzungen

Rust lässt sich über den offiziellen Installer auf <https://rustup.rs> einrichten.
Falls Cargo noch nicht im Suchpfad liegt, `source "$HOME/.cargo/env"` ausführen.
Mit `rustc --version`, `cargo --version`, `node --version` und `npm --version` prüfen.

## Entwicklungsumgebung einrichten

Im Repository-Verzeichnis arbeiten. Die folgenden Beispiele verwenden
`~/Developer/repo/TaskManager`; bei einem anderen Speicherort den Pfad entsprechend ersetzen.

```bash
cd ~/Developer/repo/TaskManager
npm install
npm --prefix frontend install
```

### Backend starten (Terminal 1)

```bash
cd ~/Developer/repo/TaskManager/backend
cargo run
```

Die DB wird beim ersten Start automatisch als `data/taskmanager.db` im Repository angelegt und migriert.
Das Backend aus `backend/` starten, da die Standardpfade relativ zum Arbeitsverzeichnis sind.
Optional `.env.example` nach `.env` im Repository kopieren; das Backend lädt diese Datei auch aus dem übergeordneten Verzeichnis.
Test: <http://127.0.0.1:8787/api/health> → `{"status":"ok"}`.

### Web-App starten (Terminal 2)

```bash
cd ~/Developer/repo/TaskManager
npm run web:dev
```

Dann <http://localhost:1420> öffnen. Vite leitet `/api` an den Rust-Server weiter.

### macOS Desktop-App starten (Terminal 2 statt Web-App)

```bash
cd ~/Developer/repo/TaskManager
npm run desktop:dev
```

Die Desktop-App enthält den Rust-Fachcode und eine lokale SQLite-Datenbank.
Ein separater Backend-Prozess ist dafür nicht nötig. Den Pi-Abgleich in Settings
ausdrücklich verbinden; ohne Pi bleibt die Desktop-App lokal nutzbar.

### Statische Web-App über den Rust-Server

```bash
cd ~/Developer/repo/TaskManager
npm run web:build
cd backend
cargo run
```

Danach ist bei vorhandenem `frontend/dist` die Weboberfläche unter <http://127.0.0.1:8787> erreichbar.

### iPhone-Web-App

Die mobile Web-App wird vom Pi über die private Tailscale-HTTPS-Adresse geöffnet.
Einrichtung und Home-Bildschirm: [Installationsanleitung](docs/INSTALLATION.md).

## Raspberry Pi mit Docker

Dockerfile und Compose-Konfiguration sind vorbereitet. Einrichtung, Tailscale,
Persistenzprüfung und Backup/Restore stehen in [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md).
Konfiguration liegt auf dem Pi unter `/etc/nextleaf`, Daten unter
`/var/lib/nextleaf`. Der Container veröffentlicht nur einen lokalen Host-Port.
pi7 läuft auf ARM64 und wurde nach dem Update geprüft. Der öffentliche
Download-/Pull-Installationsweg bleibt bis zur Release-Veröffentlichung offen.

## Funktionsstand

Aufgaben, Projekte, Areas, Tags/Personen, Focus, Termine und Waiting-Kontakte
sind bearbeitbar. Desktop speichert lokal und synchronisiert zum Pi; die
mobile Web-App bleibt online. REST und MCP verwenden denselben Rust-Fachcode.
Perspectives, Review und native iPhone-Offline-App sind noch nicht implementiert.
Die vollständigen Regeln stehen in [SPECIFICATION.md](docs/SPECIFICATION.md).

## Struktur

Für die Weiterentwicklung zuerst [AGENTS.md](AGENTS.md) lesen. Spezifikation,
Architektur, Entscheidungen und Arbeitsstand werden dauerhaft in `docs/` gepflegt.

```text
backend/      Rust (Axum, SQLx), SQLite-Migration und Unit-Tests
frontend/     React, TypeScript, Vite; responsive Weboberfläche
src-tauri/    Tauri-2-Desktop-Shell (macOS/Windows/Linux)
docs/         Spezifikation und Architektur
data/         lokale SQLite-Dateien, nicht in Git
```

## Entwicklung und Freigabe

Lokale Daten und Secrets sind durch `.gitignore` ausgeschlossen. Vor Änderungen
[AGENTS.md](AGENTS.md) lesen. Windows/Linux, signierte Release-Pakete und die
Erstinstallation über veröffentlichte Downloads sind getrennt zu prüfen.
