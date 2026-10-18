# Install Nextleaf: Raspberry Pi, Mac, and iPhone

Version: **0.1.1 installation-test prerelease**. Install the server first, then the
native Mac app, then the iPhone web interface. This walkthrough starts with empty
data. It does not import the old test database or Nirvana data.

[Release and downloads](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/v0.1.1)

| Component | Package |
| --- | --- |
| Linux server | `ghcr.io/silviosackewitz-code/nextleaf:0.1.1`, ARM64 and AMD64 |
| Mac | `Nextleaf-0.1.1-macOS-Apple-Silicon.dmg`, Apple Silicon only |
| iPhone | Online web interface served by your Linux server |
| Source | `Nextleaf-0.1.1-source.tar.gz`, GPL-3.0-only |

The Mac DMG is a test build without Apple Developer ID signing or notarization.
Expect a macOS approval step. Intel Mac, Windows, and Linux desktop installers
are not included in this release. The owner has confirmed the basic fresh installation on Pi, Mac, and iPhone,
including Mac/server synchronization. Detailed remaining tests are recorded in
[NEXT_STEPS.md](NEXT_STEPS.md).

## Storage and backup layout

Configuration and persistent data have separate, predictable locations:

| Location | Contents | Back up? |
| --- | --- | --- |
| `/opt/nextleaf-app/` | `docker-compose.yml` and protected `.env` | Yes, the complete directory |
| `/srv/nextleaf/` | SQLite database and any companion files | Yes, consistently; stop Nextleaf before copying |
| Docker-managed storage | Downloaded images and disposable container layers | Not needed to restore Nextleaf |

The container mounts `/srv/nextleaf` as `/data`. Its internal database path remains
`/data/taskmanager.db`. Version 0.1.1 keeps the existing layout and upgrades sync to protocol 3.
Update the server before the Mac app.
[Storage standard and backup scope](STORAGE.md) explains reuse for other applications.

## 1. Prerequisites

On the Pi: 64-bit Ubuntu, Docker Engine with the Compose plugin, `wget`, `curl`,
`sha256sum`, and a connected Tailscale client. The existing test Pi uses Ubuntu
24.04 LTS on ARM64. A Linux AMD64 server can use the same Compose files.

```bash
uname -m
docker --version
docker compose version
tailscale status
```

