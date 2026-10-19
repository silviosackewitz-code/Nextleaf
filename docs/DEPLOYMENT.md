# Deployment and backup

For a fresh installation use [INSTALLATION.md](INSTALLATION.md). This document
covers runtime layout, normal updates, backups, and the historical Pi test path.

## Runtime layout

| Location | Purpose |
| --- | --- |
| `/opt/nextleaf-app/docker-compose.yml` | Download-release Compose configuration |
| `/opt/nextleaf-app/.env` | Image version, host data path, optional MCP token; mode 0600 |
| `/srv/nextleaf` | Persistent SQLite database and companion files; UID/GID 10001 |
| `127.0.0.1:8787` | Host-only HTTP endpoint |
| Tailscale Serve | Private HTTPS proxy on the host |

No web login is implemented. Restrict the tailnet audience and do not publish the
REST/sync port. MCP's Bearer token protects only `/mcp`. Keep secrets out of logs,
Git, screenshots, and shell history. `docker compose config --quiet` validates
without printing interpolated secrets.

The container runs as UID/GID 10001 with a read-only root, dropped capabilities,
no-new-privileges, bounded temporary storage, and rotated logs. Precreate the bind
mount with correct ownership; Compose does not create it implicitly. A health
check probes `/api/health`. Healthy runtime alone does not establish sync correctness.

The layout follows [STORAGE.md](STORAGE.md): back up both `/opt/nextleaf-app/`
and a consistent copy of `/srv/nextleaf/`. Installation revision 2 sets
`NEXTLEAF_DATA_DIR=/srv/nextleaf` in both the environment template and Compose fallback.
Existing installations using that path need no reinstall; preserve their `.env`.

## Normal updates

1. Stop clients and make a consistent backup with the current version/config.
2. Read the target release's migration and compatibility notes.
3. Download and verify its Compose/template/checksums. Preserve the existing `.env`
   and data path; merge configuration changes rather than replacing secrets.
4. Select the new exact image version or digest in `.env`, then run:

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml config --quiet
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml pull
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml up -d --wait
curl --fail http://127.0.0.1:8787/api/health
```

5. Verify private HTTPS, expected capabilities, and existing data. Update compatible
   Mac clients after the server, then check bidirectional sync and conflicts.

Protocol 3 carries tag identity, repeat rules, and the active timezone. Old sync clients are
rejected rather than silently losing that data. Do not downgrade a migrated
SQLite database in place. Reverting the image alone is not a database rollback.

## Update from 0.1.2 to 0.1.3

This release migrates the database and sync protocol from 3 to 4. Stop desktop
clients and back up both replicas and server configuration consistently. Preserve
pending local edits. Update the server first; old clients cannot sync with protocol 4.
Do not downgrade a migrated database in place.

1. Download the matching `docker-compose.yml` and checksums from
   [v0.1.3](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/v0.1.3).
   Verify the checksum, then replace only `/opt/nextleaf-app/docker-compose.yml`.
2. Keep the existing `.env`, including secrets and data path. Set
   `TASKMANAGER_IMAGE=ghcr.io/silviosackewitz-code/nextleaf:0.1.3` and add
   `NEXTLEAF_HTTP_HOSTS=your-pi.your-tailnet.ts.net` with the actual private hostname
   (no scheme, port or path). The new Compose file passes this setting to the server.
3. Pull and recreate the service:

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml config --quiet
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml pull
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml up -d --wait
curl --fail http://127.0.0.1:8787/api/health
```

4. Check health through private HTTPS too; expect `sync-v4` and `scheduling`.
5. Replace the Mac app using `Nextleaf-0.1.3-macOS-Apple-Silicon.dmg`, retaining
   its existing data folder. Launch it, synchronize and check existing records.
   Reload the iPhone web page. No uninstall or Nirvana reimport is needed.

All missed repeating occurrences are retained. Backlogs above 100 pause until
explicitly approved in Edit/Resume Repeating. This is still an ad-hoc-signed Mac
test release without Developer ID or notarization. See the protocol/host and
restore notes below before any rollback.

## Update from 0.1.1 to 0.1.2

This is a shared UI update for the native Mac app and server-hosted iPhone view.
The database schema and sync protocol remain unchanged at protocol 3. Version
compatibility is checked by the validated sync format and stored server identity;
patch releases do not need identical app version strings to exchange compatible data.
Keep your existing configuration, tasks, and local import receipts.

1. Quit the Mac app, close the iPhone view, and make consistent server and local
   desktop backups as described below.
2. Open the existing Pi configuration with `sudo nano /opt/nextleaf-app/.env` and
   change only its image line:

```dotenv
TASKMANAGER_IMAGE=ghcr.io/silviosackewitz-code/nextleaf:0.1.2
```

