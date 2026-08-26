# g10.011 Foundation And Transport Contract Assessment

Date: 2026-08-26
Roadmap: `g10.011`
Contracts: `010`, `020`
Status: complete
Posture: `strict-ready`

## Outcome

Verdict: `strained`.

The shared foundation is coherent. UUIDs, base envelopes, validation
normalization, path parsing, cookies, browser HTTP behavior, CORS, cache
helpers, server config, and CSP helpers match their promoted contracts and have
focused tests.

Three bounded edges drift:

1. request-context extractor errors still emit plain text
2. page-list wire casing is wrong in OpenAPI and stale architecture prose
3. Rust HTTP-client infallible fallbacks can discard configured timeouts

One query-input policy remains materially ambiguous. No production behavior
changed in this assessment.

## Evidence Matrix

| Contract clause | Live implementation | Test or artifact evidence | Result | Disposition |
| --- | --- | --- | --- | --- |
| `010` identifier and generator seam | `underlay-core/src/id.rs`: `Uuid`, `IdGenerator`, `SystemIdGenerator` | `underlay-core/src/tests/core_tests.rs::uuid_round_trips_from_string` | matched | contract match |
| `010` single and bounded-list envelopes | `underlay-core/src/dto.rs`; `underlay-http/src/responses.rs` | `responses_tests.rs`; TS `client/envelopes.ts`; OpenAPI `SingleResponse`/`ListResponse` | matched | contract match |
| `010` canonical error envelope and `AppError` | `underlay-core/src/{dto,error}.rs`; `underlay-http/src/errors.rs`; TS `client/errors.ts` | core camel-case serialization test, `errors_tests.rs`, `client/errors.test.ts`, OpenAPI `ErrorBody`/`ErrorEnvelope` | matched | contract match |
| `010` validation collapse at HTTP boundary | `underlay-validation/src/axum_integration.rs` | `axum_integration_tests.rs` proves malformed JSON and field errors use `ErrorEnvelope` and omit empty maps | matched | contract match |
| `020` success/error response helpers | `underlay-http/src/{responses,errors}.rs` | `responses_tests.rs`, `errors_tests.rs` | matched except context-specific rejections | implementation repair `g10.012` |
| `020` sort/filter query vocabulary | Rust `underlay-query` plus `underlay-http/src/query/params.rs`; TS `client/query.ts` | Rust `query_tests.rs`; TS `client/query.test.ts` | matched for declared inputs; invalid/alias policy ambiguous | operator decision; no incidental repair |
| `020` pagination defaults and legacy nested envelope | `underlay-http/src/pagination.rs` | `pagination_tests.rs` proves page 1, limit 20, max 100, nested metadata | matched | contract match |
| `020` canonical flat page-list boundary inherited from `115` | Rust `underlay-http/src/page_list.rs`; TS `client/envelopes.ts`; OpenAPI YAML | Rust proves raw `has_more`; TS/OpenAPI drift guard currently declares `hasMore` | drifted | contract-artifact repair `g10.013` |
| `020` UUID path parsing | `underlay-http/src/path.rs` | `path_tests.rs`; implementation returns `400` plus `validation.invalid_id` through `error_response()` | matched | contract match |
| `020` request context and trusted proxies | `underlay-http/src/context/{parse,model,extractors}.rs` | `context_tests.rs` proves `x-request-id`, fail-closed forwarding headers, declared proxy modes, and authenticated identity seam | drifted only on rejection serialization | implementation repair `g10.012` |
| `020` auth-cookie composition | `underlay-http/src/cookies/**` | cookie config, builder, extractor, and header tests prove defaults, flags, typed setters, and paired set/clear | matched | contract match |
| `020` browser HTTP client | TS `client/http.ts`, `http-types.ts`, `errors.ts` | request, auth-header, refresh, retry-timeout, and metadata tests | matched | contract match |
| `020` Rust outbound HTTP client | `underlay-http-client/src/lib.rs` | tests prove normal constructors, timeouts, SSRF target classification, and external profile; fallback code explicitly loses defaults | drifted | implementation repair `g10.014` |
| `020` CORS boundary | `underlay-http/src/cors.rs` | `cors_tests.rs` proves wildcard/credential constraint, explicit origin paths, local-only mirrored credentials, methods, headers, and max age | matched | contract match |
| `020` in-process cache and validators | `underlay-http/src/caching.rs` | `caching_tests.rs` proves TTL, bound, invalidation, single-flight, stable weak ETags, and matchers | matched | contract match |
| `020` server config | `underlay-http/src/http_config.rs` | `http_config_tests.rs` proves bind/public-host split, localhost default, URLs, and fallible env parsing | matched | contract match |
| `020` CSP and security headers | TS `server/csp*.ts` | `server/csp/*.test.ts` proves default directives, nonce, report-only mode, and security-header composition | matched | contract match |

