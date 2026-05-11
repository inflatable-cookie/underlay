# Contract Index

Status: active
Owner: repo maintainers

## Purpose

This file is the canonical index for Underlay's Northstar contract surface.

It distinguishes:

- contracts that already exist
- contracts that should exist for the major shared systems
- the review order for implementation assessment after contract coverage exists

## Current Contracts

| ID | Status | Scope | Notes |
|---|---|---|---|
| [001-working-rules.md](./001-working-rules.md) | active | repo delivery rules | updated for parallel generation mode |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | active | shared media library and usage graph | the only substantial feature contract today |
| [`contracts/openapi/underlay.openapi.yaml`](/Users/tom/Dev/projects/underlay/contracts/openapi/underlay.openapi.yaml) | machine-readable reference | shared envelope and OpenAPI shapes | useful evidence, not complete system authority |

## Planned Contract Set

| Proposed ID | Status | System family | Primary sources | Core questions |
|---|---|---|---|---|
| [010-foundation-primitives-and-envelopes.md](./010-foundation-primitives-and-envelopes.md) | active | IDs, `AppError`, envelopes, validation primitives | `underlay-core`, `underlay-validation*`, `underlay-http` | settles the shared primitive model and records current transport-normalization drift |
| [020-http-transport-and-server-boundary.md](./020-http-transport-and-server-boundary.md) | active | HTTP helpers, cookies, query/pagination, CSP/server TS helpers | `underlay-http`, `underlay-http-client`, `ts/src/client/**`, `ts/src/server/**` | settles the shared transport contract and records current caller-shape drift |
| [030-auth-and-session-systems.md](./030-auth-and-session-systems.md) | active | auth boundary, sessions, MFA, WebAuthn, OAuth, browser auth runtime | `underlay-auth*`, `ts/src/client/auth.ts`, `ts/src/runtime/auth.ts`, `ts/src/patterns/auth-workflows/**` | settles the shared auth/session boundary and records current schema/runtime drift |
| [040-storage-blob-and-media-systems.md](./040-storage-blob-and-media-systems.md) | active | DB bootstrap, blob backends, storage ownership, media orchestration, soft delete | `underlay-db`, `underlay-blob`, `underlay-media`, `underlay-soft-delete`, `underlay-aws` | settles the durable storage/media seam and its relationship to `050` |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | active | media library contract | existing contract | does it still match implementation and repo goals |
| [060-jobs-events-and-operator-systems.md](./060-jobs-events-and-operator-systems.md) | active | jobs, scheduled tasks, events, audit, security alerts, rate limiting, email | `underlay-jobs`, `underlay-events`, `underlay-audit`, `underlay-security-alerts`, `underlay-ratelimit`, `underlay-email` | settles the shared operator-facing async/control-plane contract |
| [070-nightfire-and-migration-systems.md](./070-nightfire-and-migration-systems.md) | active | Nightfire content model, editor/runtime, migration-core | `underlay-nightfire`, `underlay-migration-core`, `ts/src/nightfire/**` | settles the shared content-system and migration discipline and records current TS/runtime authority drift |
| [080-ai-runtime-and-suggestions.md](./080-ai-runtime-and-suggestions.md) | active | AI runtime, provider boundary, routing candidates, generic relation suggestions | `underlay-ai-runtime`, `underlay-suggestions`, TS suggestion helpers | settles the lower AI/runtime and suggestion request boundary and records current TS authority drift |
| [090-ts-runtime-and-client-orchestration.md](./090-ts-runtime-and-client-orchestration.md) | active | runtime helpers and browser/data/navigation/media orchestration | `ts/src/runtime/**`, selected `ts/src/client/**` | settles the retained TS runtime/client seam and records the remaining runtime-vs-pattern authority drift |
| [100-shared-patterns-and-workflow-shells.md](./100-shared-patterns-and-workflow-shells.md) | active | relation selector, form shells, batch/list/reorder/navigation state, upload flows, i18n | `ts/src/patterns/**` | settles the retained workflow-shell boundary and records the remaining pattern-vs-runtime/template drift |
| [110-admin-template-system.md](./110-admin-template-system.md) | active | entity list/detail/form templates | `ts/src/templates/**`, template docs, consumer rollout evidence | settles the stable template hierarchy, extension model, and form stop point while recording rollout-era drift |
| [115-admin-resource-api-shapes.md](./115-admin-resource-api-shapes.md) | active | list/detail/tab API shapes for admin resource pages | template types, API profile guide, `underlay-reference` consumer evidence | settles the page-shaped API seam the template rollout depends on |
| [116-canonical-collection-routes-and-query-profiles.md](./116-canonical-collection-routes-and-query-profiles.md) | active | canonical collection routes, query profiles, selector/filter/page command convergence | `073`, `080`, `100`, `115`, client types, selector workflow contract | settles the next API simplification lane: one route family and one command posture across page and selector consumers |
| [117-hybrid-collection-shells.md](./117-hybrid-collection-shells.md) | active | shared shape for root/tab hybrid collection shells with batch/reorder/transform workflow | `100`, `110`, `115`, `116`, sweep `030`, Dairy `ModulesList` evidence | settles the missing middle shape between plain `EntityList` and app-owned workflow composites |
| [120-tooling-testing-and-contract-artifacts.md](./120-tooling-testing-and-contract-artifacts.md) | active | testing helpers, devtools, scanners, guardrails, machine-readable contract artifacts | `underlay-testing`, `underlay-devtools`, `ts/src/tools/**`, `ts/src/testing/**`, `contracts/**` | settles the shared support-layer boundary and records current artifact/tooling drift |

## Assessment Order After Coverage

After the contract set exists, assess implementation against contract in the
same order:

1. foundation and transport
2. auth
3. storage and media
4. jobs and operator systems
5. Nightfire and migration
6. AI and suggestions
7. TS runtime/client orchestration
8. shared patterns
9. admin templates
10. tooling and contract artifacts

## Promotion Rule

Do not promote a system into active implementation-assessment work until:

- its contract file exists
- the contract identifies the system goal, interface, invariants, extension
  points, and known caller families
- the source-of-truth location is explicit
- the next repair work can be expressed as bounded roadmap cards instead of
  exploratory drift

## Next Task

The first contract-coverage and assessment generation is complete. Open a new
explicit roadmap generation before promoting fresh contract-driven work.
