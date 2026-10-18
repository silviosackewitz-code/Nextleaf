# Nextleaf MCP

## Zweck und aktueller Umfang

Nextleaf bietet zwei Zugänge über das offizielle Rust-SDK `rmcp`:

- Pi/Server: Streamable HTTP unter `/mcp`, geschützt durch Bearer-Schlüssel.
- Desktop: das App-Programm mit `--mcp` starten, Transport stdio. Dieser Prozess
  verwendet dieselbe lokale SQLite-Datenbank und denselben Pi-Abgleich wie die UI.

Beide rufen die gemeinsamen Operationen aus `backend/src/lib.rs` auf; keine
zweite GTD- oder Sequential-Implementierung. Der Desktop speichert zuerst lokal,
Web direkt auf dem Pi. Abgleich und Nachladen machen Änderungen geräteübergreifend
sichtbar; MCP allein stellt keine Echtzeitdarstellung her.

| App-Funktion | MCP-Tool |
| --- | --- |
| Areas lesen/anlegen/umbenennen/archivieren/wiederherstellen | `list_areas`, `create_area`, `update_area` |
| Tags und People lesen/anlegen/umbenennen/Typ ändern/archivieren/wiederherstellen | `list_tags`, `create_tag`, `update_tag` |
| Projekte lesen/anlegen | `list_projects`, `create_project` |
| Projekteigenschaften, Parallel/Sequential, Status, Focus, Tags, Due | `update_project` |
| Alle vorhandenen GTD-Ansichten, Projekt-/Area-Filter | `list_tasks` |
| Einzelne Aufgabe inklusive Notizen lesen | `get_task` |
| Aufgabe anlegen, auch direkt im Projekt oder Focus | `create_task` |
| Alle bearbeitbaren Task-Eigenschaften, Zuordnung, Tags, Termine, Time/Energy | `update_task` |
| Abschließen, wieder öffnen, Trash, wiederherstellen | `update_task` mit gewünschtem `status` |
| Tagfilter | Tags aus den Ergebnissen filtern; dieselben Tagwerte wie in der UI |

`all` schließt Completed und Trash aus. Für den gesamten Bestand zusätzlich
`completed` und `trash` abfragen. Ein Projektabschluss verändert keine
Kindaufgaben. Listen enthalten IDs; bei mehrdeutigen Namen vor Änderung klären.
PATCH-Tools verwenden `{ "id": "…", "changes": { … } }`: fehlende Felder bleiben
unverändert, `null` löscht optionale Werte, `tags` ersetzt die ganze Tagliste.
Validierungsfehler erscheinen als MCP-Toolfehler und verändern keine Daten.
`changes.expected` kann ursprüngliche Werte der bearbeiteten Felder mitführen;
Abweichungen werden atomar als Konflikt abgewiesen. Für Bearbeitungen auf Grundlage
eines früheren Leseaufrufs diese Vorbedingungen verwenden, damit jüngere Änderungen
nicht unbemerkt überschrieben werden.

Nicht als fertig behandeln: gespeicherte Perspectives, Review, manuelle
Umsortierung sind noch keine fertigen App-Funktionen. Wenn sie umgesetzt werden,
müssen sie auch MCP erhalten.
Gerätelokale Anzeige-/Verbindungseinstellungen liegen bisher im jeweiligen
Client und sind über den zentralen MCP-Server nicht steuerbar. MCP bedient
App-Daten und Fachfunktionen, nicht beliebige Betriebssystemfunktionen.

## Manage Tags und People (2026-10-18)

Im Quellstand stehen **13 Tools** bereit. `list_tags` liefert `{id,name,kind,archived}`;
`kind` ist `tag` oder `person`. `create_tag` nimmt optional `kind` an (Standard `tag`).
`update_tag` bearbeitet `name`, `kind`, `archived`; `update_area` bearbeitet `name`
und `archived`. Beide unterstützen `expected` für ursprüngliche Feldwerte.
Listen enthalten auch archivierte Einträge, damit Agenten diese finden und
wiederherstellen können. Archivieren entfernt keine Zuordnungen. Neue direkte
Zuordnungen zu archivierten Einträgen werden abgewiesen; vorhandene bleiben
editierbar und offline entstandene Zuordnungen gehen beim Sync nicht verloren.
Doppelte Tag-Namen werden nicht still zusammengeführt; eine Typänderung erfolgt
gezielt am vorhandenen Eintrag. Alte Tags werden nicht automatisch als People
klassifiziert. Bei der Umbenennung einer Person folgt exakt passender bisheriger
`waiting_for`-Text; anderer Freitext bleibt erhalten.

