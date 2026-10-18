# Architecture

[Requirements](SPECIFICATION.md), [decisions](DECISIONS.md), and
[current verification](NEXT_STEPS.md) are the persistent project handoff.

```text
iPhone / browser -- HTTP --> Pi: Axum + shared Rust domain code --> SQLite
                                           ^
                                           | /api/sync
Desktop React -- Tauri IPC --> embedded Rust domain code --> local SQLite
                                           ^
Agent -------- stdio -------> Nextleaf --mcp
```

## Shared backend and transports

`backend/src/lib.rs` provides `api_router`, `create_pool`, `serve_stdio`, and the
shared data/GTD operations. `main.rs` starts the server. Tauri invokes the Axum
router through Tower inside the process; desktop requires neither a local HTTP
port nor a separate backend sidecar. React chooses Tauri IPC on desktop and HTTP
in the browser. Validation and Sequential availability remain shared.

Both MCP transports use the official `rmcp` SDK and the same 13 operations.
Server `/mcp` requires a Bearer token and allowed host, rejects browser Origin,
and returns 503 when unconfigured. Desktop `--mcp` opens the same local database
and starts the same background sync. It exposes domain data, not general OS
control. [MCP.md](MCP.md) records client limitations and authentication.

## Storage and domain rules

SQLite stores Areas, Projects, Tasks, Tags, links, and reserved saved-view data.
IDs are UUID text; timestamps are RFC3339 text. Ordering is explicit. Availability
and `blocked_by_sequence` are derived, never independent client truth. The first
unfinished non-trash item blocks a Sequential project; only an eligible Next item
at that position is available. Waiting or deferred predecessors are not skipped.

Migrations add imported states (`later`, `recurring`, `trash`), Focus, Waiting,
time/energy, and project completion metadata, then sync and tag management.
Imported projects are Parallel. Import dates preserve day precision, ordering
uses source retrieval order where no explicit order exists, and raw source
metadata preserves information the application cannot interpret.
Recurring templates are visible in Scheduled but do not generate occurrences
without a known recurrence rule. Completion is saved immediately. The frontend
retains recently completed rows temporarily; Logbook sorts by completion date.

Stable tag IDs preserve task/project associations across rename and kind changes.
People use `kind=person`; normal contexts use `kind=tag`. Archive retains existing
links but prevents new direct assignments. Renaming a person updates exactly
matching `waiting_for` text; arbitrary Waiting text remains separate. REST/MCP
continue accepting tag names while the database and sync retain IDs.

## Desktop sync

`backend/src/sync.rs` implements bidirectional snapshot sync with server identity,
per-client baseline, and retry protection. Protocol 2 includes tag IDs and kind/
archive metadata. Old local v1 baselines migrate at startup; pending v1 requests
are reconciled against the retained baseline with a new sequence. Incompatible
requests fail before mutation. Update the server before desktop clients.

Three-way comparison merges independent edits. Same-field changes surface explicit
conflicts; the UI lets the user choose values. Expected-value guards reject stale
resolutions and stale ordinary edits atomically. Protect assignment integrity,
transaction rollback, server identity, and URL binding. Snapshot limits are 16 MiB
and 50,000 records. Do not resolve conflicts by silently replacing a database.

Desktop data lives under `dirs::data_dir()/dev.taskmanager.desktop/nextleaf.db`.
On macOS this is `~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
`NEXTLEAF_DESKTOP_DATA_DIR` selects an isolated test directory. Keep the legacy
identifier for compatibility even though the checkout and product are Nextleaf.

A remote is saved with its database. Startup reconnects only to that remote;
old browser connection storage must not connect a fresh database automatically.
UI and stdio MCP attempt background sync every 15 seconds while running.
Settings shows local path, identity/status/conflicts, and the actual MCP executable.
A web client writes directly to the Pi and requires connectivity.

## Shared UI and responsive behavior

One React frontend serves desktop and web. Native dialog/popover primitives supply
focus containment and dismissal. Desktop titles open accessible inline editors;
Save/Cancel/Escape restore focus. TaskFields and TagEditor share form behavior.
The primary fields are Time, Energy, Due, State, and Project; additional fields
are under More options. Unsaved drafts require an explicit discard choice.

Quick actions share values with the editor and issue minimal PATCH requests.
Desktop submenus open to the left, clamp to the viewport, and use controlled
popover visibility so opening a category by hover does not cause a subsequent
click to dismiss it. Outside click closes the menu chain. Touch and keyboard do
not depend on hover. Errors retain the old value and menu; writes are serialized.
Date controls share a month calendar and preserve date-only values.

Tag text becomes a tag only on Enter or explicit suggestion selection. Blur/Tab/
Save do not create unfinished tags. Project-inherited Area cannot be overridden.
Task rows have separate title/metadata regions, optional notes, modest counters,
and transient completion display. Project grouping and display limits never
change backend availability. Filter/count/overview behavior agrees on Untagged,
including contacts and free-text Waiting values.

Phone layouts use a drawer, floating plus button, full-width task rows, one-column
sheets and tap menus, safe-area spacing, and visual-viewport-aware sizing. Main
touch targets are at least 44px; input text avoids iOS automatic zoom. A shortened
viewport check is useful, but does not prove actual Safari keyboard behavior.

## Server and distribution

Multi-stage Docker builds compile React with Node and Rust with locked dependencies;
Rust tests run during the backend build. The runtime is Debian slim with a non-root
UID/GID 10001, curl health check, web assets, executable, and license notices.
Release Compose drops capabilities, uses a read-only root filesystem and a bounded
`/tmp`, binds only `127.0.0.1:8787`, and mounts a precreated host data directory.
Tailscale Serve terminates private HTTPS on the host. REST/sync currently have no
application login. MCP authentication does not protect those other endpoints.

`compose.yaml` and `deploy/.env.example` support historical local image builds.
`deploy/release/docker-compose.yml` and `example.env` select a published version.
Configuration belongs in `/opt/nextleaf-app`; data belongs in `/srv/nextleaf`.
This follows the owner's shared application layout in [STORAGE.md](STORAGE.md).
Installation revision 2 updates the `.env` template and Compose fallback to
`/srv/nextleaf` and documents configuration under `/opt/nextleaf-app`. Original
0.1.0 release assets remain available; the revision reuses the same image and DMG.
Use one consistent backup with matching version/config; do not downgrade a migrated
SQLite database in place. See [DEPLOYMENT.md](DEPLOYMENT.md).

The release workflow builds natively on AMD64 and ARM64 Linux runners, tests fresh
Compose startup/API/MCP/container recreation, publishes platform tags, then combines
them into a versioned multiarchitecture image. Public package visibility must also
be verified; a public source repository alone does not make a new GHCR package public.
Mac builds use Tauri and include GPL and third-party notices in app resources.
Installation revision 3 sets `bundle.macOS.signingIdentity` to `-` so Tauri signs
the complete app bundle ad hoc. Earlier DMGs only had a linker-signed executable
and failed strict bundle verification. Developer ID and notarization remain absent;
ad-hoc integrity verification is not proof of Gatekeeper acceptance.

## Future boundaries

Review and Perspectives need their own specification round, shared backend rules,
MCP coverage, and sync support. Native iPhone, Windows/Linux installers, automatic
updates, and a public multi-user service are not implicit parts of this release.
