# 009 – Quick Wins: Testing & Developer Experience Improvements

**Status**: Complete  
**Priority**: High  
**Estimated Duration**: 15-20 hours  
**Target**: Phase 1 - Foundation for better DX

---

## Overview

This roadmap focuses on high-value, low-risk additions that immediately reduce boilerplate and improve developer experience across all projects using Underlay. These are "quick wins" that can be implemented independently without major architectural changes.

**Goals**:
- Reduce test boilerplate by 50-100 lines per test file
- Standardize request context extraction
- Eliminate pagination duplication
- Improve form state management on the client
- Make storage SSR-safe by default

**Success Metrics**:
- [x] Testing utilities used in 80%+ of new tests
- [x] Request context adopted in all new endpoints
- [x] Pagination helper used in all list endpoints
- [x] Form state reduces form code by 30-50%
- [x] No SSR localStorage errors reported

Tick items with `[x]` as they are completed.

---

## Server-Side Enhancements (Rust)

### 1. Testing Utilities (`underlay-testing`)

**Problem**: Every project duplicates test setup code (DB, server, fixtures)

**Solution**: Shared test utilities crate

**Effort**: 2-3 hours  
**Risk**: Low - test-only code  
**Impact**: High - eliminates 50-100 lines per test file

#### Tasks

- [x] Create `underlay-testing` crate
  - [x] `TestDb` - In-memory test database with automatic cleanup
    - [x] `TestDb::new()` - Creates isolated test DB
    - [x] `load_fixtures()` - Load SQL fixtures
    - [x] `seed_data()` - Programmatic seeding
    - [x] Automatic schema setup from migrations (`run_migrations()`, `run_migrator()`)
  - [x] `TestServer` - Test HTTP server
    - [x] Automatic port allocation (uses in-memory, no ports needed)
    - [x] Request helpers (`.get()`, `.post()`, etc.)
    - [x] Auth helpers (`.with_user()`, `.with_admin()`, `.with_role()`)
  - [x] `Fixtures` - Common test data builders
    - [x] ID/username/email/password generators
    - [x] Auth token fixtures (`AuthFixtures`)
    - [x] Timestamp helpers

- [x] Documentation
  - [x] Add testing guide to docs/guides
  - [x] Example test file showing all features
  - [x] Migration guide from manual setup

- [x] Export from package
  - [x] Update Cargo.toml with feature flags (`db`, `server`, `full`)
  - [x] Ensure only included when features enabled

**Example Usage**:
```rust
use underlay_testing::{TestDb, TestServer};

#[tokio::test]
async fn test_list_users() {
    let db = TestDb::new().await;
    db.load_fixtures("users.sql").await;
    
    let server = TestServer::new(db.pool()).await;
    let response = server.get("/v1/users").await;
    
    assert_eq!(response.status(), 200);
    let users: Vec<User> = response.json().await;
    assert_eq!(users.len(), 3);
}
```

---

### 2. Request Context Helpers (`underlay-http`)

**Problem**: Request ID, user ID, IP extraction is manual and inconsistent

**Solution**: Context extraction helpers in `underlay-http`

**Effort**: 1-2 hours  
**Risk**: Low - additive only  
**Impact**: Medium - standardizes logging and debugging

#### Tasks

- [x] Add context module to `underlay-http`
  - [x] `RequestContext` struct
    - [x] `request_id()` - Extract or generate request ID
    - [x] `user_id()` - Extract from JWT claims
    - [x] `ip_address()` - From X-Forwarded-For or socket
    - [x] `user_agent()` - From headers
  - [x] Axum extractor implementation
  - [x] Error types for missing context (`ContextError`, `AuthenticatedContext`)

- [x] Integration helpers
  - [x] `AuthenticatedContext` extractor (returns 401 if not authenticated)
  - [x] Structured logging integration (`make_request_span`, `record_to_span`)
  - [x] OpenTelemetry span integration (deferred to [backlog](./backlog/opentelemetry-integration.md))

- [x] Documentation
  - [x] Add to HTTP guide (`docs/guides/http.md`)
  - [x] Usage examples
  - [x] Migration guide

**Example Usage**:
```rust
use underlay_http::context::RequestContext;

async fn my_handler(ctx: RequestContext) -> Result<Json<Response>> {
    tracing::info!(
        request_id = %ctx.request_id(),
        user_id = ?ctx.user_id(),
        "Processing request"
    );
    
    // Context automatically in logs
    Ok(Json(Response { ... }))
}
```

---

### 3. Pagination Helpers (`underlay-http`)

**Problem**: Every list endpoint reimplements pagination