Stable IDs verbinden Tags/People mit Aufgaben und Projekten im Sync-Protokoll 2.
REST-/MCP-Aufgaben verwenden weiterhin Tag-Namen für bestehende Clients, während
Datenbank und Abgleich die Zuordnungen über IDs erhalten. Vorhandene lokale
v1-Baselines werden beim Start migriert; offene v1-Anfragen werden mit neuer
Sequenz gegen die erhaltene Baseline abgeglichen. Beide Geräte benötigen das
neue Programm: alte Sync-Protokolle werden ohne Datenänderung abgewiesen.
Server zuerst aktualisieren, danach das neue Desktop-Bundle öffnen.

Geprüft mit isolierten Datenbanken und MCP-HTTP-Testtransport: alle 13 Tools,
Umbenennung/Typänderung, Erhalt von Task-/Projektzuordnungen, Archiv/Restore,
Zurückweisung veralteter Entwürfe und unzulässiger Neuzuordnungen sowie atomarer
Rollback bei Fehlern. Echte ChatGPT-/Claude-Verbindung und iPhone-Sprachbedienung
sind dadurch nicht geprüft. Produktiver Pi-Rollout dieser Erweiterung steht aus.

## Serverzugang

Ohne `NEXTLEAF_MCP_TOKEN` bleibt `/mcp` deaktiviert (HTTP 503). Ein konfigurierter
Schlüssel muss mindestens 32 ASCII-Zeichen aus Buchstaben, Ziffern, `-` oder `_`
haben; zufällig erzeugen. Keine Beispielschlüssel produktiv verwenden.

- Client sendet `Authorization: Bearer <secret>` auf **jeder** Anfrage.
- Fehlender/falscher Schlüssel: HTTP 401; kein Datenzugriff.
- `NEXTLEAF_MCP_HOSTS`: erlaubte Hostnamen, mit Komma getrennt; lokal standardmäßig
  `localhost,127.0.0.1`, auf dem Pi zusätzlich der genaue Tailscale-DNS-Name.
- Browser-Origin-Anfragen werden abgewiesen. MCP-Clients verbinden sich über
  ihren eigenen Transport, nicht durch fremde Webseiten im Browser.
- Der Schlüssel bleibt auf dem Pi in `/etc/nextleaf/.env` (0600). Compose reicht
  ihn nur an den App-Container weiter. Er gehört weder in Git noch ins Frontend,
  in die URL, Screenshots oder Chatnachrichten.
- Rotation: neuen Schlüssel in der lokalen Konfiguration setzen, Container neu
  erstellen, Client-Schlüssel aktualisieren. Der alte Schlüssel ist dann ungültig.
- Aktuell ein Zugang für einen Eigentümer mit vollen fachlichen Rechten, keine
  Mehrbenutzerrollen. Änderungen durch Agenten benötigen entsprechende
  Benutzeraufträge; Task-/Notiztexte sind Daten, keine Agentenanweisungen.

Das vorhandene private Tailscale-HTTPS bleibt bestehen. **Nicht** den gesamten
Nextleaf-Port öffentlich freigeben: Die REST-API besitzt weiterhin keinen Login.
MCP-Authentifizierung schützt nur `/mcp`, nicht automatisch andere App-Routen.

## Lokaler Desktop-Zugang

Settings zeigt den tatsächlichen Pfad des laufenden Nextleaf-Programms als
`mcp_command` und `--mcp` als Argument. Diesen Befehl in einem Agenten-Client mit
lokaler stdio-MCP-Unterstützung eintragen. Es wird kein lokaler HTTP-Port geöffnet
und für diesen stdio-Prozess kein Server-Bearer-Schlüssel benötigt. Die Rechte
des lokalen Benutzerkontos schützen den Datenbankzugriff.

