# Code review — 2026-10-19

Follow-up: [verification and repair results](../docs/REVIEW_RESOLUTION.md).
The original findings below are retained unchanged for traceability.

Read-only multi-agent review of `main` at `e0fbdb3` (plus uncommitted doc edits).
Five reviewers covered sync, security, GTD domain/API, scheduling/import, and
frontend/desktop. No source files were changed by the review.

## How to work through this

- Follow `AGENTS.md`: read SPECIFICATION, ARCHITECTURE, DECISIONS, NEXT_STEPS first;
  record decisions in DECISIONS and verification in NEXT_STEPS.
- Each finding lists location, failure scenario, and a suggested direction. The
  suggestions are not agreed decisions; check them against the specification.
- "Reproduced" means a reviewer ran a throwaway test against a scratch copy of
  `backend/`. "Code-read" means traced by reading only. Items marked "spot-checked"
  were re-read in the source by the coordinating reviewer.
- Add a regression test for every fix, especially the reproduced ones.

## Baseline verification (2026-10-19, macOS)

- `cargo test --locked` in `backend/`: 37 passed, 1 ignored.
- `npx tsc --noEmit` in `frontend/`: clean.
- `node --test --experimental-strip-types tests/*.test.ts` in `frontend/`: 42/42 passed.
- No platform or installation tests were performed.

---

## P1 — data loss, duplication, or permanently stuck sync

### 1. Repeat generation backfills every date since `start` (occurrence storm)
- **Where:** `backend/src/schedule.rs:215-243`; `RepeatRule::validate` (`schedule.rs:36`) accepts any start ≥ 1900.
- **Problem:** `advance` scans from occurrence index 0 and creates every missing date up
  to today. `remaining = 100` resets per call, and `advance` runs from the 30 s worker,
  every `fetch_tasks` (`lib.rs:808`), every sync GET/exchange, and timezone reports.
- **Scenario (reproduced):** yearly rule starting 1960-05-03 → 67 starred Next copies in
  one pass. Daily rule from 2000-01-01 → ~9,800 tasks at 100 per pass. A daily rule from
  1900 exceeds the 50,000-record sync limit and stops sync. Also triggered by moving a
  first date backwards, resuming after a long pause, server downtime, or adding a rule
  to an imported template (RepeatDialog pre-fills the old Nirvana `defer_until`,
  `frontend/src/StateField.tsx:97/102`).
- **Direction:** persist a per-template cursor (last generated occurrence) and only
  generate from there; on rule creation/edit, start from today (or the next future
  occurrence), not from history. Decide and document catch-up semantics after downtime
  (e.g. at most the latest missed occurrence for daily/weekly rules).

### 2. Changing `rule.id` duplicates every past occurrence
- **Where:** `backend/src/lib.rs:955-961` (update_task), `schedule.rs:227-233`, sync validate (`sync.rs:414`).
- **Problem:** occurrence ID = `v5(template:rule.id:day)` and the backend trusts any
  client-supplied `rule.id`.
- **Scenario (reproduced):** weekly template with 7 copies → 14 after resubmitting the same
  rule with a new id. Triggers: an MCP agent sending a fresh UUID (docs/MCP.md does not
  say to keep `id`); UI "Stop Repeating" then "Make Repeating" mints `crypto.randomUUID()`.
- **Direction:** the server owns the cycle identity — keep the existing `rule.id` on edit,
  or derive occurrence IDs from template + date only. Fix together with #1.

### 3. A rejected sync request is frozen and retried forever
- **Where:** `backend/src/sync.rs:747-765` (existing `inflight` reused, cleared only in
  `finish`), `response` at `sync.rs:782-786`.
- **Problem:** any HTTP 400/413 leaves the same frozen `local` snapshot and sequence in
  place; later local edits cannot fix it. The client shows a generic
  "Check that the server supports offline sync" and drops the server's message.
- **Scenario (reproduced, incl. over HTTP):** client A renames tag X to "Shopping" while B
  created a new "Shopping" → merged validate fails "Duplicate tag name". After A renames
  to "Shopping 2", retries still fail. Same with a recurrence/status merge
  ("A repeat rule requires Recurring state"). Code-read triggers: restored desktop DB with
  older sequence ("Stale sync request rejected", `sync.rs:612`), note > 1,000,000 bytes
  (REST allows it, sync `validate` at `sync.rs:431` rejects it), request > 16 MiB
  (base + local → effectively ~8 MiB of data).
