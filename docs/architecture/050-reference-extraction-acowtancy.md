# Reference Extraction Map (Acowtancy)

Acowtancy is the current reference implementation. Underlay should extract *stable*, app-agnostic pieces from it over time.

This document is a living map of what to extract and where it should land.

## 1. Farmyard → Underlay (Rust)

Target: reusable API boundary primitives and infrastructure patterns.

- **Error + envelope conventions**
  - Underlay already contains `AppError`, `ErrorEnvelope`, `SingleResponse`, `ListResponse`.
  - Extract next: a small `underlay-api` crate that maps `AppError` → HTTP responses and keeps status/error-code semantics consistent.

- **ID conventions**
  - Underlay already wraps UUIDv7 (`Uuid`) and exposes a default generator.

- **Versioning + headers**
  - Extract: a version header helper/extractor ("API version" gating is cross-project).

- **Auth boundary types**
  - Extract: generic `AuthenticatedUser` concept + role/capability abstraction (apps supply concrete roles).

- **Operational/logging patterns**
  - Extract: tracing setup helpers and request correlation patterns.

## 2. Cattle Grid → Underlay (TypeScript)

Target: shared patterns for a typed API client.

- **HTTP client core**
  - Underlay now provides `createHttpClient()` under `@decodelabs/underlay/client/http`
    and `UnderlayHttpError` under `@decodelabs/underlay/client/errors`.

- **Command module pattern**
  - Next extraction: a lightweight convention for defining command modules (grouped by domain, returning typed envelopes).

- **OpenAPI type generation**
  - Keep generation in app repos initially; Underlay should standardise folder naming and the integration surface.

## 3. Froyo → Underlay (Svelte UI kit)

Target: app-agnostic UI primitives and patterns.

Candidates (start with the most generic):

- `shared/Button.svelte`, `shared/Card.svelte`
- `shared/Form.svelte`, `shared/FormActions.svelte`, `shared/Field.svelte`
- `shared/ListGrid.svelte`, `shared/ListCard.svelte`

Nightfire is higher risk because it includes app-specific block schemas. Extract it once the block registry interfaces are stable.

## 4. Dairy → Underlay (Admin UI patterns)

Target: generic SvelteKit patterns (not routes or domain pages).

- CRUD form shells (load + action + error handling patterns)
- Filter bar / list page layout primitives
- Navigation helpers that are role/capability-driven

Avoid extracting any admin domain routes directly; Underlay should only provide the reusable shells.

## 5. Farmyard media → Underlay

Target: shared media semantics and sync framework, not Acowtancy-specific block
schemas.

- **Media library contract**
  - Extract: stable `media`, `media_version`, `media_usage`, and migration
    attachment-binding semantics.
  - Landed reference contract:
    `docs/contracts/050-media-library-and-usage.md`

- **Usage-edge model**
  - Extract: generalized usage edges with owner type/id, owner field,
    locator-kind, locator-key, usage role, and provenance kind.
  - Keep app-specific owner registrations in consumer repos.

- **Usage sync engine**
  - Extract: shared set-diff reconciliation for save/import flows and audit
    scaffolding for forward/reverse consistency checks.

- **Structured-content traversal**
  - Extract: generic walker and extractor interfaces.
  - Keep Nightfire block definitions and block-specific media extraction in
    consumer repos until the block contract stabilizes across sites.
