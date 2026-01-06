# 001 – Underlay Extraction Roadmap (Farmyard + Nursery)

This roadmap defines a step-by-step plan to extract reusable, app-agnostic foundation code from:

- Acowtancy backend: `farmyard/`
- Songsprout backend: `nursery/`

…and centralise it in Underlay, without losing flexibility for future products.

Guiding rule: Underlay ships *primitives and patterns*, not app domains.

## 0. Definitions

- **App-specific**: domain nouns, routes, tables, DTO fields tied to a product.
- **Reusable**: infrastructure concerns with stable interfaces (errors, auth boundary, tracing, migrations runners, outbox mechanics).
- **Reference implementation**: Acowtancy is furthest along; Nursery has useful “missing pieces” (request-id, metrics, outbox) but is green.

## 1. How To Use This Roadmap

- Every actionable item is a checkbox.
- Tick items with `[x]` when complete.
- Also tick the *phase header checkbox* once all of its children are complete.

## 2. Phase Checklist (high-level)

- [x] Phase 1 — Core contract + primitives aligned
- [x] Phase 2 — Observability + request identity
- [x] Phase 3 — HTTP/Axum conventions
- [x] Phase 4 — Auth boundary
- [x] Phase 5 — DB helpers + migration runners
- [x] Phase 6 — Soft delete semantics
- [x] Phase 7 — Events/outbox + jobs runner
- [x] Phase 8 — Metrics

---

## Phase 1 — Core Contract + Primitive Alignment (low risk)

- [x] Underlay: confirm `underlay-core` is the canonical home for `AppError`, `ErrorEnvelope`, `SingleResponse<T>`, `ListResponse<T>`, UUIDv7 (`Uuid`, `IdGenerator`).
- [x] Underlay: ensure TS envelope exports in `@decodelabs/underlay/client` match Rust envelope JSON shape.
- [x] Underlay: document envelope + error code conventions under `docs/architecture/`.

- [x] Acowtancy: update API boundary helpers to return Underlay envelopes (`ErrorEnvelope`, `SingleResponse<T>`, `ListResponse<T>`).
- [x] Acowtancy: ensure error sites emit stable string codes.

- [x] Nursery: replace local core primitives with Underlay (`AppError`, UUIDv7 wrapper, pagination/time if adopted).

- [x] Verify: both backends compile while importing `underlay-core`.
- [x] Verify: envelope JSON shape is identical across systems.

Reference sources:
- Farmyard: `farmyard/crates/core/src/error.rs`, `farmyard/crates/core/src/id.rs`
- Nursery: `nursery/crates/core/src/error.rs`, `nursery/crates/core/src/id.rs`, `nursery/crates/core/src/pagination.rs`, `nursery/crates/core/src/time.rs`

---

## Phase 2 — Observability + Request Identity (high value, widely reusable)

- [x] Underlay: create `underlay-observability` crate (Rust).
- [x] Underlay: implement `init_tracing(config)` (pretty local, JSON prod).
- [x] Underlay: standardise request id header (`x-request-id`) and propagation rules (always include in responses).
- [x] Underlay: provide optional axum/tower layers (`request_id_layer()`, `trace_layer()`) with consistent span fields.

- [x] Acowtancy: adopt Underlay request id middleware and remove ad-hoc correlation id behaviour.
- [x] Acowtancy: ensure logs and error responses share the same request id.

- [x] Nursery: adopt Underlay request id middleware.
- [x] Nursery: fix middleware bugs during adoption (rate limiter + client IP fallback).

- [x] Verify: every request gets a stable request id (client-provided or generated).
- [x] Verify: logs, traces, and error responses share the same request id.

Reference sources:
- Farmyard: `farmyard/crates/infra/src/lib.rs`, `farmyard/crates/infra/src/config.rs`
- Nursery: `nursery/crates/infra/src/tracing.rs`, `nursery/crates/api/src/middleware.rs`

---

## Phase 3 — HTTP/Axum Conventions (envelopes, errors, middleware)

- [x] Underlay: create `underlay-http` crate (Rust, axum-specific).
- [x] Underlay: implement `error_response(status, AppError)` helper returning `ErrorEnvelope` consistently.
- [x] Underlay: implement response helpers for `SingleResponse<T>` and `ListResponse<T>`.
- [x] Underlay: add CORS helper allowing `x-request-id` and API version headers.
- [x] Underlay: define an optional `ErrorLogSink` trait (apps decide persistence).

- [x] Acowtancy: keep endpoints local, but standardise error mapping + response envelopes via Underlay helpers.
- [x] Nursery: replace “new request id per error” behaviour with a shared request id.

- [x] Verify: endpoints emit consistent JSON envelopes and status code semantics.

Reference sources:
- Farmyard: `farmyard/crates/api/src/main.rs` (envelope shapes, `error_response` pattern)
- Nursery: `nursery/crates/api/src/error.rs`, `nursery/crates/api/src/middleware.rs`

---

## Phase 4 — Auth Boundary (provider abstraction + principal type)

- [x] Underlay: create `underlay-auth` crate (Rust).
- [x] Underlay: define `Principal` (user id + claims) and an app-extensible role/capability approach.
- [x] Underlay: define `AuthProvider` trait (verify token/session) and stable error mapping.
- [x] Underlay: provide axum extractor `Authenticated<P>` built on the provider.

