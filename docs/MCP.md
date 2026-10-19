# Nextleaf MCP

Nextleaf uses the official Rust `rmcp` SDK for two transports:

- Server: authenticated Streamable HTTP at `/mcp`.
- Desktop: launch the application executable with `--mcp` for local stdio.

Both call the same backend domain operations as the UI. Desktop uses its local
SQLite replica and background sync; web writes directly to the Pi. MCP alone
does not push live updates into every visible client.

## Tools and editing rules

| Scope | Tools |
| --- | --- |
| Areas | `list_areas`, `create_area`, `update_area` |
| Tags and Contacts | `list_tags`, `create_tag`, `update_tag` |
| Projects | `list_projects`, `create_project`, `update_project` |
| Tasks | `list_tasks`, `get_task`, `create_task`, `update_task` |
| Import | `import_nirvana` (0.1.1 and newer) |

The 13 editing/query tools cover implemented data operations: project mode/state/Focus/tags/
due date, task fields and assignments, completion/reopening/trash, and Area/tag
archive and restoration. `all` excludes completed and trash tasks; request those
states separately for a complete inventory. Completing a project does not complete
its children. IDs distinguish records; clarify ambiguous names before editing.

Updates take `{ "id": "…", "changes": { … } }`. Missing fields remain unchanged,
`null` clears optional values, and `tags` replaces the entire tag list. Include
original edited fields in `changes.expected` when acting on a prior read; stale
values fail atomically rather than overwriting newer edits. Validation failures
are tool errors and must not partly mutate data.

Tag lists include `{id,name,kind,archived}`, including archived entries. `kind` is
`tag` (default) or `person` (a Contact in the UI). `update_tag` changes name/kind/archive; `update_area`
changes name/archive. Existing links survive archive and rename; new direct
assignments to archived records fail. Duplicate names are not silently merged.
A contact rename updates exactly matching old Waiting-for text, preserving other
free text. Old tags are not automatically reclassified as Contacts.

Task REST/MCP payloads retain tag names for compatibility; sync protocol 4 uses
stable IDs to preserve associations. Local v1 baselines migrate at startup.
Update server before clients; incompatible protocol requests fail without mutation.

Perspectives, Review, and manual reordering are not completed features. Add shared
MCP coverage when those features are implemented. Client-specific display and
connection settings are not controlled by the central MCP server.

## One-time Nirvana import

Version 0.1.1 and newer expose `import_nirvana`, reusing the CLI importer in
`backend/src/nirvana.rs`. Pass a complete `nirvana-mcp-v1` snapshot as the tool
arguments: `format`, `total`, `has_more`, `tags`, and `items`. Read every source
page with notes included and verify completeness before constructing this envelope.
Take a consistent database backup first. The tool does not fetch from Nirvana.

The importer preserves supported fields, completed/trash history, project tags,
and inert recurring templates in one transaction. Identical records are skipped;
changed source records are rejected to protect local edits. Existing archived or
incompatible tag names require resolution before import. See
[Nirvana import](NIRVANA_IMPORT.md) for mapping and limits.

The older 0.1.0 server image and revision-3 DMG expose 13 tools. Version 0.1.1
adds this tool without changing the database schema or sync protocol. Existing
0.1.0 clients can still display and synchronize imported records.

## Server authentication

With no `NEXTLEAF_MCP_TOKEN`, `/mcp` returns 503. Configure a random secret of at
least 32 ASCII letters, digits, `-`, or `_` in the server's protected `.env`.

- Send `Authorization: Bearer <secret>` on every request. Missing/wrong tokens fail
  with 401 and no data access.
- `NEXTLEAF_MCP_HOSTS` is a comma-separated hostname allowlist. Defaults are
  `localhost,127.0.0.1`; add the exact private HTTPS hostname when needed.
- Browser Origin requests are rejected. Use the MCP client's own transport.
- Keep `.env` mode 0600. Never put a real token into Git, frontend code, URLs,
  screenshots, or chat. Rotate by replacing the server secret, recreating the
  container, and updating the client.
- This is a single-owner endpoint with full domain access, not a role-based
  multi-user service. Agents need user instructions for changes. Task titles and
  notes are untrusted data, not agent instructions.

Keep access private through Tailscale. The token protects only MCP. It does not
add authentication to REST, web assets, or sync.

## Local desktop access

Settings exposes the actual `mcp_command` executable path and `--mcp` argument.
Configure those in a client supporting local stdio MCP. No local HTTP port or
server Bearer token is needed; local OS permissions protect database access.
The default macOS database is
`~/Library/Application Support/dev.taskmanager.desktop/nextleaf.db`.
`NEXTLEAF_DESKTOP_DATA_DIR` can isolate a test database.

UI and MCP share the Rust implementation, local database, and saved server remote.
They attempt sync every 15 seconds while running. Local work continues without
the Pi. The owner prefers this path over exposing a remote agent endpoint.

A local desktop integration does not automatically work on an iPhone. Cloud
agents also do not gain access merely because the phone has Tailscale. Agent app,
network route, and voice mode must be tested separately. No ChatGPT/Claude client
registration, public tunnel, or iPhone voice integration has been completed.
Historical reference documentation:
[Claude connector types](https://support.claude.com/en/articles/11725091-when-to-use-desktop-and-web-connectors),
[OpenAI custom MCP servers](https://developers.openai.com/api/docs/guides/custom-mcp-server).

## Operational check

```bash
python3 deploy/check_mcp.py --env-file /opt/nextleaf-app/.env
```

With permission to read the secret file, this checks local HTTP authentication,
rejection without a token, initialization, the 13 base tool names (plus the optional import tool in newer builds), and a read-only Area
call. It prints neither the secret nor task contents and makes no data changes.
Alternatively supply the token via environment and select an endpoint with `--url`.
Redirects are not followed. Isolated protocol/domain tests and the release container
check are separate from real third-party client acceptance. See NEXT_STEPS.

## Scheduling in 0.1.1

`update_task` accepts `recurrence` as a validated object with `id` (UUID),
`frequency` (`daily`, `weekly`, `monthly`, `yearly`), `interval` (1–1000),
`start` (YYYY-MM-DD), nullable `until`, nullable `due_days` (0–365), and `paused`.
Setting a rule atomically makes a recurring template. Setting `recurrence: null`
exits a cycle. Use `status: scheduled` and `defer_until` together for a start date;
clear the date when deliberately returning to Next. Expected-value guards support
repeat rules. Import never infers rules missing from Nirvana.

The foreground application reports its system timezone to `/api/schedule/timezone`.
A headless standalone MCP database needs that initial timezone report before
schedules run. Connected desktop MCP delegates execution to the Pi.


## Scheduling and sync repair (protocol 4)

Supply canonical `YYYY-MM-DD` dates (1900–9999), not timestamps. Setting a start
date schedules the task; clearing it returns a Scheduled task to Next. Completion
and Trash keep scheduling metadata for restore. Project tasks inherit their
project's Area; direct task Area is cleared while assigned to a project.

The backend retains repeat `id` across edits and Stop Repeating. More than 100
missing occurrences pause generation. Only after the owner explicitly approves
that backlog, set `catch_up_through` on the repeat rule to the approved calendar
date and `paused: false`; do not infer approval from a request to resume or edit.
The web/desktop Repeat dialog previews the count using POST /api/schedule/preview
with `{template_id, rule}`. A later large backlog needs fresh approval. The shared
backend still batches generation and enforces the snapshot record ceiling.