- **Direction:** distinguish retryable (network/5xx) from definitive rejections (4xx with
  a body). On a definitive rejection, drop the inflight request and rebuild from current
  local data with a new sequence; surface the server's error text. Align REST and sync
  validation limits. Consider sending only changed records instead of base + local.

### 4. Tag rename + new tag with the old name merges two tags and blocks sync
- **Where:** `tag_remaps` `sync.rs:258-273` (spot-checked), `remap_tag` `:307-311`,
  `finish` `:820-831`, `rekey_tag` `:283-285`.
- **Problem:** a local tag ID missing from base and remote is mapped to any remote tag with
  the same name, even when that remote tag is established (present in base).
- **Scenario (reproduced):** desktop has R "Foo"; user renames R → "Bar", creates new L
  "Foo", assigns it to T2. Server ends with one tag R "Foo"; the rename is lost and T1
  and T2 both point to R. Client then fails "Tag identity collision" and every retry
  replays the same receipt — sync is dead.
- **Direction:** only remap onto remote tags that are themselves new (absent from base);
  otherwise reject as a name conflict the user can resolve. Must be fixed with #3 so the
  client can recover.

### 5. Any non-recurring status permanently deletes the repeat rule
- **Where:** `backend/src/lib.rs:1012-1014` (spot-checked); `lib.rs:963-968` rejects
  `status: recurring` without a rule.
- **Scenario:** Trash or Complete a repeating template (allowed by StateField, quick menu
  `frontend/src/quickMenu.ts:10`, and MCP). The rule is wiped. Restore sends
  `{status: "recurring"}` (`App.tsx:179`) → 400. Imported inert templates can never
  return to Recurring.
- **Direction:** keep `recurrence` on trash/complete (only "Stop Repeating" clears it), or
  forbid completing a template and make trash/restore preserve the rule.

### 6. Open editor draft is silently discarded
- **Where:** `frontend/src/App.tsx:350` (inline, `!mobile`) vs `App.tsx:369-370` (mobile
  sheet); projects at `App.tsx:328/372`; draft state in `TaskDetails.tsx:10-14`
  (spot-checked); `useMobile.ts:3`.
- **Scenarios:**
  - Crossing the mobile breakpoint (window < 720px — Tauri `minWidth` is 680 — iPad
    rotation, Split View) unmounts the inline editor and mounts a fresh sheet; edits are
    lost without the discard prompt.
  - While editing task A, starring/quick-editing task B (`App.tsx:345/362`) or
    `DesktopConnection.onChanged` (`App.tsx:317`) reloads without `canLeaveEditor`. If
    sync moved/completed A, its row unmounts and the draft is lost.
- **Direction:** lift draft state out of the row (keyed by task id in App or a store), so
  layout switches and reloads keep it; gate every reload path consistently.

### 7. Undoing a State-menu completion leaves Scheduled tasks without a date
- **Where:** `frontend/src/scheduling.ts:6-8` (spot-checked) clears `defer_until` and
  `recurrence` on `stateChange('completed')`; restore at `App.tsx:165-179` only re-sends
  `{status: previousStatus}`.
- **Scenario:** complete a task dated Oct 25 via State → Completed, then click the
  checkbox to undo → `scheduled` with `defer_until = NULL`; the scheduler
  (`schedule.rs:197`) never activates it. Checkbox completion does not have this problem.
- **Direction:** don't clear dates on completion, or store and restore the previous
  `defer_until`/`recurrence` with `previousStatus`. Combine with #10 (backend invariant).

---

## P2 — wrong behavior, robustness, security hardening

### 8. Task-level Area overrides the project's Area
- **Where:** `backend/src/lib.rs:785` `task.area_id.or(task.project_area_id)` (spot-checked);
  `create_task` `lib.rs:904/921-923` and `update_task` `lib.rs:1025/1036` store both;
  Nirvana import `nirvana.rs:243/266`.
- **Scenario:** project P in Area B; task T in P with `area_id` A (from import, MCP, or
  `create_task {project_id, area_id}`). `GET /api/tasks?area_id=B` omits T, the UI
  (`TaskFields.tsx:16`) shows B, and moving P never carries T. Violates "project-inherited
  Area cannot be overridden".
- **Direction:** prefer `project_area_id`; reject/clear `area_id` when `project_id` is set;
  add a migration that nulls `area_id` on project tasks (check imported data first).

### 9. Lenient and timestamp dates compare wrongly as strings
- **Where:** `update_task` `lib.rs:983-993` (spot-checked) and `update_project`
  `lib.rs:404-408` use chrono's lenient parse plus RFC3339; `create_task` uses strict
  `schedule::date`. Comparisons: `lib.rs:776-779`, `schedule.rs:197` (`substr(...,1,10)`).
