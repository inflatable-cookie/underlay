# 044 - Shared UI Documentation and Demo Refresh

Status: In Progress
Owner: Platform
Created: 2026-03-26
Depends on: 031, 042, 043

## Overview

Rebuild the shared UI documentation and demo layer around the actual
post-migration Underlay surface. The Poodle adoption wave materially reduced
Underlay's generic component ownership, so the old Storybook backlog pitch is
now stale: Underlay no longer needs a broad generic component catalog for every
primitive, but it does need a reliable, interactive teaching layer for the
retained workflow shells, helper surfaces, and boundary guidance that consuming
apps still use directly.

## Research Basis

- `docs/roadmaps/backlog/storybook-component-docs.md`
- `docs/guides/090-ui-kit.md`
- `docs/guides/062-auth-ui-components.md`
- `docs/guides/077-media-library.md`
- `docs/guides/097-autonomous-list-components.md`
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `ts/src/components/index.ts`
- `ts/src/patterns/index.ts`

## Decision Summary

- The next documentation wave should target the stabilized retained Underlay
  surface, not the pre-Poodle generic component library.
- Poodle remains the canonical demo surface for primitives and generic
  composites that were migrated out of Underlay.
- `g01.044` should first decide the demo vehicle cleanly: Storybook, Histoire,
  or a deliberately smaller docs-driven demo path.
- Batch 44.1 selects Storybook as the demo/catalog tool for Underlay. The repo
  already sits on Svelte 5 and Vite 7, Storybook has an official current
  SvelteKit framework path plus documented Svelte 5 story support, and the
  retained Underlay surfaces need richer interactive workflow demos than a
  docs-only catalog would provide.
- Histoire is explicitly rejected for this roadmap wave. Its public docs are
  still framed around `@histoire/plugin-svelte` and "Svelte 3", which makes it
  a weaker fit for an execution wave that should minimize tool-risk while the
  demo layer is being re-established.
- Do not reopen `g01.042` style migration work under the label of
  documentation. Missing demo coverage should not become an excuse to re-grow
  Underlay's UI surface.
- The documentation layer must teach authority boundaries clearly: when to use
  Poodle directly, when to use retained Underlay workflow shells, and when to
  build app-owned composition locally.

## Likely Implementation Surface

- `package.json`
- `effigy.toml`
- `README.md`
- `docs/guides/README.md`
- `docs/guides/090-ui-kit.md`
- `docs/guides/062-auth-ui-components.md`
- `docs/guides/077-media-library.md`
- `docs/guides/097-autonomous-list-components.md`
- `docs/guides/098-shared-admin-patterns.md`
- a new demo/catalog workspace or config surface if the chosen tool requires it
- retained shared UI surfaces under `ts/src/components/` and `ts/src/patterns/`

## Batch 44.1 - Demo Authority and Tooling Decision

- [x] Audit the retained public Underlay UI surface and group it into demo
      domains instead of one flat catalog.
- [x] Decide whether Storybook still earns the extra tooling weight for this
      smaller retained surface or whether Histoire or a lighter docs-native demo
      approach is now the better fit.
- [x] Record the canonical boundary between Underlay demos and Poodle demos so
      the two libraries do not duplicate each other again.
- [x] Promote the decision into active guides and the roadmap front doors before
      building the demo catalog.

Audit result for 44.1:
- auth shells and helpers: `AuthLayout`, `LoginPage`, `ForgotPasswordFlow`,
  `TotpInput`, `PasswordRequirements`
- media, list, and batch workflows: `MediaPicker`, `MediaActionsMenu`,
  `LogList`, `BatchActionBar`, `CopyActionsMenu`
- structural shells and dialogs: `PageHeader`, `SpaFormShell`, `FormDialog`,
  `DropdownMenu`, `ErrorBoundary`
- specialized retained composites that still need deliberate treatment rather
  than broad primitive coverage: `AiRoutingAdmin`, `Banner`, `RelationSelector`,
  and selected `AutonomousList` / `DetailPageShell` surfaces

Authority boundary recorded in 44.1:
- Underlay Storybook should cover retained workflow shells, helpers, and
  structural composites that still belong to Underlay directly.
- Poodle preview/docs remain the canonical catalog for primitives and generic
  composites migrated out of Underlay.
