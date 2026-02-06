# 015 – Unified Error Reporting Roadmap

## Overview

This roadmap defines a single, canonical error-reporting path for Underlay-based APIs.

The target outcome is:

1. One default handler return pattern.
2. One shared Underlay error type that captures both client-safe errors and internal debugging context.
3. One migration path that can be applied consistently across `acme-*` and other Underlay-based apps.

## Progress Checklist

- [x] Phase 15.1 complete
- [x] Phase 15.2 complete
- [x] Phase 15.3 complete
- [ ] Phase 15.4 complete
- [ ] Phase 15.5 complete
- [ ] Validation plan complete
- [ ] Success metrics achieved

## Problem Statement

Current error logging middleware and schema are in place, but real-world logs are often not actionable because:

1. Route handlers frequently return raw status responses (`StatusCode::...into_response()`), which bypass structured error details.
2. `error_response_with_context()` exists but is rarely used in app routes.
3. The easy path today is not the detailed path, so most logged errors contain only status and endpoint.

This results in generic error logs that do not provide enough context to diagnose failures quickly.

## Goals

1. Make detailed error logging the default, not optional.
2. Keep all core functionality in Underlay (`underlay-http` and docs), not app-specific helpers.
3. Preserve backwards compatibility during migration.
4. Provide clear, low-friction upgrade steps for existing apps.

## Non-Goals

1. Replace app-specific business error codes.
2. Expose sensitive internals to clients.
3. Introduce a hard-breaking migration in one release.

## Canonical API Design

Introduce a unified error type in `underlay-http`:

1. `ApiError`
2. `ApiResult<T> = Result<T, ApiError>`

`ApiError` should include:

1. `status: StatusCode`
2. `code: String` (client-visible)
3. `message: String` (client-visible)
4. `field_errors: Option<HashMap<String, String>>` (client-visible, optional)
5. `context: serde_json::Value` (log-only diagnostic context)
6. optional internal cause details (`source` string or structured value) for logs only

`ApiError` implements `IntoResponse` and always:

1. emits the standard Underlay envelope
2. emits `x-error-code`
3. emits `x-error-message`
4. emits `x-error-context`

This ensures middleware can always persist useful context without per-handler header wiring.

## Ergonomic Constructors and Helpers

Add builders in `underlay-http`:

1. `ApiError::bad_request(code, message)`
2. `ApiError::not_found(code, message)`
3. `ApiError::conflict(code, message)`
4. `ApiError::internal(code, message)`
5. `.with_field_errors(map)`
6. `.with_context(json!(...))`
7. `.with_cause(&err)` (stores cause in log context, not response body)

Add `Result` helpers:

1. `ApiResultExt::map_api_err(...)`
2. `ApiResultExt::map_internal(...)`
3. `ApiResultExt::with_error_context(...)`

These helpers reduce boilerplate so rich context remains easy to add.

## Phased Implementation

## Phase 15.1 – Underlay Core API

### Tasks

- [x] Add `ApiError` and `ApiResult` to `underlay-http`.
- [x] Implement `IntoResponse` for `ApiError`.
- [x] Ensure generated response headers are compatible with existing error logging middleware.
- [x] Add unit tests for response envelope and headers.

### Implementation Tickets (Phase 15.1)

- [x] `underlay/rust/crates/underlay-http/src/errors.rs`
  - Add `ApiError` struct, constructors, `with_context`, `with_cause`, `with_field_errors`.
  - Add `ApiResult<T>` alias.
  - Implement `IntoResponse` for `ApiError` with required headers.
- [x] `underlay/rust/crates/underlay-http/src/lib.rs`
  - Export `ApiError`, `ApiResult`, and helper traits/builders.
- [x] `underlay/rust/crates/underlay-http/src/error_logging.rs`
  - Verify middleware consumes headers emitted by `ApiError` without changes.
  - Add compatibility assertions/comments where behavior coupling exists.
- [x] `underlay/rust/crates/underlay-http/src/errors_tests.rs`
  - Add tests for envelope shape and `x-error-code`, `x-error-message`, `x-error-context`.
  - Add tests that field errors and context serialization are preserved.
