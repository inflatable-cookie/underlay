# Rust API Foundation

Underlay’s Rust layer is intended to support a modular API codebase (multiple domain crates, one API surface).

## Core primitives (`underlay-core`)

- UUIDv7 IDs (`Uuid`) for predictable, time-sortable identifiers.
- A minimal `AppError` with a stable `code` and human-readable `message`.
- A stable error envelope (`ErrorEnvelope`) and response envelopes (`SingleResponse`, `ListResponse`).

## Error Codes

- Error codes are strings like `auth.forbidden`, `resource.not_found`, `content.invalid_payload`.
- Treat codes as part of the API contract; once shipped, keep them stable.

## Intended API conventions

- Versioning via a request header (e.g. `X-<Product>-Api-Version`) to support additive evolution.
- Error responses are always `ErrorEnvelope`.
- Success responses are `SingleResponse<T>` or `ListResponse<T>`.

Underlay does not yet ship an axum integration crate; add `underlay-api` once we extract stable patterns from a reference implementation.
