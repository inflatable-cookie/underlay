# New Project Quickstart

> **Superseded.** The step-by-step quickstart that once lived here described a
> pre-`g06` monorepo layout (`apps/bloom`, `libs/petal`, a `legacy/libraries`
> symlink) that no current consumer uses, and pinned stale toolchain versions
> (`axum 0.7`, `bun 9+`). It has been retired to prevent new apps bootstrapping
> against a layout the reference apps abandoned.

For bringing up a new consumer app against Underlay, use the current sources:

- **[Contract 024 - New app bootstrap and bring-up](../contracts/024-new-app-bootstrap-and-bring-up.md)** —
  the authoritative bootstrap contract (dependency wiring, auth provider setup,
  the six-consumer conventions).
- **[Guide 020 - Project structure](../guides/020-project-structure.md)** —
  the current directory and package layout.

The live consumer apps (acowtancy, songsprout, contact-patch, compli-me,
loophole, underlay-reference) are the working reference implementations; mirror
the closest one to your app's shape.