## Confirmed Repair Candidates

### `g10.012` — context rejection envelopes

- Evidence: `context/error.rs::IntoResponse` returns `(StatusCode, &str)`;
  `RequestContext` also exposes a tuple rejection type.
- Consequence: canonical clients cannot parse shared extractor failures through
  `UnderlayHttpError.envelope`.
- Boundary: rejection serialization only; no auth/session policy.
- Alternatives: keep plain text; special-case context errors in clients; emit
  the canonical envelope. The envelope repair is the only contract-preserving
  choice.
- Risk: low. Wire body changes for existing `401`/`400` context failures.
- Validation: focused extractor response-body tests plus `underlay-http` tests.
- Promotion: ready now.
- Confidence: high.
- Authority question: stable code naming. Existing shared vocabulary supports
  `auth.unauthorized`; the missing-context code must remain request-local.

### `g10.013` — page-list contract artifact sync

- Evidence: Rust `PageList<T>` serializes `has_more`; contract `115` declares
  that raw wire shape; OpenAPI and architecture `015` declare `hasMore`, while
  TS intentionally exposes normalized `hasMore`.
- Consequence: generated clients and human readers can treat the TS boundary as
  raw server wire authority.
- Boundary: OpenAPI, drift tests, and docs. No runtime serialization change.
- Alternatives: change Rust wire to camel case; change TS public shape to snake
  case; preserve the declared normalization seam. The last option matches
  contract `115` and current consumers.
- Risk: low for runtime, medium for OpenAPI generators.
- Validation: raw-wire and normalized-boundary assertions plus docs QA.
- Promotion: after `g10.012`.
- Confidence: high.
- Authority question: none. Contract `115` already settles raw versus normalized
  casing.

### `g10.014` — bounded HTTP-client fallback

- Evidence: `HttpClient::new()` and `with_user_agent()` fall back to
  `reqwest::Client::new()` when builder construction fails; the source warning
  states that custom timeouts are lost.
- Consequence: a public constructor can violate the no-unbounded-hangs
  invariant.
- Boundary: constructor fallback only. SSRF profiles and request APIs stay put.
- Alternatives: make all constructors fallible; panic; fall back through the
  known-valid bounded default builder. Preserve the API and bounded default.
- Risk: low, with one invalid-input edge to test.
- Validation: invalid user-agent and constructor tests in
  `underlay-http-client`.
- Promotion: after `g10.013`.
- Confidence: high.
- Authority question: whether invalid custom user-agent input should return an
  error or degrade to the bounded default. The fallible API already covers
  callers that need the error.

## Material Ambiguity

Invalid filter operators have no declared transport disposition:

- Rust's shared parser accepts symbolic aliases and the HTTP adapter falls back
  to equality for unknown operators.
- TypeScript's URL parser casts any word-like operator into the closed type.
- Contract `020` names the allowed vocabulary but does not say whether invalid
  input is rejected, ignored, or normalized.

Consequence: malformed query input can mean different things at each boundary.
Do not change this through a parser cleanup. Set one policy during the later
collection/query-profile assessment, then open a repair card if needed.

## Recommendation

Keep the architecture. Repair the three bounded edges in sequence. Do not
redesign the envelope, pagination, query, or client layers.

## Next Route

Execute `g10.012`. Keep `g10.013` and `g10.014` planned until the preceding
repair closes.

## Validation

- `effigy test --plan` — Vitest and Cargo workspace suites detected
- `effigy rust:test` — passed, including all cited foundation and transport
  unit and doc tests; infrastructure tests requiring external Postgres remained
  ignored by their declared posture
- `effigy test:unit` with the cited TS paths — passed; the repo selector runs
  the complete unit surface: 126 files, 781 tests
- `effigy health` — passed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `effigy validate` — passed: workspace shape 11/11, unit 781/781, component
  49/49, zero Svelte diagnostics
- `git diff --check` — passed

`effigy doctor` still reports the known attention-marker and god-file scan
backlog. The code graph was stale and refreshed cleanly before navigation;
those structural findings do not block this bounded assessment.
