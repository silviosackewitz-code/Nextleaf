# Session-Übergabe — 2026-10-18

## Mobile Webansicht und Installationsweg — 2026-10-18

Beauftragt: neue Funktionen auf iPhone-Bedienbarkeit prüfen; danach möchte der
Nutzer selbst Pi, Mac und iPhone vollständig von Nextleaf-Testständen bereinigen
und den endgültigen Installationsweg mit leeren Daten testen. Noch nichts
zurückgesetzt. Installationsanleitung im Repository ausdrücklich gewünscht:
[INSTALLATION.md](INSTALLATION.md). Pi: Downloads plus fertiges Docker-Image und
Compose; Mac: native App/DMG; iPhone: private HTTPS-Web-App/Home-Bildschirm.

Geprüft im Codex-Browser, nicht auf einem echten iPhone: laufender pi7 bei
390×844 und 320×568. Person/Area/Tag anlegen, Person umbenennen/archivieren/
wiederherstellen, Kontaktauswahl im Schnellmenü nach geänderter Texteingabe,
Waiting-Freitext speichern, Tag-Vorschlag antippen, archivierte Zuordnungen
weiter anzeigen und archivierte Tags nicht neu vorschlagen. Abbrechen fragt nach
ungespeicherten Änderungen. Eigene Testaufgabe anschließend in Trash; eigene
Testperson, Testtag und Testarea archiviert. Kein vollständiger Reset.

Kleine Korrekturen im Quellstand: mobile Entfernen-Schaltflächen für Tags/Area
und Tag-Eingabe mindestens 44 px hoch, Entfernen/Edit mindestens 44 px breit;
Verwaltungs-Checkbox 20 px; Personen-/Typ-/Filterfelder 16 px Schrift.
Web-Verbindungshinweis beschreibt jetzt Online-Web statt Desktop.
Korrigierter Build zusätzlich mit separater lokaler DB bei 320×568, 390×844,
390×360 und Desktop 1280×800 geprüft. Keine seitlich überlaufenden geprüften
Dialoge; lange Namen umbrechen, Kopfaktionen bei geringer Höhe sichtbar.
DOM-Messungen bestätigen 44×44-Pixel-Aktionsflächen und 16-Pixel-Formularschrift.
Entwurferhalt über Keep editing und Persistenz von Tag-/Kontaktänderungen geprüft.
39 Frontend-Tests, TS/Vite-Build und git diff --check bestanden.

Diese Mobilkorrekturen sind noch nicht im laufenden pi7 oder im bereits gebauten
Mac-Paket enthalten; sie gehören in das nächste Release. Kein erneutes
Serverupdate in dieser Prüfrunde. Echte iPhone-Tastatur, Safari, Home-Bildschirm-
Start und Gerätewechsel bleiben die explizite Abnahme nach INSTALLATION.md.

Release-Vorlagen unter `deploy/release/`: Image verpflichtend, Pull erlaubt,
Datenpfad über ENV, sonst Schutz-/Loopback-Einstellungen wie bestehende Compose.
Auf dem Pi mit Compose nur validiert: Standard-/alternativer Datenpfad und
fehlendes Image; keine Containeränderung. Kein Image-Pull/Erstinstallationstest.
README von überholten Gerüst-/Sidecar-Angaben bereinigt.

GitHub-Repository auf ausdrückliche Bestätigung zunächst privat angelegt:
https://github.com/silviosackewitz-code/Nextleaf (Remote origin).
Der vollständige bisher lokal entwickelte Quellstand samt Anleitung wird in
Git übernommen; Datenbanken, lokale ENV-Dateien, Pakete/Builds und Python-Caches
sind ausgeschlossen. Noch keine Releases oder Registry-Images veröffentlicht.
Nächster Schritt: fertige Images und Mac-Installer bereitstellen und den
Downloadzugang klären (privates GitHub benötigt Anmeldung), danach Neuinstallation
gemeinsam testen. Signierung/Notarisierung und Open-Source-Lizenz bleiben offen.