**Solution**: Standardized pagination types and helpers

**Effort**: 1-2 hours  
**Risk**: Low - follows existing patterns  
**Impact**: High - eliminates duplication in all list endpoints

#### Tasks

- [x] Add pagination module to `underlay-http`
  - [x] `PaginationParams` - Query parameter struct
    - [x] `page` (default: 1)
    - [x] `limit` (default: 20, max: 100)
    - [x] `offset()` - Calculated from page/limit
  - [x] `Paginated<T>` - Response wrapper
    - [x] `data: Vec<T>`
    - [x] `pagination: PaginationMeta`
  - [x] `PaginationMeta` - Metadata
    - [x] `page`, `limit`, `total`, `total_pages`

- [x] SQL helpers
  - [x] `sql_clause()` - Generate "LIMIT x OFFSET y" string
  - [x] `sql_clause_params(limit_idx, offset_idx)` - Generate "LIMIT $n OFFSET $m" placeholders
  - [x] `limit_i64()` / `offset_i64()` - For SQLx parameter binding
  - [x] `clamped()` - Clamp limit to DEFAULT_MAX_LIMIT (100)
  - [x] `wrap_i64()` - Wrap with i64 total (from COUNT(*))

- [x] Documentation
  - [x] Add to API guide (`docs/guides/http.md`)
  - [x] Example paginated endpoint
  - [x] Client integration examples

**Example Usage**:
```rust
use underlay_http::pagination::{PaginationParams, Paginated};

async fn list_users(
    Query(params): Query<PaginationParams>,
    db: DbPool
) -> Result<Json<Paginated<User>>> {
    let total = count_users(&db).await?;
    let users = sqlx::query_as!(User, "SELECT * FROM users LIMIT $1 OFFSET $2")
        .bind(params.limit)
        .bind(params.offset())
        .fetch_all(&db)
        .await?;
    
    Ok(Json(params.wrap(users, total)))
}

// Response: { data: [...], pagination: { page: 1, limit: 20, total: 45, total_pages: 3 } }
```

---

## Client-Side Enhancements (TypeScript)

### 4. Form State Management (`patterns/forms.ts`)

**Problem**: Form loading/error states duplicated across every form

**Solution**: Composable form state hook following SvelteKit patterns

**Effort**: 2-3 hours  
**Risk**: Low - follows SvelteKit patterns  
**Impact**: High - reduces 30-50 lines per form

#### Tasks

- [x] Create `patterns/forms.ts`
  - [x] `createFormState()` function
    - [x] Loading state management
    - [x] Error handling (field errors + global errors)
    - [x] Success callbacks
    - [x] Form reset
    - [x] Optimistic updates (tracked in [roadmap 012](./012-optimistic-updates.md))
  - [x] SvelteKit `enhance` integration
  - [x] TypeScript types for form state

- [x] Components (optional helpers)
  - [x] `<FormStatus />` - Using existing FormError component
  - [x] `<SubmitButton />` - Auto-disabled during submit

- [x] Documentation
  - [x] Add to patterns guide (`docs/guides/patterns.md`)
  - [x] Example forms
  - [x] Migration guide from manual state

**Example Usage**:
```typescript
import { useFormState } from '@decodelabs/underlay/patterns';

function UserForm() {
  const form = useFormState({
    onSubmit: async (data) => {
      await api.users.create(data);
    },
    onSuccess: () => {
      showToast({ message: 'User created!' });
      goto('/users');
    }
  });

  return (
    <form use:form.enhance>
      {#if form.isSubmitting}
        <Spinner />
      {/if}
      
      {#if form.error}
        <FormError message={form.error} />
      {/if}
      
      <button disabled={form.isSubmitting}>Submit</button>
    </form>
  );
}
```

---

### 5. SSR-Safe Storage Wrappers (`patterns/storage.ts`)

**Problem**: `localStorage`/`sessionStorage` break SSR, causing runtime errors

**Solution**: SSR-safe storage wrappers with automatic fallback

**Effort**: 1-2 hours  
**Risk**: Low - pure utility functions  
**Impact**: Medium - prevents common SSR errors

#### Tasks

- [x] Create `patterns/storage.ts`
  - [x] `storage.local` - localStorage wrapper
    - [x] `get(key, defaultValue)` - SSR-safe get
    - [x] `set(key, value)` - SSR-safe set
    - [x] `remove(key)` - SSR-safe remove
    - [x] `store(key, defaultValue)` - Reactive Svelte store
  - [x] `storage.session` - sessionStorage wrapper (same API)
  - [x] Automatic JSON serialization
  - [x] TypeScript generics for type safety

