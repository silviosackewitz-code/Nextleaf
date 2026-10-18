# One-time Nirvana import

Historical import: 2026-10-17. This is not ongoing synchronization and is not part
of the new empty-installation test. The original Nirvana account was not modified.

## Retrieved data

The official `https://mcp.nirvanahq.com/mcp` connector returned all 128 objects with
`get_tasks`, all states, notes included, limit 200, `has_more=false`. Separate
Logbook and Trash reads cross-checked 19 and 15 IDs. All referenced parents existed.

- 117 tasks: 21 Next, 50 Later, 4 Waiting, 1 Scheduled, 1 Recurring, 8 Someday,
  17 Completed, and 15 Trash.
- 11 projects, including 3 completed projects.
- 3 Areas and 12 Tags.
- All imported projects use Parallel mode by explicit user decision.
- The preexisting local task was preserved.

Preserve IDs, titles, notes, dates, waiting contact, time/energy, Focus, and raw
source objects. The source did not provide a complete recurrence rule: keep the
Recurring item as a template without generating occurrences. Manual ordering was
not available, so preserve retrieval order. Keep date-only values as `YYYY-MM-DD`
without inventing a time. Local `updated_at` reflects import time; source
`last_modified_days` remains in the raw record.

## Import implementation

```bash
cargo run --locked --manifest-path backend/Cargo.toml --bin import_nirvana -- input.json target.db
```

Input uses our `nirvana-mcp-v1` envelope with total, has_more, tag metadata, and
items; it is not claimed to be an official Nirvana export format. Validation
rejects duplicate/unknown IDs, unsupported types, broken parents, and ambiguous
Areas before a transaction can partly import data. `nirvana_imports` retains raw
records and stable local UUID mappings. Reimporting identical records is a no-op;
changed source records fail rather than overwriting intervening local edits.
Tests cover idempotency, validation, mapping, and rollback.

The private retrieval snapshot was stored at
`data/imports/nirvana-mcp-2026-10-17.json`, and the pre-import backup under
`data/backups/before-nirvana-live…db`. These ignored local files are not release
assets or public repository content. Never commit personal task exports.
