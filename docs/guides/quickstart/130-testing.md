# 130 - Testing

This document covers testing strategies for all layers of the application.

## Testing Strategy

### Rust Tests

```rust
// apps/nursery/crates/core/src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        // Unit tests
    }
}
```

### TypeScript Tests

```typescript
// libs/stem/src/http.test.ts
import { describe, it, expect } from 'vitest';
```

### Frontend Tests

```typescript
// apps/bloom/src/routes/+page.test.ts
import { describe, it, expect } from 'vitest';
```

## Running Tests

```bash
# Rust
cd apps/nursery && cargo test

# TypeScript
cd libs/stem && pnpm test

# Frontend
cd apps/bloom && pnpm test
```

See code examples in `/code/130-testing/`

## Next Steps

- [140-local-development.md](./140-local-development.md)
