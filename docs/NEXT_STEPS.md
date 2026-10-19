# Current status and next steps

Updated: 2026-10-18. Historical duplicate status notes have been consolidated;
requirements and reasons remain in SPECIFICATION and DECISIONS, with original
history available in Git.

## Layout refinement after 0.1.1 feedback — source verified, not released

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
The installed Mac and Pi still run published 0.1.1; no replacement artifacts have
been produced for this refinement. Continue owner scheduling acceptance after
shipping the agreed UI update.

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

## Completed owner-requested MCP import

On 2026-10-18, the owner requested a complete fresh Nirvana import through local
MCP. Read-only retrieval returned 130 objects (119 tasks, 11 projects), 3 Areas,
and 13 context tags. Separate Logbook/Trash ID reads matched the full snapshot.
The owner explicitly selected Personal for the two tasks with multiple Areas;
the untouched original records remain in the private import receipts.

A separate database test imported all 130 records, compared supported fields and
relations, and confirmed that repeating the input skipped all 130 without changes.
The same local stdio MCP tool then imported into the desktop database after a
consistent backup. The existing test task was preserved. Final totals are 120
tasks, 11 projects, 3 Areas, and 13 tags. SQLite integrity and foreign-key checks
passed. The repeated live import inserted zero records and skipped all 130.
A complete field-and-association comparison of the private Pi sync snapshot with
the local database passed after background synchronization. Nirvana was unchanged.
The refreshed iPhone screen was not independently inspected after import.

Current source exposes the shared importer as `import_nirvana` (14 MCP tools).
The release-mode desktop executable built successfully. All 30 backend tests,
including local HTTP sync, passed; the subsequently added incompatible-metadata
rollback test also passed. The initial real-data test correctly rejected ambiguous
Areas without partial writes. Private source snapshots, backups, and verification
receipts are outside the repository and must never be published.

At the time of import, the installed revision-3 app and 0.1.0 container exposed
13 tools. Import used a local development executable sharing the desktop
database. No release was published at the time of import; 0.1.1 now includes
that fourteenth tool as recorded above. Third-party client
registration remains separate. Recurring source templates retain their status but
cannot generate occurrences without a complete source rule. Import receipts remain
local-only; preserve the importing desktop database for safe retries.

## Resolved Mac installer defect

The owner's browser-downloaded Mac installer reported "damaged". Both downloaded
DMGs matched the published SHA-256, but strict app-bundle signature verification
failed. This corrects the earlier statement that no release defect was found:
that statement applied to the Compose checks, while the Mac bundle seal had not
been checked. Revision 3 enables complete Tauri ad-hoc bundle signing and uses a
plain-language macOS Apple Silicon filename. The rebuilt final DMG passed image
integrity and strict deep bundle verification, including a sealed resource manifest.
The mounted app passed isolated local MCP startup, 13-tool enumeration, tag edits,
and restart persistence with SQLite integrity. TypeScript/Vite and the Tauri release
build passed. The owner subsequently confirmed that the replacement app opens;
no quarantine removal or system security bypass was performed by this agent.
Revision 3 is published at `install-v0.1.0-r3` with eight assets. All seven payload
files were anonymously downloaded and matched the published checksums. The DMG is
`Nextleaf-0.1.0-macOS-Apple-Silicon-r3.dmg` with SHA-256
`37967a18a32007d5289c50f6de8eec0be1bc6cf38b8e77973e500f1f529ad8ac`.
Earlier release pages and Mac asset labels now identify the superseded installer.
Compose/environment files and server image are unchanged, so server runtime tests
were not repeated; new download hashes were verified. The Pi remains healthy and
no server reinstall is needed.

## Current milestone

Published the public **0.1.0 installation-test prerelease**, GPL-3.0-only, with
English repository/application text, Linux ARM64/AMD64 images, versioned Compose
files, an ad-hoc-signed Apple Silicon DMG without Developer ID/notarization,
corresponding source, and checksums.
The local checkout is `Nextleaf`. The owner confirmed removal of the old Mac app,
its data folders, and iPhone website/app state. The old pi7 container, configuration,
and data were removed, followed by a fresh public-image installation using the
new configuration location `/opt/nextleaf-app` and documented `/srv/nextleaf` path.
The owner's screenshot confirms a healthy 0.1.0 container, loopback port binding,
`status: ok`, `tag-management`, `sync-v2`, disabled MCP, and private Tailscale HTTPS.
The owner confirmed browser access on the Mac and creation of the installation
check task. Private HTTPS health and web-page retrieval also passed from the Mac.
The owner then confirmed the replacement native Mac app opens and synchronization
works: test tasks are present on both Mac and server. A screenshot shows the
installed app with the server test task and "Synced". The owner also confirmed
that the iPhone web interface works after cleanup and setup. This establishes the
basic fresh-install path across Pi, Mac, and iPhone on the owner's devices; it does
not establish every mobile interaction, offline/conflict scenario, or restore test.

Installation revision 2 updates the `.env` template and Compose fallback to the
agreed paths directly; no post-download path correction is needed. It reuses the
same 0.1.0 image and DMG. The original release remains available. README now explains
the GTD workflow, device requirements, private hosting, limitations, and feedback.

