# Session handoff

Updated 2026-10-18. Use NEXT_STEPS for actual verification and SPECIFICATION for
requirements. Do not infer missing decisions from old chat previews.

## Owner's current request

Publish Nextleaf as an open-source self-hosted task manager with a native Mac app
and iPhone web interface. GPL-3.0-only was explicitly chosen. Public publication
supersedes the earlier private-repository preference. Repository text and app UI
must be English; conversation stays German. The local checkout is now `Nextleaf`.

The owner completed the basic fresh-install path: healthy Pi server, replacement
Mac app opens and synchronizes test tasks with the Pi, and iPhone web access works.
Installation revision 3 contains the repaired ad-hoc-signed Mac DMG. Detailed
remaining acceptance checks are in NEXT_STEPS. Test tasks remain disposable.
No Apple Developer membership/certificate is available; do not claim notarization.

## Resolved Mac packaging correction

The owner reached a "damaged" alert with the downloaded Mac DMG. Hashes matched,
but the original app bundle failed strict codesign resource verification. Revision
3 enables Tauri ad-hoc bundle signing and a macOS Apple Silicon filename. Do not
use the older DMGs for the clean-install acceptance test or bypass quarantine.
The owner confirmed the replacement app opens and sync works. See NEXT_STEPS for
verification limits. Preserve the working Pi and fresh client data.

## Repository and runtime

Repository: https://github.com/silviosackewitz-code/Nextleaf, branch `main`.
The original technical identifier `dev.taskmanager.desktop` remains to preserve
local data compatibility. Local/server database files and secrets are ignored.

The shared Rust backend serves Axum REST, central MCP, in-process desktop IPC,
and local stdio MCP. React is shared between browser and desktop. Desktop owns
its local SQLite data and syncs privately; phone web is online-only. Protocol 2
retains tag IDs, People kinds, archives, and field-level conflict behavior.

The owner confirmed Mac data-folder deletion and iPhone cleanup, then installed
the public 0.1.0 image with the new layout. Screenshot evidence shows a healthy
container, expected API capabilities, and private HTTPS forwarding. The owner then confirmed native Mac synchronization
and working iPhone web access. The basic setup is complete. Further offline,
conflict, detailed mobile, MCP-client, and restore checks remain separate.

Configuration belongs in `/opt/nextleaf-app/`; persistent data is `/srv/nextleaf/`.
Back up both consistently. Installation revision 2 (`install-v0.1.0-r2`) updates
`.env` and Compose defaults directly, reuses the 0.1.0 image/DMG, and preserves the
original downloads. No reinstall is needed for the owner's correctly configured
server. Do not replace its `.env`, reset its data, or configure backup schedules.

## Preserve these constraints

- A fresh local database must not auto-connect from stale browser UI storage.
- Preserve server identity and URL binding; update server before protocol-v2 clients.
- Tag/Person renames and archive actions retain existing assignments. Archived
  entries cannot be newly assigned; Waiting free text must not be lost.
- Sequential blocking is backend truth. Display options do not override it.
- Real iPhone Safari/keyboard, third-party agent registration, and complete restore
  checks remain unverified unless recorded with new evidence.
- Do not implement Review/Perspectives based solely on research references.
- Do not publish task exports, private screenshots, databases, `.env`, or tokens.

## Documentation map

INSTALLATION covers first use and owner-performed reset; DEPLOYMENT covers normal
updates/backup and prior Pi stages. MCP documents all 13 tools and client limits.
NIRVANA_IMPORT records the historical one-time import. UX_REFERENCE, REVIEW_RESEARCH,
and NAMING distinguish observations from accepted requirements and future choices.

The checkout folder was renamed successfully. The Codex saved-project entry still
points to the previous TaskManager path; the owner must update that app-level
project link because the available tools cannot edit it. This does not affect
GitHub, the renamed checkout, or release artifacts.
