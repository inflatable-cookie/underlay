# Backlog: Caching Layer

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 6-8 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Database queries are slow and need caching to improve performance. Currently all queries go directly to PostgreSQL without any caching layer.

---

## Proposed Solution

Declarative caching with automatic invalidation.

### Potential Design

```rust
use underlay_cache::{Cache, Cached};

// Function-level caching
#[cached(ttl = "5m", key = "user:{id}")]
async fn get_user(id: Uuid) -> Result<User> {
    // Expensive DB query
    // Automatically cached for 5 minutes
}

// Manual cache management
let cache = Cache::new("users");
cache.set("user:123", user, Duration::minutes(5)).await?;
let user: User = cache.get("user:123").await?;
cache.invalidate("user:123").await?;
```

### Features

- TTL-based expiration
- Key pattern invalidation
- Cache warming strategies
- Monitoring/metrics integration
- Multiple storage backends (in-memory, Redis)

---

## Dependencies

- Storage backend decision
- Serialization strategy (serde)
- Metrics/observability integration

---

## When to Build

- Database queries taking >100ms consistently
- Same queries repeated frequently
- Profiling shows caching would help significantly
- Scale requires it

---

## Decision Framework

**Profile first, cache second.** Don't add caching complexity until there's proven performance need.

---

## Success Criteria

- [ ] Declarative caching via attribute macro
- [ ] Manual cache API for complex cases
- [ ] TTL and pattern-based invalidation
- [ ] In-memory and Redis backends
- [ ] Metrics for cache hit/miss rates
- [ ] Documentation with examples

---

## Risks & Considerations

- Cache invalidation is notoriously hard
- Stale data can cause bugs
- Memory pressure from large caches
- Complexity of distributed caching
- May encourage over-caching

---

**Created**: 2026-01-12
