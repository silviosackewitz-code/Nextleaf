# Architekturentscheidungen

Dieses Protokoll hält vereinbarte Entscheidungen fest. Neue Entscheidungen mit
Datum, Anlass, Begründung und Konsequenzen ergänzen. Überholte Entscheidungen
ausdrücklich als ersetzt markieren.

## 2026-10-17 – Nextleaf nach Recherche beibehalten

**Entscheidung:** Der Nutzer bleibt nach Kenntnis der Rechercheergebnisse bei
**Nextleaf**. Eine eigene Domain ist vorerst nicht vorgesehen. Falls später
eine Domain benötigt wird, darf deren Name vom Produktnamen abweichen.
Donebrook und Readyfern werden nicht übernommen; die Namenssuche ist beendet.

**Konsequenzen:** App, Pakete und vorbereitete Deployment-Pfade bleiben bei
Nextleaf. Keine weitere Umbenennung oder Domainregistrierung. Die bisherigen
Namensüberschneidungen und Prüfgrenzen bleiben in [NAMING.md](NAMING.md)
dokumentiert; die Produktentscheidung ist keine rechtliche Markenfreigabe.

## 2026-10-17 – Namensentscheidung wieder geöffnet (ersetzt)

Ersetzt durch „Nextleaf nach Recherche beibehalten“ oben.

**Entscheidung:** Nach den Recherchetreffern zu Nextleaf und Clear Trail sucht
der Nutzer neue, vorab geprüfte Namen. Die frühere Entscheidung für Nextleaf
gilt damit nicht mehr als endgültiger Veröffentlichungsname.

**Konsequenzen:** Vor weiteren Umbenennungen Kandidaten prüfen und dem Nutzer
zur Auswahl vorlegen. Aktuell Donebrook und Readyfern; Befunde, Quellen und
Prüfgrenzen in [NAMING.md](NAMING.md). Bis zur Auswahl bleiben Code und
Deployment-Pfade bei Nextleaf. Keine Domain oder GitHub-Organisation reserviert.

## 2026-10-17 – Projektname Nextleaf

**Entscheidung:** Die Anwendung heißt **Nextleaf**, mit „leaf“ (englisch für
„Blatt“). Die zuvor übernommene Schreibweise „Nextlief“ wurde auf Hinweis des
Nutzers korrigiert. Eine Open-Source-Veröffentlichung
ist beabsichtigt; die Lizenz ist noch auszuwählen.

**Konsequenzen:** Produktname, Fenster- und Seitentitel sowie npm-/Cargo-Pakete
tragen den neuen Namen. Der bestehende Checkout-Ordner bleibt `TaskManager`.
Bereits verwendete Speicherkennungen bleiben kompatibel: `taskmanager.db`,
`TASKMANAGER_*`, `taskmanager_api_url` und Tauri-ID `dev.taskmanager.desktop`.
So zeigt die Umbenennung nicht auf eine neue leere Datenbank oder neue lokale
Einstellungen. Der noch nicht eingerichtete Pi verwendet `/etc/nextleaf` und
`/var/lib/nextleaf`; dies ersetzt die zuvor geplanten Pfade mit `taskmanager`.
Die anschließende [Namensprüfung](NAMING.md) ergab bestehende gleichnamige
Softwareprojekte, eine belegte GitHub-Organisation und registrierte `.com`- und
`.org`-Domains. Markenhinweise bestehen ebenfalls; rechtliche Freigabe offen.
Vor öffentlicher Veröffentlichung den Namen deshalb erneut bewerten.

## 2026-10-17 – Serverdaten in einem festen Host-Verzeichnis

**Entscheidung:** Im geplanten Docker-Betrieb liegen die persistenten Daten in
einem festen Ordner auf dem Raspberry Pi und werden per Bind-Mount eingebunden.
Zuordnung nach Recherche der Linux-Standards: `/var/lib/nextleaf` auf dem
Host nach `/data` im Container. Compose und lokale Einstellungen liegen unter
`/etc/nextleaf`. Der ursprüngliche Vorschlag `/srv/taskmanager/data` ist
damit ersetzt.

**Begründung:** Der Nutzer möchte App und Daten getrennt halten und den
Datenordner unmittelbar in die Serversicherung aufnehmen können. Für die
interne Datenbank verwenden wir die FHS-Trennung von Anwendungszustand und
Host-Konfiguration. Recherche und App-Vergleich stehen in `ARCHITECTURE.md`.

**Konsequenzen:** Keine Daten im Image oder in der beschreibbaren Container-Schicht.
Das gesamte SQLite-Verzeichnis einbinden; Berechtigungen passend zum App-Prozess
setzen. Container-Neuerstellung darf Daten nicht entfernen. Für Backups die
SQLite Online Backup API verwenden oder die App für eine konsistente
Dateisicherung stoppen. Auch Compose-Konfiguration, lokale Einstellungen und
App-Version sichern. Das Container-Image wird erneut bezogen oder aus
gesichertem Quellcode gebaut. Wiederherstellung gesondert testen.

Status: Dockerfile, `.dockerignore`, Compose und Konfigurationsbeispiel erstellt.
Compose validiert; Image-Build und Betrieb auf dem Pi noch ungeprüft.
Die App läuft im Container als UID/GID `10001:10001`, mit schreibgeschütztem
Root-Dateisystem und beschreibbarem Daten-Bind-Mount. Das Image wird zunächst
auf dem Pi aus dem Repository gebaut und mit einem expliziten lokalen Tag
gestartet; eine Registry-Veröffentlichung ist nicht eingerichtet.
Einrichtung und Prüfablauf: [DEPLOYMENT.md](DEPLOYMENT.md).
Referenzen: [Docker Bind-Mounts](https://docs.docker.com/engine/storage/bind-mounts/),
[SQLite Backup API](https://sqlite.org/backup.html).

## 2026-10-17 – Einheitlich englische Benutzeroberfläche

**Entscheidung:** Alle von der Anwendung bereitgestellten UI-Texte sind auf
Englisch, einschließlich Fehlermeldungen und barrierefreier Beschriftungen.
Das HTML-Dokument verwendet `lang="en"`.

**Begründung:** Der Nutzer wünscht eine einheitliche englische Oberfläche
anstelle der bisherigen Mischung aus Deutsch und Englisch.

**Konsequenzen:** Neue UI-Texte ebenfalls auf Englisch schreiben. Keine
Sprachauswahl oder Übersetzungsbibliothek erforderlich. Benutzerinhalte werden
nicht übersetzt; die deutschsprachige Projektdokumentation bleibt bestehen.

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
