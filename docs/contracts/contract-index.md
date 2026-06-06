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
| [021-database-migration-and-schema-workflow.md](./021-database-migration-and-schema-workflow.md) | active | durable schema migration layout, dev overlay separation, reset/replay loop, migration proof posture | six API packages, `070`, `120`, migration usage policy | settles the boring baseline for schema work so new apps do not improvise migration workflow from one consumer repo |
| [022-testing-posture-and-shared-harnesses.md](./022-testing-posture-and-shared-harnesses.md) | active | minimum and strong proof posture for API/admin/front packages plus shared harness usage | six app families, `120`, shared harness code | settles the default app-level test bar so health/validate/qa and harness use stop drifting between consumer repos |
| [023-release-and-compatibility-rollout.md](./023-release-and-compatibility-rollout.md) | active | fleet rollout order, compatibility windows, upgrade notes, and retirement proof for shared changes | `001`, `027`, `111`, `190`, `g01.031`, six-consumer rollout evidence | settles how Underlay changes move through the consumer fleet without ad hoc rollout policy |
| [024-new-app-bootstrap-and-bring-up.md](./024-new-app-bootstrap-and-bring-up.md) | active | workspace root shape, package family, docs authority, Effigy-first bootstrap and bring-up posture | six consumer workspaces, `025`, `110`, `120` | settles what a normal Underlay app repo looks like on day one and how it should start cleanly |
| [025-rust-app-runtime-assembly-and-router-topology.md](./025-rust-app-runtime-assembly-and-router-topology.md) | active | API workspace/runtime assembly, `AppState`, router builder, middleware order, health/OpenAPI/metrics posture | current consumer `*/api` crates, `020`, `030`, `060` | settles what a normal Underlay Rust API app looks like so new apps can assemble one by declared pattern |
| [026-route-families-and-access-model.md](./026-route-families-and-access-model.md) | active | shared/front/admin/operator route families, auth gates, role gates, CSRF/version/rate-limit posture | current consumer routers, `020`, `030`, `115` | settles admin vs front vs shared endpoint semantics and the repeated access-control/runtime policy layer |
| [027-api-canonical-path-cutovers-and-compatibility-retirement.md](./027-api-canonical-path-cutovers-and-compatibility-retirement.md) | active | canonical-path migration, compatibility aliases, client/server cutover order, retirement policy | `026`, `118`, `g05.009`, `composer-api` normalization evidence | settles how Underlay apps move from older paths to canonical families without leaving alias and retirement policy app-local |
| [028-runtime-surface-and-openapi-maturity-levels.md](./028-runtime-surface-and-openapi-maturity-levels.md) | active | runtime maturity ladder for health, metrics, OpenAPI JSON, and Swagger exposure | `025`, `026`, endpoint-family matrix, six-site runtime evidence | settles lean versus rich runtime posture so audits stop treating every runtime difference as unexplained drift |
| [029-non-resource-workflow-action-route-grammar.md](./029-non-resource-workflow-action-route-grammar.md) | active | workflow-action naming and placement for restore/purge/reorder/complete/skip/claim/release-style routes | `026`, `027`, `119`, six-site workflow-route evidence | settles the repeated non-resource action grammar so workflow verbs stop drifting between CRUD, helper, and action-family styles |
| [030-auth-and-session-systems.md](./030-auth-and-session-systems.md) | active | auth boundary, sessions, MFA, WebAuthn, OAuth, browser auth runtime | `underlay-auth*`, `ts/src/client/auth.ts`, `ts/src/runtime/auth.ts`, `ts/src/patterns/auth-workflows/**` | settles the shared auth/session boundary and records current schema/runtime drift |
| [031-config-and-secrets.md](./031-config-and-secrets.md) | active | config classes, env naming, typed-config layering, local secret posture, and bootstrap boundary | `120-configuration`, `121-consumer-config-rollout-kit`, `024`, `030`, six consumer API setups | settles where env ends, typed config begins, and how normal apps should keep config and secrets from drifting into folklore |
| [032-openapi-quality-and-declaration.md](./032-openapi-quality-and-declaration.md) | active | minimum versus strong OpenAPI declaration coverage, envelope typing, helper/status schema posture, and anonymous-object quality bar | `028`, `115`, `118`, `119`, shared OpenAPI wrappers, six-site route declaration evidence | settles what good-enough OpenAPI means once an app exposes it and stops route documentation quality from drifting app by app |
| [033-error-codes-and-operator-audit.md](./033-error-codes-and-operator-audit.md) | active | stable error-code posture plus audit and operator-evidence requirements for privileged mutations and workflow actions | `010`, `029`, `030`, `060`, error/audit/security guidance and shared crate evidence | settles how stable codes, audit requirements, and durable operator evidence should work across normal Underlay apps instead of drifting by route family |
| [040-storage-blob-and-media-systems.md](./040-storage-blob-and-media-systems.md) | active | DB bootstrap, blob backends, storage ownership, media orchestration, soft delete | `underlay-db`, `underlay-blob`, `underlay-media`, `underlay-soft-delete`, `underlay-aws` | settles the durable storage/media seam and its relationship to `050` |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | active | media library contract | existing contract | does it still match implementation and repo goals |
| [051-media-library-fleet-capability-policy.md](./051-media-library-fleet-capability-policy.md) | active | fleet-wide requirement and classification policy for the admin media family | `040`, `050`, `110`, `111`, media capability matrix, `g05.004`, `g05.019` | settles whether all six consumer apps must own the full media quartet and makes missing media capability auditable drift instead of open interpretation |
| [060-jobs-events-and-operator-systems.md](./060-jobs-events-and-operator-systems.md) | active | jobs, scheduled tasks, events, audit, security alerts, rate limiting, email | `underlay-jobs`, `underlay-events`, `underlay-audit`, `underlay-security-alerts`, `underlay-ratelimit`, `underlay-email` | settles the shared operator-facing async/control-plane contract |
| [070-nightfire-and-migration-systems.md](./070-nightfire-and-migration-systems.md) | active | Nightfire content model, editor/runtime, migration-core | `underlay-nightfire`, `underlay-migration-core`, `ts/src/nightfire/**` | settles the shared content-system and migration discipline and records current TS/runtime authority drift |
| [080-ai-runtime-and-suggestions.md](./080-ai-runtime-and-suggestions.md) | active | AI runtime, provider boundary, routing candidates, generic relation suggestions | `underlay-ai-runtime`, `underlay-suggestions`, TS suggestion helpers | settles the lower AI/runtime and suggestion request boundary and records current TS authority drift |
| [090-ts-runtime-and-client-orchestration.md](./090-ts-runtime-and-client-orchestration.md) | active | runtime helpers and browser/data/navigation/media orchestration | `ts/src/runtime/**`, selected `ts/src/client/**` | settles the retained TS runtime/client seam and records the remaining runtime-vs-pattern authority drift |
| [100-shared-patterns-and-workflow-shells.md](./100-shared-patterns-and-workflow-shells.md) | active | relation selector, form shells, batch/list/reorder/navigation state, upload flows, i18n | `ts/src/patterns/**` | settles the retained workflow-shell boundary and records the remaining pattern-vs-runtime/template drift |
| [110-admin-template-system.md](./110-admin-template-system.md) | active | entity list/detail/form templates | `ts/src/templates/**`, template docs, consumer rollout evidence | settles the stable template hierarchy, extension model, and form stop point while recording rollout-era drift |
| [111-consumer-template-adoption-and-exception-policy.md](./111-consumer-template-adoption-and-exception-policy.md) | active | consumer adoption rules and exception posture for retained admin templates | `110`, template overview/docs, `g05.001` to `g05.008` convergence evidence | settles when consumer apps must use retained list/detail/form/trash/card shells and what counts as a real exception instead of template drift |
| [115-admin-resource-api-shapes.md](./115-admin-resource-api-shapes.md) | active | list/detail/tab API shapes for admin resource pages | template types, API profile guide, `underlay-reference` consumer evidence | settles the page-shaped API seam the template rollout depends on |
| [116-canonical-collection-routes-and-query-profiles.md](./116-canonical-collection-routes-and-query-profiles.md) | active | canonical collection routes, query profiles, selector/filter/page command convergence | `073`, `080`, `100`, `115`, client types, selector workflow contract | settles the next API simplification lane: one route family and one command posture across page and selector consumers |
| [117-hybrid-collection-shells.md](./117-hybrid-collection-shells.md) | active | shared shape for root/tab hybrid collection shells with batch/reorder/transform workflow | `100`, `110`, `115`, `116`, sweep `030`, Dairy `ModulesList` evidence | settles the missing middle shape between plain `EntityList` and app-owned workflow composites |
| [118-front-and-shared-read-api-shapes.md](./118-front-and-shared-read-api-shapes.md) | active | front/public/shared read envelopes, canonical read-route families, helper vs page-shaped read surfaces | current consumer routers, `020`, `115`, `116` | extends the page-shape line beyond admin templates so normal non-admin read surfaces are also declared |
| [119-helper-search-and-lookup-route-catalogue.md](./119-helper-search-and-lookup-route-catalogue.md) | active | helper-route naming, lookup/search/suggest/detect/status catalogue, bounded helper envelope rules | `020`, `026`, `118`, six-site helper cleanup evidence | settles the repeated non-resource helper family so lookup/search/status routes stop drifting as one-off exceptions |
| [120-tooling-testing-and-contract-artifacts.md](./120-tooling-testing-and-contract-artifacts.md) | active | testing helpers, devtools, scanners, guardrails, machine-readable contract artifacts | `underlay-testing`, `underlay-devtools`, `ts/src/tools/**`, `ts/src/testing/**`, `contracts/**` | settles the shared support-layer boundary and records current artifact/tooling drift |
| [121-underlay-app-review-checklist-and-audit-artifact.md](./121-underlay-app-review-checklist-and-audit-artifact.md) | active | retained app-audit checklist contract and machine-readable review artifact | `021`, `022`, `024`, `025`, `111`, `120`, app-review checklist JSON | settles how the live contract set turns into repeatable consumer-app audits instead of one-off manual review formats |
| [122-rust-public-api-inventory.md](./122-rust-public-api-inventory.md) | active | Rust public API inventory and first platform-contract migration gates | `g06.001`, Rust crate export scan, six-consumer dependency evidence | classifies stable, adapter, internal, candidate-type, and candidate-remove surfaces before the typed-boundary migration proceeds |

## Assessment Order After Coverage

After the contract set exists, assess implementation against contract in the
same order:

1. foundation and transport
2. database migration and schema workflow
3. testing posture and shared harnesses
4. new app bootstrap and bring-up
5. Rust app runtime assembly and router topology
6. auth
7. route families and access model
8. canonical-path cutovers and compatibility retirement
9. runtime surface and OpenAPI maturity
10. non-resource workflow action grammar
11. storage and media
12. jobs and operator systems
13. Nightfire and migration
14. AI and suggestions
15. TS runtime/client orchestration
16. shared patterns
17. admin templates
18. front/shared read API shapes
19. helper/search/lookup route catalogue
20. tooling and contract artifacts

## Promotion Rule

Do not promote a system into active implementation-assessment work until:

- its contract file exists
- the contract identifies the system goal, interface, invariants, extension
  points, and known caller families
- the source-of-truth location is explicit
- the next repair work can be expressed as bounded roadmap cards instead of
  exploratory drift

## Next Task

The active contract wave is live inside `g07`.

Execute `g07.005`: duplicated auth-aware fetch orchestration decision.
