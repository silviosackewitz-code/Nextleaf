# Application storage and backup standard

Agreed on 2026-10-18, following the owner's existing Immich directory specification.
Use this separation for Nextleaf and future personal server applications. Existing
applications are not migrated automatically by this policy.

## Common layout

| Location | Purpose | Backup scope |
| --- | --- | --- |
| `/opt/<app>-app/` | Compose file, `.env`, and other deployment configuration | Entire directory; protect secrets |
| `/srv/<app>/` | Persistent databases, uploads, documents, and other application data | Entire required data set, using a consistent database backup |
| Docker-managed internal storage | Images, disposable containers, rebuildable caches | Recreate from configuration and recorded image versions |

Docker normally manages its own storage under `/var/lib/docker`; exact paths depend
on its configuration and storage driver. Do not relocate Docker merely to match the
application layout. Each new application must explicitly inventory its persistent
mounts: if a named Docker volume holds unique data, include that volume in its backup
or place the data under `/srv/<app>` before declaring the app covered.

Downloaded installers, source checkouts, and build caches are not the live database.
The local Nextleaf development checkout remains separate from the Pi installation.

## Nextleaf

```text
/opt/nextleaf-app/
├── docker-compose.yml
└── .env

/srv/nextleaf/
├── taskmanager.db
├── taskmanager.db-wal    (when present)
└── taskmanager.db-shm    (when present)
```

The database retains its existing internal filename. `NEXTLEAF_DATA_DIR=/srv/nextleaf`
binds this host directory to `/data` in the container. UID/GID 10001 owns the data;
configuration is root-owned and `.env` has mode 0600. Specify both the Compose file
and environment file paths when operating the service.

Use the installation revision 2 downloads linked in [INSTALLATION.md](INSTALLATION.md).
The `.env` template and Compose fallback already use `/srv/nextleaf`. Verify their
matching checksums before installation. Older 0.1.0 downloads retain their original
defaults; an existing installation explicitly using `/srv/nextleaf` needs no reset.

## What must be recoverable

1. Complete `/opt/nextleaf-app/`, including `.env` and the selected image version.
2. A consistent copy of complete `/srv/nextleaf/`. Stop Nextleaf before copying,
   or use a proper SQLite online backup. Copying only a live main `.db` can miss WAL
   transactions. See [DEPLOYMENT.md](DEPLOYMENT.md) for the stopped-service commands.
3. The image version/digest and required Tailscale setup information. Keep private
   access credentials protected and restore initially with the matching app version.

Nextleaf has no unique task data inside its container layer or a Docker named volume.
Its images can be downloaded again, so backing up Docker's internal image store is
not part of the Nextleaf data backup. A backup on the same disk is only a staging
copy: independent storage is required for protection against disk failure.

The desktop has a local replica. Server backups contain only changes that have
reached the server. Protect unsynced desktop work through the Mac's own backup
before erasing a local replica. The online iPhone interface has no separate offline
task database to restore.

The Immich reference describes external-disk and encrypted off-site backups as
planned destinations. No automated Nextleaf backup, external-disk job, off-site
upload, or completed restore test is implied by this directory policy.
