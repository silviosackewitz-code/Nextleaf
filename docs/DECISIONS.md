# Decision log

Consolidated English record. Later decisions supersede earlier ones where stated;
Git history retains the original development notes. Dates are decision dates,
not promises that every target platform was tested.

## 2026-10-18: repair the Mac bundle signature

The owner's browser-downloaded DMG reported that Nextleaf was damaged. Both local
download hashes matched the published artifact. Strict bundle verification failed
with "code has no resources but signature indicates they must be present": only
the linker-generated executable signature existed, with no sealed bundle resources.
The previous release checks missed this defect. Configure Tauri ad-hoc signing
for the complete bundle and publish installation revision 3 with a clearly named
macOS Apple Silicon DMG. Keep earlier artifacts for traceability but mark their
Mac installers superseded. Preserve server image, data, and installed configuration.
Verify the final mounted DMG's bundle signature before release. This does not
supply an Apple identity or notarization, or replace real downloaded-GUI testing.

## 2026-10-18: visible installation and download guidance

The owner could not identify the Mac installer or find Linux installation commands
on the GitHub Code page. Put the complete first-install server command sequence in
README, explain Apple Silicon/M-series versus Intel next to the Mac download, and
use a human-readable release asset label. Keep existing filenames and checksums
so earlier download links remain valid. Add relevant product topics to GitHub's
About section for discovery, including task-management, todo-list, productivity,
and getting-things-done alongside the existing task-manager and self-hosted topics.

## 2026-10-18: ship the agreed paths as installation defaults

The owner requested the actual environment template be updated immediately, along
with more explanatory GitHub text. Publish installation files revision 2 under
`install-v0.1.0-r2`, with `/srv/nextleaf` in `.env` and the Compose fallback, and
`/opt/nextleaf-app` in installation instructions and errors. This supersedes the
plan to wait for the next application release. The 0.1.0 image and Mac DMG remain
unchanged; distribute the revised configuration, guide, corresponding source, and
matching checksums together. Preserve original release downloads. An installation
already using the agreed paths does not need to be reset or have its `.env` replaced.

## 2026-10-18: shared application directories and explicit backup scope

The owner supplied the existing Immich directory specification and requested the
same pattern for Nextleaf and future personal applications. Use `/opt/<app>-app/`
for Compose and `.env`, `/srv/<app>/` for persistent data, and leave Docker's
internal storage managed by Docker. For Nextleaf these are `/opt/nextleaf-app/`
and `/srv/nextleaf/`. Both belong in backups; SQLite must be copied consistently.
This supersedes the earlier `/etc/nextleaf` and `/var/lib/nextleaf` deployment layout.
The earlier spoken “Source”/“SRC” was clarified by the supplied document's `/srv`.

Keep published 0.1.0 assets and checksums intact. The image already supports an
external data path, so the installation guide sets `NEXTLEAF_DATA_DIR=/srv/nextleaf`
explicitly and installs the configuration under `/opt/nextleaf-app`. No application
rebuild is required. Align template defaults in the next release. Backups to an
external disk and encrypted off-site storage are a separate setup task; the supplied
reference's planned destinations are not proof that any backup is running.

## 2026-10-18: public release, GPLv3, and English repository

The owner explicitly approved public GitHub publication, superseding the earlier
same-day private-repository decision. Use GPL version 3 only (`GPL-3.0-only`).
Describe Nextleaf as a self-hosted task manager with a native Mac app and iPhone
web interface. All current repository prose and UI text must be English; German
conversation is fine. Translate and consolidate outdated status notes, without
rewriting old commits. Rename the local checkout from TaskManager to Nextleaf.
Keep internal database/application identifiers stable for compatibility.

The owner has no Apple Developer membership and approved an unsigned test DMG.
Ship an explicitly labeled installation-test prerelease, with matching versioned
server images, Compose files, source, notices, and checksums. Signing/notarization
remain necessary work for a smoother public Mac installation experience.

## 2026-10-18: fresh installation is the next acceptance test

After mobile verification, the owner will remove old Pi/Mac/iPhone test installs
and start from empty data, server first. Use wget and Docker Compose on the Pi,
a native DMG on Mac, and the web interface on iPhone. Document the whole path.
Tasks are disposable test data with originals still in Nirvana. No automatic reset
or import is part of release preparation. An old desktop replica must not silently
connect to or repopulate a fresh server.

## 2026-10-18: connection belongs to the local database

A remembered browser UI URL must not auto-connect a fresh desktop database.
Startup uses only the remote saved with that database. Keep server identity and
URL checks. Test with an isolated data directory and no accidental old connection.

## 2026-10-18: embedded desktop, offline sync, and local MCP

Embed the existing Rust backend in Tauri through in-process Tower calls, rather
than maintaining a second implementation or sidecar. UI and stdio MCP share the
same local SQLite data and 15-second background sync. Sync uses a retained baseline,
merges independent edits, and presents same-field conflicts explicitly with stale
resolution protection. Web remains server-backed and online-only.

The preferred agent path is local agent → desktop stdio → private Pi sync. A working
transport is not proof of ChatGPT/Claude registration or iPhone voice support.

## 2026-10-18: tag management and sync protocol 2

