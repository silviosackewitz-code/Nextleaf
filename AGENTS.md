# Zusammenarbeit im TaskManager-Repository

Dieses Projekt wird vollständig in Codex entwickelt und geplant. Dauerhaftes
Projektwissen gehört ins Repository, nicht nur in den Chat.

- Vor Änderungen `docs/SPECIFICATION.md`, `docs/ARCHITECTURE.md`,
  `docs/DECISIONS.md` und `docs/NEXT_STEPS.md` lesen.
- Neue oder geänderte Anforderungen in `docs/SPECIFICATION.md` festhalten.
- Architekturentscheidungen mit Datum, Begründung und Konsequenzen in
  `docs/DECISIONS.md` dokumentieren; `docs/ARCHITECTURE.md` konsistent halten.
- Arbeitsstand, offene Probleme und nächste Schritte in `docs/NEXT_STEPS.md`
  aktualisieren. Geprüftes, Geplantes und noch Ungeprüftes klar unterscheiden.
- Vorgesehener Stack: Rust, Axum, SQLx/SQLite, React, TypeScript, Vite, Tauri 2.
  Zielplattformen: macOS, Windows und Linux. GTD-Regeln liegen im Backend.
- Aktuelle Etappe: vorhandenes Gerüst zum Laufen bringen und prüfen.
  Größere UI-Änderungen erst nach entsprechendem Auftrag.
- Kleine, vollständige Änderungen bevorzugen. Keine produktiven Daten oder
  Secrets einchecken. Plattformtests nur als bestanden melden, wenn ausgeführt.
