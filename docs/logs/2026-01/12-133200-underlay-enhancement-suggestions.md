# Underlay Enhancement Suggestions

**Date**: 2026-01-12  
**Context**: Post-Acowtancy migration - identifying useful additions to improve development across all projects

---

## Current Underlay Surface Area

### Rust Crates (Server-Side)
- ✅ `underlay-http` - CORS, error responses, structured responses
- ✅ `underlay-db` - Pool management, migrations, schema management
- ✅ `underlay-auth-*` - JWT, password, OAuth, TOTP, WebAuthn
- ✅ `underlay-jobs` - Background job system
- ✅ `underlay-metrics` - Metrics collection/serving
- ✅ `underlay-observability` - Logging/tracing
- ✅ `underlay-openapi` - OpenAPI generation
- ✅ `underlay-soft-delete` - Soft delete patterns
- ✅ `underlay-events` - Event system

### TypeScript (Client-Side)
- ✅ `client/` - HTTP client, auth hooks, SvelteKit helpers
- ✅ `components/` - 20+ Svelte components (forms, dialogs, buttons, etc.)
- ✅ `patterns/` - Toasts, clipboard, auth patterns, page layouts
- ✅ `nightfire/` - Rich content editor
- ✅ `tools/` - Guardrails CLI
- ✅ `styles/` - CSS utilities

**Total**: ~84 TypeScript files, comprehensive Rust crate ecosystem

---

## Suggested Enhancements

### 🟢 High Value, Low Risk (Quick Wins)

#### Server-Side (Rust)

**1. Testing Utilities** (`underlay-testing`)
```rust
// Problem: Every project duplicates test setup
// Solution: Shared test utilities

use underlay_testing::{TestDb, TestServer, Fixtures};

#[tokio::test]
async fn test_my_endpoint() {
    let db = TestDb::new().await;
    db.load_fixtures("users.sql").await;
    
    let server = TestServer::new(db.pool()).await;
    let response = server.get("/v1/users").await;
    
    assert_eq!(response.status(), 200);
}
```

**Why**: Eliminates 50-100 lines of boilerplate per test file  
**Risk**: Low - test-only code  
**Effort**: 2-3 hours

**2. Request Context Helpers** (`underlay-http`)
```rust
// Problem: Request ID, user ID extraction is manual
// Solution: Context extraction helpers

use underlay_http::context::{RequestContext, extract_request_id};

async fn my_handler(ctx: RequestContext) -> Result<Json<Response>> {
    // Automatic request ID, user ID, IP address extraction
    ctx.request_id();  // Already parsed
    ctx.user_id()?;    // Extracted from JWT
    ctx.ip_address();  // From headers/connection
}
```

**Why**: Reduces duplication, standardizes logging  
**Risk**: Low - additive only  
**Effort**: 1-2 hours

**3. Validation Helpers** (`underlay-validation`)
```rust
// Problem: Validation scattered across handlers
// Solution: Declarative validation

use underlay_validation::{Validate, validate};

#[derive(Validate)]
struct CreateUserRequest {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8, max = 100))]
    password: String,
    
    #[validate(custom = "validate_age")]
    age: i32,
}

async fn create_user(req: Json<CreateUserRequest>) -> Result<()> {
    req.validate()?;  // Returns 400 with field errors
    // ...
}
```

**Why**: Standardizes validation, reduces boilerplate  
**Risk**: Low - opt-in  
**Effort**: 3-4 hours

**4. Pagination Helpers** (`underlay-http`)
```rust
// Problem: Every list endpoint reimplements pagination
// Solution: Standardized pagination

use underlay_http::pagination::{Paginated, PaginationParams};

async fn list_users(
    params: Query<PaginationParams>
) -> Result<Json<Paginated<User>>> {
    let users = db.query()
        .paginate(&params)  // Handles limit/offset
        .execute()
        .await?;
    
    Ok(Json(params.wrap(users, total_count)))
}

// Returns: { data: [...], pagination: { page, limit, total } }
```

**Why**: Eliminates duplication, standardizes API responses  
**Risk**: Low - follows existing patterns  
**Effort**: 1-2 hours

---

#### Client-Side (TypeScript)

**1. Form State Management** (`patterns/forms.ts`)
```typescript
// Problem: Form loading/error states duplicated
// Solution: Composable form state hook

import { useFormState } from '@inflatable-cookie/underlay/patterns';

function MyForm() {
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
      <!-- Automatic loading state, error handling, optimistic updates -->
      {#if form.isSubmitting}
        <Spinner />
      {/if}
      
      {#if form.error}
        <FormError message={form.error} />
      {/if}
    </form>
  );
}
```

