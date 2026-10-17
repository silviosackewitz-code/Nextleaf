# TaskManager

Ein GTD-Task-Manager nach dem **einfachen Bedienprinzip von Nirvana HQ** mit OmniFocus-inspirierter Sequential-Logik, später eigenen Perspectives und MCP. Eigenständiges Open-Source-fähiges Projekt; kein Code von Nirvana oder OmniFocus enthalten.

## Voraussetzungen auf dem Mac
- Rust (`rustup`, inklusive Cargo)
- Node.js (aktuelle LTS-Version) und npm
- Xcode Command Line Tools (`xcode-select --install`), für Tauri auf macOS außerdem die üblichen Tauri-Systemvoraussetzungen

Rust lässt sich über den offiziellen Installer auf <https://rustup.rs> einrichten.
Falls Cargo noch nicht im Suchpfad liegt, `source "$HOME/.cargo/env"` ausführen.
Mit `rustc --version`, `cargo --version`, `node --version` und `npm --version` prüfen.

## Installation

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

Auch für die Desktop-App muss der Rust-Server **separat laufen**. Das automatische Starten des Servers als Sidecar folgt in einem späteren Entwicklungsschritt.

### Statische Web-App über den Rust-Server

```bash
cd ~/Developer/repo/TaskManager
npm run web:build
cd backend
cargo run
```

Danach ist bei vorhandenem `frontend/dist` die Weboberfläche unter <http://127.0.0.1:8787> erreichbar.

### iPhone im lokalen WLAN (nur später bei Bedarf)

Der Server bindet standardmäßig aus Sicherheitsgründen an `127.0.0.1`. Für Tests im **privaten WLAN** kann man ihn gezielt unter `0.0.0.0:8787` starten (und die Web-App vorher bauen). Ohne Authentifizierung **niemals** einen öffentlichen Port freigeben. Auf dem iPhone dann `http://<mac-ip>:8787` im Browser öffnen.

```bash
cd ~/Developer/repo/TaskManager/backend
TASKMANAGER_BIND=0.0.0.0:8787 cargo run
```

Die produktive Konfiguration mit Zugriffsschutz/TLS/Tailscale folgt erst später.

## Erste Funktionen im Gerüst
- Areas erstellen
- Parallele und sequenzielle Projekte erstellen
- Aufgaben in Inbox oder Next/Waiting/Scheduled/Someday anlegen
- Aufgaben abschließen und wieder öffnen
- Globale GTD-Listen anzeigen und Projekte/Areas filtern
- In globaler **Next**-Ansicht erscheinen bei sequenziellen Projekten nur die ersten offenen verfügbaren Aufgaben
- Server-URL in der Desktop-Oberfläche manuell anpassen (für Windows/Linux-Clients im LAN)

**Noch nicht implementiert:** Tags-UI, Focus, Datumsverwaltung, Reihenfolge verschieben, editierbare Projekte, gespeicherte Perspectives, MCP, Authentifizierung, Sync, native iPhone-App, Desktop-Server-Sidecar. Sie sind geplant, nicht als fertig markieren.

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

## Git

Nach erfolgreichem Einrichten:

```bash
git add .
git commit -m "Initiales TaskManager-Geruest"
```

Die `.gitignore` schützt DB-Dateien, `node_modules`, Rust-Build-Artefakte und lokale Secrets. Für eine zuverlässige Entwicklungsumgebung bei späterer Nutzung `Cargo.lock` und die npm-Lockfiles nach Installation committen.

## Hinweis
Dieses Repository ist ein **Starterprojekt**, noch keine fertige Task-Manager-Anwendung.
Backend-Tests, API, SQLite und Web-App wurden auf macOS geprüft; Tauri-Entwicklungsbuild
und macOS-App-Bundle bauen erfolgreich. Die native Sichtprüfung sowie Windows/Linux-
und Release-Tests stehen noch aus. Details stehen in [docs/NEXT_STEPS.md](docs/NEXT_STEPS.md).
