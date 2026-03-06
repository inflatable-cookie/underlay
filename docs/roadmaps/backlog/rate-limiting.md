# Backlog: Rate Limiting

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 4-6 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Need to protect endpoints from abuse, DoS attacks, and excessive API usage.

---

## Proposed Solution

Pluggable rate limiting middleware for Axum with declarative attributes.

### Potential Design

```rust
use underlay_rate_limit::{RateLimiter, RateLimit};

// Per-user rate limit
#[rate_limit(requests = 100, per = "1m", key = "user_id")]
async fn expensive_operation() -> Result<()> {
    // Protected
}

// IP-based rate limit
#[rate_limit(requests = 10, per = "1m", key = "ip")]
async fn public_endpoint() -> Result<()> {
    // Protected
}

// Global rate limit
#[rate_limit(requests = 1000, per = "1m")]
async fn global_endpoint() -> Result<()> {
    // Protected
}
```

### Features

- Multiple rate limit strategies (token bucket, sliding window)
- Per-user, per-IP, and global limits
- Rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset)
- Configurable storage backends (in-memory, Redis)
- Graceful handling of distributed systems

---

## Dependencies

- Storage backend decision (Redis for distributed, in-memory for single-server)
- Axum middleware integration patterns

---

## When to Build

- DoS attacks or abuse reported
- API endpoints being hammered
- Need to protect expensive operations
- Rate limiting required for API tier/billing

---

## Success Criteria

- [ ] Rate limiting middleware works with Axum
- [ ] Supports per-user, per-IP, and global limits
- [ ] Returns proper 429 responses with headers
- [ ] Works in distributed environments (multiple servers)
- [ ] Configurable storage backends
- [ ] Documentation with examples

---

## Risks & Considerations

- Storage backend choice affects complexity
- Distributed systems need shared storage (Redis)
- Different strategies have different trade-offs
- May need bypass for internal/admin requests

---

**Created**: 2026-01-12