- [x] `underlay/rust/crates/underlay-http/ERROR_LOGGING.md`
  - Update examples to use `ApiError` as the primary path.
  - Mark `error_response_with_context` as compatibility fallback.

### Acceptance Criteria

- [x] A handler returning `Err(ApiError::internal(...))` produces:
  - correct status
  - standard error envelope
  - `x-error-code`, `x-error-message`, `x-error-context`
- [x] Existing middleware records all required fields without route-specific code.

## Phase 15.2 – Documentation Canonicalization

### Tasks

- [x] Update `underlay/docs/guides/070-api-handlers.md` to make `ApiResult` + `ApiError` the primary pattern.
- [x] Update `underlay/docs/guides/078-error-logging.md` to position `error_response_with_context()` as legacy/migration-only.
- [x] Add a concise “Do / Don’t” section:
  - Do: return `ApiResult`
  - Don’t: return raw status for errors in handlers

### Implementation Tickets (Phase 15.2)

- [x] `underlay/docs/guides/070-api-handlers.md`
  - Add canonical handler signatures using `ApiResult`.
  - Replace primary examples that currently use raw `error_response(...)`.
  - Add a lintable rule section for route modules (no raw `StatusCode::...into_response()` in error branches).
- [x] `underlay/docs/guides/078-error-logging.md`
  - Make `ApiError` examples first-class and default.
  - Move `error_response_with_context(...)` to migration/fallback section.
  - Add “safe context payload” examples and “unsafe context” anti-examples.
- [x] `underlay/docs/patterns/000-index.md`
  - Update error-handling pattern references to point to the new canonical `ApiError` flow.
- [x] `underlay/docs/guides/200-project-sync.md`
  - Add explicit upgrade checklist items for adopting `ApiError` and removing raw status error returns.

### Acceptance Criteria

- [x] Guides have one recommended pattern, not multiple equivalent patterns.
- [x] Migration fallback is documented but clearly secondary.

## Phase 15.3 – Compatibility and Migration Layer

### Tasks

- [x] Keep `error_response()` and `error_response_with_context()` available for compatibility.
- [x] Add deprecation guidance in docs (soft deprecation first).
- [x] Add optional lint/check script for apps to detect raw error status returns in route modules.

### Acceptance Criteria

- [x] Existing apps compile unchanged.
- [x] New apps and updated apps can standardize on `ApiError` without custom wrappers.

## Phase 15.4 – Reference App Migration (`acme-api`)

### Tasks

- [x] Replace direct `StatusCode::...into_response()` error branches in route handlers with `ApiError`.
- [x] Replace `error_response(...)` callsites with `ApiError` where practical.
- [x] Add context for high-value failure paths first:
  - DB operations
  - external integrations
  - auth/session operations
- [x] Keep business error codes stable.

### Implementation Tickets (Phase 15.4)

- [x] `acme-api/crates/api/src/error.rs`
  - Re-export or wrap Underlay `ApiError` / `ApiResult` so handlers have one local import surface.
  - Keep backward-compatible shim functions during migration window.
- [x] `acme-api/crates/api/src/routes/admin/users.rs`
  - Convert raw status error returns to `ApiError`.
  - Add structured context for DB failure branches (`operation`, `user_id`, `query params`).
- [x] `acme-api/crates/api/src/routes/admin/dashboard.rs`
  - Replace generic 500 status returns with typed `ApiError::internal(...)`.
  - Include failing stat query identifier in context.
- [x] `acme-api/crates/api/src/routes/admin/media.rs`
  - Convert high-volume 500/404 branches to `ApiError`.
  - Add context for storage/db operation, media ID/version ID, and operation type.
- [x] `acme-api/crates/api/src/routes/tasks.rs`
  - Convert raw status error returns for authorization/not-found/internal branches.
  - Preserve existing business semantics while adding logging context.
- [x] `acme-api/crates/api/src/routes/admin/tasks.rs`
  - Convert raw status error branches and add operation-level context.
