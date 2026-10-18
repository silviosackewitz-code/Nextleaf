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
| Tags and People | `list_tags`, `create_tag`, `update_tag` |
| Projects | `list_projects`, `create_project`, `update_project` |
| Tasks | `list_tasks`, `get_task`, `create_task`, `update_task` |

These 13 tools cover implemented data operations: project mode/state/Focus/tags/
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
`tag` (default) or `person`. `update_tag` changes name/kind/archive; `update_area`
changes name/archive. Existing links survive archive and rename; new direct
assignments to archived records fail. Duplicate names are not silently merged.
A person rename updates exactly matching old Waiting-for text, preserving other
free text. Old tags are not automatically reclassified as People.

Task REST/MCP payloads retain tag names for compatibility; sync protocol 2 uses
stable IDs to preserve associations. Local v1 baselines migrate at startup.
Update server before clients; incompatible protocol requests fail without mutation.

Perspectives, Review, and manual reordering are not completed features. Add shared
MCP coverage when those features are implemented. Client-specific display and
connection settings are not controlled by the central MCP server.

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
rejection without a token, initialization, all 13 tool names, and a read-only Area
call. It prints neither the secret nor task contents and makes no data changes.
Alternatively supply the token via environment and select an endpoint with `--url`.
Redirects are not followed. Isolated protocol/domain tests and the release container
check are separate from real third-party client acceptance. See NEXT_STEPS.
