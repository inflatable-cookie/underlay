# New Project Quickstart

> **Superseded.** The step-by-step quickstart that once lived here described a
> pre-`g06` layout (`apps/bloom`, `libs/petal`, a `legacy/libraries` symlink)
> that no current consumer uses, and pinned stale toolchain versions
> (`axum 0.7`, `bun 9+`). It has been retired to prevent new apps bootstrapping
> against a layout the reference apps abandoned.

A new Underlay consumer is **one Git repository** with `apps/*`, `packages/*`,
and a root `docs/`. Polyrepo layouts are unsupported.

For bringing up a new consumer app, use the current sources:

- **[Contract 024 - New app bootstrap and bring-up](../contracts/024-new-app-bootstrap-and-bring-up.md)** —
  the authoritative bootstrap contract: workspace topology, the normative root
  `package.json` shape, dependency rules, and the bring-up flow.
- **[Guide 020 - Project structure](../guides/020-project-structure.md)** —
  the step-by-step build of that layout.
- **[Guide 030 - Underlay integration](../guides/030-underlay-integration.md)** —
  how Underlay and Poodle enter the dependency graph.
- **[Monorepo rollout closeout](../logs/2026-08/26-151525-g10-006-010-fleet-closeout.md)** —
  the six-consumer proof and migration evidence.

`acowtancy` is the live proof of the contract; mirror it. `underlay-reference`
is the bootstrap fixture. Contact Patch, Compli Me, Songsprout, and Composer
also conform after the `g10.006`–`g10.010` rollout.
