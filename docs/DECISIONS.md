# Architekturentscheidungen

Dieses Protokoll hält vereinbarte Entscheidungen fest. Neue Entscheidungen mit
Datum, Anlass, Begründung und Konsequenzen ergänzen. Überholte Entscheidungen
ausdrücklich als ersetzt markieren.

## 2026-10-17 – Dokumentation und Planung im Repository

**Entscheidung:** Entwicklung und Planung erfolgen in Codex. `docs/` ist die
dauerhafte Quelle für Spezifikation, Anforderungen, Architektur und Arbeitsstand.
`AGENTS.md` verweist neue Sitzungen auf diese Dokumente.

**Begründung:** Neue Codex-Sitzungen sollen ohne Zugriff auf ältere Chats
weiterarbeiten können.

**Konsequenzen:** Anforderungen in `SPECIFICATION.md`, Systemaufbau in
`ARCHITECTURE.md`, begründete Entscheidungen hier und offene Arbeit in
`NEXT_STEPS.md` bei relevanten Änderungen gemeinsam aktualisieren.

## 2026-10-17 – Bestehender Stack und gemeinsame Geschäftslogik

**Entscheidung:** Rust/Axum stellt die HTTP-API bereit, SQLx verwaltet SQLite.
React/TypeScript/Vite liefert dieselbe Oberfläche für Browser und Tauri 2.
GTD-Verfügbarkeit und Sequential-Regeln werden ausschließlich im Backend berechnet.

**Begründung:** Web, Desktop und ein späterer MCP-Adapter sollen dieselben Regeln
verwenden. Desktop-Zielplattformen sind macOS, Windows und Linux.

**Konsequenzen:** Keine direkte Datenbankanbindung aus dem Frontend und keine
zweite GTD-Implementierung im Client. Jede Zielplattform separat bauen und testen.

## 2026-10-17 – Zuerst das Gerüst verifizieren

**Entscheidung:** Zunächst Installation, Builds, SQLite, API und lokalen Start
prüfen. Größere Änderungen an der Benutzeroberfläche folgen erst nach Auftrag.
Der Backend-Dienst wird in dieser Etappe separat von Tauri gestartet.

**Begründung:** Ein verifiziertes Gerüst ist die Grundlage für die weitere
Produktentwicklung; automatische Serververwaltung ist ein späterer Schritt.

**Konsequenzen:** Aktuell nur Startprobleme und kleine Konfigurationsfehler
beheben. Kein Sidecar, keine Synchronisation und kein Remote-Betrieb in dieser Etappe.
