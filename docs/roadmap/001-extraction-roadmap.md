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

- [ ] Phase 1 — Core contract + primitives aligned
- [ ] Phase 2 — Observability + request identity
- [ ] Phase 3 — HTTP/Axum conventions
- [ ] Phase 4 — Auth boundary
- [ ] Phase 5 — DB helpers + migration runners
- [ ] Phase 6 — Soft delete semantics
- [ ] Phase 7 — Events/outbox + jobs runner
- [ ] Phase 8 — Metrics

---

## Phase 1 — Core Contract + Primitive Alignment (low risk)

- [x] Underlay: confirm `underlay-core` is the canonical home for `AppError`, `ErrorEnvelope`, `SingleResponse<T>`, `ListResponse<T>`, UUIDv7 (`Uuid`, `IdGenerator`).
- [x] Underlay: ensure TS envelope exports in `@decodelabs/underlay/client` match Rust envelope JSON shape.
- [x] Underlay: document envelope + error code conventions under `docs/architecture/`.

- [ ] Acowtancy: update API boundary helpers to return Underlay envelopes (`ErrorEnvelope`, `SingleResponse<T>`, `ListResponse<T>`).
- [ ] Acowtancy: ensure error sites emit stable string codes.

- [ ] Nursery: replace local core primitives with Underlay (`AppError`, UUIDv7 wrapper, pagination/time if adopted).

- [ ] Verify: both backends compile while importing `underlay-core`.
- [ ] Verify: envelope JSON shape is identical across systems.

Reference sources:
- Farmyard: `farmyard/crates/core/src/error.rs`, `farmyard/crates/core/src/id.rs`
- Nursery: `nursery/crates/core/src/error.rs`, `nursery/crates/core/src/id.rs`, `nursery/crates/core/src/pagination.rs`, `nursery/crates/core/src/time.rs`

---

## Phase 2 — Observability + Request Identity (high value, widely reusable)

- [x] Underlay: create `underlay-observability` crate (Rust).
- [x] Underlay: implement `init_tracing(config)` (pretty local, JSON prod).
- [x] Underlay: standardise request id header (`x-request-id`) and propagation rules (always include in responses).
- [x] Underlay: provide optional axum/tower layers (`request_id_layer()`, `trace_layer()`) with consistent span fields.

- [ ] Acowtancy: adopt Underlay request id middleware and remove ad-hoc correlation id behaviour.
- [ ] Acowtancy: ensure logs and error responses share the same request id.

- [ ] Nursery: adopt Underlay request id middleware.
- [ ] Nursery: fix middleware bugs during adoption (rate limiter + client IP fallback).

- [ ] Verify: every request gets a stable request id (client-provided or generated).
- [ ] Verify: logs, traces, and error responses share the same request id.

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

- [ ] Acowtancy: keep endpoints local, but standardise error mapping + response envelopes via Underlay helpers.
- [ ] Nursery: replace “new request id per error” behaviour with a shared request id.

- [ ] Verify: endpoints emit consistent JSON envelopes and status code semantics.

Reference sources:
- Farmyard: `farmyard/crates/api/src/main.rs` (envelope shapes, `error_response` pattern)
- Nursery: `nursery/crates/api/src/error.rs`, `nursery/crates/api/src/middleware.rs`

---

## Phase 4 — Auth Boundary (provider abstraction + principal type)

- [x] Underlay: create `underlay-auth` crate (Rust).
- [x] Underlay: define `Principal` (user id + claims) and an app-extensible role/capability approach.
- [x] Underlay: define `AuthProvider` trait (verify token/session) and stable error mapping.
- [x] Underlay: provide axum extractor `Authenticated<P>` built on the provider.

- [ ] Acowtancy: ensure `AuthenticatedUser` extraction calls a provider (not ad-hoc header logic).
- [ ] Nursery: introduce the same boundary early (even if provider is dev-only initially).

- [ ] Verify: auth verification can be swapped without changing handlers.

Reference sources:
- Farmyard: `farmyard/crates/auth/src/provider.rs`, `farmyard/crates/auth/src/principal.rs`

