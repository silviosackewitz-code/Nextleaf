# Inbox review resolution — 2026-10-19

Reviewed `Inbox/2026-10-19-code-review.md` against the implementation and specification.
The report's suggestions were treated as proposals. The owner explicitly selected
all missed repeats with confirmation before large backlogs. This report records the source-repair phase;
existing uncommitted documentation was preserved. Deployment and release outcomes
from the subsequently authorized 0.1.3 update are recorded in NEXT_STEPS.md.

## Findings and disposition

| Finding | Verification and change |
| --- | --- |
| 1. Repeat backfill | Confirmed. All missing dates remain eligible. More than 100 missing copies pause before generation; the Repeat dialog previews the count and requires a date-bounded confirmation. Generation remains limited to 100 per pass and the total sync record ceiling. Calendar scanning is retained instead of introducing a distributed cursor. |
| 2. Repeat identity | Confirmed. Backend retains the cycle ID through edits and Stop Repeating. Protocol 4 synchronizes the identity ledger, including stopped templates, so a newly connected desktop cannot restart an old cycle under a fresh ID. Existing copies are preserved. |
| 3. Frozen rejected sync | Confirmed over HTTP. Definitive 4xx responses abandon only the rejected inflight request; 408/429, network errors and 5xx retain retry protection. The next attempt uses current data, and server error text is shown. Stale-sequence rejection refreshes client identity without discarding the baseline. Text limits now agree on writes/import/sync. Complete snapshots and their size cap remain. |
| 4. Tag rename plus old-name creation | Confirmed and regression-tested. Name-based remapping is limited to independently new remote tags. The existing rename, both tag identities and their separate task links now survive sync. |
| 5. Repeat rule lost on completion/trash | Backend defect confirmed; the State menu already disabled completion of recurring templates, unlike the report's UI claim. Completion/trash now retain rules; restoring Recurring works, including inert imported templates. Ordinary state changes/Stop Repeating clear the active rule. |
| 6. Lost editor drafts | Confirmed. App owns task/project draft state. Layout changes and list reloads preserve it; a vanished row retains its editor in a sheet. Browser regression covers 1120→700→1120 and a remotely completed task disappearing during another row's save. |
| 7. Completion undo loses start date | Confirmed. State-menu completion/trash now send only status, retaining the date/rule for undo. Backend regression covers both transitions and restoration. |
| 8. Project Area override | Confirmed. Project Area is authoritative even when it is empty. REST/MCP/import clear direct task Area on project assignment; sync rejects contradictory assignments. Migration retains cleared values in `task_area_repairs`. |
| 9. Date formats | Confirmed. New task/project/sync/import writes accept only canonical day dates. Upgrade normalizes legacy values and matching baselines; timestamps use the saved clock zone, and originals remain in `date_repairs`. A Berlin-midnight timestamp and a short date are covered by a migration test. |
| 10. Scheduled/date invariant | Confirmed. Backend ties start dates to Scheduled, rejects undated Scheduled creation, and returns to Next when its date is removed. Sync validates the invariant; legacy/import states are normalized. |
| 11. Read/write locking | Confirmed. Both runtime pools use WAL/15-second timeout. Task reads no longer run the scheduler. A file-backed test verifies reads complete while another connection owns the write lock. This removes the avoidable post-commit scheduler failure; ordinary storage failures can still fail reads. |
| 12. Coupled merge fields | Confirmed. Scheduling and Project/Area assignments merge as groups. Conflicts can be resolved individually or together, with stale-value guards. Tests cover a rule-vs-state conflict and mixed local/server choices without losing unresolved drafts. |
| 13. Server rollback after restore | Confirmed by code path; mitigated operationally. Restore instructions now require server identity rotation before reconnecting clients and explicit reconciliation of protected backups. Automatic detection of an arbitrary restore retaining its identity is still not implemented. |
| 14. False connected state | Confirmed. Unsuccessful first contact no longer delegates scheduling or advertises an active saved remote. Disconnect suspends sync while preserving server binding/baseline, and server workers ignore copied desktop connection metadata. Tentative URL/identity can remain internally for safe retry binding. |
| 15. Waiting text on blur | Confirmed. Text commits when focus leaves the Waiting choices, while explicit contact buttons retain precedence. Browser test types a contact and directly clicks Save changes, then checks persisted data. |
| 16. Stale open editor | Confirmed. Unchanged fields rebase on refreshed data; conflicting draft fields show local/saved values and explicit choices. Failed writes reload current values while retaining the draft. Browser test verifies a clean editor refreshes after an external edit. |
| 17a. Receipt growth | Confirmed. Store SHA-256 request fingerprints instead of full requests. Cap receipts at 128 client IDs and 64 MiB, rejecting excess atomically rather than evicting replay protection. Existing legacy receipts are accepted; owner review is required if the budget is exhausted. |
| 17b–e. Browser hardening | Confirmed as defense gaps, not a demonstrated existing XSS. Removed cross-origin CORS allowance, added server Host/Origin checks and anti-framing/nosniff headers, and added packaged/dev Tauri CSP. Host/Origin unit tests pass. |
| 17f. Exposure documentation | Confirmed. Deployment/installation docs explicitly state that reachable REST/sync peers have full access and explain removing broad Tailscale grants with an owner-only example. No live tailnet policy was changed or audited. |
| 18. Startup failure | Confirmed. DB initialization failure now keeps the app window open with its path and error instead of returning from setup and quitting. Native compilation passes; an actual packaged-app migration failure has not been exercised. |
| 19. Identical import retry | Confirmed. Matching item receipts return before Area/tag metadata processing. Regression verifies a no-op after Area rename/archive and tag rename/Contact conversion/archive. Incremental import/source-ID mapping remains outside the one-time import contract. |
| P3. Frontend CI | Added `npm test`, a source-check workflow with tests and TypeScript/Vite build, and frontend tests in the Docker build. |
| P3. Filters during save | Confirmed. Navigation/filter cleanup refuses while saving, and ordinary filter changes explicitly refresh after invalidating older loads. |
| P3. Docker tag pinning | Valid hardening suggestion, left unchanged. A digest-update policy belongs with release maintenance; no claim that current image tags are immutable. |

## Verification

- Backend: `cargo test --locked --offline -- --include-ignored`: **52 passed**,
  including both real loopback HTTP sync tests. Network tests were run outside
  the filesystem/network sandbox against temporary databases only.
- Frontend: **45 tests passed**, TypeScript check and Vite production build passed.
- Tauri/macOS: `cargo check --locked --offline` passed against the final backend.
- Chromium: draft retention across breakpoint changes and disappearing rows,
  clean-editor refresh, Waiting blur plus immediate Save, large-backlog confirmation
  and Cancel passed against an isolated local server/database.
- Migration: legacy date/Area normalization, audit preservation, baseline equality,
  repeat identity retention and protocol 3→4 pending-edit preservation passed.

No physical iPhone/Safari, Windows/Linux desktop, Docker image/Compose execution,
CI runner, packaged startup-failure dialog, installer, Pi deployment, tailnet policy,
or full backup/restore rehearsal was tested in this repair.

## Before shipping

Back up both replicas. Configure `NEXTLEAF_HTTP_HOSTS` with the actual Serve hostname
(or the existing MCP host allowlist). Update server first, then desktops: protocol 4
rejects older peers. Do not downgrade a migrated database. The production datasets
were not inspected or modified by this task; migration tests used synthetic data.
Older repeat histories whose cycle identity was already erased before this fix
cannot be reconstructed reliably from titles; no heuristic cleanup was attempted.
