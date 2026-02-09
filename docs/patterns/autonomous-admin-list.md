# Recipe: Autonomous Admin List (Paginated + Filtered + Batch)

**Use when**: You need a self-contained admin list component that can run as a full page or embedded tab, with pagination, filtering, and batch actions.

**Example prompt**: "Build an autonomous list for Resources with filters and batch delete"

---

## Key Principle

Make the list component own its own data lifecycle and controls:
1. **Fetch + paginate inside the component**
2. **Keep filter/query state local and URL-safe where needed**
3. **Use batch action hooks for multi-select destructive operations**

---

## Checklist

### Phase 1: API + Client Contract

- [ ] Provide paginated list endpoint returning `PaginatedResponse<T>`
- [ ] Support filter/sort query params
- [ ] Add client command with `pagination` + `filters` arguments

**References**:
- `cattle-grid/src/commands/learning/modules.ts`
- `cattle-grid/src/commands/media-commands.ts`

### Phase 2: Component Skeleton

**File**: `dairy/src/lib/lists/{Entity}List.svelte`

- [ ] Add props for `variant`, context filters (`pathwayId`-style), and `onDataChange`
- [ ] Render with `PageHeader`, `FilterBar`, `ListGrid`/`DataTable`, `Pagination`
- [ ] Keep list reusable in both root pages and tab content

### Phase 3: Data Loading Pattern

- [ ] Use `createPaginationController()` for server pagination
- [ ] Use `useAuthenticatedData()` for supporting data (dropdown options, etc.)
- [ ] Call `tryFetch($authLoading, $currentUser)` in `$effect`

### Phase 4: Filters and Query Mapping

- [ ] Keep filter state local (`search`, `status`, etc.)
- [ ] Map filter state to command query shape in one helper function
- [ ] Use `pagination.reset()` or `refresh()` on filter changes
- [ ] Persist pagination state via `persistKey` when useful

### Phase 5: Batch Selection and Actions

- [ ] Use `useBatchActions<string>()`
- [ ] Register destructive action(s) (`delete`, `archive`, etc.)
- [ ] Add confirmation copy with count-aware message
- [ ] Render `BatchActionBar` only in selection mode

### Phase 6: Row/Card Actions + Navigation Context

- [ ] Create `sourceContext` from current route
- [ ] Use `gotoWithContext()` for create/edit/detail navigation
- [ ] Keep per-row actions in list card or action menu components

### Phase 7: UX States

- [ ] Show `PageLoading` for initial load
- [ ] Show `FormError` for failures
- [ ] Show empty state copy distinct from "no matches"
- [ ] Use `useToasts()` for action success/failure feedback

---

## Atomic Patterns Used

| Pattern | Purpose |
|---------|---------|
| `createPaginationController` | Server pagination state + fetch lifecycle |
| `useBatchActions` | Multi-select actions + confirmation |
| `FilterBar` | Unified filtering UI |
| `gotoWithContext` | Context-preserving navigation |
| `useAuthenticatedData` | Auth-gated fetch and retries |

---

## References in Acowtancy

- `dairy/src/lib/lists/ModulesList.svelte`
- `dairy/src/lib/lists/MediaList.svelte`
- `dairy/src/lib/lists/VideosList.svelte`
- `dairy/src/lib/lists/DocumentsList.svelte`
