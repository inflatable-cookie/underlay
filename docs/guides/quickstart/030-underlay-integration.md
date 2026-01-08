# 030 - Underlay Integration

This document explains how to integrate the Underlay foundation library into your new project. Underlay provides cross-cutting primitives that your app composes rather than reimplementing.

## What is Underlay?

Underlay is a **reusable foundation** that provides:

| Crate | Purpose |
|-------|---------|
| `underlay-core` | IDs, error types, DTO envelopes |
| `underlay-http` | Axum response helpers, CORS, error responses |
| `underlay-auth` | Auth boundary types, provider abstraction, Axum extractors |
| `underlay-db` | SQLx pool setup, migration runners |
| `underlay-observability` | Tracing bootstrap, request identity |
| `underlay-metrics` | Prometheus registry, `/metrics` handler |
| `underlay-soft-delete` | Soft-delete conventions |
| `underlay-events` | Outbox/event patterns |
| `underlay-jobs` | Background job skeleton |

## Integration Options

### Option A: Sibling Directory (Recommended for Active Development)

Use this when you're **actively developing both Underlay and your project**.

```bash
# Assuming Underlay is at the same level as your project
cd my-project
ln -s ../libraries/underlay libs/underlay
```

**Pros:**
- Easy to make changes to Underlay while developing your app
- No git submodule maintenance
- Instant reflection of Underlay changes

**Cons:**
- Requires Underlay to exist at the sibling path
- May have path references that need adjustment

### Option B: Git Submodule (Recommended for Stable Dependencies)

Use this when you want **stable, versioned Underlay dependencies**.

```bash
cd my-project
git submodule add https://github.com/your-org/underlay.git libs/underlay
git submodule update --init --recursive
```

**Pros:**
- Version control of Underlay dependency
- Deterministic builds
- Can use different Underlay versions for different projects

**Cons:**
- Requires submodule update to get Underlay changes
- More complex CI/CD

### Option C: Cargo/git Dependencies (For Published Versions)

Use this when Underlay is **published to a registry** (crates.io, private registry).

```toml
# In apps/nursery/Cargo.toml
[dependencies]
underlay-core = "0.1"
underlay-http = "0.1"
underlay-auth = "0.1"
# ...
```

**Pros:**
- Standard Rust dependency management
- Easy version upgrades

**Cons:**
- Requires Underlay to be published
- Less flexible for active development

## Path Configuration

### For Sibling Directory (Option A)

Create path overrides in your Rust workspace:

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
# Underlay (local dev via relative paths)
underlay-core = { path = "../../../libs/underlay/rust/crates/underlay-core" }
underlay-http = { path = "../../../libs/underlay/rust/crates/underlay-http" }
underlay-auth = { path = "../../../libs/underlay/rust/crates/underlay-auth" }
underlay-db = { path = "../../../libs/underlay/rust/crates/underlay-db" }
underlay-observability = { path = "../../../libs/underlay/rust/crates/underlay-observability" }
underlay-metrics = { path = "../../../libs/underlay/rust/crates/underlay-metrics" }
underlay-soft-delete = { path = "../../../libs/underlay/rust/crates/underlay-soft-delete" }
underlay-devtools = { path = "../../../libs/underlay/rust/crates/underlay-devtools" }
```

For TypeScript projects, use pnpm workspace protocol:

```json
{
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay"
  }
}
```

### For Git Submodule (Option B)

Update path references to point to the submodule:

```toml
# apps/nursery/Cargo.toml
underlay-core = { path = "../../libs/underlay/rust/crates/underlay-core" }
```

Ensure `.gitmodules` is properly configured:

```ini
[submodule "libs/underlay"]
    path = libs/underlay
    url = https://github.com/your-org/underlay.git
    branch = main
```

## Verifying Integration

### 1. Verify Rust Integration

Create a test file in `apps/nursery/crates/core/src/lib.rs`:

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
        assert!(!id.0.as_uuid().is_nil());
    }
}
```

Run tests:

```bash
cd apps/nursery
cargo test -p myapp-core
```

### 2. Verify TypeScript Integration

Create a test file in `libs/stem/src/index.ts`:

```typescript
import type { ErrorEnvelope, SingleResponse } from '@decodelabs/underlay';

// Verify Underlay types are accessible
const test: ErrorEnvelope = {
  code: 'test.ok',
  message: 'Test successful',
};

export type { ErrorEnvelope, SingleResponse };
export { test };
```

Run type check:

```bash
cd libs/stem
pnpm check
```

### 3. Verify Import Paths

Ensure you can import from Underlay in all contexts:

```rust
// Rust - apps/nursery/crates/api/src/main.rs
use underlay_core::Uuid;
use underlay_http::{SingleResponse, ApiError};
use underlay_auth::{AuthProvider, Authenticated};
```

```typescript
// TypeScript - libs/stem/src/http.ts
import { HttpClient, ApiError } from '@decodelabs/underlay';
```

```svelte
<!-- Svelte - apps/bloom/src/routes/+page.svelte -->
<script lang="ts">
  import { Button } from '@decodelabs/underlay';
</script>
```

## Common Integration Issues

### Issue: "cannot find underlay-core"

**Cause:** Path references are incorrect.

**Solution:**
```bash
# Verify the path exists
ls -la libs/underlay/rust/crates/underlay-core/Cargo.toml

# Update paths in Cargo.toml if needed
```

### Issue: "Circular dependency detected"

**Cause:** Your app crate depends on Underlay which depends on your app.

**Solution:** Ensure Underlay only contains app-agnostic code. Move app-specific code to your app crates.

### Issue: "Module not found: Can't resolve '@decodelabs/underlay'"

**Cause:** TypeScript can't find the Underlay package.

**Solution:**
```bash
# Ensure pnpm install has been run
cd apps/bloom
pnpm install

# Verify the symlink exists
ls -la node_modules/@decodelabs/underlay
```

## Underlay Update Workflow

### For Sibling Directory (Option A)

```bash
# Pull latest Underlay changes
cd libs/underlay
git pull origin main

# Your project sees changes immediately
cd ../../my-project
cargo test
```

### For Git Submodule (Option B)

```bash
# Update submodule to latest commit
cd libs/underlay
git fetch
git pull origin main
cd ../..
git add libs/underlay
git commit -m "chore: update underlay to latest"
```

## Recommended Underlay Crates

Start with these core crates:

| Crate | Always Include? | Description |
|-------|----------------|-------------|
| `underlay-core` | ✅ Yes | IDs, error types, envelopes |
| `underlay-http` | ✅ Yes | Axum helpers, CORS |
| `underlay-auth` | ✅ Yes | Auth boundary |
| `underlay-db` | ✅ Yes | Database utilities |
| `underlay-observability` | ✅ Yes | Tracing |
| `underlay-metrics` | ❌ Optional | Prometheus metrics |
| `underlay-soft-delete` | ❌ Optional | Soft-delete patterns |
| `underlay-events` | ❌ Optional | Event sourcing |
| `underlay-jobs` | ❌ Optional | Background jobs |

## Next Step

With Underlay integrated, proceed to [040-rust-backend](./040-rust-backend.md) to set up the Rust API backend.