3. Validate, pull, and replace the server container:

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml config --quiet &&
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml pull &&
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml up -d --wait
curl --fail --show-error http://127.0.0.1:8787/api/health
```

4. Download `Nextleaf-0.1.2-macOS-Apple-Silicon.dmg` from the
   [0.1.2 release](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/v0.1.2),
   verify its checksum, then drag Nextleaf into Applications and choose Replace.
   Preserve `~/Library/Application Support/dev.taskmanager.desktop/`.
5. Open the installed Mac app and reload the iPhone page. Check existing tasks,
   bidirectional sync, note/metadata alignment, the Focus control, and
   **State → Scheduled → Start Date…**. The State field should show a calendar icon,
   one chevron, and a compact date such as `Scheduled: Oct 19, 2026`.

No uninstall or Nirvana reimport is needed. The Mac remains an ad-hoc-signed,
non-notarized test package. Windows/Intel Mac/Linux desktop installers are not
included; ARM64 and AMD64 Linux server containers are included.

## Update from 0.1.0 to 0.1.1

This release migrates sync protocol 2 to 3. Update the server first, then replace
the Mac app, then reload the iPhone page. During this short mixed-version period,
sync is rejected rather than discarding repeat rules. Do not uninstall or delete
application data. Open the updated app once to report the current system timezone.


This update changes People/Person labels to Contacts/Contact and includes the
transactional Nirvana MCP import tool. Existing data, the server address, storage
paths, and contact IDs are preserved. No reimport is required. Contacts retain
the internal `person` kind; the new repeat fields require sync protocol 3.

1. Quit Nextleaf on the Mac and close the iPhone web app. Make the consistent
   server/config backup described below and retain the local desktop database,
   including its import receipts.
2. On the server, open the existing configuration:

```bash
sudo nano /opt/nextleaf-app/.env
```

Change only the image line to:

```dotenv
TASKMANAGER_IMAGE=ghcr.io/silviosackewitz-code/nextleaf:0.1.1
```

Then run:

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml config --quiet &&
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml pull &&
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml up -d --wait
curl --fail --show-error http://127.0.0.1:8787/api/health
```

3. Download `Nextleaf-0.1.1-macOS-Apple-Silicon.dmg` and verify its checksum from
   the [0.1.1 release](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/v0.1.1).
   With Nextleaf quit, drag the new app into Applications and choose Replace.
   Keep `~/Library/Application Support/dev.taskmanager.desktop` intact. Open the
   installed app; the existing server connection and tasks should remain.
4. Reload the iPhone web page or reopen its Home Screen app. Check **Settings →
   Manage Tags → Contacts**, the Waiting contact choices, and synchronization.

## Consistent backup and restore

Stop the Nextleaf service before copying its complete host data directory, or
use a proper SQLite online backup. Copying only a live main `.db` can omit WAL
transactions. Save the matching Compose file, protected `.env`, image version/
digest, and release source reference. Protect backup secrets like the originals.

A simple stopped-service backup for the agreed data path:

```bash
NEXTLEAF_BACKUP_DIR="/var/backups/nextleaf/$(date -u +%Y%m%dT%H%M%SZ)"
sudo install -d -m 0700 "$NEXTLEAF_BACKUP_DIR"
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml stop
sudo cp -a /srv/nextleaf "$NEXTLEAF_BACKUP_DIR/data"
sudo cp -a /opt/nextleaf-app "$NEXTLEAF_BACKUP_DIR/config"
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml start
```

This creates a local staging backup on the same server; copy it to a separate
backup destination to protect against failure of the server disk. No scheduled
backup or remote storage job is configured by these commands.
Adjust the data path if configured differently. Confirm every copy succeeded and
that the service is healthy afterward. Test restoration into a separate directory
and isolated port using the matching version before relying on a backup. Restore
a consistent full snapshot, not selected tables. A full disaster-recovery exercise
on the owner's Pi has not yet been completed.

## Restoring a backup without rolling clients backward

Before restoring, stop the server and close/suspend every desktop and local MCP
process. Back up each client as well as the current server. Restore into an isolated
location first and verify the matching version, SQLite integrity and contents.
**Never reconnect existing desktops to an older server snapshot with the same
`sync_identity`: unchanged client values could silently adopt the old server values.**

Before exposing the restored server, rotate its identity in the restored database
while it is stopped (substitute the actual database path):

```bash
sudo sqlite3 /srv/nextleaf/taskmanager.db 'BEGIN IMMEDIATE; DELETE FROM sync_receipts; DELETE FROM sync_identity; COMMIT;'
```

The next start/snapshot creates a fresh UUID. Existing clients now stop with an
identity mismatch, preserving their data. Do not reset their baseline merely to
silence the error. Compare the protected client and server backups, reconcile the
intended records explicitly, then establish fresh client databases from the
reconciled server. Preserve local Nirvana receipts separately. This procedure is a
manual safeguard; automatic rollback detection is not implemented.

## Protocol 4 and HTTP host configuration

Back up both replicas before upgrading. The review fixes add a migration and sync
protocol 4; update the server before every desktop. Old clients are rejected before
writes. Do not downgrade a migrated database in place. Large repeat backlogs pause
until approved in Edit/Resume Repeating. Generation stops at 50,000 total entity
records; larger datasets need a different sync design.

