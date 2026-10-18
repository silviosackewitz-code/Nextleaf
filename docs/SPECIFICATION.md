# Nextleaf – Produktspezifikation (Stand: 18.10.2026)

## Leitidee
Ein eigenständiger GTD-Task-Manager: Bedienung und Einfachheit orientieren sich an Nirvana HQ, ausgewählte gute Konzepte an OmniFocus. Keine 1:1-Übernahme von geschütztem Code oder Gestaltung. Schrittweise entwickeln und nur gewünschte Funktionen ergänzen.

Der Produktname ist **Nextleaf**. Der Nutzer hat den Namen nach Kenntnis der
Rechercheergebnisse erneut bestätigt. Eine eigene Domain ist vorerst nicht
geplant; eine spätere Domain darf anders heißen. Ergebnisse und Grenzen der
[Namensprüfung](NAMING.md) bleiben dokumentiert. Open Source ist beabsichtigt;
eine Lizenz wurde noch nicht ausgewählt.

## Grundstruktur
- Flach: **Area → Projekt → Aufgabe**. Aufgaben dürfen auch ohne Projekt bestehen.
- Keine Unterprojekte und keine Unteraufgaben in Version 1.
- Inbox, Next, Waiting, Scheduled, Someday, Focus, Logbook, Areas, Projekte, Aufgaben, Tags.
- Projekte: **Sequential** und **Parallel**.
- Sequential wie OmniFocus: **nur die erste noch offene Aufgabe** eines aktiven sequenziellen Projekts darf als nächste Aktion angeboten werden; spätere Aufgaben sind im Projekt sichtbar, aber nicht in globalen Next-Listen. Ist die erste offene Aufgabe nicht verfügbar (z. B. Waiting), wird nichts übersprungen. Die Reihenfolge ist veränderbar.
- Die Berechnung der Verfügbarkeit liegt im Backend; Oberflächen und KI-Clients wenden dieselben Regeln an.

## Eigene Ansichten / Perspectives – später
- Perspectives ersetzen später den heutigen Area-Selektor. Ein Perspektiveneditor
  legt fest, welche Areas und Tags aktiv sind; der übrige Bestand wird in dieser
  Perspektive ausgeblendet. In dieser Etappe ausdrücklich nicht implementieren.
- Benutzerdefinierte, gespeicherte Filterabfragen über Aufgaben **und** Projekte.
- Beispiele: Area, Tags, GTD-Status, Verfügbarkeit, Projektart, Termine.
- Filter zunächst mit UND-Verknüpfung; später bei Bedarf ODER und Gruppierung.
- Eine Ansicht speichert Kriterien, nicht die Ergebnismenge.

## Review – nächste Spezifikationsrunde, noch kein Implementierungsauftrag

Der Nutzer möchte einen Review-Workflow für die spätere Planung vormerken.
Als Bedienreferenz dient OmniFocus, besonders die vom Nutzer geschätzte
Review-Oberfläche auf dem iPad. In dieser Etappe ausdrücklich nicht umsetzen.

Gewünscht ist ein geführter Durchgang in kleinen Schritten durch Inbox und
Projekte, nicht eine gemeinsame Ansicht aller zu prüfenden Inhalte. Die
historische OmniFocus-Projektprüfung und ihre Grenzen sind in
[REVIEW_RESEARCH.md](REVIEW_RESEARCH.md) recherchiert. Die Inbox-Stufe ist ein
bewusster Nextleaf-Wunsch. Intervalle, konkrete Reihenfolge, Fortsetzen und
Bestätigungsregeln bleiben Gegenstand der späteren Spezifikation.

