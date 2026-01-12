# 011 – Advanced Features: When Needed

**Status**: Backlog - Wait for Concrete Use Cases  
**Priority**: Low  
**Estimated Duration**: Variable (40-60 hours total)  
**Target**: Phase 3 - Only implement when projects need them

---

## Overview

This roadmap covers advanced features that would be nice to have but should **only be implemented when there's a concrete use case**. These features are complex, require significant effort, and could over-engineer Underlay if added prematurely.

**Philosophy**: **YAGNI (You Aren't Gonna Need It)** - Don't build these until projects actually need them.

**Decision Criteria** for implementing these:
- At least 2 projects need the feature
- Manual implementation is painful (>100 lines of boilerplate)
- The feature is generic enough to be reusable
- We have capacity to maintain it long-term

Tick items with `[x]` as they are completed.

---

## Server-Side Features

### 1. Rate Limiting (`underlay-rate-limit`)

**Status**: Deferred until needed  
**When to build**: When we get DoS attacks or abuse  
**Effort**: 4-6 hours  
**Risk**: Medium - requires Redis or similar storage

**Problem**: Need to protect endpoints from abuse

**Solution**: Pluggable rate limiting middleware

#### Potential Design

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

**Considerations**:
- Storage backend (Redis, in-memory, database?)
- Distributed systems (multiple servers)
- Different strategies (token bucket, sliding window)
- Rate limit headers (X-RateLimit-*)

**Decision**: Wait until we have a concrete abuse problem

---

### 2. Caching Layer (`underlay-cache`)

**Status**: Deferred until needed  
**When to build**: When we have performance issues  
**Effort**: 6-8 hours  
**Risk**: Medium - cache invalidation is hard

**Problem**: Database queries are slow, need caching

**Solution**: Declarative caching with automatic invalidation

#### Potential Design

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

**Considerations**:
- Cache backend (Redis, in-memory)
- Invalidation strategies
- Cache warming
- Monitoring/metrics

**Decision**: Wait until we have concrete performance issues. Profile first, cache second.

---

### 3. Background Job Dashboard (`underlay-jobs` extension)

**Status**: Deferred until needed  
**When to build**: When we need to debug failed jobs frequently  
**Effort**: 8-10 hours  
**Risk**: Low - admin-only feature

**Problem**: Can't see what jobs are running or why they failed

**Solution**: Web UI for monitoring background jobs

#### Potential Features

- List queued/running/failed jobs
- View job payloads and errors
- Retry failed jobs
- Cancel running jobs
- Job statistics and metrics
- Search/filter jobs

**Considerations**:
- Authentication (admin-only)
- Real-time updates (WebSocket?)
- Performance (large job queues)
- UI framework (embed in existing admin?)

**Decision**: Wait until we have enough background jobs that debugging becomes painful

---

## Client-Side Features

### 4. GraphQL Support (`underlay-graphql`)

**Status**: Deferred indefinitely  
**When to build**: When REST becomes too limiting  
**Effort**: 20-30 hours  
**Risk**: High - major paradigm shift

**Problem**: REST endpoints are too inflexible, need GraphQL

**Solution**: Code-first GraphQL schema generation

**Considerations**:
- Server-side: Schema generation, resolvers
- Client-side: Query builder, cache management
- Subscriptions (real-time)
- Authentication/authorization
- N+1 query problem

**Decision**: Probably overkill. REST + good API design is usually sufficient. Only consider if multiple projects need complex, nested queries.

---

### 5. Real-time / WebSocket Layer (`underlay-realtime`)

**Status**: Deferred until needed  
**When to build**: When we need live updates or collaboration  
**Effort**: 15-20 hours  
**Risk**: High - infrastructure complexity

**Problem**: Need real-time updates (chat, notifications, presence)

**Solution**: WebSocket infrastructure with channels and presence

#### Potential Features

- Connection management
- Channel subscriptions
- Broadcast messages
- Presence tracking (who's online)
- Reconnection handling
- Message persistence (optional)

**Considerations**:
- Server infrastructure (WebSocket support)
- Scaling (multiple servers, sticky sessions)
- Authentication
- Fallback for old browsers

**Decision**: Wait until we have a concrete real-time use case. Don't build infrastructure for hypothetical features.

---

### 6. CLI Scaffolding (`underlay-cli`)

**Status**: Deferred until patterns stabilize  
**When to build**: When patterns are stable and repetitive  
**Effort**: 10-15 hours  
**Risk**: Medium - maintenance burden

**Problem**: Creating CRUD endpoints is repetitive

**Solution**: CLI to generate boilerplate code

#### Potential Commands

```bash
# Generate CRUD endpoint
underlay generate crud users

# Generate database migration
underlay generate migration add_users_table

# Generate Svelte component
underlay generate component DataCard

# Generate API client command
underlay generate command users
```

**Considerations**:
- Templates need maintenance
- Different projects have different patterns
- Over-generates code (too much boilerplate)
- May encourage bad patterns

**Decision**: Wait until patterns are very stable. Hand-coding ensures developers understand the code.

---

### 7. Advanced Data Table Features

**Status**: Extend DataTable from roadmap 010 if needed  
**When to build**: When basic DataTable is insufficient  
**Effort**: 5-10 hours  
**Risk**: Low - extends existing component

**Potential Features**:
- Column reordering (drag-drop)
- Column resizing
- Column visibility toggle
- Saved views (filters + sorts)
- Export to CSV/Excel
- Inline editing
- Expandable rows
- Tree/hierarchical data

**Decision**: Start with basic DataTable (roadmap 010). Add features incrementally as projects need them.

---

### 8. Advanced Form Features

**Status**: Extend form patterns from roadmap 009 if needed  
**When to build**: When basic forms are insufficient  
**Effort**: 4-8 hours  
**Risk**: Low - extends existing patterns

**Potential Features**:
- Multi-step forms (wizard)
- Form state persistence (save draft)
- Field-level validation (async)
- Conditional fields (show/hide based on values)
- Auto-save
- Undo/redo

**Decision**: Start with basic form state (roadmap 009). Add features as projects need them.

---

## What NOT to Build

### ❌ Complete Admin Generator

**Why not**: Too opinionated, locks projects into specific patterns

Instead: Provide building blocks (DataTable, forms, etc.) that projects compose

### ❌ ORM or Query Builder

**Why not**: SQLx is excellent, no need to reinvent

Instead: Provide helpers that work WITH SQLx (pagination, soft delete)

### ❌ Full Auth UI

**Why not**: Every project has different auth requirements

Instead: Provide components (LoginForm, RegisterForm) that projects customize

### ❌ State Management Library

**Why not**: Svelte stores are good, don't need Redux/Zustand

Instead: Provide patterns (optimistic updates) that work with Svelte stores

### ❌ Component Library Lock-in

**Why not**: Projects should be able to use any UI library

Instead: Provide unstyled/lightly-styled components that are easily customizable

---

## Decision Framework

Before implementing any feature from this roadmap, ask:

1. **Is it needed?**
   - Do we have a concrete use case?
   - Are multiple projects asking for it?
   - Is manual implementation painful?

2. **Is it generic?**
   - Will it work across different projects?
   - Or is it too specific to one use case?

3. **Can we maintain it?**
   - Do we have capacity for long-term maintenance?
   - Will it create tech debt?

4. **Is there a better alternative?**
   - Can we use an existing library?
   - Can we solve it with better patterns?

5. **What's the cost of NOT building it?**
   - How much boilerplate does it save?
   - How much time does it save?

If the answer to all these is YES, then consider building it. Otherwise, defer.

---

## Monitoring for Need

Watch for these signals that a feature is needed:

### Rate Limiting
- ☐ DoS attacks or abuse reported
- ☐ API endpoints being hammered
- ☐ Need to protect expensive operations

### Caching
- ☐ Database queries taking >100ms consistently
- ☐ Same queries repeated frequently
- ☐ Profiling shows caching would help significantly

### Real-time
- ☐ Users requesting live updates
- ☐ Polling becoming excessive
- ☐ Collaboration features needed

### CLI Scaffolding
- ☐ Creating 10+ similar endpoints feels painful
- ☐ Patterns are very stable (no changes in 3+ months)
- ☐ Onboarding new developers takes too long

---

## Success Criteria for Advanced Features

Only implement if:
- ✅ Clear use case from 2+ projects
- ✅ Manual implementation is >100 lines
- ✅ Feature is generic and reusable
- ✅ We have capacity to maintain it
- ✅ No good existing library available
- ✅ Cost/benefit analysis shows positive ROI

If criteria not met: **Don't build it**

---

## Related Roadmaps

- 009 - Quick Wins (build these first)
- 010 - Medium Value (build these second)
- This roadmap (build only when needed)

---

**Created**: 2026-01-12  
**Last Updated**: 2026-01-12  
**Related Report**: `docs/reports/2026-01-12-underlay-enhancement-suggestions.md`  
**Philosophy**: YAGNI - You Aren't Gonna Need It (until you do)