**Why**: Reduces 30-50 lines per form  
**Risk**: Low - follows SvelteKit patterns  
**Effort**: 2-3 hours

**2. Data Tables** (`components/DataTable.svelte`)
```svelte
<!-- Problem: Every list page reimplements sorting/filtering -->
<!-- Solution: Composable table component -->

<DataTable
  data={users}
  columns={[
    { key: 'name', sortable: true, filterable: true },
    { key: 'email', sortable: true },
    { key: 'createdAt', sortable: true, formatter: formatDate }
  ]}
  actions={(user) => [
    { label: 'Edit', href: `/users/${user.id}/edit` },
    { label: 'Delete', onClick: () => deleteUser(user.id), variant: 'danger' }
  ]}
  pagination={{ page, limit, total }}
  on:sort={handleSort}
  on:filter={handleFilter}
/>
```

**Why**: Eliminates 100-200 lines of table boilerplate per page  
**Risk**: Low - opt-in component  
**Effort**: 4-5 hours

**3. SSR-Safe Storage Wrappers** (`patterns/storage.ts`)
```typescript
// Problem: localStorage/sessionStorage break SSR
// Solution: SSR-safe wrappers

import { storage } from '@inflatable-cookie/underlay/patterns';

// Works in SSR and browser
const theme = storage.local.get('theme', 'light');
storage.session.set('lastVisited', Date.now());

// Reactive store that syncs across tabs
const $preferences = storage.local.store('preferences', {
  darkMode: false,
  notifications: true
});
```

**Why**: Prevents SSR errors, standardizes storage  
**Risk**: Low - follows Svelte patterns  
**Effort**: 1-2 hours

**4. Loading Skeletons** (`components/Skeleton.svelte`)
```svelte
<!-- Problem: Loading states are inconsistent -->
<!-- Solution: Composable skeleton components -->

<Skeleton.Card>
  <Skeleton.Title />
  <Skeleton.Text lines={3} />
  <Skeleton.Button />
</Skeleton.Card>

<!-- Or use smart skeletons -->
<DataSkeleton count={5} type="list" />
```

**Why**: Better UX, consistent loading states  
**Risk**: Low - purely visual  
**Effort**: 2-3 hours

---

### 🟡 Medium Value, Medium Effort (Worth Considering)

#### Server-Side (Rust)

**5. Rate Limiting** (`underlay-rate-limit`)
```rust
// Protects endpoints from abuse
use underlay_rate_limit::{RateLimiter, RateLimit};

#[rate_limit(requests = 100, per = "1m", key = "user_id")]
async fn expensive_operation() -> Result<()> {
    // Rate limited per user
}
```

**Why**: Security, prevents abuse  
**Risk**: Medium - needs Redis/storage  
**Effort**: 4-6 hours

**6. Caching Layer** (`underlay-cache`)
```rust
// Standard caching patterns
use underlay_cache::{Cache, Cached};

#[cached(ttl = "5m", key = "user:{id}")]
async fn get_user(id: Uuid) -> Result<User> {
    // Cached result, automatic invalidation
}
```

**Why**: Performance, reduces DB load  
**Risk**: Medium - cache invalidation complexity  
**Effort**: 6-8 hours

**7. Background Job Dashboard** (`underlay-jobs` extension)
```rust
// Web UI for monitoring jobs
// /admin/jobs - view queued/running/failed jobs
// Retry, cancel, inspect payloads
```

**Why**: Operational visibility  
**Risk**: Low - admin-only feature  
**Effort**: 8-10 hours

---

#### Client-Side (TypeScript)

**5. Optimistic Updates** (`client/optimistic.ts`)
```typescript
// Problem: CRUD operations feel slow
// Solution: Optimistic update helpers

import { optimistic } from '@inflatable-cookie/underlay/client';

const deleteUser = optimistic({
  mutate: (id) => api.users.delete(id),
  optimistic: (id) => {
    users = users.filter(u => u.id !== id);  // Immediate UI update
  },
  onError: (id, error) => {
    // Rollback on failure
    fetchUsers();
  }
});
```

**Why**: Better UX, feels faster  
**Risk**: Medium - needs careful rollback handling  
**Effort**: 3-4 hours

**6. File Upload Component** (`components/FileUpload.svelte`)
```svelte
<!-- Drag-drop, progress, previews -->
<FileUpload
  accept="image/*"
  maxSize={5 * 1024 * 1024}
  on:upload={handleUpload}
  showPreview
  multiple
/>
```

**Why**: Common pattern, reduces duplication  
**Risk**: Low - standalone component  
**Effort**: 4-5 hours