Separate Areas, People, and ordinary Tags. Preserve IDs and associations on rename,
kind change, archive, and restore. Do not infer People from old tag names. A person
rename updates only exactly matching Waiting text. Retain archived links but reject
new direct assignments. Use stable tag IDs in sync v2 and migrate local v1 baselines.
Update the server first; reject incompatible clients without mutation.

Add contextual filters and Untagged (contacts, including free text, count as tagged),
three note display modes, and view-specific options only for global Next and Focus.
Next may limit available project actions without overriding Sequential blocking.
Top 3 is the initial project display limit; Top 1–9 and All remain choices.

## 2026-10-18: mobile and visual restraint

Phone UI gets a drawer, full-width rows, reachable plus button, touch menus, and
single-column details with keyboard/safe-area handling. Keep one React app.
Inbox count is red; Focus overdue/other counts are red/gray; Waiting count is gray.
Only the completion circle is red for overdue tasks. Real iPhone Safari checks
remain distinct from desktop viewport tests.

## 2026-10-17: task and project interaction refinements

The user confirmed inline task details rather than a separate OS window. Primary
fields are Time, Energy, Due, State, and Project; additional fields move under More
options. Save/Cancel remains explicit. Title and metadata may occupy two lines,
superseding an earlier strict single-line preference. Quick actions use compact
left-opening desktop submenus; category hover, direct selection, keyboard, and
touch must coexist. Free text commits on Enter/blur. Tag input commits only on
explicit Enter or suggestion selection, never incidental blur/Tab/Save.

Use a shared calendar, accessible native selects, compact project editing, and
an overview with active/someday/completed sections including empty projects.
Project completion does not complete children. Keep GTD availability in Rust.

Group nonempty project actions, put standalone actions first, and remove repeated
project labels. Replace the permanent capture row with New Item. Projects title
opens the overview while its separate arrow collapses navigation. Move Areas to
Settings and filtering; keep Logbook/All/Trash/Settings at the bottom.

Completed rows remain temporarily struck through until refresh/view change, with
immediate backend persistence and undo at the checkbox. No completion banner.
This is the implemented cleanup policy; no separate Cleanup command was approved.

## 2026-10-17: platform and distribution direction

Use the existing Rust/Axum/SQLx/SQLite + React/TypeScript/Vite + Tauri 2 scaffold;
verify it before expanding the UI. Develop in Codex and keep requirements and
handoff in Git. Preserve the shared backend for web, native UI, and MCP.

Target the existing Raspberry Pi 5 running 64-bit Ubuntu 24.04 LTS (24.04.5 was
recorded during setup, not a new requirement). Tailscale stays on the host.
Use host data `/var/lib/nextleaf`, config `/etc/nextleaf`, a non-root container,
and loopback binding. Prefer persistent host data over an opaque container layer.

The eventual server install downloads Compose/environment files and pulls a ready
image. Native packages are DMG, Windows Setup EXE, and Linux AppImage/DEB, with Mac
first. No development toolchain should be required by desktop users. Updates are
manual package replacement initially; automatic update infrastructure is deferred.
The iPhone uses a responsive online web app first; native iPhone is later.

## 2026-10-17: central MCP coverage

Expose all implemented task/project/Area operations through the official Rust SDK
and shared domain rules, then include tag management when implemented. Use authenticated
Streamable HTTP centrally and local stdio for desktop. Do not expose unauthenticated
REST/sync publicly. Agent-access setup and voice testing remain separate work.

## 2026-10-17: one-time Nirvana import

Use the official Nirvana MCP connector read-only. Import all 128 retrieved objects,
including completed and trash items, preserving notes and source identity. Set all
11 imported projects to Parallel by explicit request. Do not invent unavailable
recurrence rules or ongoing sync. Make repeat import idempotent and reject changed
source records rather than overwriting local edits. See [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

## 2026-10-17: naming

Nextleaf is the chosen name (Nextlief was a typo). Research found existing uses;
the owner still chose Nextleaf. Clear Trail was rejected after overlaps were found.
Donebrook and Readyfern were researched alternatives, not chosen replacements.
No domain was purchased and no legal name clearance was established.
See [NAMING.md](NAMING.md) for the dated research evidence.

## 2026-10-17: future Review and Perspectives

Review should eventually guide the user through Inbox and individual projects in
small steps, inspired by old OmniFocus iPad workflows. Research is not an instruction
to implement it now. Perspective filters and Review dates/intervals/resume behavior
need a dedicated specification round. Mobile, offline desktop, MCP, and installation
have priority.

## 2026-10-18: complete fresh-install import through local MCP

The owner explicitly requested all Nirvana tasks in Nextleaf through MCP after
confirming fresh installation on Mac, Pi, and iPhone. Read Nirvana only. Preserve
existing Nextleaf records and include projects, completed/trash history, notes,
and metadata. Reuse the existing transactional importer through a new MCP tool
because ordinary task creation cannot retain historical dates or recurring
source templates. Keep project mode Parallel as previously chosen. This is a
one-time transfer, not ongoing Nirvana synchronization. Local import receipts and
raw source data must remain private and be backed up with the desktop database.

The owner subsequently selected Personal for both source tasks with multiple
Areas. Keep each untouched source object in the private prepared record under
`original_source`, while mapping the task to the explicitly selected Area. The
successful live import and Pi synchronization are recorded in NEXT_STEPS.
