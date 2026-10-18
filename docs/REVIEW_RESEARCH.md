# Future Review: OmniFocus on iPad as a reference

Research date: 2026-10-17. This supports a later specification round, not a current
implementation request. The owner wants small guided steps including Inbox and
individual projects. Mobile, offline desktop, MCP, and installation take priority.

## Historical evidence

The official 2010 iPad guide describes one project at a time with an explicit
Mark Reviewed action advancing to the next. Project status and interval are editable.
The illustrated section is printed page 6 / PDF page 7. Its historical two-week
default is not a Nextleaf requirement.
[Original iPad guide](https://www.omnigroup.com/documents/review_guides/OmniFocus-iPad-Review-Guide.pdf#page=7)

The September 7, 2012 article shows pending projects in a sidebar and one selected
project, with last review, interval, status, and confirmation below. It recommends
small review batches and project-specific intervals, checking relevance/completion
and a clear next action. Paused projects still need review.
[OmniFocus at School: Reviews](https://www.omnigroup.com/blog/omnifocus-at-school-reviews),
[manufacturer archive](https://www.omnigroup.com/blog/archives/)

The OmniFocus 2 iOS 2.5 manual also documents per-project confirmation. Interval
placement differs between iPad and iPhone, and its default is one week. Long-press
allows early review. Defaults were not identical across releases.
[OmniFocus 2 Review](https://support.omnigroup.com/documentation/omnifocus/ios/2.5/en/review/)

The retrieved OmniFocus 4.9.3 manual separates Previous/Next navigation from Mark
Reviewed, which records completion and computes the next review date. Pending count,
last review, and interval stay visible. Sorting and paused/deferred inclusion options
are not automatically adopted requirements.
[OmniFocus 4 Review](https://support.omnigroup.com/documentation/omnifocus/universal/4.9.3/en/perspectives/#review)

## Inbox is a distinct part of a broader review

The documented built-in Review perspective reviews projects. These sources do not
prove a built-in wizard that first processes Inbox and then all lists/projects.
A broader GTD Weekly Review also includes capture, calendar, Waiting, and Someday;
Omni's whitepaper distinguishes that routine and recurring checklist templates.
[GTD whitepaper](https://inside.omnifocus.com/gtd-whitepaper)

A Nextleaf Inbox-plus-project workflow therefore follows the owner's request,
rather than being a verified one-to-one copy of an old OmniFocus function. The
specific version the owner used is unknown.

## Design implications and open decisions

One item at a time may reduce distraction; an explicit confirmation distinguishes
review from browsing. Visible remaining work and small batches could help the
owner's workflow. These are design inferences, not measured productivity claims.

Before implementation, decide:

1. Scope and order: Inbox, projects, and possibly Waiting/Someday; session size.
2. Default/per-project intervals and handling of empty/paused projects. Review
   dates must remain distinct from task due dates.
3. Pause/resume and skipping without a false review confirmation.
4. Perspective interactions: do not claim a complete review while silently hiding
   Areas or Tags.
5. Shared UI/offline/MCP rules. An agent must not mark reviews done without instruction.

Only documentation was checked; no OmniFocus version was operated live. PDF image
retrieval failed, so visual details were not independently verified from that PDF.
No Review UI, fields, or migrations have been implemented.
