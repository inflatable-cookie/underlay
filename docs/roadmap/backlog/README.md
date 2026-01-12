# Feature Backlog

This directory contains deferred features and enhancement ideas that are not currently scheduled for implementation. These items were identified during roadmap work but deprioritized to keep focused delivery.

## Backlog Items

| File | Description | Effort | Priority |
|------|-------------|--------|----------|
| [opentelemetry-integration.md](./opentelemetry-integration.md) | OTLP tracing for distributed observability | 4-6h | Medium |
| [storage-expiration.md](./storage-expiration.md) | TTL support for SSR-safe storage | 2-3h | Low |
| [smart-skeletons.md](./smart-skeletons.md) | Auto-layout DataSkeleton component | 3-4h | Low |
| [api-reference-docs.md](./api-reference-docs.md) | Automated rustdoc/TypeDoc generation | 6-8h | Medium |
| [performance-benchmarks.md](./performance-benchmarks.md) | Criterion benchmarks + CI integration | 4-6h | Low |

## When to Promote

Move items from backlog to active roadmap when:

1. **Demand**: Multiple consumers request the feature
2. **Dependency**: An active roadmap item requires it
3. **Opportunity**: Developer has time between planned work
4. **Strategic**: Aligns with upcoming project goals

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