Set `NEXTLEAF_HTTP_HOSTS=nextleaf.example.ts.net` to your actual Tailscale hostname
(no scheme, port or path) in the Compose `.env`. It falls back to `NEXTLEAF_MCP_HOSTS`
if unset, and loopback is always allowed. Configure this **before** installing the
new server, even when MCP is disabled, or private HTTPS requests will receive 403.
Serve must forward the original Host. Browser requests must use the same origin;
the development frontend already proxies /api. Arbitrary remote browser API URL
overrides are no longer supported by server CORS.

## Restrict private access explicitly

REST/sync have no application login. Every peer allowed to reach the service can
read and change its data. Tailscale's initial allow-all policy is not an owner-only
boundary. Review grants, ACLs and shared-node access before exposing Serve.

For example, replace the initial broad rule with a grant limited to the owner's
actual login and the Pi's actual Tailscale IP, retaining separately reviewed rules
for your other services:

```json
{
  "grants": [
    { "src": ["owner@example.com"], "dst": ["100.101.102.103"], "ip": ["tcp:443"] }
  ]
}
```

Do not simply append this alongside an allow-all rule: permissions are additive.
Test access from the owner device and rejection from an unapproved device. No
policy was changed or live tailnet access tested by the source review.
See [Tailscale grant examples](https://tailscale.com/docs/reference/examples/grants)
and [grants syntax](https://tailscale.com/docs/reference/syntax/grants).

## Historical local-build Pi releases

The original path uses root `compose.yaml` plus `deploy/.env.example`, locally
built `nextleaf:pi*` images, `/etc/nextleaf/compose.yaml`, and `/var/lib/nextleaf`
on the server. It is separate from the downloadable release path. Do not substitute an update package
for a fresh installer or run both Compose configurations on the same port/data.

| Stage | Purpose / evidence |
| --- | --- |
| pi1 | First ARM64 Docker deployment; health, private HTTPS, imported data, and persistence checked |
| pi2 | Task editing and the initial compact UI round |
| pi3 | Compact navigation, inline task interaction, and quick actions |
| pi4 | Prepared project properties and calendar changes |
| pi5 | Project/MCP functionality and local transport checks |
| pi6 | Offline desktop and initial sync protocol |
| pi7 | Tags/People/archive management and sync protocol 2 |

The owner ran the pi7 updater and reported `NEXTLEAF_UPDATE_OK`; subsequent
health and API checks confirmed tag-management and sync-v2. A disposable database
on the Mac was used for offline/MCP/sync verification. The owner subsequently
removed the old pi7 container, configuration, data, and
pi7 image during the clean-install walkthrough. The owner then completed the new
installation with configuration under
`/opt/nextleaf-app` and the documented `/srv/nextleaf` data path. The screenshot
confirmed a healthy 0.1.0 container, expected API capabilities, and private HTTPS
forwarding. The owner subsequently confirmed that the corrected Mac app opens,
Mac/server task synchronization works, and the iPhone web interface works.
Detailed offline, conflict, mobile-interaction, and restore tests remain separate.

The pi7 update package was staged under `data/update-tags-20261018` locally and
`~/nextleaf-update-tags-20261018` on the Pi. Its backup lives under
`/var/backups/nextleaf/<UTC>-pi7`. Rollback must restore the pre-migration backup
and matching image together. Existing package scripts remain the authority for
that historical update; the new release guide targets an empty installation.

## Building release artifacts

Create a version tag matching package versions. `.github/workflows/containers.yml`
builds native ARM64 and AMD64 images, runs Rust tests plus the disposable Compose
check, and publishes a combined GHCR manifest only after both succeed.
`deploy/check_container.py` checks empty startup, web/API, the available MCP tools, tag/contact
operations, and data/server-identity persistence across container recreation.

Build the Mac package on a Mac:

```bash
npm ci
npm --prefix frontend ci
npm run desktop:build -- --bundles app,dmg
```

The first build targets Apple Silicon and has no Developer ID certificate or
notarization. Bundle LICENSE and THIRD_PARTY_NOTICES in the application and server.
Release the corresponding tagged source, Compose files, and SHA256SUMS alongside
the DMG. Confirm anonymous downloads and public GHCR access independently of an
authenticated build job. See NEXT_STEPS for actual outcomes and pending tests.

## Mac package signature gate

For the test release, Tauri uses `bundle.macOS.signingIdentity: "-"` to ad-hoc sign
the entire bundle. Before publishing any DMG, mount it read-only and run:

```bash
codesign --verify --deep --strict --verbose=2 /Volumes/Nextleaf/Nextleaf.app
```

Use the actual mounted volume path. A checksum and a successful executable launch
are insufficient: the original DMG passed those checks but lacked a valid bundle
resource seal. Verify the app from the final DMG, not only the build directory.
Developer ID and notarization are separate release requirements. Keep quarantine
and Gatekeeper intact during the owner's downloaded-installer acceptance test.