- **Scenarios:** `defer_until: "2026-10-5"` activates on 2026-11-01 (string order);
  `" 2026-12-24"` activates immediately. `2026-10-19T22:00:00Z` (Berlin midnight Oct 20)
  activates on Oct 19 — reproduced. Same for `due_date` overdue checks.
- **Direction:** normalize to canonical `YYYY-MM-DD` on every write path (REST, MCP, sync,
  import); convert timestamps into the clock's timezone before truncating, or reject them.

### 10. Backend does not tie Scheduled state to a start date
- **Where:** `create_task`/`update_task` set `status` and `defer_until` independently;
  only the frontend enforces the rule.
- **Scenario:** MCP `update_task {defer_until: "2026-12-01"}` on a Next task → hidden from
  Next and absent from Scheduled (`lib.rs:838`) until December. `{defer_until: null}` on a
  Scheduled task → never activates.
- **Direction:** enforce in the shared backend: a future `defer_until` implies `scheduled`;
  clearing it on a scheduled task returns it to `next`.

### 11. Server SQLite: no WAL, 5 s busy timeout, and every read takes the write lock
- **Where:** server pool `lib.rs:1139-1146` (spot-checked; desktop `create_pool` sets WAL
  and 15 s). `fetch_tasks` (`lib.rs:808`) calls `schedule::tick` → `BEGIN IMMEDIATE`,
  template scan, and `SELECT id FROM tasks` (all IDs) on every list/get/after every write.
- **Scenario:** a sync exchange holding the lock > 5 s on the Pi makes concurrent REST/MCP
  reads 500. Worse, `update_task` commits, then the follow-up `fetch_tasks` fails → client
  sees 500 for a saved write, retries with stale `expected` → 409.
- **Direction:** reuse `create_pool` options on the server (WAL, busy timeout); run
  `tick` only from the worker and explicit triggers, not on reads; don't fail a committed
  write because the post-write read failed.

### 12. Coupled fields merged independently → unresolvable conflicts / broken invariants
- **Where:** `merge` `sync.rs:209-239`, `resolve_conflicts` `sync.rs:880-881`.
- **Scenarios:** desktop edits rule R→R2 while server sets Next → status merges, only
  `recurrence` conflicts, and "keep local" always fails (reproduced). Because the choice
  applies to all conflicts at once, the user can never keep local values for any other
  conflict either. Desktop moves task to area A while server moves it to project Q →
  choosing "server" yields project Q + area A (code-read).
- **Direction:** treat `status`/`defer_until`/`recurrence` and `project_id`/`area_id` as
  merge units; allow per-conflict resolution; run full assignment validation after merge.

### 13. Server restore from backup silently rolls back newer client edits
- **Where:** `merge` `sync.rs:221-226`, baseline in `finish` `sync.rs:835`; restore steps
  in `docs/DEPLOYMENT.md:137-162`.
- **Scenario (reproduced):** backup has title "Original"; desktop edits to "Newer" and
  syncs; Pi restored (same `sync_identity`). Client local == baseline, so it adopts
  "Original". Only records created after the backup are detected.
- **Direction:** add a server generation/epoch that changes on restore and is checked
  against the client baseline; until then, document that restore must rotate identity or
  that clients must re-baseline.

### 14. "Connected desktop" flag is set before any successful contact and never cleared
- **Where:** `schedule.rs:183-191`, `sync.rs:704`.
- **Scenario:** a standalone Mac user tries "Connect and sync" once with a wrong URL; that
  database never activates start dates or generates repeats again, and there is no
  disconnect in the UI. If a Pi database ever carries a `sync_client.remote_url` row (e.g.
  from a desktop-made DB), the Pi silently stops scheduling.
- **Direction:** persist `remote_url` only after the first successful exchange; add
  Disconnect; decide server vs desktop role by process, not by data.

### 15. Waiting free text is not committed on blur
- **Where:** `frontend/src/StateField.tsx:88-90` (Enter / "Use contact" only), light-dismiss
  at `StateField.tsx:68`. Spec: "Free text commits on Enter or blur."
- **Scenario:** type a contact, click Save changes → other edits save, Waiting/contact do not.
- **Direction:** commit on blur and on outside dismiss when the field is non-empty.

### 16. Open editor never refreshes, leading to repeated 409s
- **Where:** `frontend/src/TaskDetails.tsx:10-11` freezes `original`/`draft` at open.
- **Scenario:** task open on desktop, phone edits notes, sync brings it in; editor shows old
  notes; any save sends stale `expected` → 409 every time; only Cancel escapes.
