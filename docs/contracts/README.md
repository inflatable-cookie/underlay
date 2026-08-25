# Contracts

Contracts hold durable reference artifacts such as schemas, inventories, and
machine-readable policy files.

## Current contracts

| Contract | Scope | Description |
|----------|-------|-------------|
| [001-working-rules.md](./001-working-rules.md) | M | Shared authoring and delivery rules for Underlay work |
| [010-foundation-primitives-and-envelopes.md](./010-foundation-primitives-and-envelopes.md) | M | Shared id, success-envelope, error-envelope, and validation-boundary contract |
| [020-http-transport-and-server-boundary.md](./020-http-transport-and-server-boundary.md) | M | Shared Rust and TS HTTP helpers for envelopes, query, pagination, cookies, context, caching, and CSP/security headers |
| [021-database-migration-and-schema-workflow.md](./021-database-migration-and-schema-workflow.md) | M | Shared durable schema migration, dev-overlay, reset/replay, and proof contract for normal Underlay app databases |
| [022-testing-posture-and-shared-harnesses.md](./022-testing-posture-and-shared-harnesses.md) | M | Shared minimum and strong proof posture plus shared Rust/TS harness usage contract for normal Underlay app packages |
| [023-release-and-compatibility-rollout.md](./023-release-and-compatibility-rollout.md) | M | Shared fleet rollout, deprecation-window, upgrade-note, and compatibility-retirement proof contract for Underlay changes that affect consumer repos |
| [024-new-app-bootstrap-and-bring-up.md](./024-new-app-bootstrap-and-bring-up.md) | M | Single-repository `apps/*`/`packages/*` workspace topology, root Bun manifest, dependency, docs-authority, Effigy-first bootstrap, and bring-up contract for normal Underlay apps |
| [025-rust-app-runtime-assembly-and-router-topology.md](./025-rust-app-runtime-assembly-and-router-topology.md) | M | Shared Rust API workspace shape, thin entrypoint posture, `AppState`, router builder, middleware order, and operational endpoint/runtime assembly contract |
| [026-route-families-and-access-model.md](./026-route-families-and-access-model.md) | M | Shared route-family taxonomy and access-model contract for runtime, shared, front/public, and admin API surfaces |
| [027-api-canonical-path-cutovers-and-compatibility-retirement.md](./027-api-canonical-path-cutovers-and-compatibility-retirement.md) | M | Shared canonical-path migration, compatibility-alias, and retirement-order contract for API route cutovers |
| [028-runtime-surface-and-openapi-maturity-levels.md](./028-runtime-surface-and-openapi-maturity-levels.md) | M | Shared runtime maturity ladder for health, metrics, OpenAPI JSON, and Swagger exposure across Underlay API apps |
| [029-non-resource-workflow-action-route-grammar.md](./029-non-resource-workflow-action-route-grammar.md) | M | Shared workflow-action naming and placement contract for non-CRUD verbs such as restore, purge, reorder, complete, skip, claim, and release |
| [030-auth-and-session-systems.md](./030-auth-and-session-systems.md) | M | Shared auth provider, session, credential-family, browser auth-store, passkey, OAuth, and retained workflow-shell contract |
| [031-config-and-secrets.md](./031-config-and-secrets.md) | M | Shared config classes, env naming, typed-config layering, local secret posture, and bootstrap-boundary contract for normal Underlay apps |
| [032-openapi-quality-and-declaration.md](./032-openapi-quality-and-declaration.md) | M | Shared minimum versus strong OpenAPI route declaration, envelope-typing, helper/status schema, and anonymous-object quality contract for normal Underlay APIs |
| [033-error-codes-and-operator-audit.md](./033-error-codes-and-operator-audit.md) | M | Shared stable error-code, privileged-mutation audit, workflow-action audit, and operator-evidence contract for normal Underlay app workflows and admin mutations |
| [121-underlay-app-review-checklist-and-audit-artifact.md](./121-underlay-app-review-checklist-and-audit-artifact.md) | M | Shared consumer-audit checklist contract plus machine-readable review artifact for contract-backed app reviews across the Underlay app family |
| [040-storage-blob-and-media-systems.md](./040-storage-blob-and-media-systems.md) | M | Shared DB/bootstrap, soft-delete, blob adapter, storage-key, and lower media repository/storage contract |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | L | Shared media asset, usage-graph, structured-content sync, and migration replay contract |
| [051-media-library-fleet-capability-policy.md](./051-media-library-fleet-capability-policy.md) | M | Shared fleet policy for whether the full admin media family is required across the six consumer apps and how full / partial / absent media capability is classified |
| [060-jobs-events-and-operator-systems.md](./060-jobs-events-and-operator-systems.md) | M | Shared job queue, scheduler, outbox, audit, email, rate-limit, and security-alert contract |
| [070-nightfire-and-migration-systems.md](./070-nightfire-and-migration-systems.md) | L | Shared Nightfire durable value, strategy, TS runtime shell, and migration-core pipeline/replay contract |
| [080-ai-runtime-and-suggestions.md](./080-ai-runtime-and-suggestions.md) | M | Shared Rust AI runtime, resilience/fallback helpers, suggestion query contract, and thin TS suggestion-param shell |
| [090-ts-runtime-and-client-orchestration.md](./090-ts-runtime-and-client-orchestration.md) | L | Retained `runtime/*` subpath model, browser auth/controller layer, SvelteKit glue, and client-side orchestration seam |
| [100-shared-patterns-and-workflow-shells.md](./100-shared-patterns-and-workflow-shells.md) | L | Shared SPA form shell, auth-aware loading, relation selector, list/reorder/upload controllers, optimistic helpers, and retained workflow-shell contract |
| [110-admin-template-system.md](./110-admin-template-system.md) | L | Shared three-level admin template system contract for list/detail/form page shapes, section reuse, and declarative extension |
| [111-consumer-template-adoption-and-exception-policy.md](./111-consumer-template-adoption-and-exception-policy.md) | M | Shared consumer adoption, exception, and review-posture contract for when admin apps must use the retained page, trash, and card templates |
| [115-admin-resource-api-shapes.md](./115-admin-resource-api-shapes.md) | M | Shared list/detail/tab API shape contract for page-shaped admin resource surfaces and child collection tabs |
| [116-canonical-collection-routes-and-query-profiles.md](./116-canonical-collection-routes-and-query-profiles.md) | M | Shared canonical route, query-profile, and command-posture contract for page, selector, and filter collection consumers |
| [117-hybrid-collection-shells.md](./117-hybrid-collection-shells.md) | M | Shared hybrid collection-shell contract for root/tab list surfaces with batch, reorder, and transform-launch behavior |
| [118-front-and-shared-read-api-shapes.md](./118-front-and-shared-read-api-shapes.md) | M | Shared front/public/shared read envelope and helper-vs-resource shape contract for non-admin read surfaces |
| [119-helper-search-and-lookup-route-catalogue.md](./119-helper-search-and-lookup-route-catalogue.md) | M | Shared helper-route naming and bounded envelope contract for lookup, search, suggest, detect, requirements, and status routes |
| [120-tooling-testing-and-contract-artifacts.md](./120-tooling-testing-and-contract-artifacts.md) | M | Shared Rust/TS test harnesses, migration/schema devtools, guardrail scanners, and machine-readable contract-artifact boundary |
| [122-rust-public-api-inventory.md](./122-rust-public-api-inventory.md) | M | Rust public API classification and first typed-boundary migration gates for the `g06` platform-contract transition |