## pi7 und Mac-App geprüft — 2026-10-18

Der Nutzer hat den Pi-Updater ausgeführt und `NEXTLEAF_UPDATE_OK` bestätigt.
Anschließend unabhängig geprüft: Docker/Tailscale aktiv, lokale und private
HTTPS-Health erfolgreich mit `tag-management` und `sync-v2`, Web-Dateien passend
zum unveränderten pi7-Paket, anonymer MCP-Zugriff korrekt mit HTTP 401 abgewiesen.
Root-Update-Log, Backup-Inhalte und authentifizierte MCP-Prüfung wurden danach
mangels sudo-Zugang nicht erneut gelesen; dafür liegt die Erfolgsmeldung des
Updaters vom Nutzer vor. Das vorbereitete pi7-Paket bleibt unverändert.

Mac-Debug-Bundle neu gebaut und mit der bestehenden lokalen Datenbank gestartet.
UI zeigt `Synced` / `Changes are synced.`; vollständiger Vergleich mit dem Pi
bestätigt identische Inhalte einschließlich Beziehungen und bestätigter Baseline,
Protokoll 2, keine Konflikte oder ausstehenden Änderungen, SQLite-Integrität/FKs ok.
Bestand: 120 Aufgaben, 11 Projekte, 3 Areas, 13 Tags einschließlich Archiv/Trash.
Die ursprünglich vorhandenen 119 Aufgaben, 11 Projekte, 3 Areas und 12 Tags sind
gegenüber der Momentaufnahme vor den Mac-Tests unverändert.

Beim ersten nativen Test übernahm die separate Testdatenbank eine alte Server-URL
aus dem WebView-Speicher und synchronisierte eine zusätzliche Testaufgabe und
Testperson. Nur diese Aufgabe wurde in Trash verschoben, nur diese Person
archiviert. Ursache in `frontend/src/main.tsx` behoben: automatischer Startabgleich
verwendet ausschließlich die in der jeweiligen Datenbank gespeicherte Verbindung.
Eine neue Datenbank erfordert explizites Connect. Anschließender nativer Test mit
frischer Datenbank blieb lokal trotz alter WebView-URL; Bearbeiten und Persistenz
funktionierten. Der Nutzer hat klargestellt: sämtliche Aufgaben sind Testdaten,
die Originale liegen weiterhin in Nirvana.

Weitere Nachweise: Migration einer Kopie der alten Mac-DB von v1 auf v2 ohne
Verlust bisheriger Tabellen/Beziehungen; lokaler stdio-MCP mit 13 Tools,
Person-Anlage/Umbenennen/Archivieren/Wiederherstellen und erhaltener Task-Verknüpfung;
39 Frontend-Tests, TS/Vite-Build und macOS-Debug-Build bestanden. Manage Tags
öffnet in der laufenden Mac-App. Sicherung vor Migration:
`data/mac-pi7-20261018/mac-before-pi7.db` (ignoriert, nicht einchecken).

Die laufende App liegt unter `src-tauri/target/debug/bundle/macos/Nextleaf.app`.
Zusätzlich wurde ein ZIP der Test-App im Ausgabeverzeichnis dieses Codex-Chats
bereitgestellt. Kein signierter/notarisierter Release. Noch offen: echter
iPhone-/Safari-Test, Windows/Linux, reale Agenten-Clients sowie Commit/Push.
Review/Perspectives bleiben zurückgestellt. Frühere Abschnitte beschreiben den
jeweiligen historischen Stand; pi7-Ausrollung und Mac-Aktivierung sind erledigt.

## Neuester Stand der Fortsetzung — 2026-10-18

