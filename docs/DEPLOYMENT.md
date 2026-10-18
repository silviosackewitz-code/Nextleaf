# Deployment and backup

For a fresh installation use [INSTALLATION.md](INSTALLATION.md). This document
covers runtime layout, normal updates, backups, and the historical Pi test path.

## Runtime layout

| Location | Purpose |
| --- | --- |
| `/etc/nextleaf/docker-compose.yml` | Download-release Compose configuration |
| `/etc/nextleaf/.env` | Image version, host data path, optional MCP token; mode 0600 |
| `/var/lib/nextleaf` | Persistent SQLite database and companion files; UID/GID 10001 |
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

## Normal updates

1. Stop clients and make a consistent backup with the current version/config.
2. Read the target release's migration and compatibility notes.
3. Download and verify its Compose/template/checksums. Preserve the existing `.env`
   and data path; merge configuration changes rather than replacing secrets.
4. Select the new exact image version or digest in `.env`, then run:

```bash
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml config --quiet
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml pull
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml up -d --wait
curl --fail http://127.0.0.1:8787/api/health
```

5. Verify private HTTPS, expected capabilities, and existing data. Update compatible
   Mac clients after the server, then check bidirectional sync and conflicts.

Protocol 2 carries tag identity and archive/kind state. Old sync clients are
rejected rather than silently losing that data. Do not downgrade a migrated
SQLite database in place. Reverting the image alone is not a database rollback.

## Consistent backup and restore

Stop the Nextleaf service before copying its complete host data directory, or
use a proper SQLite online backup. Copying only a live main `.db` can omit WAL
transactions. Save the matching Compose file, protected `.env`, image version/
digest, and release source reference. Protect backup secrets like the originals.

A simple stopped-service backup for the default data path:

```bash
NEXTLEAF_BACKUP_DIR="/var/backups/nextleaf/$(date -u +%Y%m%dT%H%M%SZ)"
sudo install -d -m 0700 "$NEXTLEAF_BACKUP_DIR"
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml stop
sudo cp -a /var/lib/nextleaf "$NEXTLEAF_BACKUP_DIR/data"
sudo cp -a /etc/nextleaf "$NEXTLEAF_BACKUP_DIR/config"
sudo docker compose --env-file /etc/nextleaf/.env -f /etc/nextleaf/docker-compose.yml start
```

Adjust the data path if configured differently. Confirm every copy succeeded and
that the service is healthy afterward. Test restoration into a separate directory
and isolated port using the matching version before relying on a backup. Restore
a consistent full snapshot, not selected tables. A full disaster-recovery exercise
on the owner's Pi has not yet been completed.

## Historical local-build Pi releases

The original path uses root `compose.yaml` plus `deploy/.env.example`, locally
built `nextleaf:pi*` images, and `/etc/nextleaf/compose.yaml` on the server. It is
separate from the downloadable release path. Do not substitute an update package
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
on the Mac was used for offline/MCP/sync verification. The current release work
has not reset or replaced the owner's Pi installation.

The pi7 update package was staged under `data/update-tags-20261018` locally and
`~/nextleaf-update-tags-20261018` on the Pi. Its backup lives under
`/var/backups/nextleaf/<UTC>-pi7`. Rollback must restore the pre-migration backup
and matching image together. Existing package scripts remain the authority for
that historical update; the new release guide targets an empty installation.

## Building release artifacts

Create a version tag matching package versions. `.github/workflows/containers.yml`
builds native ARM64 and AMD64 images, runs Rust tests plus the disposable Compose
check, and publishes a combined GHCR manifest only after both succeed.
`deploy/check_container.py` checks empty startup, web/API, 13 MCP tools, tag/person
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