- Active Underlay guides should link outward to Poodle docs for those migrated
  surfaces instead of re-documenting them locally.

## Batch 44.2 - Catalog Bootstrap and Repo Integration

- [x] Install and configure the chosen demo/catalog tool with Svelte 5 support.
- [x] Add owner-approved local tasks for running, checking, and building the
      catalog.
- [x] Wire the demo layer into README and guides so developers can discover it
      without reading source first.
- [x] Keep the initial bootstrap generic and repo-owned instead of hard-coding
      app-specific fixture data.

Completed in 44.2:
- bootstrapped a working local Storybook catalog with the official Storybook
  Svelte-Vite path for this library-oriented repo
- added repo-owned `effigy storybook` and `effigy storybook:build` tasks plus
  matching convenience package scripts
- landed the first minimal catalog shell with representative stories for the
  retained surface boundary, `TotpInput`, `PageHeader`, and `CopyActionsMenu`
- linked the catalog from README and active shared UI guides so it is
  discoverable as a teaching surface instead of stranded tooling

## Batch 44.3 - High-Value Retained Surface Coverage

- [x] Add interactive demo coverage for the highest-value retained shared
      surfaces:
  - auth shells/helpers: `AuthLayout`, `LoginPage`, `ForgotPasswordFlow`,
    `TotpInput`, `PasswordRequirements`
  - media/list/action workflows: `MediaPicker`, `MediaActionsMenu`, `LogList`,
    `BatchActionBar`, `CopyActionsMenu`
  - structural shells: `PageHeader`, `SpaFormShell`, `FormDialog`,
    `DropdownMenu`, `ErrorBoundary`
- [x] Use realistic retained-surface fixtures so workflow boundaries are obvious
      in demos without introducing fake generic props.
- [x] Link adjacent guides to those demos once the first coverage wave exists.

Completed so far in 44.3:
- auth story coverage now includes `AuthLayout`, `LoginPage`,
  `ForgotPasswordFlow`, and `TotpInput`
- retained workflow coverage now includes `MediaPicker`, `MediaActionsMenu`,
  `LogList`, `BatchActionBar`, `CopyActionsMenu`, `PageHeader`, `FormDialog`,
  and `ErrorBoundary`
- all new demos use repo-owned fixtures and callback simulations instead of
  lifting app-specific route code into the catalog
- the catalog now demonstrates the Underlay/Poodle authority split with real
  retained surfaces instead of a placeholder shell

## Batch 44.4 - Guidance Consolidation and Upgrade Path

- [x] Rewrite any active docs that still teach source-reading or app-running as
      the primary way to understand retained shared UI surfaces.
- [x] Add explicit guidance for when developers should go to Poodle docs instead
      of the Underlay catalog.
- [x] Update compatibility and onboarding guidance so new shared UI work lands
      with demo coverage by default.
- [x] Close the roadmap only when the demo layer reflects the settled
      post-migration surface closely enough to act as the canonical teaching
      entry point.

Completed in 44.4:
- active retained-surface guides now point directly at the local Storybook
  catalog instead of leaving developers to infer coverage from source alone
- lower-value remaining surfaces were explicitly assessed instead of left as a
  vague future tail:
  - direct story coverage added for `PasswordRequirements`, `DropdownMenu`, and
    `SpaFormShell`
  - `AiRoutingAdmin` is intentionally guide-only for now because useful demos
    would require a much richer operational data fixture set than this refresh
    wave should invent
- the catalog now covers the main retained auth, media, list/action, dialog,
  and structural shell families closely enough to act as the canonical local
  teaching layer

## Deferred

- Full parity demo coverage for every historical or internal-only Underlay
  component.
- Rebuilding a broad primitive catalog that now belongs in Poodle.
- Public deployment or external hosting before the local and repo-owned
  workflow proves stable.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- This roadmap should not force consuming apps to change runtime behavior.
- The main consumer-visible change is improved discoverability and clearer
  authority boundaries for shared UI usage.
- If the chosen demo tool introduces new developer tasks or conventions, those
  changes must be documented in onboarding and shared UI guides.

## Validation

```bash
effigy qa:docs
effigy qa:northstar
```

## Completion

`g01.044` is complete. Underlay now has a repo-owned Storybook catalog for the
retained post-migration shared UI surface, and the active guides point to that
catalog as the canonical interactive teaching layer.
