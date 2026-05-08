# Contract: HTTP Transport and Server Boundary

Status: active
Owner: repo maintainers
Depends on: `010-foundation-primitives-and-envelopes.md`

## Purpose

Define the shared HTTP transport contract Underlay owns across Rust services and
TypeScript consumers.

This contract covers:

- Rust HTTP helpers for responses, errors, query parsing, pagination, path
  parsing, cookies, context extraction, CORS, caching, and server config
- the shared Rust outbound HTTP client wrapper
- the TypeScript HTTP client, error parsing, query-string helpers, and server
  CSP/security-header helpers

It does not define auth/session meaning, storage semantics, or app-specific
server setup. Those depend on this layer and belong in later contracts.

## Sources of Truth

Primary:

- [`rust/crates/underlay-http/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/lib.rs)
- [`rust/crates/underlay-http/src/errors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/errors.rs)
- [`rust/crates/underlay-http/src/responses.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/responses.rs)
- [`rust/crates/underlay-http/src/query.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/query.rs)
- [`rust/crates/underlay-http/src/pagination.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/pagination.rs)
- [`rust/crates/underlay-http/src/path.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/path.rs)
- [`rust/crates/underlay-http/src/cookies.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/cookies.rs)
- [`rust/crates/underlay-http/src/context.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/context.rs)
- [`rust/crates/underlay-http/src/cors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/cors.rs)
- [`rust/crates/underlay-http/src/caching.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/caching.rs)
- [`rust/crates/underlay-http/src/http_config.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/http_config.rs)
- [`rust/crates/underlay-http-client/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http-client/src/lib.rs)
- [`ts/src/client/http.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/http.ts)
- [`ts/src/client/errors.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/errors.ts)
- [`ts/src/client/query.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/query.ts)
- [`ts/src/server/csp.ts`](/Users/tom/Dev/projects/underlay/ts/src/server/csp.ts)
- [`ts/src/server/csp-config.ts`](/Users/tom/Dev/projects/underlay/ts/src/server/csp-config.ts)
- [`ts/src/server/csp-types.ts`](/Users/tom/Dev/projects/underlay/ts/src/server/csp-types.ts)

Supporting:

- [`docs/contracts/010-foundation-primitives-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- [`docs/architecture/015-error-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/architecture/015-error-and-envelopes.md)
- [`docs/architecture/030-ts-api-client.md`](/Users/tom/Dev/projects/underlay/docs/architecture/030-ts-api-client.md)

If these diverge, the shared code wins. Architecture docs must catch up.

## Contract Goal

Underlay should provide one transport layer that keeps Rust and TS callers
structurally aligned:

- same envelope expectations
- same query-string vocabulary
- same basic auth-cookie and 401 retry seams
- same operator-facing request metadata headers
- same security-header and CSP composition boundary

The point is consistency, not a heavyweight framework.

## Shared Boundary

### Success and error responses

The base success/error envelope rules come from
[`010-foundation-primitives-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md).

Transport helper rules:

- Rust `ok()` and `created()` emit `SingleResponse<T>`.
- Rust `list_ok()` emits `ListResponse<T>`.
- Rust `no_content()` emits `204 No Content` with no JSON body.
- Rust `error_response()` and `ApiError` emit the canonical error envelope.
- TS `createHttpClient()` treats canonical error envelopes as first-class and
  raises `UnderlayHttpError`.

### Query-string contract

The shared query vocabulary is:

- `sort=field1:asc,field2:desc`
- `filter[field]=value`
- `filter[field][op]=value`
- `page=<1-indexed page>`
- `limit=<page size>`

Rules:

- sort directions are `asc` and `desc`
- filter operators are `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, and `like`
- Rust `QueryParams` owns sort/filter parsing
- Rust `PaginationParams` owns page/limit parsing and clamping
- TS `buildQueryString()`, `appendQueryParams()`, and
  `queryParamsToFlatRecord()` must serialize this same vocabulary
- merging new query params must replace existing keys rather than duplicate
  them

This transport contract covers structure only. It does not define which fields
each resource allows for sorting or filtering.

### Pagination boundary

Underlay owns two distinct list-response shapes:

- primitive list envelope: `ListResponse<T>` for simple lists
- paginated list envelope: `Paginated<T>` with:
  - `data`
  - `pagination.page`
  - `pagination.limit`
  - `pagination.total`
  - `pagination.total_pages`

Rules:

- `Paginated<T>` is an opt-in higher transport shape, not a replacement for the
  primitive list envelope
- page numbering is 1-indexed
- default limit is `20`
- shared default max limit is `100` unless a caller clamps lower
- query params and returned pagination metadata must agree on the same
  `page`/`limit` language

### Path parsing

`parse_uuid_path()` and `parse_uuid_path_raw()` are the shared path-ID helpers.

Rules:

- invalid UUID path parameters become `400 Bad Request`
- transport error code for this case is `validation.invalid_id`
- the boundary stays on UUID parsing; route-specific existence and permission
  semantics belong elsewhere

### Request context and headers

`RequestContext` and `AuthenticatedContext` define the shared request-metadata
seam.

Rules:

- `x-request-id` is the canonical request-correlation header
- client IP extraction may use `x-forwarded-for`, `x-real-ip`, or
  `cf-connecting-ip`
- authenticated user identity may be attached through request extensions and
  surfaced as `AuthenticatedUser`
- `AuthenticatedContext` is the shared extractor for routes that require a user
  id in request context

This contract does not define the auth middleware that sets the user id. It
only defines the extraction seam and header vocabulary.

### Cookie boundary

`AuthCookieConfig` is the shared cookie-composition contract for browser auth
transport.

Rules:

- refresh-token cookie is `HttpOnly`
- refresh-token cookie path defaults to `/v1/auth`
- logged-in indicator cookie is readable by JavaScript
- `SameSite`, `Secure`, `Domain`, max age, and cookie prefix are explicit
  config knobs
- shared helpers set and clear the refresh-token and logged-in cookies
  together

This is a transport contract, not an auth policy contract. Token contents,
rotation rules, MFA, and session semantics are out of scope here.

### Browser and server HTTP client behavior

The TS `HttpClient` contract is intentionally small.

Rules:

- the shared client accepts a base URL and optional default headers
- request bodies are JSON-encoded when present
- `Accept: application/json` is added when absent
- `Authorization: Bearer <token>` is attached when an access token exists and
  the request did not already provide `Authorization`
- 401 refresh is coordinated through one `refreshInFlight` path to avoid
  refresh storms
- retries are only for idempotent methods (`GET`, `DELETE`) and only on the
  configured retryable statuses
- timeout protection only applies to idempotent methods
- `requestWithMeta()` preserves status and headers; `request()` returns the
  parsed body only
- `204` and `304` may return `null` bodies

The Rust `underlay-http-client` contract is narrower:

- shared `reqwest::Client` setup
- Underlay user-agent default
- optional custom user-agent override

It is a convenience seam, not a domain client framework.

### CORS and caching

Underlay owns a light reusable HTTP hardening boundary:

- `CorsConfig` and `cors_layer()` for origin/header/method/credential policy
- `MicroCache` and `SingleFlight` for hot read-path suppression
- weak ETag helpers and matchers
- shared cache-control constants for admin revalidation and no-store behavior

Rules:

- wildcard origin and credentialed origin handling are mutually constrained
- mirrored origin is the shared escape hatch for credentialed local-dev setups
- cache helpers are in-process only and must not be treated as distributed
  cache infrastructure
- ETag helpers are validators, not content-version authorities

### Server config and security headers

The shared server boundary is intentionally thin:

- `HttpServerConfig` for bind/public-host/base-url composition
- TS CSP/security-header helpers for SvelteKit server code

Rules:

- bind address and public host are separate concepts
- default local posture favors localhost-only binding unless environment
  settings imply a public bind
- CSP config is nonce-capable and default-deny oriented
- TS server helpers own CSP and basic security-header composition, not the full
  server lifecycle

## Invariants

- Rust and TS transport layers must agree on the canonical error envelope and
  query-string vocabulary.
- Shared transport helpers must stay generic and app-agnostic.
- Transport helpers may normalize auth, retry, cookie, and header behavior, but
  they must not silently embed domain policy.
- Query serialization must be idempotent with respect to existing URLs: adding
  a new `limit` replaces the old one rather than duplicating it.
- Pagination metadata is only authoritative when a route explicitly returns
  `Paginated<T>`.

## Extension Points

Allowed:

- resource-local sort/filter field allowlists on top of the shared format
- route-local limit clamping below the shared default max
- app-local token refresh implementation via `HttpAuthOptions.refresh`
- app-local CSP source expansion and header overrides
- explicit route-local accepted-status handling through `requestWithMeta()`

Not allowed:

- inventing parallel shared query syntaxes for the same concepts
- bypassing the canonical error-envelope parsing path in shared clients
- treating the shared cookie helpers as proof of a specific auth/session model
- widening server helpers into app-framework ownership without a later contract

## Known Drift And Assessment Hooks

Current drift to assess later:

- Rust splits sort/filter (`QueryParams`) from page/limit (`PaginationParams`),
  while TS combines them in one `QueryParams` shape. The wire format aligns,
  but the ownership split should be checked for clarity and caller confusion.
- `ts/src/client/pagination.ts` is currently just a re-export from pattern
  types rather than a transport-specific pagination contract surface.
- `ts/src/client/http.ts` comments imply broader retry behavior than the actual
  implementation, which retries only retryable HTTP statuses and not generic
  network failures.
- the richer validation-rejection helper identified in the foundation contract
  still needs a proper transport-normalization assessment.

These are assessment hooks, not reasons to widen the contract.

## Assessment Questions

Use this contract to judge later implementation work:

- do Rust and TS still speak the same query and error vocabulary
- does a shared helper encode transport structure or drift into domain policy
- is pagination returned only where the route actually owns total-count meaning
- do retry, timeout, and refresh behaviors remain explicit and bounded
- are CSP/security-header helpers still generic server glue rather than app
  runtime ownership

## Next Task

Execute `g04.005`: write `030-auth-and-session-systems.md`.
