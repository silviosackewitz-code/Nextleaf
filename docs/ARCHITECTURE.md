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