Die unten beschriebene Frontend-Integration ist abgeschlossen. api.ts unterstützt
Tag/Area-Verwaltung einschließlich Typ, Archivstatus, Legacy-Defaults und expected.
Waiting-Kontaktauswahl funktioniert im Editor und Schnellmenü; Freitext bleibt
möglich. Archivierte Zuordnungen bleiben erhalten, neue Auswahl/Area-Vererbung
wird eingeschränkt. Manage Tags lädt nach abgebrochenem Konfliktentwurf neu.

Nachgewiesen: 39 Frontend-Tests, vollständiger TS-/Vite-Build, 28 Backend-Tests,
separater echter HTTP-/Offline-Test, Backend-Clippy all targets und Mac-Debug-Bundle.
Browserprüfung mit isolierter DB auf Desktop und bei 390×844 einschließlich
Kontaktklick nach Texteingabe, Archiv/Restore, Umbenennen und Konflikt-Neuladen.
Keine produktive Datenbank geöffnet/geändert, keine Pi-Ausrollung und kein Start
der neuen Mac-App auf dem produktiven Bestand. Kein echter iPhone-/Safari-Test.

**Hier fortsetzen:** pi7-Serverpaket getrennt vorbereiten und prüfen, danach
Server zuerst aktualisieren. Nutzer hat diese Reihenfolge im neuen Chat bestätigt.
Alte Mac-App vor Wechsel beenden und konsistente Sicherungen erstellen; neue
Mac-App erst nach erfolgreichem Pi-Update starten. Vorhandenes pi6-Paket ist kein
pi7-Paket. Native Laufzeitprüfung dieser Runde, Pi-Paket/Update sowie Commit/Push
sind weiterhin offen. Details und bisherige Produktivmesswerte siehe NEXT_STEPS.md.
Die folgenden Abschnitte beschreiben die vorherige Übergabe und sind beim
Integrationsstatus durch diesen Nachtrag ersetzt.

## Historischer Auftrag und Unterbrechung

Der Nutzer möchte wegen langsamer Antworten eine neue Codex-Session öffnen und
hat zuletzt ausdrücklich nur die sofortige Dokumentation beauftragt. Keine
weitere Implementierung/Ausrollung in diesem Abschluss. Vorheriger Auftrag bleibt:
alle besprochenen Funktionen fertigstellen, prüfen, danach Commit und Push.
**Commit und Push sind noch offen.** Kein Zustand als fertig ausgeben, solange
Integration und vollständiger Build fehlen. Kurze Rückfragen sofort beantworten;
regelmäßige knappe Fortschrittsmeldungen, keine langen stillen Wartephasen.

Repository: `/Users/silviosackewitz/Developer/repo/TaskManager`, Branch `main`,
letzter Commit beim Abschluss `54c23be` (Nextleaf-Umbenennung und Docker-Betrieb
vorbereiten). Viele geänderte und unversionierte Quelldateien gehören zur bisherigen
Arbeit. Nicht zurücksetzen, nicht nur getrackte Dateien übernehmen. Agentenergebnisse
sind im gemeinsamen Arbeitsbaum gespeichert; Agenten nicht erneut starten müssen.

## Zuerst lesen

AGENTS.md, SPECIFICATION.md, ARCHITECTURE.md, DECISIONS.md, NEXT_STEPS.md.
Diese Übergabe konkretisiert die nächsten Schritte. Historische Abschnitte der
Dokumente sind keine Behauptung über den aktuellen Ausrollungsstand.
Alle UI-Texte Englisch; Erläuterungen für den Nutzer Deutsch. Linux-Befehle jeweils
kurz erklären. Produktive Daten nicht für Schreibtests verwenden.

## Historischer Integrationsauftrag — am 2026-10-18 abgeschlossen

1. `frontend/src/api.ts`: Tag `{id,name,kind:'tag'|'person',archived:boolean}`,
   Area `{id,name,sort_order,archived:boolean}` ergänzen. Alte Antworten mit
   `kind ?? 'tag'` und `archived ?? false` normalisieren.