## KI-Schnittstelle
- Vollständige funktionale MCP-Abdeckung ist erforderlich: Ein autorisierter Agent soll alle fachlichen Funktionen der App bedienen können, nicht nur Daten lesen oder Aufgaben anlegen.
- Dazu gehören Aufgaben, Projekte, Areas und Tags mit allen Eigenschaften, Zuordnungen, Reihenfolge, Focus, Terminen, Time/Energy, Abschluss/Wiederöffnung und Papierkorb/Wiederherstellung, soweit die jeweilige Funktion implementiert ist. Ansichten und Filter sowie später umgesetzte Perspectives und Review müssen ebenfalls über MCP zugänglich werden. Diese Anforderung zieht den noch offenen Review-Entwurf nicht vor.
- Eine gemeinsame Geschäftslogik für REST, MCP und die Oberfläche; neue fachliche App-Funktionen erhalten entsprechende MCP-Abdeckung und Tests.
- Implementiert sind der zentrale MCP-Zugang `/mcp` und der lokale Desktop-Zugang über `Nextleaf --mcp` (stdio). Der lokale Adapter nutzt dieselbe Desktop-Datenbank wie die Oberfläche und deren privaten Pi-Abgleich. Für diesen Weg muss der Pi nicht aus einer Anbieter-Cloud erreichbar sein; konkrete Agenten-Clients sind noch einzurichten und zu prüfen.
- Voraussetzung ist tatsächliche lokale Tool-Unterstützung der gewählten Agenten-App. MCP in einer iPhone-Web-App allein ermöglicht keinen Zugriff aus ChatGPT/Claude. Desktop-, Mobile- und Sprachunterstützung getrennt nachweisen. Die Desktop-App speichert lokal in SQLite und gleicht mit dem Pi ab; die iPhone-Web-App bleibt online (siehe unten).
- Rein gerätelokale Einstellungen (z. B. Serveradresse und Darstellung) sind bei der Abdeckungsprüfung ausdrücklich zu erfassen: Der zentrale Server kann diese bislang lokal gespeicherten Werte nicht automatisch ändern. Eine nötige Client-Anbindung ist gesondert zu entwerfen, nicht still als erledigt auszunehmen.
- Umsetzung beauftragt und lokal implementiert: eigener MCP-Server mit vollständiger Abdeckung der aktuell vorhandenen fachlichen Operationen, privatem Streamable-HTTP-Zugang und Bearer-Authentifizierung. Pi-Ausrollung und konkrete Agentenverbindung gesondert prüfen. Die Nirvana-MCP-Verbindung bleibt unabhängig. Umfang und Grenzen: [MCP.md](MCP.md).
- Sprachbefehle auf iPhone und Mac sollen über die jeweilige Agenten-App dieselben Tools aufrufen können. Unterstützung und tatsächliche Netzwerkerreichbarkeit des ausgewählten Clients prüfen; Tailscale auf dem Endgerät stellt keine automatische Verbindung aus einer Anbieter-Cloud her.

