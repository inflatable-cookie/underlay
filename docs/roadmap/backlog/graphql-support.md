# Backlog: GraphQL Support

**Status**: Backlog (Deferred Indefinitely)  
**Priority**: Very Low  
**Estimated Effort**: 20-30 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

REST endpoints are too inflexible for complex, nested data queries. Clients need to make multiple requests to assemble data.

---

## Proposed Solution

Code-first GraphQL schema generation with async-graphql.

### Potential Design

```rust
use underlay_graphql::{Object, Query};

#[derive(Object)]
struct User {
    id: Uuid,
    name: String,
    #[graphql(resolver)]
    async fn posts(&self, ctx: &Context<'_>) -> Vec<Post> {
        // Lazy load posts
    }
}

#[derive(Query)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Option<User> {
        // Resolve user
    }
}
```

---

## Considerations

- **Server-side**: Schema generation, resolvers, DataLoader for N+1
- **Client-side**: Query builder, cache management (Apollo/urql patterns)
- **Subscriptions**: Real-time via WebSocket
- **Authentication**: Header/cookie-based auth per request
- **Authorization**: Field-level permissions

---

## When to Build

- REST becomes too limiting for complex queries
- Multiple projects need nested/graph-like data access
- Mobile clients need flexible queries

---

## Why Deferred Indefinitely

**Probably overkill.** REST + good API design is usually sufficient. Only consider if:
- Multiple projects need complex, nested queries
- Over-fetching/under-fetching becomes a real problem
- Team has GraphQL expertise

---

## Success Criteria

- [ ] Code-first schema generation
- [ ] DataLoader integration for N+1 prevention
- [ ] Subscription support
- [ ] Client-side type generation
- [ ] Authentication/authorization integration
- [ ] Documentation and examples

---

## Risks & Considerations

- Major paradigm shift from REST
- Learning curve for team
- Complexity of schema design
- Performance footguns (unbounded queries)
- Caching is harder than REST

---

**Created**: 2026-01-12