## Index

- [contract-index.md](./contract-index.md) is the canonical planning index for
  the current and planned contract surface.
- [app-review/underlay-app-review-checklist.json](./app-review/underlay-app-review-checklist.json)
  is the retained machine-readable review checklist artifact for consumer-app
  audits.
- [media-capability/fleet-media-capability-matrix.csv](./media-capability/fleet-media-capability-matrix.csv)
  is the retained fleet media capability inventory artifact.

## Current lane

- the active contract-driven lane is `g06`
- `g06.003` is complete as the auth/session contract reset lane
- `g06.004` is complete as the HTTP safe-builder consolidation lane
- `g06.005` is complete as the DB identifier and schema boundary normalization
  lane
- `g06.006` is complete as the media repository contract and adapter split
  completion lane
- `g06.007` is complete as the devtools bundle/store boundary isolation lane
- `g06.008` is complete as the six-consumer compatibility proof and
  release-note closeout lane
- `g06.009` is complete as the Effigy doctor structural backlog triage lane
- `g06.010` is complete as the first Rust god-file split repair batch
- `g06.011` is complete as the second Rust structural split repair batch
- `g06.012` is complete as the high-severity Rust structural backlog triage
  lane
- `g06.013` is complete as the security-adjacent Rust adapter split batch
- `g06.014` is complete as the Rust platform transition validation and
  release-readiness closeout
- `g06.015` is complete as the Rust platform transition release-note handoff
- `g06.016` is complete as the Rust platform hardening backlog batch
- `g06.017` is complete as the Rust quality re-audit and fresh-start assessment
- `g06.018` is superseded by `g06.019`
- `g06.019` is complete as the reference-grade architecture reset inventory
- `g06.020` is next as the public Rust surface diet and consumer import matrix
- keep fresh Rust platform-contract work inside `g06`