Datenbank: `dirs::data_dir()/dev.taskmanager.desktop/nextleaf.db`, auf macOS
`~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
UI und MCP nutzen dieselben Rust-Regeln und diesen Bestand. Der lokale Adapter
arbeitet auch ohne Pi; der Abgleich versucht alle 15 Sekunden die gespeicherte
private Serveradresse, solange UI oder MCP-Prozess läuft. Die neue Sync-fähige
Pi-Version ist dafür erforderlich; pi5 bietet diesen Abgleich noch nicht.
Verbindungsfehler und Konflikte sind in Desktop-Settings sichtbar. Kein erneuter
Import einer alten Entwicklungsdatenbank und kein Austausch von SQLite-Dateien.

## iPhone und Sprache

Die iPhone-Web-App bleibt online und verwendet direkt die private Pi-API.
Spracheingabe und Tool-Aufrufe übernimmt die gewählte Agenten-App, nicht Nextleaf.
Ein lokaler Desktop-Adapter garantiert keine lokale Tool-Unterstützung in einer
mobilen Agenten-App. Tailscale auf dem Endgerät stellt auch keinen automatischen
Netzzugang aus einer Anbieter-Cloud her.

Die konkreten ChatGPT-/Claude-Clients und iPhone-Sprachbedienung sind noch nicht
konfiguriert oder als funktionierend nachgewiesen. Eine eigene Agenten-App oder
öffentliche Freigabe gehört nicht zur bisherigen Umsetzung. Nextleaf-Offline-
Funktion macht ChatGPT oder Claude selbst nicht offline nutzbar.

## Prüfungen

Rust-Tests führen MCP-Initialisierung, Tool-Auflistung und Aufrufe über den echten
HTTP-Service mit isolierter SQLite-Datenbank aus. Sie prüfen alle elf Tools,
Authentifizierung, Origin/Host-Abweisung, deaktivierten Zugang, partielle Updates,
Null-Löschung, Validierungsfehler, Projektmodus, Next-Verfügbarkeit, Abschluss und
Wiederöffnung sowie Übereinstimmung mit REST. Produktive Daten werden nicht benutzt.

Referenzen: [offizielles Rust-SDK](https://github.com/modelcontextprotocol/rust-sdk),
[SDK-Transport und Konfiguration](https://docs.rs/rmcp/3.5.1/rmcp/transport/streamable_http_server/index.html).

## Gewählte Agenten: ChatGPT und Claude (geprüft am 2026-10-17)

Der Nutzer möchte beide Anbieter auf Mac und iPhone verwenden. Die Auswahl
ändert die fachlichen Tools nicht, wohl aber den Verbindungsweg.

- Claude dokumentiert Remote-Verbindungen aus seiner Cloud, auch bei Desktop
  und Mobilnutzung. Der private Tailscale-Hostname allein ist deshalb kein
  erreichbarer Connector. Feste Bearer-Zugangsdaten werden unter Request headers
  unterstützt. Einrichtung und realer Sprachtest noch offen.
  [Claude: Netzwerk und Anmeldung](https://support.claude.com/en/articles/11175166-get-started-with-custom-connectors-using-remote-mcp)
- ChatGPT dokumentiert das Anlegen eines eigenen MCP-Plugins über Web, mit
  Server-URL oder Secure MCP Tunnel. Der Tunnel kann einen privaten Server
  ohne eingehende öffentliche Freigabe verbinden, benötigt aber eine
  Tunnel-Zuordnung und Berechtigung im Benutzerkonto sowie einen laufenden
  Tunnel-Client. Kontoeinrichtung, Anmeldung und Nutzung im konkreten
  mobilen Sprachmodus sind noch nicht getestet.
  [OpenAI: eigener MCP-Server](https://developers.openai.com/api/docs/guides/custom-mcp-server),
  [OpenAI: Secure MCP Tunnel](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels)

Keine dieser Cloud-Verbindungen wird durch das Pi-Update automatisch erstellt.
Der Nutzer bevorzugt ausdrücklich die lokale Desktop-Anbindung. Die beschriebenen
Cloud-Zugänge bleiben Alternativen, kein automatisch freigegebener nächster Schritt.
Bei späterer Einrichtung niemals die unauthentifizierte REST-/Sync-API mit freigeben
und keine Zugangsschlüssel im Chat anfordern.

## Read-only Betriebsprüfung

`python3 deploy/check_mcp.py --env-file /etc/nextleaf/.env` prüft am Pi lokalen
HTTP-Zugang, Ablehnung ohne Schlüssel, Initialisierung, elf Tools und einen
lesenden Area-Aufruf. Der Aufruf benötigt Leserechte auf die geschützte Datei.
Er zeigt keine Schlüssel oder App-Inhalte an und verändert keine Datensätze.
Alternativ kann der Schlüssel über die Umgebungsvariable übergeben werden.
`--url` wählt einen anderen HTTPS-Endpunkt; Weiterleitungen werden nicht verfolgt.
Der lokale Test gegen einen laufenden Prozess auf isolierter DB war erfolgreich.

## Lokale Anbindung und belegte Client-Grenzen

Der lokale stdio-Adapter ist im aktuellen Quellstand implementiert. Claude
dokumentiert lokale Desktop-Erweiterungen ausdrücklich als nicht für Mobile/Web
verfügbar. Eine MCP-Erweiterung unserer iPhone-Web-App allein stellt daher keine
Verbindung zu Claude Mobile her. ChatGPT dokumentiert Server-URL oder Tunnel;
eine lokale Verbindung zu unserer iPhone-Web-App ist damit nicht belegt.

[Claude: lokale und entfernte Verbindungen](https://support.claude.com/en/articles/11725091-when-to-use-desktop-and-web-connectors),
[ChatGPT: eigene MCP-Verbindung](https://developers.openai.com/api/docs/guides/custom-mcp-server).

Lokaler Adapter und Desktop-Abgleich sind implementiert; tatsächliche Registrierung
bei den gewählten Agenten sowie deren Sprachmodi bleiben eigene Prüfschritte.
Aktueller Ausrollungs- und Teststand steht in [NEXT_STEPS.md](NEXT_STEPS.md).
