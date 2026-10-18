# Nirvana-Testimport

Stand: 2026-10-17. Der Nutzer hat alle gelieferten Inhalte für den Testbetrieb
angefordert und ausdrücklich **Parallel für sämtliche Projekte** festgelegt.

## Ergebnis

- 128 MCP-Objekte übernommen: 117 Aufgaben und 11 Projekte, davon 3 abgeschlossene
  Projekte. Zusätzlich 3 Areas und 12 Kontext-Tags, einschließlich unbenutzter Tags.
- Aufgaben im Snapshot: 21 Next, 50 Later, 4 Waiting, 1 Scheduled, 1
  Wiederholungsvorlage, 8 Someday, 17 erledigt und 15 Trash. Eine zuvor vorhandene
  lokale Aufgabe wurde unverändert erhalten. Anzahlen ändern sich beim Testen.
- Alle gelieferten Titel, Notizen, Projektbeziehungen, Area-/Kontext-Zuordnungen,
  Focus-Markierungen, Start-/Fälligkeitsdaten, Abschlussdaten, Waiting-Kontakte
  und Zeit-/Energieangaben gespeichert. Notizen und Metadaten sind sichtbar.
- Erledigte Aufgaben liegen im Logbook, zuletzt erledigte zuerst. Trash erscheint
  nur in seiner eigenen Ansicht; Vorlagen erzeugen keine Next-Actions.
- Keine Nirvana-Inhalte verändert. Kein laufender Hintergrundabgleich.

## Quelle und Grenzen

Offizielle Verbindung: `https://mcp.nirvanahq.com/mcp`, in der lokalen
Codex-Konfiguration als `nirvana`. OAuth und Lesezugriff erfolgreich. Nach dem
App-Neustart sind die Werkzeuge verfügbar, obwohl der Nutzer den Eintrag in der
MCP-Oberfläche nicht findet; Anzeigeursache nicht untersucht.

`get_tasks` mit allen dokumentierten Zuständen, `include_notes=true`, `limit=200`
lieferte `total=128`, 128 Objekte und `has_more=false`. Logbook (19 Objekte) und
Trash (15) separat gelesen und IDs gegen den Gesamtabruf verglichen. Alle
Elternprojekte vorhanden. `get_tags` lieferte Areas und Kontext-Tags.
Quelle: [Nirvana MCP](https://www.nirvanahq.com/help/ai/mcp-setup).

MCP liefert nicht den gesamten Originaldatensatz eines vollständigen Exports:

- Eine Wiederholungsvorlage kommt als `state=recurring`, jedoch ohne Rhythmus,
  Pause oder Endregel. Sie bleibt unter Scheduled sichtbar, mit englischem
  Hinweis auf die fehlende Regel. **Keine automatischen Wiederholungen.**
- Ursprüngliche Projektmodi fehlen. Bewusst auf Nutzerwunsch durch Parallel ersetzt.
- Manuelle Sortierreihenfolge fehlt; vorläufig wird die gelieferte Reihenfolge
  verwendet. Erstellungs-/Abschlussdaten haben nur Tagesgenauigkeit und bleiben
  als solche erhalten. `updated_at` bezeichnet den lokalen Importzeitpunkt;
  `last_modified_days` bleibt im Originalobjekt, wird nicht in einen vermeintlich
  exakten Zeitpunkt umgerechnet.
- Kein vollständiger Kontoexport zum unabhängigen Abgleich vorhanden. Der Import
  ist vollständig für die 128 gelieferten Objekte, nicht als verlustfreie Kopie
  sämtlicher nicht per MCP gelieferter Kontometadaten bestätigt.

Falls später benötigt: [offizieller JSON-Export](https://www.nirvanahq.com/help/how-to/export-data)
über das [Kontodashboard](https://account.nirvanahq.com/dashboard). Der bisherige
Browserzugriff führt zur Anmeldung. Kein Exportparser auf Basis eines vermuteten
Formats implementiert.

## Sicherung und einmaliger Import

Private Quelle: `data/imports/nirvana-mcp-2026-10-17.json`.
Konsistente SQLite-Backups unter `data/backups/`, insbesondere
`before-nirvana-live-2026-10-17.db` unmittelbar vor dem Live-Import.
Diese Pfade sind von Git ausgeschlossen. Keine persönlichen Inhalte oder
Zugangsdaten in Dokumentation oder Test-Fixtures aufnehmen.

Rust-Wartungsprogramm `backend/src/bin/import_nirvana.rs`, keine Importoberfläche
und kein neuer Netzwerk-Endpunkt. Aufruf aus dem Repository nach konsistenter
Sicherung, zunächst mit einer Testkopie der Datenbank:

```bash
cargo run --locked --manifest-path backend/Cargo.toml --bin import_nirvana -- \
  data/imports/nirvana-mcp-2026-10-17.json data/backups/nirvana-test-final-2026-10-17.db
```

Nur das eigene Snapshotformat `nirvana-mcp-v1` wird unterstützt. Es enthält
`total`, `has_more`, `tags` und `items` aus dem MCP-Abruf. Das ist nicht der
ungeprüfte offizielle JSON-Export. Fehlende Eltern, doppelte IDs, unbekannte
Typen/Statuswerte, mehrdeutige Areas und ungültige Daten stoppen den Import.
Referenzlisten sind im gelieferten Bestand nicht vorhanden; unbekannte Typen
werden abgelehnt statt stillschweigend verworfen.

Die Migration erweitert die vorhandenen Tabellen und erhält bisherige Aufgaben
und Tag-Verknüpfungen. Import innerhalb einer Transaktion; bei Fehler Rollback.
`nirvana_imports` speichert unveränderte Quellobjekte und ihre lokalen UUIDs.
Erneuter Import derselben Objekte überspringt sie; geänderte Quelle wird
abgelehnt, lokale Bearbeitungen werden nicht überschrieben. Bei einer späteren
Ergänzung fehlender Felder zuerst ein gezieltes Mapping festlegen.

## Prüfung

- Probelauf auf konsistenter SQLite-Kopie: alle 128 Objekte, Titel/Notizen,
  Datums-/Metadatenfelder, Elternbeziehungen, Tags und Originalobjekte verglichen.
  Vorhandene lokale Zeilen erhalten. Wiederholung: 0 neu, 128 übersprungen.
- Rust-Tests: Wiederholung ohne Duplikate/Überschreiben, ungültige Eltern und
  geänderte Quelle, Rollback bei SQL-Fehler, Migration vorhandener Tag-Links,
  bestehende Sequential-Regeln, Trash/Vorlagen-Ausschluss und Logbook-Sortierung.
- Live-Import nach erneuter Sicherung erfolgreich. SQLite-Integrität und
  Fremdschlüssel geprüft. API-Listen gegen erwartete Mengen abgeglichen;
  Notizen/Tags ausgeliefert. Browser zeigt importierte Inhalte.
- Abschlussablauf mit synthetischer Aufgabe in separater Testdatenbank im
  Browser geprüft: Rückmeldung, View Logbook, Undo und Rückkehr nach Next.
- TypeScript/Vite und macOS-Debug-App-Build erfolgreich; eigene native
  Sichtprüfung sowie Windows/Linux/Docker bleiben offen.

Nächste Funktionsetappe gemeinsam festlegen: Titel/Notizen/Zuordnung bearbeiten,
Projekt-/Area-Bearbeitung und Reihenfolge, danach weiterer GTD-Ausbau. Keine
umfassende UI-Umgestaltung aus dem Testimport ableiten.
