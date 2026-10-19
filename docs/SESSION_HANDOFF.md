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

## UI refinement release 0.1.2 — published, owner update pending

The owner rejected the duplicated State chevron, wrapping scheduled ISO date and
uneven notes/tag alignment, and asked for a more spacious list. The source now
uses one shared dropdown chevron, a calendar icon and compact English scheduled
dates. Desktop rows reserve a Focus column, keep notes beside the title, use
quiet text tags and more spacing, and use the available window width.

42 frontend tests and the TypeScript/Vite build passed. Isolated browser checks
at 1120×760 verified identical title/menu positions for rows with and without
notes, calendar selection and saving, plus the new Focus control. At 390px and
320px, the complete scheduled label and icon fit; the 320px control remained
46px high with no page overflow. These are browser checks with synthetic data,
not physical iPhone or native Mac acceptance. Screenshots stay outside the repo.
Release [v0.1.2](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/v0.1.2)
is published with an Apple Silicon DMG, ARM64/AMD64 server images, matching source,
Compose/environment files, license notices and checksums. The final mounted DMG
passed image integrity and strict deep ad-hoc signature verification. Its MCP
startup against an isolated copy of the existing 0.1.1 database preserved every
table/field (including migration history and import receipts), relations, client/
server identities and baseline, with SQLite integrity and foreign keys valid.

Both native Linux builds passed Rust tests, fresh Compose/API/MCP checks, repeat
generation, recreation persistence, and a container update from published 0.1.1.
Anonymous public installation passed on both architectures. All eight payloads
were independently downloaded and matched the published checksums.
[Container/update verification](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39196352642)
· [Public install verification](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39196892721).

The owner plans to install 0.1.2 on the following day; the real Mac and Pi remain
on 0.1.1. Schema and sync protocol stay at version 3. Owner update/scheduling
acceptance and physical iPhone keyboard checks remain pending. Community UI and
installation requests are open in issues #1 and #2, now targeting 0.1.2; external
tester results have not been received as part of this verification.

## Scheduling and Contacts update 0.1.1 — owner update and sync accepted

The owner explicitly requested both Start Date and Make Repeating through State →
Scheduled, no More options disclosure, Waiting contact choices, and travel-aware
system timezones. These are implemented in current source with sync protocol 3.
The existing two-contact correction is preserved, including verified Pi sync.
The shared importer remains available as the fourteenth MCP tool.

Current checks: 41 frontend tests, 38 backend tests including scheduling, timezone,
protocol migration, repeat sync and HTTP sync, plus the TypeScript/Vite build pass.
Desktop-width browser checks passed for draft scheduling and immediate repeating;
390px checks reached the repeat editor and Cancel returned without changing the
parent draft. This is not a physical iPhone or Safari keyboard acceptance test.
The final 0.1.1 Apple Silicon DMG passed image integrity and strict deep ad-hoc
signature verification. Its mounted app upgraded an isolated copy of the existing
desktop database: every old entity field, relation, import receipt and connection
was preserved; SQLite integrity, foreign keys and protocol-3 migration passed.
Release v0.1.1 is published with the Mac DMG, corresponding source, Compose
configuration, license notices and checksums. Native ARM64 and AMD64 container
builds passed fresh-install, API/MCP, repeat generation, recreation persistence
and in-place update checks from 0.1.0. Anonymous release downloads and public
GHCR installs passed on both architectures; all eight downloaded payload hashes
match the published checksum file.

Verification runs: [containers and update](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39176317896)
and [public installation](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39176765491).
The earlier Contacts-only DMG is superseded. The owner completed the in-place
update on 2026-10-18 after a stopped-service Pi backup and a consistent local
desktop backup. The updated private server reports healthy with sync-v3 and
scheduling. The installed Mac bundle is 0.1.1, its sync baseline uses protocol 3,
and the Pi snapshot matches that baseline for all 120 tasks, 11 projects, 3 Areas
and 13 tags at the verification point; no sync errors or conflicts were present.
The owner then confirmed the practical test: creating a task on the Mac appeared
on the iPhone, and renaming it on the iPhone returned to the Mac. Basic update
and bidirectional synchronization acceptance passed. Start Date and Make
Repeating acceptance on the owner's devices remains pending; timezone travel and
a full disaster-recovery exercise have not been tested on those devices.

## Completed MCP import

The owner requested all Nirvana tasks/projects through local MCP and selected
Personal for both ambiguous multi-Area tasks. The shared importer is now exposed
in current source as `import_nirvana`. A separate database run verified complete
mapping and repeat safety before live import through stdio. The original source
objects and explicit mapping are retained privately, with a pre-import backup.

Imported 119 tasks, 11 projects, 3 Areas, and 13 tags. The existing test task remains,
so Nextleaf now has 120 tasks total. Repeating the prepared snapshot inserted zero
records and skipped all 130. Every persistent entity field and tag association
matched between desktop and Pi after background sync. Nirvana was not modified.
No post-import iPhone screen inspection or third-party client registration occurred.

At the time of the import, public binaries exposed 13 tools and the development
executable exposed 14 while sharing the local desktop DB. Release 0.1.1 now
includes that fourteenth tool; see the release verification above. Preserve the working installation and local-only import receipts. Do
not reimport into a separate replica without those receipts. See NEXT_STEPS for
verification and NIRVANA_IMPORT for mapping/recurrence limitations.

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
updates/backup and prior Pi stages. MCP documents the 13 release tools, the additional development import tool, and client limits.
NIRVANA_IMPORT records the historical import and current one-time MCP workflow. UX_REFERENCE, REVIEW_RESEARCH,
and NAMING distinguish observations from accepted requirements and future choices.

The checkout folder was renamed successfully. The Codex saved-project entry still
points to the previous TaskManager path; the owner must update that app-level
project link because the available tools cannot edit it. This does not affect
GitHub, the renamed checkout, or release artifacts.