- [x] `acme-api/crates/api/src/routes/admin/projects.rs`
  - Convert remaining raw status branches and include project/task identifiers in context.
- [x] `acme-api/crates/api/src/routes/admin/categories.rs`
  - Convert remaining raw status branches and include category identifiers in context.
- [x] `acme-api/crates/api/src/routes/admin/validation.rs`
  - Ensure validation endpoint failures emit structured `ApiError` context.
- [x] `acme-api/crates/api/src/routes/admin/activity.rs`
  - Ensure list/fetch failures emit structured `ApiError` context.
- [x] `acme-api/crates/api/src/routes/mod.rs`
  - Confirm no route-level adapters bypass `ApiError` response path.
- [x] `acme-api/crates/api/src/routes/admin/error_logs.rs`
  - Convert error listing/getting routes to `ApiError`.
  - Add structured context for list/get failures.
- [x] `acme-api/crates/api/src/routes/admin/scheduled_tasks.rs`
  - Convert list/get/toggle/trigger handlers to `ApiError`.
  - Add structured context for SQL and job trigger failures.
- [x] `acme-api/crates/api/src/routes/admin/jobs.rs`
  - Convert list/get/cancel/retry/stats handlers to `ApiError`.
  - Add structured context for job lifecycle failures.
- [x] `acme-api/crates/api/src/routes/admin/captured_emails.rs`
  - Convert list/get/delete handlers to `ApiError`.
  - Add structured context for filter parsing and DB failures.
- [x] `acme-api/crates/api/src/routes/shared/account.rs`
  - Convert profile load/update handlers to `ApiError`.
  - Add structured context for profile persistence failures.
- [x] `acme-api/crates/api/src/routes/shared/auth.rs`
  - Migrate helper and endpoint error paths incrementally to `ApiError`.
  - Preserve cookie/header behavior while replacing generic error responses.
- [x] Verification sweep (`acme-api` routes)
  - Run grep check to identify remaining `StatusCode::...into_response()` error branches.
  - Track remaining migrations as explicit TODO items until zero.

### Acceptance Criteria

- [ ] Error logs consistently include message and context for migrated handlers.
- [ ] `handler_context` null-rate is measurably reduced on real failures.

## Phase 15.5 – Downstream App Upgrade Playbook

### Tasks

- [x] Publish a short upgrade checklist in docs:
  - update Underlay version
  - migrate handler return types
  - convert raw status error returns
  - verify logs in admin UI
- [x] Include copy-paste migration examples for common handler patterns.

### Acceptance Criteria

- [ ] Another Underlay-based app can migrate with minimal local glue code.
- [ ] Upgrade path requires no app-specific forks of error infrastructure.

## Security and Data Hygiene Requirements

All phases must preserve strict separation between:

1. client-visible error payload (safe)
2. log context (diagnostic, potentially sensitive)

Rules:

- [x] Never include credentials, tokens, secrets, or raw PII in `context`.
- [x] Add explicit examples of safe context fields.
- [x] Prefer IDs, operation names, and failure class over raw payload dumps.

## Validation Plan

- [x] Unit tests in `underlay-http` for header and envelope behavior.
- [x] Integration tests for middleware capture path (`ApiError` -> response headers -> `platform.error_log.context`).
- [x] Reference app smoke test:
  - force a known DB failure
  - verify logged `error_code`, `message`, and `handler_context` fields are present and useful.

## Success Metrics

- [ ] 0 new route handlers using raw `StatusCode::...into_response()` for error branches in migrated apps.
- [ ] Significant reduction in error logs with empty/`null` handler context.
- [ ] Faster diagnosis in admin error logs without requiring terminal-only reproduction.

## Migration Policy

1. Introduce canonical API in a minor release.
2. Keep compatibility helpers for at least one full release cycle.
3. After migration window, consider stronger deprecation messaging in docs and checks.

## Deliverables

- [x] `underlay-http`: `ApiError`, `ApiResult`, helpers, tests.
- [x] Updated guides: `070-api-handlers.md`, `078-error-logging.md`.
- [x] Reference migration in `acme-api`.
- [x] Reusable downstream upgrade checklist.
