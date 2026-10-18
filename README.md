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

## Install

Follow the [installation guide](docs/INSTALLATION.md) for the complete path:

1. Download the Compose and environment files on your Raspberry Pi or Linux server.
2. Pull the prebuilt container and connect through private Tailscale HTTPS.
3. Install the native Mac app from the DMG and connect it to your server.
4. Open the same private address on your iPhone.

[Release downloads](https://github.com/silviosackewitz-code/Nextleaf/releases/tag/install-v0.1.0-r2)
include versioned configuration, checksums, corresponding source, and the Mac DMG.
The first release is an **installation-test prerelease**, not a production release.
The Mac package is for **Apple Silicon** and is not Developer ID signed or notarized.
Server images target **Linux ARM64 and AMD64**.

The server currently has no web login. Keep it on a trusted private network;
the supplied Compose file binds to loopback and the guide uses Tailscale Serve.
The iPhone web interface requires an available server. It is not an offline app.

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
release 0.1.0, installation revision 2. The environment template already contains
these defaults. Docker images can be downloaded again. See the
[backup instructions](docs/DEPLOYMENT.md#consistent-backup-and-restore) before
copying a running database. For a clean restart, follow the
[removal steps](docs/INSTALLATION.md#8-remove-the-old-test-installation-before-starting-over).

## Features

- Inbox, Next, Waiting, Scheduled, Someday, Focus, Later, Logbook, and Trash.
- Areas and flat projects, with Parallel and Sequential project modes.
- Inline task editing on desktop and touch-friendly sheets on phones.
- Tags and People, archiving and restoration, waiting contacts, dates, time, and energy.
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
