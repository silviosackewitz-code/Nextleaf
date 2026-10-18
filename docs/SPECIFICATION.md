# Nextleaf specification

This is the agreed product scope, consolidated on 2026-10-18. Implementation and
verification status belong in [NEXT_STEPS.md](NEXT_STEPS.md). Requirements below
must not be mistaken for proof that a platform has been tested.

## Purpose and structure

A calm, self-hosted GTD task manager inspired by Nirvana's compact daily workflow
and OmniFocus's Sequential projects. Keep the product name Nextleaf despite the
known naming overlaps; no domain purchase or trademark clearance has been agreed.
All application and repository text is English. Conversations may be German.

Use one monorepo, Rust/Axum/SQLx/SQLite, React/TypeScript/Vite, and Tauri 2.
The local checkout is named `Nextleaf`; its path is not a technical dependency.
Use simple existing icons. Avoid unnecessary helper text, banners, and visual noise.

The hierarchy is flat: **Area → Project → Task**, with standalone tasks allowed.
No nested projects or subtasks in version 1. Projects are Parallel by default;
Sequential is an explicit choice. The backend is the authority for availability.
In a Sequential project, the first unfinished, non-trash item blocks later items,
including when that first item is Waiting or deferred. Parallel projects expose
all otherwise eligible next actions. View options must not bypass these rules.

## Views and navigation

Inbox is separate at the top. Daily views are Focus, Next, Waiting, Scheduled,
Someday, and Later. Projects have a title button opening an overview and a separate
collapse control. Tags have their own collapsible section. Logbook, All, Trash,
and Settings stay at the bottom. Manage Areas through Settings / Manage Tags.

New Item offers task destinations Inbox, Next, Waiting, Scheduled, Someday, Later,
and Focus, plus Project and Tag. Focus sets a flag rather than replacing task state.
Within a project, new tasks start in Next and the creation form omits the status
choice. New projects inherit the relevant Area. Archived Areas cannot be newly assigned.

Completed tasks remain struck through in the current view until refresh or a
view change. Clicking their completion control again restores the previous state.
The backend saves completion immediately, unlocks Sequential successors, and puts
completed tasks in the Logbook. General reopening from Logbook uses Next.
Logbook is newest completion first. No completion banner is required.

## Task editing and quick actions

On desktop, clicking a title expands an inline editor, with only one task open.
Keep title, notes, tags, and Focus together; show Time, Energy, Due, State, and
Project as the five primary properties. Put additional properties such as Starts
and Waiting for under More options. Hide the completion control while editing.
Save changes and Cancel are explicit. Protect unsaved drafts from silent loss;
return focus to the title on close. Preserve error messages and edits on failed saves.

Compact rows use separate title and metadata areas with up to two visible lines;
full titles remain available in the editor. Notes have off, preview, and full modes.
Focus is yellow. Overdue open tasks use only a red completion-circle border.
Inbox has a red count; Focus separates overdue red and other gray counts;
Waiting has a gray count. Do not put a red border around the entire view.

The right-side quick menu is narrow, with nested desktop choices opening to the
left. Mouse hover may open categories, but clicks and keyboard controls must work.
Selecting a value saves immediately and closes after success. Free text commits
on Enter or blur. Date choices use the shared calendar. Keep the existing value
and menu on errors, and prevent concurrent duplicate writes.

Tags commit only on explicit Enter or suggestion selection. Blur, Tab, and Save
must not silently turn unfinished text into a new tag. Support keyboard suggestion
navigation and removing chips. Area appears as a distinct chip; project-inherited
Area cannot be overridden on the task. Archived links stay visible but archived
entries are not offered for new assignments.

Dates retain `YYYY-MM-DD` precision without invented times. The shared month
calendar updates a form draft, or saves immediately from quick actions; support
removing dates. Ordinary select fields remain accessible native controls.

## Projects

The overview contains active, someday, and completed projects, including empty
projects. Counts include open actions, excluding completed, trash, and recurring
templates. Project tag filters apply to the overview. Opening a project's actions
clears the task tag filter. Show nonempty project groups with unassigned actions
first and without repeated project labels on every task.

Inline project properties include name, notes, tags, Area, due date, status,
Parallel/Sequential mode, and Focus. Completing a project does not silently
complete its child tasks. Manual reordering is not yet a completed feature.

## Areas, People, Tags, and filters

Settings → Manage Tags has distinct Areas, People, and Tags sections. People are
a real tag kind, not a tag named People. Rename, change kind, archive, and restore
without losing task/project links. Do not automatically reclassify existing tags.
Person renames update exactly matching Waiting-for text, while unrelated free text
stays unchanged. Make the same operations available through MCP and desktop sync.

Waiting highlights the contact before the task title and offers contacts in its
filter bar, alongside relevant Areas and ordinary Tags. Preserve existing free-text
Waiting values. All clears the tag filter. **— / Untagged** selects items without
tags or contacts, including free-text Waiting contacts; an Area alone is allowed.
It does not create a tag or change stored data. Apply it consistently to tasks,
project overview, counts, and mobile filter selection.