2. `addTag(name, kind?)`, `updateTag(id, changes, expected?)` und
   `updateArea(id, changes, expected?)` ergänzen. PATCH nach `/tags/:id` bzw.
   `/areas/:id`, expected als Objekt im Body. Health-Fähigkeit `tag-management`
   vor neuen Verwaltungs-/People-Operationen verlangen; ältere Server dürfen
   unbekannte Felder nicht still ignorieren. Gewöhnliches addTag mit einem
   Argument kann die vorhandene item-creation-Kompatibilität behalten.
3. `ManageTags.tsx` ruft diese Methoden bereits auf (inklusive expected).
   `App.tsx` liest bereits kind/archived und übergibt `people` an TaskDetails und
   TaskActions. Diese Props fehlen noch: in TaskDetails/TaskActions/TaskFields
   integrieren und bestehende Kontakte als Auswahl für Waiting for anbieten,
   vorhandenen Freitext erhalten. Auswahl nicht durch Blur-Speicherung überholen.
4. Archivierte Areas/Tags nicht neu anbieten, bereits zugewiesene Werte aber
   weiter anzeigen/erhalten. TagEditor/TaskFields/ProjectDetails und neue
   Einträge darauf prüfen. ManageTags-Fehler-/Konfliktverhalten prüfen.
5. API-Kompatibilität/Payloads sinnvoll testen; vollständigen TypeScript-Build
   ausführen. Dann gemeinsame UI mit isolierter Datenbank prüfen, Mac bauen.

REST-Vertrag: GET tags/areas enthält auch archivierte Einträge. Tag-PATCH
name/kind/archived, Area-PATCH name/archived, jeweils optional expected derselben
Felder. Bestehende Zuordnungen bleiben bei Archivierung erhalten; neue
Zuordnungen zu Archiviertem werden atomar abgelehnt. Keine automatische
Umklassifizierung existierender Namen zu People.

## Bereits umgesetzt / Agenten-Prüfergebnisse

Backend: `backend/src/lib.rs`, `sync.rs`, `mcp.rs`, Migration
`20261018010000_tag_management.sql`, `deploy/check_mcp.py`, `docs/MCP.md`.
13 MCP-Tools, neue update_tag/update_area, conditional updates, Sync v2 mit
stabilen Tag-IDs und v1-Clientzustandsmigration. 28 reguläre Tests (26 Library,
2 Importer) und separater echter HTTP-/Offline-Test bestanden; Clippy all targets,
Rust-Formatprüfung und Python-Syntaxprüfung bestanden. Kein Produktivbestand geändert.

Frontend: App.tsx, taskList.ts, viewOptions.ts, styles.css, ManageTags.tsx/.css
und Tests. Kontextfilter/Waiting-Kontakte, drei Notizmodi, Untagged,
View Options nur Next/Focus, Next Project actions Top 1–9/All, Standard Top 3.
Hover-Untermenü öffnet links, mobil per Tippen; ungespeicherte Entwürfe geschützt.
33 Frontend-Tests bestanden. Isolierte Desktop- und 390×844-Browserprüfung mit
Testdaten: Limits/Show more, volle Notizen, Waiting ohne View Options, Hover/Klick,
mobiles Menü innerhalb Viewport. Vollständiger TS-Build noch blockiert durch
oben genannte Integration; Vite-Vorschau war ohne TypeScript-Prüfung gebaut.
Testserver und Vorschautab sind beendet. Temporäre Screenshots/Server nicht nötig.

Prüfbefehle nach Integration (vom Repository aus):
- `node --experimental-strip-types --test frontend/tests/*.test.ts`
- `npm run web:build`
- `cargo test --manifest-path backend/Cargo.toml`
- `cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings`
- `npm run desktop:build -- --debug --bundles app`

Neue Mac-App für Laufzeittests nur mit separatem `NEXTLEAF_DESKTOP_DATA_DIR`
starten. Passende Tests nicht unnötig wiederholen; Prüfstände nach Integration
klar vom vorherigen Agententest unterscheiden.

## Produktiver Stand und Update-Reihenfolge