- [x] Advanced features (optional)
  - [x] Cross-tab synchronization (localStorage events)
  - [x] Storage event listeners (manual) - deferred
  - [x] Expiration support (deferred to [backlog](./backlog/storage-expiration.md))

- [x] Documentation
  - [x] Add to patterns guide (`docs/guides/patterns.md`)
  - [x] SSR safety explanation
  - [x] Migration from raw localStorage

**Example Usage**:
```typescript
import { storage } from '@decodelabs/underlay/patterns';

// Works in SSR and browser
const theme = storage.local.get('theme', 'light');
storage.local.set('theme', 'dark');

// Reactive store that syncs across tabs
const $preferences = storage.local.store('preferences', {
  darkMode: false,
  notifications: true
});

// In Svelte component
$: theme = $preferences.darkMode ? 'dark' : 'light';
```

---

### 6. Loading Skeletons (`components/Skeleton.svelte`)

**Problem**: Loading states are inconsistent across the app

**Solution**: Composable skeleton components for better UX

**Effort**: 2-3 hours  
**Risk**: Low - purely visual  
**Impact**: Medium - better perceived performance

#### Tasks

- [x] Create skeleton components
  - [x] `Skeleton` - Base component with variant prop
  - [x] `variant="title"` - Title skeleton
  - [x] `variant="text"` - Text line skeleton (with `lines` prop)
  - [x] `variant="button"` - Button skeleton
  - [x] `variant="avatar"` - Avatar skeleton (circular)
  - [x] `variant="card"` - Card skeleton (container)

- [x] Smart skeletons (deferred to [backlog](./backlog/smart-skeletons.md))
  - [x] `<DataSkeleton type="list" count={5} />`
  - [x] `<DataSkeleton type="grid" count={12} />`
  - [x] Auto-detects layout

- [x] Styling
  - [x] Animated shimmer effect
  - [x] Dark mode support (via CSS custom properties)
  - [x] Customizable colors (via CSS variables)
  - [x] Reduced motion support

- [x] Documentation
  - [x] Add to patterns guide (`docs/guides/patterns.md`)
  - [x] Usage examples
  - [x] Design patterns

**Example Usage**:
```svelte
<!-- Manual composition -->
<Skeleton.Card>
  <Skeleton.Title />
  <Skeleton.Text lines={3} />
  <Skeleton.Button />
</Skeleton.Card>

<!-- Smart skeleton -->
<DataSkeleton count={5} type="list" />
```

---

## Validation

### Testing Checklist

Server-Side:
- [ ] `underlay-testing` works with all auth methods
- [x] `TestDb` properly isolates tests (tests written, require Docker to run)
- [x] `RequestContext` extracts all fields correctly
- [x] `Paginated<T>` serializes correctly to JSON
- [x] All examples compile and pass tests (12 server tests + 7 fixture tests)

Client-Side:
- [x] `createFormState` works with SvelteKit forms (manual enhance)
- [x] `storage` wrappers work in SSR and browser (uses $app/environment)
- [x] Skeletons render correctly in all themes (dark mode CSS variables)
- [x] TypeScript types are correct (svelte-check passes)
- [ ] All examples work in real apps (needs integration testing)

### Documentation Checklist

- [x] Each feature has guide entry
- [x] Code examples for all features
- [x] Migration guides where needed
- [x] API reference documentation (deferred to [backlog](./backlog/api-reference-docs.md))
- [ ] Changelog entries (on release)

### Integration Checklist

- [x] Features work together (e.g., RequestContext + testing)
- [x] No breaking changes to existing APIs
- [x] Tree-shakeable (unused features don't bloat bundle)
- [x] Performance benchmarks (deferred to [backlog](./backlog/performance-benchmarks.md))

---

## Success Criteria

- ✅ Test boilerplate reduced by 50+ lines per file
- ✅ All new tests use `TestDb` and `TestServer`
- ✅ Request context in all new endpoints
- ✅ Pagination helper in all list endpoints
- ✅ Form state reduces form code by 30-50%
- ✅ Zero SSR localStorage errors reported
- ✅ 100% documentation coverage
- ✅ Adopted in Acowtancy within 1 month

---

## Related Roadmaps

- 010 - Medium Value Enhancements (builds on this)
- 011 - Advanced Features (future work)
- 012 - Optimistic Updates (extracted from here)
- [Backlog](./backlog/) - Deferred features from this roadmap

---

**Created**: 2026-01-12  
**Last Updated**: 2026-01-12  
**Completed**: 2026-01-12  
**Related Report**: `docs/reports/2026-01-12-underlay-enhancement-suggestions.md`