The GitHub Code-page README now includes the full Linux first-install commands,
plus visible macOS Apple Silicon/M-series versus Intel guidance and the route to
Releases → Assets. Both Mac release assets have a plain-language display label;
filenames and checksums are unchanged. Repository topics include task-manager,
task-management, todo-list, productivity, getting-things-done, and self-hosted.

## Installation revision 2 verification

[Revision 2](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/install-v0.1.0-r2)
is published with eight assets. All seven payload files were anonymously downloaded
and matched the new SHA256SUMS. The Mac DMG matches the original release byte for
byte; application source remains unchanged. Revised Compose fallback was evaluated
on the Pi without changing the running service and resolved to `/srv/nextleaf`.

[Public installation checks](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39156831309)
passed on ARM64 and AMD64: downloads, checksums, exact template match, standard data
path, unauthenticated image pull, fresh install, API/MCP, and recreation persistence.
The first new check failed because one Compose version omits false-valued JSON
fields; the assertion was corrected. No release or application defect was found.

## Release verification

[Release 0.1.0](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/v0.1.0)
is public, with eight assets including checksums. Application source at the release
tag: `3764c3546a68d0c8fbc6edcdc0f3e4b5c59f1a83`.

- [Native image build and smoke tests](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39150545266):
  ARM64 and AMD64 passed. Each build ran 28 Rust tests; the explicitly ignored
  localhost-network test was not included in the ordinary cargo test invocation.
  The container smoke check exercised real HTTP/MCP and restart persistence.
- [Anonymous installation test](https://github.com/silviosackewitz-code/Nextleaf/actions/runs/39150955212):
  both architectures downloaded and verified the release configuration, pulled
  the image with empty registry credentials, and passed fresh Compose/API/MCP,
  tag/Person rename, and data/server-identity persistence checks.
- Independently downloaded every asset without authentication; all seven file
  hashes matched the published SHA256SUMS and local originals.
- Mac DMG: image checksum verified; Apple Silicon executable, correct version,
  Applications link, GPL/third-party resources checked. Running the executable
  from the read-only DMG against isolated data passed all 13 MCP tools, tag/Person
  creation/rename/archive/restore, restart persistence, and SQLite integrity.
- Re-ran the 39 frontend tests successfully. TypeScript/Vite and the native release
  build passed. No application source differs between the tested DMG and tag.

Container index digest: `sha256:ed1f29adf8fa75c04d0d28e0f6afaf80e127952b7d21ce0c3a8d7374d45241dd`.
The Mac build has no Developer ID signature or notarization. Fresh-machine
Gatekeeper behavior and actual physical-device installation remain owner tests.
The first container attempt caught an unsupported read route in the new check;
the check was corrected before publishing any images or the release.

## Verified before release packaging

- Pi7 updater completed with `NEXTLEAF_UPDATE_OK`; server health includes
  `tag-management`, `sync-v3`, and `scheduling`.
- Shared tag/People/Area operations, archive/restore, stable links, stale-edit
  protection, and sync v2 are integrated in backend, desktop, web, and MCP.
- Isolated Mac verification covered local persistence, offline operation, stdio
  MCP, and synchronization with the Pi. A fresh database no longer auto-connects
  using an old browser-stored remote.
- Frontend: 39 tests passed; TypeScript and Vite build passed.
- Mobile browser checks covered 320×568, 390×844, and 390×360, plus desktop 1280px.
  Verified reachable controls, bounded sheets, no full-page horizontal overflow,
  and touch sizing. New controls use 44px targets and 16px input text.
- Live Pi browser checks at phone dimensions covered tag/Person/Area creation,
  rename, archive/restore, Waiting contacts, assignment retention, and cleanup
  of disposable test items.

These checks do not claim an actual physical iPhone Safari/Home Screen keyboard
test or a complete download-to-clean-Pi installation.

## Next acceptance steps

1. Check offline desktop edits and reconnection on the fresh installation.
2. Check concurrent edit conflicts, real Safari keyboard/landscape, and Home Screen
   behavior explicitly; the owner's general iPhone confirmation is not a full checklist.
3. Set up an actual assistant through MCP when requested, and rehearse backup/restore.
4. Record outcomes before expanding features.

## Remaining work after this milestone

- Developer ID signing/notarization and broader clean-machine Mac acceptance.
- Intel Mac, Windows, and Linux desktop packaging and actual platform checks.
- Full backup/restore rehearsal, beyond container recreation persistence.
- Real ChatGPT/Claude registration and voice-mode tests; local stdio is preferred.
- Manual ordering, saved Perspectives, and guided Review need further work;
  Review/Perspectives require the next specification round.
- No native iPhone app, offline web app, automatic desktop updater, or automatic
  recurrence engine for imported templates is promised in this release.

## Historical milestones

2026-10-17: verified scaffold, completed the one-time Nirvana import, ran the first
Pi container, refined compact navigation/task/project editing and calendars, and
introduced central MCP. Pi stages pi1–pi5 tracked those iterations.

2026-10-18: offline desktop/local MCP and initial sync evolved through pi6, followed
by stable tag identity, People/archive management, and sync v2 in pi7. Mobile
reference work became responsive UI and browser checks. The repository was first
private, then explicitly made public by the owner. GPLv3 and English repository
text were chosen before the first downloadable release.