---

## Phase 5 — DB Helpers + Migration Runners (keep schemas app-owned)

- [ ] Underlay: create `underlay-db` crate (Rust).
- [ ] Underlay: implement Postgres pool creation + common SQLx wiring helpers.
- [ ] Underlay: implement migration runner helpers.
- [ ] Underlay: implement dev reset patterns (guarded by environment).
- [ ] Underlay: decide whether to add `underlay-migrations` for optional template migrations.

- [ ] Acowtancy: switch pool + migrator setup to Underlay helpers; keep migrations in `farmyard/migrations/`.
- [ ] Nursery: switch pool + migrator setup to Underlay helpers; keep migrations in `nursery/migrations/`.

- [ ] Verify: both apps can run `migrate_dev_db`-equivalent flows using Underlay helpers.

Reference sources:
- Farmyard: `farmyard/crates/db/src/lib.rs`, `farmyard/crates/db/src/bin/migrate_dev_db.rs`, `farmyard/crates/db/src/bin/reset_dev_db.rs`
- Nursery: `nursery/crates/db/src/lib.rs`

---

## Phase 6 — Soft Delete Semantics (generic; cascades remain app-local)

- [ ] Underlay: add `underlay-soft-delete` (crate or module under `underlay-db`).
- [ ] Underlay: standardise column naming (`deleted_at`, `delete_batch_id`).
- [ ] Underlay: standardise result enums (`NotFound`, `AlreadyDeleted`, `Deleted { batch_id }`).
- [ ] Underlay: standardise restore semantics by batch id.

- [ ] Acowtancy: keep cascade queries local, but use shared Underlay semantics/result types.

- [ ] Verify: soft-delete operations across apps communicate state consistently.

Reference sources:
- Farmyard: `farmyard/crates/db/src/learning.rs`, `farmyard/crates/db/tests/learning_soft_delete.rs`

---

## Phase 7 — Events / Outbox + Jobs Runner (harder; big payoff)

- [ ] Underlay: create `underlay-events` crate (Rust).
- [ ] Underlay: define event record type (id, type, payload, occurred_at, processed_at).
- [ ] Underlay: define writer interface and recommended Postgres schema (template SQL).
- [ ] Underlay: create `underlay-jobs` crate (Rust) with runner skeleton + handler registration.

- [ ] Nursery: harden outbox processing (don’t mark processed unless delivery succeeds; define retry/backoff).
- [ ] Acowtancy: adopt once interface is stable; start with a single event type.

- [ ] Verify: one app can emit events and a worker processes them with correct locking semantics.

Reference sources:
- Farmyard: `farmyard/crates/jobs/src/lib.rs`, `farmyard/crates/jobs/src/main.rs`
- Nursery: `nursery/migrations/202601021339__create_domain_events.sql`, `nursery/crates/platform/src/events.rs`, `nursery/crates/jobs/src/lib.rs`

---

## Phase 8 — Metrics (Prometheus) (optional, recommended)

- [ ] Underlay: create `underlay-metrics` crate (Rust).
- [ ] Underlay: implement registry wrapper.
- [ ] Underlay: implement `/metrics` handler.
- [ ] Underlay: add helper to register default process/runtime metrics.

- [ ] Acowtancy: add `/metrics` endpoint to API binary and configure deployment.
- [ ] Nursery: add `/metrics` endpoint to API binary and configure deployment.

- [ ] Verify: `/metrics` works locally and on target deployment.

Reference sources:
- Nursery: `nursery/crates/api/src/metrics.rs`, `nursery/fly.production.toml`

---

## Open Questions / Decisions

- [ ] Decide Axum version strategy for Underlay HTTP/auth integrations.
- [ ] Decide whether Underlay ships `underlay-openapi` (utoipa helpers) vs keeping OpenAPI generation app-local.
- [ ] Decide whether Underlay ships template infra migrations (error log table, outbox table) as optional imports.
- [ ] Decide API version header naming convention (app-specific prefix vs generic).
