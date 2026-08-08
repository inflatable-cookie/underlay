# Backlog: Smart Skeletons (DataSkeleton)

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 3-4 hours  
**Source**: Deferred from roadmap 009 (Quick Wins)

---

## Problem Statement

The current `Skeleton` component provides basic building blocks (title, text, button, avatar, card), but developers must manually compose them to match their data layouts. This leads to:

- Repetitive skeleton composition for common patterns
- Skeletons that don't match actual data layouts
- Extra effort maintaining skeleton/data layout parity
- Inconsistent loading experiences across the app

---

## Proposed Solution

Create a `DataSkeleton` component that automatically generates appropriate skeleton layouts:

### 1. List Skeleton

```svelte
<!-- Generates 5 list item skeletons -->
<DataSkeleton type="list" count={5} />

<!-- With avatar + text pattern -->
<DataSkeleton type="list" count={5} pattern="avatar-text" />

<!-- With card wrapping -->
<DataSkeleton type="list" count={3} pattern="card" />
```

### 2. Grid Skeleton

```svelte
<!-- 12-item grid (auto-detects columns from container) -->
<DataSkeleton type="grid" count={12} />

<!-- Explicit column count -->
<DataSkeleton type="grid" count={12} columns={4} />

<!-- Product card pattern -->
<DataSkeleton type="grid" count={8} pattern="product-card" />
```

### 3. Table Skeleton

```svelte
<!-- Table with 10 rows, 5 columns -->
<DataSkeleton type="table" rows={10} columns={5} />

<!-- With header row -->
<DataSkeleton type="table" rows={10} columns={5} header />
```

### 4. Detail View Skeleton

```svelte
<!-- Common detail page layout -->
<DataSkeleton type="detail" />

<!-- Custom sections -->
<DataSkeleton type="detail" sections={['header', 'stats', 'description', 'actions']} />
```

### 5. Pattern Registry

Allow apps to register custom patterns:

```typescript
import { registerSkeletonPattern } from '@inflatable-cookie/underlay/components';

registerSkeletonPattern('user-card', () => `
  <div class="user-card-skeleton">
    <Skeleton variant="avatar" size="lg" />
    <Skeleton variant="title" />
    <Skeleton variant="text" lines={2} />
  </div>
`);

// Usage
<DataSkeleton pattern="user-card" count={6} />
```

---

## Dependencies

- Existing `Skeleton.svelte` component
- CSS Grid/Flexbox for layout detection

---

## Success Criteria

- [ ] `DataSkeleton` component with list/grid/table/detail types
- [ ] Auto-detects column count from container (for grid)
- [ ] Built-in patterns for common layouts
- [ ] Custom pattern registration API
- [ ] Maintains existing Skeleton API (non-breaking)
- [ ] Storybook examples for all patterns

---

## Risks & Considerations

- **Layout detection**: CSS container queries may not work everywhere
- **Pattern proliferation**: Need to keep built-in patterns minimal
- **Performance**: Avoid expensive layout calculations
- **SSR**: Layout detection must be SSR-safe

---

## Design Decisions

### Option A: Component Props (Recommended)
```svelte
<DataSkeleton type="list" count={5} pattern="avatar-text" />
```
- Pros: Simple, declarative, tree-shakeable
- Cons: Less flexible for complex layouts

### Option B: Slot-based
```svelte
<DataSkeleton type="list" count={5}>
  <SkeletonItem slot="item">
    <Skeleton.Avatar />
    <Skeleton.Text />
  </SkeletonItem>
</DataSkeleton>
```
- Pros: Maximum flexibility
- Cons: More verbose, harder to use

**Decision**: Start with Option A, add slot support if needed.

---

## Related

- `ts/src/components/Skeleton.svelte` - Base skeleton component
- `docs/guides/100-frontend-bloom.md` - Component documentation

---

**Created**: 2026-01-12
