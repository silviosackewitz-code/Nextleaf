# UI reference: Nirvana

Observed in the owner's existing Nirvana macOS app on 2026-10-17, without creating,
editing, or completing tasks. Observations are not blanket authorization to copy
every visible feature. The agreed Nextleaf rules live in SPECIFICATION.

## Desktop observations and accepted direction

Inbox stands separately above daily GTD views. Projects has a title button for
its overview and a separate collapse arrow; Tags is collapsible, Logbook/Trash
are near the bottom, and Areas filter at the top. Next offers unified/project
and sequential-next display plus Top 1–9/All project-action limits. Opening those
menus did not functionally validate Nirvana's underlying availability rules.

Rows show compact tags/dates and note indicators. The large task detail region is
an editor expanded in the list, not an OS window: title/tags/notes on the left,
time/energy/due/state/project on the right, Save Changes and Cancel below.
A completed task remained struck through. Cleanup was described as collecting
completed items into Logbook; it was not run, and automatic timing was not tested.

The right action menu includes due date, Areas/Contacts/Contexts, Time, Energy,
state, and Move. Time includes 5/10/15/30/45 minutes and hour values; Energy includes
low/medium/high/None. Visible Convert/Copy/Email actions are not automatically in scope.

The user approved compact rows, inline details, quick menus, and New Item task/
project/tag destinations. Later screenshots clarified two-line titles and separate
metadata, superseding strict single-line rows. Desktop submenus should open left,
category hover should work, and direct choices should save immediately. Keep five
primary editor fields and move the others under More options.

Nextleaf's completed-row policy is now explicit: retain locally until view change/
refresh, while saving completion immediately. Group nonempty projects, show standalone
actions first, omit repeated project labels, and retain metadata visibility choices.
Use New Item rather than a permanent capture row. Areas belong in Settings and filters.
Review/Perspectives remain future work; observations do not authorize implementing them.

## iPhone reference

Six owner-supplied Nirvana iPhone screenshots were viewed in Photos on 2026-10-17:
list, navigation, entry with keyboard, details, Time filter, and view menu. No photos
were modified and no private screenshot contents were copied into the repository.
Public [App Store screenshots](https://apps.apple.com/us/app/nirvana-for-gtd/id1022913190)
were supplementary; the owner's screenshots were the primary reference.

Observed full-width wrapping task rows, completion on the left, Focus on the right,
metadata underneath, faded completed rows, and a round plus button. A compact header
contains navigation, Area choice, and view menu; a horizontal filter strip shows
All/Due/Time/Energy/Tags. A partly visible continuation suggests scrolling but no
scroll gesture was observed in still images.

Navigation overlays from the side. Details are a tall one-column sheet with Done,
title/notes, Focus, tags/time/energy/due, assignment, and lower actions. An entry
image shows Cancel/Save above the keyboard. Whether Done autosaves and the exact
state transitions cannot be inferred from screenshots. The Time filter uses one
compact menu. Additional Theme, Share, Paste, and Log completed options are not
implicitly Nextleaf requirements.

Adopt responsive navigation, full-width lists, tap menus, mobile details, and
keyboard-aware controls. Do not change desktop completion behavior solely because
of a reference screenshot. The initial code review found stacked desktop navigation,
small targets, cramped metadata, overflowing submenus, and missing safe areas;
these motivated the implemented mobile pass. Actual iPhone Safari behavior remains
a separate test. See INSTALLATION and NEXT_STEPS for acceptance and evidence.
