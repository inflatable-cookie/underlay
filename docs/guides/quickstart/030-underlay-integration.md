# 030 - Underlay Integration

This document explains how to integrate the Underlay foundation library into your new project. Underlay provides cross-cutting primitives that your app composes rather than reimplementing.

## What is Underlay?

Underlay is a reusable foundation that provides:

| Crate | Purpose |
|-------|---------|
| `underlay-core` | IDs, error types, response envelopes |
| `underlay-http` | Axum helpers, CORS, standard error responses |
| `underlay-auth` | Auth boundary, provider abstraction, Axum extractors |
| `underlay-db` | SQLx helpers (optional) |
| `underlay-observability` | Tracing bootstrap, request ID |
| `underlay-metrics` | Prometheus registry helpers (optional) |
| `underlay-events` | Outbox/event patterns (optional) |

## Integration Options

### Option A: Sibling Directory (Recommended for Active Development)

Use this when you're actively developing both Underlay and your project.

```bash
cd my-project
ln -s ../libraries/underlay libs/underlay
```

### Option B: Git Submodule (Recommended for Stable Dependencies)

```bash
cd my-project
git submodule add https://github.com/your-org/underlay.git libs/underlay
git submodule update --init --recursive
```

### Option C: Cargo / npm Dependencies (For Published Versions)

When Underlay is published:

```toml
# apps/nursery/Cargo.toml
[workspace.dependencies]
underlay-core = "0.1"
underlay-http = "0.1"
underlay-auth = "0.1"
```

## Path Configuration (Rust)

If Underlay lives at `libs/underlay/` inside your monorepo, set workspace deps like:

```toml
# apps/nursery/Cargo.toml

[workspace]
members = [
  "crates/core",
  "crates/api",
  "crates/auth",
  "crates/db",
  "crates/infra",
]

[workspace.dependencies]
underlay-core = { path = "../../../libs/underlay/rust/crates/underlay-core" }
underlay-http = { path = "../../../libs/underlay/rust/crates/underlay-http" }
underlay-auth = { path = "../../../libs/underlay/rust/crates/underlay-auth" }
underlay-observability = { path = "../../../libs/underlay/rust/crates/underlay-observability" }
underlay-metrics = { path = "../../../libs/underlay/rust/crates/underlay-metrics" }
```

## Path Configuration (TypeScript)

Underlay’s TS package is `@decodelabs/underlay`. With pnpm, you can depend on it via a local file reference:

```json
{
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay"
  }
}
```

## Verifying Integration

### 1) Verify Rust Integration

```rust
use underlay_core::Uuid;

#[derive(Debug)]
pub struct MyId(pub Uuid);

impl MyId {
    pub fn new() -> Self {
        Self(Uuid::new_v7())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generation_works() {
        let id = MyId::new();
        // Underlay Uuid is a UUIDv7 wrapper.
        assert_ne!(id.0.to_string(), "");
    }
}
```

### 2) Verify TypeScript Integration

Underlay’s error envelope shape is:

```ts
import type { ErrorEnvelope } from "@decodelabs/underlay";

const example: ErrorEnvelope = {
  error: {
    code: "test.ok",
    message: "Test successful",
  },
};
```

## Common Integration Issues

### Issue: "cannot find underlay-core"

Cause: wrong path dependencies in Cargo workspace.

### Issue: "Module not found: @decodelabs/underlay"

Cause: missing pnpm install or incorrect `file:` path.

## Next Step

Proceed to [040-rust-backend](./040-rust-backend.md).