If needed, follow the [official Docker Ubuntu installation guide](https://docs.docker.com/engine/install/ubuntu/)
and [Compose plugin guide](https://docs.docker.com/compose/install/).
Connect the Mac and iPhone to the same private Tailscale network. The packaged
Mac app does not need Git, Rust, Node.js, or Docker.

Nextleaf has no web login yet. Restrict access to trusted users in your tailnet.
Do not forward the server port from your router or enable Tailscale Funnel.
The optional MCP token protects only MCP, not the web, REST, or sync endpoints.

## 2. Download the server configuration

Use a new download directory. All files below belong to the same release:

```bash
mkdir nextleaf-install-0.1.1
cd nextleaf-install-0.1.1
NEXTLEAF_RELEASE_BASE=https://github.com/silviosackewitz-code/Nextleaf/releases/download/v0.1.1
wget --https-only -O docker-compose.yml "$NEXTLEAF_RELEASE_BASE/docker-compose.yml"
wget --https-only -O example.env "$NEXTLEAF_RELEASE_BASE/example.env"
wget --https-only -O SHA256SUMS "$NEXTLEAF_RELEASE_BASE/SHA256SUMS"
sha256sum --check --ignore-missing SHA256SUMS
```

Continue only when **both** `docker-compose.yml` and `example.env` report `OK`.
Stop if a download is missing or a checksum fails. No GitHub login is needed for
public release downloads and the public container image.

Create the configuration directory. These are first-install commands: do not
replace an existing `.env` during a normal update.

```bash
sudo install -d -m 0750 /opt/nextleaf-app
sudo install -m 0640 docker-compose.yml /opt/nextleaf-app/docker-compose.yml
sudo install -m 0600 example.env /opt/nextleaf-app/.env
```

The template selects image `0.1.1` and `NEXTLEAF_DATA_DIR=/srv/nextleaf` directly.
No path correction or manual environment edit is needed for this first installation.
`NEXTLEAF_MCP_TOKEN` is empty, so MCP starts disabled. Later integration is covered
in [MCP.md](MCP.md). Use the exact private hostname in `NEXTLEAF_MCP_HOSTS` if enabling MCP.

Version 0.1.1 adds Start Date, Make Repeating, travel-aware scheduling, Contacts,
and the one-time Nirvana MCP import tool. It retains the complete ad-hoc bundle signing introduced in
revision 3. Apple Developer ID and notarization remain unavailable.
Existing installations keep their `.env`, data, and server address; use
[normal updates](DEPLOYMENT.md#update-from-010-to-011) instead of resetting.

Create the persistent data directory. Adjust the path if you changed it in `.env`:

```bash
sudo install -d -o 10001 -g 10001 -m 0700 /srv/nextleaf
```

For a fresh installation this directory must be empty. It is outside the container
and survives normal updates. The release Compose file deliberately refuses to
create a missing host directory with the wrong owner.

## 3. Pull and start the server

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml config --quiet
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml pull
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml up -d --wait
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml ps
curl --fail http://127.0.0.1:8787/api/health
```

`pull` downloads the correct architecture; nothing is compiled on the Pi.
Expect a healthy service and JSON containing `status: ok`, `tag-management`,
and `sync-v3` and `scheduling`. If it fails, inspect logs before connecting clients:

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml logs --tail 100
```

## 4. Enable private HTTPS

Inspect existing Tailscale Serve settings first so another service is not replaced.
If the HTTPS endpoint is intended for Nextleaf:

```bash
sudo tailscale serve status
sudo tailscale serve --bg http://127.0.0.1:8787
sudo tailscale serve status
```

Save the private HTTPS address shown by Tailscale. HTTPS certificates must be
allowed in your tailnet; Tailscale guides you through setup when necessary.
[Official Tailscale Serve reference](https://tailscale.com/docs/reference/tailscale-cli/serve)

Open this address in the Mac browser. The lists should be empty. Create an
`Installation check` task and reload: it must remain. Do not restore old data
for this clean-install test.

## 5. Install the native Mac app

**This release is for macOS on Apple Silicon (M-series), not Intel Macs.** In
Apple menu → About This Mac, check whether it says **Chip: Apple M…** or
**Processor: Intel…**. The `.dmg` file is the macOS installer.

On the repository's Code page, open **Releases** in the sidebar, choose
**Nextleaf 0.1.1 — Scheduling and Contacts**, and expand **Assets**. Select
**Nextleaf for macOS — Apple Silicon (M-series) — DMG installer**.
The downloaded file is named `Nextleaf-0.1.1-macOS-Apple-Silicon.dmg`. Source-code archives
are for developers and are not the installer.

1. Download `Nextleaf-0.1.1-macOS-Apple-Silicon.dmg` and `SHA256SUMS` from the release.
   In Terminal, run `shasum -a 256` on the downloaded DMG and compare its full
   hash with that file's entry in `SHA256SUMS`.
2. Open the DMG, drag `Nextleaf.app` into **Applications**, and eject the DMG.
3. Open Nextleaf from Applications. For this ad-hoc-signed test build, macOS may block
   the first launch. After attempting to open it, use **System Settings → Privacy
   & Security → Open Anyway** for Nextleaf and confirm. Only approve the package
   you downloaded from this repository and verified. Do not disable Gatekeeper.
   [Apple's instructions](https://support.apple.com/en-us/102445)
4. In **Settings → Local data and sync**, enter the Pi's private HTTPS address
   and choose **Connect and sync**. Tailscale must be connected.
5. Wait for **Synced** / **Changes are synced.** The browser's test task must appear.
6. Create another task on the Mac. Reload the browser and check that it appears.
   Quit and reopen the installed Mac app to confirm local persistence.

If an earlier `Nextleaf_0.1.0_aarch64.dmg` reports that the app is damaged, cancel
that launch and use the current installer instead. The original files had an
incomplete bundle signature despite matching their download checksums. Do not
remove quarantine attributes or disable Gatekeeper to hide this packaging defect.

For the offline test, disconnect Tailscale on the Mac, edit a test task, and
restart Nextleaf. The local edit must remain. Reconnect Tailscale, then verify
that the Pi and Mac converge and the sync status clears. If both sides edited
the same field, resolve the displayed conflict deliberately.

The usual local database is
`~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
The older internal application identifier is retained for data compatibility.
Settings shows the actual data location. Updating the app normally preserves it.

## 6. Set up the iPhone web interface

1. Connect Tailscale and open the private HTTPS address in Safari.
2. Check the test tasks, open the navigation drawer, and create a task with **+**.
3. Use Safari's **Share → Add to Home Screen**. If offered, enable **Open as Web App**
   and confirm the name `Nextleaf`.
4. Start it from the Home Screen and check loading and saving again.

Menu placement depends on iOS. See [Apple's Home Screen web app guide](https://support.apple.com/en-lamr/guide/iphone/iphea86e5236/ios).
This icon opens the online web app. It does not install a native offline client.

## 7. Check the real iPhone

Repeat in Safari and from the Home Screen:

- Portrait, landscape, scrolling, and the on-screen keyboard: fields and Save/Done/
  Cancel remain reachable, with no horizontally clipped sheets.
- **Settings → Manage Tags**: create an Area, Contact, and Tag, rename them, archive
  them, and restore them using **Show archived → Restore**. Include a long name.
- Open a task, select a tag suggestion, remove a tag/area, save, and reopen.
- Use **State → Waiting**, both with a suggested contact and free text;
  also check the task's **⋯ → Waiting for** action.
- Archive an assigned tag/area: the assignment remains visible, but it is not
  offered for new assignments.
- Cancel an unsaved edit: **Keep editing** retains it; **Discard changes** discards it.
- Check iPhone edits on the Mac and Mac edits on the iPhone after refresh.

Desktop browser checks at iPhone dimensions do not replace these device tests.

## 8. Remove the old test installation before starting over

Only use this section when deliberately deleting the old local test state. For a
normal update, keep your data. First stop all Nextleaf clients and local MCP processes. Otherwise an old desktop replica could
refill an empty server; a recreated server also has a new sync identity.

- **Pi:** Stop the service with its existing Compose configuration. The old pi7
  install uses `/etc/nextleaf/compose.yaml` with data in `/var/lib/nextleaf`;
  installations following this guide use
  `/opt/nextleaf-app/docker-compose.yml`. Pass the matching `.env` and run `down`.
  Remove or move aside only the Nextleaf configuration and actual Nextleaf host
  data directory. `down --volumes` does not clear a bind mount. Select old images
  or update folders individually if removing them; do not run a global Docker prune.
- **iPhone:** Remove the Home Screen web app, close its Safari tabs, and clear
  website data only for the private Nextleaf hostname in Safari settings. This
  does not delete server tasks.

### Remove Nextleaf from a Mac

1. Quit Nextleaf with **Command-Q**. Stop development instances with **Control-C**
   in their Terminal and stop any client that launches `Nextleaf --mcp`.
2. In Finder, move **Applications → Nextleaf.app** to the Trash. Remove any other
   old test copies of the app as well.
3. Press **Shift-Command-G** in Finder and open `~/Library/Application Support/`.
   Move only the **dev.taskmanager.desktop** folder to the Trash. This removes
   local tasks, sync state, and SQLite companion files. If Settings showed a
   custom data location, clear that location instead.
4. Open `~/Library/WebKit/` and `~/Library/Caches/` the same way. In each folder,
   remove **dev.taskmanager.desktop** if it exists, to clear stored UI state.
5. Eject any mounted Nextleaf installer disk image. The downloaded DMG itself is
   optional to keep; do not launch an old app copy before the clean installation.

Removing the app alone leaves the local task database behind. Deleting the folders
above also removes unsynced Mac edits; it does not delete tasks on the server.
Keep your source checkout: it is separate from the installed app and its local data.

Keep Docker, Tailscale, other applications, and the source repository. Start the
new empty Pi first, connect a fresh Mac app next, and add the iPhone last.

## Updates and backup

For normal updates, preserve data and `.env`, make a consistent backup, update
server first, and then install compatible desktop clients. See
[DEPLOYMENT.md](DEPLOYMENT.md) and [verification status](NEXT_STEPS.md).
The root `compose.yaml` is the historical local-build workflow; use
`deploy/release/docker-compose.yml` and release assets for download installations.
