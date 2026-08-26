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
- Rust `QueryParams` (in `underlay-http`) owns the HTTP-side wire parsing of
  `sort=`/`filter[...]` into the shared query model
- the shared query model and SQL generation - `FilterField`, `FilterOperator`,
  `SortField`, `SortDirection`, `WhereBuilder`, `FieldMapping`, `SqlValue` -
  live in the `underlay-query` crate (not `underlay-http`), so database-layer
  code builds SQL without depending on the HTTP crate. `underlay-http`
  re-exports these at `underlay_http::query::*` for compatibility; new
  db-layer callers should import from `underlay-query`
- Rust page/limit parsing is `underlay_http::PagePaginationParams` (offset);
  cursor/keyset pagination is `underlay_db::pagination::CursorPaginationParams`
  (the two were disambiguated in g08.017)
- TS `buildQueryString()`, `appendQueryParams()`, and
  `queryParamsToFlatRecord()` must serialize this same vocabulary
- merging new query params must replace existing keys rather than duplicate
  them

This transport contract covers structure only. It does not define which fields
each resource allows for sorting or filtering.

### Pagination boundary

Underlay owns three distinct list-response shapes:

- primitive list envelope: `ListResponse<T>` for simple lists
- legacy offset envelope: `Paginated<T>` with:
  - `data`
  - `pagination.page`
  - `pagination.limit`
  - `pagination.total`
  - `pagination.total_pages`
- canonical page-shaped resource envelope: `PageList<T>` with raw wire fields
  `data`, `total`, and `has_more`, as defined by contract `115`; TypeScript may
  normalize this to `PagedListResponse<T>` with `hasMore` at its public client
  boundary

Rules:

- `Paginated<T>` is an opt-in higher transport shape, not a replacement for the
  primitive list envelope
- `PageList<T>` is the normal page-shaped resource response once a route owns
  total-count meaning; it is not the same shape as legacy `Paginated<T>`
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
- client IP resolution honours a declared trusted-proxy boundary
  (`TrustedProxyConfig` request extension). The default trusts no forwarding
  headers and uses the socket peer address; `x-forwarded-for` (rightmost
  untrusted hop), `x-real-ip`, or `cf-connecting-ip` are consulted only under
  the matching declared topology. Forwarding headers are never trusted
  implicitly - the resolved IP feeds rate limiting and security alerting
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
- auth, refresh, logged-in, and CSRF cookie emission should go through
  `underlay-http` builders instead of route-local `Set-Cookie` string assembly
- new config code should prefer fallible typed setters such as
  `try_with_domain`, `try_with_cookie_prefix`, and
  `try_with_refresh_token_path` so invalid cookie settings fail before serving
  traffic

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
- default connect and total timeouts on every profile (no unbounded hangs)
- two profiles: `HttpClient::new` (internal/trusted targets) and
  `HttpClient::external` (untrusted/user-influenced targets), the latter
  SSRF-guarded - `validate_external_url` and the redirect policy reject
  private, loopback, link-local, and unspecified hosts (incl. the cloud
  metadata endpoint) and constrain redirect hops
- server-side proxies over this client that interpolate caller input into an
  upstream URL must validate that input first (the embed proxy restricts `id`
  to `[A-Za-z0-9_-]+`)

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

Resolved assessment:

- `g06.181` normalized the richer validation-rejection helper identified in the
  foundation contract. `ValidatedJsonRejection` now returns the canonical
  `ErrorEnvelope` for malformed JSON and validation failures, with optional
  `error.fieldErrors` only when field-scoped feedback exists.

These are assessment hooks, not reasons to widen the contract.

`g07.022` re-audited the TS query and pagination surfaces. `client/query`
remains the focused TS owner for the shared query-string vocabulary. Its
combined sort/filter/page shape is intentional for browser route-state and API
command callers. `client/pagination` remains a cursor-pagination compatibility
export; page-shaped admin/resource browse surfaces should prefer
`client/page-lists` plus `client/envelopes` `PagedListResponse<T>`.

`g07.023` re-audited TS HTTP retry and timeout behavior. The implementation is
intentional: retry applies only to idempotent methods and configured retryable
HTTP statuses; network failures and timeout aborts are normalized to
`UnderlayHttpError(0)` without retry. Timeout protection is also limited to
idempotent methods.

`g10.011` confirmed the declared query vocabulary for valid inputs, pagination
defaults, UUID path parsing, trusted-proxy resolution, cookies, browser HTTP
behavior, CORS, in-process cache helpers, server config, and CSP helpers.

Confirmed repair hooks:

- `g10.012`: `ContextError` and the request-context rejection seam still emit
  plain text instead of the canonical error envelope
- `g10.013`: Rust and contract `115` declare raw `PageList<T>.has_more`, while
  OpenAPI declares `hasMore` and architecture prose attributes the flat shape
  to the wrong Rust type
- `g10.014`: infallible Rust HTTP-client fallback paths can discard Underlay's
  configured connect and total timeouts

Material ambiguity remains for invalid filter operators. Rust accepts symbolic
aliases and falls back unknown operators to equality; TypeScript casts
word-like operators into the closed type. Set reject/ignore/normalize policy
before changing either parser.

## Assessment Questions

Use this contract to judge later implementation work:

- do Rust and TS still speak the same query and error vocabulary
- does a shared helper encode transport structure or drift into domain policy
- is pagination returned only where the route actually owns total-count meaning
- do retry, timeout, and refresh behaviors remain explicit and bounded
- are CSP/security-header helpers still generic server glue rather than app
  runtime ownership

## Next Task

Execute `g10.012`: normalize shared context extractor failures through the
canonical error envelope.
