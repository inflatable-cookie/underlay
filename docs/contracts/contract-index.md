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
| [025-rust-app-runtime-assembly-and-router-topology.md](./025-rust-app-runtime-assembly-and-router-topology.md) | active | API workspace/runtime assembly, `AppState`, router builder, middleware order, health/OpenAPI/metrics posture | current consumer `*/api` crates, `020`, `030`, `060` | settles what a normal Underlay Rust API app looks like so new apps can assemble one by declared pattern |
| [026-route-families-and-access-model.md](./026-route-families-and-access-model.md) | active | shared/front/admin/operator route families, auth gates, role gates, CSRF/version/rate-limit posture | current consumer routers, `020`, `030`, `115` | settles admin vs front vs shared endpoint semantics and the repeated access-control/runtime policy layer |
| [027-api-canonical-path-cutovers-and-compatibility-retirement.md](./027-api-canonical-path-cutovers-and-compatibility-retirement.md) | active | canonical-path migration, compatibility aliases, client/server cutover order, retirement policy | `026`, `118`, `g05.009`, `composer-api` normalization evidence | settles how Underlay apps move from older paths to canonical families without leaving alias and retirement policy app-local |
| [028-runtime-surface-and-openapi-maturity-levels.md](./028-runtime-surface-and-openapi-maturity-levels.md) | active | runtime maturity ladder for health, metrics, OpenAPI JSON, and Swagger exposure | `025`, `026`, endpoint-family matrix, six-site runtime evidence | settles lean versus rich runtime posture so audits stop treating every runtime difference as unexplained drift |
| [029-non-resource-workflow-action-route-grammar.md](./029-non-resource-workflow-action-route-grammar.md) | active | workflow-action naming and placement for restore/purge/reorder/complete/skip/claim/release-style routes | `026`, `027`, `119`, six-site workflow-route evidence | settles the repeated non-resource action grammar so workflow verbs stop drifting between CRUD, helper, and action-family styles |
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
| [118-front-and-shared-read-api-shapes.md](./118-front-and-shared-read-api-shapes.md) | active | front/public/shared read envelopes, canonical read-route families, helper vs page-shaped read surfaces | current consumer routers, `020`, `115`, `116` | extends the page-shape line beyond admin templates so normal non-admin read surfaces are also declared |
| [119-helper-search-and-lookup-route-catalogue.md](./119-helper-search-and-lookup-route-catalogue.md) | active | helper-route naming, lookup/search/suggest/detect/status catalogue, bounded helper envelope rules | `020`, `026`, `118`, six-site helper cleanup evidence | settles the repeated non-resource helper family so lookup/search/status routes stop drifting as one-off exceptions |
| [120-tooling-testing-and-contract-artifacts.md](./120-tooling-testing-and-contract-artifacts.md) | active | testing helpers, devtools, scanners, guardrails, machine-readable contract artifacts | `underlay-testing`, `underlay-devtools`, `ts/src/tools/**`, `ts/src/testing/**`, `contracts/**` | settles the shared support-layer boundary and records current artifact/tooling drift |

## Assessment Order After Coverage

After the contract set exists, assess implementation against contract in the
same order:

1. foundation and transport
2. Rust app runtime assembly and router topology
3. auth
4. route families and access model
5. canonical-path cutovers and compatibility retirement
6. runtime surface and OpenAPI maturity
7. non-resource workflow action grammar
8. storage and media
9. jobs and operator systems
10. Nightfire and migration
11. AI and suggestions
12. TS runtime/client orchestration
13. shared patterns
14. admin templates
15. front/shared read API shapes
16. helper/search/lookup route catalogue
17. tooling and contract artifacts

## Promotion Rule

Do not promote a system into active implementation-assessment work until:

- its contract file exists
- the contract identifies the system goal, interface, invariants, extension
  points, and known caller families
- the source-of-truth location is explicit
- the next repair work can be expressed as bounded roadmap cards instead of
  exploratory drift

## Next Task

The runtime-contract wave is live inside `g05.009`.

Do not open a new roadmap generation for fresh contract-driven work unless the
active generation is explicitly rolled over.
