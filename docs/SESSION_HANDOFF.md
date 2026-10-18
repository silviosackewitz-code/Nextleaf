# Session handoff

Updated 2026-10-18. Use NEXT_STEPS for actual verification and SPECIFICATION for
requirements. Do not infer missing decisions from old chat previews.

## Owner's current request

Publish Nextleaf as an open-source self-hosted task manager with a native Mac app
and iPhone web interface. GPL-3.0-only was explicitly chosen. Public publication
supersedes the earlier private-repository preference. Repository text and app UI
must be English; conversation stays German. The local checkout is now `Nextleaf`.

Release 0.1.0 is published with tested ARM64/AMD64 images, an unsigned Apple Silicon
DMG, corresponding source, checksums, and the English install guide. Both anonymous
installation jobs passed; exact evidence and remaining device checks are in NEXT_STEPS. The owner is now removing old installations and testing a fresh setup, Pi first. Test tasks are disposable and still exist in Nirvana. The old Pi test installation has been removed; client reset is still pending.
No Apple Developer membership/certificate is available, so do not claim notarization.

## Repository and runtime

Repository: https://github.com/silviosackewitz-code/Nextleaf, branch `main`.
The original technical identifier `dev.taskmanager.desktop` remains to preserve
local data compatibility. Local/server database files and secrets are ignored.

The shared Rust backend serves Axum REST, central MCP, in-process desktop IPC,
and local stdio MCP. React is shared between browser and desktop. Desktop owns
its local SQLite data and syncs privately; phone web is online-only. Protocol 2
retains tag IDs, People kinds, archives, and field-level conflict behavior.

The owner has now removed the pi7 container/network, its old configuration/data,
and pi7 image. Cleanup of older Nextleaf images and release-file download verification
are not yet confirmed. Mac and iPhone reset is pending; keep old replicas closed.

The agreed shared layout is `/opt/nextleaf-app/` for Compose and `.env`, with data
under `/srv/nextleaf/`. This mirrors the supplied Immich directory specification.
Back up both, using a consistent SQLite copy. See STORAGE and the updated installation
guide. Published 0.1.0 defaults still point to `/var/lib/nextleaf`; the guide overrides
`NEXTLEAF_DATA_DIR` explicitly. Do not replace already-published assets or checksums.
The new host-path installation and full device acceptance are not yet complete.
Pi administrative steps require the owner's sudo access; do not claim installation
from runner-only tests or configure backup schedules without a separate request.

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
