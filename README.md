# Nextleaf

**A self-hosted GTD task manager with a native Mac app and an iPhone web interface.**

Keep your tasks on your own server. Use the Mac app with a local database, work
when the server is unavailable, and sync when you reconnect. On iPhone, use the
responsive web interface in Safari or add it to your Home Screen.

Nextleaf follows Getting Things Done (GTD): capture ideas in Inbox, organize them
into projects and areas, and choose what to do next using tags, dates, time, and
energy. Run the server with Docker Compose on a Raspberry Pi or Linux machine;
install the native macOS client from a DMG and use Safari on iPhone.

Nextleaf is open source under the **GNU GPL v3.0**.

Already installed? Follow [Update from 0.1.1 to 0.1.2](docs/DEPLOYMENT.md#update-from-011-to-012).
The server container and Mac app are replaced while your data stays in place.

## Install Nextleaf

Start with the Linux server, then install the Mac app, then open Nextleaf on iPhone.
The commands below are for a **fresh installation** of application 0.1.2. The release is an **installation-test prerelease**.
For removal, updates, backups, and detailed verification, use the
[complete installation guide](docs/INSTALLATION.md).

### 1. Linux server: Raspberry Pi or AMD64

You need a 64-bit Linux server with Docker Engine, the Docker Compose plugin,
`wget`, `curl`, `sha256sum`, and Tailscale already installed and connected.
The server images support **Linux ARM64** (including Raspberry Pi) and **AMD64**.

Run each block on your server, in order. Stop if any command fails.

**Download and verify the installation files:**

```bash
mkdir -p ~/nextleaf-install-0.1.2 &&
cd ~/nextleaf-install-0.1.2 &&
wget --https-only -O docker-compose.yml https://github.com/silviosackewitz-code/Nextleaf/releases/download/v0.1.2/docker-compose.yml &&
wget --https-only -O example.env https://github.com/silviosackewitz-code/Nextleaf/releases/download/v0.1.2/example.env &&
wget --https-only -O SHA256SUMS https://github.com/silviosackewitz-code/Nextleaf/releases/download/v0.1.2/SHA256SUMS &&
sha256sum --check --ignore-missing SHA256SUMS
```

Continue only when both `docker-compose.yml` and `example.env` report **OK**.

**Create the configuration and empty data directories:**

```bash
cd ~/nextleaf-install-0.1.2 &&
sudo install -d -m 0750 /opt/nextleaf-app &&
sudo install -m 0640 docker-compose.yml /opt/nextleaf-app/docker-compose.yml &&
sudo install -m 0600 example.env /opt/nextleaf-app/.env &&
sudo install -d -o 10001 -g 10001 -m 0700 /srv/nextleaf
```

These are first-install commands. Do not replace an existing `.env` during an
update, and start with an empty data directory for a clean-install test. The
template already sets `NEXTLEAF_DATA_DIR=/srv/nextleaf`; no manual edit is needed.

**Pull the image and start Nextleaf:**

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml config --quiet &&
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml pull &&
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml up -d --wait
```

**Check the running server:**

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml ps
curl --fail --show-error http://127.0.0.1:8787/api/health
```

Expect **healthy** and JSON with `"status":"ok"`. The image is prebuilt; no source
compilation is needed. For troubleshooting:

```bash
sudo docker compose --env-file /opt/nextleaf-app/.env -f /opt/nextleaf-app/docker-compose.yml logs --tail 100
```

**Connect through private HTTPS:**

First inspect the existing proxy configuration:

```bash
sudo tailscale serve status
```

If the HTTPS endpoint is available for Nextleaf, configure it as follows. If it
already proxies to `http://127.0.0.1:8787`, no change is needed. Do not overwrite
another application's proxy; see the [full guide](docs/INSTALLATION.md#4-enable-private-https).

```bash
sudo tailscale serve --bg http://127.0.0.1:8787
sudo tailscale serve status
```

Open the HTTPS address displayed by Tailscale from a connected Mac or iPhone.
Nextleaf currently has **no web login**: only trusted users in your private tailnet
should have access. Do not expose the port publicly or enable Tailscale Funnel.
Server MCP starts disabled until you configure a token; ordinary web access and
Mac synchronization do not depend on it.

### 2. Mac: download the macOS installer

**Choose “Nextleaf for macOS — Apple Silicon (M-series) — DMG installer” under
Releases → Nextleaf 0.1.2 — UI refinements → Assets.**

The downloaded filename is `Nextleaf-0.1.2-macOS-Apple-Silicon.dmg`.
It is for **macOS on Apple Silicon (M-series)**.

| Your Mac, shown under Apple menu → About This Mac | Download |
| --- | --- |
| **Chip: Apple M…** (Apple Silicon / M-series) | `Nextleaf-0.1.2-macOS-Apple-Silicon.dmg` |
| **Processor: Intel…** | No compatible installer in this release |

On this repository's **Code** page, find **Releases** in the sidebar, open the
release, and expand **Assets**. Choose the DMG above; **Source code** is for
building the app yourself. If the sidebar is hidden on a narrow screen, use
[the releases page](https://github.com/silviosackewitz-code/Nextleaf/releases).

Verify the DMG against the release's `SHA256SUMS`, open it, drag **Nextleaf** into
**Applications**, and eject the disk image. Open the installed app and enter your
private server address under **Settings → Local data and sync → Connect and sync**.

Revision 3 fixes an incomplete app-bundle signature in the earlier Mac installers.
The bundle now has a complete ad-hoc test signature. It remains **without Apple
Developer ID signing or notarization**.
If macOS blocks the first launch, follow the [Mac installation steps](docs/INSTALLATION.md#5-install-the-native-mac-app).
Do not disable Gatekeeper globally.

### 3. iPhone: open the web interface

Connect Tailscale, open the same private HTTPS address in Safari, then optionally
choose **Share → Add to Home Screen**. This is an **online web interface** and
requires your server to be available; it is not a native offline iPhone app.

## How Nextleaf fits into your day

Capture a task in Inbox, give it a project or area when you are ready, and use
Next to see available actions. Parallel projects let you choose among actions;
Sequential projects keep the first unfinished action in front. Waiting helps you
track follow-ups, and Focus highlights the tasks you want to keep in sight.

Use the native Mac app for daily work, including when your server is unreachable.
It stores tasks locally and synchronizes after you reconnect. Conflicting edits
are shown for you to resolve. On iPhone, the web interface connects directly to
your server; add it to your Home Screen for convenient access.

## Choose your setup

| Component | What you need | What it does |
| --- | --- | --- |
| Server | Raspberry Pi with 64-bit Linux, or an AMD64 Linux server; Docker Compose | Stores shared data and serves the web interface |
| Mac | Apple Silicon Mac; downloadable DMG | Native desktop client with local storage and offline edits |
| iPhone | Safari and a connection to your private server | Responsive online web interface; optional Home Screen icon |
| Private connection | Tailscale on the server and clients | HTTPS access within your trusted tailnet |

Nextleaf currently targets a personal, trusted setup. There are no user accounts,
team permissions, or public web login. Host it privately. A native iPhone app,
offline phone editing, and automatic desktop updates are not part of this release.

## Where your data lives

| Server folder | Contents | Backup |
| --- | --- | --- |
| `/opt/nextleaf-app/` | Docker Compose configuration and `.env` | Entire folder, including secrets |
| `/srv/nextleaf/` | Persistent SQLite task database | Consistent copy of the entire folder |

The [installation guide](docs/INSTALLATION.md) sets these paths explicitly for
release 0.1.2. The environment template already contains
these defaults. Docker images can be downloaded again. See the
[backup instructions](docs/DEPLOYMENT.md#consistent-backup-and-restore) before
copying a running database. For a clean restart, follow the
[removal steps](docs/INSTALLATION.md#8-remove-the-old-test-installation-before-starting-over).

## Features

- Inbox, Next, Waiting, Scheduled, Someday, Focus, Later, Logbook, and Trash.
- Areas and flat projects, with Parallel and Sequential project modes.
- Inline task editing on desktop and touch-friendly sheets on phones.
- Tags and Contacts, archiving and restoration, waiting contacts, dates, time, and energy.
- Project grouping, contextual filters, note display options, and a project overview.
- Local desktop storage and bidirectional sync with explicit conflict resolution.
- MCP access through the local desktop app or an authenticated server endpoint.

Saved Perspectives, guided Review, native iPhone support, and Windows/Linux desktop
installers remain future work. See [current status](docs/NEXT_STEPS.md).

## Feedback

This is an early installation-test release. Reports from real installations help
identify what needs fixing. [Open an issue](https://github.com/silviosackewitz-code/Nextleaf/issues)
with your OS, device architecture, Nextleaf version, reproduction steps, and the
expected and actual result. Remove tokens, private task content, and other secrets
from logs or screenshots before posting.

Want to help without writing code? Try one device and share your result:

- [UI testing on Mac and iPhone](https://github.com/silviosackewitz-code/Nextleaf/issues/1)
- [Installation testing](https://github.com/silviosackewitz-code/Nextleaf/issues/2)

## Develop

Install Rust and Node.js, plus the [Tauri platform prerequisites](https://v2.tauri.app/start/prerequisites/).
The checkout folder is `Nextleaf`; its location is not a technical requirement.

```bash
git clone https://github.com/silviosackewitz-code/Nextleaf.git
cd Nextleaf
npm ci
npm --prefix frontend ci
npm run desktop:dev
```

The desktop embeds the backend and creates its own local database. For web development,
start the server and frontend in separate terminals:

```bash
cd backend
cargo run --locked
```

```bash
npm run web:dev
```

Run the frontend command from the repository root. The backend defaults to paths
relative to `backend/`; run it there unless configuring explicit paths.
The Vite proxy targets the local backend. Never commit the generated database or `.env`.

```bash
cargo test --locked --manifest-path backend/Cargo.toml
node --experimental-strip-types --test frontend/tests/*.test.ts
npm run web:build
npm run desktop:build -- --bundles app,dmg
```

Building a desktop bundle is platform-specific. A successful local build does not
establish signing, notarization, or compatibility on untested platforms.

## Documentation

- [Installation](docs/INSTALLATION.md), [storage and backup scope](docs/STORAGE.md),
  and [deployment/backup](docs/DEPLOYMENT.md)
- [Requirements](docs/SPECIFICATION.md) and [architecture](docs/ARCHITECTURE.md)
- [Decisions](docs/DECISIONS.md) and [current status](docs/NEXT_STEPS.md)
- [MCP integration](docs/MCP.md) and [one-time Nirvana import](docs/NIRVANA_IMPORT.md)
- [UI reference](docs/UX_REFERENCE.md), [future Review research](docs/REVIEW_RESEARCH.md),
  and [name research](docs/NAMING.md)

## License

Copyright (C) 2026 Silvio Sackewitz. Nextleaf is licensed under
[GNU General Public License version 3 only](LICENSE), SPDX `GPL-3.0-only`.
Third-party components retain their own licenses; see [third-party notices](THIRD_PARTY_NOTICES.txt).
Release source and build instructions are provided alongside the binaries.
