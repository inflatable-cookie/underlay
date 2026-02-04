# Feature Backlog

This directory contains deferred features and enhancement ideas that are not currently scheduled for implementation. These items were identified during roadmap work but deprioritized to keep focused delivery.

## Backlog Items

### From Roadmap 011 (Advanced Features - YAGNI)

| File | Description | Effort | Priority |
|------|-------------|--------|----------|
| [rate-limiting.md](./rate-limiting.md) | Pluggable rate limiting middleware | 4-6h | Low |
| [caching-layer.md](./caching-layer.md) | Declarative caching with invalidation | 6-8h | Low |
| [background-job-dashboard.md](./background-job-dashboard.md) | Web UI for monitoring jobs | 8-10h | Low |
| [graphql-support.md](./graphql-support.md) | Code-first GraphQL schema | 20-30h | Very Low |
| [realtime-websocket.md](./realtime-websocket.md) | WebSocket channels and presence | 15-20h | Low |
| [cli-scaffolding.md](./cli-scaffolding.md) | Generate boilerplate code | 10-15h | Low |
| [advanced-datatable.md](./advanced-datatable.md) | Column reorder, resize, inline edit | 5-10h | Low |
| [advanced-forms.md](./advanced-forms.md) | Wizard, auto-save, conditional fields | 4-8h | Low |

### Other Deferred Items

| File | Description | Effort | Priority |
|------|-------------|--------|----------|
| [opentelemetry-integration.md](./opentelemetry-integration.md) | OTLP tracing for distributed observability | 4-6h | Medium |
| [storage-expiration.md](./storage-expiration.md) | TTL support for SSR-safe storage | 2-3h | Low |
| [smart-skeletons.md](./smart-skeletons.md) | Auto-layout DataSkeleton component | 3-4h | Low |
| [api-reference-docs.md](./api-reference-docs.md) | Automated rustdoc/TypeDoc generation | 6-8h | Medium |
| [performance-benchmarks.md](./performance-benchmarks.md) | Criterion benchmarks + CI integration | 4-6h | Low |
| [storybook-component-docs.md](./storybook-component-docs.md) | Interactive component documentation with Storybook | 8-12h | Low |

## When to Promote

Move items from backlog to active roadmap when:

1. **Demand**: Multiple consumers request the feature
2. **Dependency**: An active roadmap item requires it
3. **Opportunity**: Developer has time between planned work
4. **Strategic**: Aligns with upcoming project goals

For items from the former roadmap 011, also consider:
- Is there a concrete use case from 2+ projects?
- Is manual implementation >100 lines of boilerplate?
- Is the feature generic enough to be reusable?
- Do we have capacity to maintain it long-term?

## Adding New Items

When deferring work from an active roadmap:

1. Create a new file: `descriptive-name.md`
2. Include: problem statement, proposed solution, effort estimate, dependencies, success criteria
3. Update the table above
4. Link from the source roadmap with `(deferred to backlog)`

## Template

```markdown
# Backlog: Feature Name

**Status**: Backlog  
**Priority**: Low/Medium/High  
**Estimated Effort**: X-Y hours  
**Source**: Deferred from roadmap NNN

---

## Problem Statement

[Why this matters]

---

## Proposed Solution

[How to solve it]

---

## Dependencies

- [What's needed first]

---

## Success Criteria

- [ ] [Measurable outcomes]

---

## Risks & Considerations

- [What could go wrong]

---

**Created**: YYYY-MM-DD
```