- [x] Acowtancy: ensure `AuthenticatedUser` extraction calls a provider (not ad-hoc header logic).
- [x] Nursery: introduce the same boundary early (even if provider is dev-only initially).

- [x] Verify: auth verification can be swapped without changing handlers.

Reference sources:
- Farmyard: `farmyard/crates/auth/src/provider.rs`, `farmyard/crates/auth/src/principal.rs`

---

## Phase 5 — DB Helpers + Migration Runners (keep schemas app-owned)

- [x] Underlay: create `underlay-db` crate (Rust).
- [x] Underlay: implement Postgres pool creation + common SQLx wiring helpers.
- [x] Underlay: implement migration runner helpers.
- [x] Underlay: implement dev reset patterns (guarded by environment).
- [x] Underlay: decide whether to add `underlay-migrations` for optional template migrations.
  - Decision: keep migrations app-owned for now; no `underlay-migrations` crate.

- [x] Acowtancy: switch pool + migrator setup to Underlay helpers; keep migrations in `farmyard/migrations/`.
- [x] Nursery: switch pool + migrator setup to Underlay helpers; keep migrations in `nursery/migrations/`.

- [x] Verify: both apps can run `migrate_dev_db`-equivalent flows using Underlay helpers.

Reference sources:
- Farmyard: `farmyard/crates/db/src/lib.rs`, `farmyard/crates/db/src/bin/migrate_dev_db.rs`, `farmyard/crates/db/src/bin/reset_dev_db.rs`
- Nursery: `nursery/crates/db/src/lib.rs`

---

## Phase 6 — Soft Delete Semantics (generic; cascades remain app-local)

- [x] Underlay: add `underlay-soft-delete` (crate or module under `underlay-db`).
- [x] Underlay: standardise column naming (`deleted_at`, `delete_batch_id`).
- [x] Underlay: standardise result enums (`NotFound`, `AlreadyDeleted`, `Deleted { batch_id }`).
- [x] Underlay: standardise restore semantics by batch id.

- [x] Acowtancy: keep cascade queries local, but use shared Underlay semantics/result types.
- [x] Nursery: adopt `underlay-soft-delete` semantics for a first Nursery table (`platform.api_keys`).

- [x] Verify: soft-delete operations across apps communicate state consistently.

Reference sources:
- Farmyard: `farmyard/crates/db/src/learning.rs`, `farmyard/crates/db/tests/learning_soft_delete.rs`
- Nursery: `nursery/migrations/202601061340__create_platform_api_keys.sql`, `nursery/crates/db/src/platform.rs`, `nursery/crates/db/tests/api_keys_soft_delete.rs`

---

## Phase 7 — Events / Outbox + Jobs Runner (harder; big payoff)

- [x] Underlay: create `underlay-events` crate (Rust).
- [x] Underlay: define event record type (id, type, payload, occurred_at, processed_at).
- [x] Underlay: define writer interface and recommended Postgres schema (template SQL).
- [x] Underlay: create `underlay-jobs` crate (Rust) with runner skeleton + handler registration.

- [x] Nursery: harden outbox processing (don’t mark processed unless delivery succeeds; define retry/backoff).
- [x] Acowtancy: adopt once interface is stable; start with a single event type.

- [x] Verify: one app can emit events and a worker processes them with correct locking semantics.

Reference sources:
- Farmyard: `farmyard/migrations/202601061230__create_platform_domain_events.sql`, `farmyard/crates/db/src/platform.rs`, `farmyard/crates/jobs/src/outbox.rs`, `farmyard/crates/jobs/src/lib.rs`
- Nursery: `nursery/migrations/202601021339__create_domain_events.sql`, `nursery/migrations/202601052200__add_domain_events_retry_fields.sql`, `nursery/crates/jobs/src/lib.rs`

---

## Phase 8 — Metrics (Prometheus) (optional, recommended)

- [x] Underlay: create `underlay-metrics` crate (Rust).
- [x] Underlay: implement registry wrapper.
- [x] Underlay: implement `/metrics` handler.
- [x] Underlay: add helper to register default process/runtime metrics.

- [x] Acowtancy: add `/metrics` endpoint to API binary and configure deployment.
- [x] Nursery: add `/metrics` endpoint to API binary and configure deployment.

- [x] Verify (local): `/metrics` works locally for both apps.
- [x] Verify (deploy): deferred until hosting is selected.

Reference sources:
- Farmyard: `farmyard/crates/api/src/main.rs`, `farmyard/Dockerfile`
- Nursery: `nursery/crates/api/src/metrics.rs`, `nursery/Dockerfile`

---

## Open Questions / Decisions

- [x] Decide Axum version strategy for Underlay HTTP/auth integrations (target Axum 0.7; `underlay-*` crates use workspace deps; consumers should align on Axum 0.7).
- [x] Decide whether Underlay ships `underlay-openapi` (utoipa helpers) vs keeping OpenAPI generation app-local (ship `underlay-openapi` as an optional utoipa-only crate for envelopes + common schemas).
- [x] Decide whether Underlay ships template infra migrations (error log table, outbox table) as optional imports (ship optional SQL templates per crate; apps own migrations).
- [x] Decide API version header naming convention (app-specific prefix vs generic) (keep app-specific header names; Underlay only standardises `x-request-id` and `x-error-code`).
