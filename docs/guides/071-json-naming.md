# 071 - JSON Naming Policy

This guide defines the canonical JSON naming convention for Underlay and Underlay-based apps.

## Canonical Convention

Use `snake_case` for all exported JSON fields.

This applies to:

1. API request payloads
2. API response payloads
3. Background job payloads
4. Error context persisted in logs
5. Seed, fixture, export, and import JSON files

## Why `snake_case`

1. It aligns with Rust field naming and SQL conventions.
2. It reduces per-field serde rename boilerplate.
3. It avoids mixed conventions between DB, API, and jobs.
4. It minimizes migration ambiguity across app families.

## Rust DTO Rules

Use snake_case Rust struct fields and default serde behavior.

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProjectSummaryDto {
    pub project_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

Avoid:

1. `#[serde(rename_all = "camelCase")]` on internal DTOs
2. `#[serde(rename = "...")]` with camelCase names unless handling external contracts

## Exception Policy (External Contracts Only)

`camelCase` is only allowed when integrating with third-party contracts you do not control.

When an exception is required:

1. Keep camelCase only at the boundary DTO.
2. Add a short note explaining the external system.
3. Map to internal snake_case types immediately after deserialization.
4. Avoid leaking external naming conventions through internal APIs.

## TypeScript Boundary Rule

TypeScript app internals may use camelCase if needed for local ergonomics, but API boundary types and wire payloads must be `snake_case`.

If conversion is needed, centralize it in one adapter layer rather than per-component transforms.

## Migration Guidance

1. Standardize shared crates first (`underlay-*` runtime contracts).
2. Migrate DTOs and job payloads by domain.
3. Use temporary dual-read compatibility only when required.
4. Time-box and remove compatibility adapters after cutover.

## Guardrail Script

Use the Underlay guardrail script to detect likely internal camelCase serde drift:

```bash
./scripts/check-json-naming.sh
```

To scan a specific directory:

```bash
./scripts/check-json-naming.sh rust/crates
```

To allow specific path patterns from an exceptions file:

```bash
./scripts/check-json-naming.sh rust/crates ./scripts/json-naming-allowlist.txt
```
