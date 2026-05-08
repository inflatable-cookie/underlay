# Contract: Foundation Primitives and Envelopes

Status: active
Owner: repo maintainers
Depends on: `001-working-rules.md`

## Purpose

Define the shared primitive contract every higher Underlay system builds on:

- canonical identifier type
- success envelopes
- transport error envelope
- core application error shape
- validation boundary between internal errors and wire payloads

This contract exists to stop later auth, storage, jobs, content, and template
contracts from silently redefining the lowest shared layer.

## Sources of Truth

Primary:

- [`rust/crates/underlay-core/src/id.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-core/src/id.rs)
- [`rust/crates/underlay-core/src/dto.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-core/src/dto.rs)
- [`rust/crates/underlay-core/src/error.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-core/src/error.rs)
- [`rust/crates/underlay-http/src/responses.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/responses.rs)
- [`rust/crates/underlay-http/src/errors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/errors.rs)
- [`ts/src/client/types.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts)
- [`contracts/openapi/underlay.openapi.yaml`](/Users/tom/Dev/projects/underlay/contracts/openapi/underlay.openapi.yaml)

Supporting:

- [`docs/architecture/015-error-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/architecture/015-error-and-envelopes.md)
- [`rust/crates/underlay-validation/src/error.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/error.rs)
- [`rust/crates/underlay-validation/src/axum_integration.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/axum_integration.rs)

If these diverge, `underlay-core` plus the transport helpers and OpenAPI shape
win. Architecture docs and helper integrations must be repaired to match.

## Contract Goal

Underlay should expose one small, stable primitive model that every consumer
can depend on without learning crate-local or app-local dialects.

That primitive model must:

- keep identifiers boring and portable
- keep successful JSON payloads structurally consistent
- keep transport errors machine-readable and form-friendly
- let richer internal validation or error structures exist without leaking
  custom wire formats by accident

## Canonical Primitive Set

### Identifier

`Uuid` is the canonical shared identifier type.

Rules:

- Rust generation uses UUIDv7 through `Uuid::new_v7()`.
- Rust may accept any valid UUID string at parse boundaries unless a tighter
  caller contract says otherwise.
- TypeScript treats `Uuid` as an opaque string contract, not a branded runtime
  wrapper.
- Shared contracts, DTOs, and APIs must speak in terms of `Uuid`, not crate- or
  app-specific id wrappers, unless a system contract explicitly introduces one.

`IdGenerator` is the shared generation seam.

Rules:

- domain crates may inject alternate generators for tests or deterministic
  workflows
- production default remains `SystemIdGenerator`
- higher-level contracts may depend on deterministic allocation, but they must
  do so through the generator seam, not by bypassing the shared type

### Success envelopes

The shared success envelope family is:

- `SingleResponse<T>` -> `{ "data": T }`
- `ListResponse<T>` -> `{ "data": T[] }`

Rules:

- success payloads use `data` as the only top-level business field
- list responses are structural list envelopes, not pagination contracts
- envelope shape must stay aligned across Rust serialization, TS types, and
  OpenAPI
- `204 No Content` is the only standard no-body success path

Non-goals:

- cursor metadata
- page metadata
- links objects
- ad hoc `success`, `meta`, or `errors` siblings

Those belong in later system-specific contracts if they are truly needed.

### Transport error envelope

The canonical transport error payload is:

```json
{
  "error": {
    "code": "stable.error_code",
    "message": "Human-readable summary",
    "fieldErrors": {
      "fieldName": "Field-specific message"
    }
  }
}
```

Rules:

- every transport error body uses the top-level `error` envelope
- `code` is stable, machine-readable, and part of the public contract
- `message` is human-readable and may be shown to operators or end users
- `fieldErrors` is optional and only present for field-scoped feedback
- transport serialization uses camelCase on the wire, even where Rust fields
  are stored as snake_case

`AppError` is the canonical minimal shared source for this envelope.

Rules:

- `AppError` stays intentionally small: `code`, `message`, optional
  `field_errors`
- domain crates may add helpers and domain-local constructors around it
- domain crates must not replace the shared transport envelope shape
- `ApiError` may add HTTP status and logging context, but it still serializes
  the same envelope

### Validation boundary

`underlay-validation` owns richer internal validation structures such as:

- `ValidationError`
- `FieldError`
- nested merge and namespacing helpers

That richer structure is not the shared wire contract by default.

Rules:

- the shared wire-level validation contract collapses to the canonical transport
  error envelope
- field-level transport feedback is `Record<string, string>` /
  `HashMap<String, String>`
- internal validation codes or nested objects may exist inside validation
  crates, but they require an explicit promotion decision before they become
  transport authority
- higher-level systems may preserve richer validation structure internally, but
  transport helpers must normalize before crossing the HTTP boundary

## Invariants

- Rust, TS, and OpenAPI must agree on the envelope field names and top-level
  shape.
- A caller that understands only `data`, `error.code`, `error.message`, and
  optional `error.fieldErrors` must still function across shared Underlay
  systems.
- Error codes are part of the compatibility surface. Changing them is a
  contract change.
- `ListResponse<T>` is not permission to invent pagination semantics.
- Shared primitives stop at transport-safe structure. Domain meaning lives in
  later contracts.

## Extension Points

Allowed:

- domain-specific stable error-code namespaces
- domain-specific error constructors and mappers
- deterministic `IdGenerator` implementations for tests or replay flows
- additional higher-level response metadata only when a later contract defines
  it explicitly

Not allowed:

- replacing `Uuid` with ad hoc shared id primitives
- adding top-level success envelope fields by convention
- returning non-enveloped JSON errors from shared HTTP helpers
- treating internal `ValidationError` structure as transport authority without a
  contract update

## Known Drift And Assessment Hooks

Current drift worth assessing next:

- [`rust/crates/underlay-validation/src/axum_integration.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-validation/src/axum_integration.rs)
  currently emits `field_errors` and leaks the richer internal validation map
  shape instead of fully normalizing to the canonical transport envelope.
- `ts/src/client/types.ts` is currently the live TS primitive authority, but it
  is mixed with domain types and should later be reassessed for clearer
  ownership under the transport contract.

These are implementation-assessment items, not reasons to widen this contract.

## Assessment Questions

Use this contract to judge later implementation work:

- does the system keep UUID ownership and generation on the shared seam
- does every HTTP success/error helper preserve the canonical envelope shape
- does any validation helper leak richer internal structures onto the wire
- do TS types, OpenAPI, and Rust serialization still agree
- is a proposed primitive change truly shared, or is it domain-specific and
  misplaced here

## Next Task

Execute `g04.004`: write `020-http-transport-and-server-boundary.md`.
