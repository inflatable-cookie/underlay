# Contracts

Contracts hold durable reference artifacts such as schemas, inventories, and
machine-readable policy files.

## Current contracts

| Contract | Scope | Description |
|----------|-------|-------------|
| [001-working-rules.md](./001-working-rules.md) | M | Shared authoring and delivery rules for Underlay work |
| [010-foundation-primitives-and-envelopes.md](./010-foundation-primitives-and-envelopes.md) | M | Shared id, success-envelope, error-envelope, and validation-boundary contract |
| [020-http-transport-and-server-boundary.md](./020-http-transport-and-server-boundary.md) | M | Shared Rust and TS HTTP helpers for envelopes, query, pagination, cookies, context, caching, and CSP/security headers |
| [030-auth-and-session-systems.md](./030-auth-and-session-systems.md) | M | Shared auth provider, session, credential-family, browser auth-store, passkey, OAuth, and retained workflow-shell contract |
| [040-storage-blob-and-media-systems.md](./040-storage-blob-and-media-systems.md) | M | Shared DB/bootstrap, soft-delete, blob adapter, storage-key, and lower media repository/storage contract |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | L | Shared media asset, usage-graph, structured-content sync, and migration replay contract |
| [060-jobs-events-and-operator-systems.md](./060-jobs-events-and-operator-systems.md) | M | Shared job queue, scheduler, outbox, audit, email, rate-limit, and security-alert contract |
| [070-nightfire-and-migration-systems.md](./070-nightfire-and-migration-systems.md) | L | Shared Nightfire durable value, strategy, TS runtime shell, and migration-core pipeline/replay contract |
| [080-ai-runtime-and-suggestions.md](./080-ai-runtime-and-suggestions.md) | M | Shared Rust AI runtime, resilience/fallback helpers, suggestion query contract, and thin TS suggestion-param shell |
| [090-ts-runtime-and-client-orchestration.md](./090-ts-runtime-and-client-orchestration.md) | L | Retained `runtime/*` subpath model, browser auth/controller layer, SvelteKit glue, and client-side orchestration seam |
| [100-shared-patterns-and-workflow-shells.md](./100-shared-patterns-and-workflow-shells.md) | L | Shared SPA form shell, auth-aware loading, relation selector, list/reorder/upload controllers, optimistic helpers, and retained workflow-shell contract |
| [110-admin-template-system.md](./110-admin-template-system.md) | L | Shared three-level admin template system contract for list/detail/form page shapes, section reuse, and declarative extension |
| [115-admin-resource-api-shapes.md](./115-admin-resource-api-shapes.md) | M | Shared list/detail/tab API shape contract for page-shaped admin resource surfaces and child collection tabs |
| [116-canonical-collection-routes-and-query-profiles.md](./116-canonical-collection-routes-and-query-profiles.md) | M | Shared canonical route, query-profile, and command-posture contract for page, selector, and filter collection consumers |
| [117-hybrid-collection-shells.md](./117-hybrid-collection-shells.md) | M | Shared hybrid collection-shell contract for root/tab list surfaces with batch, reorder, and transform-launch behavior |
| [120-tooling-testing-and-contract-artifacts.md](./120-tooling-testing-and-contract-artifacts.md) | M | Shared Rust/TS test harnesses, migration/schema devtools, guardrail scanners, and machine-readable contract-artifact boundary |

## Index

- [contract-index.md](./contract-index.md) is the canonical planning index for
  the current and planned contract surface.

## Current lane

- the `g04` contract-coverage and assessment generation is complete
- future contract-driven work should open a new explicit roadmap generation