Letzter unabhängig geprüfter Stand: **pi6**, `nextleaf:0.1.0-pi6`.
Pi: `dersack@100.65.99.74`, privates HTTPS `https://dersack.tail1d53da.ts.net`.
Docker Host-Port nur Loopback8787; Tailscale Serve HTTPS443 leitet dorthin weiter.
Eine adaptive Web-App für Telefon und Computer; Tailscale wählt keine Geräte-UI.
Pi-DB `/var/lib/nextleaf/taskmanager.db`, Konfiguration `/etc/nextleaf/.env`
und `/etc/nextleaf/compose.yaml`. Keine Secrets ins Repository.
Mac-DB `~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
Mac-Bundle `src-tauri/target/debug/bundle/macos/Nextleaf.app`.

pi6-Prüfung: Health/HTTPS erfolgreich, Mac Synced, 119 Aufgaben/11 Projekte/
3 Areas/12 Tags in vollständigen Snapshots identisch; keine Konflikte oder
Pending-Änderungen, SQLite-Integrität/Fremdschlüssel ok; anonymes MCP401.
Das sind damalige Messwerte, keine neue Live-Prüfung beim Sessionabschluss.
Nutzer bestätigt danach: bisherige iPhone-App funktioniert. Eigene vollständige
Safari-/Tastatur-/Gestenprüfung ist damit nicht nachgewiesen.

**Neue Tags-/Ansichtenrunde noch nicht ausgerollt.** Erst alten Mac-Prozess
beenden und Sicherungen erstellen, dann Pi aktualisieren, danach neues
Mac-Bundle mit produktiver DB öffnen. Sync v2 lehnt v1 ab; neue Desktop-App
migriert lokalen Synczustand. Kein ungeprüftes Downgrade nach Migration.

Vorheriges ignoriertes Paket liegt unter `data/update-offline-20261018/` mit
source/update.sh/verify.py/SHA256SUMS, Archiv
`data/nextleaf-update-offline-20261018.tar.gz`. Neues Paket getrennt vorbereiten,
z. B. update-tags-20261018 / pi7; nicht alten Paketstand als neue Version ausgeben.
Nur Quell-/Builddateien, keine DB/Secrets/node_modules/targets übernehmen.
Build zuerst, konsistente Sicherung vor Wechsel, Konfiguration/Secrets erhalten,
Health um tag-management und MCP-Prüfung um13 Tools erweitern. Vorhandenes
verify.py prüfte Integrität, keinen unabhängigen Vorher/Nachher-Inhaltsvergleich.
SSH funktioniert, sudo-Update benötigt Nutzereingabe. Paket verifizieren,
dann ausführbaren Nutzerbefehl mit Erklärung liefern. Noch kein neues Paket erstellt.

## Abschluss der nächsten Session

- Vollständigen Integrations-/Build-/UI-Prüfstand dokumentieren, verbleibende
  Grenzen ehrlich ausweisen; aktuelle Dokumente konsistent halten.
- Daten/Secrets/Buildartefakte und `deploy/__pycache__/` nicht einchecken.
  Dirty-Stand umfasst mehrere vorangegangene Funktionsrunden; Änderungen vor
  Commit gemeinsam sichten. Remote/Branch vor dem autorisierten Push prüfen.
- Commit und Push sind ausdrücklich beauftragt, aktuell noch nicht ausgeführt.
- Review und Perspectives ausdrücklich später. Review-Recherche ist in
  REVIEW_RESEARCH.md, iPhone-Referenz in UX_REFERENCE.md.
- Echte ChatGPT-/Claude-Verbindung und Sprachtest noch offen; lokales Mac-stdio-MCP
  bedeutet keine automatische lokale MCP-Anbindung einer iPhone-Web-App.
- Signierte Releasepakete, Windows/Linux, veröffentlichte Compose/ENV-Downloads
  und vorgebaute Images sowie Backup-Restore bleiben gesonderte offene Arbeit.
