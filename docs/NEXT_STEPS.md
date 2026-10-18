# Current status and next steps

Updated: 2026-10-18. Historical duplicate status notes have been consolidated;
requirements and reasons remain in SPECIFICATION and DECISIONS, with original
history available in Git.

## Current milestone

Prepare the public **0.1.0 installation-test prerelease**, GPL-3.0-only, with
English repository/application text, Linux ARM64/AMD64 images, versioned Compose
files, an unsigned Apple Silicon DMG, corresponding source, and checksums.
The local checkout is `Nextleaf`. The owner will perform the later empty-device
installation test using INSTALLATION.md. Do not reset existing installations now.

Release preparation is in progress. Before announcing it ready, record successful
container jobs, anonymous image access/downloads, and Mac artifact checks here.
The first local Mac app/DMG build succeeded; it must be rebuilt after bundling
license notices. No Developer ID certificate is configured.

## Verified before release packaging

- Pi7 updater completed with `NEXTLEAF_UPDATE_OK`; server health includes
  `tag-management` and `sync-v2`.
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

1. Finish publishing and verifying release artifacts and public registry access.
2. Owner removes old Pi/Mac/iPhone test installations and local replicas.
3. Owner follows the guide: empty Pi, native Mac, then iPhone web.
4. Check persistence, offline desktop edits, reconnection, concurrent edit conflicts,
   real Safari keyboard/landscape, and Home Screen behavior.
5. Record actual outcomes and fix installation issues before expanding features.

## Remaining work after this milestone

- Developer ID signing/notarization and clean-machine Mac acceptance.
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