- **Direction:** when not dirty, rebase draft on the refreshed task; when dirty, show the
  conflict and let the user pick per field.

### 17. Security hardening (no critical issues; `/mcp` auth is sound)
- **17a. Disk exhaustion via `sync_receipts`** — `sync.rs:635` stores full `request_json`
  and reply per random `client_id`, uncapped. Any tailnet peer can loop `POST /api/sync`
  with fresh client IDs to fill `/srv/nextleaf`. Direction: store a request hash, cap or
  prune receipts.
- **17b. CORS allowlist trusts other apps' origins** — `lib.rs:1164-1172` allows
  `tauri://localhost`, `http://tauri.localhost`, and port 1420; these are shared by every
  Tauri app and Vite template. Desktop uses IPC, so this is unneeded. Direction: remove in
  production or gate behind a dev-only flag.
- **17c. No Host/Origin check on REST/sync** — `lib.rs:1094-1117`; DNS rebinding works on
  plain-HTTP setups (local `cargo run` against real data, historical `compose.yaml`).
  Tailscale Serve HTTPS mitigates the intended deployment. Direction: Host allowlist for
  `/api/*` (reuse `NEXTLEAF_MCP_HOSTS`), reject foreign `Origin` on mutating routes.
- **17d. Desktop webview has no CSP** — `src-tauri/tauri.conf.json:23` `"csp": null`. No
  current XSS sink found, but an injection could bind an unlinked DB to an attacker
  server. Direction: strict CSP, e.g. `default-src 'self'; connect-src ipc:
  http://ipc.localhost; img-src 'self' data:; style-src 'self' 'unsafe-inline'`.
- **17e. No anti-framing headers** — `lib.rs:1158` ServeDir; clickjacking is the remaining
  browser write path. Direction: `X-Frame-Options: DENY` / `frame-ancestors 'none'`,
  `X-Content-Type-Options: nosniff`.
- **17f. Docs understate unauthenticated exposure** — `docs/INSTALLATION.md:55-57`,
  `docs/DEPLOYMENT.md:19-21`. Tailscale's default policy is allow-all, including shared-in
  nodes. Direction: add an ACL/grants example; consider checking Tailscale Serve's
  `Tailscale-User-Login` header against an allowlist.

### 18. Desktop startup failure quits with no message
- **Where:** `src-tauri/src/lib.rs:175-179`, `lib.rs:198` (`.expect(...)`).
- **Scenario:** migration failure after installing an older build over a newer DB, changed
  migration checksum, unwritable data dir → app silently quits.
- **Direction:** show a native error dialog with the database path and the error.

### 19. Nirvana re-import is not idempotent after local renames
- **Where:** `backend/src/nirvana.rs:185-228` (match Areas/tags by current name).
- **Scenario:** renamed Area "Personal" → identical retry creates a new empty "Personal"
  that syncs to the Pi. A tag converted to a Contact → identical retry fails "Existing tag
  name belongs to a person". Contradicts "identical retries are a no-op".
- **Direction:** record source-ID → local-ID mappings in the import receipts and match by
  those first.

---

## P3 — smaller issues and process

- **Frontend tests are not wired up:** no `test` script in `frontend/package.json` and no
  CI step runs them. Add `"test": "node --test --experimental-strip-types tests/*.test.ts"`
  and a CI job (plus `tsc --noEmit`).
- **Filter click during a save drops the post-save refresh:** `App.tsx:283-293` controls
  stay enabled; `changeFilters` → `clearRecent` bumps `loadNumber` (`App.tsx:199`), so the
  in-flight reload is discarded (`App.tsx:107`). Sequential successors and counts stay
  stale until the 30 s timer.
- **Docker base images pinned by tag, not digest** (low supply-chain risk).

## Checked and found sound

`/mcp` auth (constant-time token compare, Host/Origin checks, 503 when unconfigured); SQL
parameter binding; desktop sync client URL handling (HTTPS-only, no redirects, size cap,
server-ID binding); Docker hardening; GitHub Actions (SHA-pinned, no untrusted input in
`run`); no secrets in tracked files or history; Sequential availability logic; PATCH
null-vs-absent handling; transactions and `expected` guards; month-end, leap-year,
DST and year-boundary repeat arithmetic; worker/API/sync serialization; import
validation and rollback; no XSS sinks; date-only handling in the frontend; out-of-order
reload protection; UI and stdio-MCP processes syncing the same DB.
