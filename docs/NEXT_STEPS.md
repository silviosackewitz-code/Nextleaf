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