## Plattformen
- Ab erster Phase: Rust-Server lokal auf dem Mac; Web-App (mobil nutzbar im iPhone-Browser), Desktop-App für macOS mit Tauri 2; Windows und Linux von Anfang an in der Architektur berücksichtigen und getrennt testen.
- Später optional VPS (z. B. Hetzner) und sichere Verbindung (z. B. Tailscale).
- Als erste Serverumgebung ist der vorhandene Raspberry Pi 5 mit Ubuntu Server 24.04 LTS (Noble, 24.04.5) und Tailscale vorgesehen. Backend, SQLite und gebautes Web-Frontend laufen dort; Browser verwendet die Server-API, Desktop eine lokale Replik mit Abgleich. Betriebssystem, ARM64, Docker-Grundbetrieb und Tailscale-Anmeldung sind anhand der Nutzer-Terminalausgaben bestätigt; Nextleaf-ARM64-Betrieb, Datenpersistenz nach Container-Neuerstellung und HTTPS-Erreichbarkeit über Tailscale inzwischen ebenfalls geprüft.
- Gewählte Betriebsform ist Docker Compose mit einem ARM64-App-Container; Tailscale läuft direkt auf dem Ubuntu-Host. Die Daten liegen ausdrücklich in einem festen Host-Verzeichnis außerhalb des Containers und werden per Bind-Mount eingebunden, um Sicherung und Wiederherstellung zu erleichtern. Containerkonfiguration und Betriebsanleitung sind vorhanden, Docker-/ARM64-Build und Laufzeit erfolgreich geprüft.
- Die Serverablage orientiert sich am Linux Filesystem Hierarchy Standard: Konfiguration unter `/etc/nextleaf`, persistente App-Daten unter `/var/lib/nextleaf` (Bind-Mount nach `/data`). Container müssen ohne Datenverlust ersetzbar sein. Für Wiederherstellung werden konsistente Daten, lokale Konfiguration und die passende App-Version gesichert; das Image ist erneut abrufbar oder aus gesichertem Quellcode baubar.
- Später optional native iPhone-App: offline nutzbar, Änderungen lokal sichern und nach Verbindung synchronisieren.
- Desktop-Offline-Nutzung mit späterem Pi-Abgleich ist jetzt ausdrücklich erforderlich; ersetzt die frühere Beschränkung auf reine Online-Clients. Im aktuellen Quellstand implementiert und isoliert geprüft; Ausrollung getrennt prüfen. Die iPhone-Web-App bleibt vorerst online und benötigt eine Verbindung zum Pi.
- Der Nutzer hat die einmalige vollständige Übernahme seiner vorhandenen Nirvana-HQ-Daten nach Nextleaf beauftragt. Die Daten sollen bevorzugt über Nirvanas offiziellen MCP-Server gelesen werden; Export als Ergänzung bei Umfangslücken. Keine permanente Importfunktion oder laufende Nirvana-Synchronisation in Version 1. Umfang und Feldzuordnung anhand der tatsächlichen Quelldaten prüfen; nichts stillschweigend verwerfen. Siehe [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

## Technik
- Rust, Axum, SQLite und SQLx für den Backend-Dienst.
- React, TypeScript und Vite für dieselbe UI in Browser und Tauri 2.
- Tauri 2 für macOS, Windows und Linux.
- Ein Monorepo; der lokale Checkout liegt aktuell unter `~/Developer/repo/TaskManager`. Der Speicherort ist keine technische Voraussetzung.
- Testdatenbank lokal in `data/`, nicht in Git. Desktop-Daten liegen im OS-spezifischen Anwendungsdatenordner, Pi-Daten unter `/var/lib/nextleaf`.

## Grundsätze
- Im Testimport alle von Nirvana gelieferten Projekte ausdrücklich auf **Parallel** setzen; alle gelieferten Aufgaben einschließlich Historie und Trash erhalten. Notizen, Tags, Focus, Termine und Waiting-/Zeit-/Energieangaben sichtbar machen. `Later` bleibt ein eigener Status; Wiederholungsvorlagen ohne Regel lösen keine Wiederholungen aus.
- Abgehakte Aufgaben sind im Logbook wiederauffindbar, zuletzt erledigte zuerst. Nach dem Abhaken zunächst durchgestrichen an derselben Stelle belassen; kein Completed-Banner.
- Auswahlfelder für Projektart und Aufgabenstatus passen optisch zur App: flache Farben, dezenter Pfeil und sichtbarer Tastaturfokus statt nativer macOS-Verläufe. Native Auswahl- und Tastaturbedienung bleibt erhalten.
- Die Benutzeroberfläche ist vollständig auf Englisch: Navigation, Formulare, Hinweise, Fehlermeldungen, Tooltips und barrierefreie Beschriftungen. Englisch gilt für Web und Desktop; es ist keine Sprachauswahl vorgesehen. Vom Benutzer eingegebene Inhalte bleiben in ihrer Originalsprache.
- Entwicklung und Planung erfolgen vollständig in Codex. Spezifikation, neue Anforderungen und Architekturentscheidungen werden dauerhaft unter `docs/` gepflegt, sodass neue Sitzungen nahtlos weiterarbeiten können. `AGENTS.md` ist der Einstieg; `DECISIONS.md` dokumentiert Entscheidungen und `NEXT_STEPS.md` den Arbeitsstand.
- Einfachheit vor optionaler Flexibilität.
- Server-seitige Regeln für den GTD-Status, keine doppelte Businesslogik im Frontend.
- Stabile UUIDs und dauerhaft gespeicherter Abgleichzustand; keine stillen Konfliktüberschreibungen.
- Lokales Testsystem nicht ungeschützt ins Internet stellen; Authentifizierung und Transportverschlüsselung vor Remote-/VPS-Betrieb implementieren.

## Vereinbarte Bedienung (2026-10-17)

Konkrete Beobachtungen und offene Unterschiede: [UX_REFERENCE.md](UX_REFERENCE.md).

- Nirvana bleibt die Orientierung für einen einfachen GTD-Alltag; die bereits
  vorgesehenen Ergänzungen sind vollständiger MCP-Zugriff und OmniFocus-artige
  Perspectives sowie Parallel/Sequential. Kein zusätzlicher Funktionsumfang
  allein wegen verfügbarer Technik.
- Inbox steht separat ganz oben. Darunter Focus, Next, Waiting, Scheduled,
  Someday und Later; keine zusätzliche Überschrift „Focus“. Logbook, All tasks
  und Trash stehen als seltener genutzte Ansichten unten bei Settings.
- Projects lässt sich unabhängig von der gewählten Ansicht ein-/ausklappen;
  neue Projekte werden ausschließlich über New Item oben angelegt. Das bisherige
  Plus neben Projects entfällt.
- Über dem Ansichts-/Projekt-/Area-Namen steht kein zusätzlicher „GTD / …“-Text.
- Bestehende schlichte Icons beibehalten. Projektsymbole und -titel beginnen
  unabhängig von der Namenslänge an derselben Position.
- Areas werden in Settings angelegt. New Item → Project öffnet einen
  kleinen Dialog für Name und Parallel/Sequential (Vorgabe Parallel); die
  gewählte Area wird übernommen. Keine dauerhaften Erfassungsformulare in der Sidebar.
- Neue Aufgaben in einem Projekt gehören direkt zu diesem Projekt und beginnen
  als Next; dort keine separate Inbox-/Statusauswahl beim Erfassen.
- Beim Abhaken wird der Abschluss sofort gespeichert. Die Aufgabe bleibt zunächst
  durchgestrichen an ihrer Listenposition; erneutes Klicken auf das Häkchen holt sie zurück.
  Keine Completed-Meldung oberhalb der Liste und kein Verschieben der Ansicht.
  Bei Ansichtswechsel oder explizitem Refresh endet diese vorübergehende Anzeige.
  Im Logbook bleibt die Aufgabe dauerhaft erreichbar. Folgeaktionen sequenzieller
  Projekte werden sofort anhand der Backend-Regeln verfügbar.
- In geschlossenen Zeilen sitzt der runde Abschlussknopf an der ersten Titelzeile, unabhängig von
  Notizen, Tags, Projekthinweisen oder Titelumbrüchen. Tags als erkennbare Chips
  darstellen, optisch vom Projektnamen getrennt.
- Klick auf den Titel klappt die Details direkt in der Liste auf (vom Nutzer
  nach direktem Nirvana-Vergleich ausdrücklich bestätigt; ersetzt die frühere
  Beschreibung als extra Fenster). Titel und Tags oben, großer Notizbereich,
  Eigenschaften daneben; bei schmaler Ansicht darunter. Abschlussknopf ist
  nur in der geschlossenen Zeile sichtbar. Im Editor Titel, Notizen, Tags, Time, Energy, Termine,
  Status, Projekt/Area, Waiting for und Focus bearbeiten; Save changes/Cancel.
  Standardmäßig rechts nur Time, Energy, Due, Status und Project zeigen.
  Starts und Waiting for unter zugeklapptem More options; Area in der Tagzeile,
  Focus als anklickbarer gelber Stern neben dem Titel.
  Kleine Abstände und niedriger, bei Bedarf vergrößerbarer Notizbereich.
  Der geöffnete Task hebt sich durch einen etwas dunkleren Hintergrund, klareren
  Rand und dezenten Schatten von den übrigen Zeilen ab; er bleibt inline.
  Escape schließt, ungespeicherte Änderungen beim Navigieren nicht still verwerfen.
- Geschlossene Aufgaben stehen normalerweise in einer kompakten Zeile. Längere
  Titel dürfen auf zwei Zeilen umbrechen; Titel und Metadaten haben getrennte,
  begrenzte Spalten. Tags/Area-Hinweise können ebenfalls auf zwei Zeilen umbrechen.
  Noch längere Titel werden gekürzt, vollständig im Editor zugänglich. Keine
  dauerhaften Statusfelder oder zusätzlichen Notizzeilen. Kopfbereich, Erfassung,
  Sidebar und Zeilen insgesamt dichter gestalten, ohne globale Zoom-Verkleinerung.
  Der Pfeil rechts öffnet ein kompaktes Menü mit denselben Einstellungen und
  einem Einstieg in den vollständigen Editor. Kategorien öffnen sich beim
  Darüberfahren mit der Maus, jeweils nur eine. Die Kategorien bleiben an festen
  Positionen. Das schmale Hauptmenü sitzt rechts am Zeilenpfeil; separate
  Untermenüs öffnen links daneben. Werte direkt anklicken und sofort speichern,
  ohne Save/Übernehmen/Cancel. Erst nach erfolgreichem Speichern schließen; Fehler
  sichtbar melden. Klick/Touch und Tastatur bleiben bedienbar. Freie Texte/Datumsfelder
  mit Enter oder beim Verlassen übernehmen; Termine über den gemeinsamen Kalender. Die beiden Wege nutzen dieselbe API.
- Energy: None/Low/Medium/High; Time als Minuten/Stunden-Auswahl. Bestehende
  importierte Werte erhalten, Angaben müssen sich auch löschen lassen.
- Zentrales New Item-Menü: Action → Inbox/Next/Waiting/Scheduled/Someday/Later/
  Focus sowie Project und Tag. Aufgabenanlage in einem kleinen Dialog; aktuelles
  Projekt bzw. Area bleibt erhalten. Focus ist eine Markierung für eine Next-
  Aufgabe, kein zusätzlicher gespeicherter Status. Neue Tags sind anschließend
  im Aufgaben-Editor als Vorschläge verfügbar; gleiche Namen erzeugen kein Duplikat.
  Pfeil bei New Item vertikal mittig ausrichten und Form/Größe an den Pfeil des
  All Areas-Filters angleichen.
- Keine dauerhafte „Capture a new task“-Zeile. Neue Aufgaben über New Item;
  explizites Ziel wählen, aktuellen Projekt-/Area-Kontext übernehmen.
- Gemischte Aufgabenlisten nach Projekt gruppieren: projektlose Aufgaben unter
  Actions, danach nichtleere Projektgruppen in Sidebar-Reihenfolge. Projektname
  nur als Gruppenüberschrift, niemals erneut in jeder Aufgabe. Im geöffneten
  Einzelprojekt keine zusätzliche Gruppenüberschrift. Reihenfolge der Aufgaben
  innerhalb einer Gruppe erhalten (auch Abschlussreihenfolge im Logbook).
- Projects und Tags liegen im scrollbareren oberen Sidebar-Bereich. Tags als
  kompakte umbrechende Filterliste mit All; jeweils ein Tag aktiv, erneuter Klick
  hebt den Filter auf. Projects und Tags separat einklappbar. Der Filter schränkt
  die aktuelle GTD-/Area-/Projektansicht ein und verändert keine Verfügbarkeitsregeln.
  Logbook, All tasks, Trash und Settings bleiben auf Desktop fest unten sichtbar,
  auch bei überlangen Projekt-/Taglisten; nur der Bereich darüber scrollt.
- Keine Areas-Liste in der Sidebar. Kompakter All Areas-Filter oben, der Aufgaben
  und Projektliste eingrenzt und beim GTD-Ansichtswechsel erhalten bleibt.
  Areas in Settings verwalten. Settings unten mit gleicher Zeilenhöhe, Textkante
  und Icon-Spalte wie Logbook/All tasks/Trash; Zahnrad optisch passend groß.
- Kurze UI-Texte: leerer Zustand nur „All done“; redundante Hinweise zur Eingabe
  und Projektverfügbarkeit sowie technische Vorschautexte entfernen.
- Über View options lassen sich Area-Hinweis, Tags und Fälligkeit je
  Ansicht ein-/ausblenden. Vorgabe: sichtbar. Einstellungen bleiben lokal je Client
  gespeichert; Projekt- und Area-Ansichten verwenden jeweils gemeinsame Vorgaben.
  Aufgabeninhalte ändern sich dadurch nicht.
- Browser verwendet den Pi direkt; der neue Desktop verwendet einen lokalen
  Bestand mit Pi-Abgleich. Die alte Entwicklungsdatenbank bleibt Rückfallstand
  und wird nicht erneut importiert oder als neue Replik verwendet.

## Erstes Inkrement (historischer Gerüst-Umfang)
- Projektstruktur, Spezifikation, Grund-API, SQLite-Migration, erste Weboberfläche und Tauri-Konfiguration.
- Kernaktionen: Areas und Projekte anlegen, Aufgaben erfassen und abschließen, Listen filtern, Next-Actions für sequenzielle Projekte filtern.
- Der ursprüngliche Gerüst-Umfang umfasste noch keine vollständige Bearbeitung. Der aktuelle Stand einschließlich inzwischen ergänzter Aufgabenfelder steht in `NEXT_STEPS.md`; Perspectives, Review und allgemeine Server-Authentifizierung bleiben weitere Etappen. MCP und Desktop-Synchronisation sind inzwischen implementiert.

## Projektübersicht und kompakter Editor (2026-10-17, Präzisierung)

- Klick auf Projects öffnet eine eigenständige Übersicht mit Active Projects,
  Someday und Completed Projects. Ein separater kleiner Pfeil klappt nur die
  Sidebar-Liste ein/aus. Kein Plus; neue Projekte über New Item → Project.
- Jede Projektzeile zeigt die Anzahl offener Aktionen (ohne Completed, Trash und
  Wiederholungsvorlagen), Name, Area, Tags und optionale Fälligkeit/Notizsymbol.
  Leere Projekte bleiben mit 0 Actions sichtbar. Area-Filter gilt auch hier;
  der Tag-Filter bezieht sich in der Übersicht auf Projekt-Tags. Beim Öffnen der
  Aufgaben aus dieser Übersicht wird er aufgehoben (keine automatische Übertragung
  eines Projekt-Tags als Aufgabenfilter).
- Klick auf den Namen öffnet kompakte Projekteinstellungen inline: Name, Notizen,
  Tags, Area, Due, Active/Someday/Completed, Parallel/Sequential und Focus-Stern.
  Save changes/Cancel wie beim Task-Editor. Actions-Zähler oder Sidebar-Projekt
  öffnet dessen Aufgaben einschließlich abgeschlossener Aufgaben, ohne Trash.
- Projektabschluss ändert nur den Projektstatus; Aufgaben werden nicht automatisch
  abgehakt. Inaktive Projekte liefern keine verfügbaren Next-Aktionen. Wechsel
  Parallel/Sequential erhält Reihenfolge und Inhalte aller Aufgaben.
- Die rechte Eigenschaftsspalte hat keine zusätzlichen sichtbaren Labels:
  Time/Energy/Due bei leerem Wert, sonst direkt Wert/Datum. Status und Projekt
  zeigen ihre Auswahl. Beschriftungen für assistive Technologien bleiben erhalten.
- Due/Starts verwenden einen kleinen Monatskalender mit Monatswechsel, Tageswahl,
  Close und Remove date. Während er offen ist, ist der Rest abgedunkelt und
  nicht bedienbar; Escape schließt ohne Änderung. Im Editor ändert Tageswahl den
  Entwurf (Save changes speichert); im Schnellmenü wird die Wahl sofort gespeichert.
- In der Tagzeile direkt hinter den Chips tippen, ohne Add tag-Text/Plus. Nur Enter
  übernimmt neuen Text; vorhandene passende Tags erscheinen als anklickbare
  Vorschläge. Blur, Tab und Save übernehmen unbestätigten Text ausdrücklich nicht.
  Area als eigener Chip, kleines Dreieck rechts öffnet die Area-Auswahl. Bei
  Projektaufgaben bleibt Area vom Projekt geerbt und ist dort nicht überschreibbar.
- Im geöffneten Task verschwindet der Abschlusskreis. Gelber Focus-Stern am Titel
  ist umschaltbar; geschlossene fokussierte Aufgaben zeigen denselben gelben Stern.

## Desktop offline, iPhone-Web-App online (2026-10-17)

- Die Desktop-App erhält einen dauerhaften lokalen SQLite-Datenbestand im
  jeweiligen Anwendungsdatenordner. Nach erster Datenübernahme müssen vorhandene
  Daten ohne Pi-Zugriff lesbar und die vorhandenen fachlichen Funktionen nutzbar
  sein: anlegen, bearbeiten, abschließen, wiederöffnen und Papierkorb bedienen.
- Änderungen lokal sicher speichern, auch über App-Neustarts hinweg. Sobald
  der Pi wieder erreichbar ist, lokale Änderungen übertragen und Änderungen
  anderer Geräte übernehmen. Keine SQLite-Dateien gegeneinander kopieren.
- Oberfläche und lokale MCP-Anbindung verwenden dieselben lokalen Rust-Regeln
  und denselben Bestand; alle Änderungen nehmen am selben Abgleich teil.
  Die Agenten-App selbst kann weiterhin eine Internetverbindung benötigen.
- Pi bleibt gemeinsamer Server und Abgleichpunkt. Web-App auf dem iPhone bleibt
  zunächst online: keine Offline-Datenbank und keine Offline-Schreibwarteschlange.
  Erforderlich ist Pi-Erreichbarkeit, üblicherweise über Internet und Tailscale;
  Internet allein genügt bei ausgeschaltetem oder unerreichbarem Pi nicht.
- Implementiert: Abgleich von Areas, Tags, Projekten, Aufgaben und Beziehungen
  über persistierte Baseline und Dreiwege-Vergleich pro Feld. Unterschiedliche
  Felder werden zusammengeführt; widersprüchliche Werte bleiben als Konflikte
  sichtbar. Explizite Wahl der lokalen oder Serverwerte gilt nur für Konfliktfelder.
  Ändern sich die Konfliktwerte vor der Entscheidung, erneut prüfen und bestätigen.
- Dauerhafte Request-IDs und Server-Quittungen erlauben Wiederholung nach Abbruch;
  Serveridentität verhindert unbemerktes Zusammenführen anderer Datenbanken.
  Trash/Wiederherstellung wird abgeglichen. Nicht unterstützte physische Löschungen
  außerhalb der App stoppen den Abgleich ohne Datenlöschung.
- Automatischer Desktop-Startabgleich nutzt nur die Verbindung der aktuellen
  lokalen Datenbank. Ohne diese Verbindung ist ausdrückliches Connect erforderlich;
  eine alte Adresse im WebView-Speicher reicht dafür nicht aus.
- Hintergrundabgleich alle 15 Sekunden, solange Desktop-UI oder lokaler MCP-Prozess
  läuft. Settings zeigt Verbindung, ausstehende Änderungen, Fehler/Konflikte,
  lokalen Datenordner und den MCP-Startbefehl. Ohne Pi bleiben lokale Funktionen nutzbar.
- Editoränderungen senden ursprüngliche Feldwerte als Vorbedingung; abweichende
  aktuelle Werte führen zu einem sichtbaren Konflikt statt stiller Überschreibung.
- Im Quellstand umgesetzt und mit isolierten Daten geprüft; Tauri-Rustprüfung und
  macOS-Debug-Bundle erfolgreich. Produktiver Pi-Abgleich, tatsächliche Agenten-
  Verbindungen und Plattformtests separat nachweisen. Alte Mac-Datenbank nicht
  als aktuellen Sync-Bestand verwenden oder erneut über den Pi-Bestand importieren.

## Künftige Serverinstallation über Release-Dateien (2026-10-17)

- Installation nach dem vom Nutzer gewünschten Immich-Ablauf: Ordner anlegen,
  Compose-Datei und ENV-Vorlage per `wget` herunterladen, Vorlage als `.env`
  speichern, Einstellungen anpassen und mit Docker Compose starten.
- Fertige versionierte Container-Images für Linux ARM64 (Pi) und AMD64 anbieten.
  Kein Rust-/Frontend-Build und kein Quellcode-Checkout auf dem Zielgerät nötig.
- Compose-Datei und ENV-Vorlage müssen zur gewählten Release-Version passen;
  echte veröffentlichte Download-Adressen erst nach Einrichtung dokumentieren.
- Konfigurationsordner frei wählbar, Standard weiterhin `/etc/nextleaf`.
  Persistente Daten standardmäßig weiterhin `/var/lib/nextleaf`, außerhalb des
  Containers. Datenpfad über `.env` einstellbar; Anleitung umfasst Anlage und
  passende Rechte für den Container-Benutzer.
- Bestehende `.env`, Secrets und Daten bei Updates erhalten. Neue Version bewusst
  auswählen, konsistent sichern, Image herunterladen und Container erneuern.
  Keine automatische Aktualisierung oder Neuinstallation bestehender Daten.
- Linux-Befehle jeweils mit kurzer Erklärung. Docker/Compose und privater
  Tailscale-Zugang bleiben Voraussetzungen. Ziel für künftige Releases;
  aktuelle Pi-Installation wird dadurch nicht verändert.

## Desktop-Installation bestätigt (2026-10-17)

Fertige native Pakete über Release-Downloads: macOS DMG, Windows Setup-EXE,
Linux AppImage bzw. DEB für Debian/Ubuntu. Kein Git-/Rust-/Docker-Setup für
Desktop-Nutzer erforderlich. Beim ersten Verbinden Pi-Adresse konfigurieren;
der Offline-Desktop übernimmt den Bestand über das Sync-Protokoll in seine lokale Datenbank.
Updates zunächst durch Installation einer neuen Version, lokale Daten und
Einstellungen erhalten. Integrierter Updater bleibt spätere Option.

## Eigene mobile Bedienung für die iPhone-Web-App (2026-10-17)

Der Nutzer wünscht eine an Nirvana auf dem iPhone orientierte mobile Oberfläche.
Sechs vom Nutzer in Fotos bereitgestellte Screenshots sind die primäre Referenz;
Beobachtungen und offene Details stehen in UX_REFERENCE.md. Es geht um eigene
mobile Bedienabläufe, nicht nur kleinere Schrift oder schmalere Desktop-Spalten.

- Smartphone: Aufgabenliste als Hauptfläche, Navigation bei Bedarf seitlich
  einblenden. Area-Auswahl und Ansichtsmenü kompakt erreichbar.
- Touchgerechte Aufgabenzeilen mit umbrechenden Titeln, Abschluss und gelbem
  Focus-Stern; Metadaten platzsparend unter dem Titel statt schmaler Seitenspalte.
  Die bisherige Nextleaf-Gruppierung nach Projekt bleibt Ausgangspunkt; die in
  Nirvana sichtbaren Projektlabels sind keine automatische Rücknahme dieser Regel.
- Neue Einträge über einen gut erreichbaren mobilen Plus-Knopf (dieselbe Aktion
  wie New Item); kein dauerhaftes Erfassungsformular.
- Aufgabendetails auf Telefon als einspaltiger, nahezu bildschirmfüllender Bereich
  mit Titel/Notizen und erreichbaren Eigenschaften Tags/Time/Energy/Due sowie
  Zuordnung und weiteren Aktionen. Desktop behält Inline-Details.
- Auswahl per Tippen in einspaltigen Menüs/Sheets; keine Hover-Abhängigkeit oder
  seitlichen Menüketten. Filter kompakt oberhalb der Liste zugänglich.
- Ausreichend große Touchflächen, Bildschirmränder und Bildschirmtastatur
  berücksichtigen; keine verdeckten Speichern-/Schließen-Aktionen oder horizontale
  Überbreite der gesamten Seite. English UI gilt unverändert.
- Mobile Web-App bleibt vorerst online. Gleiche Daten/Funktionen und GTD-Regeln
  wie Desktop; genaue Menüeinträge, Speicherverhalten und noch fehlende Filter
  im mobilen Entwurf festlegen. Kein pauschaler Auftrag zum Nachbau sämtlicher
  sichtbarer Nirvana-Zusatzfunktionen (z. B. Share oder Themes).
- Mobile Darstellung ist im aktuellen Quellstand umgesetzt: Navigation als Drawer,
  Plus-Knopf, Touch-Menüs und einspaltige Details/Sheets, Filter und Safe Areas.
  Kein separates Web-Projekt. Echte iPhone-/Safari-/Tastaturprüfung bleibt gesondert
  nachzuweisen; Screenshots oder Desktop-Viewport-Prüfung ersetzen sie nicht.

## Dezente Zähler und Fälligkeit (2026-10-18)

- Inbox zeigt einen roten Zähler offener Eingänge.
- Focus zeigt überfällige fokussierte Aufgaben rot und die übrigen offenen Focus-Aufgaben dezent grau;
  Waiting zeigt einen grauen Zähler. Kein globaler roter Rahmen um die Ansicht.
- Bei überfälligen offenen Aufgaben erhält nur der Abschlusskreis einen roten Rand.
  Die restliche Aufgabenzeile bleibt ruhig und kompakt.


## Tags, Waiting und ansichtsabhängige Listen (2026-10-18)

Neue Nutzeranforderungen anhand der Nirvana-Screenshots. Backend und UI-Teile
sind inzwischen vollständig integriert und lokal geprüft; Ausrollung noch offen. Verbindlicher
Prüfstand: NEXT_STEPS.md und SESSION_HANDOFF.md.

- Settings → **Manage Tags**: gemeinsame Verwaltung mit getrennten Bereichen
  **Areas**, **People** (Kontakte) und **Tags** (normale Kontext-Tags).
  People sind ein eigener fachlicher Typ, nicht nur ein Tag mit dem Namen People.
  Bestehende Zuordnungen müssen bei Umbenennung und Typänderung erhalten bleiben.
  Personenverwaltung und neue Funktionen ebenfalls über MCP und Desktop-Sync
  anbieten. Migration im Quellstand vorhanden, noch nicht produktiv angewendet;
  keine automatische Umklassifizierung bestehender Tags.
- **Waiting** hebt den Kontakt hervor, auf den gewartet wird: kompakt vor dem
  Aufgabentitel. Kontaktfilter oben direkt erreichbar. Die Referenz zeigt dort
  zusätzlich Areas und normale Tags; die Filterleiste ist nicht ausschließlich
  eine Kontaktliste. Bestehende Waiting-for-Texte beim Ausbau erhalten.
- **— / Untagged** filtert Einträge ohne Tags. Eine Area-Zuordnung ist erlaubt.
  Kontakte einschließlich Waiting-for berücksichtigen: Eintrag mit Kontakt ist nicht ungetaggt.
  All hebt den Tagfilter auf; Kombination mit Area und bestehender GTD-Ansicht
  bleibt erhalten. Der Filter erzeugt keinen Tag und ändert keine Daten.
  Im aktuellen Quellstand für Aufgaben, Projektübersicht, Zähler und mobile
  Filterauswahl implementiert. Noch nicht Teil des produktiven pi6-Pakets.
- Filterleiste passend zur Ansicht und deren Inhalten: Next und Focus bieten
  **Time** und **Energy**, relevante Tags/Areas sowie **All** und **—**;
  **Due** passend zu vorhandenen Fälligkeiten. Waiting betont Kontakte;
  Scheduled und kontaktbezogene Ansichten erhalten passende Filter statt einer
  unveränderlichen globalen Leiste. Keine Rückkehr der zuvor entfernten Hilfstexte.
- Drei kompakte Strich-Schaltflächen für Notizdarstellung: ohne Notizen,
  kurze Notizvorschau und volle Notizen. Aufgabentitel dürfen weiterhin umbrechen;
  die Einstellung ist keine starre Begrenzung des Titels auf eine Zeile.
- **View Options** über das Dreipunktmenü ausschließlich in globalem **Next**
  und **Focus**, nicht in allen Ansichten. Je Ansicht eigene Auswahl:
  - Next: **Unified list**, **Group by project**, **Sequential next actions**
    und Untermenü **Project actions**.
  - Focus: **Unified list**, **Group by project**, **Group by state**.
  Auswahl je Ansicht merken. Die bereits beschlossenen Backend-Regeln für
  Sequential-Projekte gelten weiterhin; eine Darstellungsoption darf keine
  blockierten Aufgaben verfügbar machen. Sequential next actions begrenzt auf
  die erste verfügbare Aktion pro Projekt, ohne den gespeicherten Projektmodus
  zu ändern oder blockierende Waiting-Aufgaben zu überspringen.
  Project actions bietet Top 1 bis Top 9 und All, Vorgabe Top 3; weitere
  Aufgaben über Show more. Referenz im aktuellen Durchlauf geprüft:
  [Nirvana Display options](https://www.nirvanahq.com/help/how-to/display-options).

Referenz zum leeren Tagfilter und zur Verwaltung:
[Nirvana: Filter by tags](https://www.nirvanahq.com/help/how-to/filter-by-tags),
[Nirvana: Manage tags](https://www.nirvanahq.com/help/how-to/tags).

## Neuinstallation als Endnutzer testen (2026-10-18)

Nach Prüfung der mobilen Webansicht möchte der Nutzer die bisherigen Nextleaf-
Testinstallationen auf Pi, Mac und iPhone selbst vollständig entfernen und von
vorn beginnen. Kein Import alter Daten für diesen Durchlauf. Erst Pi über
Release-Downloads, fertiges Image und Docker Compose; dann native Mac-App, danach
iPhone-Web-App. Eine nachvollziehbare Installationsanleitung gehört ins Git-
Repository. Vorbereitung und Nachweise stehen in INSTALLATION.md/NEXT_STEPS.md;
das ist noch kein ausgeführter Reset oder bestandener Endnutzer-Installationstest.
