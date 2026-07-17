# Error Codes & Response Envelopes

Underlay standardises the *shape* of responses and errors across Rust APIs, TypeScript clients, and SvelteKit apps.

The goal is not to force domain structure, but to provide stable primitives so applications can evolve without re-inventing cross-cutting concerns.

## 1. Response Envelopes

Underlay uses three success envelopes:

- `SingleResponse<T>`
  - JSON: `{ "data": <T> }`
- `ListResponse<T>`
  - JSON: `{ "data": [<T>, ...] }`
- `PagedListResponse<T>`
  - JSON: `{ "data": [<T>, ...], "total": <n>, "hasMore": <bool> }`

These are defined in:

- Rust: `SingleResponse`/`ListResponse` in `rust/crates/underlay-core/src/dto.rs`;
  the paged wire shape is produced by `Paginated<T>` in `underlay-http`
  (`pagination.rs`)
- TypeScript: `ts/src/client/envelopes.ts` (re-exported via `client/types.ts`)
- OpenAPI: `contracts/openapi/underlay.openapi.yaml`

## 2. Error Envelope

All errors are returned as:

- `ErrorEnvelope`
  - JSON: `{ "error": { "code": string, "message": string, "fieldErrors"?: Record<string, string> } }`

Notes:

- `fieldErrors` is optional and is only included when relevant.
- In Rust, field names use `camelCase` serialization so `field_errors` becomes `fieldErrors`.

## 3. Error Codes

Error codes are stable strings like:

- `auth.forbidden`
- `auth.unauthorized`
- `resource.not_found`
- `resource.deleted`
- `validation.invalid`

Rules:

- Treat codes as part of the API contract: once shipped, keep them stable.
- Prefer `namespace.reason` naming.
- Keep codes app-agnostic; domain-specific codes are allowed but should remain consistent (e.g. `content.invalid_payload`).

## 4. Field Errors

Use `fieldErrors` for form-style validation feedback.

Conventions:

- Keys should match the *client field name*, not internal DB column names.
- Prefer `camelCase` field keys (matches typical TS/Svelte naming).

## 5. Client Behaviour (TypeScript)

Client code should:

- Parse JSON error bodies into `ErrorEnvelope` when possible.
- Throw a typed error that includes HTTP status and the parsed envelope (when present).

Underlay’s baseline is `UnderlayHttpError` in `ts/src/client/errors.ts`.

## 6. Server Behaviour (Rust)

Server code should:

- Return `ErrorEnvelope` for all error responses.
- Ensure request-id/correlation-id is consistently available to logs and responses (defined in the observability phase).

## 7. Compatibility

Any changes to envelope JSON shapes must be reflected in all three:

- Rust DTOs (`rust/crates/underlay-core/src/dto.rs`)
- TS types (`ts/src/client/envelopes.ts`, re-exported via `client/types.ts`)
- OpenAPI contract (`contracts/openapi/underlay.openapi.yaml`)
