# Working on Nextleaf

Project knowledge belongs in the repository, not only in chat history.

- Before changing code, read `docs/SPECIFICATION.md`, `docs/ARCHITECTURE.md`,
  `docs/DECISIONS.md`, and `docs/NEXT_STEPS.md`.
- Record changed requirements in SPECIFICATION, dated decisions and their reasons
  in DECISIONS, and current verification and remaining work in NEXT_STEPS.
  Keep ARCHITECTURE consistent with the implementation.
- Use the existing Rust, Axum, SQLx/SQLite, React, TypeScript, Vite, and Tauri 2
  stack. Keep GTD rules in the shared backend. Target macOS, Windows, and Linux;
  distinguish supported release artifacts from future platform work.
- All repository prose and application text must be English, including errors,
  tooltips, accessibility labels, and documentation. Conversation language may differ.
- Prefer small, complete changes. Do not commit user data or secrets.
- Do not report a platform test as passed unless it was actually performed.
  Distinguish implemented behavior, planned work, and unverified assumptions.
- The current milestone is downloadable installation-test releases. Review and
  Perspectives need a separate specification round before implementation.