Filters depend on the view and its content: Next and Focus have Time/Energy,
relevant tags and Areas, All and Untagged; show Due when relevant. Waiting emphasizes
contacts; Scheduled and contact views get appropriate choices. Avoid a fixed,
identical toolbar on every view. Preserve the Area and GTD view while filtering.

View Options exists only in global Next and Focus. Remember each view separately:

- Next: Unified list, Group by project, Sequential next actions, and Project actions.
- Focus: Unified list, Group by project, and Group by state.
- Project actions: Top 1 through Top 9 or All, default Top 3, with Show more.

Sequential next actions limits display to the first available project action; it
does not change a project's mode or skip a blocking Waiting action.

## Desktop, server, and mobile

Server target: Raspberry Pi 5, 64-bit Ubuntu 24.04 LTS, Docker Compose. Also ship
Linux AMD64 images. Keep config in `/opt/nextleaf-app`, data in `/srv/nextleaf`,
Tailscale on the host, and container HTTP bound to host loopback. No public
unauthenticated REST/sync access. MCP authentication is separate.

Follow the owner's common application storage policy: `/opt/<app>-app/` contains
Compose and `.env`, `/srv/<app>/` contains persistent application data, and Docker
manages its own replaceable image/container storage. Document backup scope for
every app. For Nextleaf, back up the complete configuration directory and a
consistent SQLite data directory. The existing Immich layout is the reference;
this decision does not authorize migrating other installed applications. See
[STORAGE.md](STORAGE.md).

Desktop embeds the shared backend and stores locally. Bidirectional sync runs
against the private server, with a baseline, stable identity, and explicit
field-level conflict resolution. Preserve independent edits; do not silently
choose one whole database. Detect server identity/URL mismatches and reject
incompatible protocols without changing data. Background sync runs every 15 seconds
while UI or local MCP is running, only for the remote saved with that database.
A fresh database must not inherit an old connection from browser UI storage.

The iPhone web interface is online-only and uses the same application and GTD
rules. Use a navigation drawer, full-width wrapping task rows, completion on the
left and Focus on the right, compact filters, and a reachable floating plus button.
Mobile details use a nearly full-screen, single-column sheet. Touch choices do not
rely on hover or side-opening submenu chains. Respect safe areas and the on-screen
keyboard; keep Save/Done/Cancel reachable. Use at least 44px primary touch targets
and suitable input text sizes. Keep desktop inline editing unchanged.

The user's six Nirvana phone screenshots guide layout, not extra features such as
Share or Themes. Real iPhone/Safari/keyboard checks are required separately from
viewport simulation. Native iPhone and offline web support are future possibilities.

## MCP and import

Expose implemented task, project, Area, and tag operations through the common
backend. Desktop provides stdio (`Nextleaf --mcp`), server provides authenticated
Streamable HTTP. The owner prefers local desktop integration. ChatGPT/Claude
registration and iPhone voice operation are separate tests, not established merely
by adding an MCP endpoint. Client display/connection settings remain local.

Nirvana import is one-time only, with all imported projects Parallel by explicit
choice. Preserve source IDs, notes, dates, and raw source data; do not infer missing
recurrence rules or create ongoing Nirvana sync. See [NIRVANA_IMPORT.md](NIRVANA_IMPORT.md).

## Distribution and acceptance

Publish the repository under GPL-3.0-only. Product description: a self-hosted task
manager with a native Mac app and iPhone web interface. Keep all repository prose
English. Do not rewrite earlier Git commits merely to translate their history.

Use versioned, public release downloads with checksums and prebuilt images.
The first server install uses wget, `.env`, and Docker Compose without compiling.
Mac users install a DMG; Windows Setup EXE and Linux AppImage/DEB are later targets.
Updates initially install a new version and preserve local data; no integrated
updater is required yet. The owner approved a Mac test build without Apple Developer ID because no
Apple Developer membership is available. Provide a valid ad-hoc bundle signature
for package integrity; do not describe this as Apple-verified or notarized.

After preparation, the owner will remove test installations on Pi, Mac, and iPhone
and run the documented installation from empty data, server first. Existing tasks
are disposable tests backed by the original Nirvana account. This is authorization
to prepare the release, not a claim that the reset or acceptance test has occurred.

## Deferred specification

**Perspectives:** replace the simple Area selector later with saved Area/tag filters,
initially AND semantics. Hide data outside the chosen perspective consistently.
An editor, exact behavior, and implementation remain a later increment.

**Review:** a guided, low-distraction sequence including Inbox and individual
projects, inspired by old OmniFocus iPad Review. Define scope, intervals, pause/
resume/skip, status editing, and perspective interactions before implementation.
Opening or skipping a project must not count as reviewing it. No Review migrations
or UI are currently authorized by the reference research alone.