**7. Internationalization Helpers** (`patterns/i18n.ts`)
```typescript
// Date, number, currency formatting
import { format } from '@inflatable-cookie/underlay/patterns';

format.date(date, 'short');  // "12 Jan 2026"
format.currency(1234.56, 'GBP');  // "£1,234.56"
format.relative(date);  // "2 hours ago"
```

**Why**: Standardizes formatting, i18n ready  
**Risk**: Low - pure utilities  
**Effort**: 2-3 hours

---

### 🔵 Advanced / Nice-to-Have

**8. GraphQL Support** (`underlay-graphql`)  
- Code-first schema generation
- Resolver helpers
- Subscriptions support

**Why**: Modern API alternative  
**Risk**: High - major paradigm shift  
**Effort**: 20-30 hours  
**Decision**: Probably overkill unless multiple projects need it

**9. Real-time / WebSocket Layer** (`underlay-realtime`)  
- Connection management
- Presence tracking
- Broadcast channels

**Why**: Live updates, collaboration features  
**Risk**: High - infrastructure complexity  
**Effort**: 15-20 hours  
**Decision**: Wait for concrete use case

**10. CLI Scaffolding** (`underlay-cli`)  
- Generate CRUD endpoints
- Generate DB migrations
- Generate Svelte components

**Why**: Speed up development  
**Risk**: Medium - maintenance burden  
**Effort**: 10-15 hours  
**Decision**: Useful but not urgent

---

## Implementation Priority

### Phase 1: Quick Wins (1-2 weeks)
1. ✅ Testing utilities (Rust)
2. ✅ Request context helpers (Rust)
3. ✅ Pagination helpers (Rust)
4. ✅ Form state management (TS)
5. ✅ SSR-safe storage (TS)
6. ✅ Loading skeletons (TS)

**Total effort**: ~15-20 hours  
**Impact**: High - immediate reduction in boilerplate

### Phase 2: Medium Value (3-4 weeks)
1. ✅ Validation helpers (Rust)
2. ✅ Data tables (TS)
3. ✅ Optimistic updates (TS)
4. ✅ File upload (TS)
5. ✅ i18n helpers (TS)

**Total effort**: ~20-25 hours  
**Impact**: Medium - improves UX and consistency

### Phase 3: Advanced (When Needed)
- Rate limiting
- Caching layer
- Background job dashboard
- Real-time features

**Decision**: Wait for concrete use cases

---

## Design Principles for Underlay

### 1. **Opt-In, Not Prescriptive**
- Every feature should be optional
- Don't force patterns on consumers
- Provide escape hatches

### 2. **Zero Breaking Changes**
- Always additive
- Deprecate gracefully
- Version carefully

### 3. **Well-Tested & Documented**
- Comprehensive tests
- Clear examples
- Migration guides

### 4. **Performance First**
- Minimal runtime overhead
- Tree-shakeable
- Lazy-loadable

### 5. **Developer Experience**
- Clear error messages
- TypeScript-first
- IDE autocomplete-friendly

---

## What NOT to Add

### ❌ Avoid Over-Engineering
- **No** heavy frameworks (don't become Next.js/Laravel)
- **No** opinionated state management (let apps choose)
- **No** complete CMS/admin generators
- **No** kitchen-sink UI library (keep components focused)

### ❌ Avoid Lock-In
- **No** proprietary formats
- **No** vendor-specific integrations (keep generic)
- **No** breaking changes without migration path

### ❌ Avoid Duplication
- **No** reimplementing well-solved problems (use dependencies)
- **No** competing with ecosystem (e.g., don't build another ORM)

---

## Measuring Success

### Metrics to Track
1. **Lines of code reduction** in consuming projects
2. **Time to implement feature** (before vs after Underlay)
3. **Bug count** in common patterns (should decrease)
4. **Adoption rate** (% of features used across projects)
5. **Developer satisfaction** (survey feedback)

### Success Criteria
- ✅ Reduces boilerplate by 30-50% for common patterns
- ✅ No performance regression vs. hand-rolled code
- ✅ Adopted in 80%+ of new features across projects
- ✅ Clear documentation for 100% of APIs
- ✅ Zero breaking changes in minor versions

---

## Next Steps

### Immediate Actions
1. **Gather feedback** - Ask team which patterns are most painful
2. **Prototype** 1-2 quick wins to validate approach
3. **Document** design patterns and guidelines
4. **Set up** testing infrastructure for Underlay itself

### Long-Term Vision
- Underlay becomes the "standard library" for Acowtancy projects
- New projects start with 50% less boilerplate
- Common bugs eliminated at framework level
- Team can focus on domain logic, not infrastructure

---

**Conclusion**: Focus on **high-value, low-risk additions** that reduce boilerplate without being prescriptive. Start with testing utilities, form state, and pagination helpers. Avoid over-engineering - Underlay should enable, not constrain.
