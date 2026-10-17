# TaskManager – Produktspezifikation (Stand: 17.10.2026)

## Leitidee
Ein eigenständiger GTD-Task-Manager: Bedienung und Einfachheit orientieren sich an Nirvana HQ, ausgewählte gute Konzepte an OmniFocus. Keine 1:1-Übernahme von geschütztem Code oder Gestaltung. Schrittweise entwickeln und nur gewünschte Funktionen ergänzen.

## Grundstruktur
- Flach: **Area → Projekt → Aufgabe**. Aufgaben dürfen auch ohne Projekt bestehen.
- Keine Unterprojekte und keine Unteraufgaben in Version 1.
- Inbox, Next, Waiting, Scheduled, Someday, Focus, Logbook, Areas, Projekte, Aufgaben, Tags.
- Projekte: **Sequential** und **Parallel**.
- Sequential wie OmniFocus: **nur die erste noch offene Aufgabe** eines aktiven sequenziellen Projekts darf als nächste Aktion angeboten werden; spätere Aufgaben sind im Projekt sichtbar, aber nicht in globalen Next-Listen. Ist die erste offene Aufgabe nicht verfügbar (z. B. Waiting), wird nichts übersprungen. Die Reihenfolge ist veränderbar.
- Die Berechnung der Verfügbarkeit liegt im Backend; Oberflächen und KI-Clients wenden dieselben Regeln an.

## Eigene Ansichten / Perspectives
- Benutzerdefinierte, gespeicherte Filterabfragen über Aufgaben **und** Projekte.
- Beispiele: Area, Tags, GTD-Status, Verfügbarkeit, Projektart, Termine.
- Filter zunächst mit UND-Verknüpfung; später bei Bedarf ODER und Gruppierung.
- Eine Ansicht speichert Kriterien, nicht die Ergebnismenge.

## KI-Schnittstelle
- Ein MCP-Server für Lesen, Anlegen und Aktualisieren von Areas, Projekten, Aufgaben, Tags sowie Abfragen von Perspectives ist vorgesehen.
- Eine gemeinsame Geschäftslogik für REST, MCP und die Oberfläche.

## Plattformen
- Ab erster Phase: Rust-Server lokal auf dem Mac; Web-App (mobil nutzbar im iPhone-Browser), Desktop-App für macOS mit Tauri 2; Windows und Linux von Anfang an in der Architektur berücksichtigen und getrennt testen.
- Später optional VPS (z. B. Hetzner) und sichere Verbindung (z. B. Tailscale).
- Später optional native iPhone-App: offline nutzbar, Änderungen lokal sichern und nach Verbindung synchronisieren.
- Derzeit **keine** Offline-Synchronisation in Phase 1 erforderlich. Online-Web-App benötigt erreichbaren Server.
- Keine permanente Nirvana-Importfunktion in Version 1; vorhandene Daten bei Bedarf später einmalig einfügen.

## Technik
- Rust, Axum, SQLite und SQLx für den Backend-Dienst.
- React, TypeScript und Vite für dieselbe UI in Browser und Tauri 2.
- Tauri 2 für macOS, Windows und Linux.
- Ein Monorepo; der lokale Checkout liegt aktuell unter `~/Developer/repo/TaskManager`. Der Speicherort ist keine technische Voraussetzung.
- Testdatenbank lokal in `data/`, nicht in Git. Produktive Daten später in OS-spezifischem Anwendungsdatenordner.

## Grundsätze
- Entwicklung und Planung erfolgen vollständig in Codex. Spezifikation, neue Anforderungen und Architekturentscheidungen werden dauerhaft unter `docs/` gepflegt, sodass neue Sitzungen nahtlos weiterarbeiten können. `AGENTS.md` ist der Einstieg; `DECISIONS.md` dokumentiert Entscheidungen und `NEXT_STEPS.md` den Arbeitsstand.
- Einfachheit vor optionaler Flexibilität.
- Server-seitige Regeln für den GTD-Status, keine doppelte Businesslogik im Frontend.
- stabile UUIDs und Änderungszeitpunkte als Vorbereitung auf spätere Synchronisation.
- Lokales Testsystem nicht ungeschützt ins Internet stellen; Authentifizierung und Transportverschlüsselung vor Remote-/VPS-Betrieb implementieren.

## Erstes Inkrement (diese Lieferung)
- Projektstruktur, Spezifikation, Grund-API, SQLite-Migration, erste Weboberfläche und Tauri-Konfiguration.
- Kernaktionen: Areas und Projekte anlegen, Aufgaben erfassen und abschließen, Listen filtern, Next-Actions für sequenzielle Projekte filtern.
- Perspektiven, Tags-UI, Focus-Workflow, umfangreiche Bearbeitung, MCP, Authentifizierung und Synchronisation kommen **nach** dem Gerüst. Das Gerüst ist nicht die fertige Version 1.
